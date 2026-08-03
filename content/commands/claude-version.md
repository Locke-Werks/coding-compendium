---
id: claude-version
title: claude --version
type: command
verified: 2026-08-02
volatility: weekly

tool: claude
command: claude --version
shell: powershell

verify: claude --version

does: >
  Prints the installed version of Claude Code and exits, which is the fastest way to confirm the
  tool is installed and reachable.

flags:
  - flag: "--version"
    means: Print the version and stop. It starts no session and reads no files.
  - flag: "doctor"
    means: >
      `claude doctor` runs a health check on the installation and reports what it finds. The next
      thing to try when the version prints but sessions fail.
  - flag: "update"
    means: >
      `claude update` pulls the newest version. The native Windows installer usually updates
      itself, so this is mostly a way to force it now rather than later.

expect: >
  A single line with a version number, such as `2.0.14 (Claude Code)`. If instead you get
  `The term 'claude' is not recognized`, it is either not installed or your terminal has not
  picked up the `PATH` change yet. Close the window and open a new one before concluding
  anything.

see_also:
  - claude-cli
  - get-command
  - b6-install-claude-code
  - c4-path-and-command-not-found

keywords:
  - is claude code installed
  - claude version
  - claude not recognized
  - check claude install
---

This is the check step after installing. A version number means the install worked and the
terminal can find it, which are two separate things that fail separately.
