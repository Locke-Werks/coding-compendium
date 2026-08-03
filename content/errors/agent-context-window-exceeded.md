---
id: agent-context-window-exceeded
title: "prompt is too long: N tokens > 200000 maximum"
type: error
verified: 2026-08-02
volatility: weekly

category: broke-at-runtime

sample: |
  PS C:\Users\nyx\dev\site> claude
  API Error: 400 {"type":"error","error":{"type":"invalid_request_error","message":"prompt is too long: 217483 tokens > 200000 maximum"}}

patterns:
  - "prompt is too long"
  - "context_length_exceeded"
  - "maximum context length"
  - "tokens > "
  - "Context low"

means: >
  A model can hold a fixed amount of text at once, called the context window, and this session no
  longer fits. Everything counts toward it: every message you have sent, every file the agent has
  read, every command it ran and the output that came back, and the instructions it was given at
  the start. Nothing is broken. The conversation outgrew the container.

fix_ladder:
  - try: Compact the conversation.
    command: /compact
    shell: powershell
    why: >
      Assumes the session is long but its content still matters. In Claude Code this replaces the
      conversation so far with a summary and continues in the same session. You lose the exact
      wording of earlier turns and keep the conclusions, which is the right trade most of the
      time.

  - try: Start a fresh session with a short handoff.
    command: /clear
    shell: powershell
    why: >
      Assumes the session has accumulated a lot that no longer matters, such as three abandoned
      approaches and the output of a long build. Before clearing, ask for a summary of the current
      state and what is left to do, then paste that into the new session. A clean session with
      good notes outperforms a full one.

  - try: Look for one enormous thing in the session.
    why: >
      Assumes a single input blew the budget. A pasted build log, a whole lockfile, a minified
      bundle, or a listing of a dependency folder can each be tens of thousands of tokens. If the
      limit was hit suddenly rather than gradually, this is why.

  - try: Name files instead of pasting them.
    why: >
      Assumes you are pasting whole files into the chat. An agent with file access can read the
      part it needs and skip the rest. Telling it the path costs a handful of tokens, and pasting
      a thousand-line file costs thousands, every turn from then on.

  - try: Move the standing context into a project instruction file.
    why: >
      Assumes you are re-explaining the project each session because a reset loses everything. A
      `CLAUDE.md` or `AGENTS.md` file at the project root is read at the start of every session,
      so a fresh session starts informed and short. See e4-claude-md-and-agents-md.

if_none_worked: >
  Paste the error with the token numbers intact, and say what happened immediately before it. The
  numbers matter: being a few thousand over means compacting is enough, and being double the limit
  means one specific input is responsible. The step before the failure is what identifies it.

see_also:
  - e2-context-windows
  - e6-when-to-reset-context
  - e4-claude-md-and-agents-md
  - e8-tokens-and-cost

keywords:
  - prompt is too long
  - context window exceeded
  - context length
  - compact conversation
  - too many tokens
---

The window is a hard limit rather than a suggestion, and hitting it stops the request outright.

Long before you hit it, the session gets worse. A conversation near the limit produces answers
that miss decisions made an hour ago and reintroduce bugs you already fixed. That is mechanical
rather than the model getting tired: earlier material is competing with everything else for
attention.

So the useful habit is to reset before you are forced to. When an agent forgets a decision or
argues with itself, that is the signal, and it arrives well before the error does.

Handing off well is what makes resetting cheap. Ask for a short summary of what is done, what is
in progress, and what was decided and why. Paste that into the new session. Two minutes of
handoff beats twenty minutes of a degraded session.

The number in the error is tokens, roughly three quarters of a word each for English prose and
considerably more for code. Two hundred thousand tokens is a large book, which is why a session
has to be quite long or quite badly fed to reach it.
