---
id: git-show
title: git show
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git show <commit-hash>
shell: any

does: >
  Prints one commit in full: its message, its author, and the complete diff of what that
  commit changed.

flags:
  - flag: "<commit-hash>"
    means: >
      The commit you want. The first seven characters are enough. With no hash at all,
      `git show` displays the most recent commit on your branch.
  - flag: "--stat"
    means: >
      Replaces the diff with a per-file summary of how many lines changed. Use it when the
      commit is large and you want the shape first.
  - flag: "--name-only"
    means: Lists only the filenames the commit touched, with no diff at all.
  - flag: "<commit-hash>:<file>"
    means: >
      Prints that file exactly as it was in that commit, with no diff markers. This is how
      you read an old version of a file without changing anything.

expect: >
  A header block with `commit`, `Author:`, and `Date:`, then the message, then the diff in
  plus and minus lines. Press `q` to leave the pager.

see_also:
  - git-log
  - git-diff
  - git-revert
  - d9-reading-a-diff

keywords:
  - view a commit
  - what was in that commit
  - inspect commit
  - old version of a file
---
