---
id: git-diff
title: git diff
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git diff
shell: any

does: >
  Shows the line-by-line changes you have made to tracked files but have not staged yet.

flags:
  - flag: "-- <file>"
    means: >
      Limits the output to one file. The bare `--` separates paths from options so git does
      not mistake a filename for a flag.
  - flag: "--stat"
    means: >
      Prints a summary instead of the full text: one line per file with a count of how many
      lines changed. Use it first on a large change to see the shape before the detail.
  - flag: "--word-diff"
    means: >
      Highlights the changed words inside a line instead of marking the whole line. Much
      easier to read when someone reworded a sentence.
  - flag: "HEAD"
    means: >
      `git diff HEAD` shows staged and unstaged changes together, which is everything you
      have done since the last commit.

expect: >
  Blocks of text where lines beginning with `-` were removed and lines beginning with `+`
  were added. If you have no unstaged changes it prints nothing at all.

see_also:
  - git-diff-staged
  - git-status
  - git-show
  - d9-reading-a-diff

keywords:
  - what did i change
  - see my changes
  - view diff
---

The output opens in a pager, so the terminal appears to freeze. Press `q` to get out.
Space scrolls a page, the arrow keys scroll a line. This catches everyone once.

`git diff` shows only what is unstaged. Once you run `git add`, the change disappears from
this view and appears in `git diff --staged` instead. Nothing was lost.
