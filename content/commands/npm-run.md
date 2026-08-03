---
id: npm-run
title: npm run
type: command
verified: 2026-08-02
volatility: quarterly

tool: npm
command: npm run <script-name>
shell: any

does: >
  Runs one of the named commands defined in the `scripts` section of the project's
  `package.json`.

flags:
  - flag: "<script-name>"
    means: >
      The key from the `scripts` object, such as `dev`, `build`, or `lint`. It is whatever the
      project's author decided to call it, so there is no universal list.
  - flag: "with no script name"
    means: >
      Bare `npm run` prints every script the project defines, along with the command each one
      stands for. This is how you find out what a project can do.
  - flag: "-- <arguments>"
    means: >
      Everything after the bare `--` is passed through to the underlying command rather than
      consumed by npm, as in `npm run test -- --watch`.
  - flag: "--silent"
    means: Hides npm's own output so you see only what the script itself prints.

expect: >
  A line echoing the script, such as `> myproject@0.1.0 dev`, then the command's own output. A
  dev server keeps running and holds the terminal until you press Ctrl+C.

see_also:
  - npm-install
  - npx
  - c3-what-running-means
  - j3-project-layouts

keywords:
  - how do i run this project
  - npm start
  - dev server
  - missing script
  - what scripts are available
---

`npm start` and `npm test` are special: they work without the word `run`. Everything else
needs it.

`npm error Missing script: "dev"` means that name is not in `package.json`. Run bare `npm run`
and read the real list rather than guessing.
