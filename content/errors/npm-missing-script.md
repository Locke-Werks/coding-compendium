---
id: npm-missing-script
title: "npm error Missing script: 'dev'"
type: error
verified: 2026-08-02
volatility: quarterly

language: javascript
category: not-found

# With no arguments, npm prints every script this project defines along with
# the command each one runs.
verify: npm run

sample: |
  PS C:\Users\you\dev\site> npm run dev
  npm error Missing script: "dev"
  npm error
  npm error Did you mean one of these?
  npm error     npm run start # run the "start" package script
  npm error     npm star # Mark your favorite packages
  npm error
  npm error To see a list of scripts, run:
  npm error   npm run

patterns:
  - "Missing script"
  - "npm ERR! missing script"
  - "To see a list of scripts, run"

means: >
  npm looked in the `scripts` section of `package.json` for an entry named `dev` and there is
  no such entry. Scripts are shortcuts a project defines for itself, so there is no standard
  set and no guarantee that a project has any particular one. Nothing is missing from your
  machine. This project names its commands differently, or you are not in the project you
  think you are in.

fix_ladder:
  - try: List the scripts this project actually has.
    command: npm run
    shell: powershell
    why: >
      Assumes the project uses a different name for the same job. `dev`, `start`, and `serve`
      all mean "run it locally" depending on who set the project up. This prints every
      available name with the command behind it, so you can see what each one does.

  - try: Check that you are in the right folder.
    command: Get-Location; Test-Path package.json
    shell: powershell
    why: >
      Assumes you are in a parent folder or a sibling project. `False` from the second command
      means there is no `package.json` here at all, and npm was reading a different one
      further up the folder tree or none.

  - try: Look at the scripts section directly.
    command: Get-Content package.json -Raw | ConvertFrom-Json | Select-Object -ExpandProperty scripts
    shell: powershell
    why: >
      Assumes `npm run` printed nothing useful, which happens when the `scripts` section is
      missing entirely. This prints it as a list, and an error here means the section does not
      exist and the project was never set up with any.

  - try: Check whether this project uses a different package manager.
    command: Get-ChildItem -Filter "*lock*"
    shell: powershell
    why: >
      Assumes a tool mismatch. A `pnpm-lock.yaml` means the project expects `pnpm run dev`,
      and a `yarn.lock` means `yarn dev`. Scripts usually still work through npm, but installs
      done with the wrong tool produce a second lockfile and a confusing dependency tree.

  - try: Add the script yourself.
    why: >
      Assumes the project genuinely has no such command and you know what should run. Open
      `package.json`, find `"scripts"`, and add a line such as `"dev": "vite"`. The name on the
      left is what you type after `npm run`, and the value on the right is the command.

if_none_worked: >
  Paste the error, the output of `npm run` with no arguments, and the `scripts` section of
  `package.json`. The scripts section is what people leave out because they assume the tool
  already read it. Seeing the real names next to the name you typed answers the question
  immediately.

see_also:
  - c3-what-running-means
  - j3-project-layouts
  - g2-package-managers
  - javascript

keywords:
  - missing script
  - npm run dev not found
  - no start script
  - package.json scripts
  - wrong package manager
---

There is no universal command for running a project, and this error is where most people
find that out.

`npm run <name>` looks up `<name>` in one section of one file. What lives there is whatever
the person who made the project decided to put there. A Vite project usually has `dev`. A
Create React App project has `start`. A library might have neither, because there is nothing
to run.

This is why the answer to "how do I run this" is always in the project's manifest rather than
in general knowledge. `npm run` with no arguments is the fastest way to read it.

Two nearby messages mean different things. `npm error code ENOENT` with `package.json` in the
path means there is no manifest here at all. `Missing script` means there is one and it does
not define that name.
