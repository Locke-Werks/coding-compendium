---
id: l3-your-first-codex-session
title: Your first session with Codex
type: section
track: L
order: 30
verified: 2026-08-25
volatility: weekly
verify: codex --version
answer: >
  Commit first, `cd` into the project, run `codex`, and know that two separate settings govern
  what happens next: the sandbox decides what it can touch at all, and the approval policy
  decides when it stops to ask you.
owns:
  - the first Codex session
  - approval in practice
see_also:
  - b7-install-codex
  - l2-your-first-claude-code-session
  - e10-using-two-agents
  - l1-your-first-repository
  - e4-claude-md-and-agents-md
keywords:
  - first codex session
  - how to use codex cli
  - openai codex walkthrough
  - codex approval
  - codex sandbox
  - codex getting started
  - what do i type in codex
  - codex vs claude code
---

## More

Installation and the config file are [b7](#b7-install-codex). This is the session itself.

The shape is the same as any agent: commit first, start it in the project, ask for one small
thing, read the diff yourself. What is specific to Codex is that it runs inside a sandbox by
default, and the sandbox is a separate control from the approval prompt.

**Start clean.**

```powershell
git status
```

`nothing to commit, working tree clean` means a bad result costs you one `git restore .`
instead of an afternoon.

**Start it in the project.**

```powershell
cd ~/dev/first-project ; codex
```

It opens a text interface in your terminal and shows the current model and the current
sandbox mode. Read that line. It is the difference between an agent that can edit your files
and one that can only look.

**Ask for something small.**

```text
Add a "Running this" section to README.md with one powershell code
block showing how to start the project.
```

**Understand what stops it.** Two things can, and they are not the same:

- **The sandbox** is a wall. `read-only` means it physically cannot write, no matter what
  either of you wants. `workspace-write` lets it edit inside the project folder and still
  blocks the network.
- **The approval policy** is a question. It decides whether Codex pauses to ask before doing
  something it is allowed to do.

A tight sandbox with no questions is safe and quiet. A loose sandbox with no questions is the
configuration people regret. [b7](#b7-install-codex) has the exact values.

**Check the work.**

```powershell
git diff
```

Same rule as every agent: the summary is a claim, the diff is the evidence.

## Full

### The first-run prompts

The first time you start it in a folder, expect two questions before you get to work.

**Sign in.** Choose "Sign in with ChatGPT" and approve in the browser, unless you are using an
API (Application Programming Interface) key, which [b7](#b7-install-codex) covers.

**Trust this folder.** Codex asks whether it should trust the directory you started it in.
Saying yes records that folder as trusted so it stops asking. Say yes for your own projects.
Do not say yes inside a repository you downloaded and have not read.

### Reading the approval prompt

When it wants to do something the approval policy covers, it stops and shows the exact
command or the exact patch. Read the command, not the description of the command. Two things
are worth a second look every time:

- **Anything with a path outside the project.** An edit to a file in your home folder is a
  different category from an edit inside the repo.
- **Anything that reaches the network.** Installing a package pulls in code you have not read.
  [g7](#g7-dependency-risk) covers why that matters more than it sounds.

Approving once is a decision about one action. Approving a category is a decision about every
future action of that shape, made before you have seen any of them.

### Stopping it

`Esc` interrupts the current turn. `Ctrl+C` twice exits. Files already written stay written,
which is why the working tree was clean before you started.

If it loops on the same failing fix, stop it rather than letting it try again. Each retry
adds the failed attempt to the context, which makes the next attempt worse rather than
better. [e7](#e7-agent-failure-modes) has the pattern.

### The instruction file

Codex reads `AGENTS.md` at the project root as standing instructions for every session, the
same role `CLAUDE.md` plays for Claude Code. One project can have both, and keeping them
saying the same thing is a small chore worth doing.
[e4](#e4-claude-md-and-agents-md) owns what goes in them.

### Running it alongside Claude Code

Normal, and useful. They are different engines with different failure modes, and a second
opinion on a change is cheap. The practical rule is one agent per working tree at a time: two
agents editing the same files in the same folder produces conflicts that neither of them
understands, because each sees the other's edits as changes nobody made.

[e10](#e10-using-two-agents) covers the ways to run both without that happening.

### The differences you will actually notice

Both agents read your project, propose changes, run commands, and ask permission. Set
expectations at the level that survives a version bump.

- **Codex sandboxes by default and says so in the interface.** Claude Code's controls are
  permission prompts rather than a wall. Neither approach is stricter in practice; they put
  the control in a different place.
- **The instruction files are `AGENTS.md` and `CLAUDE.md`.** Same job, different name.
- **The settings formats differ.** Codex uses TOML (Tom's Obvious, Minimal Language) at
  `~/.codex/config.toml`. Claude Code uses JSON (JavaScript Object Notation) in
  `settings.json`. [b9](#b9-where-settings-live) has both locations and which level wins.

Anything more specific than that changes on a schedule. Model names especially: take the
current one from the picker in the app rather than from any written source, including this
one. https://developers.openai.com/codex is the authority when this card and the docs
disagree.
