---
id: git-branch
title: git branch
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git branch
shell: any

does: >
  Lists the branches in your repository and marks the one you are currently on with an
  asterisk.

flags:
  - flag: "-a"
    means: >
      Lists remote branches as well as local ones. Remote entries appear with an
      `origin/` prefix, such as `remotes/origin/main`.
  - flag: "-r"
    means: Lists only the remote branches, leaving out your local ones.
  - flag: "-v"
    means: >
      Adds the latest commit hash and message beside each branch name, so you can see how
      far along each one is.
  - flag: "--show-current"
    means: >
      Prints just the current branch name and nothing else. Useful when you want the name
      to read quickly rather than a list to scan.
  - flag: "<new-branch-name>"
    means: >
      Creates a branch with that name but leaves you standing where you are.
      `git switch -c <new-branch-name>` creates it and moves you onto it, which is nearly
      always what you meant.

expect: >
  A list with one branch per line and an asterisk on the current one, for example
  `* main` and `  feature/login`. Press `q` if the list is long enough to open a pager.

see_also:
  - git-switch
  - git-switch-create
  - git-branch-delete
  - d5-branches

keywords:
  - list branches
  - what branches exist
  - see all branches
  - which branch
---
