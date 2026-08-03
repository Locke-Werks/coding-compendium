---
id: git-reset-hard
title: git reset --hard
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git reset --hard
shell: any

does: >
  Throws away every uncommitted change in your working folder and staging area, returning
  every tracked file to its state at the last commit.

flags:
  - flag: "--hard"
    means: >
      Reset all three places at once: the branch pointer, the staging area, and the actual
      files on disk. The other modes leave your files alone. This one rewrites them.
  - flag: "HEAD~1"
    means: >
      Adding a target moves the branch as well. `git reset --hard HEAD~1` deletes the last
      commit and every uncommitted change together. The commit is recoverable through the
      reflog; the uncommitted changes are not.
  - flag: "origin/main"
    means: >
      `git reset --hard origin/main` makes your local branch identical to the remote one,
      discarding anything local that differs. Use it to abandon a local mess completely.

destructive: true

danger: >
  Every uncommitted edit in the repository is deleted, in every tracked file, not only the
  one you were thinking about. There is no confirmation prompt and there is no undo for
  uncommitted work. This is the single most common way people lose hours in git.

destroys: >
  All uncommitted changes to tracked files, staged or not. Those never entered a commit, so
  `git reflog` cannot reach them and nothing can bring them back. There is no undo. If you
  also moved the branch, the commits you moved past stay recoverable from the reflog for
  about 90 days. Untracked files are the one thing left alone.

safer_first: >
  Run `git stash` instead, or first. It clears your working folder exactly the same way and
  keeps everything in a place you can retrieve with `git stash pop`. If you meant to move
  the branch rather than discard edits, `git reset --soft` does that without touching a
  file.

undo: >
  For the uncommitted changes, you cannot. For a commit you reset past, run `git reflog`,
  find the hash from before the reset, and `git reset --hard <that-hash>`.

expect: >
  A single line naming where you now are, such as `HEAD is now at 3f2a1b9 fix login
  redirect`. Your files change on disk immediately and `git status` reports a clean tree.

see_also:
  - git-stash
  - git-reset-soft
  - git-reflog
  - d10-undo-everything
  - d11-when-you-lose-work

keywords:
  - throw away all my changes
  - start over
  - reset to last commit
  - discard everything
---
