---
id: javascript-referenceerror-not-defined
title: "ReferenceError: x is not defined"
type: error
verified: 2026-08-02
volatility: low

language: javascript
category: broke-at-runtime

sample: |
  PS C:\Users\you\dev\site> node index.js
  C:\Users\you\dev\site\src\cart.js:14
    const total = subtotal + taxRate;
                             ^

  ReferenceError: taxRate is not defined
      at calculateTotal (C:\Users\you\dev\site\src\cart.js:14:28)
      at Object.<anonymous> (C:\Users\you\dev\site\index.js:5:1)
      at Module._compile (node:internal/modules/cjs/loader:1364:14)

  Node.js v22.11.0

patterns:
  - "ReferenceError:"
  - "is not defined"

means: >
  Your code used a name that does not exist at that point in the program. Not a name holding
  nothing, a name that was never created at all. The code ran until it reached that line and
  stopped there, so everything before it worked. The usual causes are a typo, a variable
  declared inside a different function or block, or a browser-only name used in code running
  under Node.

fix_ladder:
  - try: Read the name in the error and compare it to where you defined it.
    why: >
      Assumes a typo or a capitalization difference. JavaScript names are case sensitive, so
      `taxrate` and `taxRate` are two different names, and nothing warns you when you create
      the second by accident. The error names the exact spelling it went looking for.

  - try: Check where the variable was declared.
    why: >
      Assumes a scope problem. A `const` or `let` declared inside `{ }` exists only inside
      those braces. Declared inside a function, it exists only inside that function. Code
      that reads it from outside gets this error even though the declaration is right there
      on screen.

  - try: Look at whether the name belongs to a browser.
    why: >
      Assumes browser code running under Node. `window`, `document`, `localStorage`, and
      `alert` exist in a browser and nowhere else. Node has none of them. Agents mix the two
      environments constantly, and the code looks completely normal.

  - try: Check whether you meant to import it.
    why: >
      Assumes a missing import line. A function from another file has to be imported by name
      at the top of the file that uses it. Deleting an import that looked unused, or an agent
      writing a call before adding the import, both produce this.

  - try: Check the order things run in.
    why: >
      Assumes the name exists but not yet. A `const` cannot be read on a line above where it
      is declared, and a variable assigned inside a callback that has not fired yet is not
      there when synchronous code looks for it.

if_none_worked: >
  Paste the whole error including the two lines showing the source and the caret, the entire
  file it points at, and the command you ran. The caret line is what people trim as
  decoration, and it marks the exact column. The whole file matters more than the one line,
  because the answer is nearly always about where something was declared rather than where it
  was used.

see_also:
  - f2-stack-traces
  - f1-how-to-read-an-error-message
  - javascript

keywords:
  - ReferenceError
  - is not defined
  - undefined variable
  - variable scope
  - window is not defined
---

Read the first line of the stack trace, not the last. Node prints the innermost call first,
so the top line is the code that actually failed. Python prints them the other way around,
which is a common source of confusion when you move between the two.

The two most common versions have completely different fixes. A typo is a one-character
edit. A scope problem means the variable exists but not where you are standing, and moving
the declaration outward is the fix.

`window is not defined` deserves its own note because it is so common in framework projects.
It means code written for a browser ran on a server during a build or a server render. The
fix is to move that code into an effect or a browser-only guard, rather than to define
`window` yourself.

There is one close relative worth telling apart. `ReferenceError` means the name does not
exist. `TypeError: Cannot read properties of undefined` means the name exists and holds
nothing useful, which is a different problem with a different card.
