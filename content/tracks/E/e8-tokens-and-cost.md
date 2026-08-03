---
id: e8-tokens-and-cost
title: Tokens, and what you are actually paying for
type: section
track: E
order: 80
verified: 2026-08-02
volatility: weekly
answer: >
  A token is about four characters of text, the entire conversation is re-sent
  to the model every single turn, and that one fact explains why a long session
  costs more per message than a short one.
owns:
  - token
  - pricing model
  - why long context costs more
see_also:
  - e2-context-windows
  - e6-when-to-reset-context
  - e9-mcp
  - e5-prompting-that-works
keywords:
  - how much does claude code cost
  - token
  - usage limit
  - rate limit
  - why is it expensive
  - prompt caching
  - api pricing
---

## More

A token is a chunk of text, roughly four characters or three quarters of a word. A common
word is one token. An unusual one splits into two or three. A curly brace is one. Every
piece of text going in or coming out is counted this way, and tokens are the unit of both
memory ([e2](#e2-context-windows)) and money.

There are two prices and they are not the same. **Input** is everything sent to the model:
your message, the instruction file, every file it read, every line of command output, and
the whole conversation so far. **Output** is what it writes back. Output costs several
times more per token, but input volume is far larger, so a long working session is
dominated by input.

Here is the part that explains the bill. The model has no memory between turns, so the
entire transcript is re-sent every time you press Enter. Turn one sends a paragraph. Turn
fifty sends everything from turns one through forty-nine plus your new paragraph. Cost per
turn rises as the session grows, and the total cost of a session grows faster than the
number of messages in it.

**Prompt caching** takes the edge off. Providers store the unchanged front of your
conversation and charge much less to reuse it. It is usually automatic. It rewards a stable
beginning and punishes editing anything early in the session, which is one more reason to
front-load your constraints and leave them alone.

On a subscription rather than a per-token bill, you are spending the same resource with a
different meter. The limit arrives as "you have reached your usage cap, try again in three
hours" instead of a charge.

Every concrete number in this card will be wrong eventually. Prices fall, models get
renamed, and the tiers move. Treat the figures below as shapes, and check your provider's
pricing page before planning around any of them.

## Full

### The arithmetic, with deliberately illustrative numbers

Suppose input costs three dollars per million tokens and output costs fifteen. Those are
the right order of magnitude for a mid-tier model in 2026 and they are exactly the kind of
number that goes stale, so use them to learn the shape and nothing else.

- **A 500-line source file** is roughly 6,000 tokens. Sending it once costs about two
  cents. Nothing.
- **A fifty-turn session** whose transcript has grown to 100,000 tokens re-sends all
  100,000 on the final turn. That single turn costs around thirty cents before the model
  writes a word of reply.
- **The whole session**, averaged across its growth, lands somewhere in the low tens of
  dollars for a serious afternoon, and meaningfully less with caching doing its job.

The shape matters more than the digits. Sending a file once is free in every practical
sense. Sending it fifty times, because it is stuck in a transcript that keeps getting
re-read, is where the money actually goes.

### What burns tokens fastest

- **Command output you did not need.** A test run printing every passing test, an install
  log, a full `git log`. The single biggest avoidable cost.
- **Whole-file reads** for a one-function question.
- **Exploration caused by a vague request.** "Fix the login" makes it read the project.
  Naming the file does not ([e5](#e5-prompting-that-works)).
- **Very long sessions**, for the re-sending reason above
  ([e6](#e6-when-to-reset-context)).
- **Every MCP (Model Context Protocol) server you connect.** Its tool descriptions load at
  the start of every session whether you use them or not ([e9](#e9-mcp)).

### Five things that cut it, in order of effect

1. **Shorter sessions.** Clear at every task boundary
   ([e6](#e6-when-to-reset-context)). This one is worth more than the other four combined.
2. **Name the file instead of pasting it.** A file read on purpose is smaller than a file
   pasted in a panic.
3. **Paste the failure, not the run.** Twenty lines, not six hundred
   ([f5](#f5-what-to-paste-and-what-not-to)).
4. **Keep the instruction file short.** Every line is re-sent at the top of every session
   forever ([e4](#e4-claude-md-and-agents-md)).
5. **Use a cheaper model for mechanical work.** Renaming things, writing boilerplate,
   turning a described case into a test. Save the expensive model for design and for
   debugging, which is where the difference shows.

### Reading your own usage

Both tools expose a usage or cost command inside the session. In Claude Code it is
typically `/cost`; the names move around, and `/help` inside the session lists whatever is
current for your version.

Run it once, mid-session, on a normal working day. The number surprises most people
exactly once, and after that they close long sessions without being told to.

### Why slow and expensive are the same signal

A large input costs more and takes longer to process. When a session starts feeling
sluggish, that is the same underlying fact as the bill: the transcript has grown large.
Sluggishness is the free version of the warning, and it arrives before the invoice does.

### The rule that survives every price change

The cheapest token is the one you never send. The second cheapest is one the provider has
already cached. Every practical suggestion above is one of those two sentences applied to a
specific habit, and both of them will still be true when the numbers in this card are
worthless.
