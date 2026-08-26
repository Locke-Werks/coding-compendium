---
id: rust-e0382-borrow-of-moved-value
title: "error[E0382]: borrow of moved value"
type: error
verified: 2026-08-02
volatility: low

language: rust
category: wont-compile

# Checks the code without producing a program. Much faster than a full build
# and reports the same errors.
verify: cargo check

sample: |
  PS C:\Users\you\dev\tool> cargo run
     Compiling tool v0.1.0 (C:\Users\you\dev\tool)
  error[E0382]: borrow of moved value: `name`
   --> src/main.rs:6:20
    |
  3 |     let name = String::from("ada");
    |         ---- move occurs because `name` has type `String`, which does not implement the `Copy` trait
  4 |     let greeting = build(name);
    |                          ---- value moved here
  5 |
  6 |     println!("{}", name);
    |                    ^^^^ value borrowed here after move
    |
  help: consider cloning the value if the performance cost is acceptable
    |
  4 |     let greeting = build(name.clone());
    |                              ++++++++

  For more information about this error, try `rustc --explain E0382`.
  error: could not compile `tool` (bin "tool") due to 1 previous error

patterns:
  - 'error\[E0382\]'
  - "borrow of moved value"
  - "use of moved value"
  - "value moved here"
  - "value borrowed here after move"

means: >
  In Rust, every value has exactly one owner. Passing a value to a function hands ownership over
  unless you deliberately lend it instead, and the original variable is then unusable. Your code
  passed `name` to `build`, which took ownership, and then used `name` again afterward. Nothing
  ran. The compiler refused to produce a program.

fix_ladder:
  - try: Read the three underlined spots in the error.
    why: >
      Assumes the diagnostic already contains the answer, which it usually does. Rust marks
      where the value was created, where it moved, and where you tried to use it after. Those
      three lines are the whole story and you rarely need anything else.

  - try: Lend the value instead of giving it away.
    command: let greeting = build(&name);
    shell: powershell
    why: >
      Assumes the function does not need to own the value. An ampersand passes a reference, a
      borrow rather than a handover, and `name` stays usable afterward. The function signature
      has to accept a reference too, so `fn build(n: String)` becomes `fn build(n: &str)`.

  - try: Take the compiler's suggested clone.
    command: let greeting = build(name.clone());
    shell: powershell
    why: >
      Assumes the function genuinely needs its own copy. `.clone()` makes a second copy so both
      halves have one. Rust names the tradeoff in the help text: cloning costs time and memory.
      For a short string in a program that runs once, that cost is irrelevant.

  - try: Reorder the code so the last use comes before the move.
    why: >
      Assumes you only needed the value briefly. Moving the `println!` above the call to `build`
      makes the problem disappear with no clone and no reference, because the value is still
      owned at that point.

  - try: Read the full explanation for the error code.
    command: rustc --explain E0382
    shell: powershell
    why: >
      Assumes the pattern is new to you and worth learning once. Every Rust error code has a
      written page with examples, available offline in the compiler itself. This one is the
      core rule of the language and reading it once saves a week of guessing.

if_none_worked: >
  Paste the entire error block including the diagram with the underlines and the `help:` section,
  plus the function signature the value was passed to. The diagram is what people trim because it
  looks like ASCII decoration, and it names the exact three lines involved. The signature decides
  whether a reference or a clone is the right answer.

see_also:
  - c2-compiled-vs-interpreted
  - f1-how-to-read-an-error-message
  - rust

keywords:
  - E0382
  - borrow of moved value
  - use after move
  - ownership rust
  - clone
---

This is Rust's central idea showing up as a compiler error, and it is worth understanding
rather than working around.

One value, one owner. When the owner goes away, the memory is freed. That single rule is how
Rust avoids an entire category of bugs that other languages solve with a garbage collector or
do not solve at all. The cost is that the compiler enforces it in your face.

Three ways out, in the order worth trying. Borrow with `&` when the function only needs to
read. Clone when it needs its own copy and the cost does not matter. Restructure when the
value is used in an awkward order.

Cloning to make the compiler stop complaining is a real strategy and not a shameful one. In a
program that runs for a second and exits, the difference is unmeasurable. Learn the borrow
form as you go rather than treating every clone as a defeat.

Agents write Rust that fails this check regularly. The code looks correct and expresses the
right idea, and the ownership does not line up. That is a compile error rather than a bug that
ships, which is the whole argument for the language.
