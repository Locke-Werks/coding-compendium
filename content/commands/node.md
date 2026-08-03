---
id: node
title: node
type: command
verified: 2026-08-02
volatility: low

tool: node
command: node <file>
shell: any

verify: node --version

does: >
  Runs a JavaScript file outside a browser, using the Node.js runtime installed on your
  machine.

flags:
  - flag: "<file>"
    means: >
      The JavaScript file to execute, such as `node index.js`. The `.js` extension can be left
      off in some cases, and including it always works.
  - flag: "-v"
    means: >
      Prints the version and exits. `node -v` is the standard check for whether Node.js is
      installed at all.
  - flag: '-e "<code>"'
    means: Runs a snippet given on the command line instead of a file, which is handy for a one-line check.
  - flag: "--watch"
    means: Restarts the program automatically when the file changes. Built in since Node.js 18, so no extra tool is needed.
  - flag: "with no arguments"
    means: >
      Opens an interactive prompt where you type JavaScript one line at a time and see the result
      immediately. Leave it with `.exit` or Ctrl+D.

expect: >
  Whatever your program prints, and nothing else. A program that finishes without printing
  produces no output at all, which means it worked.

see_also:
  - npx
  - npm-run
  - javascript
  - c3-what-running-means

keywords:
  - run javascript file
  - node not recognized
  - node version
  - javascript outside the browser
---

`node` is the runtime, `npm` is its package manager, and they install together. If `node -v`
works and `npm -v` does not, something went wrong with the install rather than with your
project.

An error reading `Cannot find module './thing'` is a path problem in your code. An error reading
`Cannot find module 'express'` is a missing dependency, and `npm install` is the fix.
