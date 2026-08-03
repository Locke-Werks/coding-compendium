---
id: get-childitem
title: Get-ChildItem
type: command
verified: 2026-08-02
volatility: low

tool: powershell
command: Get-ChildItem
shell: powershell

does: >
  Lists the files and folders in a directory, which is PowerShell's version of what most
  tutorials write as `ls` or `dir`.

flags:
  - flag: "-Path <path>"
    means: >
      Lists a directory other than the one you are standing in. The name `-Path` is optional:
      `Get-ChildItem C:\Users\<yourname>\dev` works.
  - flag: "-Recurse"
    means: >
      Descends into every subfolder. On a project with a `node_modules` folder this prints
      tens of thousands of lines, so pair it with a filter.
  - flag: "-Filter <pattern>"
    means: >
      Keeps only names matching a wildcard pattern, such as `-Filter *.log`. Faster than the
      alternatives because Windows applies it during the search rather than afterward.
  - flag: "-Force"
    means: >
      Also shows hidden and system items. Without it, `.git` and other dot-folders are
      invisible, which is why a folder can look empty when it is not.
  - flag: "-Name"
    means: Prints bare names instead of the full table, which is easier to read and to paste elsewhere.

expect: >
  A table headed `Mode`, `LastWriteTime`, `Length`, `Name`, with one row per item. An empty
  folder prints nothing at all.

see_also:
  - set-location
  - test-path
  - select-string
  - c7-files-folders-and-paths

keywords:
  - list files
  - ls in powershell
  - dir
  - show hidden files
  - find files
---

`ls`, `dir`, and `gci` are all built-in aliases for this command in PowerShell, so tutorials
written for macOS or Linux mostly work. The aliases accept PowerShell flags, not Unix ones:
`ls -Force` works, `ls -la` does not.
