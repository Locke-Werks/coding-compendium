---
id: git-detached-head
title: "You are in 'detached HEAD' state"
type: error
verified: 2026-08-02
volatility: low

category: config

# Prints the branch name when you are on a branch, and prints nothing at all
# when HEAD is detached. Silence is the answer.
verify: git branch --show-current

sample: |
  PS C:\Users\nyx\dev\scraper> git checkout 4f2a1c9
  Note: switching to '4f2a1c9'.

  You are in 'detached HEAD' state. You can look around, make experimental
  changes and commit them, and you can discard any commits you make in this
  state without impacting any branches by switching back to a branch.

  HEAD is now at 4f2a1c9 fix: handle empty response body

patterns:
  - "detached HEAD"
  - "HEAD is now at"
  - "You can look around, make experimental"

means: >
  HEAD is git's word for "where you are standing in the history". Normally it points at a
  branch name, and the branch moves forward as you commit. Right now it points straight at
  one specific commit with no branch attached. You can look at anything and edit anything.
  Any commit you make here belongs to no branch, so nothing points at it, and switching
  away leaves it with nothing to find it by.

fix_ladder:
  - try: Go back to the branch you came from.
    command: git switch -
    shell: powershell
    why: >
      Assumes you checked out an old commit to look at something and made no commits. That
      is the common case and there is nothing to save. The dash means "the branch I was on
      before", the same way `cd -` works in a shell.

  - try: Check whether you made any commits while detached.
    command: git log --oneline -5
    shell: powershell
    why: >
      Assumes you have been working for a while and are not sure. If the top commit is one
      of yours rather than the old one you checked out, do not switch away until you have
      done the next step.

  - try: Give the work a branch name so it stops floating.
    command: git switch -c <branch-name>
    shell: powershell
    why: >
      Assumes you committed here and want to keep it. This creates a branch at exactly the
      commit you are on and attaches HEAD to it. Nothing moves and nothing is lost.
      `-c` means create.

  - try: Recover commits you already left behind.
    command: git reflog
    shell: powershell
    why: >
      Assumes you already switched away and the commits vanished from the log. Reflog is
      git's private list of every place HEAD has been for the last ninety days. Find the
      commit, copy its short id, and run `git switch -c <branch-name> <id>`.

  - try: Work out how you got here, so it stops happening.
    command: git reflog -5
    shell: powershell
    why: >
      Assumes something put you here without you noticing. The usual causes are checking
      out a commit id or a tag rather than a branch, a `git bisect` session left running,
      or a submodule, which is always detached by design.

if_none_worked: >
  Paste the full message including the `HEAD is now at` line, the output of `git reflog -10`,
  and the output of `git log --oneline -5`. The reflog is the piece nobody includes and it is
  the complete record of how you arrived and what you might have left behind.

see_also:
  - d5-branches
  - d11-when-you-lose-work
  - d10-undo-everything
  - d1-what-git-actually-stores

keywords:
  - detached head
  - HEAD is now at
  - not on any branch
  - lost commits
  - git switch back
---

The message reads like a warning and is mostly a description. Nothing is broken. You are
standing on a commit rather than on a branch.

Branches are the handles you pick history up by. A commit with no branch pointing at it
still exists, but the only way back to it is its 40-character id or the reflog. Git even
deletes unreachable commits eventually during its own cleanup, though not for months.

The thing that makes this dangerous is how quiet it is. You can commit repeatedly in
detached HEAD and git will let you, with a small note each time. The work looks saved,
because it is saved. It just has no name.

If you deliberately came here to look at an old version of a file, `git switch -` gets you
back the moment you are done and costs nothing.
