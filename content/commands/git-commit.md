---
id: git-commit
title: git commit -m
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git commit -m "<message>"
shell: any

does: >
  Records everything currently staged as a permanent snapshot in your repository's
  history, labeled with the message you give it.

flags:
  - flag: '-m "<message>"'
    means: >
      Supplies the commit message on the command line. Without `-m`, git opens a text
      editor and waits, which is where beginners get stranded inside Vim. Keep the message
      short and in the imperative, such as `fix login redirect`.
  - flag: "-a"
    means: >
      Stages every tracked file that changed, then commits, in one step. It does not pick
      up brand new files that git has never seen, which is the trap: `git commit -am` can
      silently leave your new file out.
  - flag: "--no-verify"
    means: >
      Skips any pre-commit checks the project has configured. It exists for emergencies.
      Skipping the checks is how a broken commit reaches the shared branch.

expect: >
  One summary line such as `[main 3f2a1b9] fix login redirect`, then a count line such as
  `1 file changed, 4 insertions(+), 2 deletions(-)`. The short string after the branch
  name is the commit hash, which is how you refer to this commit later.

see_also:
  - git-add
  - git-commit-amend
  - git-log
  - git-status
  - d4-commit-well

keywords:
  - save my work
  - make a commit
  - commit message
  - nothing to commit
---

If it says `nothing added to commit but untracked files present`, you have changes but
never staged them. Run `git add <file>` first.

Commit more often than feels necessary. Every commit is a point you can return to, and the
cost of an extra one is nothing.
