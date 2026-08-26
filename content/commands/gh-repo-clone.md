---
id: gh-repo-clone
title: gh repo clone
type: command
verified: 2026-08-02
volatility: quarterly

tool: gh
command: gh repo clone <owner>/<repository-name>
shell: powershell

does: >
  Downloads a GitHub repository into a new folder, using the credentials the GitHub CLI
  (Command-Line Interface) already holds so private repositories work without extra setup.

flags:
  - flag: "<owner>/<repository-name>"
    means: >
      The short form of the address, such as `yourname/myproject`. The owner is the user or
      organization. If you own it, the owner half can be left off.
  - flag: "<folder-name>, added at the end"
    means: Clones into a folder you name instead of one named after the repository.
  - flag: "-- --depth 1"
    means: >
      Anything after a bare `--` is passed straight through to `git clone`. This is how you
      reach git's own flags, such as a shallow clone.

expect: >
  `Cloning into 'myproject'...` and the same counting output `git clone` produces, then a new
  folder in your current directory.

see_also:
  - git-clone
  - gh-repo-create
  - gh-auth-status
  - d2-repo-remote-clone-origin

keywords:
  - clone a private repo
  - download repo with gh
  - get someone else's project
---

The difference from plain `git clone` is authentication. Because the GitHub command-line
tool already holds your login, a private repository clones without a credential prompt and
without you choosing between an address that uses HTTPS (Hypertext Transfer Protocol
Secure) and one that uses SSH (Secure Shell).

For a repository you do not own and intend to change, you want a fork first:
`gh repo fork <owner>/<repository-name> --clone`.
