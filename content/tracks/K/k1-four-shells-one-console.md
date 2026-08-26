---
id: k1-four-shells-one-console
title: Four shells, and why the same command is not the same command
type: section
track: K
order: 10
verified: 2026-08-25
volatility: low
verify: '$PSVersionTable.PSVersion'
answer: >
  There are two families: Command Prompt and PowerShell came from Microsoft,
  Bash and Zsh came from Unix, and a command written for one family usually
  fails in the other rather than doing something close to what you wanted.
owns:
  - the two shell families
  - Zsh
  - bash outside Git Bash
  - where each shell is met
see_also:
  - b1-terminal-shell-command-line
  - k2-anatomy-of-a-command
  - k8-the-translation-table
  - k7-same-word-different-program
keywords:
  - powershell vs bash
  - what is zsh
  - what is bash
  - shells compared
  - cmd vs powershell vs bash
  - wsl shell
  - mac terminal commands on windows
  - why does this command not work
---

## More

A shell is a small programming language you type one line at a time. There are four you will
meet, and they come from two different family trees.

**The Microsoft side.**

- **Command Prompt.** The old one, descended from the disk operating system Windows was
  originally built on top of. It is still installed on every Windows machine and it is still
  what a lot of old documentation assumes. Its syntax is odd by modern standards:
  `%VARIABLE%` for a variable, `/S` for a switch.
- **PowerShell.** The modern Windows shell and your default. Commands are named
  `Verb-Noun`, like `Get-ChildItem`, and they pass structured data rather than plain text.
  This is the one to use on Windows unless a card here tells you otherwise.

**The Unix side.**

- **Bash.** The standard shell on Linux servers, inside Docker containers, in the build logs
  of CI (Continuous Integration) systems, and inside Git Bash on your own machine. Almost
  every command example on the internet was written for bash.
- **Zsh.** Bash with better tab completion and prompts. It is the default on macOS, so a
  Mac tutorial is usually a zsh tutorial. For everything in this track, zsh behaves like
  bash. Where they differ, this track says so.

The split matters because a command is not a universal instruction. It is a word in one
particular language. `ls -la` is fluent bash and fails in Command Prompt. `Get-ChildItem` is
fluent PowerShell and fails everywhere else. Neither one is broken.

Two habits follow from that. Read the shell label on any command block before you paste it,
including the ones in this app. And when a copied command fails immediately with a syntax
complaint rather than doing something wrong, suspect the shell before you suspect the
command. [b1](#b1-terminal-shell-command-line) tells you which shell you are sitting in.

## Full

### Where each one turns up

You do not choose these four in a vacuum. Each one arrives attached to something.

| Shell | You get it from |
|---|---|
| PowerShell | Windows Terminal, by default. Nothing to install. |
| Command Prompt | Windows Terminal's dropdown, or by pasting a command someone wrote in 2009. |
| Bash | Git Bash, which arrives with Git for Windows. Also every Linux server, every Docker container, and GitHub Actions. |
| Zsh | A Mac. Also a Linux machine somebody configured on purpose. |

There is a fifth door worth knowing about. WSL (Windows Subsystem for Linux) runs a real
Linux system inside Windows, with a real bash inside that. It is the honest way to run Linux
commands on a Windows machine when Git Bash is not enough. You do not need it to follow this
track, and installing it is a decision, not a step.

### What being in the same family predicts

Family membership is a useful bet, not a guarantee.

Bash and zsh share nearly everything a beginner touches: the same command names, the same
`-flag` style, the same `$VARIABLE` syntax, the same quoting, the same pipes. A bash script
usually runs unchanged in zsh. Zsh adds conveniences on top rather than changing the base.

PowerShell and Command Prompt share much less. They share a heritage and a set of habits,
so both accept `C:\Users\<yourname>` without complaint and both treat `dir` as a real
command. Under that, PowerShell is a different language with a different data model. It runs
most old Command Prompt commands because it deliberately keeps them working, which is a
courtesy rather than a family resemblance.

### Two peace treaties, and their limits

PowerShell defines aliases so that Unix habits mostly work. `ls`, `cat`, `cp`, `mv`, `rm`
and `pwd` all exist there and do roughly what you expect.

This is the single most productive trap in the console. The alias points at a PowerShell
command, so the name works and the flags do not:

```powershell
ls -Force
```

Lists everything here including hidden items. Correct PowerShell.

```powershell
ls -la
```

Fails with a complaint about a parameter named `la`, because `-la` is bash's spelling and
`ls` here is PowerShell wearing a bash name. [k7](#k7-same-word-different-program) has the
full list of names that mean two things.

Git Bash is the mirror image of the same treaty. It gives you real bash on Windows, and it
translates paths on the way through, so `/c/Users/<yourname>` means `C:\Users\<yourname>`.
The translation is good and not perfect. Windows programs launched from Git Bash sometimes
receive a path they cannot read, which is why a tool that works in PowerShell can fail in
Git Bash with a path error and no other symptom.

### Which one to actually use

Use PowerShell for your own work on Windows. It is the default, it is what this app assumes,
and it is the one where the error messages are written for the system you are on.

Use Git Bash when a command was written for bash and you would rather run it than translate
it. Claude Code also prefers to run its own commands there, so you will see bash syntax in
agent output whether or not you chose it.

Learn to read all four. You do not have to write Command Prompt to recognize `%PATH%` in a
Stack Overflow answer and know that the answer is fifteen years old.

### Confirming which family you are in

One command each, no memorization needed. Run this:

```powershell
$PSVersionTable.PSVersion
```

A small table with `Major` and `Minor` columns means PowerShell. An error saying the command
is not recognized means Command Prompt. A blank line means bash or zsh, because both expand
an unset variable to nothing and print the rest.

```bash
echo $SHELL
```

In bash and zsh this prints a path ending in `bash` or `zsh`, which settles the last pair.
In PowerShell it prints an empty line, because `$SHELL` is a variable PowerShell has never
set. In Command Prompt it prints the literal text `$SHELL` back at you, because `$` means
nothing there at all.
