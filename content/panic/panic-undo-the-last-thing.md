---
id: panic-undo-the-last-thing
title: I want to undo the last thing I did
type: panic
verified: 2026-08-02
volatility: low
danger: >
  Two answers in this tree destroy work. `git restore` throws away every
  uncommitted edit to a file, and `git reset --hard` throws away a checkpoint and
  every uncommitted change in the folder at once. Neither has an undo for the
  uncommitted part. Run `git stash push -u` first if there is any chance you want
  it back, and use `git revert` for anything already sent to GitHub.
symptom: >
  Something just went wrong and I want it back the way it was a minute ago,
  whatever that takes.
reassurance: >
  Almost everything in git is reversible, and git keeps a private log of every
  position you have been in for about ninety days, so even a wrong undo is
  usually undoable. The one thing with no recovery is uncommitted work thrown
  away by a sharp command, which is why every branch below states what it costs
  before you run it.
backup_first: git branch backup-now
root: what-was-it
nodes:
  what-was-it:
    ask: What was the last thing you did?
    how_to_tell: git reflog -10
    branches:
      - label: I edited files and want them back the way they were
        goto: undo-edits
      - label: I ran git add and want to take that back
        goto: undo-staging
      - label: I saved a checkpoint and want it undone
        goto: undo-commit
      - label: I sent it to GitHub
        goto: undo-pushed
      - label: I switched branches, merged, or rebased
        goto: use-reflog
      - label: I genuinely do not know
        goto: use-reflog

  undo-edits:
    ask: Put the file back the way it was at your last checkpoint.
    resolve:
      command: git restore "<path/to/the/file>"
      shell: powershell
      does: >
        Rewrites that file on disk with its contents from your last checkpoint.
      destroys: >
        Every edit to that file since your last checkpoint. There is no undo for
        this and reflog cannot bring it back, because it was never committed.
      verify: >
        `git status` no longer lists the file and `git diff` prints nothing for
        it.
      if_it_did_not_work: >
        If you have not run it yet and you are unsure, shelve instead: `git stash
        push -u` gives you the same clean file and keeps a copy. To restore every
        changed file at once, `git restore .` carries the same cost across all of
        them.

  undo-staging:
    ask: Take the file back out of the queue for the next checkpoint.
    resolve:
      command: git restore --staged "<path/to/the/file>"
      shell: powershell
      does: >
        Removes the file from the staging area. Its contents on disk do not
        change at all.
      destroys: Nothing. Your edits are untouched.
      verify: >
        `git status` moves the file from the staged group into the unstaged
        group, with the edits still listed.
      if_it_did_not_work: >
        To unstage everything at once, run `git restore --staged .`. If you meant
        to undo the edits as well, go back and pick the first answer instead.

  undo-commit:
    ask: Do you want to keep the work that was in it, or throw the work away too?
    branches:
      - label: Keep the work, I only want the checkpoint undone
        goto: soft-reset
      - label: Throw all of it away
        goto: hard-reset
      - label: It is already on GitHub
        goto: undo-pushed

  soft-reset:
    ask: Remove the checkpoint and keep everything it held.
    resolve:
      command: git reset --soft HEAD~1
      shell: powershell
      does: >
        Removes the most recent checkpoint. Every change it contained goes back
        into the staging area, ready to be committed again once you have fixed
        whatever was wrong.
      destroys: Nothing.
      verify: >
        `git log --oneline -3` no longer shows it at the top, and `git status`
        shows its contents staged.
      if_it_did_not_work: >
        If you only wanted a better message, `git commit --amend -m "<the better
        message>"` does that in one step. Do not amend anything already pushed.

  hard-reset:
    ask: Remove the checkpoint and the work with it.
    resolve:
      command: git reset --hard HEAD~1
      shell: powershell
      does: >
        Removes the most recent checkpoint and rewrites every file in your folder
        to match the one before it.
      destroys: >
        The checkpoint, and every uncommitted change anywhere in the folder,
        including changes that had nothing to do with it. There is no undo for
        the uncommitted part. Run `git stash push -u` first if there is any doubt
        at all.
      verify: >
        `git log --oneline -3` no longer shows the checkpoint and `git status`
        reports a clean tree.
      if_it_did_not_work: >
        If you went one step too far, the checkpoint itself is not really gone.
        `git reflog` lists it and `git switch -c rescue-work <hash>` brings it
        back. Uncommitted changes are the part reflog cannot recover.

  undo-pushed:
    ask: Cancel a checkpoint that other copies may already have.
    resolve:
      command: git revert --no-edit HEAD
      shell: powershell
      does: >
        Adds a new checkpoint that is the exact opposite of the last one. The
        history keeps both, which is what makes this safe for anything already
        sent to GitHub.
      destroys: >
        Nothing. It adds a checkpoint rather than removing one, so no other copy
        of the branch breaks.
      verify: >
        `git log --oneline -3` shows a new entry starting with `Revert`, and the
        file no longer holds the change. Push normally.
      if_it_did_not_work: >
        To cancel something further back, take its hash from `git log --oneline`
        and run `git revert --no-edit <hash>`. If git stops on a conflict,
        panic-merge-conflict-stuck covers it.

  use-reflog:
    ask: >
      Git logged every position you have been in. Find the line describing the
      moment before the thing you want undone.
    how_to_tell: git reflog -20
    branches:
      - label: I found the line I want
        goto: jump-back
      - label: Nothing in the list looks right
        goto: widen-search

  jump-back:
    ask: Make a branch at that old position.
    resolve:
      command: git switch -c rescue-work <the-hash-from-the-left-column>
      shell: powershell
      does: >
        Creates a branch sitting exactly at that position and moves you onto it.
        Your files become that version.
      destroys: >
        Nothing. The branch you were on is untouched, so a wrong guess costs you
        nothing but the time.
      verify: >
        `git log --oneline -5` shows the history you expected and the files look
        right.
      if_it_did_not_work: >
        If uncommitted changes are in the way, shelve them with `git stash push
        -u` and run it again. If the position was wrong, pick a different line
        and use a different branch name.

  widen-search:
    ask: Look further back through the log of positions.
    resolve:
      command: git reflog -100
      shell: powershell
      does: >
        Shows the last hundred moves instead of twenty. The log holds roughly
        ninety days, so almost anything from the last few weeks is in here.
      destroys: Nothing. It only reads.
      verify: >
        When a line matches the state you want, take its hash and go back to the
        previous step.
      if_it_did_not_work: >
        If it truly is not there, the work may never have been committed.
        d11-when-you-lose-work covers the recovery routes outside git.

see_also:
  - d10-undo-everything
  - d11-when-you-lose-work
  - d3-the-three-places
  - panic-lost-my-changes
  - panic-wrong-branch
  - panic-merge-conflict-stuck
keywords:
  - undo
  - take it back
  - go back to before
  - i broke something
  - revert the last commit
  - unstage
  - reset
  - what did i just do
---

Every undo in git is a choice about which of three places you want changed: the files in
your folder, the staging area, or the saved history. The first question in the tree is
really asking which of the three you touched last.

Two rules decide almost every case. Anything already sent to GitHub gets `git revert`, which
cancels a checkpoint by adding another one and leaves every other copy working. Anything
still only on your machine can use `git reset`, which walks the branch backward and is sharp
enough to be worth reading the cost line first.

If you are not sure what you did, `git reflog` is the honest answer. It lists every position
you have occupied, newest first, with the action that caused each one. The line below the
mistake is where you were before it, and making a branch there costs nothing and destroys
nothing.

[d10](#d10-undo-everything) has the same commands ranked by exactly how much each one
destroys, if you would rather read the whole list than answer questions.
