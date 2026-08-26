---
id: gh-repo-create
title: gh repo create
type: command
verified: 2026-08-02
volatility: quarterly

tool: gh
command: gh repo create <repository-name> --private --source . --remote origin --push
shell: powershell

does: >
  Creates a new repository on GitHub and, with the flags below, connects your existing local
  folder to it and uploads what you have, all in one command.

flags:
  - flag: "--private"
    means: >
      Only you can see it. Use `--public` for one anybody can read. One of the two is
      required, and there is no default, which is deliberate.
  - flag: "--source ."
    means: >
      Use the folder you are standing in as the contents, rather than creating an empty
      repository. The dot means the current directory.
  - flag: "--remote origin"
    means: >
      Name the new connection `origin`, which is the nickname every other git command assumes
      by default.
  - flag: "--push"
    means: >
      Upload your existing commits immediately after creating the repository. Without it, the
      GitHub side stays empty until you push yourself.
  - flag: "--clone"
    means: >
      For the other direction: create an empty repository on GitHub and download it into a new
      local folder. Do not combine it with `--source`.

expect: >
  A line confirming creation such as `Created repository yourname/myproject on GitHub`, then
  a line adding the remote, then push output. Confirm with `git remote -v`.

see_also:
  - gh-repo-clone
  - git-remote-add
  - git-push-set-upstream
  - d2-repo-remote-clone-origin

keywords:
  - new github repo from terminal
  - publish my project
  - create repository
  - put this folder on github
---

Your folder must already be a git repository with at least one commit before `--source .`
works. Run `git init`, `git add .`, and `git commit -m "initial commit"` first.

Check `.gitignore` before you push a folder for the first time. A `--private` repository is
still a copy of every file you send it, credentials included.
