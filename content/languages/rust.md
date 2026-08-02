---
id: rust
title: Rust
type: language
verified: 2026-08-02
volatility: low

name: Rust
aka: [rustlang, rs, rust-lang]
family: compiled
likelihood: likely
extensions: ['.rs']

# Every `note` below is rendered as the evidence line in the identifier, which is
# why each one contrasts against a neighbour language rather than just describing
# what the token does. "fn declares a function" is trivia. "Only Rust uses exactly
# fn, Go uses func" is the thing that teaches recognition.
tells:
  - pattern: 'fn'
    kind: token
    weight: 9
    note: >
      Only Rust uses exactly `fn`. Go uses `func`, Kotlin uses `fun`, Python and
      Ruby use `def`, JavaScript uses `function`.
  - pattern: 'let mut'
    kind: regex
    weight: 10
    note: >
      Rust variables cannot be changed unless you say `mut`, so this pair shows up
      constantly. No other common language has it.
  - pattern: '::'
    kind: operator
    weight: 5
    note: >
      The separator between modules and types, as in `std::io::Write`. C++ also uses
      it; Go, Python, and JavaScript never do.
  - pattern: '->'
    kind: operator
    weight: 4
    note: >
      Sits before a return type: `fn add(a: i32, b: i32) -> i32`. PHP uses `->` for
      member access instead, which reads completely differently.
  - pattern: '#\['
    kind: regex
    weight: 9
    note: >
      A hash followed by a square bracket is a Rust attribute, usually
      `#[derive(Debug, Clone)]` sitting on the line above a type.
  - pattern: '\w+!\s*\('
    kind: regex
    weight: 8
    note: >
      An exclamation mark on a call, as in `println!` or `vec!`, means it is a macro.
      Nothing else in common use looks like this.
  - pattern: 'impl'
    kind: token
    weight: 7
    note: >
      Attaches behavior to a type. Go uses bare methods with receivers, Java and C#
      put methods inside the class body.

rules_out:
  - pattern: 'def'
    because: Python or Ruby
  - pattern: 'func'
    because: Go or Swift
  - pattern: 'function'
    because: JavaScript, TypeScript, or PHP
  - pattern: 'public static void'
    because: Java or C#
  - pattern: '<?php'
    kind: regex
    because: PHP
  - pattern: '#include'
    kind: line_start
    because: C or C++

project_fingerprint:
  manifests:
    - file: Cargo.toml
      decisive: true
      note: >
        If this file is at the root, the project is Rust. The single most reliable
        identifier on this card.
  lockfiles: [Cargo.lock]
  build_dirs: [target/]
  entry_points: [src/main.rs, src/lib.rs]

shape:
  blocks: braces
  statement_end: semicolon
  comment_line: '//'
  comment_block: '/* */'
  string_quotes: >
    Double quotes only. Single quotes mean exactly one character, which is a
    different type.
  naming: snake_case for functions and variables, CamelCase for types, SCREAMING_SNAKE_CASE for constants
  import_keyword: use

tooling:
  package_manager: Cargo
  registry: crates.io
  runtime: none, it compiles to a standalone .exe
  install_command: cargo add <crate-name>
  run_command: cargo run
  test_command: cargo test

confusable_with:
  - language: go
    settle_it: >
      Go declares with `:=` and functions with `func`. Rust uses `let` and `fn`, and
      uses `::`, which Go never does. A brace language with no semicolons anywhere is
      Go, because Rust ends statements with them.
    tiebreak: { pattern: ':=', kind: operator, favors: go }
  - language: cpp
    settle_it: >
      Both use `::` and angle brackets. C++ has `#include <...>` at the top and
      `std::cout <<` for output. Rust has `use ...;` and `println!`. The exclamation
      mark has no C++ equivalent.
    tiebreak: { pattern: '#include', kind: line_start, favors: cpp }
  - language: typescript
    settle_it: >
      Both annotate types after a colon. TypeScript writes `function` or `=>` and
      imports with `import ... from`. Rust writes `fn` and `use`.
    tiebreak: { pattern: 'import .* from', kind: regex, favors: typescript }

errors_look_like:
  sample: |
    error[E0382]: borrow of moved value: `name`
     --> src/main.rs:5:20
      |
    3 |     let name = String::from("nyx");
      |         ---- move occurs because `name` has type `String`
  recognize_by: >
    Three things: an error code in square brackets like `error[E0382]`, an arrow line
    giving file:line:column, and a diagram drawn from pipes and dashes pointing at the
    exact characters. No other language draws pictures in its errors.
  patterns:
    - 'error\[E\d{4}\]'
    - '^\s*--> .*\.rs:\d+:\d+'

meet_it_when: >
  An agent picks it when you ask for something fast and self-contained, you clone a
  tool from GitHub to build yourself, or you go reading the source of a tool that is
  misbehaving. Several tools you already use are written in it.

what_agents_get_wrong: >
  Agents write Rust that does not compile more often than they write Python that does
  not run, because the borrow checker enforces rules that are invisible in the shape of
  the code. This works in your favor: the compiler catches the mistake before you do,
  and the error is specific. Expect a build-fix-build loop and let the agent run it.
  The thing to watch in a diff is `.unwrap()` and `.clone()` sprinkled everywhere. That
  is how an agent gets past the borrow checker without thinking about it. It compiles,
  it runs, and it is the Rust equivalent of duct tape. Ask why each one is there.

version_landscape: >
  Rust does not break your code across versions. It uses editions (2015, 2018, 2021,
  2024) declared in `Cargo.toml`, and old editions keep compiling forever. An answer you
  find from 2019 is probably still correct, which is not true of most of this deck.

see_also:
  - go
  - cpp
  - c3-what-running-means
  - g2-package-managers
  - f1-how-to-read-an-error-message

keywords: [borrow checker, crate, cargo, ownership, rustc, rustup]
---

A compiled systems language that refuses to build your program until it can prove the
memory handling is safe.

Pronounced as it looks. Its packages are called **crates**, which will sound wrong for
about a week.

## The shape

Blocks use curly braces. Statements end with a semicolon. Indentation is four spaces and
means nothing to the compiler, unlike Python where it means everything.

One quirk that confuses everyone at first: a line at the end of a block with **no**
semicolon is the block's return value. The missing semicolon is deliberate, not a typo.

```rust
let count = 3;              // cannot be changed
let mut total = 0;          // can be changed
const MAX: u32 = 100;       // fixed at compile time

fn add(a: i32, b: i32) -> i32 {
    a + b                   // no semicolon: this is the answer
}
```

Comments are `//` for a line and `/* */` for a block. Two extra kinds you will see in real
code: `///` documents the item directly below it, and `//!` documents the whole file.

The program starts at `fn main()` inside `src\main.rs`. That is the entry point.

## Six lines of it

```rust
use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("nyx".to_string(), 10);
    println!("{:?}", scores.get("nyx"));
}
```

## What it is for

Command-line tools, game engines, web servers where speed matters, WebAssembly,
operating-system-level work, and the insides of tools you already use. `ripgrep` is Rust.
So is the Tauri framework this app is built on.

Rust compiles ahead of time into a single `.exe`. There is no runtime to install on the
machine you ship to, which is a real advantage over Python and JavaScript and the reason it
keeps showing up in desktop tooling. You install the whole toolchain with `rustup`, which
also manages compiler versions.

## Reading its errors

Rust's compiler is unusually helpful and usually tells you the fix at the bottom of the
message. Read the whole thing before changing anything: the first line names the problem,
but the last line is frequently a working suggestion you can paste.
