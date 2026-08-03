---
id: gh-auth-login
title: gh auth login
type: command
verified: 2026-08-02
volatility: quarterly

tool: gh
command: gh auth login
shell: powershell

verify: gh auth status

does: >
  Signs the GitHub CLI (Command-Line Interface) into your GitHub account and stores the
  credentials so git can push without asking you again.

flags:
  - flag: "--web"
    means: >
      Skips the questions and goes straight to the browser flow. You get an eight-character
      code to paste into the page that opens.
  - flag: "-p https"
    means: >
      Short for `--git-protocol https`. Picks how git will talk to GitHub afterward. `https`
      uses the token this command just stored. `ssh` uses a key pair instead.
  - flag: "-h <hostname>"
    means: >
      Signs into a GitHub Enterprise server rather than github.com. Leave it off for a normal
      account.
  - flag: "--with-token"
    means: >
      Reads a token from the pipeline instead of opening a browser. For automation. Do not
      paste a token into a chat window with an agent.

expect: >
  A series of questions, then a browser opens. After you paste the code and approve, the
  terminal prints a success line ending with `Logged in as nyxlocke`.

see_also:
  - gh-auth-status
  - gh-repo-create
  - b4-github-and-gh
  - b5-ssh-vs-https

keywords:
  - sign in to github
  - authenticate gh
  - github login terminal
  - gh not authenticated
---

Answer `GitHub.com`, then `HTTPS`, then yes to authenticating git with your credentials, then
`Login with a web browser`. Those four answers are right for nearly everyone on a personal
machine.

This is what lets an agent open a pull request as you. Until it succeeds, every `gh` command
fails with `authentication required`.
