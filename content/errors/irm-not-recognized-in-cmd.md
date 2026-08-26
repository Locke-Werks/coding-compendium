---
id: irm-not-recognized-in-cmd
title: "'irm' is not recognized as an internal or external command"
type: error
verified: 2026-08-02
volatility: low

language: powershell
category: not-found

# Prints a version table in PowerShell. In Command Prompt it fails, which is
# itself the answer.
verify: $PSVersionTable.PSVersion

sample: |
  C:\Users\you>irm https://claude.ai/install.ps1 | iex
  'irm' is not recognized as an internal or external command,
  operable program or batch file.

patterns:
  - "is not recognized as an internal or external command"
  - "operable program or batch file"

means: >
  You are typing PowerShell commands into Command Prompt. They are two different shells
  that both open in a black window and look nearly identical. `irm` is short for
  Invoke-RestMethod, and `iex` is short for Invoke-Expression. Both exist only in
  PowerShell. Command Prompt has never heard of either, so it reports them the way it
  reports any unknown name.

fix_ladder:
  - try: Look at the prompt you are typing into.
    why: >
      Assumes you are in the wrong shell and have not noticed. Command Prompt shows a path
      followed by a greater-than sign, like `C:\Users\you>`. PowerShell shows the same path
      with `PS` in front, like `PS C:\Users\you>`. That prefix is the whole tell.

  - try: Open PowerShell and run the command there.
    command: powershell
    shell: cmd
    why: >
      Assumes you want to stay in the window you already have. Typing `powershell` inside
      Command Prompt starts PowerShell inside it, and the prompt gains its `PS` prefix
      immediately. Paste your command again after that.

  - try: Open Windows Terminal on its PowerShell tab instead.
    why: >
      Assumes you launched the wrong app. Press the Windows key, type `Windows Terminal`,
      and open it. The default tab is PowerShell. The little arrow next to the plus sign
      at the top opens a menu of the other shells installed, and it names each one.

  - try: Confirm the shell before you paste anything else.
    command: $PSVersionTable.PSVersion
    shell: powershell
    why: >
      Assumes you want certainty rather than a guess from the prompt text. In PowerShell
      this prints a small table of version numbers. In Command Prompt it prints nothing
      useful, because `$PSVersionTable` means nothing there.

if_none_worked: >
  Paste the command you ran, the two lines of error, and the first line of your window
  showing the prompt itself, the `C:\Users\...>` part. That prompt line is what people
  delete before pasting and it is the single piece of evidence that identifies which shell
  you are in.

see_also:
  - b1-terminal-shell-command-line
  - b6-install-claude-code
  - c4-path-and-command-not-found
  - powershell

keywords:
  - irm not recognized
  - iex not recognized
  - internal or external command
  - wrong shell
  - cmd vs powershell
---

Every install instruction written this decade assumes PowerShell. Windows still opens
Command Prompt in plenty of places, including some right-click menus and anything that
inherits an older shortcut.

The distinction matters beyond this one error. Command Prompt and PowerShell disagree
about how to set a variable, how to chain commands, and how to quote a path with a space
in it. A command copied from a blog post and pasted into the wrong one fails in ways that
look like the command is broken.

If the same line then fails in PowerShell with different wording about a cmdlet not being
recognized, that is a different problem and it is about PATH. Read
[c4-path-and-command-not-found](#c4-path-and-command-not-found).
