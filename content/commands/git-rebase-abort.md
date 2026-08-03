---
id: git-rebase-abort
title: git rebase --abort
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git rebase --abort
shell: any

does: >
  Cancels a rebase that is part way through and restores your branch to exactly the
  commits it had before the rebase started.

flags:
  - flag: "--abort"
    means: >
      Stops the replay, discards the rewritten commits produced so far, and moves your
      branch back to its original position. It works only while the rebase is in progress,
      which is any time your prompt or `git status` mentions a rebase.

expect: >
  Nothing printed on success. Run `git status`: it should name your branch normally with no
  line reading `interactive rebase in progress`, and `git log --oneline` should show your
  original commits with their original hashes.

undo: >
  Run `git rebase <base-branch>` again. Aborting costs only the conflict resolutions you
  had typed, and the rebase itself can always be retried.

see_also:
  - git-rebase
  - git-merge-abort
  - git-status
  - d6-merge-and-rebase

keywords:
  - cancel a rebase
  - stuck in a rebase
  - get out of rebase
  - rebase in progress
---

If you find yourself in a rebase you did not intend to start, this is the exit. It is
always safe while the rebase is unfinished.

The one case it cannot help with is a rebase that already completed. For that, `git reflog`
finds your original commits.
