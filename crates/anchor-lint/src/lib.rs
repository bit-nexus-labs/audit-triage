use std::collections::BTreeSet;
use syn::{punctuated::Punctuated, spanned::Spanned, visit::Visit, Token};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Rule {
    R1,
    R2,
    R3,
}

impl Rule {
    pub fn id(self) -> &'static str {
        ["R1", "R2", "R3"][self as usize]
    }
    pub fn message(self) -> &'static str {
        match self {
            Self::R1 => "may panic",
            Self::R2 => "unchecked arithmetic",
            Self::R3 => "account without CHECK comment",
        }
    }
}

pub type Finding = (usize, Rule);

fn is_const(e: &syn::Expr) -> bool {
    match e {
        syn::Expr::Lit(_) => true,
        syn::Expr::Paren(p) => is_const(&p.expr),
        syn::Expr::Path(p) => p.path.segments.last().is_some_and(|s| {
            let s = s.ident.to_string();
            s.chars().any(char::is_uppercase) && !s.chars().any(char::is_lowercase)
        }),
        _ => false,
    }
}

fn is_test(a: &syn::Attribute) -> bool {
    a.path().is_ident("test")
        || matches!(&a.meta, syn::Meta::List(l) if l.path.is_ident("cfg") && l.tokens.to_string() == "test")
}

fn has_check(f: &syn::Field) -> bool {
    f.attrs.iter().any(|a| {
        a.path().is_ident("doc")
            && match &a.meta {
                syn::Meta::NameValue(nv) => matches!(&nv.value, syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) if s.value().trim().starts_with("CHECK:")),
                _ => false,
            }
    })
}

struct Linter(BTreeSet<(usize, Rule)>);

impl<'ast> Visit<'ast> for Linter {
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if i.method == "unwrap" || i.method == "expect" {
            self.0.insert((i.method.span().start().line, Rule::R1));
        }
        syn::visit::visit_expr_method_call(self, i);
    }

    fn visit_expr_binary(&mut self, i: &'ast syn::ExprBinary) {
        let op = &i.op;
        use syn::BinOp::*;
        if matches!(
            op,
            Add(_) | Sub(_) | Mul(_) | AddAssign(_) | SubAssign(_) | MulAssign(_)
        ) && !(is_const(&i.left) && is_const(&i.right))
        {
            self.0.insert((op.span().start().line, Rule::R2));
        }
        syn::visit::visit_expr_binary(self, i);
    }

    fn visit_macro(&mut self, m: &'ast syn::Macro) {
        let skip = [
            "msg", "println", "print", "eprintln", "eprint", "format", "write", "writeln", "dbg",
            "info", "debug", "warn", "error", "trace",
        ];
        let name = m.path.segments.last().map(|s| s.ident.to_string());
        if name.as_deref().is_some_and(|s| skip.contains(&s)) {
            return;
        }
        if let Ok(es) = m.parse_body_with(Punctuated::<syn::Expr, Token![,]>::parse_terminated) {
            es.iter().for_each(|e| self.visit_expr(e));
        }
    }

    fn visit_item_struct(&mut self, s: &'ast syn::ItemStruct) {
        let mut acc = false;
        for a in s.attrs.iter().filter(|a| a.path().is_ident("derive")) {
            let _ = a.parse_nested_meta(|m| {
                acc |= m.path.is_ident("Accounts");
                Ok(())
            });
        }
        if acc {
            for f in &s.fields {
                let is_acc = matches!(&f.ty, syn::Type::Path(p) if p.path.segments.last().is_some_and(|s| s.ident == "AccountInfo" || s.ident == "UncheckedAccount"));
                if is_acc && !has_check(f) {
                    self.0.insert((f.ty.span().start().line, Rule::R3));
                }
            }
        }
        syn::visit::visit_item_struct(self, s);
    }

    fn visit_item_mod(&mut self, m: &'ast syn::ItemMod) {
        if !m.attrs.iter().any(is_test) {
            syn::visit::visit_item_mod(self, m);
        }
    }

    fn visit_item_fn(&mut self, f: &'ast syn::ItemFn) {
        if !f.attrs.iter().any(is_test) {
            syn::visit::visit_item_fn(self, f);
        }
    }
}

pub fn lint_source(src: &str) -> Result<BTreeSet<(usize, Rule)>, syn::Error> {
    let file = syn::parse_str::<syn::File>(src)?;
    let mut l = Linter(BTreeSet::new());
    l.visit_file(&file);
    Ok(l.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use Rule::*;

    const SRC: &str = "fn f() {\nx.unwrap();\ny.expect(\"\");\nz.unwrap_or(0); let s = \"x.unwrap() + 1\"; // x.unwrap() + 1\nlet _ = x+y;\nlet _ = x\n + y;\nx += 1; let _ = FEE + 5; let _ = 1 + 2;\nx.checked_add(1); y.wrapping_add(2); msg!(\"{}\", a + b); require!(a + b > 1, E); let _ = a + b + c;\n}\n#[derive(Accounts)] struct S { a: AccountInfo,\n/// CHECK: ok\nb: AccountInfo }\nstruct N { c: AccountInfo }\n#[cfg(test)] mod t { fn f() { x.unwrap(); } }\n#[test] fn t2() { a + b; }";

    #[test]
    fn test_rules() {
        let res = lint_source(SRC).unwrap();
        let r = [(2, R1), (3, R1), (5, R2), (7, R2), (8, R2), (9, R2)];
        let exp: BTreeSet<_> = r.into_iter().chain([(11, R3)]).collect();
        assert_eq!(res, exp);
        assert!(lint_source("fn (").is_err());
    }
}
