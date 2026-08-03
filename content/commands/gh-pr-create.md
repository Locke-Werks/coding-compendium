---
id: gh-pr-create
title: gh pr create
type: command
verified: 2026-08-02
volatility: quarterly

tool: gh
command: gh pr create --title "<title>" --body "<description>"
shell: powershell

does: >
  Opens a pull request on GitHub proposing that your current branch be merged into the main
  branch, without leaving the terminal.

flags:
  - flag: '--title "<title>"'
    means: >
      The headline of the proposal. Leave it off and the command asks interactively, which
      also works.
  - flag: '--body "<description>"'
    means: >
      The description. Use `--body-file <file>` for anything longer than a sentence, since a
      long quoted string in PowerShell is awkward to type correctly.
  - flag: "--base <branch-name>"
    means: >
      The branch you want to merge into, defaulting to the repository's default branch.
      Set it when you are targeting a release branch rather than `main`.
  - flag: "--draft"
    means: >
      Opens it marked as not ready for review. Continuous integration still runs, and the
      merge button is disabled until you mark it ready.
  - flag: "--fill"
    means: Uses your commit messages as the title and body instead of asking you for them.
  - flag: "--web"
    means: Opens the GitHub page to finish the form in a browser instead of in the terminal.

expect: >
  The address of the new pull request printed on its own line, such as
  `https://github.com/nyxlocke/myproject/pull/12`. Ctrl+click it to open.

see_also:
  - gh-pr-list
  - gh-pr-merge
  - git-push-set-upstream
  - d8-pull-requests

keywords:
  - open a pull request
  - propose changes
  - create pr
  - submit for review
---

Push the branch first. Without an upstream, the command either fails or offers to push for
you, and the offer is easy to miss. `git push -u origin <branch-name>` settles it.

A PR (Pull Request) is worth opening even when you work alone. It gives you one page showing
every change side by side, which is a far better review surface than a terminal diff.
