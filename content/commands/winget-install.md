---
id: winget-install
title: winget install
type: command
verified: 2026-08-02
volatility: quarterly

tool: winget
command: winget install --id <package-id> -e
shell: powershell

verify: winget --version

does: >
  Downloads and installs a program on Windows from the command line, using Microsoft's
  built-in package manager.

flags:
  - flag: "--id <package-id>"
    means: >
      Match on the exact package identifier rather than a fuzzy name search. Identifiers look
      like `Git.Git`, `GitHub.cli`, or `Microsoft.PowerShell`. Find one with
      `winget search <name>`.
  - flag: "-e"
    means: >
      Short for `--exact`. Turns off partial matching, so `-e` with `--id Git.Git` installs
      that package and never a similarly named one. Without it, an ambiguous search stops and
      asks.
  - flag: "--source winget"
    means: >
      Installs from the community package repository rather than the Microsoft Store. Worth
      naming when a package exists in both and the Store version behaves differently.
  - flag: "--silent"
    means: Suppresses the installer's own windows and prompts. Handy for scripts, unhelpful when something fails.
  - flag: "--accept-package-agreements"
    means: Answers yes to a license prompt that would otherwise wait for you forever in a script.

expect: >
  A progress bar, then `Successfully installed`. Confirm the program itself works by running
  its own version command, such as `git --version`.

see_also:
  - winget-upgrade
  - get-command
  - c4-path-and-command-not-found
  - b4-github-and-gh

keywords:
  - install a program
  - windows package manager
  - winget not recognized
  - install git
---

The command works and the program is still "not recognized". That is almost always the
`PATH` variable: your terminal read it when it opened and does not know about the new entry.
Close the window, open a new one, and try again.

Run it in an ordinary PowerShell window first. Most packages install for your user alone and
never need an administrator prompt.
