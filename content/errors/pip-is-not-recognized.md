---
id: pip-is-not-recognized
title: "'pip' is not recognized as the name of a cmdlet"
type: error
verified: 2026-08-02
volatility: low

language: python
category: not-found

# Prints the pip version and, in brackets, which Python it belongs to. That
# second part is the useful half.
verify: python -m pip --version

sample: |
  PS C:\Users\nyx\dev\scraper> pip install requests
  pip : The term 'pip' is not recognized as the name of a cmdlet, function, script file, or operable program. Check the spelling of the name, or if a path was included, verify that the path is correct and try again.
  At line:1 char:1
  + pip install requests
  + ~~~
      + CategoryInfo          : ObjectNotFound: (pip:String) [], CommandNotFoundException
      + FullyQualifiedErrorId : CommandNotFoundException

patterns:
  - "'pip' is not recognized"
  - "'pip3' is not recognized"
  - "'python' is not recognized"
  - "No module named pip"

means: >
  Windows cannot find a program called `pip` in any folder on PATH. PATH is the list of folders
  Windows searches for a command you type. Python installs pip as a separate small program in
  a `Scripts` folder, and that folder is added to PATH only if the installer was told to add
  Python to PATH. Python itself may well be working fine while pip is invisible.

fix_ladder:
  - try: Run pip through Python instead of on its own.
    command: python -m pip install requests
    shell: powershell
    why: >
      Assumes Python works and only pip's own launcher is missing from PATH. `-m` means "run
      this module", and pip ships inside Python itself. This works whenever `python` works, and
      it guarantees the install lands in the same Python that will run your code, which bare
      `pip` does not.

  - try: Check that Python itself is findable.
    command: python --version
    shell: powershell
    why: >
      Assumes Python is missing too. A version number means Python is fine and only pip's
      folder is off PATH. The same not-recognized message means neither is available, which is
      a different fix.

  - try: Close the terminal and open a new one.
    why: >
      Assumes Python was installed while this window was open. A terminal reads PATH at
      startup, so a window older than the install has never seen the new folders.

  - try: Turn off the Microsoft Store shortcut if Python opens the Store.
    why: >
      Assumes the Windows app execution alias is intercepting the command. Windows ships stub
      files for `python.exe` that open the Microsoft Store instead of running anything. Press
      the Windows key, type "Manage app execution aliases", and turn off both Python entries.

  - try: Use the Python launcher, which is always on PATH.
    command: py -m pip install requests
    shell: powershell
    why: >
      Assumes `python` is not on PATH but the launcher is. The python.org installer puts `py`
      into a Windows system folder that is always searched, so it works even when nothing else
      does. Add `-3.12` after `py` to pick a specific version when several are installed.

  - try: Reinstall Python with the PATH box ticked.
    why: >
      Assumes the install was done without it. Download the installer from python.org, run it,
      and tick "Add python.exe to PATH" on the first screen. That single checkbox is the cause
      of most of this, and it is off by default.

if_none_worked: >
  Paste the whole error, the output of `python --version`, and the output of
  `Get-Command python, py, pip -ErrorAction SilentlyContinue`. That last command shows which of
  the three Windows can actually find, and it is the piece nobody thinks to include even
  though it names the exact gap.

see_also:
  - c4-path-and-command-not-found
  - g4-environments-and-isolation
  - g2-package-managers
  - python

keywords:
  - pip is not recognized
  - pip not found windows
  - python not recognized
  - add python to path
  - py launcher
---

Use `python -m pip` rather than bare `pip`, permanently, and most of this stops being a
problem you ever think about.

The reason goes beyond PATH. `pip` and `python` are two separate programs, and on a machine
with more than one Python installed they can easily point at different ones. That produces the
situation where pip reports a successful install and Python then says the module does not
exist. Running pip through a specific Python makes it impossible for them to disagree.

The Microsoft Store stub deserves its own warning. Windows ships a placeholder `python.exe`
that exists only to open the Store page. Typing `python` on a machine with no Python installed
opens a shopping page rather than reporting anything useful, and on a machine where Python was
installed later the stub sometimes still wins.

A virtual environment changes the answer again. Inside an activated one, `pip` is on PATH and
points at that environment's Python, which is exactly what you want.
