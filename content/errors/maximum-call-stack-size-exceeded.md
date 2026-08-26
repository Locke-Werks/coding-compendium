---
id: maximum-call-stack-size-exceeded
title: "RangeError: Maximum call stack size exceeded"
type: error
verified: 2026-08-02
volatility: low

language: javascript
category: broke-at-runtime

sample: |
  PS C:\Users\you\dev\site> node index.js
  C:\Users\you\dev\site\src\tree.js:8
      return countChildren(node);
             ^

  RangeError: Maximum call stack size exceeded
      at countChildren (C:\Users\you\dev\site\src\tree.js:8:12)
      at countChildren (C:\Users\you\dev\site\src\tree.js:8:12)
      at countChildren (C:\Users\you\dev\site\src\tree.js:8:12)
      at countChildren (C:\Users\you\dev\site\src\tree.js:8:12)
      at countChildren (C:\Users\you\dev\site\src\tree.js:8:12)

  Node.js v22.11.0

patterns:
  - "Maximum call stack size exceeded"
  - "RangeError:"
  - "too much recursion"

means: >
  A function called itself, or called something that called it back, and never stopped. Every
  call in progress takes a slot in a fixed-size region of memory called the call stack. When
  the slots run out the program stops. The stack trace is the giveaway: the same function name
  repeats over and over, which is a shape no ordinary error produces.

fix_ladder:
  - try: Read the repeating name in the stack trace.
    why: >
      Assumes you need to find the loop, which the error hands you directly. One name
      repeating is a function calling itself. Two names alternating is a pair calling each
      other, which is harder to spot in the code and just as common.

  - try: Find the stopping condition and check whether it can ever be true.
    why: >
      Assumes the function is meant to recurse and the exit is broken. Every self-calling
      function needs a case where it returns without calling itself. A check for an empty list
      that tests the wrong variable, or a counter that never reaches its limit, both produce
      infinite recursion that reads as correct.

  - try: Check whether the value passed in actually gets smaller.
    command: console.log('depth', node.id)
    shell: powershell
    why: >
      Assumes the exit condition is right and the input never approaches it. A function
      recursing over a tree that contains a loop back to itself never runs out of nodes. This
      prints what is being passed each time, and a repeating value proves the loop.

  - try: Look for a property that calls itself.
    why: >
      Assumes the recursion is not in an obvious function. A getter that reads its own
      property, a `toString` that formats an object containing itself, or two React components
      rendering each other all produce this without any function visibly calling itself.

  - try: Check for a circular structure being copied or serialized.
    why: >
      Assumes the loop is in the data rather than in your code. An object containing a
      reference back to itself makes deep copies and `JSON.stringify` recurse forever. Node
      usually catches that case with a "Converting circular structure" message, and a
      hand-written copy function does not.

if_none_worked: >
  Paste the top of the error plus about ten lines of the repeated stack entries, and the full
  source of the repeating function. Keeping several repeats matters: it shows whether one
  function loops on itself or two alternate, which is the first thing an agent needs and the
  first thing people trim because the lines look identical.

see_also:
  - f2-stack-traces
  - c5-processes-and-killing-them
  - javascript

keywords:
  - maximum call stack size exceeded
  - RangeError
  - infinite recursion
  - stack overflow
  - too much recursion
---

The call stack is a list of what is currently running. Function A calls B, B calls C, and the
stack holds all three until C finishes. It has a fixed size, around ten thousand entries in
Node, and this error means it filled up.

Nearly always that means recursion with no way out. Occasionally it means legitimate
recursion that goes too deep, such as walking a very large tree, and the fix there is to
rewrite the walk as a loop rather than to make the stack bigger.

The repeated stack lines are the diagnostic. Ordinary errors produce a stack of different
function names. This one produces the same name printed until Node gives up truncating it.

Browsers word it differently. Chrome and Node say "Maximum call stack size exceeded", Firefox
says "too much recursion", and Safari says "RangeError: Maximum call stack size exceeded".
Same failure.
