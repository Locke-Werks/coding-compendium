---
id: git-please-tell-me-who-you-are
title: "Please tell me who you are"
type: error
verified: 2026-08-02
volatility: low

category: config

# Prints the name and email git will stamp on your next commit. If either
# prints nothing, that is the one that is missing.
verify: git config --global user.name; git config --global user.email

sample: |
  PS C:\Users\nyx\dev\scraper> git commit -m "feat: add scraper"
  Author identity unknown

  *** Please tell me who you are.

  Run

    git config --global user.email "you@example.com"
    git config --global user.name "Your Name"

  to set your account's default identity.
  Omit --global to set the identity only in this repository.

  fatal: unable to auto-detect email address (got 'nyx@DESKTOP-4K2M.(none)')

patterns:
  - "Please tell me who you are"
  - "Author identity unknown"
  - "unable to auto-detect email address"

means: >
  Every commit is stamped with a name and an email address, and git has neither. It guessed
  from your Windows username and machine name, produced something that is not a real email
  address, and refused to write it into permanent history. Nothing was committed. Your
  staged changes are untouched and still staged.

fix_ladder:
  - try: Set your name and email once, for every repository on this machine.
    command: git config --global user.name "Nyx"; git config --global user.email "you@example.com"
    shell: powershell
    why: >
      Assumes this is a fresh Git install and it has never been told. `--global` writes to
      `C:\Users\<yourname>\.gitconfig`, so you do this once and never again. Use the email
      attached to your GitHub account or the commits will not link back to your profile.

  - try: Read it back.
    command: git config --global user.name; git config --global user.email
    shell: powershell
    why: >
      Assumes the setting did not take. Each command prints one line. A blank line means
      that value is still unset, usually because a quote was mismatched in the command that
      set it.

  - try: Commit again.
    command: 'git commit -m "feat: add scraper"'
    shell: powershell
    why: >
      Assumes the identity is now set and the original commit never happened. Nothing needs
      re-staging. Git refused before it wrote anything, so the same commit command works as
      written.

  - try: Use a different identity for this one project.
    command: git config user.email "work@example.com"
    shell: powershell
    why: >
      Assumes you have a reason to keep this project separate, such as a work account. Left
      off, `--global` is not applied and the setting lands in this repository's own config
      file, where it overrides the global one.

  - try: Use GitHub's no-reply address if you would rather not publish your email.
    command: git config --global user.email "12345678+nyxlocke@users.noreply.github.com"
    shell: powershell
    why: >
      Assumes you noticed that every commit email is public forever on a public repository.
      GitHub gives you this address under Settings, then Emails. The number in front is
      yours and is shown on that page.

if_none_worked: >
  Paste the whole message including the `fatal:` line with the guessed address in it, plus
  the output of `git config --list --show-origin | Select-String user`. The origin column is
  what people trim, and it shows which file each setting came from, which is how you find a
  repository-level setting quietly overriding your global one.

see_also:
  - b3-tell-git-who-you-are
  - d4-commit-well
  - b9-where-settings-live

keywords:
  - author identity unknown
  - user.email not set
  - git config identity
  - cannot commit
  - tell me who you are
---

Git refuses rather than guessing because the stamp is permanent. A commit's author is
baked into its identity, so changing it later means rewriting history and every commit id
downstream.

This is why it appears on a brand new machine and then never again. `--global` writes to
one file in your home folder and every repository reads it.

The email is worth getting right the first time. GitHub matches commits to your profile by
email address. Commits made with an address GitHub does not recognize still work perfectly.
They show up as an anonymous name with no avatar and no link, and fixing that after the
fact means rewriting history.
