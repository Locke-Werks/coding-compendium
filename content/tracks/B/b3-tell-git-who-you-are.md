---
id: b3-tell-git-who-you-are
title: Tell Git who you are
type: section
track: B
order: 30
verified: 2026-08-02
volatility: low
verify: git config --global user.email
answer: >
  Every commit is stamped with a name and email, so set yours once with
  `git config --global user.name` and `user.email`, then read them back to
  confirm. Until you do, git refuses to make your first commit.
owns:
  - git config identity
  - default branch name
see_also:
  - d4-commit-well
  - b2-install-git
  - d1-what-git-actually-stores
  - b9-where-settings-live
keywords:
  - git config
  - please tell me who you are
  - set git email
  - default branch main
  - pull.rebase
  - gitconfig location
---

## More

Git stamps every commit with a name, an email address, and a timestamp. It has no way to
guess yours, so you tell it once, globally, and every project on the machine uses it. Use the
email attached to your GitHub account, otherwise GitHub cannot connect your commits to you
and your contribution history stays blank.

Four commands, run in any shell, any folder:

```powershell
git config --global user.name "Nyx Locke"
git config --global user.email "<your-github-email>"
git config --global init.defaultBranch main
git config --global pull.rebase false
```

Swap `<your-github-email>` for the address on your GitHub account, keeping the quotes.
`--global` means "for me, on this machine, in every project."

The last two are not about identity and are worth setting in the same sitting.
`init.defaultBranch main` makes every new repository start on a branch named `main`, which is
the current convention and replaced the older `master`. `pull.rebase false` makes `git pull`
behave one predictable way instead of asking you a question you do not yet have an opinion
about.

Read them back to confirm each one took:

```powershell
git config --global user.name
git config --global user.email
```

Each prints the value you set, on its own line. Printing nothing means it did not take,
usually a typo in the key name.

If you skip this, your first commit fails with `Please tell me who you are`, which is git
asking for exactly these two commands.
[git-please-tell-me-who-you-are](#git-please-tell-me-who-you-are) has that error in full.

## Full

### Where these end up

`--global` writes to a plain text file at `C:\Users\<yourname>\.gitconfig`, which in
PowerShell you can also write as `~/.gitconfig`. Open it and it reads like this:

```text
[user]
	name = Nyx Locke
	email = nyx@example.com
[init]
	defaultBranch = main
[pull]
	rebase = false
```

Nothing magic. Every `git config --global` command is editing that file for you, and you can
edit it by hand if you prefer. To see everything currently set, including where each value
came from:

```powershell
git config --global --list
```

Add `--show-origin` and each line is prefixed with the file it was read from, which is how
you settle "why is this value not what I set."

### The three levels, and which wins

Git reads three files and the narrowest one wins:

| Level | Flag | File | Applies to |
|---|---|---|---|
| System | `--system` | inside the git install | every user on the machine |
| Global | `--global` | `C:\Users\<yourname>\.gitconfig` | you, in every project |
| Local | `--local` | `.git\config` inside a project | that one project |

You will use `--global` for almost everything. The one common reason to reach for `--local`
is a project that needs a different email, for example work code under a work address. Run it
inside that project, without `--global`:

```powershell
git config user.email "<your-other-email>"
```

That value applies to that repository only, and it beats the global one. Every tool that
keeps settings at several levels works this way, and
[b9](#b9-where-settings-live) covers the same idea for Claude Code and Codex.

### Keeping your email private

Publishing commits publishes the email in them. GitHub can give you a forwarding address of
the form `<id>+nyxlocke@users.noreply.github.com` for exactly this reason. Find the real one
under Settings, then Emails, on github.com, where the option to keep your address private
also lives. Set that address with the same `user.email` command and your commits carry the
noreply address instead.

Worth doing before your first public repository rather than after, because commits already
pushed keep whatever email they were made with.

### What the stamp is actually for

The name and email are labels, not credentials. Git does not check them against anything, and
setting them does not give you permission to push anywhere. That is a separate thing,
handled by SSH (Secure Shell) keys or a token, and it is [b5](#b5-ssh-vs-https).

What the stamp does is answer "who made this change" when you read history six months later,
which is the main reason [d13](#d13-tags-releases-and-history) exists. Git records two
identities per commit, author and committer, which are usually the same person and diverge
when a commit is replayed onto another branch.
[d1](#d1-what-git-actually-stores) shows both inside a real commit.

### Changing it afterward

Running the commands again overwrites the values, and takes effect on the next commit.
Commits already made keep the old stamp, because a commit's identity is part of what its
hash is computed from. Rewriting them is possible and it changes every hash from that point
forward, which is a real cost. If you notice a wrong email one commit in, fix the config and
amend that one commit. If you notice it fifty commits in on a private project, fix the config
and leave history alone.

### The settings worth adding once you have hit the problem

Not needed today. Worth knowing they exist, because each solves a specific annoyance:

- `git config --global core.autocrlf true` handles Windows line endings. The Git for Windows
  installer sets this for you. [c8](#c8-line-endings-and-encoding).
- `git config --global core.editor "code --wait"` uses Visual Studio Code for commit
  messages instead of the default editor, if you have it installed.
- `git config --global alias.st status` makes `git st` mean `git status`. Aliases are
  personal preference and nothing here depends on them.
