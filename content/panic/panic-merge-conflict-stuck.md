---
id: panic-merge-conflict-stuck
title: There is a conflict and now nothing works
type: panic
verified: 2026-08-02
volatility: low
danger: >
  Aborting throws away the edits you made while resolving the conflict, though
  everything committed on either side survives untouched. `git checkout --ours`
  and `--theirs` discard one side's version of a file for this combine only. Both
  originals stay in their own checkpoints, so nothing here loses committed work.
symptom: >
  Git said there was a conflict, my files have rows of angle brackets in them,
  and commands I normally run are refusing.
reassurance: >
  A conflict is git refusing to guess between two versions of the same line. It
  is not damage. Both versions are safe in their own checkpoints, and one command
  puts everything back exactly as it was the moment before you started. You can
  always walk away and try again later.
backup_first: git branch backup-now
root: what-now
nodes:
  what-now:
    ask: >
      Do you want to finish combining the two versions, or get back to exactly
      how things were before you started?
    how_to_tell: git status
    branches:
      - label: Get me back to before I started
        goto: which-operation
      - label: I want to finish it
        goto: find-conflicts
      - label: My code will not run and I see rows of angle brackets in a file
        goto: leftover-markers

  which-operation:
    ask: Which word does git use in the first few lines of `git status`?
    how_to_tell: git status
    branches:
      - label: It says merge
        goto: abort-merge
      - label: It says rebase
        goto: abort-rebase
      - label: It says cherry-pick or revert
        goto: abort-other

  abort-merge:
    ask: Cancel the merge.
    resolve:
      command: git merge --abort
      shell: powershell
      does: >
        Cancels the merge and puts every file back to how it was the moment
        before it started.
      destroys: >
        Any edits you made while resolving the conflict. Everything committed on
        either branch is untouched.
      verify: >
        `git status` says the working tree is clean and says nothing about a
        merge in progress.
      if_it_did_not_work: >
        If git says there is no merge to abort, the merge already finished, or
        you are in a rebase instead. `git status` names which one you are in.

  abort-rebase:
    ask: Cancel the rebase.
    resolve:
      command: git rebase --abort
      shell: powershell
      does: >
        Stops the rebase and returns your branch to exactly where it was before
        it started, checkpoints and all.
      destroys: >
        Any edits you made while resolving the conflict, plus any progress the
        rebase had made through its list. Nothing committed before you started is
        affected.
      verify: >
        `git status` reports a clean tree with no rebase in progress, and `git
        log --oneline -5` shows your original checkpoints.
      if_it_did_not_work: >
        If it says there is no rebase in progress, check `git status` again for
        the word merge or cherry-pick and take that branch instead.

  abort-other:
    ask: Cancel the cherry-pick or the revert.
    resolve:
      command: git cherry-pick --abort
      shell: powershell
      does: >
        Cancels the cherry-pick and restores the state from before it. If `git
        status` said revert rather than cherry-pick, use `git revert --abort`
        instead, which behaves the same way.
      destroys: >
        Any edits you made while resolving the conflict. Nothing committed is
        affected.
      verify: >
        `git status` is clean and mentions no operation in progress.
      if_it_did_not_work: >
        If both abort commands say there is nothing to abort, the operation
        already finished. Nothing is stuck, and d10-undo-everything covers
        undoing a completed one.

  find-conflicts:
    ask: How do you want to settle the files git is stuck on?
    how_to_tell: git diff --name-only --diff-filter=U
    branches:
      - label: I will edit them by hand
        goto: edit-by-hand
      - label: One side should win the whole file
        goto: take-one-side
      - label: The list came back empty
        goto: finish-it

  edit-by-hand:
    ask: Fix one file, then tell git it is settled.
    resolve:
      command: git add <the-file-you-just-fixed>
      shell: powershell
      does: >
        Marks that file resolved. Before running it, open the file, delete the
        marker lines and whichever version you do not want, and save.
        d7-merge-conflicts explains what each marker means.
      destroys: >
        Nothing. Both original versions stay in their own checkpoints no matter
        what you save here.
      verify: >
        `git diff --name-only --diff-filter=U` gets shorter each time you do
        this. When it prints nothing, every file is settled.
      if_it_did_not_work: >
        If the file still has angle brackets in it after you saved, you missed a
        marker. Find them with `Select-String -Path <the-file> -Pattern "<<<<<<<"`.

  take-one-side:
    ask: Take one side's version of the file whole.
    resolve:
      command: git checkout --ours <the-file>
      shell: powershell
      does: >
        Replaces the conflicted file with your side of the conflict entirely.
        `--theirs` takes the other side instead. During a rebase the two are
        swapped, which is the most confusing thing about rebasing and worth
        checking with `git status` first.
      destroys: >
        The other side's version of that file, for this combine only. Both
        versions still exist in their own checkpoints and neither is lost.
      verify: >
        Run `git add <the-file>`, then `git diff --name-only --diff-filter=U`.
        When it prints nothing, everything is settled.
      if_it_did_not_work: >
        If you picked the wrong side, abort the whole combine and start again.
        Nothing committed is at risk.

  finish-it:
    ask: Every file is settled. Complete the operation.
    resolve:
      command: git commit --no-edit
      shell: powershell
      does: >
        Finishes a merge using the message git already prepared. If you are in a
        rebase rather than a merge, the command is `git rebase --continue`
        instead, and `git status` tells you which one you are in.
      destroys: Nothing.
      verify: >
        `git status` reports a clean tree with no operation in progress, and `git
        log --oneline -5` shows the combined history.
      if_it_did_not_work: >
        If git says there is nothing to commit, the merge already completed. If a
        rebase stops again on the next checkpoint, that is normal: rebases replay
        one at a time and can conflict more than once.

  leftover-markers:
    ask: Find the marker lines that are stopping the code from running.
    resolve:
      command: git grep -n "^<<<<<<<"
      shell: powershell
      does: >
        Searches every tracked file for leftover conflict markers and prints each
        file and line number. Those lines are not code, so anything containing
        them fails to parse.
      destroys: Nothing. It only reads.
      verify: >
        Clean up each hit, then run the command again. Silence means they are all
        gone. Then run your build or your tests.
      if_it_did_not_work: >
        If it prints nothing but the code still fails, the failure is unrelated
        to the conflict. Read the error message with
        f1-how-to-read-an-error-message.

see_also:
  - d7-merge-conflicts
  - d6-merge-and-rebase
  - d10-undo-everything
  - panic-wrong-branch
  - panic-cant-push
  - f1-how-to-read-an-error-message
keywords:
  - merge conflict
  - conflict markers
  - angle brackets in my code
  - cannot merge
  - rebase stopped
  - how do i get out of this
  - abort merge
---

A conflict happens when two checkpoints change the same lines and git will not choose for
you. It stops, writes both versions into the file with markers between them, and waits.

The markers look alarming and are ordinary text. `<<<<<<<` opens your version, `=======`
divides the two, and `>>>>>>>` closes the other one. They are not valid code in any language,
which is why everything stops working the moment they appear.

You have three exits and all of them are safe. Abort and go back to before. Settle the files
and finish. Or leave it half-done and come back later, because git holds the state until you
decide.

[d7](#d7-merge-conflicts) covers reading the markers and deciding which side is right, which
is the one part of this no tool can do for you.
