# audit-triage

A command-line tool that turns raw lint findings from Anchor (Solana) programs into a prioritised, persistent, reportable list for a security auditor.

## Status

Workspace setup done. M1 is partly done: anchor-lint is split into a library (lint_source, no printing) and a small binary, tests green. The Finding type with a token span is not done yet.

## How to Build and Run

```sh
cargo build
cargo test
cargo run -p triage-cli -- --help
```

## Plan and Milestones

The full specification is in [`docs/SPEC.md`](docs/SPEC.md).

- [ ] **M1.** Split anchor-lint into library plus binary (done); Finding with span (not done yet); tests still green.
- [ ] **M2.** Enrichment and scoring, with the reasons list.
- [ ] **M3.** Fingerprint and the state file, `mark` and `scan`.
- [ ] **M4.** Markdown and JSON report, diff section.
- [ ] **M5.** Failure modes, real-world run, README with measured results.

## Crate Layout

| Crate | Kind | Responsibility |
|---|---|---|
| `anchor-lint` | library (+ its own small binary, kept) | Library lint_source(src) returning rule and line pairs; the span is planned for M1. No printing in the library. |
| `triage-core` | library | `Finding` types, enrichment, scoring, fingerprint, state, report (today only Priority and priority_from_score). Pure functions over data, no I/O except the state module. |
| `triage-cli` | binary `audit-triage` | argument parsing, reading files, calling `triage-core`, printing, exit codes. The only place that touches the terminal. |
