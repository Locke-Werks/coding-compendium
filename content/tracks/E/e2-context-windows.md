---
id: e2-context-windows
title: Context windows, and why long sessions get worse
type: section
track: E
order: 20
verified: 2026-08-02
volatility: quarterly
answer: >
  A context window is the fixed amount of text a model can read in one turn, and
  because the whole conversation is re-sent every turn, a long session buries
  your instructions in old output rather than tiring the model out.
owns:
  - context window
  - context rot
  - tokens as a unit
see_also:
  - e6-when-to-reset-context
  - e8-tokens-and-cost
  - e4-claude-md-and-agents-md
  - f5-what-to-paste-and-what-not-to
keywords:
  - context rot
  - it forgot what I said
  - session got dumb
  - compaction
  - conversation too long
  - why is it slower now
---

## More

A context window is the total amount of text a model can consider in a single turn. It is a
hard size limit, and everything competes for the same space: your instructions file, every
message either of you has sent, every file the agent read, and every line of output from
every command it ran.

The part that surprises people is the mechanism. The model holds no memory between turns.
Each time you press Enter, the entire conversation so far is sent again, from the top, and
read again from scratch. Nothing persists in its head between messages. The transcript is
the memory.

Two things go wrong as that transcript grows.

**It fills up.** When the window is full, something has to leave. The tool either squeezes
the conversation into a summary and drops the originals, or the oldest material falls off
the front. Either way, the exact words of a decision you made ninety minutes ago are now a
paraphrase, or gone. The agent is not ignoring you. It cannot see what you said.

**It gets noisy well before it fills.** Long before the limit, your one important
constraint is sitting between four thousand lines of install output and three files it read
and did not need. A model's ability to find the relevant line degrades as the pile grows,
and material stuck in the middle fares worst of all. This is the effect people call
**context rot**: a session that started sharp is dull by hour three.

Neither of these is fatigue. A model does not get tired, and hour three is the same model
as minute one working from a worse input. That distinction is practical, because it tells
you what the fix is. You cannot rest it. You can hand it a cleaner input, which means a new
session ([e6](#e6-when-to-reset-context)).

The same mechanic shows up on the bill. A long transcript is re-sent every single turn, so
every turn late in a session costs more than the identical turn early in one.
[e8](#e8-tokens-and-cost) has the arithmetic.

## Full

### What is actually in the window

Roughly in the order it arrives:

1. **The vendor's own system prompt.** Instructions you never see and cannot remove. It is
   not small.
2. **Your project instruction files.** `CLAUDE.md` or `AGENTS.md`, read at session start
   ([e4](#e4-claude-md-and-agents-md)). They sit near the top and stay there, which is why
   they get followed reasonably well and why every line of them costs you.
3. **Tool definitions.** A description of every tool available, including every tool from
   every MCP (Model Context Protocol) server you connected ([e9](#e9-mcp)). Connect five
   servers and you have spent real space before typing a word.
4. **The conversation.** Your messages, its messages, every file it read in full, and the
   complete output of every command it ran.

Only the fourth item grows, and it grows unevenly. One command can add more to the window
than an hour of talking.

### What eats it fastest

- **Command output.** A test run that prints every passing test. An install log. A `git
  log` with no limit. These are the biggest single contributors and the easiest to avoid.
- **Whole-file reads.** Asking about one function in a two-thousand-line file puts the
  whole file in.
- **Exploration.** When the agent does not know where something lives, it reads until it
  finds out. That is the hidden price of a vague request
  ([e5](#e5-prompting-that-works)).
- **Its own thinking.** Reasoning tokens count too, and a hard problem generates a lot of
  them.

### Compaction, and the specific thing it loses

When the window is close to full, the tool compacts: it writes a summary of the
conversation and continues from that instead of the original text.

This is useful and it is lossy in a predictable direction. Summaries keep conclusions and
drop reasons. After a compaction the agent knows you chose one library over another, and
does not know why, so it will cheerfully reopen the question the moment that library is
inconvenient. Watch for that specific symptom: settled decisions coming back to life with
no memory of the argument.

### Bigger windows do not solve this

Vendors quote large window sizes and the numbers keep going up. Two honest caveats. First,
the quoted figure is a maximum, and your plan, your tool, and your model may allow less.
Second, capacity is not attention: a model's accuracy at finding a specific instruction
drops as the surrounding volume rises, even when everything technically fits. A bigger
window is a bigger desk, not a better memory.

### A worked example

A four-hour session, doing real work the whole time.

- **Hour one.** Fifteen files read, a plan agreed, two commits. Everything sharp.
- **Hour two.** Two failing test runs pasted whole, plus a six-hundred-line build log.
  Still fine, still answering well.
- **Hour three.** Compaction fires. You do not notice, because the reply that follows reads
  normally.
- **Hour four.** It reintroduces the bug you fixed in hour one, and explains the change
  confidently.

Nothing broke. The fix, the reasoning behind it, and the output that proved it were all in
the part that got summarized away. This is the system working exactly as designed, and it
is why [e6](#e6-when-to-reset-context) exists as its own card.

### What to do about it

1. **One task per session.** The cheapest habit here by a wide margin.
2. **Front-load the constraint.** Say what must not change in your first message, where it
   sits near the top and survives longest, rather than in message thirty.
3. **Keep instruction files short** ([e4](#e4-claude-md-and-agents-md)). Every line is
   re-sent at the start of every session forever.
4. **Do not paste what it can read.** Give it the path. A file the agent reads on purpose
   is a file it needed; a file you pasted is a file it now carries all session.
5. **Paste the failure, not the run.** Twenty relevant lines beat six hundred
   ([f5](#f5-what-to-paste-and-what-not-to)).
6. **Reset on the signs, not on the clock** ([e6](#e6-when-to-reset-context)).

The mental model that keeps all six straight: the window is a desk, everything you put on
it stays there for the rest of the session, and the agent has to look past all of it to
find the thing you actually care about.
