---
id: pip-externally-managed-environment
title: "error: externally-managed-environment"
type: error
verified: 2026-08-02
volatility: quarterly

language: python
category: config

# Prints the path of the Python that will run your code. Inside an activated
# virtual environment it points at your project's own .venv folder.
verify: python -c "import sys; print(sys.executable)"

danger: >
  The `--break-system-packages` flag is named accurately. It lets pip overwrite libraries the
  operating system itself depends on, and on a Linux machine that can leave system tools
  broken in ways that are tedious to undo. Use a virtual environment instead. Reach for the
  flag only on a machine you can afford to rebuild.

sample: |
  $ pip install requests
  error: externally-managed-environment

  This environment is externally managed
  To install Python packages system-wide, try apt install
  python3-xyz, where xyz is the package you want to install.

  If you wish to install a non-Debian-packaged Python package,
  create a virtual environment using python3 -m venv path/to/venv.
  Then use path/to/venv/bin/python and path/to/venv/bin/pip.

  note: If you believe this is a mistake, please contact your Python installation or OS distributor.
  hint: See PEP 668 for the detailed specification.

patterns:
  - "externally-managed-environment"
  - "This environment is externally managed"
  - "break-system-packages"

means: >
  This Python was installed by the operating system, and the system relies on its own tools
  written in that Python. If pip upgraded a shared library, those tools could break. So the
  Python installation carries a marker saying "do not install into me directly", and pip
  refuses. Nothing was installed and nothing changed.

fix_ladder:
  - try: Make a virtual environment for the project and use that.
    command: python -m venv .venv
    shell: powershell
    why: >
      Assumes you are working on a project, which is the normal case. A virtual environment is
      a private copy of Python inside your project folder, with its own set of installed
      libraries and no connection to the system one. This is the answer the error message
      itself recommends.

  - try: Activate it, then install as usual.
    command: .\.venv\Scripts\Activate.ps1
    shell: powershell
    why: >
      Assumes the environment now exists. Activating points this terminal at the project's
      Python, and your prompt gains a `(.venv)` prefix so you can see it worked. Inside
      Windows Subsystem for Linux the command is `source .venv/bin/activate` instead. Every
      `pip install` after that lands in the project.

  - try: Confirm which Python you are now using.
    command: python -c "import sys; print(sys.executable)"
    shell: powershell
    why: >
      Assumes activation may not have taken. The path printed should be inside your project's
      `.venv` folder. If it still points at a system folder, the environment is not active and
      pip will hit the same refusal.

  - try: Use pipx for tools you want available everywhere.
    command: pipx install ruff
    shell: powershell
    why: >
      Assumes you are installing a command-line program rather than a library for a project.
      pipx makes a private environment per tool and puts the command on PATH, which is exactly
      what you wanted from a system-wide install without the risk this error exists to
      prevent.

  - try: Override the refusal, knowing what it protects.
    command: python -m pip install --break-system-packages requests
    shell: powershell
    why: >
      Assumes you are in a throwaway container or a virtual machine where breaking the system
      Python costs nothing. On a machine you care about, this is the wrong tool. The flag is
      named the way it is on purpose.

if_none_worked: >
  Paste the whole error including the `note:` and `hint:` lines, the output of
  `python -c "import sys; print(sys.executable)"`, and say whether you are on Windows directly
  or inside Windows Subsystem for Linux. The executable path is what people leave out, and it
  identifies which Python is refusing you.

see_also:
  - g4-environments-and-isolation
  - g2-package-managers
  - g1-what-a-dependency-is
  - python

keywords:
  - externally-managed-environment
  - PEP 668
  - break-system-packages
  - venv required
  - pip refuses to install
---

You will meet this inside Windows Subsystem for Linux, in a Docker container, or on a rented
Linux server rather than on Windows itself. Python installed from python.org on Windows does
not carry the marker, so a plain Windows machine never sees it.

The rule underneath is worth adopting everywhere regardless. One virtual environment per
project, created in the project folder, activated before you install anything. Rust and Node
give you this by default: dependencies land in the project rather than on the machine. Python
makes it a step you have to take.

The payoff is that two projects can use different versions of the same library without
fighting, and deleting a project folder removes everything it installed.

The `.venv` folder is disposable and belongs in `.gitignore`. What gets committed is
`requirements.txt` or `pyproject.toml`, the list of what to install, so anyone cloning the
project can rebuild the environment.
