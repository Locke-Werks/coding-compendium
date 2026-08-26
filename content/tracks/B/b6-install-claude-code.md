---
id: b6-install-claude-code
title: Install and sign in to Claude Code
type: section
track: B
order: 60
verified: 2026-08-02
volatility: weekly
verify: claude --version
answer: >
  In PowerShell, run `irm https://claude.ai/install.ps1 | iex`, close and reopen
  the terminal, then run `claude --version`. Start it inside a project folder by
  typing `claude`, and sign in through the browser window it opens.
owns:
  - claude code installation
  - the native installer
see_also:
  - b1-terminal-shell-command-line
  - e4-claude-md-and-agents-md
  - b8-turn-off-ai-attribution
  - b9-where-settings-live
  - e3-plan-mode
  - c4-path-and-command-not-found
keywords:
  - install claude code
  - claude command not found
  - irm not recognized
  - execution policy
  - claude code windows
  - native installer
  - sign in to claude code
---

## More

Two things have to be true before this works: Git for Windows is installed
([b2](#b2-install-git)), and your Claude account is on a plan that includes Claude Code. The
free tier does not. Paid individual and team plans do, as does an Anthropic Console account
with credits on it. Plan names and boundaries move, so https://code.claude.com/docs is the
authority, not this card.

Open **PowerShell**, not Command Prompt, and run:

```powershell
irm https://claude.ai/install.ps1 | iex
```

`irm` fetches the installer script and `iex` runs it. This is the native installer, a
self-contained program that needs no other runtime and updates itself in the background. Use
it unless you have a specific reason not to.

**Close and reopen the terminal**, then confirm:

```powershell
claude --version
```

A version number means you are installed. Then move into a project folder and start it:

```powershell
cd ~/dev/my-first-project
claude
```

In PowerShell, `~` is shorthand for `C:\Users\<yourname>`. The first run opens your browser to
sign in. Approve it, come back to the terminal, and you are in a session where you type
requests in plain English.

Three commands to know on day one. `/help` lists everything. `/init` reads the project and
writes its instruction file, which is [e4](#e4-claude-md-and-agents-md). `/model` opens the
picker, which is where current model names live, so take them from there rather than from any
blog post.

Two settings to change before you build anything real: turn off the commit attribution
([b8](#b8-turn-off-ai-attribution)), and learn Shift+Tab for plan mode
([e3](#e3-plan-mode)).

## Full

### The three snags, with their exact on-screen text

**`'irm' is not recognized as an internal or external command`**

You are in Command Prompt. `irm` is a PowerShell command and Command Prompt has never heard
of it. Look at your prompt: PowerShell starts with `PS`. Open PowerShell and run it there.
[b1](#b1-terminal-shell-command-line) tells the shells apart and
[irm-not-recognized-in-cmd](#irm-not-recognized-in-cmd) has the full card.

**`cannot be loaded because running scripts is disabled on this system`**

PowerShell's execution policy is blocking the downloaded script. Run this once, then run the
installer again:

```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

`RemoteSigned` means scripts you wrote locally run, and scripts downloaded from the internet
must carry a valid signature. `-Scope CurrentUser` limits the change to your account rather
than the whole machine, which is why this does not need an administrator terminal. Details in
[powershell-execution-policy-disabled](#powershell-execution-policy-disabled).

**`claude` is not recognized, right after a successful install**

The terminal built its list of available commands when it opened, before `claude` existed.
Close every terminal window and open a new one. This is the single most common Windows
confusion of the whole setup and [c4](#c4-path-and-command-not-found) explains the mechanism.
[claude-command-not-found-after-install](#claude-command-not-found-after-install) is the
card.

### The other ways to install it

```powershell
winget install Anthropic.ClaudeCode
```

Works, and carries one tradeoff worth stating: the winget version does not update itself, so
you would run `winget upgrade Anthropic.ClaudeCode` yourself from time to time. The native
installer above auto-updates, which is the reason it is the recommendation.

There is also an older method through npm, the Node Package Manager, which installs through
the JavaScript tooling stack and needs a recent Node.js. Skip it on a fresh Windows machine.
It exists in enough old instructions to be worth recognizing.

### What a session looks like

You are in a text interface inside your terminal. You type a request, it responds and takes
actions, and it asks before doing things that change files or run commands. Slash commands
control the tool rather than the conversation:

| Command | What it does |
|---|---|
| `/help` | lists every command |
| `/init` | scans the project and writes its instruction file |
| `/model` | shows the current model picker |
| `/clear` | starts a fresh context in the same folder |
| `/login` | signs in again, or as a different account |

`/clear` matters more than it looks. A long session gets worse, mechanically, and clearing is
the fix. [e6](#e6-when-to-reset-context) covers when to reach for it.

Start it from the project root every time. It anchors what the agent can see, and starting it
from your home folder is how people end up with an agent that cannot find files that are
plainly there.

### Git Bash, and why the install order mattered

Because Git for Windows was already installed, Claude Code can use Git Bash as the shell it
runs commands in, which handles more of what it wants to do than PowerShell does. Nothing to
configure. If you installed Claude Code first and it complains about a missing shell, install
git ([b2](#b2-install-git)) and restart the session.

### Connecting it to GitHub

Nothing to do, if you finished [b4](#b4-github-and-gh). Because `gh` is authenticated as
`yourname`, Claude Code can run `git` and `gh` commands with your credentials: commit, push,
open a pull request. Ask for it in plain language.

A separate, optional thing is the GitHub App, which lets you mention Claude inside a GitHub
issue or pull request and have it respond on GitHub's servers. From inside a session:

```text
/install-github-app
```

It walks you through installing it on your repositories. Convenience for later, not required
to work locally, and safe to skip.

### Where its settings live

Claude Code reads settings from JSON (JavaScript Object Notation) files at three levels: one
for your whole machine, one committed with a project, and one private to you inside a
project. Narrower wins. The paths and the precedence rules are
[b9](#b9-where-settings-live), and the first thing you should put in the machine-level file
is the attribution block from [b8](#b8-turn-off-ai-attribution).

### Keeping current

The native install updates itself, so there is nothing to do. Two consequences: a flag or a
menu can change between one session and the next, and any specific command in this card can
go out of date faster than the rest of this app. That is why this card carries a weekly
freshness budget and shows a badge quickly. When something here disagrees with
https://code.claude.com/docs, the docs are right.
