---
id: rust-e0433-failed-to-resolve
title: "error[E0433]: failed to resolve: use of undeclared crate"
type: error
verified: 2026-08-02
volatility: low

language: rust
category: not-found

# Lists the crates this project depends on as a tree. The one you are importing
# has to appear near the top level.
verify: cargo tree --depth 1

sample: |
  PS C:\Users\you\dev\tool> cargo run
     Compiling tool v0.1.0 (C:\Users\you\dev\tool)
  error[E0433]: failed to resolve: use of undeclared crate or module `rand`
   --> src/main.rs:1:5
    |
  1 | use rand::Rng;
    |     ^^^^ use of undeclared crate or module `rand`

  For more information about this error, try `rustc --explain E0433`.
  error: could not compile `tool` (bin "tool") due to 1 previous error

patterns:
  - 'error\[E0433\]'
  - "failed to resolve"
  - "use of undeclared crate or module"
  - "maybe a missing crate"

means: >
  Your code refers to a crate that this project does not depend on. A crate is Rust's word for a
  package, and every dependency has to be listed in `Cargo.toml` before any code can use it.
  Writing the `use` line does not add the dependency. Nothing is broken and nothing is
  corrupt: a name is missing from a list.

fix_ladder:
  - try: Add the crate to the project.
    command: cargo add rand
    shell: powershell
    why: >
      Assumes the crate is real and not listed yet. This writes the dependency into
      `Cargo.toml` with a current version and downloads it. It is the whole fix the large
      majority of the time.

  - try: Check what the project already depends on.
    command: Get-Content Cargo.toml
    shell: powershell
    why: >
      Assumes the name is close but not exact. Crate names use hyphens and code uses
      underscores, so the dependency `serde-json` is written `serde_json` in a `use` line. That
      mismatch looks like a missing crate and is not.

  - try: Confirm the crate exists under that name.
    command: cargo search rand
    shell: powershell
    why: >
      Assumes the agent invented it. This searches the public registry and prints matching names
      with descriptions. No results means the crate does not exist, and an import written against
      a plausible-sounding name is a real agent failure rather than a rare one.

  - try: Check whether it needs a feature turned on.
    command: cargo add tokio --features full
    shell: powershell
    why: >
      Assumes the crate is present and the specific module inside it is not. Many crates ship
      most of their contents switched off, and using one of those parts requires naming the
      feature in `Cargo.toml`. The crate's own page lists which features exist.

  - try: Check whether you meant one of your own modules.
    why: >
      Assumes the name is a file in your project rather than a package. A file at `src/utils.rs`
      needs `mod utils;` declared in `main.rs` before `use utils::thing` works. Rust does not
      pick up files automatically the way Python and JavaScript do.

if_none_worked: >
  Paste the whole error, your entire `Cargo.toml`, and the first ten lines of the file that
  failed. The `Cargo.toml` is the piece people leave out because the error is about code, and it
  is the file that actually decides whether the crate exists for this project.

see_also:
  - g1-what-a-dependency-is
  - g2-package-managers
  - g7-dependency-risk
  - rust

keywords:
  - E0433
  - undeclared crate
  - failed to resolve
  - cargo add
  - missing dependency rust
---

Rust splits this into two steps where other languages have one. Listing the dependency in
`Cargo.toml` makes it available, and the `use` line brings a name into scope. Miss the first and
you get this error.

`cargo add` is the command that does the first step properly. Editing `Cargo.toml` by hand works
too and means picking a version yourself.

Two lookalikes are worth telling apart. `error[E0432]: unresolved import` means the crate is
there and the specific path inside it is wrong, usually because a type moved between versions.
This error, E0433, means the crate is not there at all.

The hyphen and underscore rule catches people once and then never again. On the registry and in
`Cargo.toml`, crates are written with hyphens. In code, every hyphen becomes an underscore.
