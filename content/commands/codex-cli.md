---
id: codex-cli
title: codex
type: command
verified: 2026-08-02
volatility: weekly

tool: codex
command: codex
shell: powershell

verify: codex --version

does: >
  Starts an interactive Codex session in the folder you are standing in, giving the agent access
  to the files in that folder.

flags:
  - flag: 'exec "<prompt>"'
    means: >
      Runs one prompt without an interactive session and exits, which is the form to use inside a
      script.
  - flag: "resume"
    means: Reopens an earlier session instead of starting a new conversation.
  - flag: "--model <name>"
    means: Chooses the model for this session, overriding whatever your config file sets.
  - flag: "--sandbox <mode>"
    means: >
      Controls what the agent is allowed to do: read files only, write inside the workspace, or
      reach the network. This is the what-it-can-do setting.
  - flag: "-a <policy>"
    means: >
      Short for `--ask-for-approval`. Controls when it stops to ask you. This is the when-it-asks
      setting, and it is a different question from the sandbox. Mixing the two up is the usual
      source of confusion.

danger: >
  A wide sandbox combined with an approval policy that never asks means the agent edits files and
  runs commands with no checkpoint from you. Commit and push before handing it a large task, so
  the worst case costs you a reset rather than your work.

expect: >
  A prompt waiting for input, with the current model and working folder shown. On first run it
  asks you to sign in with your ChatGPT account.

see_also:
  - codex-version
  - claude-cli
  - b7-install-codex
  - e10-using-two-agents

keywords:
  - start codex
  - openai coding agent
  - codex cli
  - launch codex
---

Settings live in `config.toml` under your home folder. The two knobs people mix up are the ones
above: the sandbox is what it can do, the approval policy is when it asks first.

These tools ship changes weekly, so the official documentation is the source of truth if a flag
here behaves differently.
