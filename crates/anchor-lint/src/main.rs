use anchor_lint::lint_source;
use std::path::{Path, PathBuf};
use std::{env, fs, io, process};

fn collect_files(p: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    if p.is_file() {
        out.push(p.to_path_buf());
    } else {
        for e in fs::read_dir(p)? {
            let p = e?.path();
            let s = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if p.is_dir() && s != "target" && !s.starts_with('.') {
                collect_files(&p, out)?;
            } else if p.extension().is_some_and(|e| e == "rs") {
                out.push(p);
            }
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: anchor-lint <path>");
        process::exit(2);
    }
    let (p, mut files) = (Path::new(&args[1]), Vec::new());
    if !p.exists() || collect_files(p, &mut files).is_err() {
        eprintln!("{}: unreadable path", p.display());
        process::exit(2);
    }
    files.sort();
    let (mut total, mut err) = (0, false);
    for f in &files {
        let Ok(src) = fs::read_to_string(f) else {
            eprintln!("{}: read error", f.display());
            err = true;
            continue;
        };
        match lint_source(&src) {
            Err(e) => {
                eprintln!("{}: parse error: {e}", f.display());
                err = true;
            }
            Ok(res) => {
                let lines: Vec<_> = src.lines().collect();
                for (l, r) in res {
                    total += 1;
                    let txt = lines.get(l.saturating_sub(1)).unwrap_or(&"").trim();
                    println!("{}:{l}: [{}] {}\n  {txt}", f.display(), r.id(), r.message());
                }
            }
        }
    }
    println!("{total} finding(s)");
    process::exit(if err { 2 } else { i32::from(total > 0) });
}
