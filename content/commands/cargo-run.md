---
id: cargo-run
title: cargo run
type: command
verified: 2026-08-02
volatility: low

tool: cargo
command: cargo run
shell: any

does: >
  Compiles a Rust project if anything changed and then runs the resulting program, in one
  step.

flags:
  - flag: "--release"
    means: >
      Builds and runs the optimized version. Slower to compile, much faster to execute. Needed
      when you are measuring performance, since a debug build can be an order of magnitude
      slower.
  - flag: "--bin <name>"
    means: >
      Picks which program to run when the project defines several. Without it, a project with
      more than one executable stops and lists your options.
  - flag: "-- <arguments>"
    means: >
      Everything after the bare `--` is handed to your program instead of to cargo. This is how
      you pass command-line arguments, as in `cargo run -- --input data.csv`.

expect: >
  Build output, then a `Running` line naming the executable it built, such as
  `target\debug\myproject.exe`, then whatever your program prints. Nothing extra is printed
  when the program finishes successfully.

see_also:
  - cargo-build
  - cargo-test
  - rust
  - c3-what-running-means

keywords:
  - run rust program
  - execute my code
  - cargo run arguments
  - which binary
---

If it fails with `error: could not find Cargo.toml`, you are not inside a Rust project. Cargo
looks in the current folder and every folder above it, so move into the project root first.

The bare `--` catches people out. Without it, cargo reads your program's flags as its own and
complains about an argument it does not recognize.
