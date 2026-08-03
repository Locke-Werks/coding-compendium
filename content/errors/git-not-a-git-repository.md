---
id: git-not-a-git-repository
title: "fatal: not a git repository"
type: error
verified: 2026-08-02
volatility: low

category: not-found

# Prints the top folder of the repository you are inside. If it errors, you are
# not inside one.
verify: git rev-parse --show-toplevel

sample: |
  PS C:\Users\nyx\dev> git status
  fatal: not a git repository (or any of the parent directories): .git

patterns:
  - "fatal: not a git repository"
  - "or any of the parent directories"
  - "not a git repository"

means: >
  Git works out which project you mean by looking for a folder named `.git` in the folder
  you are standing in, then in its parent, then in its parent, all the way to the top of
  the drive. It found none. So either you are outside the project folder, or this folder
  was never made into a repository, or the `.git` folder was deleted. Git is not confused
  about your files. It cannot find the project's history.

fix_ladder:
  - try: Print the folder you are standing in.
    command: Get-Location
    shell: powershell
    why: >
      Assumes the repository exists and you are one folder away from it. A terminal opened
      from the Start menu begins at `C:\Users\<yourname>`, and an agent that ran a `cd`
      earlier in the session is still wherever it went.

  - try: Move into the project folder and run the command again.
    command: Set-Location C:\Users\<yourname>\dev\<project>
    shell: powershell
    why: >
      Assumes you now know where the project is. Every git command has to be run from
      inside the project folder or one of its subfolders. There is no global git.

  - try: Look for the hidden `.git` folder.
    command: Get-ChildItem -Force -Filter .git
    shell: powershell
    why: >
      Assumes you are in the right folder and something is wrong with the repository
      itself. `.git` is hidden, so a plain `Get-ChildItem` will not show it and neither
      will File Explorer with default settings. `-Force` includes hidden items.

  - try: Turn this folder into a repository, if it never was one.
    command: git init
    shell: powershell
    why: >
      Assumes the files came first and version control was never set up. This happens
      constantly with folders an agent created. `git init` creates the `.git` folder and
      changes none of your files.

  - try: Clone the project from GitHub instead.
    command: git clone https://github.com/nyxlocke/<repo>.git
    shell: powershell
    why: >
      Assumes the real repository is on GitHub and this folder is a stray copy of the files
      with no history attached. Cloning gives you the files and the history together, in a
      new folder named after the repository.

if_none_worked: >
  Paste the error, the output of `Get-Location`, and the output of `Get-ChildItem -Force`
  in that folder. The hidden-file listing is the piece people trim, and it is the only way
  to tell "wrong folder" apart from "right folder, missing `.git`", which need completely
  different answers.

see_also:
  - d2-repo-remote-clone-origin
  - d1-what-git-actually-stores
  - c7-files-folders-and-paths

keywords:
  - not a git repository
  - fatal git
  - git init
  - no .git folder
  - wrong directory git
---

Git has no idea which project you mean beyond the folder you are standing in. That is the
whole mechanism, and it explains everything about this message.

Two situations produce it more than any others. You opened a fresh terminal and it started
in your home folder rather than in the project. Or an agent created a folder full of files,
you assumed it was a repository because it looked like one, and nobody ever ran `git init`.

There is a third case worth knowing. If you are one level too high, say in `dev` rather
than in `dev\scraper`, git will not find the repository below you. It only looks upward,
never downward. Moving down one folder fixes it.
