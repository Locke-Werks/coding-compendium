---
id: d2-repo-remote-clone-origin
title: Repo, remote, clone, origin
type: section
track: D
order: 20
verified: 2026-08-02
volatility: low
answer: >
  A repository is a folder git is tracking, a remote is another copy of it
  somewhere else, and origin is the nickname git gives the remote you cloned
  from. Cloning downloads a repository to your machine; forking copies one into
  your GitHub account.
owns:
  - repository
  - remote
  - origin
  - clone
  - fork
see_also:
  - d1-what-git-actually-stores
  - b5-ssh-vs-https
  - b4-github-and-gh
  - d5-branches
keywords:
  - repo
  - what is origin
  - git remote -v
  - fork vs clone
  - upstream
  - download a repo
  - where does my code live
---

## More

Four words answer the question "where does this code live", and you meet them in this
order.

A **repository**, always shortened to **repo**, is one project folder that git is tracking.
It holds your files plus a hidden `.git` folder containing the entire history. Two ways to
get one:

```powershell
git init
```

Turns the folder you are standing in into a repo. Check it worked with `git status`, which
prints `On branch main` instead of `fatal: not a git repository`.

```powershell
git clone https://github.com/nyxlocke/sandbox.git
```

Downloads an existing repo from GitHub, complete with every commit ever made to it, into a
new folder named `sandbox` inside the folder you are standing in. Check it by running
`cd sandbox` and then `git log --oneline -3`, which prints the three newest commits.

A **remote** is another copy of the same repository somewhere your machine can reach,
almost always on GitHub. Your local repo keeps a short list of them, each stored as a
nickname paired with an address. See yours:

```powershell
git remote -v
```

```text
origin  https://github.com/nyxlocke/sandbox.git (fetch)
origin  https://github.com/nyxlocke/sandbox.git (push)
```

Two lines for one remote, because git records where it reads from and where it writes to
separately. If it prints nothing, this repo has no remote, which is normal for a project
you started with `git init` and have not pushed anywhere yet.

**origin** is a nickname, not a rule. Cloning sets it up for you and points it at wherever
you cloned from. Nothing in git treats the word specially, and you can rename it or have
five remotes with other names. Almost every set of instructions you read says `origin`
because almost everyone accepts the default.

A **fork** is a GitHub feature rather than a git command. It makes a copy of somebody
else's repository under your own account, which you then clone to your machine to work on.
You fork when you want to change a project you do not have permission to push to. For your
own projects, you never fork anything.

## Full

### Attaching a remote to a repo you started locally

If you ran `git init` first and made a repository on GitHub afterward, the two do not know
about each other yet. Introduce them:

```powershell
git remote add origin https://github.com/nyxlocke/sandbox.git
```

`add` means create a new nickname, `origin` is the nickname, and the rest is the address.
Confirm with `git remote -v`, which should now print the two lines shown above. Then send
your commits up for the first time:

```powershell
git push -u origin main
```

`-u` sets `origin/main` as the default target for this branch, so every later push on it
is a bare `git push`. You only pass `-u` once per branch.

Two failures you may hit here. `error: remote origin already exists` means a remote by that
name is set up, so run `git remote -v` and look at where it points before changing
anything. `Repository not found` on a repository you know exists usually means an
authentication problem rather than a typo: GitHub hides private repositories from
unauthenticated requests instead of admitting they are there.
[b5](#b5-ssh-vs-https) covers proving who you are.

### The two address formats

A repository address is a URL (Uniform Resource Locator), a web-style address, and GitHub
offers it in two shapes for the same repository:

```text
https://github.com/nyxlocke/sandbox.git
git@github.com:nyxlocke/sandbox.git
```

The first uses HTTPS (Hypertext Transfer Protocol Secure) and authenticates with a token.
The second uses SSH (Secure Shell) and authenticates with a key file on your machine. They
reach the same repository, and you can switch between them later with
`git remote set-url origin <new-address>`. Which to pick, and how to set either one up, is
[b5](#b5-ssh-vs-https).

### Fork or clone, decided in one line

Clone when you can push to the repository: your own projects, and anything you have been
given write access to. Fork first when you cannot, which means somebody else's public
project. A fork gives you a repository you own, and your changes travel back to the
original as a pull request ([d8](#d8-pull-requests)).

When you have forked, the convention is two remotes:

```powershell
git remote add upstream https://github.com/originalowner/project.git
```

`origin` is now your copy and `upstream` is the original, so `git pull upstream main`
brings down what the original project has done since you forked. Again, `upstream` is a
nickname people agreed on, not a keyword.

### What clone actually brings down

The full history, every branch that exists on the remote, every tag, and a working copy of
the default branch checked out and ready to edit. It also creates `origin` pointing back at
the source. What it does not bring down is anything the repository never contained: build
output, dependencies, and `.env` files are excluded on purpose
([d12](#d12-gitignore-and-what-not-to-commit)), which is why a fresh clone usually needs an
install step before it will run.

Clone into the folder where you keep code, not into the repo itself. Cloning a repository
inside another repository produces a nested repo that the outer one half-tracks, and the
symptom is confusing: the inner project shows up in `git status` as a single unexplained
entry rather than as its files.

### Two Windows details that cost people an afternoon

Keep your code somewhere plain, such as `C:\Users\<yourname>\dev\`, which PowerShell also
writes as `~/dev`. Replace `<yourname>` with your actual Windows username.

Avoid putting repositories inside a OneDrive folder. OneDrive syncs the `.git` folder while
git is writing to it, and the result ranges from slow commands to a repository that reports
corruption on a machine where nothing went wrong. Desktop and Documents are OneDrive
folders by default on many Windows 11 installs, so check before you settle in. If your
project is already there, the fix is to move the folder somewhere else and clone or copy
it fresh.

### Renaming and removing a remote

```powershell
git remote rename origin github
```

Renames the nickname and nothing else. Every command that mentioned `origin` now needs
`github`, which is the entire reason people leave the default alone. Verify with
`git remote -v`. To detach a remote completely, `git remote remove <nickname>` deletes the
nickname locally and does nothing whatsoever to the copy on GitHub.
