---
id: git-local-changes-would-be-overwritten
title: "Your local changes would be overwritten by merge"
type: error
verified: 2026-08-02
volatility: low

category: conflict

# Lists what is modified and not committed. Empty output means nothing is in
# the way any more.
verify: git status --porcelain

danger: >
  One step below uses `git checkout -- <file>`, which throws away your edits to that file
  with no undo and no recycle bin. `git stash` is the safe alternative and is listed first:
  it puts the same edits somewhere you can get them back from. Only discard when you have
  read the diff and decided the changes are worthless.

sample: |
  PS C:\Users\nyx\dev\scraper> git pull
  Updating 4f2a1c9..8b31e02
  error: Your local changes to the following files would be overwritten by merge:
          src/app.py
  Please commit your changes or stash them before you merge.
  Aborting

patterns:
  - "Your local changes to the following files would be overwritten"
  - "Please commit your changes or stash them before you"
  - "would be overwritten by merge"
  - "would be overwritten by checkout"

means: >
  You have edits in your folder that are not committed, and the incoming commits change the
  same files. Git will not silently overwrite work you have not saved anywhere, so it stops
  before doing anything. Nothing was merged, nothing was downloaded into your files, and your
  edits are exactly where you left them.

fix_ladder:
  - try: See what your uncommitted changes actually are.
    command: git diff
    shell: powershell
    why: >
      Assumes you do not know what is in the way, which is normal when an agent has been
      editing. Every option below depends on whether these changes are worth keeping, and
      this is the only way to answer that.

  - try: Commit them, then pull.
    command: 'git add .; git commit -m "wip: local changes"; git pull'
    shell: powershell
    why: >
      Assumes the changes are real work. Once committed they are safe, and git can merge
      them properly. If the same lines changed on both sides you will get a merge conflict,
      which is a normal next step rather than a setback.

  - try: Set them aside, pull, then put them back.
    command: git stash; git pull; git stash pop
    shell: powershell
    why: >
      Assumes the changes are half-finished and you would rather not commit them yet.
      `git stash` lifts them out and stores them, `git stash pop` puts them back on top of
      the freshly pulled code. If pop reports a conflict, resolve it the same way you would
      any merge conflict.

  - try: Check whether the changes are yours at all.
    command: git status
    shell: powershell
    why: >
      Assumes the modified file is something generated rather than something you wrote. A
      lockfile, a build output folder, or a file with only line-ending changes is often
      listed here. Those are safe to discard and probably belong in `.gitignore`.

  - try: Throw the local edits away and take the incoming version.
    command: git checkout -- src/app.py
    shell: powershell
    why: >
      Assumes you have read the diff and decided the edits are worthless. This is
      irreversible: the file goes back to its last committed state and your changes are
      gone. Do this only for a specific named file, never for the whole folder.

if_none_worked: >
  Paste the whole error including the indented list of file names, the output of
  `git status`, and the output of `git diff` for the files it named. The diff is what people
  leave out because it is long, and it is the only thing that answers the actual question,
  which is whether those changes are worth keeping.

see_also:
  - d3-the-three-places
  - d10-undo-everything
  - d7-merge-conflicts
  - d12-gitignore-and-what-not-to-commit

keywords:
  - local changes would be overwritten
  - commit your changes or stash them
  - git pull aborted
  - stash
  - uncommitted changes
---

This is git protecting the one category of work it cannot recover: edits that were never
committed.

Anything committed can be found again through the reflog, even after a bad reset. An
uncommitted edit exists in exactly one place, the file on your disk. Git knows this, so it
refuses to write over it and hands the decision back to you.

The habit that makes the message stop appearing is committing before you pull. Small
frequent commits mean there is rarely anything uncommitted to be in the way, and a commit
you regret is trivially undone.

Watch for one variant. If the file listed is a lockfile such as `package-lock.json` or
`Cargo.lock`, an install command changed it as a side effect and you may not have touched
it yourself. That is still a real change and still needs a decision, but the decision is
usually to discard it and let the next install regenerate it.
