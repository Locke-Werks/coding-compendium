---
id: git-cherry-pick
title: git cherry-pick
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git cherry-pick <commit-hash>
shell: any

does: >
  Copies the changes from one commit on another branch onto the branch you are currently
  standing on, as a new commit.

flags:
  - flag: "<commit-hash>"
    means: >
      The commit to copy. Find it with `git log --oneline --all`. The result is a new commit
      with a new hash holding the same changes, so the original stays where it is.
  - flag: "<hash-a>..<hash-b>"
    means: >
      Picks a range of commits. The first hash is excluded and the second is included, which
      trips up everyone at least once.
  - flag: "-n"
    means: >
      Short for `--no-commit`. Applies the changes and stages them without committing, so
      you can adjust them or fold several picks into one commit.
  - flag: "-x"
    means: >
      Appends a line to the message recording which commit this was picked from. Useful when
      the same fix has to land on two branches.

expect: >
  A commit summary such as `[main 8c4d2e1] fix login redirect` with a file count. If it
  stops with `CONFLICT`, resolve the files, `git add` them, then `git cherry-pick
  --continue`.

undo: >
  While it is running, `git cherry-pick --abort`. Once it has committed, `git revert
  <the-new-commit-hash>` removes the change without rewriting history.

see_also:
  - git-log-oneline-graph
  - git-revert
  - git-merge
  - d6-merge-and-rebase

keywords:
  - copy a commit
  - apply one commit to another branch
  - take just this fix
  - backport a change
---

Use this when one commit from a branch is worth having and the rest is not, most often a
hotfix that needs to be on both `main` and a release branch.

If you find yourself picking more than three commits in a row, you probably wanted `git
merge` or `git rebase` instead.
