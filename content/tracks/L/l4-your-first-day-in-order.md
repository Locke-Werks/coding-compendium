---
id: l4-your-first-day-in-order
title: Your first day, in order
type: section
track: L
order: 40
verified: 2026-08-25
volatility: quarterly
verify: gh auth status
answer: >
  Install five things, make one repository, run one agent, ask for one small change, review
  the diff, commit, push. Everything after the first day is that last half repeated.
owns:
  - the day-one checklist
see_also:
  - a4-the-loop
  - l1-your-first-repository
  - l2-your-first-claude-code-session
  - k3-the-first-ten-commands
  - b4-github-and-gh
keywords:
  - getting started checklist
  - what do i do first
  - setup order
  - day one
  - vibe coding quick start
  - beginner checklist
  - where do i start
  - first steps
---

## More

A checklist rather than an explanation. Each line links to the card that covers it properly.
Work down the list. Do not skip the checks.

**Install, once.** Roughly an hour, most of it waiting.

1. Windows Terminal, already on your machine. [b1](#b1-terminal-shell-command-line)
2. Git. Check: `git --version` prints a number. [b2](#b2-install-git)
3. Your name and email in git. Check: `git config --global user.name` prints your name.
   [b3](#b3-tell-git-who-you-are)
4. The GitHub tool. Check: `gh auth status` prints a green check. [b4](#b4-github-and-gh)
5. An agent. Claude Code ([b6](#b6-install-claude-code)) or Codex
   ([b7](#b7-install-codex)), or both. Check: `claude --version` or `codex --version`.
6. Turn off the AI (Artificial Intelligence) attribution the agents add to commit messages,
   before your first commit rather than after. [b8](#b8-turn-off-ai-attribution)

**Make something, once.** Twenty minutes.

7. One repository, empty folder to code on github.com. [l1](#l1-your-first-repository)
8. Start the agent in that folder and ask for one small change.
   [l2](#l2-your-first-claude-code-session) or [l3](#l3-your-first-codex-session)
9. Read `git diff` before you believe the summary.
   [h3](#h3-reviewing-a-diff-you-cannot-fully-read)
10. Commit and push. [d4](#d4-commit-well)

**Then repeat steps 8 through 10 forever.** That is the job.
[a4](#a4-the-loop) explains why it is shaped this way.

## Full

### The four commands you will type most

Not the four most important. The four you will actually run dozens of times a day, worth
getting into your fingers early.

```powershell
git status
```

What has changed and what is staged. Run it before and after everything.
Tab and the up arrow save most of the typing: [k6](#k6-history-completion-and-keys).

```powershell
git diff
```

The lines that changed. This is the one people skip and should not.

```powershell
git add . ; git commit -m "<what changed and why>"
```

Save point. Small and frequent beats large and careful.

```powershell
git push
```

Send it to GitHub. After the first `git push -u origin main`, the bare form works.

[k3](#k3-the-first-ten-commands) covers the console commands underneath these, for moving
around and looking at files.

### What to do when something breaks on day one

Three failures account for most first days, and none of them mean you did anything wrong.

**A command is not recognized, right after you installed it.** The terminal was open before
the install and has not noticed. Close it, open a new one, run the check again.
[c4](#c4-path-and-command-not-found) explains why that works, and it is worth reading once so
you stop being surprised by it.

**A command from a tutorial fails with a strange complaint about a parameter.** You are in a
different shell than the person who wrote it, or the name means something else here.
[k7](#k7-same-word-different-program) has the list.

**The agent changed something you did not want.** The working tree was clean before you
started, so:

```powershell
git restore .
```

Puts every tracked file back to the last commit. It throws away uncommitted work, which is
exactly what you want here and exactly what you do not want at any other time.
[d10](#d10-undo-everything) covers the whole undo ladder, and
[d11](#d11-when-you-lose-work) is the card for when something is genuinely lost.

### What not to do on day one

Short list, and each one costs a real afternoon.

- **Do not turn off the permission prompts** because they are slowing you down. They are the
  slowing-down. [e11](#e11-what-to-never-let-an-agent-do)
- **Do not commit before looking at `git status`.** The one irreversible mistake here is
  committing a password or a key, and deleting the file afterward does not remove it from the
  history. [g6](#g6-secrets-and-what-never-to-commit)
- **Do not start on a project that matters.** Make something disposable first. The learning
  is in recovering from the mistakes, and you want the first ten in a folder you can delete.
- **Do not run five commands pasted as one block.** Run them one at a time so you find out
  which one failed. [k6](#k6-history-completion-and-keys)

### Week one, once day one works

In rough order of how soon each one starts paying:

1. **Plan mode.** Ask for a plan before a change larger than one file.
   [e3](#e3-plan-mode)
2. **A project instruction file.** `/init` writes one. Every later session starts from it.
   [e4](#e4-claude-md-and-agents-md)
3. **Branches.** Work on a branch, keep `main` working.
   [d5](#d5-branches)
4. **A first test.** One test that fails when the thing is broken is worth more than reading
   every line of a diff. [h1](#h1-what-a-test-is)
5. **Reading an error properly**, top line first, rather than pasting the whole wall of text
   at the agent. [f1](#f1-how-to-read-an-error-message)

None of that is required to start. All of it is required to keep going without the whole
thing getting away from you.
