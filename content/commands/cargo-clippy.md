---
id: cargo-clippy
title: cargo clippy
type: command
verified: 2026-08-02
volatility: low

tool: cargo
command: cargo clippy
shell: any

does: >
  Checks a Rust project for code that compiles but is wrong, clumsy, or slower than it needs to
  be, and explains each finding.

flags:
  - flag: "--fix"
    means: >
      Applies the fixes it is confident about directly to your files. It refuses to run on a
      repository with uncommitted changes unless you also pass `--allow-dirty`, which is the
      tool protecting you.
  - flag: "-- -D warnings"
    means: >
      Treats every warning as an error, so the command fails instead of printing advice you can
      ignore. This is what continuous integration usually runs. The bare `--` separates clippy's
      own flags from the compiler flags after it.
  - flag: "--all-targets"
    means: Also checks tests, examples, and benchmarks, which are otherwise skipped.
  - flag: "--workspace"
    means: Checks every package in a multi-package repository rather than only the current one.

expect: >
  Warnings in the compiler's format, each with a file and line, an explanation, and usually a
  suggested replacement plus a link to the rule. `Finished` with no warnings means the project
  is clean.

see_also:
  - cargo-fmt
  - cargo-build
  - cargo-test
  - rust
  - h4-what-good-looks-like

keywords:
  - rust linter
  - clippy warnings
  - improve my rust code
  - idiomatic rust
---

Worth running on anything an agent wrote. Clippy catches the specific category agents produce
most: code that works, compiles, and does something in a roundabout way because the model
pattern-matched an older idiom.
