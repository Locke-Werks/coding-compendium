---
id: git-password-authentication-removed
title: "Support for password authentication was removed"
type: error
verified: 2026-08-02
volatility: quarterly

category: permission

# Prints your GitHub username and the scopes your stored credential has. An
# error here means nothing is signed in.
verify: gh auth status

sample: |
  PS C:\Users\nyx\dev\scraper> git push
  remote: Support for password authentication was removed on August 13, 2021.
  remote: Please see https://docs.github.com/get-started/getting-started-with-git/about-remote-repositories#cloning-with-https-urls for information on currently recommended modes of authentication.
  fatal: Authentication failed for 'https://github.com/nyxlocke/scraper.git/'

patterns:
  - "Support for password authentication was removed"
  - "Authentication failed for"
  - "currently recommended modes of authentication"

means: >
  You are connecting to GitHub over HTTPS (HyperText Transfer Protocol Secure) and something
  sent your account password. GitHub stopped accepting account passwords for git operations
  years ago. It wants either a personal access token, which is a long generated string used
  in place of a password, or a credential helper that signs you in through a browser. The
  password itself is not wrong. That whole method no longer exists.

fix_ladder:
  - try: Sign in with the GitHub command-line tool and let it store the credential.
    command: gh auth login
    shell: powershell
    why: >
      Assumes `gh` is installed. Pick GitHub.com, then HTTPS, then authenticate through the
      browser. It writes a working credential into Windows Credential Manager, and git picks
      it up automatically from then on. This is the shortest path by a wide margin.

  - try: Check what is signed in right now.
    command: gh auth status
    shell: powershell
    why: >
      Assumes something is signed in already and is the wrong account or has expired. This
      names the account and the permissions attached to it. Tokens do expire, and an expired
      one fails exactly like a wrong password.

  - try: Clear the stale credential Windows has saved.
    why: >
      Assumes Windows Credential Manager is handing git an old password saved years ago.
      Press the Windows key, type `Credential Manager`, open it, choose Windows Credentials,
      and delete any entry mentioning `git:https://github.com`. The next push will ask again.

  - try: Use a personal access token as the password.
    why: >
      Assumes you would rather not install anything. On GitHub go to Settings, then Developer
      settings, then Personal access tokens, and create one with `repo` permission. When git
      prompts for a password, paste the token instead. Treat it like a password, because it
      is one.

  - try: Switch this repository to SSH instead.
    command: git remote set-url origin git@github.com:nyxlocke/<repo>.git
    shell: powershell
    why: >
      Assumes you already have an SSH (Secure Shell) key set up on your GitHub account. SSH
      uses a key file rather than a password and never expires on a schedule. If you have no
      key, this trades one setup task for another.

if_none_worked: >
  Paste every `remote:` line, the `fatal:` line with the full address in it, and the output of
  `gh auth status`. The address matters because it shows you are on HTTPS rather than SSH, and
  people trim it as if it were private. It is the same address that is on your GitHub page.

see_also:
  - b5-ssh-vs-https
  - b4-github-and-gh
  - g6-secrets-and-what-never-to-commit

keywords:
  - password authentication removed
  - authentication failed github
  - personal access token
  - gh auth login
  - credential manager
---

Two ways in to this one. You typed your GitHub password at a prompt, or Windows Credential
Manager still holds a password saved before the rules changed and is submitting it for you.

The second case is the confusing one, because you are not typing anything and it fails
anyway. Nothing on screen suggests a saved credential is involved. Clearing the entry in
Credential Manager forces a fresh prompt and usually ends it.

A token is a password with an expiry date and a limited set of permissions. That is the
entire difference. It goes in the same box, gets saved the same way, and needs replacing
when it expires. Never paste one into a chat, a commit, or a file in the repository.
