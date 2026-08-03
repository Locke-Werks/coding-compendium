---
id: git-commit-amend
title: git commit --amend
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git commit --amend -m "<new-message>"
shell: any

does: >
  Replaces the most recent commit with a new one, letting you fix its message or fold in
  whatever you have staged since.

flags:
  - flag: "--amend"
    means: >
      Rewrites the last commit instead of adding a new one on top. The result gets a new
      commit hash, so it is a different commit that happens to look like the old one.
  - flag: "--no-edit"
    means: >
      Keeps the existing message untouched. Use this when you only wanted to add a
      forgotten file: stage it, then run `git commit --amend --no-edit`.
  - flag: '-m "<new-message>"'
    means: Supplies the replacement message directly instead of opening an editor.

destructive: true

danger: >
  This rewrites the last commit rather than adding to it. If that commit was already
  pushed to GitHub, your local history and the remote history no longer agree, and the
  only way to reconcile them is a force push, which can overwrite someone else's work.

destroys: >
  The original commit object, including its message and its hash. The content stays
  recoverable from `git reflog` for about 90 days. Whatever you had staged is folded in,
  replacing the previous version of those files inside the commit.

safer_first: >
  Only amend a commit you have not pushed. Run `git log --oneline --graph --all` and see
  whether `origin/main` is already sitting on that commit. If it is, make a new commit or
  use `git revert` instead.

undo: >
  Run `git reflog`, find the line showing the commit as it was before the amend, and run
  `git reset --soft <that-hash>`. This works only while the reflog still holds it.

expect: >
  A summary line that looks like an ordinary commit, such as
  `[main 8c4d2e1] fix login redirect`, carrying a different hash than before.

see_also:
  - git-commit
  - git-reflog
  - git-push-force-with-lease
  - d10-undo-everything

keywords:
  - fix commit message
  - forgot a file in my commit
  - change last commit
  - reword commit
---
