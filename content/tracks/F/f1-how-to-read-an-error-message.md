---
id: f1-how-to-read-an-error-message
title: How to read an error message
type: section
track: F
order: 10
verified: 2026-08-02
volatility: low
answer: >
  An error message has a fixed shape: what went wrong, where, and what led up to
  it. Read the last line first, then find the first file path that belongs to
  your own project.
owns:
  - error anatomy
  - reading bottom-up
  - my-code-vs-library
  - cannot-find vs broke-at-runtime
see_also:
  - f2-stack-traces
  - f3-exit-codes-and-streams
  - f5-what-to-paste-and-what-not-to
  - c4-path-and-command-not-found
keywords:
  - red text
  - traceback
  - it says error
  - what does this mean
  - terminal error
---

## More

Red text in the terminal is not a punishment and it is usually not a wall. Almost every
error is built from the same three parts, and once you can see the parts you can act in
about ten seconds.

1. **The type.** A short name for the category of failure, usually first on the line:
   `ModuleNotFoundError`, `TypeError`, `EADDRINUSE`, `fatal:`. This is the part worth
   searching for.
2. **The message.** The specific complaint in human words: `No module named 'requests'`,
   `address already in use`.
3. **The location.** A file path and a line number: `src\app.py, line 42`. When there are
   many locations stacked up, that is a stack trace, and [f2](#f2-stack-traces) covers
   reading those.

Order matters. Terminals print the most important line **last**, at the bottom, right above
your prompt. Start at the bottom and read upward. Most people start at the top, hit a wall
of unfamiliar paths from inside somebody else's library, and give up before reaching the one
line that says what actually happened.

Then ask two questions.

**Is this my code or somebody else's?** Look at the file paths. If every one of them lives
in `node_modules\`, `site-packages\`, or a folder you did not create, the failure surfaced
inside a library but was almost certainly caused by how your code called it. Scan down the
list for the first path inside your own project. That line is usually the real culprit.

**Is this "cannot find" or "found it and it broke"?** Cannot-find errors (`command not
found`, `No module named`, `Cannot find module`, and `ENOENT`, which is how Node spells "no
such file or directory") mean something is not installed or a path is wrong. Those are cheap, and [c4](#c4-path-and-command-not-found) solves a large
share of them by itself. Found-it-and-broke errors (`TypeError`,
`NullReferenceException`, `panic:`) mean the code ran and hit a case it did not expect.
Those need an actual change.

What to do next, in order:

1. Read the last line out loud. Sometimes that is the whole fix.
2. Paste it into Identify. If it is one of the fifty errors you are most likely to meet, you
   get the meaning and a fix ladder immediately.
3. Paste it into your agent **whole and unedited**, along with the command you ran. Do not
   summarize it and do not trim it to the part you think matters. The parts you would trim
   are the parts the agent uses. See [f5](#f5-what-to-paste-and-what-not-to) for the one
   category you must strip first.
4. If the first fix does not work, say so and paste the new error. Do not accept "try this
   instead" three times in a row without asking the agent to state what it now thinks the
   cause is. An agent that is guessing will keep guessing until you stop it.

One habit that pays for itself: before you paste an error anywhere, run `git status` so you
know whether you have uncommitted work. Fixes go faster when you can throw the attempt away.

## Full

### The anatomy, annotated

Here is a real one, with the parts labelled:

```text
Traceback (most recent call last):
  File "C:\Users\nyx\dev\scraper\main.py", line 12, in <module>
    import requests
ModuleNotFoundError: No module named 'requests'
```

- Line 1 is a header telling you a stack trace follows, and which end is which.
- Lines 2 and 3 are the location: your file, your line number, and the actual line of code.
- Line 4 is the type and the message, and it is at the **bottom**, which is the whole reason
  you read upward.

This one is a cannot-find error, in your own file, on an `import`. That combination means
one thing: the library is not installed in the environment you are running. Not a bug in
your code, and not something to ask an agent to rewrite around.

### Warnings are not errors

A warning says something is questionable. An error says something stopped. Terminals print
both in alarming colors and beginners treat them the same, which leads to hours spent
chasing a deprecation notice while the actual failure scrolled past.

The rule: find the thing that **stopped**. If the command finished and produced output, the
red text above it was a warning, however loud it looked. Three warnings you can safely
ignore forever on Windows:

- `npm WARN deprecated <something>` during install. It installed. It works.
- Git's `LF will be replaced by CRLF`. Line endings, covered in
  [c8](#c8-line-endings-and-encoding), and harmless.
- Any warning containing the word `peer` from npm. It is npm arguing with itself about
  version ranges.

### The same failure reads differently in different shells

PowerShell renders errors from its own commands as a block of red with a `+ CategoryInfo`
section underneath. Git Bash renders the same underlying failure as one plain line. The
error is identical; the presentation is not. If you follow instructions written for one
shell and the output looks nothing like the screenshot, check which shell you are in before
concluding something is wrong. [b1](#b1-terminal-shell-command-line) tells them apart.

### A worked example

You run the dev server and get this:

```text
Error: listen EADDRINUSE: address already in use :::3000
    at Server.setupListenHandle [as _listen2] (node:net:1817:16)
    at listenInCluster (node:net:1865:12)
    at Server.listen (node:net:1953:7)
    at C:\Users\nyx\dev\site\server.js:14:8
```

Four steps.

1. **Read the bottom line first.** It is `server.js:14`, which is your file. Except the
   important line here is the top one, because this is a Node error and Node prints the
   message first and the stack after. Knowing which end to read is language-specific, and
   [f2](#f2-stack-traces) is the card that settles it per language.
2. **Find the type.** `EADDRINUSE`. That is the searchable part.
3. **Sort it.** This is neither cannot-find nor a logic bug. It is a resource conflict:
   something else already holds port 3000.
4. **Act.** Almost always, it is the same dev server you started in another terminal and
   forgot. [c6](#c6-ports-and-localhost) has the command to find and stop it.

Note what you did not do: you did not paste it into an agent and ask it to fix your code.
There is nothing wrong with your code.

### When the error is empty

Sometimes a command fails, prints nothing, and returns a non-zero exit code. This happens,
and it is not your fault. Two mechanisms account for almost all of it.

Every command hands back a number when it finishes, zero for success and anything else for
failure, and nothing shows it unless you ask:

```powershell
$LASTEXITCODE
```

Programs also write on two separate channels, ordinary output and errors, so sending output
to a file captures the first and drops the second. Merging them keeps the error:

```powershell
npm run build > log.txt 2>&1
```

[f3](#f3-exit-codes-and-streams) owns both and has the rest of the procedure. The part to
carry away is what you report at the end of it: "this command exits with code N and prints
nothing" is specific and useful, and a much better prompt than "it doesn't work."
