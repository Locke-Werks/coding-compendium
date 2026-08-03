---
id: e6-when-to-reset-context
title: When to start a fresh session
type: section
track: E
order: 60
verified: 2026-08-02
volatility: quarterly
answer: >
  Start a fresh session when the agent forgets a decision, reintroduces a fixed
  bug, or argues with itself, and make the reset free by asking for a handoff
  note before you clear rather than after.
owns:
  - clearing context
  - session hygiene
  - handoff notes
see_also:
  - e2-context-windows
  - e4-claude-md-and-agents-md
  - f6-when-the-agent-loops
  - d4-commit-well
keywords:
  - clear command
  - compact
  - start over
  - fresh session
  - it forgot everything
  - handoff note
  - new conversation
---

## More

Six signs the session is finished and you are the last to know:

1. It forgets a decision you made in this same session and re-proposes the thing you
   already ruled out.
2. It reintroduces a bug it fixed an hour ago.
3. It contradicts itself inside a single reply.
4. It reads the same file over and over.
5. Its summary of what happened includes things that did not happen.
6. Three "try this instead" attempts with no stated diagnosis
   ([f6](#f6-when-the-agent-loops)).

None of these mean the model is broken. They mean the transcript it re-reads every turn has
filled up with noise ([e2](#e2-context-windows)). The fix is a new transcript, not a better
prompt.

Reset on purpose too, not only on symptoms. Three good moments: when a task is finished,
when you switch to an unrelated one, and right after you paste something enormous into the
session.

The reset itself, in four moves:

1. **Commit or stash.** Uncommitted work is the only thing a reset can actually cost you
   ([d4](#d4-commit-well)).
2. **Ask for the handoff before you clear.** "Write a handoff note: what we are building,
   what is done, what is in progress, the decisions we made and why, and the exact next
   step." Do this while it still has the context to answer.
3. **Save it somewhere outside the session.** A file, a commit message, a scratch note.
   Anywhere that survives the clear.
4. **Clear, then paste the handoff as your first message.**

Done that way, a reset costs a minute and buys back a sharp agent. Skipped, it costs you
the afternoon you spend re-deciding things you already decided once.

## Full

### What a handoff note actually contains

```text
Handoff note, invoice-tool, 2026-08-02.

Building: a command-line tool that turns spreadsheet exports into printable
invoices. Python 3.12, uv for packages.

Done: the parser reads the sample file. Six tests passing.

In progress: page layout in src/invoice_tool/render.py. The header renders,
the line-items table does not.

Decided: reportlab, because weasyprint needs system libraries that are
painful on Windows. Rejected: generating a web page and printing it, too
fragile to test.

Next step: make the line-items table render. The test to make pass is
tests/test_render.py::test_line_items.

Do not touch: src/invoice_tool/parse.py. It is finished and covered.
```

Notice what it is not. It is not a story of the session. It is decisions with their
reasons, one concrete next step, and one boundary. The reasons are the part that matters,
because a conclusion without its reason gets reopened the first time it is inconvenient.

### The handoff you already wrote

Your commit history. If you commit in small steps with real messages
([d4](#d4-commit-well)), then this is a handoff note you did not have to write:

```powershell
git log --oneline -20
```

A fresh session that reads the last twenty commit subjects and the current `git status`
knows more about the state of the work than a three-hour conversation that has been
compacted twice. This is the practical, selfish argument for committing often, separate
from the safety argument in [a4](#a4-the-loop).

### Clear against compact

Most tools offer both. Compacting summarizes the conversation and continues from the
summary. Clearing throws it away and starts empty.

Compaction is convenient and lossy in one predictable direction, and
[e2](#e2-context-windows) covers exactly what it drops.

Clearing plus your own handoff is better, because you choose what survives instead of
letting a summarizer choose. Use compaction when you are mid-task and cannot stop. Use a
clear at every task boundary.

### What a reset actually costs

It re-reads files, and that costs tokens ([e8](#e8-tokens-and-cost)). The cost is real and
much smaller than people fear.

A fresh session given a two-hundred-word handoff usually reaches a working understanding in
two or three turns. Every one of those turns is cheaper and faster than a single turn on a
bloated transcript, because the bloated one was already paying to re-send three hours of
install logs. The reset frequently pays for itself inside the same task.

The one thing that makes a reset genuinely expensive is uncommitted work with no note. That
is a state you can avoid entirely with thirty seconds of preparation.

### Sort durable facts from session facts

If something came up twice and will come up again next month, it does not belong in a
handoff note. It belongs in `CLAUDE.md` ([e4](#e4-claude-md-and-agents-md)).

- **Handoff notes** are about this task. They expire when it ships.
- **Instruction files** are about this project. They live as long as the repository.

Sorting them that way is what keeps both of them short, and short is what keeps both of
them read.

### End sessions on purpose

The habit underneath all of this: decide when a session ends instead of noticing that it
should have ended two hours ago. The trigger is a finished task and a commit, not an agent
that has started behaving strangely.

Sessions that end on purpose start well. Sessions that end because everything went sideways
begin the next one with you re-explaining a project to a model that has never seen it,
while you are already annoyed.
