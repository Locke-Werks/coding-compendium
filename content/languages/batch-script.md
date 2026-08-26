---
id: batch-script
title: Batch (.bat and .cmd)
type: language
verified: 2026-08-02
volatility: low

name: Batch
aka: [bat, cmd, batch file, dos batch, command prompt script]
family: shell
likelihood: possible
extensions: ['.bat', '.cmd']

danger: >
  `del /s /q <folder>` and `rmdir /s /q <folder>` delete everything under a folder
  with no prompt and no Recycle Bin. A batch file does all of this the instant you
  double-click it, with no confirmation and no window you can read afterwards.
  Open an unfamiliar `.bat` in Notepad and read it before running it, and drop the
  `/q` so it asks first.

tells:
  - pattern: '^@echo off'
    kind: regex
    weight: 10
    note: >
      The first line of nearly every batch file. The `@` hides the command itself
      from the output. Bash and PowerShell have no equivalent and no reason for
      one.
  - pattern: '%\w+%'
    kind: regex
    weight: 9
    note: >
      Variables are wrapped in percent signs on both sides: `%USERPROFILE%`, and
      `%1` for the first argument. PowerShell writes `$env:USERPROFILE`, bash
      writes `$HOME`.
  - pattern: '^\s*(REM|rem)\s'
    kind: regex
    weight: 8
    note: >
      A comment line. PowerShell and bash both use `#`, which in a batch file is
      not a comment and will be run as a command.
  - pattern: '^:\w+'
    kind: regex
    weight: 8
    note: >
      A line starting with a colon is a jump target for `goto`. Real languages
      have functions; batch has labels and `goto :eof` to leave one.
  - pattern: 'setlocal'
    kind: token
    weight: 7
    note: >
      Usually `setlocal enabledelayedexpansion`, the incantation that makes
      variables update inside a loop. Nothing else in computing is spelled like
      this.
  - pattern: 'errorlevel'
    kind: token
    weight: 7
    note: >
      `if errorlevel 1` checks whether the last command failed. Bash reads `$?`
      and PowerShell reads `$LASTEXITCODE`.

rules_out:
  - pattern: '$env:'
    kind: sigil
    because: >
      PowerShell. A `.bat` file uses `%NAME%`.
  - pattern: '#!/'
    kind: line_start
    because: >
      A Unix shebang, so the file is bash or Python.
  - pattern: '^\s*#'
    kind: regex
    because: >
      A `#` comment line means PowerShell, bash, Python, or YAML. Batch comments
      are `REM` or `::`.

project_fingerprint:
  manifests:
    - file: '*.bat'
      decisive: true
      note: >
        A script for the old Windows shell. Double-clicking runs it immediately in
        a black window that closes when it finishes, which is why errors in one
        are so hard to read.
    - file: '*.cmd'
      decisive: true
      note: >
        The same language with a different extension. The differences are small
        enough that you can treat them as one thing.
    - file: 'gradlew.bat'
      note: >
        A `.bat` sitting next to an extensionless file of the same name is the
        Windows twin of a shell script. Run the `.bat`, ignore the other.
    - file: 'build.bat'
      note: >
        Common in older Windows projects and in anything that predates PowerShell.
  entry_points: ['build.bat', 'install.bat', 'run.cmd']

shape:
  blocks: none
  statement_end: newline
  comment_line: 'REM'
  string_quotes: >
    Double quotes only, and they often become part of the value rather than
    disappearing, which is why quoted paths in batch files behave strangely.
  naming: commands conventionally uppercase, though the whole language ignores case
  import_keyword: call

confusable_with:
  - language: powershell
    settle_it: >
      Percent signs decide it. `%VAR%` and `@echo off` are batch. `$env:VAR` and
      `Verb-Noun` commands are PowerShell. Both run on Windows and only one of
      them gives you a usable error message.
    tiebreak: { pattern: '%\w+%', kind: regex, favors: batch-script }
  - language: bash
    settle_it: >
      Both are terse shell scripts. Batch uses `%VAR%`, `REM`, and backslash
      paths. Bash uses `$VAR`, `#`, forward slashes, and a `#!/bin/bash` first
      line.
    tiebreak: { pattern: '#!/bin/bash', kind: line_start, favors: bash }

errors_look_like:
  sample: |
    'pyton' is not recognized as an internal or external command,
    operable program or batch file.
  recognize_by: >
    The phrase `is not recognized as an internal or external command` is Command
    Prompt and nothing else. PowerShell words the same failure as
    `The term 'x' is not recognized as the name of a cmdlet`. Batch also has no
    line numbers in its errors, so you get the message and no location.
  patterns:
    - 'is not recognized as an internal or external command'
    - 'The syntax of the command is incorrect'
    - 'Access is denied\.'

meet_it_when: >
  You download an older tool and it ships `install.bat` or `run.bat`. You see
  `gradlew.bat` beside `gradlew` in a Java project. An agent writes one because it
  saw a Windows target and reached for the oldest thing it knew.

what_agents_get_wrong: >
  Agents write batch rarely and badly, and its two famous traps are exactly the
  ones they fall into. Percent signs double inside a file but not at the prompt, so
  a loop variable is `%%i` in a `.bat` and `%i` when typed live; agents mix these
  up in both directions. Worse, a variable set inside an `if` or `for` block keeps
  its old value when read as `%VAR%`, so the script runs to the end with stale data
  and no error at all. The fix is `setlocal enabledelayedexpansion` and `!VAR!`,
  which agents omit. If an agent offers you a `.bat`, ask for a `.ps1` instead:
  PowerShell is on every Windows 11 machine and its failures are readable.

see_also:
  - powershell
  - bash
  - b1-terminal-shell-command-line

keywords: [bat, cmd, command prompt, dos, batch file, errorlevel, goto]
---

The scripting language of Command Prompt, the Windows shell that predates
PowerShell. You will read batch files more often than you write them, because a
lot of software still ships one.

Variables sit between percent signs. Comments are `REM`. There are no blocks, only
labels and jumps. The whole language is case-insensitive, so `ECHO`, `echo`, and
`Echo` are the same command.

```cmd
@echo off
setlocal
set NAME=ada
echo Hello %NAME%
if not exist build mkdir build
if errorlevel 1 echo Something failed
```

`set NAME=ada` has no spaces around the `=` on purpose. Adding them puts the spaces
inside the value, and nothing warns you.

A batch file runs the moment you double-click it, in a window that closes when it
ends, so a failing one flashes past unread. Open it from an already-running Command
Prompt instead and the message stays on screen. If you have a choice, use
[PowerShell](#powershell): same machine, better errors, and variables that behave.
