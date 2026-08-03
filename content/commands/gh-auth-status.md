---
id: gh-auth-status
title: gh auth status
type: command
verified: 2026-08-02
volatility: quarterly

tool: gh
command: gh auth status
shell: powershell

does: >
  Reports whether the GitHub CLI (Command-Line Interface) is signed in, which account it is
  using, and what that login is allowed to do.

flags:
  - flag: "-t"
    means: >
      Short for `--show-token`. Prints the stored access token in full. Treat that output the
      way you would treat a password: never paste it into a chat, a screenshot, or a file.
  - flag: "-h <hostname>"
    means: Checks one host only, which matters if you are signed into both github.com and a company server.

expect: >
  A block naming `github.com`, a check line reading `Logged in to github.com account
  nyxlocke`, the protocol in use, and a `Token scopes:` line. If it says `You are not logged
  into any GitHub hosts`, run `gh auth login`.

see_also:
  - gh-auth-login
  - gh-pr-create
  - b4-github-and-gh

keywords:
  - am i logged in to github
  - check gh authentication
  - which github account
  - token scopes
---

Run this first whenever a `gh` command fails or an agent reports it cannot open a pull
request. It answers the question in one line and changes nothing.

Check the account name. Being signed in as the wrong account is a real failure mode when
you have a personal and a work login on the same machine.
