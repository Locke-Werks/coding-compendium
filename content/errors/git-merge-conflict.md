---
id: git-merge-conflict
title: "CONFLICT (content): Merge conflict in <file>"
type: error
verified: 2026-08-02
volatility: low

category: conflict

# Searches every tracked file for a leftover conflict marker. No output means
# none are left. Run it before you commit the resolution.
verify: git grep -n "<<<<<<<"

danger: >
  `git merge --abort` throws away every conflict resolution you have made since the merge
  started and returns the folder to how it looked before. `git checkout --ours <file>`
  discards the other side's version of that file completely. Both are recoverable only
  from commits that already exist, so do not run either with uncommitted work you care
  about sitting in the same file.

sample: |
  PS C:\Users\nyx\dev\scraper> git merge feature/login
  Auto-merging src/app.py
  CONFLICT (content): Merge conflict in src/app.py
  Automatic merge failed; fix conflicts and then commit the result.

patterns:
  - 'CONFLICT \(content\): Merge conflict in'
  - "Automatic merge failed; fix conflicts"
  - "fix conflicts and then commit the result"
  - "Unmerged paths"

means: >
  Two commits changed the same lines of the same file, and git will not guess which version
  wins. Everywhere the two sides changed different parts of a file, git already merged them
  silently. It stopped only where they overlap. The merge is now half done: some files are
  merged and staged, the conflicted ones are sitting in your folder with both versions
  written into them.

fix_ladder:
  - try: List which files are actually conflicted.
    command: git status
    shell: powershell
    why: >
      Assumes you need the scope before you touch anything. The files under "Unmerged paths"
      are the only ones needing your attention. Everything else in the merge is done, and
      the list is usually shorter than the wall of output suggests.

  - try: Open each conflicted file and pick what the final version should say.
    why: >
      Assumes you can read the two versions and decide. Git writes both into the file
      between `<<<<<<<`, `=======`, and `>>>>>>>` markers. Delete the markers along with
      whichever text is wrong, leaving the file as you want it to read. See d7-merge-conflicts
      for a walkthrough of the markers.

  - try: Mark each file resolved, then finish the merge.
    command: git add src/app.py; git commit
    shell: powershell
    why: >
      Assumes the files now read correctly. `git add` on a conflicted file is how you tell
      git you are done with it. The commit needs no message: git fills in a merge message
      already.

  - try: Take one side of a file wholesale.
    command: git checkout --ours src/app.py
    shell: powershell
    why: >
      Assumes one version is correct and the other is disposable, which is common
      for generated files such as a lockfile. `--ours` is the branch you were on, `--theirs`
      is the branch you are merging in. Run `git add` on the file afterward.

  - try: Undo the whole merge and go back to before it started.
    command: git merge --abort
    shell: powershell
    why: >
      Assumes you want out. This always works while a merge is in progress and returns
      every file to its pre-merge state. Nothing about the merge is permanent until you
      commit, so this is the escape hatch and it is worth knowing before you need it.

if_none_worked: >
  Paste the output of `git status`, the full contents of one conflicted file including every
  marker line, and what the file is supposed to do. The markers are what people strip before
  pasting because they look like junk. They are the entire structure of the problem, and
  without them an agent cannot tell which side is which.

see_also:
  - d7-merge-conflicts
  - d6-merge-and-rebase
  - d10-undo-everything
  - d9-reading-a-diff

keywords:
  - merge conflict
  - conflict markers
  - unmerged paths
  - automatic merge failed
  - fix conflicts
---

A conflict is not a failure. It is git declining to make a decision that only you can make.

The reason it stops instead of picking is that both versions are valid code. Git compares
text. It has no idea that one version calls a function that no longer exists, and that is
exactly the kind of thing a wrong pick produces.

Two habits keep conflicts small. Commit often, because a conflict spans however much work
sits between the two sides. And pull before you start editing, because the conflicts you
get are the ones you built on top of stale code.

Handing the file to an agent works well here, with one condition. Give it the whole file
with the markers intact and tell it what the code is supposed to do. An agent handed
marker-free fragments will produce something that compiles and quietly drops one side's
change.
