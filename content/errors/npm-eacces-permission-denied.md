---
id: npm-eacces-permission-denied
title: "EACCES: permission denied"
type: error
verified: 2026-08-02
volatility: low

language: javascript
category: permission

# Lists the packages installed directly in this project. It fails the same way
# if the install never completed.
verify: npm ls --depth=0

danger: >
  One step deletes the `node_modules` folder. Nothing you wrote lives there and `npm install`
  rebuilds it from `package.json`, so the cost is download time. Never aim that command at
  any other folder, and never at your project root.

sample: |
  PS C:\Users\you\dev\site> npm install
  npm error code EPERM
  npm error syscall rename
  npm error path C:\Users\you\dev\site\node_modules\esbuild
  npm error dest C:\Users\you\dev\site\node_modules\.esbuild-Xk3mQp
  npm error errno -4048
  npm error Error: EPERM: operation not permitted, rename 'C:\Users\you\dev\site\node_modules\esbuild' -> 'C:\Users\you\dev\site\node_modules\.esbuild-Xk3mQp'
  npm error   [Error: EPERM: operation not permitted, rename] {
  npm error   errno: -4048,
  npm error   code: 'EPERM',
  npm error   syscall: 'rename'
  npm error }

patterns:
  - "EACCES"
  - "EPERM"
  - "permission denied"
  - "operation not permitted"

means: >
  Windows refused to let npm create, rename, or delete a file. On Windows this is usually a
  lock rather than a rights problem: another program has the file open and Windows will not
  let two things change it at once. The genuine rights version happens when npm is writing
  outside your own folders, which a global install does.

fix_ladder:
  - try: Read the `path` line in the error and see where it points.
    why: >
      Assumes you can sort the two causes apart in one look, which decides everything below. A
      path inside your project means a lock. A path under `C:\Program Files` or
      `C:\Users\<yourname>\AppData\Roaming\npm` means a global install and a rights question.

  - try: Stop the dev server, close your editor, and run the install again.
    why: >
      Assumes a lock. A running dev server holds files inside `node_modules` open, and so do
      some editor extensions. This is the cause the large majority of the time on a project
      folder, and it costs one retry to test.

  - try: Install into the project instead of globally.
    command: npm install <name>
    shell: powershell
    why: >
      Assumes you were running `npm install -g` and hit a rights wall. A project-local install
      writes only inside your own folder, needs no special permissions, and is what you want
      for anything the project itself uses. Run it afterward with `npx <name>`.

  - try: Open PowerShell as administrator for a genuine global install.
    why: >
      Assumes the tool really does need to be global, such as a command-line utility you want
      everywhere. Right-click the PowerShell entry in the Start menu and pick Run as
      administrator. Use this only for global installs, never as a habit for project work.

  - try: Delete the dependency folder and install from scratch.
    command: Remove-Item -Recurse -Force node_modules; npm install
    shell: powershell
    why: >
      Assumes a previous install died partway and left files in a state npm cannot fix. If the
      delete itself fails with the same error, something still has a file open, and a restart
      is the reliable way to clear every lock at once.

  - try: Exclude your project folder from real-time antivirus scanning.
    why: >
      Assumes antivirus is grabbing files as npm writes them. The signature is failure at a
      random package each time rather than the same one. Windows Security has an Exclusions
      list under Virus and threat protection settings.

if_none_worked: >
  Paste the whole error including the `path`, `dest`, and `syscall` lines, and say whether a
  dev server was running. `syscall` names the operation that failed, so `rename` points at a
  lock and `mkdir` points at rights. Those three lines are the ones people cut as noise and
  they contain the entire diagnosis.

see_also:
  - g2-package-managers
  - g4-environments-and-isolation
  - c5-processes-and-killing-them
  - javascript

keywords:
  - EACCES npm
  - EPERM rename
  - operation not permitted
  - npm install permission
  - node_modules locked
---

Most Node instructions on the internet were written for Linux and macOS, where `EACCES` means
what it says and the traditional answer is to install somewhere else or fix folder ownership.
On Windows the same code usually means a file is locked.

The tell is the `syscall` line. `rename` is npm swapping a temporary folder into place, and
that fails when something is reading the destination. `mkdir` or `open` on a path outside your
home folder is a real permissions refusal.

Two habits avoid nearly all of it. Stop the dev server before installing anything, and prefer
project-local installs over global ones. `npx` runs a project-local tool by name, so global
installs are rarely needed at all.
