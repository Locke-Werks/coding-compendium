---
id: claude-clear
title: /clear
type: command
verified: 2026-08-02
volatility: weekly

tool: claude
command: /clear
shell: any

does: >
  Empties the current conversation so the agent starts from nothing, keeping the session open in
  the same folder.

flags:
  - flag: "/clear"
    means: >
      Typed inside a running Claude Code session, not into PowerShell. It discards the
      conversation only. Your files, your git history, and your standing instruction file are
      untouched.
  - flag: "/compact"
    means: >
      The gentler alternative. It summarizes the conversation so far and keeps the summary,
      which frees room while preserving the decisions you made together.

destructive: true

danger: >
  Everything the agent worked out during this conversation is dropped: the decisions you agreed
  on, the dead ends you already ruled out, the details of a file it read an hour ago. Nothing on
  disk changes, so no code is lost. What you lose is the shared understanding, and the next
  session will happily repeat a mistake you already fixed together.

destroys: >
  The conversation context, immediately. Anything the agent knew but never wrote into a file or
  a commit is gone from the session. Your code, your commits, and `CLAUDE.md` are untouched.

safer_first: >
  Try `/compact` first, which keeps a summary instead of nothing. If you do want a clean start,
  ask the agent to write a handover note into the project first: what it was doing, what it had
  decided, what remains.

undo: >
  `claude --resume` lists earlier sessions and can reopen the one you cleared. Do not count on
  it. Treat anything only the agent knew as gone the moment you press enter.

expect: >
  The screen clears and the conversation history is empty. The agent no longer refers to
  anything you discussed before, which is the confirmation that it worked.

see_also:
  - claude-cli
  - claude-init
  - e6-when-to-reset-context
  - e2-context-windows

keywords:
  - clear claude context
  - fresh session
  - agent is confused
  - reset the conversation
  - compact
---

The signs it is time: the agent forgets a decision you made twenty minutes ago, reintroduces a
bug you already fixed, or argues with itself. Those are mechanical symptoms of a full context
window rather than the model having a bad day.
