---
id: uv
title: uv sync
type: command
verified: 2026-08-02
volatility: quarterly

tool: uv
command: uv sync
shell: powershell

verify: uv --version

does: >
  Manages Python environments and packages the way cargo manages Rust ones, replacing the
  separate steps of creating a virtual environment, installing pip packages, and pinning
  versions.

flags:
  - flag: "uv sync"
    means: >
      Reads `pyproject.toml` and `uv.lock`, then makes the project's `.venv` folder match them
      exactly, creating it if needed. The one command that gets a checked-out project running.
  - flag: "uv add <package-name>"
    means: Adds a dependency, records it in `pyproject.toml`, and updates the lockfile. The equivalent of `cargo add`.
  - flag: "uv run <command>"
    means: >
      Runs a command inside the project's environment without you activating it first, as in
      `uv run python main.py`. This sidesteps the entire activation problem.
  - flag: "uv venv"
    means: Creates a virtual environment only, as a faster drop-in for `python -m venv`.
  - flag: "uv pip install <package-name>"
    means: >
      A compatibility mode that accepts pip's own arguments, for a project that is not yet
      organized around `pyproject.toml`.

expect: >
  A resolution line, then a list of installed packages with versions, then a timing line. `uv`
  is noticeably fast: an install that takes pip a minute usually finishes in seconds.

see_also:
  - python-venv
  - python-pip-install
  - python
  - g2-package-managers
  - g3-lockfiles

keywords:
  - fast python package manager
  - uv sync
  - replace pip
  - pyproject.toml
  - python lockfile
---

`uv` is newer than the tools it replaces and its command set is still growing, so check the
official documentation if a flag here does not behave as described.

Two things make it worth the switch: it is fast enough that installs stop interrupting you, and
`uv run` removes the "did I activate the environment" question that causes most Python import
failures.
