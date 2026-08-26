---
id: rust-e0502-cannot-borrow-as-mutable
title: "error[E0502]: cannot borrow as mutable"
type: error
verified: 2026-08-02
volatility: low

language: rust
category: wont-compile

# Checks without building a program. Same errors, much faster.
verify: cargo check

sample: |
  PS C:\Users\you\dev\tool> cargo run
     Compiling tool v0.1.0 (C:\Users\you\dev\tool)
  error[E0502]: cannot borrow `items` as mutable because it is also borrowed as immutable
   --> src/main.rs:5:5
    |
  4 |     let first = &items[0];
    |                  ----- immutable borrow occurs here
  5 |     items.push(4);
    |     ^^^^^^^^^^^^^ mutable borrow occurs here
  6 |     println!("{}", first);
    |                    ----- immutable borrow later used here

  For more information about this error, try `rustc --explain E0502`.
  error: could not compile `tool` (bin "tool") due to 1 previous error

patterns:
  - 'error\[E0502\]'
  - "cannot borrow"
  - "as mutable because it is also borrowed as immutable"
  - "immutable borrow occurs here"
  - "mutable borrow occurs here"

means: >
  Rust allows either one writer or any number of readers at a time, never both. Your code holds
  a reader, `first`, which points into `items`. Then it changes `items` while that reader is
  still in use. Growing a list can move its contents somewhere else in memory, which would leave
  the reader pointing at nothing, so the compiler refuses. Nothing ran.

fix_ladder:
  - try: Read the three marked lines in the diagram.
    why: >
      Assumes the answer is already on screen. Rust marks where the read borrow starts, where
      the write happens, and where the read borrow is last used. The overlap between the first
      and third is the problem, and shrinking it is what every fix below does.

  - try: Move the use of the reader above the change.
    why: >
      Assumes you can reorder. If the `println!` runs before the `push`, the read borrow is
      finished by the time the write starts and the conflict disappears. Rust ends a borrow at
      its last use, not at the end of the block, so this works more often than it sounds like it
      would.

  - try: Copy out the value instead of pointing at it.
    command: let first = items[0];
    shell: powershell
    why: >
      Assumes the item is a simple value such as a number. Dropping the ampersand copies it, so
      nothing is borrowing the list afterward. For a `String` or a struct, use `.clone()`
      instead, which has the same effect at a small cost.

  - try: Put the read inside its own block.
    command: 'let first = { items[0] };'
    shell: powershell
    why: >
      Assumes you need the borrow briefly in the middle of a longer function. A borrow ends when
      the block holding it ends, so braces around the reading part make its lifetime explicit
      and short.

  - try: Collect what you need before the loop, if this is inside one.
    why: >
      Assumes the error is inside a `for` loop over the same collection you are changing. Rust
      will not let you add to a list while walking it. Build a second list of the changes you
      want during the walk, then apply them after it ends.

if_none_worked: >
  Paste the whole error including the underline diagram, and the entire function it points at
  rather than the three marked lines. The function matters because the fix is nearly always about
  reordering, and an agent cannot reorder what it cannot see.

see_also:
  - c2-compiled-vs-interpreted
  - f1-how-to-read-an-error-message
  - rust

keywords:
  - E0502
  - cannot borrow as mutable
  - borrow checker
  - immutable borrow
  - mutable borrow conflict
---

This is the borrow checker, the part of Rust people mean when they say the compiler argues with
them.

The rule is one line: many readers or one writer, never both at once. Everything else follows.
It exists because a list that grows can relocate in memory, and a pointer into the old location
would then be reading whatever happens to be there now. That bug is common in C and impossible
in safe Rust.

The good news is that the compiler ends a borrow at its last use rather than at the end of the
enclosing block. That means reordering two lines fixes a surprising number of these, and the
diagram tells you which two.

Agents produce this error often, usually by writing a loop that changes the collection it is
walking. That pattern is normal in Python and JavaScript, where it causes subtle bugs instead of
compile errors.
