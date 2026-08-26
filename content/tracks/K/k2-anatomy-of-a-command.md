---
id: k2-anatomy-of-a-command
title: The parts of a command
type: section
track: K
order: 20
verified: 2026-08-25
volatility: low
verify: Get-Help Get-ChildItem
danger: >
  Names `rm -rf` as an example of stacked flags. That command deletes a folder and everything
  under it with no confirmation and no recycle bin. It is shown here to be read, not run.
  See k3 for deleting things on purpose.
answer: >
  A command line is a program name followed by things that change what it does: flags
  starting with a dash, values attached to those flags, and the target it acts on, in that
  rough order.
owns:
  - command anatomy
  - flags and switches
  - reading a usage line
  - asking a command for help
see_also:
  - k1-four-shells-one-console
  - k3-the-first-ten-commands
  - c7-files-folders-and-paths
  - k7-same-word-different-program
keywords:
  - what is a flag
  - what does the dash mean
  - command line arguments
  - what is a switch
  - single dash double dash
  - how to get help for a command
  - what does -f mean
  - usage line
---

## More

Every command line, in every shell, is the same four parts. Only some of them show up every
time.

```powershell
git commit -m "fix the login button"
```

- `git` is the **program**. The thing that actually runs.
- `commit` is the **subcommand**. Big programs are split into verbs, and this picks one.
  Not everything has these. `dir` has no subcommands at all.
- `-m` is a **flag**, sometimes called a switch or an option. It changes behavior.
- `"fix the login button"` is the **value** belonging to `-m`.

Anything left over is the **target**: the file, folder, or branch the command acts on.

Two rules about flags carry most of the day.

**A single dash means a short flag, and a double dash means a long one.** They usually come
in pairs that mean the same thing: `-m` and `--message`, `-r` and `--recursive`, `-f` and
`--force`. Short ones are for typing, long ones are for reading. Use the long form when you
write something down for later, because `--force` explains itself six months from now and
`-f` does not.

**Order is looser than it looks, and spaces are not.** Most commands accept flags before or
after the target. Almost none of them forgive a missing space between a flag and its value,
or a value with a space in it that you did not quote. See
[c7](#c7-files-folders-and-paths) for the quoting rules, because that is where this bites
hardest.

When a flag takes no value it is on or off by its presence. `git commit -a` means "include
everything already tracked". There is nothing to attach to it.

## Full

### Three flag styles, one per family

The three shells you will meet spell flags differently, and the style tells you where a
command came from.

| Style | Looks like | Used by |
|---|---|---|
| Single dash, one letter | `-r`, `-f`, `-v` | Unix programs, git, node, most things |
| Double dash, a word | `--recursive`, `--force` | The same programs, spelled out |
| Forward slash | `/S`, `/Q`, `/Y` | Command Prompt's own commands |

The slash style is the tell for a Command Prompt command. When you see `/S` in an answer
online, that answer will not work in bash and may or may not work in PowerShell.

Short flags can usually be stacked. `-r -f` and `-rf` mean the same thing to most Unix
programs. This is why the notorious `rm -rf` looks like one word: it is two flags holding
hands, `-r` for "go into every subfolder" and `-f` for "do not ask me about any of it". Read
that one, do not run it. [k3](#k3-the-first-ten-commands) covers deleting on purpose, with
the safe form first.

### PowerShell does it differently, and it is worth the paragraph

PowerShell has no short flags in the Unix sense. It has **named parameters**, always a
single dash and always a whole word:

```powershell
Get-ChildItem -Path C:\dev -Recurse -Filter *.log
```

Three parameters, spelled out. You can shorten any of them to the shortest unambiguous
prefix, so `-Rec` works and `-R` does too as long as no other parameter on that command
starts with R. Tab completion writes them out for you, which is the better habit. See
[k6](#k6-history-completion-and-keys).

PowerShell command names are also predictable in a way nothing else is. They are all
`Verb-Noun`: `Get-Process`, `Stop-Process`, `New-Item`, `Remove-Item`, `Set-Location`. Once
you know the noun you can usually guess the command, and this one command lists every verb
in use:

```powershell
Get-Verb
```

Prints about a hundred approved verbs with a description of each. Skim it once. It makes the
whole shell feel less like vocabulary and more like grammar.

### Reading a usage line

Every command's help output opens with a usage line, and it uses a notation that nobody
explains.

```text
usage: git branch [-a | -r] [--list] [<pattern>...]
```

- **Square brackets mean optional.** `[--list]` can be left out.
- **A pipe means pick one.** `[-a | -r]` means either, and not both.
- **Angle brackets mean you substitute something.** `<pattern>` is a placeholder. Type your
  own value, without the brackets.
- **Three dots mean repeatable.** `<pattern>...` accepts more than one.
- **Anything with no brackets is required.**

That notation is the same in git, in Node, in Docker, and in the Linux manual. Learning it
once pays out everywhere.

### Making a command explain itself

You never have to guess. Every command will describe itself, and the request is different in
each shell.

```powershell
Get-Help Get-ChildItem -Examples
```

PowerShell's own help, with worked examples at the bottom, which is the part actually worth
reading. Drop `-Examples` for the full page. The first time you run it, PowerShell may offer
to download the latest help files, and saying yes is fine.

```powershell
git commit --help
```

For programs that are not PowerShell commands, `--help` is the near-universal request. Git
opens its manual in a browser window on Windows. Most other tools print to the screen.

```bash
man ls
```

In bash and zsh, `man` opens the manual page. Press `q` to get out of it, which is the piece
of information most people are missing when they first land in one. Arrow keys and Page Down
scroll, `/word` searches.

```cmd
robocopy /?
```

Command Prompt's own commands answer to `/?` and nothing else.

### The one habit worth building

Before you run an unfamiliar command that changes something, run its help first and read
what the flags do. It costs fifteen seconds. An agent that hands you a command with `-f` in
it has made a decision on your behalf, and `-f` almost always means "do it even though
something is telling you not to".
