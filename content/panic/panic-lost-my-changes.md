---
id: panic-lost-my-changes
title: My changes disappeared
type: panic
verified: 2026-08-02
volatility: low
symptom: >
  The code I wrote is not in the file any more, or the file has gone back to
  looking like an older version of itself.
reassurance: >
  Most of the time this is not loss at all. The work is on another branch, or in
  a stash, and you are looking at the wrong place. If it was ever in a checkpoint,
  it is still on this machine and one command brings it back. Work that was never
  committed and never stashed is the one case git cannot help with, and this tree
  reaches that answer quickly instead of slowly.
backup_first: 'Copy-Item -Path . -Destination ..\panic-backup -Recurse'
root: which-branch
nodes:
  which-branch:
    ask: Are you on the same branch you were on when you did the work?
    how_to_tell: git branch --show-current
    branches:
      - label: Yes, it printed the name I expected
        goto: ever-committed
      - label: No, it printed a different name
        goto: switch-back
      - label: It printed nothing at all
        goto: not-on-a-branch

  switch-back:
    ask: Go back to the branch you were working on and look again.
    resolve:
      command: git switch <the-branch-you-were-on>
      shell: powershell
      does: >
        Moves you onto that branch. Your files change to match it, so work that
        lives there reappears.
      destroys: >
        Nothing. If you have uncommitted edits here, git carries them across with
        you, or refuses and names the files in the way.
      verify: >
        Open the file. If the code is back, you are done. `git branch
        --show-current` confirms where you are standing.
      if_it_did_not_work: >
        If you do not know the name, `git branch -a` lists every branch. If the
        work still is not there, come back and answer yes to the first question.

  not-on-a-branch:
    ask: >
      You are parked on a single point in the history rather than standing on a
      branch. Did you save any checkpoints while you were parked there?
    how_to_tell: git log --oneline -5
    branches:
      - label: No, the top line is older work I recognize
        goto: go-back-to-branch
      - label: Yes, the top line is something I did just now
        goto: keep-detached-work

  go-back-to-branch:
    ask: Put yourself back on the branch you were on before.
    resolve:
      command: git switch -
      shell: powershell
      does: >
        Returns you to whatever branch you were on last. The dash means the
        previous one.
      destroys: Nothing.
      verify: >
        `git branch --show-current` prints a branch name instead of nothing, and
        `git status` starts with `On branch`.
      if_it_did_not_work: >
        If the dash form fails, name the branch: `git switch main`. If git says
        local changes would be overwritten, you have work here after all, so
        answer yes to the previous question instead.

  keep-detached-work:
    ask: Give that work a name before you go anywhere else.
    resolve:
      command: git switch -c rescue-work
      shell: powershell
      does: >
        Creates a branch exactly where you are standing and moves you onto it, so
        those checkpoints belong to something instead of floating unattached.
      destroys: Nothing.
      verify: >
        `git branch --show-current` prints `rescue-work`, and `git log --oneline
        -5` still shows your checkpoints at the top.
      if_it_did_not_work: >
        If you already switched away before doing this, the work is still
        recoverable. Run `git reflog` and see d11-when-you-lose-work.

  ever-committed:
    ask: >
      Did you ever save a checkpoint that included this work? A checkpoint is a
      commit.
    how_to_tell: git log --oneline --all -30
    branches:
      - label: Yes, and I can see it in that list
        goto: rescue-from-log
      - label: Yes, but it is not in that list
        goto: reflog-hunt
      - label: No, I never saved one
        goto: ever-stashed
      - label: I do not know
        goto: reflog-hunt

  rescue-from-log:
    ask: Make a branch at that checkpoint so the work is yours again.
    resolve:
      command: git switch -c rescue-work <the-hash-from-the-left-column>
      shell: powershell
      does: >
        Creates a branch sitting exactly at that checkpoint and moves you onto
        it. Your files become that version.
      destroys: >
        Nothing. The branch you were on is untouched and still there, so a wrong
        guess costs you nothing.
      verify: >
        `git log --oneline -3` shows your work at the top and the file has the
        code in it.
      if_it_did_not_work: >
        If git complains that local changes are in the way, shelve them first
        with `git stash push -u`, then run the command again.

  reflog-hunt:
    ask: >
      Git keeps a private log of every position you have been in. Does any line
      in it describe the moment before the work vanished?
    how_to_tell: git reflog -30
    branches:
      - label: Yes, one line looks right
        goto: rescue-from-reflog
      - label: No, nothing in the list matches
        goto: fsck-hunt

  rescue-from-reflog:
    ask: Take the hash from that line and make a branch at it.
    resolve:
      command: git switch -c rescue-work <the-hash-from-the-left-column>
      shell: powershell
      does: >
        Creates a branch at the position that line describes and moves you onto
        it. Every reflog entry is a real point in the history you can return to.
      destroys: Nothing. Nothing else in the repository changes.
      verify: >
        `git log --oneline -5` shows the history you expected and the file has
        your code in it.
      if_it_did_not_work: >
        Pick a different line and run it again with a different branch name. You
        can make as many attempts as you like; each one is free.

  fsck-hunt:
    ask: Ask git to list every checkpoint it is holding that no branch points at.
    resolve:
      command: git fsck --lost-found
      shell: powershell
      does: >
        Walks the whole repository and prints anything unreferenced, as lines
        reading `dangling commit` followed by a hash. Check each one with `git
        show <hash>` until you find your work.
      destroys: Nothing. It only reads.
      verify: >
        When `git show <hash>` prints your code, keep it with `git switch -c
        rescue-work <hash>`.
      if_it_did_not_work: >
        The output is noisy and includes dangling blobs, which are partial saves
        rather than checkpoints. Read past those. If nothing dangling holds your
        work, it was never committed, so answer the stash question next.

  ever-stashed:
    ask: Was the work shelved at any point, by you or by an agent?
    how_to_tell: git stash list
    branches:
      - label: At least one line printed
        goto: stash-restore
      - label: Nothing printed
        goto: never-in-git

  stash-restore:
    ask: Bring the shelved work back into your files.
    resolve:
      command: git stash apply
      shell: powershell
      does: >
        Copies the most recently shelved work back into your files and keeps the
        shelved copy as a safety net.
      destroys: >
        Nothing. `apply` keeps the stash entry. `pop` would do the same thing and
        then delete the entry.
      verify: >
        Open the file. `git status` now lists the changes as modified.
      if_it_did_not_work: >
        For an older entry, name it with quotes, which PowerShell needs: `git
        stash apply "stash@{2}"`. If it reports a conflict, both versions are
        still safe and the stash entry still exists. See d7-merge-conflicts.

  never-in-git:
    ask: >
      Git has no copy, because the work was never committed and never shelved.
      Check your editor's own history instead.
    resolve:
      command: code .
      shell: powershell
      does: >
        Opens this folder in Visual Studio Code. Open the file, then open the
        Timeline section at the bottom of the Explorer panel. It lists every save
        the editor made, independently of git, and you can view or restore any of
        them.
      destroys: Nothing.
      verify: >
        If a list of timestamped saves appears under Timeline, one of them holds
        your work. Click one to view it.
      if_it_did_not_work: >
        If `code` is not recognized, Visual Studio Code is not installed or the
        terminal has not reloaded its PATH. JetBrains editors have the same
        feature under Local History. If the file was deleted rather than
        overwritten, open the Recycle Bin with `Start-Process
        shell:RecycleBinFolder`. If none of those has it, the work is gone, and
        d11-when-you-lose-work explains the habit that prevents a repeat.

see_also:
  - d11-when-you-lose-work
  - d10-undo-everything
  - panic-wrong-branch
  - panic-detached-head
  - d3-the-three-places
  - d5-branches
keywords:
  - my code is gone
  - changes disappeared
  - work vanished
  - file reverted itself
  - where did my code go
  - the agent deleted my work
  - lost everything
---

Three causes account for almost every case of this, and only one of them is real loss.

The work is on a different branch. Switching branches rewrites the files in your folder to
match wherever you switched to, so work that lives elsewhere looks deleted. It is not.

The work is in a stash. Shelving takes the changes out of your files and stores them, which
produces exactly the same appearance. Agents shelve without much announcement.

The work was thrown away by a sharp command, usually `git restore` or a reset. If it had
been committed even once, git still holds it and the tree above walks it back. If it never
was, the tree gets you to the editor and Windows fallbacks quickly, and to the honest answer
if those fail.
