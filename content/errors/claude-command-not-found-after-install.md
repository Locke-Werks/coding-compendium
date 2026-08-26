---
id: claude-command-not-found-after-install
title: "'claude' is not recognized, right after installing it"
type: error
verified: 2026-08-02
volatility: weekly

category: not-found

# Prints a version number when the command is on PATH. Run it in a terminal
# opened after the install finished.
verify: claude --version

sample: |
  PS C:\Users\you> irm https://claude.ai/install.ps1 | iex
  Downloading Claude Code...
  Installing to C:\Users\you\.local\bin
  Installation complete. Run 'claude' to get started.

  PS C:\Users\you> claude
  claude : The term 'claude' is not recognized as the name of a cmdlet, function, script file, or operable program. Check the spelling of the name, or if a path was included, verify that the path is correct and try again.
  At line:1 char:1
  + claude
  + ~~~~~~
      + CategoryInfo          : ObjectNotFound: (claude:String) [], CommandNotFoundException

patterns:
  - "'claude' is not recognized"
  - "claude: command not found"
  - "'codex' is not recognized"
  - "codex: command not found"

means: >
  The install worked. The installer added a folder to PATH, the list of places Windows searches
  for a command you type, and this terminal window read PATH when it opened, which was before the
  install happened. The window is looking at a stale copy. Nothing failed and nothing needs
  reinstalling.

fix_ladder:
  - try: Close this terminal window, open a new one, and type the command again.
    why: >
      Assumes the install succeeded and only this window's copy of PATH is old. This is the cause
      almost every time, and it is the single most common snag in installing either agent. A new
      window reads PATH fresh.

  - try: Run it by its full path to prove it is there.
    command: C:\Users\<yourname>\.local\bin\claude.exe --version
    shell: powershell
    why: >
      Assumes you want certainty before doing anything else. A version number means the program is
      installed correctly and the only problem is PATH. An error about the path not existing means
      the install put it somewhere else, or did not finish.

  - try: Look at where the installer actually put it.
    command: Get-ChildItem C:\Users\<yourname>\.local\bin
    shell: powershell
    why: >
      Assumes the location differs from what the instructions said, which happens as installers
      change between versions. The installer prints the folder it used in its own output, so
      check that line first if this folder is empty.

  - try: Add the folder to PATH yourself, permanently.
    command: '[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\Users\<yourname>\.local\bin", "User")'
    shell: powershell
    why: >
      Assumes the installer did not manage to write to PATH, which some locked-down machines
      prevent. This writes it into your user settings rather than the machine's, so it needs no
      administrator window. Open a new terminal afterward, because this window still has the old
      copy.

  - try: Check whether you are in Git Bash rather than PowerShell.
    command: echo $PATH | tr ':' '\n'
    shell: bash
    why: >
      Assumes a shell difference. Git Bash keeps its own translated copy of PATH and occasionally
      drops entries it cannot convert. If the command works in PowerShell and not in Git Bash,
      the install is fine and this is a translation problem.

if_none_worked: >
  Paste the whole error, plus the installer's complete output including the line naming where it
  installed to. That installer output is the piece everyone throws away once it scrolls past, and
  it names the exact folder that should be on PATH.

see_also:
  - b6-install-claude-code
  - b7-install-codex
  - c4-path-and-command-not-found
  - g5-environment-variables

keywords:
  - claude command not found
  - codex not recognized
  - path not updated after install
  - claude code install
  - reopen terminal
---

Both agents install this way and both hit this. The fix is boring and complete: open a new
terminal.

The mechanism is worth holding on to, because it explains a whole family of confusion. PATH is
read once, when a program starts. Every terminal window, and every editor with a built-in
terminal, carries the copy it read at launch. An installer that updates PATH changes what future
windows see and cannot reach into the ones already open.

That includes your editor. Visual Studio Code inherits PATH from whatever launched it, so its
built-in terminal can stay stale even after you open a fresh standalone PowerShell window.
Closing the editor completely and reopening it is the reliable move there.

If the command still fails in a brand new window, check the installer's own output before
reinstalling. It prints the folder it used, and the answer is nearly always that the folder is
not the one you assumed.
