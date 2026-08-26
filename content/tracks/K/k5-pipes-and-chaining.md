---
id: k5-pipes-and-chaining
title: Pipes, and running one command after another
type: section
track: K
order: 50
verified: 2026-08-25
volatility: low
verify: Get-Process | Select-Object -First 3
answer: >
  The pipe character sends one command's output into the next command as its input, and the
  three chaining operators decide whether the next command runs always, only on success, or
  only on failure.
owns:
  - the pipe
  - chaining operators
  - objects versus text
see_also:
  - f3-exit-codes-and-streams
  - k4-finding-things
  - k1-four-shells-one-console
  - k2-anatomy-of-a-command
keywords:
  - what does the pipe symbol do
  - vertical bar command line
  - double ampersand
  - run two commands at once
  - chain commands
  - pipe output to another command
  - powershell objects
---

## More

The pipe is the `|` character, above the backslash on most keyboards. It takes whatever the
command on its left produced and hands it to the command on its right, instead of printing
it to the screen.

```powershell
Get-Process | Select-Object -First 3
```

`Get-Process` produces a list of every running program. Instead of printing hundreds of
rows, the list goes into `Select-Object`, which keeps the first three and prints those. You
should see a three-row table.

Read a pipeline left to right as a sentence: get all the processes, then keep three. Long
pipelines are the same sentence with more clauses.

The other half is chaining, which is different. Chaining runs separate commands in sequence
rather than feeding one into the next. There are three operators and they differ only in
when the second command runs.

```powershell
git add . ; git status
```

A semicolon runs the second command no matter what happened to the first.

```powershell
git add . && git commit -m "fix login"
```

Two ampersands run the second command **only if the first succeeded**. This is the one you
want almost every time. If `git add` fails, you do not want a commit attempt on top of the
failure, hiding the real error under a second one.

```powershell
git push || Write-Output "push failed, check the remote"
```

Two pipes run the second command **only if the first failed**. Less common, useful for
saying something human when a step goes wrong.

Success and failure here mean the exit code, which [f3](#f3-exit-codes-and-streams) owns.
Zero means it worked.

## Full

### The version trap on `&&`

`&&` and `||` are ordinary in bash and were added to PowerShell in version 7. Windows
PowerShell 5.1, which is the one built into Windows and still the default in some places,
does not have them. Pasting `&&` there produces:

```text
The token '&&' is not a valid statement separator in this version.
```

That message is accurate and reads like a syntax error in the command you copied. Check
which version you have:

```powershell
$PSVersionTable.PSVersion
```

A `Major` of 7 or higher has the operators. A `Major` of 5 does not, and the workaround is a
semicolon plus a manual check, or installing PowerShell 7 with
`winget install Microsoft.PowerShell`.

Command Prompt has had `&&` and `||` for decades, which makes it the one place where the old
shell is friendlier than the new one.

### The genuine difference: objects against text

This is the one place where PowerShell is not a dialect of the same idea.

In bash, zsh and Command Prompt, a pipe carries **text**. The first command prints lines,
the second command reads lines. Everything in between is string handling, which is why bash
pipelines are full of tools that slice text into columns.

```bash
ls -la | grep ".log"
```

`ls` prints a table as text, `grep` keeps the lines containing `.log`. Nothing understands
what a file is. It is all characters.

In PowerShell, a pipe carries **objects**. `Get-ChildItem` does not print a table into the
pipe, it passes actual file objects that have real properties, and the table you see is just
how PowerShell chose to display them at the end.

```powershell
Get-ChildItem | Where-Object Length -gt 1MB | Sort-Object Length -Descending
```

Finds files here over one megabyte, largest first. Nothing parsed any text. `Length` is a
property on the object, `1MB` is a number PowerShell understands, and the comparison is
arithmetic.

Two consequences follow.

**Bash pipeline recipes rarely translate line by line.** A bash answer that pipes through
`awk` to grab the third column has no PowerShell equivalent, because in PowerShell you would
name the property instead. Translating the shape of the answer works. Translating the
commands does not.

**PowerShell output is not always what it looks like.** A command can display three columns
and carry twenty properties. This shows you everything:

```powershell
Get-ChildItem | Select-Object -First 1 | Format-List *
```

Prints every property of one file rather than the four the table shows. Useful when you need
a value the default display is hiding.

### Where redirection fits

The pipe's close relative is the `>` character, which sends output into a file rather than
into another command.

```powershell
Get-ChildItem -Recurse > files.txt
```

Writes the listing to `files.txt`, overwriting it. `>>` appends instead. Both work in all
four shells with the same spelling, which makes them a rare piece of common ground.

The part that catches people is that `>` captures only normal output, and error messages
travel on a separate channel that keeps going to the screen. That distinction, and the
`2>&1` incantation that merges them, belongs to [f3](#f3-exit-codes-and-streams).

### Building a pipeline without breaking anything

Add one stage at a time and look at the output after each. A pipeline is easy to write and
hard to read, and a five-stage line that produces nothing gives you no clue which stage
emptied it.

```powershell
Get-ChildItem -Recurse -Filter *.log
```

Look at it. Then narrow:

```powershell
Get-ChildItem -Recurse -Filter *.log | Where-Object Length -gt 1MB
```

Look again. Only then add the stage that acts. This matters most when the last stage deletes
something: run the pipeline without it first, read the list, then append the deletion.
