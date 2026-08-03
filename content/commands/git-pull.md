---
id: git-pull
title: git pull
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git pull
shell: any

does: >
  Downloads new commits from GitHub for your current branch and merges them into your
  local copy, in one step.

flags:
  - flag: "--rebase"
    means: >
      Replays your local commits on top of the downloaded ones instead of creating a merge
      commit. History stays a straight line. It also rewrites your local commit hashes, so
      only use it on commits you have not pushed.
  - flag: "--ff-only"
    means: >
      Refuses to do anything except move your branch straight forward. If your work and the
      remote work have diverged, it stops and tells you instead of starting a merge you did
      not expect. The safest form when you are unsure.
  - flag: "origin <branch-name>"
    means: >
      Names the remote and branch explicitly. Needed only when the branch has no upstream
      set, which is what `git push -u` establishes.

expect: >
  `Already up to date.` when there is nothing new. Otherwise a range such as
  `Updating 3f2a1b9..8c4d2e1` followed by `Fast-forward` and a file summary.

see_also:
  - git-fetch
  - git-push
  - git-merge
  - d6-merge-and-rebase

keywords:
  - get latest changes
  - update from github
  - pull changes
  - divergent branches
---

`git pull` is `git fetch` followed by `git merge`. Because of the merge half, it can stop
on a conflict and leave your working folder mid-merge. `git merge --abort` gets you out.

If it complains `You have divergent branches and need to specify how to reconcile them`,
git wants a policy. `git pull --ff-only` for this one time, or set the default with
`git config --global pull.rebase false`.
