---
id: git-push-force-with-lease
title: git push --force-with-lease
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git push --force-with-lease
shell: any

does: >
  Overwrites the branch on GitHub with your local version, but refuses if someone else has
  pushed to it since you last downloaded it.

flags:
  - flag: "--force-with-lease"
    means: >
      Force, with one safety check. Git compares the remote branch against the copy you
      last fetched. If they match, nobody has pushed since, and the overwrite proceeds. If
      they differ, git aborts rather than deleting work you have never seen. Always use
      this instead of plain `--force`, which skips the check entirely.
  - flag: "--force-if-includes"
    means: >
      An extra check added in git 2.30 that also confirms your local branch actually
      incorporates what you fetched. Modern git turns it on together with
      `--force-with-lease`.

destructive: true

danger: >
  This replaces the branch history stored on GitHub. Any commit that existed only there is
  no longer reachable by that branch name. The lease check protects you from overwriting a
  teammate's push that you have fetched, and it cannot protect you from overwriting your
  own earlier work. Never run it against `main` on a repository other people use.

destroys: >
  The commits the remote branch pointed at before, if they exist nowhere else. Anyone who
  had already pulled the old version keeps a copy locally and will get a conflict on their
  next pull. On GitHub itself, the old commits stay reachable by hash for a while but drop
  off every branch listing immediately.

safer_first: >
  Run `git fetch` then `git log --oneline --graph --all` and look at what `origin/<branch>`
  currently points at compared with your branch. Make a backup branch with
  `git switch -c backup-before-force` and `git switch -` so your current commits have a
  second name holding them.

undo: >
  Push the old commit back: find it with `git reflog` or in the GitHub Activity tab, then
  `git push --force-with-lease origin <old-commit-hash>:<branch-name>`. This works only
  while some copy of the old hash still exists somewhere.

expect: >
  A push summary with a `+` and the word `forced`, such as
  `+ 8c4d2e1...3f2a1b9 main -> main (forced update)`. If it stops with
  `stale info`, the lease check did its job: run `git fetch` and look at what changed before
  trying again.

see_also:
  - git-push
  - git-rebase
  - git-commit-amend
  - e11-what-to-never-let-an-agent-do

keywords:
  - force push
  - overwrite remote branch
  - rejected non-fast-forward
  - push after rebase
---

You only need this after rewriting history that was already pushed, which means after a
rebase or an amend. If you have not rewritten anything, a normal `git push` is the right
command and a rejection means you should pull first.
