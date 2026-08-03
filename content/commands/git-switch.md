---
id: git-switch
title: git switch
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git switch <branch-name>
shell: any

does: >
  Moves you onto an existing branch, replacing the files in your working folder with that
  branch's versions.

flags:
  - flag: "-"
    means: >
      A single dash means the branch you were on last. `git switch -` bounces you back and
      forth the way `cd -` does in a shell.
  - flag: "-c <new-branch-name>"
    means: >
      Creates the branch and moves onto it in one step. Covered on its own card, because it
      is the form you will use most.
  - flag: "--detach <commit-hash>"
    means: >
      Moves you to a specific commit rather than a branch. You end up in detached HEAD
      state, where new commits belong to no branch. Do this only to look around.

expect: >
  `Switched to branch 'main'`, and often a second line such as
  `Your branch is up to date with 'origin/main'.` Your files on disk change to match that
  branch, which is the part that surprises people.

see_also:
  - git-switch-create
  - git-branch
  - git-checkout
  - git-stash
  - d5-branches

keywords:
  - change branch
  - move to another branch
  - checkout a branch
  - switch branches
---

If git refuses with `Your local changes would be overwritten`, you have uncommitted work
that conflicts with the destination branch. Either commit it or run `git stash` to set it
aside, then switch.

`git switch` is the modern command. `git checkout` still does this job and several others,
which is exactly why it is easier to misuse.
