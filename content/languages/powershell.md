---
id: powershell
title: PowerShell
type: language
verified: 2026-08-02
volatility: low

name: PowerShell
aka: [pwsh, posh, windows powershell, powershell core, ps1]
family: shell
likelihood: certain
extensions: ['.ps1', '.psm1', '.psd1']
verify: $PSVersionTable.PSVersion

danger: >
  `Remove-Item -Recurse -Force <path>` deletes a folder and everything under it
  permanently. It does not go to the Recycle Bin, `-Recurse` means it does not
  stop to ask about the contents, and `-Force` means it takes read-only and
  hidden files too. Run it with `-WhatIf` first, which prints every file it
  would delete and deletes nothing, then remove `-WhatIf` when the list matches
  what you expected.

# Every note below is stated against the neighbor it gets confused with, because
# the neighbor is bash and the confusion happens daily. A tell with no contrast
# tells the reader nothing they can act on at a prompt.
tells:
  - pattern: '$env:'
    kind: sigil
    weight: 10
    note: >
      Environment variables live behind an `$env:` prefix, as in `$env:PATH` or
      `$env:USERPROFILE`. Bash writes the same idea as `$PATH` and sets it with
      `export`. Nothing else in this deck uses `$env:`, so one sighting settles it.
  - pattern: '\b(Get|Set|New|Remove|Start|Stop|Test|Invoke|Select|Write|Import|Export|Add|Copy|Move)-[A-Z][A-Za-z]+'
    kind: regex
    weight: 9
    note: >
      Commands are a verb, a hyphen, and a singular noun: `Get-ChildItem`,
      `New-Item`, `Test-Path`. Bash commands are short and lowercase: `ls`,
      `mkdir`, `test`. A hyphen inside the command name itself happens in no
      other shell.
  - pattern: '2>$null'
    kind: operator
    weight: 9
    note: >
      Throwing away error output. Bash writes `2>/dev/null` with slashes, because
      it has a `/dev` folder and Windows does not. This one token identifies which
      shell a pasted command was written for.
  - pattern: '\$PS[A-Z][A-Za-z]+'
    kind: regex
    weight: 9
    note: >
      Built-in variables start with `$PS`: `$PSVersionTable`, `$PSScriptRoot`,
      `$PSCommandPath`. Bash's built-ins carry no prefix at all, like `$HOME` and
      `$PWD`.
  - pattern: '$_'
    kind: sigil
    weight: 7
    note: >
      The current object inside a pipeline, as in `Where-Object { $_.Length -gt 0 }`.
      Bash has no equivalent because it pipes text rather than objects, so it
      reaches for `$1` and `awk` columns instead.
  - pattern: '-eq'
    kind: operator
    weight: 6
    note: >
      Comparisons are words with a leading hyphen: `-eq`, `-ne`, `-gt`, `-like`,
      `-match`. C-family languages use `==`. Bash also has `-eq`, but only inside
      `[ ]` or `[[ ]]`, so `-eq` with no brackets around it is PowerShell.
  - pattern: '`\s*$'
    kind: regex
    weight: 5
    note: >
      A backtick at the end of a line continues the command onto the next line.
      Bash uses a trailing backslash for that. A backtick in bash means the
      opposite kind of thing: it runs a command and pastes its output in place.

rules_out:
  - pattern: '2>/dev/null'
    kind: operator
    because: >
      Bash. There is no `/dev` folder on Windows, and PowerShell writes `2>$null`.
  - pattern: 'export'
    kind: line_start
    because: >
      Bash sets an environment variable this way. PowerShell writes
      `$env:NAME = 'value'`.
  - pattern: '#!/'
    kind: line_start
    because: >
      A Unix shebang, so the file is bash, sh, or Python. A `.ps1` file has no
      shebang on Windows.
  - pattern: 'fi'
    kind: token
    because: >
      Bash closes an `if` block with `fi`. PowerShell closes it with a brace.
  - pattern: '@echo off'
    kind: regex
    because: >
      Batch. The file is `.bat` or `.cmd`, which is the older Windows shell.
  - pattern: '%\w+%'
    kind: regex
    because: >
      Batch wraps variables in percent signs. PowerShell never does.

project_fingerprint:
  manifests:
    - file: '*.ps1'
      decisive: true
      note: >
        A script. You run it from a PowerShell prompt with `.\name.ps1`, not by
        double-clicking, and Windows blocks it the first time unless the
        execution policy allows local scripts.
    - file: '*.psm1'
      decisive: true
      note: >
        A module: a bundle of functions meant to be loaded with `Import-Module`
        rather than run top to bottom.
    - file: '*.psd1'
      decisive: true
      note: >
        A module manifest. It looks like a hashtable of metadata and it is the
        closest thing PowerShell has to a `package.json`.
    - file: 'Microsoft.PowerShell_profile.ps1'
      note: >
        Your personal startup script, in `C:\Users\<yourname>\Documents\PowerShell`.
        It runs at every new prompt, which makes it the usual suspect when your
        terminal behaves oddly and nobody changed anything.
    - file: 'build.ps1'
      note: >
        A common entry point in Windows projects, doing the job `make` does
        elsewhere.
  entry_points: ['build.ps1', 'install.ps1', 'Microsoft.PowerShell_profile.ps1']

shape:
  blocks: braces
  statement_end: newline
  comment_line: '#'
  comment_block: '<# #>'
  string_quotes: >
    Both, and the difference matters. Double quotes expand `$variables` inside
    them; single quotes are literal. Bash has the same rule, which is the one
    thing the two shells agree on.
  naming: Verb-Noun for commands and functions, PascalCase for parameters, $camelCase for variables
  import_keyword: Import-Module

tooling:
  package_manager: PowerShellGet, being replaced by PSResourceGet
  registry: PowerShell Gallery
  runtime: >
    Windows PowerShell 5.1 ships with Windows 11 and cannot be removed.
    PowerShell 7 installs alongside it as a separate program called `pwsh`.
  install_command: Install-Module <module-name> -Scope CurrentUser
  run_command: pwsh -File .\script.ps1
  test_command: Invoke-Pester

confusable_with:
  - language: bash
    settle_it: >
      Look for the dollar sign. `$env:VAR` and `Verb-Noun` commands mean
      PowerShell. `export VAR=`, `2>/dev/null`, and a `#!/bin/bash` first line
      mean bash. If the escape character at the end of a line is a backslash it
      is bash; a backtick is PowerShell.
    tiebreak: { pattern: '$env:', kind: sigil, favors: powershell }
  - language: batch-script
    settle_it: >
      Batch wraps variables in percent signs, `%USERPROFILE%`, and usually opens
      with `@echo off`. PowerShell uses `$env:USERPROFILE` and never uses percent
      signs for variables.
    tiebreak: { pattern: '@echo off', kind: regex, favors: batch-script }
  - language: csharp
    settle_it: >
      PowerShell can call the same libraries C# uses, so a line like
      `[System.IO.Path]::GetFullPath($p)` appears in both. C# files open with
      `using System;`, put everything inside `class`, and end every statement
      with a semicolon. PowerShell has a `$` on every variable and no class
      around the code.
    tiebreak: { pattern: 'using System', kind: regex, favors: csharp }

errors_look_like:
  sample: |
    Get-ChildItem : Cannot find path 'C:\Users\you\nope' because it does not exist.
    At line:1 char:1
    + Get-ChildItem C:\Users\you\nope
    + ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        + CategoryInfo          : ObjectNotFound: (C:\Users\you\nope:String) [Get-ChildItem], ItemNotFoundException
        + FullyQualifiedErrorId : PathNotFound,Microsoft.PowerShell.Commands.GetChildItemCommand
  recognize_by: >
    A block of red text several lines tall, containing `At line:N char:N`, a copy
    of your own command underlined with tildes, and two indented lines starting
    with `+ CategoryInfo` and `+ FullyQualifiedErrorId`. A bash error is one
    lowercase line and nothing else. PowerShell 7 prints a shorter version by
    default; `Get-Error` brings the full block back.
  patterns:
    - 'At line:\d+ char:\d+'
    - '\+ CategoryInfo\s+:'
    - 'FullyQualifiedErrorId'
    - 'is not recognized as the name of a cmdlet'

meet_it_when: >
  You are in it right now. Windows Terminal opens PowerShell by default, so every
  command in this app runs here unless it says otherwise. You also meet it in
  `.ps1` installer scripts, in the Windows steps of a continuous integration
  workflow, and every time an agent needs to touch your filesystem.

what_agents_get_wrong: >
  The big one, and you will hit it weekly: agents write bash and hand it to you
  for a PowerShell prompt, because their training material is overwhelmingly
  Linux and Mac. The tells are `export KEY=value`, `rm -rf`, `2>/dev/null`,
  `~/.config/...` paths, and `touch`, `grep`, or `sed` used as commands. Some of
  it fails loudly, which is fine. The confusing half fails in a way that points at
  the wrong thing. `curl` and `wget` are aliases for `Invoke-WebRequest` in Windows
  PowerShell 5.1, so a pasted `curl -sSL <url>` complains about a parameter name
  instead of anything you would recognize as a curl problem. `&&` between two
  commands works in PowerShell 7 and is a syntax error in 5.1.
  In a diff, check any line that sets an environment variable and any path with
  forward slashes and a leading tilde. Ask the agent for the PowerShell version by
  name; it knows, it just did not assume.

version_landscape: >
  Two live versions, both on your machine. Windows PowerShell 5.1 ships with
  Windows 11 and stopped getting features years ago. PowerShell 7, run as `pwsh`,
  is the current one and installs separately. Answers online rarely say which they
  mean. Three differences bite in practice: `&&` and `||` between commands work
  only in 7, `ForEach-Object -Parallel` exists only in 7, and 7 prints short
  errors by default while 5.1 prints the tall red block. Run
  `$PSVersionTable.PSVersion` to see which one you are in.

see_also:
  - bash
  - batch-script
  - b1-terminal-shell-command-line
  - c4-path-and-command-not-found
  - g5-environment-variables
  - c7-files-folders-and-paths

keywords: [cmdlet, pwsh, execution policy, ps1, terminal, windows terminal, shell]
---

The shell built into Windows: the program that reads what you type in a terminal
window and runs it. It is also a full programming language, and on Windows 11 it is
the one you are already using.

Check which shell you are in by reading the prompt. `PS C:\Users\<yourname>>` is
PowerShell. The same line without the `PS` in front is Command Prompt, an older and
much weaker shell. See [B1](#b1-terminal-shell-command-line).

## The shape

Commands are called cmdlets, and a cmdlet is always a verb, a hyphen, and a
singular noun. `Get-ChildItem`, `New-Item`, `Test-Path`, `Stop-Process`. Learn the
verbs and you can guess names you have never seen. `Get-ChildItem` is also thirteen
characters for what bash calls `ls`, which is why PowerShell ships `ls` and `dir` as
aliases for it. Someone noticed.

Variables start with `$`. Environment variables sit behind a prefix: `$env:PATH`.
Blocks use braces. A statement ends at the end of the line and needs no semicolon.
Comments are `#` for one line and `<# #>` for a block. The escape character is a
backtick, because the backslash is already the path separator on Windows.

```powershell
$name = 'ada'                       # single quotes: literal text
$greeting = "hello $name"           # double quotes: $name expands
$env:MY_KEY = 'abc123'              # environment variable, this session only
if ($name -eq 'ada') { 'match' }    # -eq, never ==
Get-ChildItem -Path . -Filter *.md | Select-Object Name, Length
```

## The pipe carries objects, not text

This is the real difference between PowerShell and bash, and most of the smaller
differences fall out of it.

In bash, every command prints text, and the pipe hands those characters to the next
command. To get the third column you slice it out by counting spaces with `awk` or
`cut`. When the output format changes, your command quietly breaks.

In PowerShell, a command returns objects, and the pipe hands over the objects
themselves. A file returned by `Get-ChildItem` still has a `Name`, a `Length`, and a
`LastWriteTime` when it arrives at the next command, so you filter on the property
by name and never parse anything:

```powershell
Get-ChildItem | Where-Object Length -gt 1MB | Sort-Object Length -Descending
```

Text is produced once, at the very end, when the last object reaches the screen.
The practical consequence: pasting `| grep something` from a bash answer gets you
nothing useful. The PowerShell equivalents are `Where-Object` for filtering objects
and `Select-String` for searching actual text. For a line-by-line translation table
between the two shells, see [Bash](#bash).

## What it is for

Installing tools, moving files, automating a build step, and reaching into Windows
itself. `Get-Process` and `Get-Service` hand back structured objects rather than
text you have to take apart, which is the whole argument for the language.

Scripts are `.ps1` files. The first one you try to run is usually blocked with
`cannot be loaded because running scripts is disabled on this system`. That is a
safety default rather than a bug. Allow your own scripts once:

```powershell
Set-ExecutionPolicy -Scope CurrentUser RemoteSigned
```

`RemoteSigned` lets scripts you wrote locally run, and still blocks unsigned ones
downloaded from the internet. `-Scope CurrentUser` keeps the change to your account
instead of the whole machine. If the next `.ps1` runs, you are done.

## Reading its errors

Windows PowerShell 5.1 prints a wall of red:

```text
Get-ChildItem : Cannot find path 'C:\Users\<yourname>\nope' because it does not exist.
At line:1 char:1
+ Get-ChildItem C:\Users\<yourname>\nope
+ ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : ObjectNotFound: (...) [Get-ChildItem], ItemNotFoundException
    + FullyQualifiedErrorId : PathNotFound,Microsoft.PowerShell.Commands.GetChildItemCommand
```

The last two lines appear in no other language's output. Five red lines with tildes
under your own command means PowerShell. One lowercase line means bash. PowerShell
7 shortens this to a single line by default, so run `Get-Error` straight after a
failure to see the whole thing.

The message you will meet most: `The term 'x' is not recognized as the name of a
cmdlet, function, script file, or operable program.` That means the program is not
installed, or the folder holding it is not on your `PATH`. See
[C4](#c4-path-and-command-not-found).
