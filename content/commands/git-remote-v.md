---
id: git-remote-v
title: git remote -v
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git remote -v
shell: any

does: >
  Lists the nicknames this repository has for remote copies of itself, along with the
  actual addresses each nickname points at.

flags:
  - flag: "-v"
    means: >
      Verbose. Without it you get the bare nicknames and nothing else, which does not answer
      the question you were asking. With it you get the addresses too.

expect: >
  Two lines per remote, one tagged `(fetch)` and one tagged `(push)`, usually identical.
  For example `origin  https://github.com/yourname/myproject.git (fetch)`. A repository
  created locally with `git init` and never connected prints nothing.

see_also:
  - git-remote-add
  - git-clone
  - git-push-set-upstream
  - b5-ssh-vs-https

keywords:
  - where does this push to
  - what is origin
  - check remote url
  - which github repo
---

Run this when you are unsure which GitHub repository your commits are about to go to,
which happens most often on a project you cloned from someone else.

The address form tells you how you authenticate. Starting with `https://` means credentials
or a token. Starting with `git@` means an SSH (Secure Shell) key.
