---
id: b1-terminal-shell-command-line
title: Terminal, shell, and command line
type: section
track: B
order: 10
verified: 2026-08-02
volatility: low
verify: '$PSVersionTable.PSVersion'
answer: >
  The terminal is the window, the shell is the program inside it that reads what
  you type, and the command line is the idea. Tell them apart by the prompt:
  PowerShell starts with `PS`, Command Prompt does not.
owns:
  - terminal
  - shell
  - PowerShell vs CMD vs Git Bash
  - telling them apart
see_also:
  - k1-four-shells-one-console
  - k3-the-first-ten-commands
  - c4-path-and-command-not-found
  - b2-install-git
  - c7-files-folders-and-paths
  - b6-install-claude-code
keywords:
  - what is a terminal
  - powershell vs command prompt
  - git bash
  - which shell am i in
  - open a terminal
  - console
  - windows terminal
---

## More

Three words for the same neighborhood, and they are not synonyms.

- **Terminal.** The window. On Windows the modern one is **Windows Terminal**, which is what
  opens when you type "Terminal" into the Start menu.
- **Shell.** The program running inside that window that reads what you type and does it.
  Windows has three you will meet: PowerShell, Command Prompt, and Git Bash.
- **Command line.** The general idea of driving a program by typing instead of clicking. The
  same idea is called a CLI (Command-Line Interface) in documentation.

One window, several possible shells, and the shell is the part that decides whether a command
works. Tell which one you are in by looking at the prompt, the text sitting to the left of
your cursor:

```text
PS C:\Users\you>                                   PowerShell
C:\Users\you>                                      Command Prompt
you@DESKTOP MINGW64 ~/dev/my-project (main)        Git Bash
```

`PS` at the front means PowerShell. A bare drive letter means Command Prompt. A username, an
`@`, and a green branch name in brackets means Git Bash.

**Use PowerShell unless a card tells you otherwise.** It is the default in Windows Terminal
and it is what most instructions here assume. Git Bash arrives with Git for Windows
([b2](#b2-install-git)), handles the handful of commands written in bash syntax, and is the
shell Claude Code prefers to run its own commands in.

When any instruction says "run this in your terminal," it means: open Windows Terminal, check
the prompt to confirm which shell you are in, paste, press Enter.

Getting this wrong produces one specific and very common failure. Paste the Claude Code
installer into Command Prompt and you get `'irm' is not recognized`, which reads like the
installer is broken and means you are in the wrong shell.
[irm-not-recognized-in-cmd](#irm-not-recognized-in-cmd) has it in full.

## Full

### Opening each one

**Windows Terminal.** Start menu, type "Terminal", press Enter. It opens on PowerShell. The
small down-arrow next to the tab strip lists every shell installed on the machine, and
picking one opens it in a new tab. That dropdown is the fastest way to switch.

**Command Prompt.** In that same dropdown, or Start menu, "Command Prompt". You will rarely
choose it on purpose. You will land in it by accident, which is why the prompt check matters.

**Git Bash.** Appears in the dropdown after you install Git for Windows. It also adds a
right-click entry in File Explorer, "Open Git Bash here", which opens a shell already sitting
in that folder. On Windows 11 it lives under "Show more options".

### When the prompt has been customized

Some setups change the prompt, and then it tells you nothing. One command each settles it.

```powershell
$PSVersionTable.PSVersion
```

In PowerShell this prints a small table with `Major`, `Minor`, and `Build` columns. In
Command Prompt it prints `'$PSVersionTable.PSVersion' is not recognized as an internal or
external command`. In Git Bash it prints an empty line, because bash expands the unset
variable to nothing.

```bash
uname -s
```

In Git Bash this prints a string beginning `MINGW64`. In PowerShell and Command Prompt it
fails, because `uname` is not a Windows command.

### What actually differs

Each shell is a separate program with its own vocabulary, so the same intention is spelled
differently in each.

| Task | PowerShell | Command Prompt | Git Bash |
|---|---|---|---|
| List the files here | `ls` or `dir` | `dir` | `ls` |
| Which folder am I in | `pwd` | `cd` with nothing after it | `pwd` |
| Your home folder | `~` | `%USERPROFILE%` | `~` |
| Read one variable | `$env:PATH` | `%PATH%` | `$PATH` |
| Clear the screen | `cls` | `cls` | `clear` |

Two differences cause most of the confusion:

**Aliases that are not the thing you expect.** In the PowerShell that ships with Windows,
`curl` is an alias for a built-in PowerShell command and does not accept curl's flags. Paste
a `curl -H ...` line from a tutorial and you get a strange complaint about a parameter,
rather than a clean "not found". In Git Bash, `curl` is the real program. When a downloaded
command behaves nothing like its documentation, the shell is the first suspect.

**Slashes and escapes.** PowerShell accepts `C:\dev\app` and `C:/dev/app` equally. Git Bash
treats `\` as an escape character, so Windows-style paths break there and you write
`/c/dev/app` instead. Paths with spaces need quotes in all three.
[c7](#c7-files-folders-and-paths) covers this properly.

### Why a command exists in one shell and not another

`irm` is PowerShell's short name for `Invoke-RestMethod`, which fetches something over the
network. Command Prompt was written years before that command existed and has never heard of
it. The reverse happens too: bash syntax like `cat > file << 'EOF'` is meaningless to
PowerShell.

This is why every command block in this app is labeled with the shell it needs. The label is
not decoration. A bash command pasted into PowerShell fails in a way that looks like the
instructions were wrong.

A different and more frequent cause of "not recognized" is a program that is genuinely
installed, in a terminal that has not noticed yet. That one is
[c4](#c4-path-and-command-not-found), and the fix is closing and reopening the terminal.

### Running as administrator

Right-clicking Terminal and choosing "Run as administrator" opens a window with permission to
change system-wide settings. You need it far less often than tutorials imply, and installing
tools with `winget` for your own account does not need it.

The rule: if a card here does not say administrator, do not use administrator. An elevated
terminal starts in a different folder, sometimes sees a different set of installed programs,
and creates files your normal account cannot edit afterward. That last one produces a genuine
mess in a git repository.

### The one habit worth building

Look at the prompt before you paste. It takes half a second and it removes an entire category
of failure where the tool is fine, the command is fine, and the two were never introduced.
