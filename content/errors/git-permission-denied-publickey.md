---
id: git-permission-denied-publickey
title: "Permission denied (publickey)"
type: error
verified: 2026-08-02
volatility: low

category: permission

# Tests the connection without touching a repository. Success prints a line
# greeting you by GitHub username, then exits with code 1, which is normal.
verify: ssh -T git@github.com

sample: |
  PS C:\Users\you\dev\scraper> git push
  git@github.com: Permission denied (publickey).
  fatal: Could not read from remote repository.

  Please make sure you have the correct access rights
  and the repository exists.

patterns:
  - 'Permission denied \(publickey\)'
  - "Could not read from remote repository"
  - "make sure you have the correct access rights"

means: >
  Git tried to prove to GitHub that this machine is allowed in, using SSH (Secure Shell), and
  GitHub rejected the proof. SSH works with a pair of files: a private key that stays on your
  machine and a public key you upload to GitHub. Either your machine offered no key, or it
  offered one GitHub has never seen, or the key exists but the program that hands it over is
  not running. GitHub never got as far as looking at which repository you wanted.

fix_ladder:
  - try: Ask GitHub who it thinks you are.
    command: ssh -T git@github.com
    shell: powershell
    why: >
      Assumes the key setup is nearly right and you need to know which part failed. Success
      prints `Hi <yourname>! You've successfully authenticated`. If that works and `git push`
      still fails, the key is fine and the repository address is the problem. If it fails the
      same way, the key is the problem.

  - try: Check whether you have a key at all.
    command: Get-ChildItem ~\.ssh
    shell: powershell
    why: >
      Assumes no key was ever made. You are looking for a pair such as `id_ed25519` and
      `id_ed25519.pub`. An empty folder or a missing folder means nothing has been set up
      and the key generation step is what you need.

  - try: Start the agent that hands the key over, and load the key.
    command: Start-Service ssh-agent; ssh-add ~\.ssh\id_ed25519
    shell: powershell
    why: >
      Assumes the key exists but nothing is offering it. On Windows the ssh-agent service is
      set to manual start by default, so it is often not running at all. `Start-Service`
      needs an administrator window the first time.

  - try: Confirm the public key is on your GitHub account.
    command: Get-Content ~\.ssh\id_ed25519.pub
    shell: powershell
    why: >
      Assumes the key exists locally and GitHub has never been given it. Copy the whole line
      this prints, go to GitHub, then Settings, then SSH and GPG keys, and compare it against
      what is listed. Add it if it is not there. Only the `.pub` file is ever shared.

  - try: Check whether this repository is using SSH or HTTPS.
    command: git remote -v
    shell: powershell
    why: >
      Assumes the whole thing is a mismatch. An address starting `git@github.com:` uses SSH
      and needs a key. One starting `https://github.com/` uses a token instead and would
      never produce this message, so seeing this error at all confirms you are on the SSH
      path.

if_none_worked: >
  Paste the error, the output of `git remote -v`, and the output of `ssh -vT git@github.com`.
  The `-v` means verbose and it prints every key the client offered and what the server said
  about each. That output is long and looks like noise, which is why people cut it, and it
  contains the actual reason on a line beginning with `debug1: Offering public key`.

see_also:
  - b5-ssh-vs-https
  - b4-github-and-gh
  - d2-repo-remote-clone-origin

keywords:
  - permission denied publickey
  - ssh key github
  - could not read from remote repository
  - ssh-agent
  - git push denied
---

The message names the method that failed, and that is the useful part. `publickey` means
GitHub was offered a key and did not accept it. It is never about the repository being
private or about you not having access to it, despite the wording of the second half.

On Windows the ssh-agent service catches people repeatedly. Windows ships it disabled or
set to manual, so a key that worked yesterday stops working after a restart. Setting the
service to start automatically makes it stick:

```powershell
Set-Service -Name ssh-agent -StartupType Automatic
```

There is a quick way out if you are stuck and need to push right now. Switch the remote to
HTTPS (HyperText Transfer Protocol Secure) and let the GitHub credential helper handle it
with a browser sign-in:

```powershell
git remote set-url origin https://github.com/<yourname>/<repo>.git
```

That trades one authentication method for another rather than fixing the key, so come back
to the key when you have time.
