---
id: node-enoent-no-such-file-or-directory
title: "Error: ENOENT: no such file or directory"
type: error
verified: 2026-08-02
volatility: low

language: javascript
category: not-found

# Prints True if the file is where you think it is, relative to the folder you
# are standing in right now.
verify: Test-Path .\data\config.json

sample: |
  PS C:\Users\you\dev\site> node index.js
  node:internal/fs/utils:355
      throw err;
      ^

  Error: ENOENT: no such file or directory, open 'data/config.json'
      at Object.openSync (node:fs:581:18)
      at Object.readFileSync (node:fs:453:35)
      at loadConfig (C:\Users\you\dev\site\src\config.js:6:26) {
    errno: -4058,
    code: 'ENOENT',
    syscall: 'open',
    path: 'data/config.json'
  }

patterns:
  - "ENOENT"
  - "no such file or directory"
  - "ENOENT: no such file or directory, open"

means: >
  Node asked the operating system for a file at that path and there is nothing there. ENOENT is
  how Node spells "no such entry". The path in the message is exactly what Node asked for, and
  when it does not start with a drive letter it is relative to the folder the program was
  started from rather than to the file containing the code. That distinction causes most of
  these.

fix_ladder:
  - try: Check whether the file is where the error says it looked.
    command: Get-Location; Test-Path .\data\config.json
    shell: powershell
    why: >
      Assumes a relative path resolved from an unexpected folder. Node resolves relative paths
      against the current working directory, which is wherever you ran `node` from. Running
      the same program from the project root and from inside `src` gives two different answers.

  - try: Build the path from the code file instead of from the working directory.
    command: import.meta.dirname
    shell: powershell
    why: >
      Assumes you want the path to mean the same thing no matter where the program is started.
      In modern Node, `import.meta.dirname` is the folder holding the current file, and older
      code uses `__dirname`. Joining your relative path onto that makes it stable.

  - try: Check the name letter by letter, including capitals.
    command: Get-ChildItem .\data
    shell: powershell
    why: >
      Assumes a spelling or case difference. Windows opens `Config.json` when you ask for
      `config.json`, and the Linux server this will eventually run on will not. A listing next
      to the error settles it in one look.

  - try: Look at whether the file is supposed to exist yet.
    why: >
      Assumes the file is generated rather than written by hand. A config file created by a
      setup step, a `.env` file that is deliberately not committed, or a build output folder
      all produce this on a fresh clone. Check the README for a setup step you skipped.

  - try: Check for a missing folder rather than a missing file.
    command: New-Item -ItemType Directory -Force -Path .\data
    shell: powershell
    why: >
      Assumes the program is writing rather than reading. Node does not create parent folders
      for you, so writing to `data/out.json` fails with this same error when `data` does not
      exist. The `syscall` line tells you which: `open` for reading, `mkdir` or `write` for
      writing.

if_none_worked: >
  Paste the whole error including the `path:` and `syscall:` lines, the output of
  `Get-Location`, and the output of `Get-ChildItem` on the folder that should hold the file.
  The working directory is the piece nobody includes and it is the difference between a file
  that is missing and a file that is fine but being looked for from the wrong place.

see_also:
  - c7-files-folders-and-paths
  - f1-how-to-read-an-error-message
  - g5-environment-variables
  - javascript

keywords:
  - ENOENT
  - no such file or directory
  - file not found node
  - readFileSync failed
  - relative path node
---

The single most useful thing to understand here is which folder a relative path counts from.

It counts from where you ran the command, not from where the code lives. A file at
`src/config.js` reading `data/config.json` works when you run `node index.js` from the project
root and fails when you run `node src/index.js` from inside `src`. Nothing about the code
changed.

That is why an agent's command works and yours does not, or the other way round: you are
standing in different folders.

Anchoring the path to the code file removes the ambiguity entirely:

```javascript
import { join } from "node:path";
const configPath = join(import.meta.dirname, "..", "data", "config.json");
```

The other frequent cause is a file that was never meant to be committed. `.env` files hold
secrets and are excluded from git on purpose, so a fresh clone has none. That is working as
intended and the project should say so in its README.
