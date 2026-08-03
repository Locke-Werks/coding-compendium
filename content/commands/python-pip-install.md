---
id: python-pip-install
title: python -m pip install
type: command
verified: 2026-08-02
volatility: low

tool: python
command: python -m pip install <package-name>
shell: powershell

verify: python -c "import sys; print(sys.executable)"

does: >
  Downloads and installs a Python library into the same Python that will run your code.

flags:
  - flag: "-m pip"
    means: >
      Runs pip through a named Python rather than as a standalone program. This is the whole
      point of the form. A machine with two Pythons installed has two pips, and bare `pip` can
      easily install into the one you are not using, which is the cause of most
      `ModuleNotFoundError` confusion.
  - flag: "-r requirements.txt"
    means: Installs every package listed in that file, which is how you restore a project's dependencies in one command.
  - flag: "-U"
    means: Short for `--upgrade`. Installs the newest version of a package you already have.
  - flag: "-e ."
    means: >
      Editable install of the project in the current folder. Your source stays where it is and
      edits take effect without reinstalling. Used while developing a library, not for
      dependencies.
  - flag: "--no-cache-dir"
    means: Ignores the local download cache. Worth trying when an install fails in a way that suggests a corrupted download.

expect: >
  Download and build lines, then `Successfully installed <package-name>-1.2.3`.
  `Requirement already satisfied` means it was there already, which is a success and often a
  sign you are in a different environment than you think.

see_also:
  - python-venv
  - pip-freeze
  - python
  - g4-environments-and-isolation
  - python-module-not-found

keywords:
  - install a python package
  - pip install
  - modulenotfounderror
  - no module named
  - requirements.txt
---

Activate your project's environment before installing, or the library lands in your system
Python and the project cannot see it. Check with the `verify` command above: the path it prints
should be inside your project's `.venv` folder.
