---
id: cargo-build
title: cargo build
type: command
verified: 2026-08-02
volatility: low

tool: cargo
command: cargo build
shell: any

does: >
  Compiles a Rust project and everything it depends on into an executable, placing the result
  in the `target` folder.

flags:
  - flag: "--release"
    means: >
      Builds an optimized version, which runs much faster and takes considerably longer to
      compile. The output moves from `target\debug` to `target\release`. Use it for anything you
      hand to someone else, and skip it while you are working.
  - flag: "-p <package-name>"
    means: Builds one package inside a workspace rather than all of them.
  - flag: "--all-targets"
    means: Also compiles tests, examples, and benchmarks, which catches errors an ordinary build misses.
  - flag: "--offline"
    means: Refuses to reach the network and uses only already-downloaded dependencies.

expect: >
  A `Compiling` line per dependency, then `Finished` with the profile name and a time. Warnings
  are printed in yellow and do not stop the build. An error stops it and prints the file and
  line.

see_also:
  - cargo-run
  - cargo-test
  - cargo-clippy
  - rust
  - i3-builds-and-artifacts

keywords:
  - compile rust
  - build the project
  - target folder
  - rust build errors
---

The first build of a project downloads and compiles every dependency and can take minutes.
Later builds only recompile what changed and are much faster.

The `target` folder is build output. It is regenerable, it is always in `.gitignore`, and
deleting it is always safe.
