---
id: b5-ssh-vs-https
title: SSH and HTTPS, and which to pick
type: section
track: B
order: 50
verified: 2026-08-02
volatility: quarterly
verify: ssh -T git@github.com
answer: >
  Both are ways your machine proves to GitHub that it is allowed to push: SSH
  uses a key pair that stays on your machine, HTTPS uses a token you paste. Pick
  SSH, because you set it up once and never think about it again.
owns:
  - SSH keys
  - HTTPS tokens
  - authentication to GitHub
see_also:
  - g6-secrets-and-what-never-to-commit
  - b4-github-and-gh
  - d2-repo-remote-clone-origin
keywords:
  - ssh key
  - personal access token
  - permission denied publickey
  - github password
  - ssh-keygen
  - which protocol
  - id_ed25519
---

## More

Pushing code to GitHub needs proof that you are allowed to. There are two ways to give it,
and you pick one when you run `gh auth login` ([b4](#b4-github-and-gh)).

**SSH (Secure Shell)** uses a key pair: two files generated together. The private one stays
on your machine at `C:\Users\<yourname>\.ssh\id_ed25519` and never leaves it. The public one
gets uploaded to your GitHub account. When you push, your machine proves it holds the private
key without ever sending it. Nothing to remember and nothing to expire.

**HTTPS (Hypertext Transfer Protocol Secure)** uses a token, a long password-like string that
GitHub generates for you and your machine sends with every request. It works fine. It can
expire, it has to be stored somewhere, and it is a secret you can accidentally paste into a
chat window or commit to a repository.

**Pick SSH.** If `gh auth login` already set it up, you are done and there is nothing to do
here.

To see which one a project is set up for, run this inside it:

```powershell
git remote -v
```

A line reading `git@github.com:nyxlocke/my-project.git` is SSH. A line reading
`https://github.com/nyxlocke/my-project.git` is HTTPS. Both print twice, once for fetch and
once for push, which is normal.

Test SSH end to end:

```powershell
ssh -T git@github.com
```

`-T` means "do not ask for an interactive terminal." Success looks like this, and reads like a
rejection until you have seen it once:

```text
Hi nyxlocke! You've successfully authenticated, but GitHub does not provide shell access.
```

That is the answer you want. GitHub is confirming who you are and declining to give you a
login shell, which it does not offer to anyone.

## Full

### Making a key by hand

`gh auth login` usually does this for you. When you need it directly, in PowerShell:

```powershell
ssh-keygen -t ed25519 -C "<your-github-email>"
```

`-t ed25519` picks the key type, which is the current default and shorter than the older
`rsa` keys. `-C` adds a comment so you can tell keys apart later; the email is a convention,
not a login.

It asks two questions:

- **Where to save it.** Press Enter to take the default, `C:\Users\<yourname>\.ssh\id_ed25519`.
- **A passphrase.** Press Enter twice for none. On a machine only you use that is a
  reasonable choice. Setting one means being asked for it the first time each session uses
  the key, in exchange for the key being useless to someone who copies the file.

Two files appear:

| File | What it is | Where it goes |
|---|---|---|
| `id_ed25519` | the private key | stays on this machine, forever |
| `id_ed25519.pub` | the public key | uploaded to GitHub, safe to show anyone |

### Uploading the public half

Print it:

```powershell
Get-Content ~/.ssh/id_ed25519.pub
```

One long line starting `ssh-ed25519` and ending with the comment you set. Copy the whole
thing, go to github.com, Settings, then `SSH and GPG keys`, then New SSH key. Give it a title
naming the machine, paste, save.

The one-line version, if `gh` is already authenticated:

```powershell
gh ssh-key add ~/.ssh/id_ed25519.pub --title "<this machine>"
```

Then `ssh -T git@github.com` to confirm, as above. The first connection ever asks whether you
trust `github.com` and shows a fingerprint. Type `yes`.

### The failure you will actually hit

```text
git@github.com: Permission denied (publickey).
fatal: Could not read from remote repository.
```

The machine offered no key GitHub recognizes. Usually the public key was never uploaded, or
the repository's remote is set to SSH while the key setup went to a different account.
[git-permission-denied-publickey](#git-permission-denied-publickey) walks the ladder.

The HTTPS equivalent, from following an old tutorial:

```text
remote: Support for password authentication was removed on August 13, 2021.
```

GitHub does not accept account passwords from the command line. That is
[git-password-authentication-removed](#git-password-authentication-removed), and the fix is
this card.

### Switching a repository from one to the other

The choice is per repository, stored in the remote URL (Uniform Resource Locator), and
changing it is one command:

```powershell
git remote set-url origin git@github.com:nyxlocke/<repo>.git
```

Swap `<repo>` for the repository name. Run `git remote -v` again to confirm it changed.
Nothing else about the project is affected: the remote address is a label on a connection,
not part of your history.

### Tokens, for when you need one anyway

Some situations still want HTTPS: a locked-down network that blocks the port SSH uses, or a
script running somewhere you cannot install a key. Then you create a PAT (Personal Access
Token) on github.com under Settings, Developer settings, Personal access tokens.

Two kinds exist. Fine-grained tokens are scoped to specific repositories and specific
permissions and expire on a date you choose. Classic tokens are broader and older. Prefer
fine-grained, give it the narrowest access that works, and set an expiry.

Then treat it exactly like a password, because it is one:

- It goes in a password manager, not a text file in your project.
- It never gets committed, and `.env` files belong in `.gitignore`.
  [g6](#g6-secrets-and-what-never-to-commit) covers what to do when one leaks, and the
  answer is always to revoke it rather than to delete the commit.
- It never gets pasted into an AI (Artificial Intelligence) chat window, including the
  agents. [g8](#g8-what-never-to-paste-into-a-chat).

### What lives in the .ssh folder

`C:\Users\<yourname>\.ssh\` holds a small number of files and only one of them is dangerous:

- `id_ed25519`, the private key. Never copy it anywhere, never commit it, never paste it. If
  it ever leaves the machine, delete the public key from GitHub and generate a new pair.
- `id_ed25519.pub`, the public key. Harmless. Publishing it is its purpose.
- `known_hosts`, a record of servers you have connected to before. Safe to ignore.
- `config`, present only if something created it, which maps host names to specific keys.

The folder is inside your user profile rather than inside any project, so git never sees it.
That is deliberate, and it is why the rule "never commit a private key" is easy to follow by
default and only gets broken when someone copies one into a project on purpose.
