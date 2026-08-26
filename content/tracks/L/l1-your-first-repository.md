---
id: l1-your-first-repository
title: Your first repository, start to finish
type: section
track: L
order: 10
verified: 2026-08-25
volatility: quarterly
verify: gh auth status
answer: >
  Six commands take you from an empty folder to code visible on github.com: make the folder,
  `git init`, `gh repo create`, `git add`, `git commit`, `git push`.
owns:
  - the first repository walkthrough
  - the order of the first commands
see_also:
  - b4-github-and-gh
  - d2-repo-remote-clone-origin
  - d4-commit-well
  - l4-your-first-day-in-order
  - d12-gitignore-and-what-not-to-commit
keywords:
  - first repository
  - create a repo
  - gh repo create
  - push to github for the first time
  - github quick start
  - how do i get my code on github
  - start a new project
  - git init walkthrough
---

## More

This walkthrough assumes git and `gh` are installed and signed in. If `gh auth status` does
not print a green check, stop here and do [b4](#b4-github-and-gh) first.

Every step has a check. Do not move to the next one until the check passes.

**1. Make a folder and go into it.**

```powershell
mkdir ~/dev/first-project ; cd ~/dev/first-project
```

Check: the prompt now ends in `first-project`.

**2. Turn it into a repository.**

```powershell
git init
```

Prints `Initialized empty Git repository`. This creates a hidden `.git` folder that holds the
history. Nothing has been saved yet. Check: `git status` runs without complaining that this
is not a repository.

**3. Put a file in it.**

```powershell
"# First project" | Set-Content README.md
```

Check: `Get-Content README.md` prints that line back.

**4. Create the GitHub side and connect it.**

```powershell
gh repo create first-project --private --source . --remote origin
```

Creates the repository on github.com and points this folder at it. `--private` keeps it
yours, `--source .` means "use this folder", and `--remote origin` names the connection
`origin`, which is the conventional name every other command assumes. Check:
`git remote -v` prints two lines with your GitHub address in them.

**5. Save the change.**

```powershell
git add . ; git commit -m "Add README"
```

`git add` stages the file, `git commit` records it. Check: `git log --oneline` prints one
line with your message.

**6. Send it to GitHub.**

```powershell
git push -u origin main
```

`-u` links your local `main` branch to the one on GitHub, so future pushes are just
`git push`. Check: `gh repo view --web` opens the repository in your browser and your README
is on the page.

## Full

### What each step actually did

Six commands is a small number for something with this many moving parts, and knowing which
part each command touched is what makes the next problem debuggable.

| Step | Where it happened |
|---|---|
| `git init` | Your machine only. Created `.git`. |
| Writing the file | Your machine. Git has not been told about it. |
| `gh repo create` | GitHub, plus one line of local config recording the address |
| `git add` | Your machine. Marks what goes in the next commit. |
| `git commit` | Your machine. Writes a permanent snapshot into `.git`. |
| `git push` | Sends commits to GitHub. The first thing here that leaves your computer. |

Everything up to the push is local and private. You can commit forty times on a plane. That
separation is the point of git, and [d3](#d3-the-three-places) explains the three places a
change can be sitting.

### Starting from a repository that already exists

The other half of the same skill. When the project is already on GitHub, you clone instead of
init:

```powershell
gh repo clone <owner>/<repo>
```

Downloads the whole project including its history and sets `origin` for you. `cd` into the
new folder and you are in the same state as the end of the walkthrough above.
[d2](#d2-repo-remote-clone-origin) covers what a remote is properly.

### The website route, and why the terminal route is better

You can create a repository at https://github.com/new by filling in a form. It works, and it
is what most tutorials show. Two things go wrong with it for a beginner.

The form offers to add a README, a `.gitignore` and a license. Accepting any of them makes
the GitHub copy have a commit your local folder does not, and the first `git push` then fails
with `Updates were rejected because the remote contains work that you do not have locally`.
That error is correct, and it is a rough first experience.
[git-failed-to-push-some-refs](#git-failed-to-push-some-refs) has the fix.

`gh repo create --source .` avoids this by creating an empty repository that matches the
folder you already have. Use the website to look at things. Use `gh` to make them.

### Before you push anything real

Two habits, both cheap now and expensive later.

**Add a `.gitignore` before the first commit.** It is a list of things git should not track:
build output, dependency folders, and anything with a password in it. Committing
`node_modules` once and removing it later leaves it in the history forever.
[d12](#d12-gitignore-and-what-not-to-commit) covers what belongs in it.

**Check what you are about to commit, every time.**

```powershell
git status
```

Lists what is staged, what is changed and not staged, and what git has never seen. Read it
before every commit. It takes two seconds and it is the only thing standing between you and
committing a file you did not mean to. [g6](#g6-secrets-and-what-never-to-commit) covers the
one category where the mistake is not recoverable by deleting the file afterward.

### When step 4 fails

`gh repo create` has two common failures and both are quick.

```text
error: name already exists on this account
```

You already have a repository with that name. Pick another, or add it to the existing one
with `git remote add origin` instead.

```text
error: To get started with GitHub CLI, please run: gh auth login
```

You are not signed in, or the token expired. Run `gh auth login` and answer the prompts.
[b4](#b4-github-and-gh) has the walkthrough.

### What to do next

Make a second change and repeat steps 5 and 6. The three-command cycle of `git status`,
`git commit`, `git push` is most of what you will do with git for the first month, and
running it twenty times is how it stops feeling like ceremony.
[l4](#l4-your-first-day-in-order) is the checklist version.
