---
id: git-stash
title: git stash
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git stash
shell: any

does: >
  Sets your uncommitted changes aside in a holding area and returns your working folder to
  a clean state, keeping everything retrievable.

flags:
  - flag: "-u"
    means: >
      Also stash untracked files, meaning files git has never seen before. Plain `git stash`
      leaves them sitting in your folder, which surprises people who expected a clean slate.
  - flag: '-m "<message>"'
    means: >
      Labels the stash so `git stash list` shows something more useful than
      `WIP on main`. Worth it the moment you have more than one stash.
  - flag: "push -- <file>"
    means: >
      `git stash push -- <file>` stashes one named file instead of everything, which is
      handy when only part of your work is in the way.

expect: >
  One line such as `Saved working directory and index state WIP on main: 3f2a1b9 fix login
  redirect`. `git status` afterward reports a clean working tree.

undo: >
  `git stash pop` puts the changes back and removes the stash entry. `git stash apply` puts
  them back and keeps the entry as a spare copy.

see_also:
  - git-stash-pop
  - git-stash-list
  - git-reset-hard
  - d10-undo-everything

keywords:
  - save changes for later
  - clean working tree temporarily
  - park my work
  - cannot switch branch uncommitted changes
---

Reach for this any time a command refuses because you have uncommitted changes, and any
time you are about to run something destructive. It is the cheap insurance that makes a
hard reset survivable.

A stash is local. It never goes to GitHub, and it does not follow you to another machine.
