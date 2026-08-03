---
id: cargo-add
title: cargo add
type: command
verified: 2026-08-02
volatility: low

tool: cargo
command: cargo add <crate-name>
shell: any

does: >
  Adds a Rust dependency to the project by writing it into `Cargo.toml` at the newest
  compatible version.

flags:
  - flag: "<crate-name>"
    means: >
      The package name as published. Rust calls its packages crates, and they all live at
      crates.io.
  - flag: "@<version>"
    means: Pins a version, as in `cargo add serde@1.0.200`, instead of taking the newest.
  - flag: '--features "<feature-list>"'
    means: >
      Turns on optional parts of the crate, as in `cargo add serde --features derive`. Many
      crates ship most of their usefulness behind a feature flag, and the compile error when
      one is missing rarely says so.
  - flag: "--dev"
    means: Records it under `dev-dependencies`, meaning it is used by tests and not by the program itself.
  - flag: "--no-default-features"
    means: Turns off the crate's default feature set, which is how you shrink a dependency to only what you use.

expect: >
  A block showing the crate, the version chosen, and a list of its features with the enabled
  ones marked. `Cargo.toml` is updated immediately, and the download happens on your next build.

see_also:
  - cargo-build
  - rust
  - g1-what-a-dependency-is
  - g7-dependency-risk

keywords:
  - add a rust crate
  - install dependency rust
  - cargo toml dependency
  - feature flag
---

Check the crate name on crates.io before running this, particularly when an agent suggested it.
An invented crate name that somebody has since registered is a real way for hostile code to
reach your machine.
