---
id: git-src-refspec-does-not-match-any
title: "error: src refspec main does not match any"
type: error
verified: 2026-08-02
volatility: low

category: not-found

# Prints the current branch name. If it errors or prints nothing, no commit
# exists yet and there is no branch to push.
verify: git branch --show-current

sample: |
  PS C:\Users\you\dev\scraper> git push -u origin main
  error: src refspec main does not match any
  error: failed to push some refs to 'https://github.com/yourname/scraper.git'

patterns:
  - "src refspec .* does not match any"
  - "src refspec"

means: >
  You asked git to push a branch called `main` and there is no such branch on your machine.
  A branch is a name pointing at a commit, so a branch cannot exist until at least one commit
  does. Either you have not committed yet, or your branch has a different name from the one
  you typed, usually `master` rather than `main`.

fix_ladder:
  - try: Find out what your branch is called, if it exists.
    command: git branch --show-current
    shell: powershell
    why: >
      Assumes a name mismatch. If this prints `master`, push that instead or rename it.
      If it prints nothing at all, you have made no commits and no branch exists yet, which
      is the next step.

  - try: Check whether anything has been committed.
    command: git log --oneline -3
    shell: powershell
    why: >
      Assumes the repository is brand new. An error saying the current branch has no commits
      confirms it. This is the cause most of the time: `git init` and `git remote add` were
      run, and `git commit` was not.

  - try: Stage everything and make the first commit.
    command: 'git add .; git commit -m "chore: initial commit"'
    shell: powershell
    why: >
      Assumes there is nothing to push because nothing has been saved. After this, `main`
      exists and points at that commit. Check what got staged first with `git status` if the
      folder contains anything large or private.

  - try: Rename the branch to match what you are pushing.
    command: git branch -m master main
    shell: powershell
    why: >
      Assumes you have commits on `master` and want the modern name. `-m` means move, which
      is git's word for rename here. This is safe and changes nothing except the name, but
      do it before the first push rather than after.

  - try: Push and set the tracking link in one go.
    command: git push -u origin main
    shell: powershell
    why: >
      Assumes the branch now exists. `-u` means set upstream, which records that local `main`
      belongs with `origin/main`. After that, plain `git push` and `git pull` know where to
      go without arguments.

if_none_worked: >
  Paste both error lines, the output of `git branch -a`, and the output of `git log --oneline -3`.
  The branch listing and the log are what people skip, and between them they answer whether
  the problem is an empty repository or a name that does not match.

see_also:
  - d5-branches
  - d4-commit-well
  - d2-repo-remote-clone-origin
  - b3-tell-git-who-you-are

keywords:
  - src refspec does not match any
  - push failed empty repo
  - master vs main
  - no commits yet
  - first push
---

The word "refspec" means the branch name you asked git to push. Git is saying it looked for
that name locally and there is nothing there.

Almost every instance is the very first push of a new project. GitHub's own setup page hands
you a block of commands ending in `git push -u origin main`, and if the commit step was
skipped or failed, this is what you get. A commit that failed because git did not know your
name is a common way to arrive here, and it prints its own error that scrolls past.

The `master` case is the other half. Git used to name the first branch `master` and now names
it `main`, controlled by a setting. An older Git install or a machine where the setting was
never applied still produces `master`, while every tutorial says `main`.

Set the default once so it stops mattering:

```powershell
git config --global init.defaultBranch main
```
