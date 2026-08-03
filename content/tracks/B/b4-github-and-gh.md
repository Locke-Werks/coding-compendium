---
id: b4-github-and-gh
title: Your GitHub account and the gh tool
type: section
track: B
order: 40
verified: 2026-08-02
volatility: quarterly
verify: gh auth status
answer: >
  Install GitHub's command-line tool with `winget install GitHub.cli`, reopen the
  terminal, then run `gh auth login` and answer three prompts. After that your
  machine can push and open pull requests as you, and so can the agents.
owns:
  - gh cli
  - gh auth login
  - winget
see_also:
  - b5-ssh-vs-https
  - d8-pull-requests
  - b2-install-git
  - c4-path-and-command-not-found
keywords:
  - github cli
  - gh auth login
  - winget install
  - connect to github
  - gh not recognized
  - authenticate github
  - nyxlocke
---

## More

You have a GitHub account, `nyxlocke`. Log in at https://github.com once and confirm you can
see it. That is everything the website side needs from you today; the rest is driven from the
terminal.

`gh` is GitHub's official command-line tool. It is the cleanest way to connect this machine to
that account, and it is what the agents use when you ask one to open a pull request. Install
it in PowerShell:

```powershell
winget install GitHub.cli
```

`winget` is the Windows Package Manager, built into Windows 11, which installs software from
the command line. `GitHub.cli` is the exact package name and the capitalization matters.

**Close and reopen your terminal**, then confirm:

```powershell
gh --version
```

If it says `'gh' is not recognized`, the terminal has not picked up the new install yet.
Close every terminal window and open a fresh one. [c4](#c4-path-and-command-not-found)
explains why that works.

Now link the machine to the account:

```powershell
gh auth login
```

It asks a short series of questions. Answer them:

- **What account do you want to log into?** GitHub.com
- **What is your preferred protocol?** SSH (Secure Shell), which is the recommendation and is
  explained in [b5](#b5-ssh-vs-https). HTTPS (Hypertext Transfer Protocol Secure) also works.
- **How would you like to authenticate?** Login with a web browser. It shows a one-time code,
  opens GitHub, you paste the code, and it finishes.

Confirm:

```powershell
gh auth status
```

It should report that you are logged in to github.com as `nyxlocke`.

## Full

### What `gh auth login` actually did

Three things, and knowing them makes the failures readable later.

1. **Stored a token for the `gh` tool itself**, in Windows Credential Manager. That is what
   lets `gh pr create` work without asking who you are.
2. **Configured git to use those credentials** for GitHub URLs, so `git push` stops
   prompting for a password.
3. **If you chose SSH, generated a key pair and uploaded the public half to your account.**
   That is the part that makes pushing silent from then on. [b5](#b5-ssh-vs-https) covers
   what a key pair is and how to check it.

Nothing here is a password. GitHub stopped accepting account passwords over the command line
years ago, which is why an old tutorial that says to type your password produces
`Support for password authentication was removed`.
[git-password-authentication-removed](#git-password-authentication-removed) has that one.

### The commands you will actually use

```powershell
gh repo create nyxlocke/my-project --private --source=. --remote=origin
```

Creates the repository on GitHub under your account, marks it private, links it to the folder
you are standing in, and names that link `origin`. Run it from inside the project folder,
after `git init`. `--source=.` means "this folder", and `origin` is the conventional nickname
for the main remote, covered in [d2](#d2-repo-remote-clone-origin).

```powershell
gh pr create --fill
```

Opens a pull request from your current branch, filling the title and body from your commit
messages. [d8](#d8-pull-requests) covers why this is worth doing even working alone.

```powershell
gh repo view --web
```

Opens the current project's GitHub page in your browser. Small, and you will use it daily.

### When a command says you are missing a scope

A token carries a list of permissions, called scopes. `gh` asks for a sensible starting set,
and some commands need more. The error names the scope it wanted:

```text
error: your authentication token is missing required scopes [read:project]
       To request it, run:  gh auth refresh -s read:project
```

The message contains its own fix. Run the command it prints, approve in the browser, and
retry. Nothing is broken and you do not need to start over.

### Checking and clearing

```powershell
gh auth status
```

Prints the account, the protocol in use, and the token's scopes. This is the first thing to
run when pushing suddenly stops working.

```powershell
gh auth logout
```

Forgets the stored credentials on this machine. Your account and your repositories are
untouched; you would run `gh auth login` again to reconnect. Useful when an account is stuck
in a half-authenticated state.

### Why the agents need this

Claude Code and Codex do not have their own GitHub accounts. When you say "commit this and
open a pull request," the agent runs the same `git` and `gh` commands you would, using the
credentials you set up here. Everything appears under `nyxlocke`, because it was you.

That is worth sitting with for a second. There is no separate approval step on GitHub's side.
Anything you can do from this terminal, an agent with permission to run commands can do too,
which is why [e11](#e11-what-to-never-let-an-agent-do) exists and why force pushes are on
its list.

### The GitHub App, which is a different thing

Claude Code can install a GitHub App on your repositories, which lets you mention it inside
an issue or a pull request and have it respond on GitHub's servers. That is separate from
everything above and it is not required for local work. Skip it until you want it.
[b6](#b6-install-claude-code) mentions where it lives.
