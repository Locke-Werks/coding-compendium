---
id: b2-install-git
title: Install Git for Windows
type: section
track: B
order: 20
verified: 2026-08-02
volatility: quarterly
verify: git --version
answer: >
  Download "Git for Windows" from https://git-scm.com and run the installer,
  accepting the defaults all the way through. Open a new terminal and run
  `git --version`. If it prints a version number, you are done.
owns:
  - installing git
see_also:
  - b1-terminal-shell-command-line
  - d1-what-git-actually-stores
  - b3-tell-git-who-you-are
  - c4-path-and-command-not-found
keywords:
  - install git
  - git for windows
  - git bash install
  - git not recognized
  - setup git windows
  - git scm
---

## More

Git is the version-control program: it records every version of your project so you can go
back to any of them. It runs on your machine and it is a separate thing from GitHub, the
website that stores copies online. You install git first, before anything else, because both
agents assume it is there.

Download "Git for Windows" from https://git-scm.com and run the installer. The defaults are
correct for this setup, so accept them all the way through. It takes about two minutes.

Three things arrive with it:

- **git**, the command itself, available in every shell afterward.
- **Git Bash**, a bash shell for the handful of commands written in bash syntax. It is also
  the shell Claude Code prefers to run its own commands in.
  [b1](#b1-terminal-shell-command-line) tells the shells apart.
- The plumbing that lets other tools call git, which is how the agents commit on your behalf.

Then **open a new terminal window** and confirm:

```powershell
git --version
```

If it prints a line beginning `git version`, you are done. The number after it does not
matter.

If it prints `'git' is not recognized as the name of a cmdlet`, the terminal you are in was
opened before git was installed and has not noticed it. Close it, open a new one, run the
command again. That fixes it almost every time, and
[c4](#c4-path-and-command-not-found) explains why.

Next: [b3](#b3-tell-git-who-you-are), which takes thirty seconds and prevents git from
refusing your first commit.

## Full

### The installer's questions, and why the defaults are right

The installer asks more than most, and every screen has a sensible default already selected.
Three of them are worth recognizing rather than reading.

**Default editor.** Git opens an editor when it wants a longer message from you. The default
choice is fine. If you are ever dropped into a full-screen editor you cannot escape and the
bottom line says something about Vim, press `Esc`, then type `:q!` and press Enter to leave
without saving.

**Adjusting your PATH.** The recommended middle option puts git on your PATH, meaning every
shell can find it, without replacing Windows tools. Take it. Choosing the narrowest option
here is the usual cause of "git works in Git Bash and nowhere else."

**Line ending conversions.** The default checks out Windows-style line endings and commits
Unix-style ones. That is the right answer on Windows and it is why git sometimes warns
`LF will be replaced by CRLF`, which is harmless.
[c8](#c8-line-endings-and-encoding) covers it if you ever want the detail.

Everything else: accept and continue.

### Installing from the command line instead

If you would rather not click through an installer:

```powershell
winget install Git.Git
```

`winget` is the Windows Package Manager, built into Windows 11, which installs software from
the command line. Close and reopen the terminal afterward, then run `git --version` to
confirm. The result is the same program with the default options selected for you.

### Where it went

Git lands in `C:\Program Files\Git`. You will never need to open that folder, and two things
inside it are worth knowing about:

- `C:\Program Files\Git\bin\bash.exe` is Git Bash. Windows Terminal finds it on its own and
  adds it to the dropdown next to the tab strip.
- `C:\Program Files\Git\usr\bin` holds small Unix programs that came along for the ride,
  including `ssh`, `ssh-keygen`, and `curl`. That is why [b5](#b5-ssh-vs-https) can tell you
  to run `ssh-keygen` on Windows without installing anything else.

Your own git settings do not live there. They go in `C:\Users\<yourname>\.gitconfig`, which
is [b3](#b3-tell-git-who-you-are).

### Keeping it current

Git for Windows updates itself only when you ask. From any shell:

```powershell
git update-git-for-windows
```

It checks for a newer release and installs it. Once or twice a year is plenty. Git is not a
tool that changes underneath you, which is why this card is not marked as fast-moving.

### The two failures worth naming

**`'git' is not recognized`, immediately after installing.** The terminal caches where
commands live when it starts. Yours started before git existed. Close every terminal window,
open a fresh one, try again. [c4](#c4-path-and-command-not-found).

**`git --version` works but Claude Code says it cannot find bash.** The installer's PATH
question was answered with the narrowest option. Rerun the installer, choose the recommended
middle option, and let it overwrite the existing install. Nothing is lost by reinstalling.

### What you just installed

Git records complete versions of your project inside a hidden `.git` folder, with no
internet connection and no GitHub account involved. What is actually in that folder is
[d1](#d1-what-git-actually-stores), and it is the card to read before you use git much.
