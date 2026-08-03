---
id: git-remote-origin-already-exists
title: "fatal: remote origin already exists"
type: error
verified: 2026-08-02
volatility: low

category: config

# Lists every remote with its address, twice each: once for fetching and once
# for pushing.
verify: git remote -v

sample: |
  PS C:\Users\nyx\dev\scraper> git remote add origin https://github.com/nyxlocke/scraper.git
  error: remote origin already exists.

patterns:
  - "remote origin already exists"
  - "remote .* already exists"

means: >
  This repository already has a remote nicknamed `origin`, and `git remote add` refuses to
  overwrite one that exists. A remote is a nickname for an address where a copy of the
  project lives. `origin` is only the conventional name for the first one, not a reserved
  word. Nothing changed. The existing address is still whatever it was.

fix_ladder:
  - try: Look at what origin currently points at.
    command: git remote -v
    shell: powershell
    why: >
      Assumes the existing remote might already be correct, which it often is when you are
      following a setup guide a second time. If the address matches what you were about to
      add, there is nothing to do.

  - try: Point origin at the new address instead of adding it.
    command: git remote set-url origin https://github.com/nyxlocke/<repo>.git
    shell: powershell
    why: >
      Assumes the existing address is wrong or outdated. `set-url` replaces the address on a
      remote that already exists, which is the command you actually wanted. It changes no
      files and no commits.

  - try: Remove the old remote and add it again.
    command: git remote remove origin; git remote add origin https://github.com/nyxlocke/<repo>.git
    shell: powershell
    why: >
      Assumes you want a clean slate. This is equivalent to `set-url` with more steps.
      Removing a remote deletes nothing except the nickname and the branches git had cached
      from it.

  - try: Add it under a different name if you genuinely want two.
    command: git remote add upstream https://github.com/<other-user>/<repo>.git
    shell: powershell
    why: >
      Assumes you are working with a fork and need both. The convention is `origin` for your
      copy and `upstream` for the one you forked from. Both can exist at once and git keeps
      them separate.

  - try: Confirm the push target after any change.
    command: git remote -v; git status -sb
    shell: powershell
    why: >
      Assumes you want to know it took effect before you push. The first command shows the
      address, the second shows which remote branch your current branch tracks. Those are
      two different settings and changing one does not change the other.

if_none_worked: >
  Paste the error, the command you ran in full, and the complete output of `git remote -v`.
  The existing address is the part people leave out, and the entire question is whether the
  remote that is already there is the one you want.

see_also:
  - d2-repo-remote-clone-origin
  - b4-github-and-gh
  - d8-pull-requests

keywords:
  - remote origin already exists
  - git remote add failed
  - set-url
  - change remote
  - wrong remote url
---

You hit this by following setup instructions on a repository that was already set up. A
cloned repository has `origin` configured from the moment it lands on your disk, so every
"now add the remote" step in a tutorial fails against a clone.

`git remote set-url` is the command that does what people mean here. Add creates, set-url
changes, remove deletes. Nothing about any of them touches your files or your history.

Worth checking while you are in there: an address ending in `.git` that starts with
`https://` uses a token or the browser sign-in, and one starting `git@github.com:` uses an
SSH (Secure Shell) key. Switching between the two is exactly a `set-url` away, and it is
the fastest workaround when authentication is failing on one of them.
