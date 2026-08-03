---
id: remove-item
title: Remove-Item -Recurse
type: command
verified: 2026-08-02
volatility: low

tool: powershell
command: Remove-Item <path> -Recurse -Force
shell: powershell

does: >
  Deletes a file or an entire folder and everything inside it, permanently and without using
  the Recycle Bin.

flags:
  - flag: "-Recurse"
    means: >
      Delete the folder's contents as well as the folder. Without it, PowerShell stops and
      asks whether you really mean to delete a folder that is not empty.
  - flag: "-Force"
    means: >
      Delete read-only and hidden items too, and stop asking. Together with `-Recurse` this
      removes every prompt that would have given you a chance to reconsider.
  - flag: "-WhatIf"
    means: >
      Lists everything that would be deleted and deletes nothing. This is the version to run
      first, every time.
  - flag: "-Confirm"
    means: Asks about each item individually. Slow, and appropriate when the path came from somewhere you do not fully trust.

destructive: true

danger: >
  Files removed this way do not go to the Recycle Bin. They are unlinked immediately. A
  mistyped path with `-Recurse -Force` can empty a folder tree in under a second, and a path
  with a variable in it that turned out to be empty is the classic way people delete far more
  than they meant to.

destroys: >
  Everything under the path you named, including files git never tracked, files you never
  committed, and files no backup covers. There is no undo. If the folder was inside a git
  repository, tracked files that were committed can be restored with `git restore`, and
  nothing else can.

safer_first: >
  Run the same command with `-WhatIf` on the end and read every line it prints. If the path
  came from a variable, print the variable by itself first and confirm it holds what you
  think.

undo: >
  You cannot, unless a backup exists. Windows File History, OneDrive version history, or a git
  commit are your only routes back. The Recycle Bin will not have it.

expect: >
  Nothing printed on success, which is exactly what a mistake looks like too. Confirm with
  `Test-Path <path>`, which should now print `False`.

see_also:
  - test-path
  - get-childitem
  - git-clean-fd
  - c7-files-folders-and-paths

keywords:
  - delete a folder
  - rm -rf on windows
  - remove directory
  - delete node_modules
---
