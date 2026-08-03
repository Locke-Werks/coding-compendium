---
id: d8-pull-requests
title: Pull requests, even when you work alone
type: section
track: D
order: 80
verified: 2026-08-02
volatility: quarterly
verify: gh pr status
answer: >
  A pull request is a proposal to merge one branch into another with a
  before-and-after view of every changed line attached. Open one even when you
  work alone, because that view is the cheapest review surface you will get.
owns:
  - pull request
  - PR review
  - gh pr create
  - squash merge
see_also:
  - d9-reading-a-diff
  - h3-reviewing-a-diff-you-cannot-fully-read
  - d5-branches
  - d6-merge-and-rebase
  - b4-github-and-gh
  - h5-ci-cd
keywords:
  - PR
  - merge request
  - gh pr create
  - open a pull request
  - files changed
  - review my own code
  - squash and merge
---

## More

A **pull request** is a proposal: here are the commits on this branch, please put them
into that branch. GitHub attaches a full before-and-after view of every changed line and
gives you a page to write down why. GitLab calls the same thing a merge request, so you
will meet both PR (Pull Request) and MR (Merge Request) in documentation about the same
idea.

Working alone, the merge is not the reason to bother. You could merge locally in one
command. The reason is the page: every file that changed, in one scrollable list, with
additions and deletions marked. That is where you notice that a change to the login form
also rewrote three test files. [h3](#h3-reviewing-a-diff-you-cannot-fully-read) covers what
to look for once you are staring at it, and [d9](#d9-reading-a-diff) covers what the
symbols mean.

The flow, starting from a branch with work on it. Branches are [d5](#d5-branches) if that
word is new.

```powershell
git push -u origin feature/login
```

Sends the branch to GitHub. `-u` records the pairing between your local branch and the
GitHub one, so every later push on this branch is just `git push`.

```powershell
gh pr create --fill
```

Opens the pull request without leaving the terminal. `--fill` takes the title and body
from your branch name and commit messages instead of asking you four questions. It prints
a link when it works. If you get `gh: The term 'gh' is not recognized`, the GitHub
command-line tool is not installed yet: [b4](#b4-github-and-gh).

```powershell
gh pr view --web
```

Opens it in your browser, which is where the Files changed tab lives. Read that tab before
you merge, every time.

When you are satisfied:

```powershell
gh pr merge --squash --delete-branch
```

`--squash` collapses every commit on the branch into one commit on `main`. `--delete-branch`
removes the branch from GitHub and from your machine afterward. Confirm it worked with
`gh pr status`: the pull request moves into the Merged column.

One tradeoff, stated plainly: a pull request adds two steps to every piece of work. For a
one-line typo fix on a project only you will ever open, merging locally is fine. For
anything an agent wrote, open the pull request.

## Full

### The four tabs, and which one matters

A pull request page on GitHub has four tabs and only one of them is load-bearing.

- **Conversation.** The description, plus comments. Useful as a record of why, months
  later, when you have forgotten.
- **Commits.** The individual checkpoints on the branch, in order. Worth a glance to see
  whether the work arrived in sensible pieces.
- **Files changed.** Every changed line in every file. This is the tab. Everything else is
  supporting material.
- **Checks.** Automated runs, if the project has any. Covered below.

The Files changed tab has a counter at the top: files changed, additions, deletions. Read
those three numbers before you read a single line. A change you described as "fix the
button label" that reports 34 files changed is telling you something important before you
have looked at anything.

### Squash, merge commit, and rebase merge

GitHub offers three merge buttons and they produce different histories.

**Squash and merge** takes every commit on the branch and lands them as one commit on
`main`. The branch's messy intermediate steps disappear. This is the default recommendation
for solo work and for anything an agent built, because agent branches accumulate commits
like `fix typo` and `actually fix typo`, and none of that helps you six months from now.

**Create a merge commit** keeps every commit on the branch and adds one more that joins the
two lines of history together. Use it when the individual commits are genuinely worth
keeping.

**Rebase and merge** replays each commit onto the tip of `main` with no join commit. It
produces a straight line. [d6](#d6-merge-and-rebase) covers what rebasing actually does and
why rewriting history has a cost.

Pick squash unless you have a reason. From the terminal:

```powershell
gh pr merge --squash --delete-branch
```

### Reviewing your own pull request

Reviewing your own work feels pointless and is not. The trick is to change your posture:
you are not re-reading code you wrote, you are auditing a stranger's proposal that claims
to do one specific thing.

Read the description first, then the file list, then the diff. In that order. The
description states what the change is supposed to do; the file list is your first chance to
notice it did something else.

You can leave comments on your own pull request. Do it. A note saying "not sure this
handles an empty list" is a real reminder that survives the session, and it is a much
better prompt to hand an agent than "look it over again."

### Checks, and what the green mark means

If the project has CI (Continuous Integration), which is automation that runs your tests
whenever you push, the pull request grows a Checks section. Green means every configured
check passed. Red means one failed and the pull request will usually refuse to merge.

```powershell
gh pr checks
```

Lists each check and its result without opening a browser. A failing check is not an
opinion about your code quality; it is one command exiting non-zero somewhere. [h5](#h5-ci-cd)
covers finding the actual error inside a long run log.

Green means the checks that exist passed. It does not mean the change is correct. A project
with two trivial tests goes green on almost anything.

### Draft pull requests

```powershell
gh pr create --draft --fill
```

A draft is a pull request that cannot be merged until you mark it ready. Use it when you
want the diff view while you are still working, which is a genuinely good habit: it gives
you a running before-and-after of the whole branch without committing to anything. Mark it
ready with `gh pr ready`.

### Keeping the branch current

If `main` moves while your branch is open, GitHub may say the branch is out of date. Bring
it up to date from your machine:

```powershell
git switch feature/login
git merge main
```

Confirm with `git status`, which should say your branch is ahead of `origin/feature/login`
by one commit. Push and the pull request updates itself. If the merge stops with a
conflict, that is normal and [d7](#d7-merge-conflicts) is the card for it.

### Staying in the terminal

Every part of a pull request has a command, if you would rather not switch to a browser.

```powershell
gh pr diff
```

Prints the whole diff for the current branch's pull request in the terminal. It goes
through a pager, so press `q` to get out and the spacebar to page down.

```powershell
gh pr checkout 14
```

Switches your working folder to the branch behind pull request number 14. This is how you
run someone else's proposed change, or your own from a different machine.

```powershell
gh pr status
```

Shows the pull requests you have open and their check results. This is the fastest way to
confirm a merge landed.

### When a pull request is the wrong tool

Two honest cases. If you are the only person on a private project and the change is a typo
in a comment, the ceremony costs more than it returns; commit to `main` and move on. And if
the branch has grown to 60 files because you kept going for three days, the pull request
will not save you, because nobody reviews a 60-file diff carefully, including you. The fix
for that one is upstream: smaller branches, merged sooner. [a4](#a4-the-loop) is the rhythm
that keeps them small.
