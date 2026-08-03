---
id: panic-wrong-branch
title: I did the work on the wrong branch
type: panic
verified: 2026-08-02
volatility: low
danger: >
  One route in this tree uses `git reset --hard`, which throws away every
  uncommitted change in your folder. The step before it saves your committed work
  under a second branch name first, so the checkpoints survive. Run `git stash
  push -u` before starting if you have unsaved edits you want to keep.
symptom: >
  I built the whole thing while standing on main, or on some other branch it was
  never supposed to go on.
reassurance: >
  Nothing is lost and nothing is broken. The work exists. It is attached to the
  wrong name, and moving a name is cheap. If you have not sent anything to GitHub
  yet this costs two commands, and if you have, there is a safe route that adds
  commits rather than removing them.
backup_first: git branch backup-now
root: saved-yet
nodes:
  saved-yet:
    ask: >
      Have you saved a checkpoint yet, or are the changes still sitting in your
      files unsaved?
    how_to_tell: git status
    branches:
      - label: Still unsaved, git lists them as changed or untracked
        goto: move-uncommitted
      - label: I have saved at least one checkpoint
        goto: pushed-yet

  move-uncommitted:
    ask: Move yourself to the right branch and take the changes with you.
    resolve:
      command: git switch <the-right-branch>
      shell: powershell
      does: >
        Moves you to that branch and carries your uncommitted changes across with
        you. Uncommitted work is not attached to any branch, so it travels.
      destroys: >
        Nothing. If carrying the changes would overwrite something on the other
        branch, git refuses and prints which files are in the way.
      verify: >
        `git branch --show-current` prints the right branch, and `git status`
        still lists your changes.
      if_it_did_not_work: >
        If the branch does not exist yet, create it where you stand with `git
        switch -c <new-name>`, which takes the changes with it. If git refused,
        shelve first: `git stash push -u`, then `git switch <branch>`, then `git
        stash pop`.

  pushed-yet:
    ask: Have those checkpoints already been sent to GitHub?
    how_to_tell: git status -sb
    branches:
      - label: No, they are only on my machine
        goto: how-many
      - label: Yes, they are on GitHub
        goto: already-pushed
      - label: I do not know
        goto: check-pushed

  check-pushed:
    ask: Ask git what GitHub already has.
    resolve:
      command: |
        git fetch
        git status -sb
      shell: powershell
      does: >
        Downloads GitHub's current state without touching your files, then prints
        one summary line. `ahead 2` means two of your checkpoints are still only
        here. No mention of ahead means GitHub has them all.
      destroys: Nothing. `fetch` only downloads.
      verify: You have a number now. Go back and answer the previous question.
      if_it_did_not_work: >
        If it says there is no upstream, the branch was never pushed, so answer
        no to the previous question.

  how-many:
    ask: How many checkpoints ended up on the wrong branch?
    how_to_tell: git log --oneline -10
    branches:
      - label: Just one
        goto: move-local-one
      - label: More than one
        goto: move-local-many

  move-local-one:
    ask: Save the work under the right name, then walk the wrong branch back.
    resolve:
      command: |
        git branch <the-right-branch-name>
        git reset --hard HEAD~1
        git switch <the-right-branch-name>
      shell: powershell
      does: >
        Line one makes a new branch pointing at your work, so the checkpoint is
        safe under a second name. Line two walks the wrong branch back one step,
        forgetting it. Line three puts you on the new branch, where the work is.
      destroys: >
        The reset throws away any uncommitted edits sitting in your files right
        now. The committed work is safe, because line one saved it first. If you
        have unsaved edits, run `git stash push -u` before you start.
      verify: >
        `git branch --show-current` prints the new branch and `git log --oneline
        -3` shows your work on it. Switch to the old branch and the same command
        no longer shows it.
      if_it_did_not_work: >
        If you walked back one step too many, nothing is gone. `git reflog` lists
        every position you have been in, and d11-when-you-lose-work walks it back.

  move-local-many:
    ask: Save the work under the right name, then walk the wrong branch back that many steps.
    resolve:
      command: |
        git branch <the-right-branch-name>
        git reset --hard HEAD~<how-many>
        git switch <the-right-branch-name>
      shell: powershell
      does: >
        The same three moves as the single-checkpoint case. `HEAD~3` means three
        checkpoints back, so replace the placeholder with the count you got from
        `git log --oneline`.
      destroys: >
        The reset throws away every uncommitted edit in your folder. The
        committed work is safe because line one saved it under the new name
        first. Shelve anything unsaved with `git stash push -u` before you start.
      verify: >
        `git log --oneline -5` on the new branch shows all of the work, and the
        same command on the old branch shows none of it.
      if_it_did_not_work: >
        Count again from `git log --oneline`. If you overshot, `git reflog` has
        the position you started from and you can branch at it.

  already-pushed:
    ask: >
      Is getting the work onto the right branch enough, or does the wrong branch
      need the checkpoints taken back out too?
    branches:
      - label: Getting it in the right place is enough
        goto: copy-to-right
      - label: The wrong branch has to be cleaned up as well
        goto: revert-on-wrong

  copy-to-right:
    ask: Publish the same work under the right branch name.
    resolve:
      command: |
        git switch -c <the-right-branch-name>
        git push -u origin <the-right-branch-name>
      shell: powershell
      does: >
        Creates the correctly named branch exactly where you are, with all the
        work already on it, and sends it to GitHub. Nothing is removed from
        anywhere.
      destroys: Nothing.
      verify: >
        The push prints a link, the branch appears on the repository page, and
        `gh pr create --fill` now offers the right branch.
      if_it_did_not_work: >
        If the push is refused, see panic-cant-push, which sorts the refusal
        messages by what they say.

  revert-on-wrong:
    ask: >
      Do the copy step first if you have not, then cancel the checkpoints on the
      wrong branch.
    resolve:
      command: git revert --no-edit HEAD~<how-many>..HEAD
      shell: powershell
      does: >
        Adds new checkpoints to this branch that undo each of the last few, one
        for one. Replace the placeholder with the count. The history keeps both
        the original and the cancellation, which is what makes this safe for
        anything already published.
      destroys: >
        Nothing. It adds checkpoints rather than removing them, so nobody else's
        copy of this branch breaks.
      verify: >
        `git log --oneline -8` shows new entries starting with `Revert`, and the
        files on this branch no longer contain the work. Push normally.
      if_it_did_not_work: >
        Make sure the work exists on the right branch before you run this, since
        it removes the work from here. If git stops on a conflict, see
        panic-merge-conflict-stuck.

see_also:
  - d5-branches
  - d10-undo-everything
  - d8-pull-requests
  - panic-cant-push
  - panic-merge-conflict-stuck
  - d11-when-you-lose-work
keywords:
  - committed to main by mistake
  - wrong branch
  - move my commits
  - i was on main
  - should have branched
  - move work to a branch
---

Committing to `main` when you meant to work on a branch is the most common git mistake there
is, and it is one of the cheapest to fix. A branch is a name pointing at a checkpoint.
Moving work between branches is mostly a matter of pointing names at different places.

Two facts make the tree above make sense. Uncommitted changes are not attached to any
branch, so switching carries them with you automatically. Committed work belongs to whatever
branch names point at it, and a checkpoint can have two names at once, which is why every
route here starts by adding a name rather than removing one.

The dividing line is whether you have pushed. Before pushing, you can rearrange freely
because your machine holds the only copy. After pushing, other copies exist and the safe
move is `git revert`, which cancels a checkpoint by adding another rather than by deleting
anything.
