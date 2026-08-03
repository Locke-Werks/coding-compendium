---
id: g4-environments-and-isolation
title: Isolated environments
type: section
track: G
order: 40
verified: 2026-08-02
volatility: quarterly
verify: Get-Command python
danger: >
  `Remove-Item .venv -Recurse -Force` deletes the whole virtual environment with
  no undo. That is safe: everything in it came from your manifest and can be
  reinstalled. It is not safe if you have been keeping files of your own inside
  that folder, so look before you run it.
answer: >
  An isolated environment gives one project its own copy of its dependencies, so
  two projects can use different versions of the same library without a fight.
  JavaScript and Rust do this by default; Python needs you to create one.
owns:
  - virtual environment
  - per-project dependencies
  - global installs
see_also:
  - g2-package-managers
  - g1-what-a-dependency-is
  - c4-path-and-command-not-found
  - g5-environment-variables
  - pip-externally-managed-environment
  - g3-lockfiles
keywords:
  - venv
  - virtualenv
  - works on my machine
  - activate
  - global install
  - conda
  - why do i need a virtual environment
  - it says the module is not installed
---

## More

Two projects on your machine need the same library at different versions. If dependencies
live in one place per machine, that is a conflict with no fix: one project gets the version
it wants and the other breaks. If dependencies live inside each project, there is nothing
to argue about.

JavaScript does the second one automatically. Every project gets its own `node_modules`,
and nothing is shared unless you asked for it. Rust, Go, and C# each resolve versions per
project too.

Python does the first one, and that is the entire reason virtual environments exist.
`pip install requests` with no environment active installs into whatever Python it finds on
your PATH, machine-wide, where every other project sees it. A **virtual environment** is a
folder inside your project holding its own Python and its own packages, so the install
lands there instead.

Two commands, run once per project:

```powershell
python -m venv .venv
```

Creates the folder. `-m venv` means "run the venv module," and `.venv` is the folder name
everyone uses.

```powershell
.\.venv\Scripts\Activate.ps1
```

Switches this terminal over to it. Your prompt gains a `(.venv)` prefix, which is how you
know it worked. If you see `running scripts is disabled on this system`, that is Windows
blocking the script rather than anything being wrong with your project, and
[the execution policy card](#powershell-execution-policy-disabled) fixes it in one line.

Then install as normal, and everything lands inside the project. Add `.venv` to your
`.gitignore` ([d12](#d12-gitignore-and-what-not-to-commit)); it is regenerable and it is
machine-specific.

The part that catches everyone: activation lasts for that one terminal window. Open a new
one and you are back to the machine-wide Python, so the first symptom is usually an import
that worked five minutes ago and does not now.

## Full

### What activation actually does

It puts your project's `.venv\Scripts` folder at the front of PATH, for that shell only.
That is all. `python` and `pip` now resolve to the copies inside your project instead of
the ones in `C:\Users\<yourname>\AppData\...`, because PATH is searched in order and yours
is now first. [c4](#c4-path-and-command-not-found) is the card on how that search works.

Knowing it is a PATH trick explains every behavior around it. A new terminal has a fresh
PATH, so it is not activated. Your editor's built-in terminal has its own PATH, so it may
be activated when your other window is not. And you can skip activation entirely by naming
the interpreter directly:

```powershell
.\.venv\Scripts\python.exe -m pip install requests
```

Longer to type and impossible to get wrong, which makes it the right form to put in a
script or hand to an agent.

To confirm which Python is in charge right now:

```powershell
Get-Command python
```

It prints the full path. If that path is inside your project folder, you are in the
environment. If it says something under `AppData` or `Program Files`, you are not, whatever
your prompt looks like.

### Who isolates by default

| Ecosystem | Isolation | What you do |
|---|---|---|
| JavaScript | per project, automatic | nothing |
| Rust | per project, automatic | nothing |
| Go | per module, automatic | nothing |
| C# | per project, automatic | nothing |
| Python | machine-wide by default | create and activate a virtual environment |
| Ruby | machine-wide by default | use `bundler` |

This table is why Python-flavored instructions online are full of steps that seem to have
no equivalent elsewhere. They are solving a problem the other ecosystems solved in their
design.

### The newer Python tools

`uv` and `poetry` create and manage the environment for you, so there is nothing to
activate and nothing to forget:

```powershell
uv run python main.py
```

That creates the environment if it is missing, installs what the manifest lists, and runs
your file inside it. If a project has a `uv.lock` or a `poetry.lock`, use that project's
tool rather than plain `pip`, because mixing them produces two sets of packages and a very
confusing afternoon.

### The error that means you skipped this

`error: externally-managed-environment` is Python refusing to let you install into the
system copy. It reads like a permissions problem and it is a guardrail: the fix is to
create an environment, not to force past it. Full detail is in
[that error's card](#pip-externally-managed-environment).

The other tell is `ModuleNotFoundError` for a package you know you installed. You installed
it into one environment and you are running in another. Check `Get-Command python` before
you debug anything else.

### Deleting and recreating

An environment that has gone strange is not worth investigating:

```powershell
Remove-Item .venv -Recurse -Force
python -m venv .venv
.\.venv\Scripts\Activate.ps1
pip install -r requirements.txt
```

Nothing of yours lives in there, so nothing of yours is lost. This is the same disposable
logic as `node_modules` in [g1](#g1-what-a-dependency-is).

### What agents get wrong here

An agent working in your project will often run `pip install something` without checking
whether an environment is active, because the command works either way and only the
destination differs. You get a package installed machine-wide, a project that still fails,
and a fix that reads as though it worked. When you see a bare `pip install` in a session,
check the environment before you trust the result.

Putting the rule in your instruction file handles it permanently:
"always activate `.venv` before running pip" ([e4](#e4-claude-md-and-agents-md)).

### The four things "works on my machine" usually means

1. Different package versions, because there is no committed lockfile
   ([g3](#g3-lockfiles)).
2. A tool installed globally on your machine and nowhere else
   ([g2](#g2-package-managers)).
3. An environment variable that exists in your shell only
   ([g5](#g5-environment-variables)).
4. A different version of the language itself. Node 18 against Node 22 breaks real things,
   and version managers such as `fnm` for Node or `rustup` for Rust exist to pin it per
   project.

Containers, which you will meet as a `Dockerfile`, are the heavyweight answer to all four
at once: they ship the operating system, the language, and the dependencies as one image.
You do not need one yet. Recognizing the file is enough
([the Dockerfile card](#dockerfile)).
