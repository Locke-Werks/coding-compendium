---
id: test-path
title: Test-Path
type: command
verified: 2026-08-02
volatility: low

tool: powershell
command: Test-Path <path>
shell: powershell

does: >
  Answers whether a file or folder exists at the path you name, printing True or False and
  changing nothing.

flags:
  - flag: "-PathType Leaf"
    means: >
      Returns True only if the path is a file. A folder with the same name gives False, which
      is how you tell the two apart.
  - flag: "-PathType Container"
    means: Returns True only if the path is a folder.
  - flag: "<path> with a wildcard"
    means: >
      `Test-Path .\logs\*.log` returns True if anything matches, which is a quick way to ask
      whether a build produced any output.

expect: >
  A single word, `True` or `False`. Nothing else and no error, even when the path is nonsense.

see_also:
  - get-childitem
  - set-location
  - remove-item
  - c7-files-folders-and-paths

keywords:
  - does this file exist
  - check if folder exists
  - file not found
  - verify a path
---

Use this before and after anything that acts on a path. It costs nothing, and it separates
"the command failed" from "the file was never there", which are different problems with
different fixes.

Quote paths containing spaces: `Test-Path "C:\Program Files\Git"`.
