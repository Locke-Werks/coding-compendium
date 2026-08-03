---
id: pip-freeze
title: pip freeze
type: command
verified: 2026-08-02
volatility: low

tool: python
command: python -m pip freeze > requirements.txt
shell: powershell

does: >
  Lists every Python package installed in the current environment with its exact version, and
  writes that list to a file other people can install from.

flags:
  - flag: "-m pip"
    means: >
      Runs pip through a specific Python, so the list describes the environment you are actually
      using rather than some other Python on the machine.
  - flag: ">"
    means: >
      Redirects the output into a file instead of the screen. It overwrites the file completely
      each time. Use `>>` to append, which you almost never want here.
  - flag: "requirements.txt"
    means: >
      The conventional filename. Nothing enforces it, but every tool and every set of
      instructions assumes it. This file gets committed.
  - flag: "--local"
    means: Excludes packages inherited from a parent environment, listing only what this environment installed itself.

expect: >
  With the redirect, nothing on screen and a new file. Without it, one line per package in the
  form `requests==2.32.3`. Read the file back with `Get-Content requirements.txt`.

see_also:
  - python-pip-install
  - python-venv
  - python
  - g3-lockfiles

keywords:
  - requirements file
  - save my dependencies
  - export python packages
  - pin versions
---

Run it inside an activated virtual environment. In a system Python it lists every package you
have ever installed for any reason, which produces a requirements file full of things your
project does not use.
