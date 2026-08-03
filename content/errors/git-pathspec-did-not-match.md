---
id: git-pathspec-did-not-match
title: "error: pathspec 'x' did not match any file(s) known to git"
type: error
verified: 2026-08-02
volatility: low

category: not-found

# Lists every branch, local and remote. The name you are trying to switch to
# has to appear here.
verify: git branch -a

sample: |
  PS C:\Users\nyx\dev\scraper> git checkout feature/login
  error: pathspec 'feature/login' did not match any file(s) known to git

patterns:
  - "did not match any file"
  - "pathspec"

means: >
  Git took the word you gave it and looked for a branch with that name, then for a file with
  that name, and found neither. "Pathspec" is git's word for the thing you named. The branch
  probably exists on GitHub but not yet on your machine, or the name is spelled differently,
  or the file was never added to git.

fix_ladder:
  - try: List every branch git knows about, including remote ones.
    command: git branch -a
    shell: powershell
    why: >
      Assumes the branch exists somewhere and you need the exact name. Remote branches appear
      with a `remotes/origin/` prefix. Seeing `remotes/origin/feature/login` with no local
      counterpart means the branch is real and you simply do not have a local copy yet.

  - try: Download the current list of branches from GitHub.
    command: git fetch
    shell: powershell
    why: >
      Assumes the branch was created after your last fetch. Git does not check GitHub on its
      own, so a branch pushed by an agent, by a pull request, or from another machine is
      invisible until you fetch. This changes no files.

  - try: Create a local branch that follows the remote one.
    command: git switch <branch-name>
    shell: powershell
    why: >
      Assumes the branch now shows up under `remotes/origin/`. Modern git sees that a remote
      branch of that name exists, creates a local one to match, and connects them. This is
      what you wanted `checkout` to do.

  - try: Check the name character by character.
    command: git branch -a | Select-String <partial-name>
    shell: powershell
    why: >
      Assumes a spelling problem. Branch names are case sensitive, and slashes matter:
      `feature/login` and `feature-login` are different branches. Searching for a fragment
      finds it without you having to read the whole list.

  - try: If you meant a file rather than a branch, check that git tracks it.
    command: git ls-files | Select-String <filename>
    shell: powershell
    why: >
      Assumes you ran something like `git checkout -- <file>` or `git restore <file>` on a
      file git has never seen. A brand new file that was never `git add`ed is unknown to git,
      so there is no committed version of it to restore.

if_none_worked: >
  Paste the error, the exact command, and the complete output of `git branch -a`. The full
  branch list is what people summarize as "the branch definitely exists", and it is the only
  way to see whether it exists locally, only on the remote, or under a slightly different name.

see_also:
  - d5-branches
  - d3-the-three-places
  - d2-repo-remote-clone-origin

keywords:
  - pathspec did not match
  - branch not found
  - checkout failed
  - did not match any files known to git
  - git switch branch
---

The wording is unhelpful because one message covers two jobs. `git checkout` switches
branches and also restores files, so when it cannot find what you named it does not know
which of the two you meant and reports both possibilities at once.

This is a good argument for the newer commands. `git switch` only changes branches and
`git restore` only restores files, so each gives you an error about the thing you were
actually doing.

The most common cause by far is a branch that exists only on GitHub. Someone else pushed
it, or an agent pushed it from another folder, or a pull request created it. Your machine
has no idea it exists until you run `git fetch`.
