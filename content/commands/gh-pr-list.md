---
id: gh-pr-list
title: gh pr list
type: command
verified: 2026-08-02
volatility: quarterly

tool: gh
command: gh pr list
shell: powershell

does: >
  Lists the open pull requests on the current repository, with each one's number, title, and
  branch.

flags:
  - flag: "--state <state>"
    means: >
      Filters by status. Accepts `open`, `closed`, `merged`, or `all`. The default is `open`,
      which is why a pull request you merged yesterday appears to have vanished.
  - flag: "--limit <count>"
    means: Returns more than the default 30 entries.
  - flag: "--author <username>"
    means: Filters to one person. Use `--author @me` for your own.
  - flag: "--json <fields>"
    means: >
      Prints machine-readable output instead of a table, as in `--json number,title,state`.
      Useful when handing the result to another tool.

expect: >
  A table with a number column, the title, the branch name, and how long ago it was updated.
  If there are none it prints
  `no open pull requests in yourname/myproject`.

see_also:
  - gh-pr-create
  - gh-pr-merge
  - d8-pull-requests

keywords:
  - list pull requests
  - open prs
  - what is waiting for review
  - find pr number
---

The number in the first column is what every other command wants. `gh pr view <pr-number>`
reads one, `gh pr checks <pr-number>` shows whether continuous integration passed, and
`gh pr merge <pr-number>` merges it.

`gh pr status` is the companion command: it shows the pull request for the branch you are
currently on, which is usually the one you care about.
