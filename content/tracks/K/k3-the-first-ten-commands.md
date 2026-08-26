---
id: k3-the-first-ten-commands
title: The first ten commands, in all four shells
type: section
track: K
order: 30
verified: 2026-08-25
volatility: low
verify: Get-Location
danger: >
  Covers deleting files and folders. Nothing removed by `Remove-Item`, `del`, or `rm` goes to
  the Recycle Bin, and there is no undo. The safe move is listing what you are about to
  delete first, which this card shows before it shows the delete.
answer: >
  Ten commands cover almost everything you will type in a week: where am I, what is here, go
  there, show me that file, make a folder, copy, move, delete, print, and clear the screen.
owns:
  - the day-one command set
see_also:
  - k2-anatomy-of-a-command
  - c7-files-folders-and-paths
  - k4-finding-things
  - k8-the-translation-table
keywords:
  - basic terminal commands
  - console quick start
  - cd ls pwd
  - how to make a folder in terminal
  - how to delete a file in powershell
  - copy a file command line
  - list files
  - clear the screen
  - basic commands cheat sheet
---

## More

Ten verbs. Learn them in PowerShell first, because that is where you are, and read the other
columns so a bash tutorial stops looking like a foreign language.

Every one of these is shown here in PowerShell. [k8](#k8-the-translation-table) has the same
list in all four shells side by side.

**Where am I.**

```powershell
Get-Location
```

Prints the folder this window is currently sitting in. `pwd` is a shorter alias for the
same thing. Run this whenever a command says it cannot find a file, because standing in the
wrong folder is the most common reason.

**What is in here.**

```powershell
Get-ChildItem
```

Lists the files and folders in the current directory. `ls` and `dir` are aliases. Add
`-Force` to include hidden items, which is how you see the `.git` folder.

**Go somewhere.**

```powershell
Set-Location C:\Users\<yourname>\projects
```

Moves this window into that folder. `cd` is the alias everybody uses. `cd ..` goes up one
level. `cd ~` goes to your home folder. See [c7](#c7-files-folders-and-paths) for what a
path is and what to do about spaces in one.

**Show me that file.**

```powershell
Get-Content package.json
```

Prints a text file to the screen. `cat` and `type` are aliases. On a large file this scrolls
past faster than you can read, so add `-Head 20` for the first twenty lines or `-Tail 20`
for the last twenty.

**Make a folder.**

```powershell
New-Item -ItemType Directory demo
```

Creates a folder called `demo` here. `mkdir demo` is the short form and does the same thing.

**Make an empty file.**

```powershell
New-Item -ItemType File notes.txt
```

Creates an empty file. There is no `touch` in PowerShell, which is the bash name for this
and the one most tutorials use.

**Copy, move, rename.**

```powershell
Copy-Item notes.txt notes-backup.txt
```

Copies a file. `Move-Item` moves one, and moving a file to a new name in the same folder is
how you rename it. `cp`, `mv` and `ren` are the aliases.

**Delete.**

```powershell
Remove-Item notes.txt
```

Deletes the file. It does not go to the Recycle Bin and there is no undo. Read the next
tier before you use this on anything you have not backed up.

**Print something, and clear the screen.**

```powershell
Write-Output "hello"
```

Prints text. `echo` is the alias. Useful mostly for checking what a variable contains.
`cls` or `Clear-Host` wipes the screen, which scrolls nothing away permanently and just
gives you a clean view.

## Full

### The check step for each one

An instruction you cannot verify is a rumor. Each of these has an obvious confirmation, and
building the habit now costs nothing.

| You ran | It worked if |
|---|---|
| `Set-Location <path>` | The prompt now shows that path |
| `New-Item -ItemType Directory demo` | `Get-ChildItem` lists `demo` with a `d` in the Mode column |
| `Copy-Item a.txt b.txt` | `Get-ChildItem` lists both, with the same Length |
| `Remove-Item a.txt` | `Get-ChildItem` no longer lists it |
| `Get-Content file.txt` | Text appears. An empty file prints nothing, which is not an error |

`Get-ChildItem` is the check step for most of these, which is a good reason to learn its
alias `ls` and use it constantly.

### Deleting, properly

This is the one command in the list that can cost you an afternoon, so it gets its own
treatment.

`Remove-Item` does not use the Recycle Bin. Neither does `del` in Command Prompt, nor `rm`
in bash. A file deleted from a console is gone in a way a file deleted in File Explorer is
not.

The safe habit is one extra command. List first, delete second, using the same filter both
times:

```powershell
Get-ChildItem *.log
```

Shows you exactly what matches. Read the list. If it is what you meant, run the delete with
the identical pattern:

```powershell
Remove-Item *.log
```

Deletes those and only those. The pattern being identical is the whole point: you have
already seen the result.

Two flags change how much damage is possible.

- `-Recurse` descends into every subfolder. Without it, deleting a folder that has anything
  in it prompts for confirmation. With it, nothing asks.
- `-Force` removes hidden and read-only files and suppresses prompts.

Together on a folder they delete a tree with no questions asked. That combination is worth
recognizing on sight in an agent's proposed command, because it looks unremarkable and is
not.

One command undoes the risk entirely on a repository: if the folder is under git and the
work is committed, a mistaken delete is recoverable. See
[d11](#d11-when-you-lose-work).

### The bash column, since you will read it constantly

Almost every tutorial online is written in bash. You need to read it even if you never type
it.

```bash
pwd
ls -la
cd ~/projects
cat package.json
mkdir demo
touch notes.txt
cp notes.txt notes-backup.txt
mv notes.txt archive/notes.txt
rm notes.txt
clear
```

Two things to notice. Bash uses forward slashes and `~` for your home folder, both of which
PowerShell also accepts. And `ls -la` means "long format, including hidden", which is why
you see it on nearly every line of every tutorial: `-l` for the detailed table, `-a` for
"all, including the dotfiles".

### Command Prompt, for when you land in it by accident

You will not choose this shell, and you will end up in it. Four commands get you back out.

```cmd
cd
```

With nothing after it, Command Prompt's `cd` prints the current folder rather than changing
it. This is the one real trap in the list, because in every other shell `cd` with no
argument sends you home.

```cmd
dir
```

Lists the folder. `dir /a` includes hidden items.

```cmd
type package.json
```

Prints a file.

```cmd
cd /d D:\work
```

Changes folder *and* drive. Without `/d` a plain `cd D:\work` silently does nothing visible,
because Command Prompt keeps a separate current folder per drive letter. That behavior
predates most people reading this and has never been removed.
