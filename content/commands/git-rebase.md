---
id: git-rebase
title: git rebase
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git rebase <base-branch>
shell: any

does: >
  Rewrites your branch's commits so they appear to have been made on top of another
  branch's latest commit, producing a straight line of history instead of a fork.

flags:
  - flag: "<base-branch>"
    means: >
      The branch you want to sit on top of, usually `main`. Your commits are replayed one
      at a time onto its tip.
  - flag: "-i"
    means: >
      Interactive. Opens a list of your commits in an editor where you can reorder them,
      reword messages, or squash several into one. Powerful, and the fastest way to make a
      mess if you are unsure.
  - flag: "--continue"
    means: >
      Resumes after you have resolved a conflict. Stage the fixed files first, then run
      this. Rebase stops once per conflicting commit, so you may do it several times.
  - flag: "--abort"
    means: Cancels the whole rebase and restores your branch. Covered on its own card.

destructive: true

danger: >
  Rebasing replaces your commits with new ones that have different hashes. If those commits
  were already pushed, your branch and the remote branch no longer share history, and
  reconciling them takes a force push. Never rebase a branch other people are working on.

destroys: >
  The original commit objects on your branch, including their hashes and their true dates.
  The content survives in the replayed copies, and the originals stay reachable through
  `git reflog` for about 90 days. Uncommitted changes block the rebase from starting, so
  they are not at risk.

safer_first: >
  Commit or stash everything, then make a backup branch at your current position with
  `git switch -c backup-before-rebase` followed by `git switch -`. If the rebase goes
  wrong, the backup branch still points at the original commits.

undo: >
  While it is running, `git rebase --abort`. After it has finished, run `git reflog`, find
  the entry for your branch just before the rebase, and run `git reset --hard <that-hash>`,
  which is itself destructive and has its own card.

expect: >
  `Successfully rebased and updated refs/heads/feature/login.` If it stops with
  `CONFLICT (content): Merge conflict in <file>`, fix the file, `git add` it, then
  `git rebase --continue`.

see_also:
  - git-rebase-abort
  - git-merge
  - git-reflog
  - d6-merge-and-rebase

keywords:
  - rebase onto main
  - straighten history
  - replay commits
  - interactive rebase
---
