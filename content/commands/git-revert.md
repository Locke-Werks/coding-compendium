---
id: git-revert
title: git revert
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git revert <commit-hash>
shell: any

does: >
  Creates a brand new commit that undoes the changes made by an earlier commit, leaving the
  original commit and all history in place.

flags:
  - flag: "<commit-hash>"
    means: >
      The commit whose changes you want undone. Find it with `git log --oneline`. The first
      seven characters are enough.
  - flag: "--no-edit"
    means: Uses the default message, `Revert "original message"`, without opening an editor.
  - flag: "-n"
    means: >
      Short for `--no-commit`. Stages the undo without committing it, so you can revert
      several commits and finish with a single commit of your own.
  - flag: "-m 1"
    means: >
      Required when reverting a merge commit. It tells git which side of the merge counts as
      the mainline, and `1` means the branch you were on when you merged.

expect: >
  A commit summary such as `[main 8c4d2e1] Revert "fix login redirect"` with a file count.
  `git log` now shows both the original commit and the revert.

undo: >
  Revert the revert: `git revert <the-revert-commit-hash>`. Nothing is destroyed at any
  point, which is what makes this the safe choice.

see_also:
  - git-reset-hard
  - git-show
  - git-log
  - d10-undo-everything

keywords:
  - undo a commit safely
  - back out a change
  - revert without rewriting history
  - undo a pushed commit
---

This is the undo to use when the commit is already on GitHub. It adds history rather than
rewriting it, so nobody else has to do anything and no force push is involved.

If it stops on a conflict, resolve the files, `git add` them, then `git revert --continue`.
`git revert --abort` backs all the way out.
