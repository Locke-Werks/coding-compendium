---
id: setx
title: setx
type: command
verified: 2026-08-02
volatility: low

tool: windows
command: setx <NAME> "<value>"
shell: any

does: >
  Writes an environment variable permanently into your Windows user profile, so it survives
  closing the terminal and restarting the machine.

flags:
  - flag: "<NAME>"
    means: >
      The variable name, conventionally in capitals. Windows treats these names as
      case-insensitive, so `Path` and `PATH` are the same variable.
  - flag: '"<value>"'
    means: >
      The value to store. Always quote it. An unquoted value stops at the first space and the
      rest is silently discarded.
  - flag: "/M"
    means: >
      Write it machine-wide, for every account on the computer, instead of just yours. Needs a
      terminal opened as administrator. You almost never want this.

destructive: true

danger: >
  `setx` overwrites the variable outright rather than adding to it, and it truncates any value
  longer than 1024 characters without warning. Running `setx PATH "<something>"` is the classic
  way to permanently destroy your `PATH`, after which many installed programs stop being found
  and the damage is not obvious until the next new terminal.

destroys: >
  The previous value of that variable, replaced with no confirmation and no record. For `PATH`
  specifically, anything past 1024 characters is cut off and gone. Recoverable only by hand, by
  retyping what was there, which you will not remember.

safer_first: >
  Print the current value first with `$env:<NAME>` in PowerShell and copy the output somewhere
  safe. Never use `setx` on `PATH`. Edit that one through the graphical editor instead: press
  the Windows key, type `environment variables`, and use the dialog, which has no length limit
  and shows you each entry separately.

undo: >
  Run `setx <NAME> "<the old value>"` with the value you saved beforehand. Without that copy,
  there is nothing to restore from.

expect: >
  `SUCCESS: Specified value was saved.` and nothing else. The variable is not available in the
  window you typed it in: open a new terminal and check with `$env:<NAME>`.

see_also:
  - powershell-env-variable
  - get-command
  - g5-environment-variables
  - c4-path-and-command-not-found

keywords:
  - permanent environment variable
  - set path
  - variable disappears when i close terminal
  - persist env var
---
