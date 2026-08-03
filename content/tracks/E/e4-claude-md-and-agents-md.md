---
id: e4-claude-md-and-agents-md
title: CLAUDE.md and AGENTS.md
type: section
track: E
order: 40
verified: 2026-08-02
volatility: quarterly
answer: >
  These are standing instructions the agent reads at the start of every session,
  so they hold the commands, conventions, and never-do rules you are tired of
  repeating, and they work better short than thorough.
owns:
  - project instruction files
  - /init
see_also:
  - e5-prompting-that-works
  - b9-where-settings-live
  - e6-when-to-reset-context
  - e2-context-windows
keywords:
  - claude md
  - agents md
  - init command
  - project instructions
  - standing rules
  - memory file
---

## More

`CLAUDE.md` and `AGENTS.md` are plain text files at the root of your project. The agent
reads one at the start of every session, before it reads anything else, and treats the
contents as standing instructions. `CLAUDE.md` is Claude Code's. `AGENTS.md` is an open
convention that Codex and several other tools follow. Having both is normal.

Running `/init` inside a Claude Code session writes a first draft by scanning the folder.
Treat what comes back as a draft. It describes your project accurately and says nothing
about what you want, because it cannot know that yet.

What belongs in one:

- **The commands.** How to install, how to run, how to test, how to lint. Exact strings,
  not descriptions.
- **Conventions you care about.** Commit message format, where new files go, which package
  manager, how dates are formatted.
- **Never-do rules, flatly stated.** "Never edit anything under `vendor/`." "Never add a
  dependency without asking."
- **Project vocabulary.** What the three services are called, what a "run" means in your
  domain, which folder is generated.

What does not belong:

- **Secrets, of any kind.** This file gets committed and often pasted
  ([g6](#g6-secrets-and-what-never-to-commit)).
- **Anything the README already says.** Point at it in one line instead.
- **General programming advice.** It already knows what a function is.
- **Anything that will be wrong next week.** Version numbers of things you upgrade often,
  the current status of a task, who is working on what.

The cost nobody mentions: every line is re-sent at the start of every session and occupies
the context window for the whole of it ([e2](#e2-context-windows)). A four-hundred-line
instruction file crowds out the thing you actually asked about, and it gets followed less
faithfully than a forty-line one. Short and specific beats long and thorough, every time.

It is a living document, and one rule keeps it honest: the second time you correct the
agent about the same thing, stop correcting and write it down.

## Full

### A working example

This is a whole file, and it is about the right length:

```markdown
# Project: invoice-tool

A Python 3.12 command-line tool that turns spreadsheet exports into printable invoices.

**Commands**
- Install: `uv sync`
- Run: `uv run invoice-tool --input data/sample.csv`
- Test: `uv run pytest`
- Lint: `uv run ruff check .`

**Conventions**
- Conventional commits: feat, fix, docs, refactor, test, chore
- New modules go in src/invoice_tool/, tests mirror the path under tests/
- Dates are always written as 2026-08-02, never localized
- Ask before adding any dependency

**Never**
- Never commit anything under data/ except data/sample.csv
- Never reformat a file you were not asked to change
- Never weaken or delete a test to make a suite pass
- Never add AI attribution to commits or pull requests
```

Around thirty lines, and every line is either a command or a rule. Nothing in it explains
Python, describes the architecture at length, or tells a story. Those last three "Never"
lines each exist because of a specific failure in [e7](#e7-agent-failure-modes), and each
one is worth its space.

### Levels, and which one wins

- **Project root.** The main one. Committed, shared, applies to everyone working in the
  repository.
- **User level**, at `C:\Users\<yourname>\.claude\CLAUDE.md`. Your personal standing rules,
  applied to every project on your machine.
- **Subdirectory files.** Read when the work touches that directory. Useful in a big
  project where the front end and the back end have different rules.

Narrower wins where they conflict, the same precedence as the settings files in
[b9](#b9-where-settings-live).

Hold on to one distinction while you are in that neighborhood. A settings file configures
the tool: permissions, model, hooks. An instruction file talks to the model. A permission
rule is enforced by software. An instruction is read and usually followed. Those are very
different guarantees, and putting a safety-critical rule in the wrong one is a common and
expensive mistake.

### Do not maintain two files

Pick one as the real file and make the other a single line pointing at it. Two instruction
files that have drifted apart is worse than either alone, because you will edit whichever
one you opened last and never know which the agent read.

Which one is real is your call. `AGENTS.md` is understood by more tools; `CLAUDE.md` is
what Claude Code looks for first. Either way the pointer costs one line and removes a whole
category of confusion.

### The honest limit

These files are strong suggestions. They are not rules.

The agent reads the file, then reads everything since, and does whatever the whole pile
pulls it toward. By hour three the instruction is far up a long transcript, competing with
everything that arrived after it ([e2](#e2-context-windows)). Nobody should be shocked when
a "Never" line gets stepped over late in a session. It happens, it is not malice, and the
file is still worth having because it raises the odds on every early turn.

What actually enforces a rule, in increasing order of strength: a line in this file,
a permission or sandbox setting ([b9](#b9-where-settings-live),
[b7](#b7-install-codex)), a git hook ([b8](#b8-turn-off-ai-attribution)), a test that
fails, and you reading the diff ([h3](#h3-reviewing-a-diff-you-cannot-fully-read)).

### Keeping it current

Three maintenance habits, all cheap:

- Add a line the second time you correct the same behavior.
- Delete a line the moment it stops being true. A stale instruction is worse than no
  instruction, because it gets followed.
- Once a month, ask the agent: "read `CLAUDE.md` and tell me which lines are stale,
  redundant, or contradict each other." It is genuinely good at this and it costs one turn.

The file you want after six months is shorter than the one you had after two, because
you learned which rules actually mattered and deleted the rest.
