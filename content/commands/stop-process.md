---
id: stop-process
title: Stop-Process
type: command
verified: 2026-08-02
volatility: low

tool: powershell
command: Stop-Process -Id <process-id> -Force
shell: powershell

does: >
  Kills a running program immediately, without asking it to save anything or shut down
  cleanly.

flags:
  - flag: "-Id <process-id>"
    means: >
      The process identification number, taken from `Get-Process`. Precise, because the number
      refers to exactly one running program.
  - flag: "-Name <process-name>"
    means: >
      Kills every process with that name at once. `Stop-Process -Name node` ends every Node.js
      process on the machine, including ones belonging to other projects and editors.
  - flag: "-Force"
    means: >
      Stops the process without the confirmation prompt that some processes trigger. It does
      not make the kill any harder, since the program is given no chance to clean up either
      way.

destructive: true

danger: >
  The program is terminated where it stands. It gets no opportunity to save, flush a file it
  was writing, or close a database connection cleanly. A file being written at that moment can
  be left half finished. Killing a system process, or a process you did not identify first,
  can make Windows unstable until you restart.

destroys: >
  Any unsaved state inside that program, and possibly the integrity of whatever file it had
  open. Not recoverable. There is no undo. Files already written to disk and closed are fine.

safer_first: >
  Press Ctrl+C in the window running the program. That asks it to stop and lets it clean up,
  and it handles the common case of a dev server you want to shut down. If you must kill it,
  run `Get-Process -Id <process-id>` first and read the name back so you know what you are
  ending.

undo: >
  You cannot. Start the program again, and expect it to complain about anything it left in a
  half-written state.

expect: >
  Nothing printed. Confirm with `Get-Process -Id <process-id>`, which should now report that
  it cannot find a process with that identifier.

see_also:
  - get-process
  - c5-processes-and-killing-them
  - c6-ports-and-localhost

keywords:
  - kill a process
  - port already in use
  - stop the dev server
  - taskkill
  - process wont stop
---
