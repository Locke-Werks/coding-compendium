---
id: claude-init
title: /init
type: command
verified: 2026-08-02
volatility: weekly

tool: claude
command: /init
shell: any

does: >
  Asks Claude Code to read through the project and write a standing instruction file the agent
  will load at the start of every future session in this folder.

flags:
  - flag: "/init"
    means: >
      Typed inside a running Claude Code session, not into PowerShell. Slash commands are
      instructions to the agent, and a terminal will report that it cannot find a program by that
      name.
  - flag: "the file it writes"
    means: >
      `CLAUDE.md` in the project root. It holds what the project is, how to build and test it,
      and any conventions worth stating once. Codex reads `AGENTS.md` for the same purpose.
  - flag: "running it again"
    means: >
      Safe. It updates the existing file rather than replacing your edits wholesale, though you
      should read the diff before committing it like any other change.

expect: >
  The agent explores the repository, then writes or updates the instruction file and summarizes
  what it added. `git status` afterward shows the file as new or modified.

see_also:
  - claude-cli
  - claude-clear
  - e4-claude-md-and-agents-md
  - e5-prompting-that-works

keywords:
  - claude md
  - project instructions
  - set up claude for this project
  - slash init
---

Run it once per project, early. What it generates is a starting point rather than a finished
document: the useful version is the one you keep editing as the project teaches you which
instructions the agent actually needed.

Read what it wrote before committing. It is a file in your repository like any other, and it
gets reviewed like any other.
