---
id: rust-e0308-mismatched-types
title: "error[E0308]: mismatched types"
type: error
verified: 2026-08-02
volatility: low

language: rust
category: wont-compile

# Checks types without building a program. Faster than a full build and
# reports exactly the same errors.
verify: cargo check

sample: |
  PS C:\Users\you\dev\tool> cargo run
     Compiling tool v0.1.0 (C:\Users\you\dev\tool)
  error[E0308]: mismatched types
   --> src/main.rs:4:18
    |
  4 |     let count: i32 = "5";
    |                ---   ^^^ expected `i32`, found `&str`
    |                |
    |                expected due to this

  For more information about this error, try `rustc --explain E0308`.
  error: could not compile `tool` (bin "tool") due to 1 previous error

patterns:
  - 'error\[E0308\]'
  - "mismatched types"
  - "expected .*, found"
  - "expected struct"
  - "expected enum"

means: >
  A value's type does not match what that position requires. Rust checks every type before
  producing a program, so this is caught at compile time rather than at runtime. The error names
  both sides: what was expected and what it found. Nothing ran, and no program was produced.

fix_ladder:
  - try: Read the expected and found types in that order.
    why: >
      Assumes the answer is already stated. "expected `i32`, found `&str`" means the slot needs a
      number and you supplied text. The arrows in the diagram point at what set the expectation,
      which is usually a type annotation or a function signature further along the line.

  - try: Convert between the two types explicitly.
    command: 'let count: i32 = "5".parse().unwrap();'
    shell: powershell
    why: >
      Assumes the value is genuinely the wrong type and needs converting. Rust never converts for
      you. Text to number is `.parse()`, number to text is `.to_string()`, and `&str` to `String`
      is `.to_string()` as well.

  - try: Check for a missing or extra reference.
    why: >
      Assumes the types are nearly right. "expected `&String`, found `String`" means you need an
      ampersand, and the reverse means you have one too many. This is the most common version of
      this error and the underlines in the diagram show which side needs changing.

  - try: Look at whether something returned a wrapper.
    why: >
      Assumes the value is inside a `Result` or an `Option`. "expected `i32`, found
      `Result<i32, ParseIntError>`" means the operation can fail and handed back a box describing
      either outcome. Getting the value out means `?` inside a function that returns `Result`, or
      `.unwrap()` to stop the program if it failed.

  - try: Let the compiler tell you what a value's type actually is.
    command: 'let _: () = value;'
    shell: powershell
    why: >
      Assumes you cannot work out what you are holding. Assigning to the empty type fails on
      purpose, and the error message names the real type of `value`. Delete the line afterward.
      This is a debugging trick rather than code.

if_none_worked: >
  Paste the entire error including the diagram and any `help:` block, plus the signature of the
  function involved. The signature is what people leave out, and the expected type nearly always
  comes from it rather than from the line the error points at.

see_also:
  - c2-compiled-vs-interpreted
  - f1-how-to-read-an-error-message
  - rust

keywords:
  - E0308
  - mismatched types
  - expected found rust
  - type error rust
  - parse unwrap
---

This is the most common Rust error there is, and it is the compiler doing the job the language
exists for.

Rust does no automatic conversion at all. A whole number is not a decimal, a `String` is not a
`&str`, and a value that might be missing is a different type from one that cannot be. Other
languages paper over these differences and produce surprises at runtime.

The `Result` version is worth recognizing on sight because it is so frequent. Any operation that
can fail returns a `Result`, which holds either the value or an error. The compiler will not let
you use it as the plain value until you have said what happens in the failure case, which is
`?`, `.unwrap()`, or a `match`.

The diagram is the useful part. Rust underlines the offending expression and separately marks
what created the expectation, and those are usually on different lines.
