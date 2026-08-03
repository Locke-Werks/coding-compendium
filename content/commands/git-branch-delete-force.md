---
id: git-branch-delete-force
title: git branch -D
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git branch -D <branch-name>
shell: any

does: >
  Deletes a local branch without checking whether its commits exist anywhere else.

flags:
  - flag: "-D"
    means: >
      Force delete. It is `-d` with the safety check switched off. Capital `D` and
      lowercase `d` are entirely different commands, and the capital is the one that does
      not ask.

destructive: true

danger: >
  Every commit that lived only on this branch stops being reachable from any branch name.
  Git will eventually garbage-collect them. Use `git branch -d` first and only reach for
  `-D` when git has refused and you have read why it refused.

destroys: >
  The branch pointer, and with it the only easy way to find any commit that was on this
  branch and nowhere else. Those commits stay recoverable through `git reflog` for about
  90 days, after which they are gone for good. Uncommitted changes are never on a branch
  at all, so this command does not touch them.

safer_first: >
  Try `git branch -d <branch-name>`. If it refuses, run `git log <branch-name> --oneline`
  and read what you are about to strand. Note the top hash before you delete anything.

undo: >
  Run `git reflog`, find the branch's last commit hash, then
  `git switch -c <branch-name> <commit-hash>`. Only works while the reflog still holds the
  entry.

expect: >
  `Deleted branch feature/login (was 3f2a1b9).` Write that hash down. It is your receipt
  and the only thing that makes the recovery above possible.

see_also:
  - git-branch-delete
  - git-reflog
  - d11-when-you-lose-work
  - d10-undo-everything

keywords:
  - force delete branch
  - branch not fully merged
  - delete unmerged branch
  - git branch capital D
---
