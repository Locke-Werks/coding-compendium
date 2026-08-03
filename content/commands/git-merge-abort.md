---
id: git-merge-abort
title: git merge --abort
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git merge --abort
shell: any

does: >
  Cancels a merge that stopped on a conflict and puts every file back exactly as it was
  before you ran `git merge`.

flags:
  - flag: "--abort"
    means: >
      Throws away the half-finished merge and restores the branch to its pre-merge state.
      It only works while a merge is actually in progress, meaning after a conflict and
      before you commit the resolution.

expect: >
  Nothing printed. Run `git status` afterward: it should say `On branch main` with no
  mention of unmerged paths, and the conflict markers should be gone from your files.

undo: >
  Run `git merge <branch-name>` again. Aborting costs you only the conflict resolutions you
  had typed so far, and the merge itself is always available to retry.

see_also:
  - git-merge
  - git-rebase-abort
  - git-status
  - d7-merge-conflicts

keywords:
  - cancel a merge
  - get out of a merge
  - undo merge conflict
  - unmerged paths
---

This is the escape hatch, and knowing it exists is what makes conflicts survivable. If you
are staring at `<<<<<<<` markers and have no idea what to do, aborting costs nothing and
puts you back where you started.

It fails with `fatal: There is no merge to abort` once the merge has been committed. At
that point use `git revert` instead.
