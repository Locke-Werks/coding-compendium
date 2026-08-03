---
id: git-stash-list
title: git stash list
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git stash list
shell: any

does: >
  Shows every set of changes you have stashed away, newest first, with the branch each one
  came from.

flags:
  - flag: "show -p stash@{<number>}"
    means: >
      `git stash show -p stash@{0}` prints the full diff of one stash so you can read what
      is inside before restoring it. Without `-p` you get a per-file summary.
  - flag: "clear"
    means: >
      `git stash clear` deletes every stash at once. There is no confirmation and no listing
      of what went, so run `git stash list` first and be sure.

expect: >
  One line per stash, such as `stash@{0}: WIP on main: 3f2a1b9 fix login redirect`. The
  number in braces is how you name that stash in other commands. Nothing printed means you
  have no stashes.

see_also:
  - git-stash
  - git-stash-pop
  - d10-undo-everything

keywords:
  - what did i stash
  - list stashes
  - find my stashed work
  - stash empty
---

Stashes are easy to forget, because nothing in `git status` mentions them. If you set work
aside last week and cannot find it, this is the first place to look.

They are numbered by recency, so `stash@{0}` changes meaning every time you stash something
new. Name your stashes with `git stash -m "<message>"` and this stops mattering.
