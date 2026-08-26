---
id: node-cannot-find-module
title: "Error: Cannot find module 'x'"
type: error
verified: 2026-08-02
volatility: low

language: javascript
category: not-found

# Swap in the package name. Silence means Node found it. An error means it is
# still missing from this folder.
verify: node -e "require('express'); console.log('found')"

sample: |
  PS C:\Users\you\dev\site> node index.js
  node:internal/modules/cjs/loader:1215
    throw err;
    ^

  Error: Cannot find module 'express'
  Require stack:
  - C:\Users\you\dev\site\index.js
      at Module._resolveFilename (node:internal/modules/cjs/loader:1212:15)
      at Module._load (node:internal/modules/cjs/loader:1043:27)
      at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:164:12) {
    code: 'MODULE_NOT_FOUND',
    requireStack: [ 'C:\\Users\\you\\dev\\site\\index.js' ]
  }

patterns:
  - "Cannot find module"
  - "MODULE_NOT_FOUND"
  - "Require stack"

means: >
  Node looked for that name and found nothing. What it looked for depends on the name. A
  plain name such as `express` means a package, and Node searches the `node_modules` folder
  beside your project. A name starting with a dot such as `./utils` means one of your own
  files, and Node looks at that exact path on disk. Nothing ran. Node stopped while working
  out what your code imports.

danger: >
  The last step deletes the `node_modules` folder. Nothing you wrote lives there and a
  reinstall recreates it completely, so the cost is download time rather than work. Never
  point that command at any other folder.

fix_ladder:
  - try: Install the project's dependencies.
    command: npm install
    shell: powershell
    why: >
      Assumes `node_modules` does not exist yet. A freshly cloned project has no dependencies
      on disk, because `node_modules` is never committed. This is the answer most of the
      time and it needs no arguments: `package.json` already lists what to fetch.

  - try: Look at whether the name in the error starts with a dot.
    why: >
      Assumes the import is one of your own files rather than a package, which changes the
      answer entirely. `./utils` means a file called `utils` in the same folder. If that file
      does not exist, or is spelled differently, or is called `utils.js` and the import left
      the extension off in a project that needs it, no amount of installing will help.

  - try: Install that one package and record it.
    command: npm install <name>
    shell: powershell
    why: >
      Assumes the code imports something that was never added to `package.json`. Agents write
      an import line and forget the install step constantly. This does both: it downloads the
      package and adds it to the dependency list so the next person gets it too.

  - try: Check you are running from the project folder.
    command: Get-Location; Test-Path package.json
    shell: powershell
    why: >
      Assumes you are one folder off. Node searches for `node_modules` starting beside the
      file it is running and walking upward, so running from a parent folder or from your
      home folder finds nothing. `False` from that second command settles it.

  - try: Confirm the package exists at all.
    command: npm view <name> version
    shell: powershell
    why: >
      Assumes the agent invented it. If this prints "404 Not Found", no such package is
      published and the import line is fiction. Agents produce plausible package names that
      have never existed, which is a real category of failure rather than a rare one.

  - try: Delete the dependency folder and install again.
    command: Remove-Item -Recurse -Force node_modules; npm install
    shell: powershell
    why: >
      Assumes the install is corrupt, usually from a run that was interrupted partway. This
      is slow and it is a real fix for a real problem, so it belongs at the bottom rather
      than at the top where people reach for it.

if_none_worked: >
  Paste the whole error including the `Require stack` block and the `code` line, the exact
  command you ran, and the `dependencies` section of your `package.json`. The require stack
  names the file doing the importing, which is the piece people cut, and it is how you tell
  your own bad import apart from a genuinely missing package.

see_also:
  - g1-what-a-dependency-is
  - g2-package-managers
  - g7-dependency-risk
  - javascript

keywords:
  - cannot find module
  - MODULE_NOT_FOUND
  - npm install missing
  - require failed
  - module not installed
---

The first thing to read is the name inside the quotes, and specifically whether it starts
with a dot.

`Cannot find module 'express'` is a package problem and `npm install` is the fix. `Cannot
find module './utils'` is your own file and installing anything is a waste of time. The two
errors look identical and have nothing in common.

For the file case, check three things in this order: that the file exists where the import
says, that its name matches exactly including capital letters, and that the extension is
right. Windows treats `Utils.js` and `utils.js` as the same file and the servers this code
will eventually run on do not, which is a bug that appears only after deployment.

The `Require stack` section names which of your files did the importing. On a large project
that is the fastest way to find the offending line.
