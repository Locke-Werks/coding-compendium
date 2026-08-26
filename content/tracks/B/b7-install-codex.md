---
id: b7-install-codex
title: Install and sign in to Codex
type: section
track: B
order: 70
verified: 2026-08-02
volatility: weekly
verify: codex --version
answer: >
  Run the PowerShell installer from https://chatgpt.com/codex, confirm with
  `codex --version`, then start it inside a project with `codex` and sign in with
  your ChatGPT account through the browser.
owns:
  - codex installation
  - sandbox_mode
  - approval_policy
see_also:
  - e4-claude-md-and-agents-md
  - b9-where-settings-live
  - e10-using-two-agents
  - g5-environment-variables
  - b8-turn-off-ai-attribution
keywords:
  - install codex
  - openai codex cli
  - sandbox mode
  - approval policy
  - config.toml
  - codex sign in
  - codex windows
---

## More

Codex is OpenAI's coding agent. It runs in your terminal, and by default it sandboxes itself:
it cannot reach the network or write outside your project folder unless you allow it. It is a
genuinely different engine from Claude Code, with different strengths, and running both is
normal ([e10](#e10-using-two-agents)).

You need a ChatGPT plan that includes it, or an OpenAI API key for pay-as-you-go use. Signing
in with ChatGPT gets you new models soonest; a key is better for automation. Plan names
change, so https://developers.openai.com/codex is the authority.

In **PowerShell**:

```powershell
powershell -ExecutionPolicy ByPass -c "irm https://chatgpt.com/codex/install.ps1 | iex"
```

The `-ExecutionPolicy ByPass` part applies to that one command only, so the installer runs
even on a machine where downloaded scripts are otherwise blocked. Nothing about your system
policy changes.

Confirm:

```powershell
codex --version
```

Then move into a project and start it:

```powershell
cd ~/dev/my-first-project
codex
```

It prompts you to sign in. Choose "Sign in with ChatGPT" and approve in the browser. That is
the simplest path and the one to use.

Two files matter afterward. `AGENTS.md` at the project root holds your standing instructions
for the project, and it is [e4](#e4-claude-md-and-agents-md). `C:\Users\<yourname>\.codex\config.toml`
holds the settings, and the two you will actually touch are covered below.

## Full

### The two knobs people mix up

They sound like the same setting and they are not. One controls what Codex is technically
able to do. The other controls when it stops to ask you first.

| Setting | Question it answers | Common values |
|---|---|---|
| `sandbox_mode` | What can it touch at all | `read-only`, `workspace-write`, a fully unlocked bypass mode |
| `approval_policy` | When does it pause to ask | `untrusted`, `on-request`, `never` |

`read-only` looks but changes nothing. `workspace-write` edits files inside the project and
still cannot reach the network. The bypass mode removes the wall entirely and is worth
avoiding until you have a specific reason and know what it costs.

`untrusted` asks about a lot. `on-request` is a sensible middle. `never` does not ask, which
belongs in automation and not on your laptop.

They combine. A tight sandbox with `never` is safe and quiet, because the wall does the work.
A loose sandbox with `never` is the configuration people regret.

### The settings file

Codex reads `C:\Users\<yourname>\.codex\config.toml`, which PowerShell also writes as
`~/.codex/config.toml`. TOML (Tom's Obvious, Minimal Language) is a settings format made of
`name = value` lines grouped under `[headings]`, with `#` starting a comment.

```toml
# C:\Users\<yourname>\.codex\config.toml
model = "<pick one from the in-app model picker>"
approval_policy = "on-request"
sandbox_mode = "workspace-write"
```

Setting `model` here makes your choice stick across sessions. Leave it out entirely and Codex
uses its default, which is a perfectly good way to start.

**Do not paste a model name out of an old blog post, including this one.** Model names in
this ecosystem change on a schedule and old ones get retired, at which point a hard-coded name
produces an error that looks like a broken install. Take the current name from the picker
when you launch `codex`, or from https://developers.openai.com/codex/models.

The approval and sandbox reference lives at
https://developers.openai.com/codex/config-advanced. [b9](#b9-where-settings-live) covers how
this file relates to Claude Code's settings and which level wins when both exist.

### Signing in with a key instead

If you are using an API key rather than a ChatGPT sign-in, set it as an environment variable.
For the current terminal only:

```powershell
$env:OPENAI_API_KEY="<your-openai-api-key>"
```

To keep it for future terminals:

```powershell
setx OPENAI_API_KEY "<your-openai-api-key>"
```

`setx` writes the value into your Windows user profile. It does not change the terminal you
are standing in, so open a fresh one afterward or the variable will look like it did not
take. [g5](#g5-environment-variables) covers the difference properly.

The key is a credential. It belongs in a password manager, never in a file inside a
repository, and never pasted into a chat window.
[g6](#g6-secrets-and-what-never-to-commit) covers what to do if one leaks, and the answer
starts with revoking it.

To clear a login that has got stuck:

```powershell
codex logout
```

### If the sandbox misbehaves on Windows

Codex's sandbox runs natively on Windows and that is fine for ordinary work. If you ever hit
sandbox errors that will not clear no matter what you change, the fallback is running Codex
inside WSL (Windows Subsystem for Linux), a full Linux environment that runs inside Windows.

You do not need it to start and this app stays on native Windows throughout. File it as a
break-glass option rather than a setup step.

### Installing it another way

```powershell
npm i -g @openai/codex
```

Works if you already have Node.js installed, which npm, the Node Package Manager, comes with.
There is no advantage to it on a fresh machine, and it adds a dependency you would otherwise
not need.

### Connecting it to GitHub

Nothing to do. Because `gh` is authenticated as `yourname` from [b4](#b4-github-and-gh),
Codex can run `git` and `gh` commands with your credentials to commit, push, and open pull
requests. The cloud version has its own setup for giving a remote environment access to your
repositories; local terminal work needs none of it.

### Before you build anything real

Turn off the commit attribution. Codex historically added none, then gained a config key for
it, so the guarantee is worth setting once: [b8](#b8-turn-off-ai-attribution).

### Keeping current

Both agents ship changes weekly. That is why this card carries a weekly freshness budget and
shows a badge quickly, and why every command here names the official documentation next to
it. When this card and the docs disagree, the docs are right.
