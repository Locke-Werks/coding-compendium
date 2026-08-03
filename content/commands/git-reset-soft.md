---
id: git-reset-soft
title: git reset --soft HEAD~1
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git reset --soft HEAD~1
shell: any

does: >
  Undoes the most recent commit while keeping every change it contained, staged and ready
  to commit again.

flags:
  - flag: "--soft"
    means: >
      Move the branch pointer only. Your files on disk are untouched and the staging area
      keeps everything the commit held. This is the gentlest of the three reset modes and
      the only one that cannot lose an edit.
  - flag: "HEAD~1"
    means: >
      One commit before where you are now. `HEAD` is git's name for your current position,
      and `~1` walks back one step. Use `HEAD~3` to undo the last three commits into a
      single pile of staged changes.
  - flag: "--mixed"
    means: >
      The default when you name no mode. Same as `--soft` except the changes end up
      unstaged rather than staged. Your files are still untouched.

expect: >
  Nothing printed. Run `git status`: the commit is gone from `git log`, and its contents sit
  under `Changes to be committed`, ready for a new `git commit`.

undo: >
  Run `git reflog`, find the entry for the commit you just undid, and run
  `git reset --soft <that-hash>` to put the branch back. Nothing was destroyed, so this is
  reliable.

see_also:
  - git-reset-hard
  - git-commit-amend
  - git-reflog
  - d10-undo-everything

keywords:
  - undo last commit
  - uncommit
  - keep my changes
  - split a commit
---

Use this when the commit was premature or the message was wrong and you would rather redo
it than amend it. Nothing on disk changes, which is why it is safe to run while you think.

Only do this to commits you have not pushed. Once a commit is on GitHub, undoing it locally
puts the two copies out of step.
