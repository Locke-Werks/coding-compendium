---
id: set-executionpolicy
title: Set-ExecutionPolicy
type: command
verified: 2026-08-02
volatility: low

tool: powershell
command: Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
shell: powershell

does: >
  Changes how willing PowerShell is to run script files, which is the setting that blocks
  installers and virtual environment activation scripts on a fresh Windows machine.

flags:
  - flag: "-ExecutionPolicy RemoteSigned"
    means: >
      Scripts you wrote locally run. Scripts downloaded from the internet must carry a
      publisher's signature. This is the recommended setting and the one most instructions
      mean.
  - flag: "-ExecutionPolicy Restricted"
    means: >
      No script files run at all. The default on Windows client editions, and the reason
      `.venv\Scripts\Activate.ps1` fails before you change anything.
  - flag: "-ExecutionPolicy Bypass"
    means: >
      Nothing is blocked and nothing warns. Appropriate for one command in a controlled
      situation, never as a permanent setting.
  - flag: "-Scope CurrentUser"
    means: >
      Apply the change to your account only. Without it the command targets the whole machine,
      which needs an administrator terminal and affects every account. Keep it.
  - flag: "-Scope Process"
    means: >
      Apply it to this window only, so the setting vanishes when you close it. The safest way
      to run one blocked script without changing anything permanently.

destructive: true

danger: >
  This lowers a security setting. The execution policy exists to stop a downloaded `.ps1` file
  from running when you double-click it or when something else invokes it on your behalf. Once
  it is relaxed, a malicious script that reaches your machine can run with your permissions.
  `RemoteSigned` at `CurrentUser` scope is the smallest change that unblocks normal work.
  `Bypass` at machine scope is not.

destroys: >
  No files and no work. What it removes is a barrier, and the change is permanent until you
  reverse it deliberately. The risk is what runs afterward.

safer_first: >
  Run `Get-ExecutionPolicy -List` and read the current settings for every scope. If you need to
  run one blocked script and nothing more, use
  `Set-ExecutionPolicy -ExecutionPolicy Bypass -Scope Process` instead, which resets itself when
  you close the window.

undo: >
  `Set-ExecutionPolicy -ExecutionPolicy Restricted -Scope CurrentUser` puts the block back.
  `-ExecutionPolicy Undefined` removes your setting entirely and lets the machine default apply
  again.

expect: >
  Either nothing, or a yes-or-no prompt describing the security risk. Answer `Y`. Confirm with
  `Get-ExecutionPolicy -Scope CurrentUser`, which should echo `RemoteSigned`.

see_also:
  - python-venv
  - get-command
  - b6-install-claude-code
  - b1-terminal-shell-command-line

keywords:
  - running scripts is disabled on this system
  - cannot be loaded because running scripts
  - activate.ps1 blocked
  - execution policy error
---
