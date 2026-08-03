---
id: git-remote-add
title: git remote add origin
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git remote add origin <repository-url>
shell: any

does: >
  Connects a local repository to a remote one by giving that remote address a nickname,
  which is what makes `git push` possible.

flags:
  - flag: "origin"
    means: >
      The nickname you are creating. `origin` is the conventional name for the main remote
      and is what every other command assumes by default. It carries no special meaning to
      git: you could call it `github` and then type that name everywhere instead.
  - flag: "<repository-url>"
    means: >
      The address, copied from the green Code button on the GitHub repository page. Pick the
      HTTPS (Hypertext Transfer Protocol Secure) or SSH (Secure Shell) form depending on how
      you authenticate.
  - flag: "set-url"
    means: >
      `git remote set-url origin <repository-url>` changes an existing nickname's address.
      Use it when `git remote add` fails with `remote origin already exists`.

expect: >
  Nothing printed. Confirm with `git remote -v`, which should now list `origin` twice with
  your address.

see_also:
  - git-remote-v
  - git-init
  - git-push-set-upstream
  - gh-repo-create

keywords:
  - connect to github
  - add a remote
  - remote origin already exists
  - link local repo to github
---

The usual order for a project that started on your machine: `git init`, then commit
something, then create the empty repository on GitHub, then `git remote add origin`, then
`git push -u origin main`. `gh repo create` does the last three at once.
