---
id: get-process
title: Get-Process
type: command
verified: 2026-08-02
volatility: low

tool: powershell
command: Get-Process -Name <process-name>
shell: powershell

does: >
  Lists the programs currently running on your machine, with the identification number
  Windows uses to refer to each one.

flags:
  - flag: "-Name <process-name>"
    means: >
      Filters to processes with that name, without the `.exe` on the end. Wildcards work, so
      `-Name node*` catches every Node.js process.
  - flag: "-Id <process-id>"
    means: >
      Looks up one process by its number. Use this to confirm what a number refers to before
      you act on it.
  - flag: "-IncludeUserName"
    means: Adds the account each process runs under. Needs an administrator terminal.

expect: >
  A table with columns including `Id`, `ProcessName`, and memory figures. If nothing matches
  the name you gave, PowerShell reports
  `Cannot find a process with the name "<process-name>"`, which is an answer rather than a
  failure.

see_also:
  - stop-process
  - c5-processes-and-killing-them
  - c6-ports-and-localhost

keywords:
  - what is running
  - find a process
  - process id
  - is the server still running
---

`ps` and `gps` are aliases for this command in PowerShell.

The `Id` column is the process identifier, the number Windows uses internally. It is what
`Stop-Process` wants, and it is unique, unlike the name. Two dev servers both show up as
`node`.
