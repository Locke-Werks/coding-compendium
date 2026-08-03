---
id: claude-cli
title: claude
type: command
verified: 2026-08-02
volatility: weekly

tool: claude
command: claude
shell: powershell

verify: claude --version

does: >
  Starts an interactive Claude Code session in the folder you are standing in, giving the agent
  access to the files in that folder.

flags:
  - flag: '-p "<prompt>"'
    means: >
      Short for `--print`. Runs one prompt, prints the answer, and exits, with no interactive
      session. This is how you use it inside a script or a pipeline.
  - flag: "-c"
    means: Short for `--continue`. Resumes the most recent conversation in this folder instead of starting fresh.
  - flag: "-r"
    means: Short for `--resume`. Lists your earlier sessions so you can pick one to reopen.
  - flag: "--model <name>"
    means: Chooses which model to use for this session, overriding your configured default.
  - flag: "--permission-mode plan"
    means: >
      Starts in plan mode, where the agent works out what it intends to do and shows you before
      touching anything. The best habit available, because reading a plan is far cheaper than
      reviewing a diff.
  - flag: "--dangerously-skip-permissions"
    means: >
      Turns off every approval prompt, so the agent edits files and runs commands without asking.
      The name is the warning.

danger: >
  `--dangerously-skip-permissions` lets the agent run any command in your folder without asking,
  which includes commands that delete files or rewrite git history. Only use it in a repository
  where everything is committed and pushed, so a bad outcome costs you a `git reset` rather than
  your work.

expect: >
  A welcome banner naming the version and the working folder, then a prompt box waiting for
  input. Leave with `/exit` or Ctrl+C twice.

see_also:
  - claude-version
  - claude-init
  - claude-clear
  - b6-install-claude-code
  - e3-plan-mode

keywords:
  - start claude code
  - launch the agent
  - claude cli
  - open claude in this folder
---

The folder you start it in is the folder it can see. Move into your project first with
`Set-Location`, or the agent will read the wrong files and be confidently unhelpful about them.

These tools ship changes weekly. If a flag here behaves differently, the official documentation
is the source of truth.
