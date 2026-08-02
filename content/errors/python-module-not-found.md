---
id: python-module-not-found
title: "ModuleNotFoundError: No module named 'X'"
type: error
verified: 2026-08-02
volatility: low

language: python
category: not-found

# Run this after any fix in the ladder below. Silence means it worked: Python
# only prints something when an import fails.
verify: python -c "import requests; print(requests.__version__)"

sample: |
  Traceback (most recent call last):
    File "C:\Users\nyx\dev\scraper\main.py", line 1, in <module>
      import requests
  ModuleNotFoundError: No module named 'requests'

# Anchored on the stable wording only. Never match the module name, the file
# path, or the line number: those change on every machine and every run, and a
# pattern that includes them matches nothing she actually pastes.
patterns:
  - "ModuleNotFoundError: No module named"
  - "ImportError: No module named"

means: >
  Python looked for a library called `requests`, did not find it, and stopped before
  running any of your code. Nothing is broken. Something is missing, or you are running a
  different Python than the one the library is installed in. The second case is far more
  common than people expect and it is the reason this error survives three rounds of
  reinstalling.

fix_ladder:
  - try: Install it into the Python you are actually running.
    command: python -m pip install requests
    shell: powershell
    why: >
      Assumes it was never installed. Note `python -m pip` rather than bare `pip`:
      this guarantees the install goes to the same Python that just failed, which is the
      whole problem in the next step down.

  - try: Check whether you are in the virtual environment you think you are in.
    command: python -c "import sys; print(sys.executable)"
    shell: powershell
    why: >
      Assumes the library is installed, just not where this Python can see it. If the path
      printed is not inside your project's `.venv\` folder, you are running the system
      Python and your project's libraries are invisible to it. Activate the environment
      with `.venv\Scripts\Activate.ps1` and try again.

  - try: Check the install name against the import name.
    command: python -m pip show <name>
    shell: powershell
    why: >
      Assumes it is installed under a different name than you import. These often differ.
      You install `beautifulsoup4` but import `bs4`. You install `pillow` but import `PIL`.
      You install `opencv-python` but import `cv2`. If `pip show` finds it, the package is
      there and only the import line is wrong.

  - try: Check for a typo, including capitalization.
    why: >
      Assumes the name is wrong. Import names are case sensitive even though Windows
      filenames are not, so `import Requests` fails while `import requests` works, and the
      error text looks identical.

  - try: Confirm the package exists at all.
    why: >
      Assumes the agent invented it. Search the name on pypi.org. Agents hallucinate
      plausible package names, and an install command for a package that never existed
      fails in a way that looks like a network problem. See g7-dependency-risk, because the
      version of this where the name was later registered by someone else is a real attack.

if_none_worked: >
  Paste the whole traceback, the exact command you ran, and the output of
  `python -c "import sys; print(sys.executable)"`. That last one is the piece nobody thinks
  to include and it settles the environment question immediately, which is what most of
  these turn out to be.

see_also:
  - g4-environments-and-isolation
  - g2-package-managers
  - f1-how-to-read-an-error-message
  - python

keywords:
  - no module named
  - import error
  - pip install not working
  - cannot import
  - module missing
---

The most common Python error there is, and the one most likely to be misdiagnosed.

The instinct is to run `pip install` again, see it say `Requirement already satisfied`,
and conclude that something is deeply wrong. Nothing is deeply wrong. `pip` and `python`
are two separate programs, and on a machine with more than one Python installed they can
easily point at different ones. `pip` installs the library somewhere real. `python` then
looks somewhere else and honestly reports that it is not there.

This is why every command in the ladder above uses `python -m pip` instead of `pip`.
Running pip *through* a specific Python makes it impossible for them to disagree.
