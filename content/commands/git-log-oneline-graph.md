---
id: git-log-oneline-graph
title: git log --oneline --graph
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git log --oneline --graph --all --decorate
shell: any

does: >
  Draws your commit history as a compact text diagram, one line per commit, with branch
  lines showing where work split apart and came back together.

flags:
  - flag: "--oneline"
    means: >
      Collapses each commit to a single line: short hash, then the first line of the
      message. Nothing else.
  - flag: "--graph"
    means: >
      Draws the branch structure down the left side with vertical bars and slashes. This is
      the only built-in way to see the shape of your history.
  - flag: "--all"
    means: >
      Includes every branch, not only the one you are standing on. Without it you cannot
      see where another branch diverged.
  - flag: "--decorate"
    means: >
      Labels each commit with any branch or tag pointing at it, such as
      `(HEAD -> main, origin/main)`. Modern git does this by default, and naming it costs
      nothing on an older install.

expect: >
  A column of lines like `* 3f2a1b9 (HEAD -> main, origin/main) fix login redirect`, with
  branching drawn to the left. Press `q` to leave the pager.

see_also:
  - git-log
  - git-branch
  - git-merge
  - d1-what-git-actually-stores

keywords:
  - visualize branches
  - history graph
  - pretty git log
  - see branch structure
---

Worth memorizing, because it answers three questions at once: which commit you are on,
whether your branch is behind the remote copy, and whether a branch you thought was merged
actually is. When `origin/main` sits several lines below `main`, you have local commits
that have not been pushed.
