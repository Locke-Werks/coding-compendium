---
id: panic-deleted-a-file
title: A file I need is gone
type: panic
verified: 2026-08-02
volatility: low
symptom: >
  A file that was there this morning is not in the folder any more, and I did not
  knowingly delete it.
reassurance: >
  If the file was ever in a checkpoint, git holds every version of it and getting
  it back is one command. If it was never committed, git has nothing, but Windows
  and your editor both keep copies of their own and one of them usually has it.
  Deleting a file does not remove it from the history, which is the same property
  that makes leaked secrets so hard to erase and file recovery so easy.
backup_first: 'Copy-Item -Path . -Destination ..\panic-backup -Recurse'
root: ever-committed
nodes:
  ever-committed:
    ask: Was that file ever saved in a checkpoint?
    how_to_tell: git log --oneline --all -- "<path/to/the/file>"
    branches:
      - label: Yes, lines printed
        goto: deletion-saved
      - label: Nothing printed
        goto: find-path
      - label: I do not know the exact path to type
        goto: find-path

  find-path:
    ask: Ask git which files it has deleted recently and look for yours.
    how_to_tell: git log --diff-filter=D --name-only --oneline -20
    branches:
      - label: I found it in that list
        goto: restore-from-history
      - label: It is not in the list and git never knew about it
        goto: never-in-git

  deletion-saved:
    ask: Have you saved a checkpoint since the file disappeared?
    how_to_tell: git status
    branches:
      - label: No, git still lists the file as deleted
        goto: restore-now
      - label: Yes, the deletion is already saved
        goto: restore-from-history
      - label: I do not know
        goto: restore-now

  restore-now:
    ask: Put the file back from your last checkpoint.
    resolve:
      command: git restore "<path/to/the/file>"
      shell: powershell
      does: >
        Writes the file back to disk exactly as it was at your last checkpoint.
      destroys: >
        Any uncommitted edits to that one file. The file is currently missing, so
        there are none.
      verify: >
        The file exists again and `git status` no longer lists it as deleted.
      if_it_did_not_work: >
        If git says the path did not match anything, the deletion is already
        saved in a checkpoint. Go back and pick the other answer.

  restore-from-history:
    ask: Find the checkpoint that removed it and take the file from the one before.
    resolve:
      command: |
        git log --diff-filter=D --oneline -- "<path/to/the/file>"
        git restore --source=<that-hash>~1 -- "<path/to/the/file>"
      shell: powershell
      does: >
        The first line prints the checkpoint that deleted the file. The second
        takes the file's contents from the checkpoint immediately before it, the
        `~1`, and writes them back to disk.
      destroys: >
        Nothing. It writes one file and changes no history at all.
      verify: >
        The file is back. `git status` lists it as new, so `git add` and commit it
        to keep it.
      if_it_did_not_work: >
        If `git restore` says it does not recognize the path, use the older
        spelling: `git checkout <that-hash>~1 -- "<path/to/the/file>"`. If the
        file moved rather than vanished, `git log --oneline --follow --
        "<path>"` tracks it through renames.

  never-in-git:
    ask: >
      Git has no copy, so the file was never committed. Look where Windows keeps
      deleted files.
    resolve:
      command: Start-Process shell:RecycleBinFolder
      shell: powershell
      does: >
        Opens the Recycle Bin. A file deleted through File Explorer is usually
        still there and restores with a right-click.
      destroys: Nothing.
      verify: >
        The file is back in its folder. Commit it this time, so this tree is one
        step long if it happens again.
      if_it_did_not_work: >
        Try your editor's own history. In Visual Studio Code, open the folder
        with `code .` and use the Timeline section under the Explorer panel,
        which records every save independently of git. If the folder sits inside
        OneDrive, the website offers Version history on the parent folder.
        d11-when-you-lose-work covers the rest.

see_also:
  - d10-undo-everything
  - d11-when-you-lose-work
  - d13-tags-releases-and-history
  - panic-lost-my-changes
  - d3-the-three-places
  - d12-gitignore-and-what-not-to-commit
keywords:
  - deleted a file
  - file is missing
  - restore a deleted file
  - the agent deleted a file
  - get my file back
  - undelete
  - file disappeared from folder
---

Git treats a deleted file as an ordinary change, so a deletion sits in `git status`
alongside your edits until you commit it. That gives you two very different situations, and
the first question in the tree sorts them.

If you have not committed since the deletion, the file is one command away and nothing else
is involved. If the deletion is already in a checkpoint, the file is still in the history
and you pull it forward from the checkpoint before the one that removed it.

The case git cannot help with is a file it never tracked: something new that was created and
deleted between commits, or a file covered by `.gitignore`. Those never entered the history,
so no git command produces them. The Recycle Bin and your editor's own save history are the
two places worth checking, in that order.

One thing to check before assuming deletion at all: `git status` shows a file as deleted
only if git knew about it. If the file is not listed anywhere and the code still runs, it may
have been moved rather than removed. `git log --diff-filter=D` in the tree above lists
genuine deletions only.
