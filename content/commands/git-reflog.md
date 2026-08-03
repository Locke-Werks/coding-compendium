---
id: git-reflog
title: git reflog
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git reflog
shell: any

does: >
  Lists every position your current branch has pointed at recently, including commits that
  no longer appear in `git log` because a reset, rebase, or amend moved past them.

flags:
  - flag: "HEAD@{<number>}"
    means: >
      How each entry is named. `HEAD@{0}` is where you are now, `HEAD@{1}` is where you were
      one move ago. You can hand these names to other commands the same way you hand them a
      hash.
  - flag: "show <branch-name>"
    means: >
      `git reflog show <branch-name>` narrows the history to one branch instead of every
      move you have made.
  - flag: "--date=iso"
    means: >
      Prints real timestamps instead of the default relative wording, which matters when you
      are trying to match an entry to something you did an hour ago.

expect: >
  One line per move, such as `3f2a1b9 HEAD@{2}: commit: fix login redirect` or
  `8c4d2e1 HEAD@{1}: reset: moving to HEAD~1`. The left column is the commit hash you need.

see_also:
  - git-reset-hard
  - git-reset-soft
  - git-branch-delete-force
  - d11-when-you-lose-work

keywords:
  - i lost a commit
  - recover deleted branch
  - undo a reset
  - find lost work
---

This is the command nobody tells beginners about, and it recovers almost everything. If a
commit existed on your machine at any point in roughly the last 90 days, it is in here with
its hash, even after a hard reset or a force-deleted branch.

The exception is real and worth knowing: work that was never committed was never in the
reflog either. Uncommitted changes discarded by a reset are gone.
