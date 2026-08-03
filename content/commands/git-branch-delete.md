---
id: git-branch-delete
title: git branch -d
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git branch -d <branch-name>
shell: any

does: >
  Deletes a local branch, but refuses if that branch holds commits that have not been
  merged anywhere else.

flags:
  - flag: "-d"
    means: >
      Delete, with a safety check. Git compares the branch against your current branch and
      its upstream, and stops with an error rather than dropping work you would lose. This
      check is the entire reason to prefer `-d` over `-D`.
  - flag: "-r"
    means: >
      Deletes your local copy of a remote-tracking branch, such as
      `git branch -dr origin/old-feature`. This does not touch the branch on GitHub.

expect: >
  `Deleted branch feature/login (was 3f2a1b9).` If it instead says
  `error: the branch 'feature/login' is not fully merged`, git just saved you: those
  commits exist nowhere else.

undo: >
  Recreate it at the same commit. The error message or `git reflog` gives you the hash, then
  `git switch -c <branch-name> <commit-hash>` puts the branch back exactly where it was.

see_also:
  - git-branch
  - git-branch-delete-force
  - git-reflog
  - d5-branches

keywords:
  - delete a branch
  - remove branch
  - branch not fully merged
  - clean up branches
---

This deletes the branch on your machine only. The copy on GitHub is untouched. To remove
that one as well, run `git push origin --delete <branch-name>`.
