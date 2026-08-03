---
id: bash-command-not-found
title: "bash: x: command not found"
type: error
verified: 2026-08-02
volatility: low

language: bash
category: not-found

# Prints the full path to the program if the shell can find it, and nothing at
# all if it cannot.
verify: which <name>

sample: |
  nyx@DESKTOP-4K2M MINGW64 ~/dev/scraper (main)
  $ pnpm dev
  bash: pnpm: command not found

patterns:
  - "command not found"
  - "No such file or directory"

means: >
  The shell looked through every folder on PATH, found nothing with that name, and stopped.
  PATH is the list of folders searched when you type a command that is not a full path to a
  file. You are in Git Bash rather than PowerShell, and the two do not always see the same
  set of programs, because Git Bash translates Windows PATH entries and skips anything it
  cannot make sense of.

fix_ladder:
  - try: Close Git Bash, open it again, and rerun the command.
    why: >
      Assumes the program was installed after this window opened. A shell reads PATH once
      at startup. This is the most common cause and the cheapest to eliminate.

  - try: Ask the shell where it thinks the program is.
    command: which <name>
    shell: bash
    why: >
      Assumes the program is findable and the name was mistyped. Output means the program
      is there and your command line was wrong. No output means the name really is missing
      from PATH.

  - try: Check whether PowerShell can find the same command.
    command: Get-Command <name>
    shell: powershell
    why: >
      Assumes the program is installed but invisible to this shell specifically. If
      PowerShell finds it and Git Bash does not, the install is fine and the problem is
      PATH translation between the two, which means you can run the command in PowerShell
      right now and sort out Git Bash later.

  - try: Look at what PATH actually contains in this shell.
    command: echo $PATH | tr ':' '\n'
    shell: bash
    why: >
      Assumes the folder is missing from this shell's PATH. Git Bash writes paths in Unix
      style, so `C:\Program Files\nodejs` appears as `/c/Program Files/nodejs`. A folder
      that is absent from this list is invisible no matter what Windows thinks.

  - try: Run it through its package manager instead of by bare name.
    command: npx <name> --version
    shell: bash
    why: >
      Assumes the tool is installed inside the project rather than globally. Project-local
      tools live in `node_modules\.bin` and are not on PATH at all. `npx` looks there
      first, and the same idea applies to `python -m <name>` for Python tools.

if_none_worked: >
  Paste the whole error, the prompt line above it showing `MINGW64` or your shell name, and
  the output of `echo $PATH`. The prompt line is what identifies which shell you are in, and
  people always cut it because it looks like decoration.

see_also:
  - c4-path-and-command-not-found
  - b1-terminal-shell-command-line
  - g4-environments-and-isolation
  - bash

keywords:
  - command not found
  - git bash
  - MINGW64
  - which command
  - bash path
---

Git Bash comes with Git for Windows and gives you a Unix-style shell on a Windows machine.
Claude Code prefers it, so you will end up there whether or not you chose it.

The catch is that Git Bash keeps its own view of PATH, converted from the Windows one at
startup. Most entries survive the conversion. Some do not, and the ones that fail tend to
be the ones added most recently or written with unusual quoting.

Read the prompt to know where you are. Git Bash shows your username, the machine name,
`MINGW64`, the folder in Unix form with a forward slash, and the git branch in parentheses.
PowerShell shows `PS` and a Windows path with backslashes. Same window, different rules.
