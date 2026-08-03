---
id: git-refusing-to-merge-unrelated-histories
title: "fatal: refusing to merge unrelated histories"
type: error
verified: 2026-08-02
volatility: low

category: conflict

# After the merge, the graph shows two lines of history joining at the top.
verify: git log --oneline --graph -10

sample: |
  PS C:\Users\nyx\dev\scraper> git pull origin main
  From https://github.com/nyxlocke/scraper
   * branch            main       -> FETCH_HEAD
  fatal: refusing to merge unrelated histories

patterns:
  - "refusing to merge unrelated histories"
  - "unrelated histories"

means: >
  The two histories have no commit in common, not even their first one. Git treats that as a
  sign you are about to merge the wrong project into this one, so it stops. The ordinary
  cause is harmless: you ran `git init` on your machine and separately created the
  repository on GitHub with a README file ticked. Both sides now have their own starting
  commit, and git has no way to know they were meant to be the same project.

danger: >
  The last step deletes your local folder and clones a fresh copy. Everything in that folder
  that was never committed and pushed is gone, including untracked files an agent created.
  Use it only after checking that the GitHub copy has the work you want.

fix_ladder:
  - try: Confirm origin points at the repository you think it does.
    command: git remote -v
    shell: powershell
    why: >
      Assumes the remote address is wrong, which is the case this refusal exists to catch.
      A copied clone command or an agent configuring the wrong repository name puts a
      completely different project on the other end. If the address is not yours, fix that
      and stop here.

  - try: Look at what is on the remote side.
    command: git log --oneline -5 origin/main
    shell: powershell
    why: >
      Assumes the address is right and you want to see what you would be joining. One
      commit called "Initial commit" that adds a `README.md` is the signature of the
      create-on-GitHub-with-a-README case, and it is safe to merge.

  - try: Allow the merge, once you have confirmed both sides are yours.
    command: git pull origin main --allow-unrelated-histories
    shell: powershell
    why: >
      Assumes both histories belong to this project and you want them joined into one. Git
      stitches the two roots together with a merge commit. The flag exists so that allowing
      it has to be a decision rather than an accident.

  - try: Resolve the conflict the merge produces.
    command: git status
    shell: powershell
    why: >
      Assumes the merge started and stopped on a file that exists on both sides with
      different contents, which is nearly always `README.md`. Edit the file so it reads the
      way you want, `git add` it, and commit.

  - try: Throw away the local folder and clone the GitHub copy instead.
    command: git clone https://github.com/nyxlocke/<repo>.git
    shell: powershell
    why: >
      Assumes the local repository holds nothing worth keeping, which is true when you
      initialized it by mistake five minutes ago. Clone into a fresh folder first, check
      that everything you want is in it, and only then delete the old one.

if_none_worked: >
  Paste the error, the output of `git remote -v`, and the output of both
  `git log --oneline -3` and `git log --oneline -3 origin/main`. Those two log outputs are
  what people leave off, and comparing the two starting commits is the only way to tell an
  ordinary double-initialized repository from origin pointing at a stranger's project.

see_also:
  - d2-repo-remote-clone-origin
  - d6-merge-and-rebase
  - d1-what-git-actually-stores

keywords:
  - unrelated histories
  - allow-unrelated-histories
  - git init twice
  - two initial commits
  - pull rejected
---

The usual path into this takes about four minutes and feels like doing everything right.

You make a folder, run `git init`, write some code, commit. Then you go to GitHub, create a
repository, and tick the box that adds a README. GitHub makes its own first commit. You add
that repository as `origin` and pull, and git sees two projects that have never met.

The fix is one flag, and it is worth reading the remote log before you use it. The same
error appears when `origin` points at somebody else's repository because a clone command
was copied from a tutorial, and merging that in makes a genuine mess.

To avoid it next time, create the repository on GitHub with no README and no license, or
create it first and clone it rather than running `git init` locally.
