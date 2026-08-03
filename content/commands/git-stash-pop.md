---
id: git-stash-pop
title: git stash pop
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git stash pop
shell: any

does: >
  Puts the most recently stashed changes back into your working folder and deletes that
  stash entry.

flags:
  - flag: "stash@{<number>}"
    means: >
      Pops a specific stash instead of the newest one, as in `git stash pop stash@{2}`. Get
      the numbers from `git stash list`. In PowerShell the curly braces are fine unquoted,
      but quoting the whole argument avoids any argument.
  - flag: "apply"
    means: >
      `git stash apply` does the same restore but keeps the stash entry. Use it when you
      want to put the same changes onto two different branches, or when you want a safety
      copy until you are sure.
  - flag: "drop"
    means: >
      `git stash drop` deletes a stash entry without restoring it. That discards those
      changes, so read them with `git stash show -p` first.

expect: >
  The output of `git status` showing your restored changes, then a final line such as
  `Dropped refs/stash@{0} (a9f3c21...)`. That last line confirms the entry was removed.

see_also:
  - git-stash
  - git-stash-list
  - git-status
  - d10-undo-everything

keywords:
  - get my stash back
  - restore stashed changes
  - unstash
  - pop stash
---

If the branch changed underneath you, popping can produce a merge conflict. When that
happens, git keeps the stash entry rather than dropping it, so your work is still safe while
you resolve the conflict markers.

Prefer `git stash apply` when you are nervous. It leaves the original in place, and you can
clear it later with `git stash drop`.
