---
id: set-location
title: Set-Location
type: command
verified: 2026-08-02
volatility: low

tool: powershell
command: Set-Location <path>
shell: powershell

does: >
  Moves your terminal into a different folder, so every command you run afterward acts on
  that folder.

flags:
  - flag: "<path>"
    means: >
      Where to go. An absolute path starts with a drive letter, as in
      `C:\Users\<yourname>\dev`. A relative path continues from where you are, as in `src`.
  - flag: ".."
    means: Up one folder. `..\..` goes up two.
  - flag: "~"
    means: >
      Your home folder, `C:\Users\<yourname>`. PowerShell understands this shorthand the same
      way a Unix shell does.
  - flag: "-"
    means: >
      A single dash returns you to the previous location, which PowerShell 6 and later
      remember for you.

expect: >
  Nothing printed. The prompt itself changes to show the new folder, which is your
  confirmation. If the folder does not exist you get
  `Cannot find path ... because it does not exist.`

see_also:
  - get-childitem
  - test-path
  - c7-files-folders-and-paths

keywords:
  - change directory
  - cd in powershell
  - move to a folder
  - cannot find path
---

`cd` and `sl` are built-in aliases, so `cd C:\Users\<yourname>\dev` works exactly the same.

Quote any path containing a space: `Set-Location "C:\Program Files\Git"`. Without quotes,
PowerShell reads the space as the end of the path and reports that it cannot find
`C:\Program`.
