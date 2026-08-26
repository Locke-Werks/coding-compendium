---
id: k8-the-translation-table
title: The translation table
type: section
track: K
order: 80
verified: 2026-08-25
volatility: low
verify: Get-Location
danger: >
  Lists the delete commands for all four shells. None of them use the Recycle Bin and none of
  them have an undo. List what you are about to delete first, which k3 shows in full.
answer: >
  The same twenty tasks, spelled four ways: PowerShell, Command Prompt, Bash and Zsh, with
  bash and zsh identical for everything on the list.
owns:
  - the four-shell translation table
  - what is identical everywhere
see_also:
  - k3-the-first-ten-commands
  - k7-same-word-different-program
  - k1-four-shells-one-console
  - k5-pipes-and-chaining
keywords:
  - powershell bash equivalent
  - cheat sheet
  - command translation
  - cmd equivalent of ls
  - bash to powershell
  - shell comparison table
  - what is the windows version of
---

## More

Bash and zsh share one column, because for everything on this list they are the same. Where
they differ, the difference is in prompts and completion rather than in command names.

| Task | PowerShell | Command Prompt | Bash and Zsh |
|---|---|---|---|
| Where am I | `Get-Location` | `cd` | `pwd` |
| List files | `Get-ChildItem` | `dir` | `ls` |
| List everything, hidden included | `Get-ChildItem -Force` | `dir /a` | `ls -la` |
| Change folder | `Set-Location <path>` | `cd /d <path>` | `cd <path>` |
| Up one level | `cd ..` | `cd ..` | `cd ..` |
| Home folder | `cd ~` | `cd %USERPROFILE%` | `cd ~` |
| Print a file | `Get-Content <file>` | `type <file>` | `cat <file>` |
| First 10 lines | `Get-Content <file> -Head 10` | no equivalent | `head <file>` |
| Last 10 lines | `Get-Content <file> -Tail 10` | no equivalent | `tail <file>` |
| Watch a file grow | `Get-Content <file> -Wait` | no equivalent | `tail -f <file>` |
| Make a folder | `mkdir <name>` | `mkdir <name>` | `mkdir <name>` |
| Make an empty file | `New-Item -ItemType File <name>` | `type nul > <name>` | `touch <name>` |
| Copy a file | `Copy-Item <a> <b>` | `copy <a> <b>` | `cp <a> <b>` |
| Copy a folder | `Copy-Item <a> <b> -Recurse` | `xcopy <a> <b> /s` | `cp -r <a> <b>` |
| Move or rename | `Move-Item <a> <b>` | `move <a> <b>` | `mv <a> <b>` |
| Delete a file | `Remove-Item <file>` | `del <file>` | `rm <file>` |
| Delete a folder | `Remove-Item <dir> -Recurse` | `rmdir /s <dir>` | `rm -r <dir>` |
| Find a file by name | `Get-ChildItem -Recurse -Filter <pat>` | `dir /s <pat>` | `find . -name <pat>` |
| Find text in files | `Select-String -Pattern <text>` | `findstr /s <text>` | `grep -rn <text>` |
| Find a program | `Get-Command <name>` | `where <name>` | `which <name>` |
| Print text | `Write-Output <text>` | `echo <text>` | `echo <text>` |
| Read one variable | `$env:PATH` | `%PATH%` | `$PATH` |
| Clear the screen | `cls` | `cls` | `clear` |
| Command history | `Get-History` | `doskey /history` | `history` |

Deleting is the row to be careful with. None of those four commands use the Recycle Bin.
[k3](#k3-the-first-ten-commands) has the safe habit, which is running the list command with
the same pattern first.

## Full

### What is genuinely the same everywhere

A short list, and worth knowing precisely because it is short. These work identically in all
four shells:

- **`cd ..`** to go up a level, and `.` for the current folder.
- **`mkdir`** to make a folder.
- **The pipe `|`**, to send one command's output into another.
- **`>` and `>>`** to send output into a file, overwriting or appending.
- **`cls`** in the two Windows shells, `clear` in the two Unix ones, and Ctrl+L in three out
  of four.
- **Tab** to complete, **up arrow** for the last command, **Ctrl+C** to stop what is running.
- **Quoting a path that has a space in it.** Double quotes work everywhere.
- **The name of most real programs.** `git`, `node`, `python`, `docker` and `gh` are the
  same word with the same flags in every shell, because they are programs rather than shell
  commands. This is a larger fraction of what you type than the table above suggests.

That last point is the reassuring one. The differences are concentrated in the built-in
file-handling commands. Everything you install behaves the same everywhere.

### The rows that need a footnote

**`cd` in Command Prompt.** Alone it prints the current folder rather than going home, and
it will not change drive without `/d`. Both behaviors are unique to that shell.

**PowerShell's aliases.** `ls`, `cat`, `cp`, `mv`, `rm` and `pwd` all work in PowerShell and
all point at the `Verb-Noun` command in the first column. The names carry over and the flags
do not, which is [k7](#k7-same-word-different-program).

**`find` means two different things.** In bash, `find` searches for files by name. In
Command Prompt, `find` searches for text inside a file, which is what bash calls `grep`. The
two are near-opposites and both are spelled the same. This is the worst collision in the
table.

**`where` in PowerShell** is an alias for filtering, not for finding a program. Write
`where.exe` when you want the Command Prompt behavior.

**Making an empty file in Command Prompt** is `type nul > name.txt`, which reads like
nonsense and is: it prints the contents of the null device, which is nothing, into a new
file. Nobody would design that. It is what there is.

### Reading the other direction

Most answers online are written for bash. Translating one into PowerShell goes in this
order:

1. **Check whether it needs translating at all.** If the command starts with `git`, `npm`,
   `node`, `python`, `docker` or `gh`, it runs in PowerShell unchanged.
2. **Look up the verb in the table above.** `cat` becomes `Get-Content`, and so on.
3. **Do not translate the flags.** `-la` has no PowerShell counterpart, because the
   PowerShell command has different options entirely. Find the option you actually want with
   `Get-Help <command> -Examples`.
4. **When the pipeline slices text into columns**, stop translating. A bash pipeline built
   around `awk` or `cut` has no direct equivalent, because PowerShell would select a property
   instead. [k5](#k5-pipes-and-chaining) covers why.

The fastest escape hatch, when the translation is not worth the time: open Git Bash and run
the original command there unchanged. It arrived with Git for Windows and it is already on
your machine.
