---
id: cargo-fmt
title: cargo fmt
type: command
verified: 2026-08-02
volatility: low

tool: cargo
command: cargo fmt
shell: any

does: >
  Rewrites every Rust source file in the project into the standard formatting, fixing
  indentation, line breaks, and spacing in place.

flags:
  - flag: "--check"
    means: >
      Reports what would change and rewrites nothing, exiting with a failure code if anything is
      misformatted. This is what continuous integration runs, and it is how you preview the
      change before letting it happen.
  - flag: "-p <package-name>"
    means: Formats one package in a workspace rather than all of them.
  - flag: "-- --emit files"
    means: Passes options through to the underlying formatter for the rare case where you need to control its output mode.

expect: >
  Nothing printed on success, and your files change on disk. `cargo fmt --check` prints a diff
  of what it would alter and says nothing when everything is already correct.

undo: >
  `git restore .` puts every tracked file back to its committed state, which is safe here only
  because formatting is the sole change. Commit your real work first.

see_also:
  - cargo-clippy
  - cargo-build
  - rust
  - d9-reading-a-diff

keywords:
  - format rust code
  - rustfmt
  - fix indentation
  - formatting check failed
---

Run it before committing, not after, or your diff mixes formatting noise with the change you
actually made. A reviewer cannot separate the two once they are in the same commit.

There is one standard Rust style and no configuration argument to have. That is the point of
the tool.
