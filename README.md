# audit-triage

Status: **specification stage.** Capstone project for SVM Engineering.

A command-line tool that turns raw lint findings from Anchor (Solana) programs into a prioritised, persistent, reportable list for a security auditor.

The full specification is in [`docs/SPEC.md`](docs/SPEC.md): problem, architecture (with diagram), technologies and why, how we check it is done, and what is out of scope.

It builds on [anchor-lint-v2](https://github.com/bit-nexus-labs/anchor-lint-v2), the week-2 linter, which becomes the rule engine.

No code yet. The first milestone (splitting the linter into a library) comes after the specification is reviewed.
