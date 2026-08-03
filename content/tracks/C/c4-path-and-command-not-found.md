---
id: c4-path-and-command-not-found
title: PATH, and why a command you just installed is not found
type: section
track: C
order: 40
verified: 2026-08-02
volatility: quarterly
verify: Get-Command git
answer: >
  Your terminal took its own copy of PATH at the moment it opened, so anything
  installed since then is invisible to it, which is why closing every terminal
  window and opening a fresh one fixes this far more often than anything else.
owns:
  - PATH
  - reopening the terminal
  - where executables live
see_also:
  - b1-terminal-shell-command-line
  - f1-how-to-read-an-error-message
  - g5-environment-variables
  - g4-environments-and-isolation
  - c7-files-folders-and-paths
keywords:
  - is not recognized
  - command not found
  - not recognized as the name of a cmdlet
  - i just installed it
  - where is the exe
  - add to path
  - close and reopen terminal
  - internal or external command
---

## More

PATH is a list of folders, in order, separated by semicolons on Windows. When you type a
bare name like `git` and press Enter, the shell does not search your disk. It walks that
list from left to right, looks for `git.exe` in each folder, and runs the first one it
finds. If it reaches the end of the list, you get `is not recognized`.

So `is not recognized` means one of three things, and they are not equally likely:

1. The program is installed, and this terminal window is holding a stale copy of PATH.
2. The program is installed, and its folder was never added to PATH at all.
3. The program is not installed, or you typed a name it does not answer to.

Number one is most of them, and here is why it happens. A process gets a copy of the
environment from whatever launched it, at the instant it launches, and that copy never
updates itself. The installer wrote the new folder into the PATH that Windows keeps stored
for your account. New processes read the stored one. Your terminal, opened before the
install finished, is still working from the copy it took on the way in. Nothing you type
into that window will refresh it.

The fix is to get a process that started after the install:

**Close every terminal window, then open a new one.** A new tab is often not enough, because
the tab inherits from the Windows Terminal process that spawned it, and that process is
exactly as stale as the tab you were in. Close the whole window. If your terminal lives
inside an editor, restart the whole editor rather than the terminal panel.

Then confirm:

```powershell
Get-Command git
```

If it prints a `CommandType`, a `Name`, and a path in the `Source` column, the command is
found and you are done. If it prints `The term 'git' is not recognized`, keep reading in
Full: the install itself did not put it on PATH, and that is a different problem with a
different fix.

## Full

### The exact error text, in each shell

Same failure, three presentations. Knowing which one you are looking at also tells you which
shell you are in, which is [b1](#b1-terminal-shell-command-line).

```text
PowerShell:
gh : The term 'gh' is not recognized as the name of a cmdlet, function, script file, or
operable program. Check the spelling of the name, or if a path was included, verify that
the path is correct and try again.

Command Prompt:
'gh' is not recognized as an internal or external command, operable program or batch file.

Git Bash:
bash: gh: command not found
```

All three mean "I walked PATH and did not find it." None of them mean the program is broken.

### Look at the list yourself

```powershell
$env:PATH -split ';'
```

`$env:PATH` is the value this terminal is working from, and `-split ';'` breaks it into one
folder per line so it is readable. This is the exact list the shell searches, in the exact
order. If the folder you expect is missing from this output, that is your answer.

```powershell
where.exe git
```

`where.exe` prints every match on PATH, not one. The `.exe` on the end is mandatory in
PowerShell, because `where` on its own is a PowerShell alias for something unrelated. Two
lines of output means two copies are installed and the top one wins.

### The refresh without restarting

Reopening the terminal is the answer. This is the other one, for when you have a long-lived
window you do not want to lose:

```powershell
$env:PATH = [System.Environment]::GetEnvironmentVariable("PATH","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("PATH","User")
```

It reads the two stored PATH values, machine-wide and per-account, and rebuilds this
session's copy from them. It affects this window only, and any window you open later gets
the correct value anyway. It fixes PATH and nothing else, so an installer that also set other
variables still needs a restart. [g5](#g5-environment-variables) covers the session-versus-stored
distinction in general.

### When reopening did not fix it

In order, stopping as soon as one works.

1. **Check the name.** `gh`, not `github`. `python`, not `python3`, on Windows. `pnpm` needs
   to be enabled before it exists. The tool's own install page has the exact word.
2. **Find the file by hand.** Look in the folders below. If the `.exe` is there, the install
   worked and only PATH is wrong.
3. **Test it with the full path.** Type the whole thing, in quotes:
   `& "C:\Program Files\GitHub CLI\gh.exe" --version`. The `&` tells PowerShell to run the
   quoted string rather than print it. If that works, this is definitely a PATH problem.
4. **Add the folder to PATH.** Press Start, type `environment`, and open "Edit environment
   variables for your account". Select `Path`, click Edit, click New, paste the folder, and
   accept. Then close every terminal and open a new one.
5. **Sign out and back in.** Rarely needed, and it is the reliable way to make a
   machine-wide change reach everything.

Do not use `setx` to edit PATH. It writes back a flattened value and has a length limit that
silently truncates a long PATH, which turns one missing command into every missing command.
The dialog in step four does not have that problem.

### Where Windows actually keeps these programs

| Tool | Folder |
|---|---|
| Git | `C:\Program Files\Git\cmd` |
| Node and npm | `C:\Program Files\nodejs` |
| Anything installed by `npm install -g` | `C:\Users\<yourname>\AppData\Roaming\npm` |
| Python itself | `C:\Users\<yourname>\AppData\Local\Programs\Python\Python312` |
| Anything installed by `pip install` | that same folder plus `\Scripts` |
| Rust and cargo | `C:\Users\<yourname>\.cargo\bin` |
| Most winget installs | `C:\Users\<yourname>\AppData\Local\Microsoft\WindowsApps` |

The Python row is the one that bites. The installer offers a checkbox called "Add Python to
PATH" and adds the interpreter folder. Command-line tools that arrive through `pip` land in
`Scripts` next door, so `python` works and the thing you installed with it does not. Adding
the `Scripts` folder by hand fixes that category permanently.

### Two versions, and the first one wins

```powershell
Get-Command python -All
```

`-All` lists every match instead of stopping at the first. If you see two, the top one is
what runs. This is how a machine ends up running Python 3.9 while you are certain you
installed 3.12, and how a project starts failing for no visible reason after you install
something unrelated that shipped its own copy of Node.

The fix is to reorder the entries in the dialog above, not to uninstall things in a panic.

### The Microsoft Store trap

Type `python` on a clean Windows 11 machine and the Microsoft Store opens. That is not a
PATH failure. Windows ships a zero-byte stub called an App Execution Alias whose only job is
to advertise the Store version, and it sits in `WindowsApps`, near the front of PATH.

Turn it off: Start, then Settings, then Apps, then Advanced app settings, then App execution
aliases. Switch off `python.exe` and `python3.exe`. Then close every terminal and open a new
one.

### The case where PATH is correct and the command is still missing

If the project uses an isolated environment, the tool exists inside that environment and
nowhere else. An unactivated Python virtual environment produces exactly the same `is not
recognized` message for a package you definitely installed. That is
[g4](#g4-environments-and-isolation), and the tell is that the fix is to activate the
environment rather than to touch PATH at all.
