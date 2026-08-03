---
id: f5-what-to-paste-and-what-not-to
title: What to paste into the agent, and what to strip first
type: section
track: F
order: 50
verified: 2026-08-02
volatility: low
answer: >
  Paste the command you ran and the error whole and unedited, because the file
  paths, line numbers, version strings, and library frames you would trim out
  are exactly the parts an agent uses to work out what happened.
owns:
  - pasting errors
  - redaction before pasting
see_also:
  - g8-what-never-to-paste-into-a-chat
  - e5-prompting-that-works
  - f1-how-to-read-an-error-message
  - f2-stack-traces
  - f4-logs
keywords:
  - what should i paste
  - how much of the error
  - paste the whole error
  - redact
  - screenshot of error
  - it doesnt work
  - how to ask for help
---

## More

Four things, every time.

1. **The command you ran**, character for character. Not "I tried to build it."
2. **The entire error**, from the command prompt down to where your prompt came back.
   Unedited.
3. **What you expected to happen.** One line.
4. **What you already tried**, if anything, and what that did.

The second one is where people go wrong, and they go wrong in a consistent direction. Faced
with thirty lines of red, the instinct is to paste the sentence that looks like the message
and drop the rest. The rest is the identifying information. File paths say which project and
which environment. Line numbers say where. Version strings in the header say whether this is
a known bug. The frames from inside somebody else's library say which library and which
function it died in ([f2](#f2-stack-traces)).

An agent given the whole thing can often name the cause in one reply. Given a summary, it
guesses, and then you spend four turns feeding it back the details you already had
([f6](#f6-when-the-agent-loops)).

There is exactly one category to remove first: **credentials.** Keys, tokens, connection
strings, passwords, and the contents of a `.env` file. That list and how to redact it belong
to [g8](#g8-what-never-to-paste-into-a-chat), which is short and worth reading once before
you need it. Everything else stays.

Do not paste a screenshot of an error. A picture cannot be searched, cannot be copied out
accurately, and turns a fifteen-second answer into a guess about whether that character is a
one or an l. Select the text in the terminal and copy it.

If you take one habit from this card, take the fourth item. "It worked this morning, and
since then I installed Node 22" is more useful than the error.

## Full

### The shape of a good paste

```text
I ran this in PowerShell, from C:\Users\nyx\projects\site:

    npm run dev

I expected the dev server to start on localhost:3000. Instead:

    [the entire error, unedited]

This worked yesterday. Since then I installed Python and restarted.
```

That is under a minute of typing and it removes almost every follow-up question. Note what it
contains that a summary would not: the shell, the folder, the exact command, the untouched
output, the expectation, and the change.

### Why the boring parts carry the information

| The part you would cut | What it tells the agent |
|---|---|
| The full file path | Which project, and whether you are in a virtual environment |
| `node_modules\` frames | Which library, and its version |
| The line and column numbers | Where to look, exactly |
| The version banner at the top | Whether this is a known issue with that version |
| The warnings above the error | Frequently the actual cause, one step earlier |
| The final `npm ERR!` lines | The path to the full log ([f4](#f4-logs)) |

The one thing that genuinely does not help is repetition. If the same block of forty lines
appears eight times because something retried, one copy plus "this repeats eight times" is
better.

### Getting the text out of the terminal cleanly

In Windows Terminal, drag to select and press Ctrl+C. Selecting copies; there is no separate
copy command.

When the output is long or already scrolled away, re-run it and capture both channels:

```powershell
npm run dev 2>&1 | Set-Clipboard
```

`2>&1` merges the error channel into the normal one so the error is included, which it
otherwise would not be ([f3](#f3-exit-codes-and-streams)). `Set-Clipboard` puts the result
straight on your clipboard, ready to paste.

To keep a copy as well:

```powershell
npm run dev 2>&1 | Tee-Object -FilePath error.txt
```

Delete `error.txt` afterward, or add it to `.gitignore`, so it does not end up committed.

### How much is too much

The rule: paste everything from the last prompt down. That is one command's worth of output
and it is almost never too much.

A log file is different. Those run to tens of thousands of lines, and pasting one fills the
context the agent needs for your actual code ([e2](#e2-context-windows)). From a log, paste
the first error, the stack trace under it, and about twenty lines before it.

Whole source files are different again. If your agent can read files, give it the path and
let it read. Pasting a 900-line file it could have opened is pure waste, and it produces a
worse answer because the agent now has your paste and the file and no way to know which is
current.

### The one exception, and where it lives

Strip credentials before pasting. Not because an error looks dangerous, but because they turn
up in error output constantly: a connection string in a database error, a token in a request
header, a full address with a key in the query part of it.

[g8](#g8-what-never-to-paste-into-a-chat) owns the list of what counts and how to replace it
without destroying the usefulness of the message. Read it once. This card does not repeat it
because a second, slightly different version of that list is worse than one.

The related rule: if you find you have pasted a real key somewhere, the fix is to rotate the
key, not to delete the message. [g6](#g6-secrets-and-what-never-to-commit) covers that.

### Things that seem helpful and are not

- **Your own diagnosis, stated as fact.** "The database connection is broken" sends the agent
  to the database. Say what you observed and let it conclude. Say your theory as a theory.
- **The agent's previous answer, pasted back at it.** It already has that. Paste the result
  of following the advice instead.
- **A trimmed error you retyped by hand.** Typos in a pasted error message send everyone
  somewhere that does not exist.
- **"It doesn't work."** The most expensive four words available to you.

### Versions, when they matter

Any problem involving installation, dependencies, or "this used to work" gets these:

```powershell
node --version; npm --version; git --version
```

One line, three answers, and it settles a category of question immediately. Swap in
`python --version` or `cargo --version` as appropriate.
