---
id: k4-finding-things
title: Finding a file, finding text, finding a program
type: section
track: K
order: 40
verified: 2026-08-25
volatility: low
verify: Get-Command git
answer: >
  Three different searches get run interchangeably and are not the same: finding a file by
  its name, finding text inside files, and finding which program a command name actually
  runs.
owns:
  - searching from the console
  - searching inside files
see_also:
  - k3-the-first-ten-commands
  - c4-path-and-command-not-found
  - k5-pipes-and-chaining
  - k7-same-word-different-program
keywords:
  - search for a file in terminal
  - grep on windows
  - find text in files powershell
  - findstr
  - select-string
  - where is a program installed
  - which command
  - search recursively
  - find file by name
---

## More

"Find it" means three different things and each one has its own command.

**Find a file by name.** You know roughly what it is called and not where it is.

```powershell
Get-ChildItem -Recurse -Filter "*.env"
```

Searches this folder and everything under it for names matching the pattern. `-Recurse`
descends, `-Filter` matches on the name. The `*` stands for any run of characters, so
`*.env` means anything ending in `.env`.

**Find text inside files.** You know a phrase that appears somewhere and not which file.

```powershell
Select-String -Path *.ts -Pattern "TODO"
```

Prints every line containing `TODO`, with the filename and line number in front of it. This
is PowerShell's version of the command everyone online calls `grep`.

`Select-String` has no recursive option of its own, which surprises everyone once. To search
subfolders, find the files first and pipe them in:

```powershell
Get-ChildItem -Recurse -Filter *.ts | Select-String -Pattern "TODO"
```

**Find the program behind a name.** You typed something and got the wrong version, or you
want to know what a name actually runs.

```powershell
Get-Command git
```

Prints the full path to the executable that runs when you type `git`. `where.exe git` does
the same and lists every match, which matters when two copies are installed and the wrong
one is winning. If this prints nothing at all, the program is either not installed or not on
your PATH, and [c4](#c4-path-and-command-not-found) is the card for that.

The one that surprises people: filename search is fast, content search is not. Searching
inside every file under a project folder means opening thousands of them, and a folder with
`node_modules` in it contains far more files than it looks like it does. Narrow the search
before you widen it.

## Full

### Filtering, and why `-Filter` beats the alternatives

Two ways to narrow a file search look identical and are not:

```powershell
Get-ChildItem -Recurse -Filter "*.log"
```

```powershell
Get-ChildItem -Recurse -Include "*.log"
```

`-Filter` hands the pattern to Windows itself, which applies it while walking the disk.
`-Include` fetches everything and discards the misses afterward. On a small folder you will
not notice. On a project with a `node_modules` folder, the first finishes and the second
appears to hang. Use `-Filter` and reach for `-Include` only when you need more than one
pattern.

Skipping the noise is worth learning as a reflex:

```powershell
Get-ChildItem -Recurse -Filter "*.ts" | Where-Object FullName -notmatch "node_modules"
```

Finds TypeScript files and drops anything living inside a dependency folder. The `|` is a
pipe, which sends the results of one command into another. [k5](#k5-pipes-and-chaining)
covers it properly.

### Searching inside files, in each shell

The command changes name in every shell and the idea does not.

```powershell
Select-String -Path .\src\*.ts -Pattern "apiKey"
```

PowerShell. Prints `file.ts:42:  const apiKey = ...` for every hit. Case-insensitive by
default, which is the opposite of what Unix users expect. Add `-CaseSensitive` when you need
it.

```bash
grep -rn "apiKey" src/
```

Bash and zsh. `-r` recurses into folders, `-n` prints line numbers, and the search is
case-sensitive unless you add `-i`. This is the command in almost every answer online.

```cmd
findstr /s /n "apiKey" *.ts
```

Command Prompt. `/s` recurses, `/n` numbers the lines. It exists, it works, and it has
almost none of grep's options.

A useful pattern in any of them: search for the error message, not for the code. When
something prints `Cannot read properties of undefined`, that string is usually sitting in a
file you can find, and finding it is faster than reasoning about where it came from.

### Finding a program, and why two copies is common

```powershell
Get-Command node
```

Prints one row: the command type, the name, the version, and the source path.

```powershell
Get-Command node -All
```

Prints every match in PATH order. The first one is the one that runs. Two installs of Node
is a normal state on a working machine, one from an installer and one from a version
manager, and the symptom is that `node --version` disagrees with what you thought you
installed.

```bash
which node
```

Bash and zsh. Prints the path of the one that would run. `type node` also tells you whether
the name is a real program or a shell alias, which is the question
[k7](#k7-same-word-different-program) exists to answer.

Command Prompt has `where node`, and PowerShell has a `where` of its own that means
something entirely different. In PowerShell, always write `where.exe`. The `.exe` is not
optional and leaving it off produces a confusing non-answer rather than an error.

### Searching your own history

The fourth kind of finding, and the one that saves the most time: you ran the right command
last Tuesday and cannot remember it.

```powershell
Get-History
```

Lists the commands you have run in this window, numbered. It resets when the window closes.

For the persistent history that survives a reboot, and the interactive search that walks it,
see [k6](#k6-history-completion-and-keys).
