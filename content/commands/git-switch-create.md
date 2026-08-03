---
id: git-switch-create
title: git switch -c
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git switch -c <new-branch-name>
shell: any

does: >
  Creates a new branch starting from where you are now and immediately moves you onto it.

flags:
  - flag: "-c"
    means: >
      Create. The new branch starts at your current commit, so it contains everything your
      current branch contains. Without `-c`, git looks for an existing branch by that name
      and fails if there is not one.
  - flag: "-c <new-branch-name> <start-point>"
    means: >
      Starts the new branch from a named commit, tag, or other branch instead of from where
      you are standing. For example `git switch -c hotfix main`.
  - flag: "-C"
    means: >
      Capital C creates the branch even if that name already exists, resetting the existing
      branch to your current commit. Use the lowercase form unless you specifically want
      that.

expect: >
  `Switched to a new branch 'feature/login'`. Any uncommitted changes you had come with
  you, which is intentional and usually what you want.

see_also:
  - git-switch
  - git-branch
  - git-push-set-upstream
  - d5-branches

keywords:
  - new branch
  - create a branch
  - start a feature branch
  - branch off
---

Branch names cannot contain spaces. Use slashes and dashes: `feature/login-redirect`,
`fix/crash-on-save`. The slash is only a naming convention, and git treats it as an
ordinary character rather than a folder.
