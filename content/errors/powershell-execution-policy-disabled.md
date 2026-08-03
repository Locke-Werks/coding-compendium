---
id: powershell-execution-policy-disabled
title: "running scripts is disabled on this system"
type: error
verified: 2026-08-02
volatility: low

language: powershell
category: permission

# Prints the policy for every scope at once. The one that matters is the first
# non-Undefined entry reading down the list.
verify: Get-ExecutionPolicy -List

sample: |
  .\.venv\Scripts\Activate.ps1 : File C:\Users\nyx\dev\scraper\.venv\Scripts\Activate.ps1 cannot be loaded because running scripts is disabled on this system. For more information, see about_Execution_Policies at https:/go.microsoft.com/fwlink/?LinkID=135170.
  At line:1 char:1
  + .\.venv\Scripts\Activate.ps1
  + ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
      + CategoryInfo          : SecurityError: (:) [], PSSecurityException
      + FullyQualifiedErrorId : UnauthorizedAccess

patterns:
  - "running scripts is disabled on this system"
  - "cannot be loaded because running scripts is disabled"
  - "UnauthorizedAccess"
  - "PSSecurityException"

means: >
  Windows blocked a `.ps1` file from running. A `.ps1` file is a PowerShell script, a text
  file full of commands. The block comes from the execution policy, a Windows setting that
  decides which scripts are allowed to run on this machine. The default on Windows 11 is
  `Restricted`, which means none of them. Nothing is broken and nothing is missing. The
  file was found, read, and refused.

fix_ladder:
  - try: Allow local scripts for your own account only.
    command: Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
    shell: powershell
    why: >
      Assumes the default `Restricted` policy is the only thing in the way. `RemoteSigned`
      lets scripts you wrote or that a package manager created run freely, while scripts
      downloaded from the internet still need a signature. `-Scope CurrentUser` changes
      your account only, so it needs no administrator window and touches nothing else on
      the machine. Answer `Y` at the prompt.

  - try: Read back what the policy is now.
    command: Get-ExecutionPolicy -List
    shell: powershell
    why: >
      Assumes the change did not take. A policy set by group policy at the `MachinePolicy`
      or `UserPolicy` level overrides your `CurrentUser` setting silently. If either of
      those rows is anything other than `Undefined`, your change is being ignored and a
      work or school machine is the usual reason.

  - try: Run the one script without changing any setting.
    command: powershell -ExecutionPolicy Bypass -File .\<script>.ps1
    shell: powershell
    why: >
      Assumes you want this one script to run and nothing more. The flag applies to that
      single launch. Useful on a machine where policy is locked and you cannot change it.

  - try: Unblock the file if it came from a download.
    command: Unblock-File -Path .\<script>.ps1
    shell: powershell
    why: >
      Assumes the policy is already `RemoteSigned` and the file carries the mark Windows
      attaches to anything downloaded from the internet. That mark is what `RemoteSigned`
      objects to. Only run this on a file you know the origin of.

if_none_worked: >
  Paste the whole error including the `PSSecurityException` line, plus the full output of
  `Get-ExecutionPolicy -List`. The list is the part people cut, and it is the only thing
  that shows whether your `CurrentUser` setting is being overridden by a machine-level
  policy you cannot change.

see_also:
  - b6-install-claude-code
  - g4-environments-and-isolation
  - b1-terminal-shell-command-line
  - powershell

keywords:
  - execution policy
  - cannot be loaded
  - activate.ps1 blocked
  - scripts disabled
  - RemoteSigned
---

The two places you meet this are activating a Python virtual environment and running an
installer script somebody handed you.

`Activate.ps1` is the script that points your terminal at a project's own copy of Python.
It is generated on your machine by Python itself, so it is not suspicious in any way. The
execution policy does not know that. It refuses every `.ps1` file equally under the
default setting.

`RemoteSigned` is the setting to pick. `Unrestricted` and `Bypass` turn the check off
entirely, which is a real reduction in safety for no extra convenience in normal work.
The tradeoff with `RemoteSigned` is that a script you download really does need
`Unblock-File` before it runs, and that friction is the point.
