---
id: get-command
title: Get-Command
type: command
verified: 2026-08-02
volatility: low

tool: powershell
command: Get-Command <name>
shell: powershell

does: >
  Tells you whether a command exists and, when it is a program, exactly which file on disk
  PowerShell would run.

flags:
  - flag: "<name>"
    means: >
      The command to look up, such as `git` or `node`. Wildcards work: `Get-Command git*`
      lists everything starting with those letters.
  - flag: "-All"
    means: >
      Shows every match rather than the first one. This is how you discover that two versions
      of Python are installed and the wrong one is winning.
  - flag: "(Get-Command <name>).Source"
    means: >
      Prints only the full file path, with no table around it. The most useful form, and the
      answer to "which one is actually running".
  - flag: "-CommandType Application"
    means: Limits results to real executable files, filtering out aliases and PowerShell functions.

expect: >
  A table with `CommandType`, `Name`, `Version`, and `Source`. The `Source` column holds the
  full path. If nothing matches you get
  `The term '<name>' is not recognized as a name of a cmdlet, function, script file, or
  executable program.`

see_also:
  - powershell-env-variable
  - setx
  - c4-path-and-command-not-found
  - winget-install

keywords:
  - which command
  - where is this program installed
  - is git installed
  - command not recognized
  - two versions installed
---

This is the PowerShell answer to `which` on macOS and Linux. `where.exe <name>` also works and
prints every match, one path per line.

The two questions it settles: whether the thing is installed at all, and if so, which copy runs
when several exist. The second question is behind most "but I installed it" confusion.
