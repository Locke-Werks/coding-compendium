---
id: e1-what-an-agent-is
title: What an agent is, and how it differs from a chatbot
type: section
track: E
order: 10
verified: 2026-08-02
volatility: low
answer: >
  An agent is a model with hands: it reads your files, writes them, and runs
  commands on your machine, so its mistakes land on disk before you have read a
  word of them.
owns:
  - agent
  - tool use
  - the action loop
see_also:
  - e7-agent-failure-modes
  - a3-the-three-pieces
  - e3-plan-mode
  - e11-what-to-never-let-an-agent-do
keywords:
  - agent vs chatbot
  - what is claude code
  - does it run commands
  - can it edit my files
  - tool use
  - does it actually do things
---

## More

A chatbot hands you text and you copy it somewhere yourself. An agent does the copying. It
reads files on your disk, writes new ones, edits existing ones, runs commands in your
terminal, and talks to GitHub using your saved credentials. That is the only difference
that matters, and it is both the whole capability and the whole risk.

A session has a shape. You state a goal. The agent picks one action, runs it, reads the
result, picks the next one, and repeats until it decides it is finished or hits something
it needs you for. Each of those steps is a **tool call**: read this file, run this command,
search the project for this word. The text scrolling past you is the agent narrating that
loop.

Four things follow, and they set up the rest of this track.

**It acts before you review.** By the time you read "I have updated the login handler," the
file on disk already changed. Your undo is git, not the agent. See
[d10](#d10-undo-everything).

**It runs as you.** Your file permissions, your GitHub login, your ability to delete a
folder. Claude Code asks before most actions and can be told to stop asking. Codex starts
inside a sandbox that limits what it can touch, controlled by `sandbox_mode`, which
[b7](#b7-install-codex) covers. Neither of those is a wall.

**It forgets.** Close the session and everything it worked out about your project is gone
unless it was written to a file. That is what `CLAUDE.md` is for
([e4](#e4-claude-md-and-agents-md)) and why a long session gets worse rather than better
([e2](#e2-context-windows)).

**It sounds the same when it is wrong.** A correct action and a broken one are narrated in
identical tone. There is no tell in the writing, which is why
[e7](#e7-agent-failure-modes) exists.

Same engine, three surfaces. The CLI (Command-Line Interface) runs in your terminal. An
extension runs inside an IDE (Integrated Development Environment) such as Visual Studio
Code. A cloud version runs on the vendor's servers, kicked off from a browser. Start with
the terminal one. It is the most capable of the three.

## Full

### The loop, annotated

A short session looks roughly like this:

```text
> add a health check endpoint to the server

  Read    src/server.js  (142 lines)
  Search  "app.get(" in src/
  Edit    src/server.js  +8 -0
  Bash    npm test
          14 passing, 0 failing

  Done. Added a /health route that returns {"status":"ok"}.
```

Four tool calls and one sentence written for you. The four are the machine's own log: what
it looked at, what it changed, what it ran, what came back. The last line is the only part
composed for a human reader.

The habit worth forming immediately: read the tool calls, not the summary. The summary is
where an agent's account of itself and reality come apart, and that gap has a name and a
tell in [e7](#e7-agent-failure-modes).

### The tools it actually has

Strip away the branding and the list is short: read a file, write or edit a file, run a
shell command, search the project, fetch a web page, and whatever extra tools you connected
through MCP (Model Context Protocol), which [e9](#e9-mcp) covers.

Run-a-shell-command is the one that matters. It is a blank check. Every other capability is
a specific verb; that one is "anything you can type." Installing a package, deleting a
folder, pushing to GitHub, and starting a server are all the same tool from the agent's
side.

### Permission, and what turning it off buys

Both tools sit somewhere on a slider between asking about everything and asking about
nothing. The tradeoff is real in both directions. Approving every single edit is slow, and
by the twentieth prompt you are pressing yes without reading, which is worse than
auto-accept because you now believe you reviewed something. Auto-accept is fast and you
find out afterward.

The workable middle: plan mode before the work starts ([e3](#e3-plan-mode)), auto-accept on
file edits inside the project, and a prompt on shell commands. Whatever you choose, the
list in [e11](#e11-what-to-never-let-an-agent-do) stays manual.

### What it cannot do

- **It cannot remember across sessions.** Anything durable goes in a file
  ([e4](#e4-claude-md-and-agents-md)).
- **It cannot see your screen.** Only what it reads and what you paste. If your dev server
  is throwing errors in another window, the agent does not know until you tell it
  ([f5](#f5-what-to-paste-and-what-not-to)).
- **It cannot know a test passed unless something ran it.** Neither can you
  ([h1](#h1-what-a-test-is)).
- **It cannot audit itself.** Ask what it did and you get a plausible account of the
  transcript, which is not the same as a record of what happened.

### The trust boundary

The agent runs with your identity. Because `gh` is authenticated as `nyxlocke`, a `gh`
command from the agent is you opening a pull request. Commits carry the name and email you
set in [b3](#b3-tell-git-who-you-are). Nothing marks the work as machine-written unless you
configure it to, and [b8](#b8-turn-off-ai-attribution) is about deliberately removing what
little marking exists.

That is the intended design. It is also the reason the review step belongs to you: there is
no second party in this system. There is you, and there is a process running as you.

### "Agent" is a spectrum, not a category

The word covers a wide range, and people arguing about agents are usually standing at
different points on it:

1. Autocomplete that finishes the line you are typing.
2. A chat window that writes code you paste yourself.
3. An agent that edits files and runs commands, pausing for approval.
4. The same thing with approvals turned off.
5. The same thing again with MCP servers reaching into your database, your issue tracker,
   and the open internet.

Everything from step three down is what this track means. When somebody tells you an agent
did something impressive or something horrifying, the useful question is which of those
five they were running.

Next: [e3](#e3-plan-mode), which is the habit that makes step three safe enough to enjoy.
