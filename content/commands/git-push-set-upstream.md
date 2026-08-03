---
id: git-push-set-upstream
title: git push -u origin main
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git push -u origin <branch-name>
shell: any

does: >
  Uploads a branch to GitHub for the first time and records which remote branch it belongs
  to, so afterward a bare `git push` and `git pull` know where to go.

flags:
  - flag: "-u"
    means: >
      Short for `--set-upstream`. It writes a permanent note in your repository's config
      pairing this local branch with the remote branch you just pushed to. That pairing is
      what lets you type `git push` alone from then on, and it is what makes `git status`
      able to say "your branch is ahead of origin/main by 2 commits". Nobody explains this
      and it is the whole point of the flag.
  - flag: "origin"
    means: >
      The nickname for the remote repository, set automatically when you clone. It is a
      name, not a keyword, so a repository can have others. `git remote -v` shows which
      nicknames exist and what addresses they point at.
  - flag: "<branch-name>"
    means: >
      The branch to push, which should be the branch you are standing on. `git status`
      prints it on the first line if you are unsure.

expect: >
  The normal push output, then a final line reading
  `branch 'feature/login' set up to track 'origin/feature/login'.` That last line is the
  part that matters: it confirms the pairing was recorded.

see_also:
  - git-push
  - git-switch-create
  - git-remote-v
  - d2-repo-remote-clone-origin

keywords:
  - set upstream
  - push a new branch
  - no upstream branch
  - git push dash u
---

You need `-u` exactly once per branch. After that, plain `git push` works for that branch
forever. Running it again is harmless.
