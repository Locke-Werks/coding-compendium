---
id: typescript-ts2307-cannot-find-module
title: "error TS2307: Cannot find module or its type declarations"
type: error
verified: 2026-08-02
volatility: low

language: typescript
category: not-found

# Type-checks the whole project without producing any output files. Silence
# means every import resolves.
verify: npx tsc --noEmit

sample: |
  PS C:\Users\you\dev\site> npx tsc --noEmit
  src/App.tsx:3:24 - error TS2307: Cannot find module './components/Header' or its corresponding type declarations.

  3 import { Header } from './components/Header';
                           ~~~~~~~~~~~~~~~~~~~~~

  Found 1 error in src/App.tsx:3

patterns:
  - "TS2307"
  - "or its corresponding type declarations"
  - "Cannot find module"

means: >
  TypeScript could not find either the thing you imported or a description of its shape. The
  second half of the message matters: TypeScript needs to know what types a module provides, and
  some packages ship that description separately. So this appears both when a file or package is
  genuinely missing and when it is installed but has no type information.

fix_ladder:
  - try: Look at whether the name starts with a dot.
    why: >
      Assumes the fork in the road, which decides everything below. `./components/Header` is one
      of your own files and no install will help. A bare name such as `express` is a package.
      The two cases share a message and share nothing else.

  - try: Check the file exists at that exact path, capital letters included.
    command: Get-ChildItem .\src\components
    shell: powershell
    why: >
      Assumes a path or case mismatch on your own file. Windows treats `header.tsx` and
      `Header.tsx` as the same file and TypeScript does not, so this compiles for you and fails
      in continuous integration. A listing next to the import line settles it.

  - try: Install the package.
    command: npm install <name>
    shell: powershell
    why: >
      Assumes a bare package name that is not installed. An import line does not add a
      dependency. This is common when an agent writes the import and the install step is skipped.

  - try: Install the separate types package.
    command: npm install --save-dev @types/<name>
    shell: powershell
    why: >
      Assumes the package is installed and ships no type information of its own. Older JavaScript
      libraries keep their types in a matching `@types` package maintained separately. If that
      package does not exist either, the library has no types at all and needs a declaration file.

  - try: Check for a path alias in the TypeScript config.
    command: Get-Content tsconfig.json | Select-String paths
    shell: powershell
    why: >
      Assumes the import uses a shortcut such as `@/components/Header`. Those are defined under
      `paths` and `baseUrl` in `tsconfig.json`, and they have to be configured in the build tool
      as well. An alias that works in the editor and fails at build time is configured in only one
      of the two places.

  - try: Restart the TypeScript service in your editor.
    why: >
      Assumes the file is fine and the editor is showing a stale result. In Visual Studio Code,
      open the command palette with Ctrl+Shift+P and run "TypeScript: Restart TS Server". If
      `npx tsc --noEmit` is clean and the red squiggle stays, this is it.

if_none_worked: >
  Paste the whole error including the underlined import line, the output of `Get-ChildItem` on the
  folder that should hold the file, and the `paths` section of `tsconfig.json` if there is one.
  The folder listing is what people leave out, and case differences are invisible in a description
  and obvious in a listing.

see_also:
  - g1-what-a-dependency-is
  - j3-project-layouts
  - c7-files-folders-and-paths
  - typescript

keywords:
  - TS2307
  - cannot find module typescript
  - corresponding type declarations
  - types package
  - path alias
---

The phrase "or its corresponding type declarations" is doing real work, and it is why this error
has two completely different causes.

TypeScript checks types before anything runs. For that it needs a description of every module's
shape. Modern packages include one. Older JavaScript packages do not, and the community publishes
descriptions separately under the `@types` name. So `npm install express` can succeed and this
error can persist until `@types/express` is installed too.

For your own files, the causes are duller and more common: a wrong relative path, a file that was
renamed, or a capital letter. The capital letter one is worth taking seriously because Windows
hides it. The build passes on your machine and fails on the Linux machine that deploys it.

One more distinction worth holding. `npx tsc --noEmit` is the truth. Your editor runs its own
copy of TypeScript and can hold a stale result for minutes after you fix something. When the two
disagree, believe the command line.
