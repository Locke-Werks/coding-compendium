---
id: codex-version
title: codex --version
type: command
verified: 2026-08-02
volatility: weekly

tool: codex
command: codex --version
shell: powershell

verify: codex --version

does: >
  Prints the installed version of Codex and exits, which confirms the tool is installed and your
  terminal can find it.

flags:
  - flag: "--version"
    means: Print the version and stop. It starts no session and reads no files.
  - flag: "--help"
    means: >
      Lists every subcommand and flag the installed version supports. More reliable than any
      written guide for a tool that changes weekly, including this card.
  - flag: "login status"
    means: >
      `codex login status` reports whether you are signed in. A version number proves the program
      exists and says nothing about whether it can reach an account.

expect: >
  A single line with a version number. If you get
  `The term 'codex' is not recognized`, it is either not installed or your terminal has not
  picked up the `PATH` change yet. Close the window, open a new one, and try again before
  concluding it failed.

see_also:
  - codex-cli
  - get-command
  - b7-install-codex
  - c4-path-and-command-not-found

keywords:
  - is codex installed
  - codex version
  - codex not recognized
  - check codex install
---

This is the check step after installing. Two separate things can be wrong: the install itself, or
your terminal not knowing where the program landed. A version number rules out both at once.
