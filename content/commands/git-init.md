---
id: git-init
title: git init
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git init
shell: any

does: >
  Turns the folder you are currently standing in into a git repository, so git starts
  tracking changes to the files inside it.

flags:
  - flag: "-b <branch-name>"
    means: >
      Names the first branch. Without it you get whatever your global setting says, which
      is `master` on an old git install and `main` on a configured one. Use
      `git init -b main` if you want to be certain.

expect: >
  One line, `Initialized empty Git repository in C:/Users/you/dev/myproject/.git/`. A
  hidden `.git` folder now exists in that directory. Run `git status` and it answers
  instead of complaining that this is not a repository.

see_also:
  - git-status
  - git-remote-add
  - d1-what-git-actually-stores
  - d2-repo-remote-clone-origin

keywords:
  - start a repo
  - make this folder a git repo
  - new repository
  - not a git repository
---

Check where you are before you run it. `git init` in your home folder or in
`C:\Users\<yourname>\Documents` makes git try to track every file underneath, which is
thousands of files you did not mean to version. If that happens, delete the hidden `.git`
folder that was just created and the folder goes back to being ordinary.
