---
id: git-branch-ahead-of-origin
title: "Your branch is ahead of 'origin/main' by N commits"
type: error
verified: 2026-08-02
volatility: low

category: config

# The short form of status. A clean line like `## main...origin/main` with
# nothing after it means local and GitHub agree.
verify: git status -sb

sample: |
  PS C:\Users\you\dev\scraper> git status
  On branch main
  Your branch is ahead of 'origin/main' by 3 commits.
    (use "git push" to publish your local commits)

  nothing to commit, working tree clean

patterns:
  - "Your branch is ahead of"
  - "to publish your local commits"

means: >
  Nothing failed. Git is reporting a fact: you have made three commits on this machine that
  GitHub does not have yet. A commit saves to your own disk only. Nothing leaves your
  machine until you push. Git prints this line on every `git status` until you either push
  the commits or get rid of them, so it will keep appearing and it is not a warning about
  anything being wrong.

fix_ladder:
  - try: Send them to GitHub.
    command: git push
    shell: powershell
    why: >
      Assumes the commits are finished work and you want them backed up. This is the answer
      almost every time. After it finishes, `git status` stops mentioning being ahead.

  - try: Look at what those commits actually are first.
    command: git log origin/main..HEAD --oneline
    shell: powershell
    why: >
      Assumes you do not remember making three commits, which is normal when an agent has
      been committing as it works. The two dots mean "everything on my side that the remote
      does not have", so this lists exactly what a push would publish.

  - try: Check whether you are also behind.
    command: git status -sb
    shell: powershell
    why: >
      Assumes the two sides have diverged, meaning each has commits the other does not. If the output
      says ahead and behind at the same time, a plain push will be rejected, and you need
      to pull first.

  - try: Confirm there is a remote to push to at all.
    command: git remote -v
    shell: powershell
    why: >
      Assumes no remote is configured. If this prints nothing, the repository exists only
      on your machine, there is no `origin/main` to be ahead of, and you would not be seeing
      this message. If it prints an address you do not recognize, that is worth sorting out
      before you push anything.

if_none_worked: >
  Paste the full `git status` output rather than the one line about being ahead, plus the
  output of `git log origin/main..HEAD --oneline`. The commit list is the part people leave
  out, and it is the difference between "push it" and "you have three copies of the same
  commit from a rebase gone sideways".

see_also:
  - d4-commit-well
  - d2-repo-remote-clone-origin
  - d3-the-three-places

keywords:
  - branch is ahead
  - unpushed commits
  - git push
  - out of sync with origin
  - publish local commits
---

Commit and push are two separate steps, and this message exists because people expect
them to be one.

A commit writes a snapshot into the `.git` folder on your disk. That is all it does. It
does not touch the network, it does not need the network, and it does not tell GitHub
anything. A push copies those snapshots up. Until you push, the only copy of that work is
on your machine, which is the argument for pushing often rather than at the end of the day.

The mirror image is "Your branch is behind 'origin/main' by N commits", which means GitHub
has work you do not. That one is fixed by `git pull`.

If both lines appear at once, the branches have diverged and a push will be rejected. That
is a different situation with its own card.
