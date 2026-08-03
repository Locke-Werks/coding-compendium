---
id: f6-when-the-agent-loops
title: When the agent is stuck in a loop
type: section
track: F
order: 60
verified: 2026-08-02
volatility: quarterly
answer: >
  Three "try this instead" attempts with no stated diagnosis means the agent is
  guessing, so stop it editing, make it say what it now thinks the cause is and
  name one command that would confirm it, and reset the session if it cannot.
owns:
  - breaking a fix-fail-fix cycle
see_also:
  - e6-when-to-reset-context
  - e7-agent-failure-modes
  - e10-using-two-agents
  - f7-reproducing-a-bug
  - d10-undo-everything
keywords:
  - stuck in a loop
  - it keeps trying things
  - fix fail fix
  - it apologized again
  - going in circles
  - same error over and over
  - it broke something else
---

## More

A working agent narrows. Each attempt rules something out and it tells you what. A looping
agent cycles: it changes something, the same error comes back, it apologizes, it changes
something else, often changing the first thing back.

The tells, roughly in order of how conclusive they are:

- **The error text has not changed at all** across three attempts. Not shorter, not moved.
  Identical.
- **It edits the same file back and forth.** `git diff` shows the file returning to where it
  started.
- **It never states a cause.** Every message is a proposed fix and none is a diagnosis.
- **It starts wrapping things.** A `try`/`catch` or `try`/`except` appears around the failing
  call. That hides the error rather than fixing it.
- **It edits the test instead of the code**, or deletes it ([h6](#h6-when-tests-lie)).
- **It suggests reinstalling everything**, deleting `node_modules`, or starting the project
  over. That is the sound of an agent with no hypothesis left.

Stop it there, before the pile of half-fixes gets big enough to be its own problem. Type
this:

```text
Stop. Do not edit any files.
In one paragraph: what do you now think the cause is, and what evidence supports it?
Then give me one command that would confirm or rule that out.
```

Two things happen. Sometimes it produces a real diagnosis, and the next fix works, because
being asked to explain forces it to actually use what is in front of it. Sometimes it
produces another guess in the shape of an explanation, and now you can see that, which is the
information you were missing.

Then run the command yourself and read the output yourself. That single step breaks more
loops than anything else, because you are now the one holding evidence instead of the one
holding a summary of evidence.

## Full

### Before you continue, see what has been done to your project

Three attempts leaves a mess, and debugging on top of a mess is why the third fix behaves
differently from the first.

```powershell
git status
```

Shows every file touched since your last commit. If files you never mentioned are in that
list, that is scope creep and [e7](#e7-agent-failure-modes) has the catalog.

```powershell
git diff
```

Shows what actually changed. Read it before deciding whether to keep any of it.

If none of the attempts helped, get back to a known state before starting again:

```powershell
git stash push -u -m "failed attempts"
```

That puts every change, including new untracked files, into storage and gives you a clean
folder. Nothing is lost and `git stash pop` brings it back. [d10](#d10-undo-everything) covers
the full set of ways to undo, ranked by cost.

This is also the argument for committing before you ask for anything. A checkpoint makes the
whole question "keep or discard" free.

### The three prompts that break a loop

**Force a diagnosis.**

```text
Stop. Do not edit any files. In one paragraph, what do you now think the cause is,
and what specific evidence in the error supports that?
```

**Force a prediction.**

```text
If that were true, what would I see when I run something? Give me one command,
tell me what output would confirm it, and what output would rule it out.
```

A guess cannot answer this question well. A real hypothesis can. This is the single most
useful thing in this card.

**Force it to consider being wrong.**

```text
Assume your last three fixes were all addressing the wrong thing. What else could
produce this exact error?
```

Agents are agreeable by default and will keep refining a wrong answer for as long as you let
them. Explicitly licensing a change of direction is often all it takes.

### Give it what it has been working without

A loop is frequently an information problem rather than a reasoning problem. Check whether it
actually has:

- The full error, unedited, including the frames it would not have guessed
  ([f5](#f5-what-to-paste-and-what-not-to)).
- The real command you ran and the folder you ran it in.
- The output of the last fix, rather than your summary of it.
- The relevant file, when the agent cannot read files itself.

An agent working from your paraphrase of an error is guessing by construction.

### When to stop talking to this session

If a forced diagnosis produces another guess, the session itself is the problem. A long
conversation full of failed attempts poisons the next attempt, because all of that wrong
reasoning is still in front of the model and it reads as context rather than as a record of
failure.

Start a fresh session, and carry over only: the goal, the current error, and what has been
ruled out. [e6](#e6-when-to-reset-context) covers the handoff so a reset costs you nothing.

Two related moves:

- **Ask the other agent.** A different model fails in different places, and a second opinion
  on a stuck bug is cheap. [e10](#e10-using-two-agents).
- **Shrink the problem.** Get the failure into the smallest thing that still fails, then hand
  that over instead of the project. [f7](#f7-reproducing-a-bug).

### The possibility nobody checks

The agent may be fixing code that is not broken. A large share of stubborn failures are
environmental, and no edit to your source will ever move them:

- The command is not found because the terminal predates the install
  ([c4](#c4-path-and-command-not-found)).
- The library is installed in a different environment than the one running
  ([g4](#g4-environments-and-isolation)).
- An old process is still running and still serving the previous version of your code
  ([c5](#c5-processes-and-killing-them)).
- A port is held by something else entirely ([c6](#c6-ports-and-localhost)).

If the agent has changed four files and the error has not moved a character, stop and check
these four before anything else. The tell is that the error is identical, because a code
change that touched the real path would normally change the error somehow, even for the
worse.

### Set the budget before you start

Decide the number in advance, when you are calm. Three attempts, then you stop and force a
diagnosis. It is a rule that only works if you set it before the fourth attempt starts to
feel reasonable.

Every loop also re-sends the whole conversation to the model, so a long loop costs real money
and gets slower as it goes ([e8](#e8-tokens-and-cost)). The budget is not only about your
patience.

### The version where you are the loop

Accepting a fix you cannot verify, then asking for another when it does not work, is the same
cycle with you driving. The break is identical: stop, state what you actually know, and get
one piece of evidence before the next change. "It still doesn't work" restarts the loop.
"Same error, identical text, and `git diff` shows only `server.js` changed" ends it.
