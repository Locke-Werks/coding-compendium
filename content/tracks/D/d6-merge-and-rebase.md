---
id: d6-merge-and-rebase
title: Merge and rebase
type: section
track: D
order: 60
verified: 2026-08-02
volatility: low
answer: >
  Merge combines two branches with one new commit that has two parents. Rebase
  replays your commits on top of another branch as brand new commits, giving a
  straight history, and rewrites anything you have already pushed.
danger: >
  Rebase replaces your commits with new ones carrying new hashes, so a branch you
  have already pushed no longer matches GitHub and the next push is rejected.
  Merging instead is always safe and never rewrites anything. If you must push a
  rebased branch, git push --force-with-lease refuses when the remote has moved,
  while a plain force push overwrites whatever is there, including commits
  somebody else pushed that you have never seen.
owns:
  - merge
  - rebase
  - fast-forward
  - squash
see_also:
  - d7-merge-conflicts
  - d8-pull-requests
  - d5-branches
  - d11-when-you-lose-work
keywords:
  - git merge
  - git rebase
  - fast forward
  - squash commits
  - merge commit
  - rewrite history
  - linear history
---

## More

Two branches, two ways to bring them together.

**Merge** keeps both lines of work and records that they came back together. Stand on the
branch that should receive the work, then name the branch to bring in:

```powershell
git switch main
git merge feature/login
```

Both commands run together because the second is meaningless without the first: git merges
into wherever you are standing. Verify with `git log --oneline --graph -8`, which now shows
a commit whose message begins `Merge branch`, with two lines feeding into it.

**Fast-forward** is the case where no merge commit is needed. If `main` has not moved since
you branched, git slides the `main` label forward to your branch's newest commit and prints
`Fast-forward`. Nothing is lost and nothing special happened; there was only ever one line
of work.

**Rebase** rewrites your branch so it looks like you started from the latest `main`. Stand
on your own branch this time:

```powershell
git switch feature/login
git rebase main
```

Git sets your commits aside, moves your branch to the tip of `main`, then replays each of
your commits on top, one at a time. Verify with `git log --oneline --graph -8`: your commits
are still there in order, sitting above main's newest commit, in a straight line with no
fork.

The part to understand before you use it: a replayed commit is a new commit with a new hash,
because a commit's hash comes from its contents and its parent has changed
([d1](#d1-what-git-actually-stores)). Your old commits still exist, but nothing points at
them anymore. This is why the rule is: rebase only commits you have not shared. Your own
unpushed branch is fine. A branch someone else has pulled, or `main` in any circumstance, is
not.

**Squash** collapses several commits into one, usually when a branch of eight small commits
should land on `main` as a single entry.

If either command stops and says `CONFLICT`, git needs you to decide something. That is
[d7](#d7-merge-conflicts), and both operations can be called off with `--abort`.

## Full

### The same situation, drawn three ways

You branched off, made two commits, and meanwhile `main` gained one. Running
`git log --oneline --graph --all` shows a fork:

```text
* 1d3e5f7 (main) docs: update README
| * 4b2f7c1 (feature/login) feat: add password reset
| * 9f2c1ab feat: add email login
|/
* 7c8b9a0 chore: initial commit
```

After `git merge feature/login` from `main`:

```text
*   8a1c9de (HEAD -> main) Merge branch 'feature/login'
|\
| * 4b2f7c1 (feature/login) feat: add password reset
| * 9f2c1ab feat: add email login
* | 1d3e5f7 docs: update README
|/
* 7c8b9a0 chore: initial commit
```

Every original commit is untouched, with the same hashes, and one new commit sits on top
with two parents. The shape is a permanent record that two lines of work existed at once.

After `git rebase main` from `feature/login`:

```text
* 2e9f4b1 (HEAD -> feature/login) feat: add password reset
* 6c3a8d2 feat: add email login
* 1d3e5f7 (main) docs: update README
* 7c8b9a0 chore: initial commit
```

One straight line, and look at the hashes: `9f2c1ab` and `4b2f7c1` have become `6c3a8d2`
and `2e9f4b1`. Same changes, same messages, different commits. The originals are still on
disk with nothing referring to them, which is what [d11](#d11-when-you-lose-work) recovers
if a rebase goes wrong.

### Choosing between them

Merge by default. It is safe, it never rewrites anything, and the merge commit is a record
rather than clutter. When a bug appears three months later, the merge commit tells you which
piece of work introduced it.

Rebase when your own unpushed branch has fallen behind `main` and you want to catch it up
before opening a pull request ([d8](#d8-pull-requests)). A branch rebased onto the current
`main` merges as a clean fast-forward and produces a history that reads in order.

Never rebase `main` itself, and never rebase a branch that someone else, or a deployment,
is working from. Do not let an agent do it unprompted either
([e11](#e11-what-to-never-let-an-agent-do)).

### The one flag on merge worth knowing

```powershell
git merge --no-ff feature/login
```

`--no-ff` means no fast-forward: make a merge commit even when git could have slid the label
forward. Some teams require it so that every feature has one commit marking where it landed.
Verify the same way, with `git log --oneline --graph -8`.

### Squashing, and what it costs

```powershell
git merge --squash feature/login
```

Takes every change from the branch, stages it all in one heap, and stops without committing.
You then write one commit message for the lot:

```powershell
git commit -m "feat: add email and password login"
```

Verify with `git log --oneline -3`: one new commit on `main`, and no trace of the eight
commits from the branch. That is the trade. `main` reads as a list of features, and the
step-by-step history of how the feature was built is gone from it, which matters when you
later want to know which of those eight steps broke something. GitHub offers the same thing
as a button when merging a pull request ([d8](#d8-pull-requests)).

### Pulling is a merge you did not notice

`git pull` is two commands: `git fetch`, which downloads what GitHub has, then `git merge`,
which combines it into your current branch. This is why an ordinary pull sometimes produces
a merge commit saying `Merge branch 'main' of github.com/...`, and occasionally a conflict.

`git pull --rebase` does the second half as a rebase instead, which keeps your local commits
on top of the remote's. It is safe for commits you have not pushed and carries the same rule
as any other rebase. Which one you get by default is a setting covered in
[b3](#b3-tell-git-who-you-are).

### After a rebase, the push is rejected

Rebasing a branch you already pushed leaves your machine and GitHub with different commits
for the same branch, and git stops you:

```text
! [rejected]        feature/login -> feature/login (non-fast-forward)
error: failed to push some refs to 'https://github.com/nyxlocke/sandbox.git'
```

This is git protecting the copy on GitHub, and it is correct to stop. If the branch is
yours alone and you meant to rebase it:

```powershell
git push --force-with-lease
```

`--force-with-lease` replaces the branch on GitHub with yours, but only if GitHub's copy is
still exactly what you last saw. If anything arrived there since, it refuses. That check is
the entire reason to use this form rather than a plain force push, which does not look
first.

Do not run it on a shared branch. Read the `danger` note at the top of this card before you
run it at all.

### When it goes wrong mid-operation

Both commands stop and wait rather than guessing:

```powershell
git merge --abort
```

```powershell
git rebase --abort
```

Either one returns you to exactly where you stood before you started, with your files as
they were. Nothing about being mid-merge or mid-rebase is a trap, and `git status` will
remind you which state you are in and what your options are. Resolving rather than aborting
is [d7](#d7-merge-conflicts).
