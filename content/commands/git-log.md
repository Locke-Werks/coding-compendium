---
id: git-log
title: git log
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git log
shell: any

does: >
  Lists the commits on your current branch, newest first, with the author, date, and full
  message for each.

flags:
  - flag: "-n <count>"
    means: >
      Shows only that many commits. `git log -n 5` gives the last five, which is usually
      all you want.
  - flag: "-p"
    means: >
      Also prints the full diff for each commit, so you see what actually changed rather
      than what the message claims changed.
  - flag: '--author="<name>"'
    means: Filters to commits by one person. The match is partial, so a surname is enough.
  - flag: '--since="<date>"'
    means: >
      Filters by time. Accepts plain English such as `--since="2 weeks ago"` as well as
      `2026-07-01`.
  - flag: "-- <file>"
    means: Shows only the commits that touched that file, which answers "when did this break".

expect: >
  Entries beginning `commit 3f2a1b9c...` followed by `Author:`, `Date:`, and the indented
  message. In a brand new repository with no commits it says
  `fatal: your current branch 'main' does not have any commits yet`.

see_also:
  - git-log-oneline-graph
  - git-show
  - git-blame
  - d13-tags-releases-and-history

keywords:
  - commit history
  - who changed this
  - when did this change
  - see past commits
---

The output opens in a pager and the terminal looks stuck. Press `q` to exit.

The 40-character string after `commit` is the hash, the permanent name of that snapshot.
You only ever need the first seven characters when passing it to another command.
