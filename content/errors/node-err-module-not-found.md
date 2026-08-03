---
id: node-err-module-not-found
title: "Error [ERR_MODULE_NOT_FOUND]"
type: error
verified: 2026-08-02
volatility: low

language: javascript
category: not-found

# Prints the installed version if the package is there, and an error if it is
# not. Run it in the project folder.
verify: npm ls <name>

sample: |
  PS C:\Users\nyx\dev\site> node index.js
  node:internal/process/esm_loader:40
        internalBinding('errors').triggerUncaughtException(
        ^

  Error [ERR_MODULE_NOT_FOUND]: Cannot find module 'C:\Users\nyx\dev\site\src\utils' imported from C:\Users\nyx\dev\site\src\index.js
  Did you mean to import "./utils.js"?
      at finalizeResolution (node:internal/modules/esm/resolve:265:11)
      at moduleResolve (node:internal/modules/esm/resolve:933:10) {
    code: 'ERR_MODULE_NOT_FOUND',
    url: 'file:///C:/Users/nyx/dev/site/src/utils'
  }

patterns:
  - "ERR_MODULE_NOT_FOUND"
  - "Cannot find package"
  - "Did you mean to import"
  - "imported from"

means: >
  This project uses ESM (ECMAScript Modules), the `import` syntax, and ESM resolves file paths
  strictly. It will not guess an extension for you. `import { x } from './utils'` fails even
  when `utils.js` is sitting right there, because the import did not say `.js`. The other
  cause is a package that is not installed, which prints "Cannot find package" instead of
  naming a file path.

fix_ladder:
  - try: Add the file extension to the import.
    why: >
      Assumes the failing name is one of your own files. Change `from './utils'` to
      `from './utils.js'`. Node even suggests this in the error when it can work out what you
      meant. Older Node code and every tutorial written before ESM omit the extension, which
      is why agents write it that way.

  - try: Install the package, if the error says "Cannot find package".
    command: npm install <name>
    shell: powershell
    why: >
      Assumes a missing dependency rather than a path problem. The wording is the tell: Node
      says "package" when the name has no slash in it and "module" with a full file path when
      it was looking for a file.

  - try: Check that the file name matches exactly, including capital letters.
    command: Get-ChildItem .\src
    shell: powershell
    why: >
      Assumes a case mismatch. Windows finds `Utils.js` when you ask for `utils.js` and most
      servers do not, so this is the class of bug that works on your machine and breaks the
      moment it is deployed.

  - try: Look at whether the project is ESM or the older CommonJS.
    command: Get-Content package.json | Select-String type
    shell: powershell
    why: >
      Assumes a mix of the two module systems. `"type": "module"` means every `.js` file in
      the project is ESM and needs extensions on imports. Without that line the project uses
      the older `require` system, where extensions are optional and this error looks
      different.

  - try: Check whether you are importing a TypeScript file by its source name.
    why: >
      Assumes the import points at `./utils.ts` or at a file that only exists before
      compiling. Compiled output is usually `.js` in a `dist` folder, and imports in ESM
      TypeScript projects point at the `.js` name even though the file on disk is `.ts`. That
      looks wrong and is correct.

if_none_worked: >
  Paste the whole error including the `url:` line at the bottom, the import line from your
  source file, and the output of `Get-ChildItem` on the folder that file should be in. The
  `url:` line shows the exact path Node built, extension and all, and it is the piece people
  cut because it looks redundant next to the message.

see_also:
  - g1-what-a-dependency-is
  - j3-project-layouts
  - c7-files-folders-and-paths
  - javascript

keywords:
  - ERR_MODULE_NOT_FOUND
  - cannot find package
  - esm import extension
  - did you mean to import
  - type module
---

JavaScript has two module systems and they disagree about this exact detail.

The older one, CommonJS, uses `require()` and fills in a missing `.js` for you. The newer
one, ESM (ECMAScript Modules), uses `import` and does not. It was designed to behave the
same in a browser as it does on a server, and browsers have never guessed extensions.

Which one a project uses comes down to one line in `package.json`. With `"type": "module"`,
every import needs its extension. Without it, they do not. A project that grew through
several agent sessions can easily end up with both styles in different files.

Agents get this wrong more than almost anything else in JavaScript, because their training
is full of pre-ESM code where the extensionless form is correct.
