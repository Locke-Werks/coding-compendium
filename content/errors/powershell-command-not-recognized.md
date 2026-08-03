---
id: powershell-command-not-recognized
title: "The term 'x' is not recognized as the name of a cmdlet"
type: error
verified: 2026-08-02
volatility: low

language: powershell
category: not-found

# Swap in the name of the command that failed. If it prints a path, this
# terminal can find the program and the original command line had a typo.
verify: Get-Command <name>

sample: |
  gh : The term 'gh' is not recognized as the name of a cmdlet, function, script file, or operable program. Check the spelling of the name, or if a path was included, verify that the path is correct and try again.
  At line:1 char:1
  + gh --version
  + ~~
      + CategoryInfo          : ObjectNotFound: (gh:String) [], CommandNotFoundException
      + FullyQualifiedErrorId : CommandNotFoundException

# Windows PowerShell 5.1 says "the name of a cmdlet". PowerShell 7 says "a name
# of a cmdlet". Both are matched. Never anchor on the command name itself.
patterns:
  - "is not recognized as the name of a cmdlet"
  - "is not recognized as a name of a cmdlet"
  - "CommandNotFoundException"

means: >
  PowerShell searched every folder listed in PATH, found nothing with that name, and
  stopped. PATH is the list of folders Windows looks through when you type a command
  that is not a full path to a file. Three things produce this message: the program
  is not installed, the program is installed in a folder PATH does not mention, or
  this terminal window is holding a copy of PATH from before the install happened.

fix_ladder:
  - try: Close this terminal window, open a new one, and run the command again.
    why: >
      Assumes the program is installed and its folder is on PATH, but this window read
      PATH when it opened and has not looked since. This is the cause most of the time
      and it costs one window to rule out.

  - try: Ask PowerShell where the command lives.
    command: Get-Command <name>
    shell: powershell
    why: >
      Assumes the program is findable and you mistyped the name. If this prints a path,
      the program is fine and the original line was wrong. If it prints the same
      not-recognized text, the name really is missing from PATH.

  - try: Check that the install actually finished.
    command: winget list --name <name>
    shell: powershell
    why: >
      Assumes the install failed quietly. Installers can exit with an error you scrolled
      past. If nothing is listed, nothing was installed and there is no PATH question to
      answer.

  - try: Find the executable by hand and look at where it sits.
    command: Get-ChildItem -Path C:\ -Filter <name>.exe -Recurse -ErrorAction SilentlyContinue | Select-Object -First 3
    shell: powershell
    why: >
      Assumes the program is installed but its folder was never added to PATH. This is
      slow and it searches the whole drive, so let it run. Once you have the folder, add
      it to PATH permanently, then open a new terminal. See g5-environment-variables.

  - try: Check that you are in PowerShell and not Command Prompt.
    command: $PSVersionTable.PSVersion
    shell: powershell
    why: >
      Assumes you are in the wrong shell. Command Prompt phrases this failure as "is not
      recognized as an internal or external command" instead, so if you are seeing the
      cmdlet wording you are in PowerShell and this is not your problem.

if_none_worked: >
  Paste the whole error including the `At line:1 char:1` block and the `CategoryInfo` line,
  the exact command you typed, and the output of `$env:Path -split ';'`. That last one is
  the piece everybody trims and it is the only thing that shows whether the folder is
  missing from PATH or present but pointing somewhere that no longer exists.

see_also:
  - c4-path-and-command-not-found
  - b1-terminal-shell-command-line
  - g5-environment-variables
  - powershell

keywords:
  - not recognized
  - command not found windows
  - path not updated
  - cmdlet not found
  - CommandNotFoundException
---

The fix is the boring one, and it is boring for a good reason.

An installer writes the new folder into PATH. Every terminal window that is already open
keeps the copy of PATH it read at startup, because that is when it reads it. Closing the
window and opening a new one is not superstition. It is how the value gets refreshed.

Two installs cause this more than any others. `winget` installs land in a folder that
gets added to PATH during install, so any window open at the time misses it. Global npm
installs land in `C:\Users\<yourname>\AppData\Roaming\npm`, which some setups never add
at all.

One thing the message never means: that the command does not exist anywhere. PowerShell
has no list of real commands. It knows what it found in the folders on PATH, and nothing
else.
