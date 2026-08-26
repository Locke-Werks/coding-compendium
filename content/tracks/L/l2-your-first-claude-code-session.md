---
id: l2-your-first-claude-code-session
title: Your first session with Claude Code
type: section
track: L
order: 20
verified: 2026-08-25
volatility: weekly
verify: claude --version
answer: >
  Commit whatever you have, `cd` into the project, run `claude`, ask for one small change,
  read the permission prompt before approving it, then check the result with `git diff`
  rather than by trusting the summary.
owns:
  - the first Claude Code session
  - the permission prompt in practice
see_also:
  - b6-install-claude-code
  - e3-plan-mode
  - e5-prompting-that-works
  - l1-your-first-repository
  - h3-reviewing-a-diff-you-cannot-fully-read
keywords:
  - first claude code session
  - how to use claude code
  - claude code walkthrough
  - permission prompt
  - claude code approve
  - what do i type in claude code
  - agent asks permission
  - claude code getting started
---

## More

Installation is [b6](#b6-install-claude-code). This is the ten minutes after that.

**Start from a clean, committed state.** Not a formality. The agent edits real files, and
`git` is what lets you throw away a bad result in one command instead of reconstructing what
you had.

```powershell
git status
```

If it says `nothing to commit, working tree clean`, you are ready. If not, commit first.

**Start it from the project root.**

```powershell
cd ~/dev/first-project ; claude
```

The folder you launch from is the folder it can see. Launching from your home directory is
how people end up with an agent that cannot find files that are plainly there.

**Ask for something small and specific.** Your first request should be something you can
verify in ten seconds:

```text
Add a section to README.md called "Running this", with a single
powershell code block showing how to start the project.
```

Name the file. Say what done looks like. [e5](#e5-prompting-that-works) covers why that
phrasing works better than "improve the readme".

**Read the permission prompt.** Before it changes anything, it stops and shows you what it
wants to do, with the exact edit or the exact command. Three answers: yes once, yes and stop
asking for this kind of thing, or no.

Say yes once, at first, every time. The "stop asking" option is convenient and it is how you
end up not knowing what happened. You can loosen it later, when you know what you are
loosening.

**Check the work yourself.**

```powershell
git diff
```

Shows every line that changed. The agent will tell you what it did, and that summary is a
claim rather than evidence. The diff is the evidence.
[h3](#h3-reviewing-a-diff-you-cannot-fully-read) covers reading one when you cannot follow
every line.

**Then commit it, or throw it away.** If the diff is right, commit. If it is wrong,
`git restore .` puts everything back and costs you nothing, which is why step one mattered.

## Full

### What the screen is showing you

You are in a text interface running inside your terminal. It scrolls. Four things happen in
it and they look different from each other.

- **Your messages**, which you type at the bottom and send with Enter.
- **Its reasoning and replies**, as ordinary prose.
- **Tool calls**, where it reads a file, runs a command, or edits something. These are the
  lines that actually do things.
- **Permission prompts**, which stop everything and wait for you.

Slash commands control the tool rather than the conversation. `/help` lists them all, and
[b6](#b6-install-claude-code) has the ones worth knowing on day one.

### The three habits that matter in week one

**Plan before edit.** Ask for a plan first on anything larger than a one-file change. Reading
a plan takes a minute and reading a wrong diff takes twenty.
[e3](#e3-plan-mode) owns this and it is the single best habit available.

**Commit between steps, not at the end.** A commit is a save point. Three small commits let
you keep two of them when the third turns out wrong. One large commit is all or nothing.
[d4](#d4-commit-well) covers writing them.

**Clear the context when the session gets long.** A long conversation gets measurably worse,
because everything said so far competes for the model's attention. `/clear` starts fresh in
the same folder. [e6](#e6-when-to-reset-context) covers when.

### Stopping it

Two different stops, and knowing which you want saves a mess.

**Escape** interrupts what it is currently doing and hands the conversation back to you. Use
it the moment you see it heading somewhere you did not intend. It is not destructive and it
does not end the session.

**Ctrl+C twice** exits the program entirely. Files it already changed stay changed, which is
what `git restore` is for.

The failure to watch for is the agent that keeps trying the same broken fix. Stop it. Every
additional attempt adds wrong context that makes the next attempt worse.
[e7](#e7-agent-failure-modes) has the shapes this takes.

### The first thing to set up, once

Run this inside a project once:

```text
/init
```

It reads the project and writes a `CLAUDE.md` file describing what the project is and how it
is built. Every future session reads that file first, so the agent starts knowing your
conventions instead of guessing at them. Edit the file freely; it is yours.
[e4](#e4-claude-md-and-agents-md) covers what belongs in it.

### What it can do without asking, and what it cannot

By default it asks before writing a file and before running a command. It reads freely.

There is a mode that stops asking entirely. It exists for automation. Running it on your own
machine on a project you care about trades away the one control you have, and the argument
for it is always convenience. [e11](#e11-what-to-never-let-an-agent-do) is the list of things
worth keeping a hand on regardless of mode.

The other boundary is network access. When it runs `git push`, that is a real push to a real
remote under your credentials, because `gh` is authenticated as you
([b4](#b4-github-and-gh)). An agent opening a pull request is doing it as you, and the
commits carry your name.

### Turn off the attribution before you commit anything

Claude Code has historically added a trailer to commit messages and a line to pull request
descriptions. Whether you want that in your history is your call, and it is a setting rather
than something to edit out by hand afterward. [b8](#b8-turn-off-ai-attribution) covers all
three places it is configured.

### When the version here disagrees with the tool

This card carries a weekly freshness budget, which means it goes yellow fast on purpose.
These tools ship changes weekly and flags move. When the app and https://docs.claude.com
disagree, the documentation is right.
