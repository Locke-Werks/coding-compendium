---
id: c7-files-folders-and-paths
title: Paths on Windows, and why the slashes keep changing
type: section
track: C
order: 70
verified: 2026-08-02
volatility: low
verify: Get-Location
answer: >
  An absolute path starts at a drive letter and works from anywhere, a relative
  path starts from the folder you are standing in, and any path containing a
  space has to be wrapped in quotes or the shell reads it as several separate
  arguments.
owns:
  - absolute vs relative paths
  - backslash vs forward slash
  - the tilde
  - spaces in paths
see_also:
  - b1-terminal-shell-command-line
  - c4-path-and-command-not-found
  - c8-line-endings-and-encoding
  - j3-project-layouts
keywords:
  - what is a path
  - relative path
  - absolute path
  - backslash forward slash
  - path with spaces
  - cannot find the path specified
  - tilde
  - dot dot slash
---

## More

A path is directions to a file. There are two kinds and the difference decides whether the
directions still work tomorrow.

**Absolute.** Starts at a drive letter and names every folder down to the file:
`C:\Users\<yourname>\projects\site\src\app.ts`. It means the same thing typed from anywhere.

**Relative.** Starts from wherever you currently are: `src\app.ts`, or `.\app.ts`, or
`..\other-project\notes.md`. Two pieces of shorthand carry all of it. A single dot `.` means
the current folder. Two dots `..` mean the folder above it. So `..\..\config.json` means up
two levels, then that file.

Almost everything you type is relative, which is why the answer to half of all
`cannot find the path specified` errors is that you are standing somewhere else.

```powershell
Get-Location
```

Prints the folder this terminal is in. `pwd` is a built-in shorthand for the same thing.

```powershell
Set-Location C:\Users\<yourname>\projects\site
```

Moves there. `cd` is the shorthand, and `cd ..` goes up one level.

Two Windows details that cause more confusion than they should.

**The slashes.** Windows separates folders with a backslash `\`. Everything else in software
uses a forward slash `/`. PowerShell, Git Bash, Node, and Python all accept either one on
Windows, so `src/app.ts` and `src\app.ts` both work when you type them. This is why the same
path appears three different ways in one project and none of them are wrong.

**The spaces.** `cd C:\Users\<yourname>\My Projects` fails, because the shell splits on the
space and decides you passed two arguments. Quote it:

```powershell
cd "C:\Users\<yourname>\My Projects"
```

The quotes are not decoration. Any path that might contain a space gets them.

## Full

### Reading a path

```text
C:\Users\nyx\projects\site\src\app.ts
|  \_________________________/ \____/
|             folders           file
drive
```

Windows gives every disk a letter. `C:` is nearly always the one your account lives on. Your
home folder is `C:\Users\<yourname>` and everything of yours hangs off it.

The shorthand for that home folder is `~`. It works in PowerShell and in Git Bash, so
`cd ~\projects` and `cd ~/projects` both land in the same place. It does not work in Command
Prompt, which is one of several reasons to stop using Command Prompt
([b1](#b1-terminal-shell-command-line)).

### Moving around without typing the whole thing

- Press Tab part way through a name and the shell completes it. Press it again to cycle
  through the matches. This is the single biggest reduction in typing errors available to
  you.
- `cd -` returns to the previous folder in PowerShell 7.
- In File Explorer, hold Shift, right-click a file, and choose "Copy as path". You get the
  absolute path already wrapped in quotes, ready to paste.
- Type `cd ` in a terminal and then drag a folder from Explorer onto the window. It pastes
  the path.

### Why code is full of double backslashes

In most programming languages the backslash starts an escape sequence, so `\n` means a new
line and `\t` means a tab. A Windows path pasted into code is therefore full of accidental
instructions: `"C:\Users\nyx"` contains `\U` and `\n`, and the string you get back is not
the one you typed.

The three fixes you will see an agent use, all correct:

```python
path = "C:\\Users\\nyx\\projects"   # doubled: each pair means one backslash
path = r"C:\Users\nyx\projects"     # raw string: the r turns escapes off
path = "C:/Users/nyx/projects"      # forward slashes: Windows accepts them
```

The third is the one to prefer in new code, because it also works unchanged on other
machines.

Git is separate from all of this. It always stores forward slashes internally, whatever your
operating system uses, which is why a `.gitignore` and a diff always show `/` even on
Windows.

### The trailing backslash trap

```powershell
cd "C:\Users\<yourname>\projects\"
```

That closing backslash immediately before the quote escapes the quote in a number of tools,
and the command fails in a way that has nothing to do with the folder. Leave it off:
`"C:\Users\<yourname>\projects"`.

### Uppercase and lowercase

Windows does not care. `App.tsx` and `app.tsx` are the same file to it. Git and Linux do
care, and they are the ones running your CI (Continuous Integration).

The failure this produces is memorable. Rename `App.tsx` to `app.tsx` in your editor,
everything keeps working locally, git records nothing because it sees no change, and the
build on GitHub fails with `Cannot find module './app'`. To rename properly, go through a
temporary name so git is forced to notice:

```powershell
git mv App.tsx temp.tsx; git mv temp.tsx app.tsx
```

### Path length, and where to keep projects

Windows has a 260-character limit on a full path that a lot of tools still obey. It sounds
generous until a JavaScript project nests dependencies six folders deep inside a project
that already lives at `C:\Users\<yourname>\OneDrive\Documents\Work\Clients\2026\...`. The
symptom is `Filename too long` from git or a build that fails on one file
([git filename too long](#git-filename-too-long)).

Keep projects shallow and near the root. `C:\Users\<yourname>\projects\<name>` is fine.
`C:\dev\<name>` is better and looks strange at first.

### Keep projects out of OneDrive

Windows 11 often redirects Desktop and Documents into OneDrive, so a project created in the
obvious place is being synced to the cloud file by file. Three things go wrong: the sync
client holds files open while a build tries to write them, `node_modules` with its tens of
thousands of tiny files makes sync miserable for both of you, and the paths get long enough
to hit the limit above.

Check where you are with `Get-Location`. If the path contains `OneDrive`, move the project
somewhere else. Git already gives you the backup that OneDrive was going to provide.

### Network paths

A path starting with two backslashes, like `\\server\share\project`, is a UNC (Universal
Naming Convention) path pointing at another machine. Plenty of development tools refuse to
work from one, and git specifically will complain about ownership. If you have to use a
network location, map it to a drive letter first and work from that.
