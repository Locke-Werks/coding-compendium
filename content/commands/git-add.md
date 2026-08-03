---
id: git-add
title: git add
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git add <file>
shell: any

does: >
  Marks a file's current contents to be included in the next commit, moving it into the
  staging area without recording anything permanently yet.

flags:
  - flag: "."
    means: >
      Stages every changed file in the folder you are standing in and every folder beneath
      it. Convenient and blunt. Run `git status` first so you know what `.` is about to
      sweep up.
  - flag: "-A"
    means: >
      Stages everything in the whole repository, including deletions, no matter which
      folder you are standing in. `git add .` run from a subfolder misses changes above
      you.
  - flag: "-p"
    means: >
      Interactive. Shows each change one chunk at a time and asks whether to stage it.
      Answer `y` for yes, `n` for no, `q` to quit. This is how you commit half of what you
      changed.
  - flag: "-u"
    means: >
      Stages changes to files git already tracks and ignores brand new files. Useful when a
      build has scattered new output you do not want committed.

expect: >
  Nothing. Silence is success. Confirm with `git status`, which should now list the file
  under `Changes to be committed`.

see_also:
  - git-status
  - git-commit
  - git-restore-staged
  - d3-the-three-places
  - d12-gitignore-and-what-not-to-commit

keywords:
  - stage a file
  - add to commit
  - git add dot
  - staging area
---

Staging is the step people skip and then wonder why their commit is empty. Git does not
commit what you changed, it commits what you staged. Two separate actions, on purpose, so
you can commit one fix without dragging along four unrelated edits.
