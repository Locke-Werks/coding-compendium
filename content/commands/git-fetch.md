---
id: git-fetch
title: git fetch
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git fetch
shell: any

does: >
  Downloads new commits and branches from GitHub without changing any of your files or
  your current branch.

flags:
  - flag: "--all"
    means: Fetches from every remote you have configured, not only `origin`.
  - flag: "--prune"
    means: >
      Deletes your local records of remote branches that no longer exist on GitHub. Without
      it, `git branch -r` keeps listing branches that were merged and deleted months ago.
  - flag: "--tags"
    means: Also downloads tags, which an ordinary fetch may skip depending on the tag.

expect: >
  Nothing at all when there is nothing new. Otherwise a list such as
  `* [new branch]      feature/login -> origin/feature/login` and a range line for updated
  branches.

see_also:
  - git-pull
  - git-status
  - git-log-oneline-graph
  - d2-repo-remote-clone-origin

keywords:
  - check for updates
  - download without merging
  - see remote branches
  - prune deleted branches
---

Fetch is the safe half of `git pull`. It brings the information down so you can look at it
with `git log --oneline --graph --all` and decide what to do, without touching a single
file in your working folder.

After fetching, `git status` can tell you `Your branch is behind 'origin/main' by 3
commits`, which it cannot know before.
