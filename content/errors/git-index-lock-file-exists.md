---
id: git-index-lock-file-exists
title: "Unable to create '.git/index.lock': File exists"
type: error
verified: 2026-08-02
volatility: low

category: conflict

# Prints True while the lock file is still there and False once it is gone.
verify: Test-Path .git\index.lock

danger: >
  Deleting `.git\index.lock` while a git command is genuinely still running can leave the
  staging area half written, which means running `git add` again afterward. It never touches
  your commits or your files. Check that no git process is running before you delete it, and
  never delete anything else inside `.git`.

sample: |
  PS C:\Users\nyx\dev\scraper> git add .
  fatal: Unable to create 'C:/Users/nyx/dev/scraper/.git/index.lock': File exists.

  Another git process seems to be running in this repository, e.g.
  an editor opened by 'git commit'. Please make sure all processes
  are terminated then try again. If it still fails, a git process
  may have crashed in this repository and be removed manually.

patterns:
  - "index.lock"
  - "Another git process seems to be running"
  - "File exists"

means: >
  Git creates a lock file before it changes the staging area, so that two commands cannot
  write to it at once, and deletes the lock when it finishes. This lock is still there. Either
  another git command really is running right now, or one was interrupted or crashed and
  never got to clean up. Your commits and your files are untouched either way.

fix_ladder:
  - try: Wait a few seconds and run the command again.
    why: >
      Assumes a real git command is in progress. A large `git add` or a fetch on a big
      repository holds the lock for a while, and your editor's git integration runs commands
      you never asked for in the background. This costs nothing to rule out.

  - try: Check whether any git process is actually alive.
    command: Get-Process git -ErrorAction SilentlyContinue
    shell: powershell
    why: >
      Assumes you cannot tell whether something is running. No output means no git process
      exists, which means the lock is stale and safe to delete. Output means wait, or find
      out what started it.

  - try: Delete the stale lock file.
    command: Remove-Item .git\index.lock
    shell: powershell
    why: >
      Assumes the lock is left over from a crash or an interrupted command. This is the fix
      the error message itself recommends. Deleting it affects nothing but the lock.

  - try: Look for an editor waiting for a commit message.
    why: >
      Assumes `git commit` without `-m` opened an editor and is sitting there waiting. Git
      holds the lock the entire time that editor is open. Find the window, save and close it,
      or press Ctrl+C in the terminal running git.

  - try: Check whether your editor is running git in the background.
    why: >
      Assumes an extension is the second process. Editors refresh git status on every file
      save, and a large repository plus an aggressive extension produces this collision
      repeatedly. Closing the editor while you work in the terminal settles it.

if_none_worked: >
  Paste the whole error including the paragraph about another git process, the command you
  ran, and the output of `Get-Process git`. The process listing is what people leave out and
  it is the one thing that distinguishes a stale lock, which you delete, from a live command,
  which you wait for.

see_also:
  - d3-the-three-places
  - c5-processes-and-killing-them
  - d4-commit-well

keywords:
  - index.lock
  - another git process
  - git add fails
  - stale lock
  - git crashed
---

The lock is a single empty file, and it is the whole mechanism. Git creates
`.git\index.lock`, does its work, renames or removes it. If the process dies in the middle,
the file stays and every later command refuses to start.

Windows makes this more common than it is elsewhere. Antivirus can hold the file open for a
moment after git tries to remove it, and Ctrl+C during a large `git add` leaves the lock
behind more often than it should.

Deleting the lock is safe and unglamorous. It is not a repository repair, and it does not
touch commits, branches, or the files in your folder. If the same lock keeps reappearing
within seconds of being deleted, something really is running git in a loop, and that is your
editor.
