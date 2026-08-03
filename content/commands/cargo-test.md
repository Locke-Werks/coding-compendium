---
id: cargo-test
title: cargo test
type: command
verified: 2026-08-02
volatility: low

tool: cargo
command: cargo test
shell: any

does: >
  Compiles and runs every test in a Rust project, then reports how many passed and how many
  failed.

flags:
  - flag: "<name-filter>"
    means: >
      Runs only tests whose name contains that text, as in `cargo test login`. No wildcards
      needed, it is a plain substring match.
  - flag: "-- --nocapture"
    means: >
      Shows output printed by passing tests, which cargo hides by default. The first `--`
      separates cargo's flags from the test runner's. Essential when you are debugging with
      print statements.
  - flag: "-- --test-threads=1"
    means: >
      Runs tests one at a time instead of in parallel. Use it when tests interfere with each
      other, which usually means they share a file or a port.
  - flag: "--release"
    means: Tests the optimized build. Slower to compile, and it catches the rare bug that only appears with optimizations on.

expect: >
  A line per test reading `test tests::name ... ok`, then a summary such as
  `test result: ok. 12 passed; 0 failed; 0 ignored`. A failure prints the assertion, the
  expected value, and the actual value.

see_also:
  - cargo-build
  - cargo-clippy
  - rust
  - h1-what-a-test-is
  - h6-when-tests-lie

keywords:
  - run rust tests
  - test result failed
  - cargo test output
  - print in tests
---

Zero failures is not the same as good coverage. Check that the number of tests is plausible for
the amount of code, and be suspicious when an agent's change makes a failing test pass without
touching the code the test covers.
