---
id: gh-pr-merge
title: gh pr merge
type: command
verified: 2026-08-02
volatility: quarterly

tool: gh
command: gh pr merge <pr-number> --squash --delete-branch
shell: powershell

does: >
  Merges a pull request into its target branch on GitHub and optionally cleans up the branch
  it came from.

flags:
  - flag: "--squash"
    means: >
      Combines every commit on the branch into one commit on the target branch. Your messy
      work-in-progress history collapses into a single tidy entry. The usual choice for a
      solo project.
  - flag: "--merge"
    means: >
      Keeps every commit and adds a merge commit joining the two lines of history. Choose this
      when the individual commits are worth keeping.
  - flag: "--rebase"
    means: >
      Replays each commit onto the target branch with no merge commit. Straight history, new
      commit hashes.
  - flag: "--delete-branch"
    means: >
      Deletes the branch on GitHub after the merge, and your local copy too if you are
      standing elsewhere. The commits survive inside the target branch.
  - flag: "--auto"
    means: >
      Waits and merges by itself once the required checks pass, instead of failing now because
      a test is still running.

danger: >
  `--delete-branch` removes the branch from GitHub and locally. Everything merged is safe
  inside the target branch. Anything on that branch that was not part of the merge, such as
  a commit pushed after the pull request was approved, loses its only branch name.

undo: >
  GitHub offers a Revert button on a merged pull request, which opens a new pull request
  undoing it. From the terminal, `git revert -m 1 <merge-commit-hash>` does the same. To bring
  a deleted branch back, use `git switch -c <branch-name> <commit-hash>` and push it again.

expect: >
  A confirmation naming the merge method, such as
  `Merged pull request #12 (add login redirect)`, followed by a line about the deleted branch
  if you asked for that.

see_also:
  - gh-pr-create
  - gh-pr-list
  - git-revert
  - d8-pull-requests

keywords:
  - merge a pull request
  - squash merge
  - close pr
  - delete branch after merge
---

Run `gh pr checks <pr-number>` first. Merging a pull request whose tests are still red is the
fastest way to break the main branch, and the terminal will not stop you.

Afterward, switch to `main` and run `git pull` so your local copy has the merge. It does not
arrive by itself.
