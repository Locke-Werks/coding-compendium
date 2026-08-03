---
id: git-restore
title: git restore
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git restore <file>
shell: any

does: >
  Overwrites a file in your working folder with the version from the last commit, throwing
  away everything you changed in it since.

flags:
  - flag: "<file>"
    means: >
      The file to reset. A dot restores every changed file in the current folder and below,
      which is a much larger action than it looks.
  - flag: "--staged"
    means: >
      Changes what the command does entirely. It unstages instead of overwriting, and your
      edits survive. Covered on its own card, because it is safe and this form is not.
  - flag: "--source <commit-hash>"
    means: Restores the file from a specific commit rather than from the most recent one.

destructive: true

danger: >
  This throws away your uncommitted edits to that file with no prompt and no output. Those
  edits were never committed, so no git command can bring them back. This is the modern
  spelling of `git checkout -- <file>` and it is exactly as sharp.

destroys: >
  Every change you made to the named file since the last commit. Not recoverable by
  `git reflog`, which only tracks commits. There is no undo.

safer_first: >
  Run `git diff -- <file>` and read what you are about to lose. If any of it might matter,
  run `git stash` instead: it clears the file the same way and keeps a copy you can get back
  with `git stash pop`.

undo: >
  You cannot. If the file was open in an editor with local history, such as the Timeline
  view in Visual Studio Code, that is your only remaining copy.

expect: >
  Nothing printed. The file on disk reverts and `git status` stops listing it as modified.
  Silence here means it worked, which is also what a mistake looks like.

see_also:
  - git-restore-staged
  - git-checkout
  - git-stash
  - d10-undo-everything

keywords:
  - discard my changes
  - revert a file
  - undo edits to a file
  - throw away changes
---
