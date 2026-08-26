---
id: typeerror-cannot-read-properties-of-undefined
title: "TypeError: Cannot read properties of undefined"
type: error
verified: 2026-08-02
volatility: low

language: javascript
category: broke-at-runtime

sample: |
  PS C:\Users\you\dev\site> node index.js
  C:\Users\you\dev\site\src\cart.js:22
      return user.profile.email;
                          ^

  TypeError: Cannot read properties of undefined (reading 'email')
      at getEmail (C:\Users\you\dev\site\src\cart.js:22:25)
      at Object.<anonymous> (C:\Users\you\dev\site\index.js:8:1)
      at Module._compile (node:internal/modules/cjs/loader:1364:14)

  Node.js v22.11.0

patterns:
  - "Cannot read properties of undefined"
  - "Cannot read property .* of undefined"
  - "Cannot read properties of null"
  - "TypeError:"

means: >
  Your code reached into something expecting an object and found `undefined` instead.
  `undefined` is what JavaScript gives you when a value was never set: a missing object key,
  a function that returned nothing, an array index past the end. Reading a property off it
  is impossible, so execution stops. The name in parentheses at the end is the property you
  asked for, and the thing to its left is what was empty.

fix_ladder:
  - try: Work out which part of the expression is empty.
    why: >
      Assumes you are reading the error backward, which is easy to do. In
      `Cannot read properties of undefined (reading 'email')`, the empty thing is not `email`
      and not `user`. It is `user.profile`. Whatever sits immediately to the left of the
      named property is what came back empty.

  - try: Print it just above the failing line.
    command: console.log('user is', user)
    shell: powershell
    why: >
      Assumes you need to see the real shape of the data. This is the fastest debugging tool
      in JavaScript and beginners skip it because it feels crude. Run the code again and read
      what actually came back rather than what you expected.

  - try: Check the spelling and capitalization of every key in the chain.
    why: >
      Assumes the object is fine and one key name is wrong. Reading a key that does not exist
      returns `undefined` silently, with no error, so the failure surfaces one step later at
      the next property access. `user.Profile` and `user.profile` are different keys.

  - try: Check whether the data has arrived yet.
    why: >
      Assumes timing. A value fetched from the network or a database is `undefined` until the
      request finishes. Code that reads it synchronously, or a component that renders before
      data loads, hits exactly this. The fix is to wait for the value or to render a
      placeholder while it is missing.

  - try: Guard the access with optional chaining.
    command: return user?.profile?.email ?? null
    shell: powershell
    why: >
      Assumes empty is a legitimate state rather than a bug. The `?.` stops and returns
      `undefined` instead of throwing, and `??` supplies a fallback. Use this when missing
      data is expected. Do not use it to silence an error whose cause you have not found,
      because it converts a loud failure into a quiet wrong answer.

if_none_worked: >
  Paste the whole error including the source line and caret, the function it points at, and
  an example of the data you expected. The data shape is the piece nobody includes and it is
  the entire question. If the value comes from an interface or a file, paste a sample of the
  real response rather than describing it.

see_also:
  - f2-stack-traces
  - f1-how-to-read-an-error-message
  - h4-what-good-looks-like
  - javascript

keywords:
  - cannot read properties of undefined
  - cannot read property of undefined
  - undefined is not an object
  - optional chaining
  - TypeError javascript
---

This is the most common runtime error in JavaScript, and almost all of it comes from one
design decision: reading a key that does not exist gives you `undefined` rather than an
error.

That means the failure surfaces one step downstream of the actual mistake. Your typo is in
`user.profil`, and the crash happens at `.email` on the next line along. The error names the
second one. The bug is in the first.

Optional chaining with `?.` is genuinely useful and genuinely overused. Agents reach for it
to make an error go away, which turns a crash into a page that renders with blank fields and
no complaint. Use it where empty is a real possibility you have thought about, and fix the
data everywhere else.

`Cannot read properties of null` is the same error with a different cause. `null` means
somebody deliberately set it to nothing, while `undefined` usually means nobody set it at
all.
