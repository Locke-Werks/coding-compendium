---
id: git-nothing-to-commit-working-tree-clean
title: "nothing to commit, working tree clean"
type: error
verified: 2026-08-02
volatility: low

category: not-found

# The short form of status. Any output at all means there is something to
# commit. Silence means git genuinely sees no changes.
verify: git status --porcelain

sample: |
  PS C:\Users\nyx\dev\scraper> git commit -m "feat: add retry logic"
  On branch main
  nothing to commit, working tree clean

patterns:
  - "nothing to commit, working tree clean"
  - "nothing to commit, working directory clean"
  - "nothing added to commit but untracked files present"

means: >
  Git compared your folder against the last commit and found no difference. Every file it
  tracks matches what was already saved. The usual explanations are that the commit already
  happened, that you are in a different folder from the one you edited, or that the files
  you changed are not tracked by git at all because they match a rule in `.gitignore`.

fix_ladder:
  - try: Check whether the commit already happened.
    command: git log --oneline -3
    shell: powershell
    why: >
      Assumes you or an agent already committed this work. Agents commit as they go, so the
      message you are trying to write may describe a commit that is sitting at the top of
      the log. Nothing is wrong in that case.

  - try: Confirm you are in the folder you edited.
    command: Get-Location; git status
    shell: powershell
    why: >
      Assumes there are two copies of the project on this machine, or that the terminal is
      in a different folder from your editor. This costs one command and rules out an
      annoying category of confusion.

  - try: Check whether the file is being ignored.
    command: git check-ignore -v <path/to/file>
    shell: powershell
    why: >
      Assumes the file changed and git is refusing to look at it. This prints the exact
      `.gitignore` line responsible, or nothing at all if the file is not ignored. A broad
      rule such as `*.log` or `dist/` catching a file you actually want is common.

  - try: Look for untracked files that git can see but is not tracking.
    command: git status --untracked-files=all
    shell: powershell
    why: >
      Assumes the files are new rather than modified. Brand new files are untracked until
      you run `git add` on them, and git summarizes whole untracked folders as one line by
      default, which hides how much is in there.

  - try: Check whether the change was only to line endings.
    command: git diff --stat
    shell: powershell
    why: >
      Assumes a tool rewrote the file with different line endings and git is normalizing
      them back. On Windows this is a real cause of "I definitely saved that file". See
      c8-line-endings-and-encoding.

if_none_worked: >
  Paste the message, the output of `git status --untracked-files=all`, the output of
  `Get-Location`, and the full path of the file you edited. The path is what people leave
  out because it seems obvious, and it settles the two-copies-of-the-project case in one
  look.

see_also:
  - d3-the-three-places
  - d12-gitignore-and-what-not-to-commit
  - d4-commit-well
  - c8-line-endings-and-encoding

keywords:
  - nothing to commit
  - working tree clean
  - git sees no changes
  - untracked files
  - gitignore hiding file
---

This one is confusing because it is not an error and it does not sound like a status report.
Git is telling you the folder matches the last commit exactly.

The most common cause is the least interesting: the commit already happened. Agents commit
frequently and often without announcing it clearly, so the work you are trying to save is
already saved.

The most annoying cause is `.gitignore`. A rule written to keep build output out of the
repository can catch a source file too, and git then treats your edits as invisible.
`git check-ignore -v` names the offending line directly, which beats reading the file and
guessing.

The related message "nothing added to commit but untracked files present" means something
different and is worth reading carefully. Git sees new files and you have not run `git add`
on them, so they are not going into the commit.
