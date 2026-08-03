---
id: git-status
title: git status
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git status
shell: any

does: >
  Reports which branch you are on, which files you have changed, and which of those
  changes are staged for the next commit.

flags:
  - flag: "-s"
    means: >
      Short format. One line per file with two letter columns: the left column is the
      staged state, the right column is the unstaged state. `M` is modified, `A` is added,
      `D` is deleted, `??` is a file git has never seen.
  - flag: "-b"
    means: Adds the branch name back to the short format, which otherwise omits it.
  - flag: "--ignored"
    means: >
      Also lists files that `.gitignore` is hiding, which is how you confirm an ignore rule
      is doing what you meant.

expect: >
  `On branch main` on the first line. Then either `nothing to commit, working tree clean`,
  or lists headed `Changes to be committed`, `Changes not staged for commit`, and
  `Untracked files`.

see_also:
  - git-add
  - git-diff
  - git-restore
  - d3-the-three-places

keywords:
  - what changed
  - which branch am i on
  - untracked files
  - working tree clean
---

Run this before and after every other git command. It costs nothing, changes nothing, and
it is the only command that tells you where you actually are. Most git confusion is
someone acting on a guess that `git status` would have corrected in one second.
