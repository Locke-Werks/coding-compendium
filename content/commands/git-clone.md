---
id: git-clone
title: git clone
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git clone <repository-url>
shell: any

does: >
  Downloads a complete copy of a remote repository, including its entire history, into a
  new folder named after the repository.

flags:
  - flag: "<repository-url>"
    means: >
      The address of the repository. Two forms exist. HTTPS (Hypertext Transfer Protocol
      Secure) looks like `https://github.com/nyxlocke/myproject.git` and asks for
      credentials. SSH (Secure Shell) looks like `git@github.com:nyxlocke/myproject.git`
      and uses a key you set up once.
  - flag: "<folder-name>, added at the end"
    means: >
      Clones into a folder of your choosing instead of one named after the repository.
      `git clone <repository-url> myfolder` puts it in `myfolder`.
  - flag: "--depth 1"
    means: >
      Downloads only the most recent commit instead of the full history. Much faster on a
      large project. You cannot read older history afterward without fetching more.
  - flag: "-b <branch-name>"
    means: Checks out that branch after cloning instead of the repository's default branch.

expect: >
  `Cloning into 'myproject'...` followed by counting and compressing lines, then `done.`
  A new folder appears in your current directory. Move into it with
  `Set-Location myproject` before running any other git command.

see_also:
  - git-init
  - git-remote-v
  - gh-repo-clone
  - d2-repo-remote-clone-origin
  - b5-ssh-vs-https

keywords:
  - download a repo
  - copy repository from github
  - get the code
---

`git clone` creates the folder for you. Do not make a folder first and clone inside it, or
you end up with `myproject\myproject`. Run it from the directory you want the project to
sit in, such as `C:\Users\<yourname>\dev`.
