---
id: git-checkout
title: git checkout
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git checkout <branch-name>
shell: any

does: >
  The older command that both moves you between branches and throws away changes to a
  file, depending on which arguments you hand it.

flags:
  - flag: "<branch-name>"
    means: >
      Moves you onto that branch. Identical to `git switch <branch-name>`, which is the
      modern spelling and does only this one job.
  - flag: "-b <new-branch-name>"
    means: Creates the branch and moves onto it. The old spelling of `git switch -c`.
  - flag: "-- <file>"
    means: >
      A completely different operation despite the shared command name. It overwrites that
      file with the version from the last commit, wiping out your edits. This is the
      dangerous one. `git restore <file>` is the modern spelling.
  - flag: "<commit-hash>"
    means: >
      Moves you to a specific commit and leaves you in detached HEAD state, where any
      commit you make belongs to no branch and is easy to lose track of. Get back with
      `git switch -`.

destructive: true

danger: >
  `git checkout -- <file>` silently overwrites that file with the committed version. There
  is no undo and no confirmation prompt. The command that switches branches and the command
  that destroys your edits differ by two characters, which is why `git switch` and
  `git restore` were split apart in 2019.

destroys: >
  Every uncommitted edit to the named file. Those edits were never in a commit, so no
  reflog and no recovery command can reach them. There is no undo.

safer_first: >
  Run `git diff -- <file>` and read what you are about to throw away. If any of it is worth
  keeping, run `git stash` instead, which sets the changes aside and lets you get them back.

undo: >
  You cannot. Uncommitted work overwritten this way is gone. If the file was open in an
  editor that keeps local history, such as Visual Studio Code, that editor's timeline is
  your only remaining chance.

expect: >
  Switching branches prints `Switched to branch 'main'`. The file form prints nothing at
  all, which is the same output as a typo and gives you no signal that something was
  destroyed.

see_also:
  - git-switch
  - git-restore
  - git-stash
  - d10-undo-everything

keywords:
  - checkout a branch
  - discard changes to a file
  - undo file changes
  - detached head
---
