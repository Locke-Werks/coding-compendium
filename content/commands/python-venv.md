---
id: python-venv
title: python -m venv
type: command
verified: 2026-08-02
volatility: low

tool: python
command: python -m venv .venv
shell: powershell

verify: python -c "import sys; print(sys.executable)"

does: >
  Creates an isolated Python environment inside your project, so the libraries this project
  needs are installed here and not scattered across your whole machine.

flags:
  - flag: "-m"
    means: >
      Run a module that ships with Python as though it were a program. `python -m venv` runs the
      venv module through the exact Python you just named, which is what makes the environment
      belong to that interpreter and no other.
  - flag: ".venv"
    means: >
      The folder to create. The leading dot is a convention that keeps it out of the way, and
      `.venv` is the name editors and tools look for automatically. Add it to `.gitignore`.
  - flag: "--prompt <name>"
    means: Changes the label shown in your prompt while the environment is active, which helps when several are open.
  - flag: "--upgrade-deps"
    means: Installs the newest pip and setuptools into the new environment instead of the versions bundled with Python.

expect: >
  Nothing printed, and a `.venv` folder appears. Activate it with
  `.venv\Scripts\Activate.ps1` in PowerShell, after which your prompt gains a `(.venv)` prefix.
  That prefix is the confirmation.

see_also:
  - python-pip-install
  - pip-freeze
  - set-executionpolicy
  - python
  - g4-environments-and-isolation

keywords:
  - virtual environment
  - venv
  - activate python environment
  - isolated python
  - activate.ps1
---

If activation fails with `running scripts is disabled on this system`, that is the PowerShell
execution policy, not Python. See `set-executionpolicy`.

The environment applies to the window it was activated in and nothing else. Open a new terminal
and you have to activate it again. If an import fails after you installed the package,
check the prefix in your prompt first.
