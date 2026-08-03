---
id: system-cannot-find-the-path-specified
title: "The system cannot find the path specified"
type: error
verified: 2026-08-02
volatility: low

category: not-found

# Prints the folder you are actually standing in. Almost every case of this
# error is answered by that one line.
verify: Get-Location

sample: |
  Set-Location : Cannot find path 'C:\Users\nyx\dev\scaper' because it does not exist.
  At line:1 char:1
  + cd C:\Users\nyx\dev\scaper
  + ~~~~~~~~~~~~~~~~~~~~~~~~~~
      + CategoryInfo          : ObjectNotFound: (C:\Users\nyx\dev\scaper:String) [Set-Location], ItemNotFoundException
      + FullyQualifiedErrorId : PathNotFound,Microsoft.PowerShell.Commands.SetLocationCommand

patterns:
  - "The system cannot find the path specified"
  - "Cannot find path"
  - "because it does not exist"
  - "ItemNotFoundException"
  - "PathNotFound"

means: >
  Windows followed the path you gave, folder by folder, and one of them is not there. The
  path is wrong, or it is right but relative to a folder you are not standing in. A
  relative path such as `.\src\main.py` means "starting from wherever I am now", so the
  same command works in one folder and fails in the next one over.

fix_ladder:
  - try: Print where you actually are.
    command: Get-Location
    shell: powershell
    why: >
      Assumes the path is correct but you are in the wrong folder. A relative path is
      resolved against your current folder, and terminals opened by an editor or an agent
      often start somewhere other than where you think.

  - try: List what is really in the folder above the missing one.
    command: Get-ChildItem C:\Users\<yourname>\dev
    shell: powershell
    why: >
      Assumes a typo or a wrong assumption about the name. Reading the real names side by
      side catches `scaper` for `scraper` and `Dev` for `dev` instantly, which staring at
      your own command does not.

  - try: Let the terminal complete the path for you.
    why: >
      Assumes you cannot spot your own typo, which is normal. Type the first few characters
      of each folder and press Tab. PowerShell fills in the rest from what exists on disk,
      so a name it will not complete is a name that is not there.

  - try: Put quotes around a path containing spaces.
    command: Set-Location "C:\Users\<yourname>\My Projects\app"
    shell: powershell
    why: >
      Assumes the path is correct and the space broke it. Without quotes, PowerShell reads
      `My` as the path and `Projects\app` as a second argument, then reports that `My` does
      not exist.

  - try: Check the slashes if the path came from a config file.
    why: >
      Assumes the path was written for a different system. Windows uses backslashes, and
      most other systems use forward slashes. PowerShell accepts both, but a program
      reading a path out of a config file might not, and a single backslash inside a
      double-quoted string in JSON (JavaScript Object Notation) has to be doubled.

if_none_worked: >
  Paste the whole error, the exact command, and the output of both `Get-Location` and
  `Get-ChildItem` on the parent folder. The directory listing is the part people leave out,
  and it turns a guessing game about spelling into a two-second comparison.

see_also:
  - c7-files-folders-and-paths
  - b1-terminal-shell-command-line
  - j3-project-layouts

keywords:
  - cannot find path
  - path does not exist
  - no such directory
  - wrong folder
  - cd fails
---

Nearly every instance is one of two things: a typo, or a relative path resolved from a
folder you did not expect to be standing in.

The second one catches people repeatedly because terminals do not all start in the same
place. A terminal opened from your editor usually starts at the project root. A terminal
opened from the Start menu starts at `C:\Users\<yourname>`. An agent that ran `cd`
somewhere earlier in the session is still there.

Tab completion is the cheapest defense there is. Type a few characters, press Tab, and
let the shell prove the folder exists by completing it. A name that will not complete does
not exist, and you find that out before you press Enter rather than after.
