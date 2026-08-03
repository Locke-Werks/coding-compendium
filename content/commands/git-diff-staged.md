---
id: git-diff-staged
title: git diff --staged
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git diff --staged
shell: any

does: >
  Shows exactly what is in the staging area, which is precisely what your next commit will
  contain.

flags:
  - flag: "--staged"
    means: >
      Compares the staging area against the last commit rather than comparing your working
      files against the staging area. That is the difference between "what have I typed"
      and "what am I about to commit".
  - flag: "--cached"
    means: >
      The older name for `--staged`. Identical behavior. You will meet it in tutorials
      written before 2015.
  - flag: "--stat"
    means: Summary form, one line per file with counts, instead of the full text.

expect: >
  The same plus and minus line format as `git diff`. Nothing printed means nothing is
  staged, so a commit right now would fail with `no changes added to commit`.

see_also:
  - git-diff
  - git-add
  - git-commit
  - d9-reading-a-diff

keywords:
  - what am i about to commit
  - review staged changes
  - git diff cached
---

Run this immediately before every commit, especially after an agent has been editing. It
is the last cheap moment to catch a file that got swept in by `git add .`.
