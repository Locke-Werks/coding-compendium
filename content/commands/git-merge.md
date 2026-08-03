---
id: git-merge
title: git merge
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git merge <branch-name>
shell: any

does: >
  Brings the commits from another branch into the branch you are currently standing on.

flags:
  - flag: "<branch-name>"
    means: >
      The branch you are merging in, not the destination. You merge into where you are, so
      switch to `main` first and then run `git merge feature/login`.
  - flag: "--no-ff"
    means: >
      Always create a merge commit, even when git could have moved the branch pointer
      forward without one. This keeps a visible record that a branch existed.
  - flag: "--squash"
    means: >
      Collapses every commit from the other branch into a single set of staged changes.
      Nothing is committed, so you run `git commit` afterward and get one tidy commit.
  - flag: "--abort"
    means: >
      Cancels a merge that stopped on a conflict and restores everything to how it was
      before you started. Covered on its own card.

expect: >
  Either `Fast-forward` with a file summary, or
  `Merge made by the 'ort' strategy.` with a file summary. If it stops with
  `Automatic merge failed; fix conflicts and then commit the result.`, you have a conflict
  to resolve and nothing is broken.

see_also:
  - git-merge-abort
  - git-rebase
  - git-switch
  - d6-merge-and-rebase
  - d7-merge-conflicts

keywords:
  - combine branches
  - merge a branch
  - bring changes from another branch
  - merge conflict
---

Check which branch you are on before merging. `git status` prints it on the first line.
Merging in the wrong direction is the most common merge mistake, and it is recoverable
with `git merge --abort` only while the merge is still in progress.
