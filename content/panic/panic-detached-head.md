---
id: panic-detached-head
title: Git says detached HEAD and I do not know what I broke
type: panic
verified: 2026-08-02
volatility: low
symptom: >
  Git printed a warning about a detached HEAD state, or `git status` no longer
  says I am on a branch.
reassurance: >
  Nothing is broken and nothing has been deleted. You are looking at one specific
  point in the history instead of standing on a branch. Every branch you had is
  exactly where you left it. If you saved checkpoints while you were here, one
  command keeps them, and this is the only part of the situation that needs any
  care at all.
backup_first: git branch backup-now
root: made-anything
nodes:
  made-anything:
    ask: >
      Since that message appeared, have you changed or saved anything, or were
      you only looking around?
    how_to_tell: git status
    branches:
      - label: Only looking, nothing changed
        goto: just-leave
      - label: I changed files but did not save a checkpoint
        goto: uncommitted-here
      - label: I saved at least one checkpoint here
        goto: commits-here
      - label: I do not know
        goto: check-status

  check-status:
    ask: Ask git what state you are actually in.
    resolve:
      command: git status
      shell: powershell
      does: >
        The first line reads `HEAD detached at` followed by a hash, which
        confirms it. Anything listed below under changes or untracked files is
        unsaved work sitting here with you.
      destroys: Nothing. It only reads.
      verify: >
        You now know whether anything is listed. Go back and answer the question.
      if_it_did_not_work: >
        To check for checkpoints made here, run `git log --oneline -5`. If the top
        entry is something you did in the last few minutes, you have work to keep
        and should answer that way.

  just-leave:
    ask: Put yourself back on a branch.
    resolve:
      command: git switch -
      shell: powershell
      does: >
        Returns you to whatever branch you were on before. The dash means the
        previous one.
      destroys: Nothing.
      verify: >
        `git status` starts with `On branch` again, and `git branch
        --show-current` prints a name instead of nothing.
      if_it_did_not_work: >
        If the dash form fails, name the branch: `git switch main`. If git says
        local changes would be overwritten, you do have work here after all, so
        go back and pick the answer about changed files.

  uncommitted-here:
    ask: Give this spot a name and take the changes with you.
    resolve:
      command: git switch -c rescue-work
      shell: powershell
      does: >
        Creates a branch exactly where you are standing and moves you onto it,
        carrying your uncommitted changes along. You are on a branch again and
        the work is attached to something.
      destroys: Nothing.
      verify: >
        `git branch --show-current` prints `rescue-work`, and `git status` still
        lists your changes.
      if_it_did_not_work: >
        Save them now so they are safe: `git add -A` then `git commit -m "wip:
        rescued work"`. Merge the branch into your normal one when you are ready,
        or see d5-branches.

  commits-here:
    ask: Name this spot, so the checkpoints belong to a branch instead of floating.
    resolve:
      command: git switch -c rescue-work
      shell: powershell
      does: >
        Creates a branch at your current position and moves you onto it. Your
        checkpoints stop being unreferenced, which means they show up in `git
        log` normally and will not be cleaned up later.
      destroys: Nothing. No other branch is affected.
      verify: >
        `git log --oneline -5` still shows your checkpoints, and `git branch
        --show-current` prints `rescue-work`.
      if_it_did_not_work: >
        If you already switched away before doing this, the checkpoints are not
        lost. `git reflog` lists every position you have been in, and
        d11-when-you-lose-work turns one back into a branch.

see_also:
  - d5-branches
  - d11-when-you-lose-work
  - d10-undo-everything
  - d13-tags-releases-and-history
  - panic-lost-my-changes
  - d1-what-git-actually-stores
keywords:
  - detached head
  - not on any branch
  - head detached at
  - you are in detached head state
  - my commits are not on a branch
  - how did i get here
---

Normally you stand on a branch, which is a moving name that follows you as you save
checkpoints. Detached means you are standing on a checkpoint directly, with no name
attached. Git tells you because anything you save here belongs to nothing, and a checkpoint
that no name points at eventually gets cleaned up.

Four things put you here, and none of them is a mistake in itself. Checking out a specific
hash to see an old version. Checking out a tag. Finishing a `git bisect` without running
`git bisect reset`. Or an agent doing any of those on your behalf while investigating
something.

If you were only looking, walk away with `git switch -`. If you did work here, name it with
`git switch -c rescue-work` before you go anywhere else. That is the entire situation.
