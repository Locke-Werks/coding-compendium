---
id: powershell-env-variable
title: "$env: environment variables"
type: command
verified: 2026-08-02
volatility: low

tool: powershell
command: $env:<NAME>
shell: powershell

does: >
  Reads or sets an environment variable, which is a named value programs look up for
  configuration, for the current PowerShell window only.

flags:
  - flag: "$env:<NAME>"
    means: >
      Reads the value. Type it alone on a line and PowerShell prints it. Nothing is printed if
      the variable is not set, which looks identical to a variable set to an empty string.
  - flag: '$env:<NAME> = "<value>"'
    means: >
      Sets the value for this window and every program you launch from it. It disappears when
      you close the window. Quote the value if it contains a space.
  - flag: "$env:PATH"
    means: >
      The list of folders Windows searches for programs, separated by semicolons. This is the
      variable behind almost every "command not found" on a freshly installed tool.
  - flag: "Get-ChildItem Env:"
    means: Lists every environment variable currently set, as a table of name and value.
  - flag: '$env:<NAME> = $null'
    means: Clears the variable for this window.

expect: >
  Reading prints the value on one line. Setting prints nothing. Confirm a set by typing
  `$env:<NAME>` on its own and reading it back.

see_also:
  - setx
  - get-command
  - g5-environment-variables
  - c4-path-and-command-not-found

keywords:
  - set an environment variable
  - api key in terminal
  - env var powershell
  - temporary variable
---

The `$env:` prefix is PowerShell-specific. Command Prompt writes `%NAME%` and a Unix shell
writes `$NAME`, so a command copied from a Linux tutorial will not work unchanged.

Use this form for a value you need for one session, such as an API key you are testing with.
It never gets written to disk, so it cannot be committed by accident. For something permanent,
use `setx`.
