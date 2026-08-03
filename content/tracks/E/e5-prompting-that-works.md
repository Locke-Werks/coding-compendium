---
id: e5-prompting-that-works
title: Asking for what you actually want
type: section
track: E
order: 50
verified: 2026-08-02
volatility: low
answer: >
  A request the agent can act on names the file, states the change, sets a
  boundary on what not to touch, and says how you will know it worked; leave one
  out and it fills the gap with a confident guess.
owns:
  - prompting technique
  - specificity
  - providing context
see_also:
  - e3-plan-mode
  - f5-what-to-paste-and-what-not-to
  - e4-claude-md-and-agents-md
  - e7-agent-failure-modes
keywords:
  - how to prompt
  - it did the wrong thing
  - prompt engineering
  - asking the agent
  - better prompts
  - it misunderstood me
---

## More

Four parts make a request actionable: where, what, the boundary, and the finish line. Leave
one out and the agent fills it in for you, fluently, from the most likely thing someone
would have meant.

Weak:

> the login is broken, fix it

Strong:

> Login returns a 500 after a correct password. The handler is in
> `src/auth/login.ts`. Here is the full error and the command I ran. Do not change the
> database schema. When it works, `npm test` should pass with no test file modified.

Same request. The second one names the file, describes the symptom, sets a boundary, and
states what done means.

Five habits carry most of the weight.

**Name the file.** If you know where it lives, say so. Otherwise the agent reads twenty
files to find out, which costs time and fills the window with things you did not need
([e2](#e2-context-windows)).

**Paste the error whole and unedited**, along with the command that produced it. The parts
you would trim are the parts it uses ([f5](#f5-what-to-paste-and-what-not-to)). Strip
credentials first ([g8](#g8-what-never-to-paste-into-a-chat)).

**Say what not to touch.** Negative constraints work well: "do not modify anything outside
`src/auth/`." This one sentence is the cheapest defense you have against scope creep
([e7](#e7-agent-failure-modes)).

**Say what done looks like.** A test that passes, a command that prints something specific,
a page that loads. Without it, "done" means "the agent stopped talking."

**One thing per message.** Five requests in a paragraph get four done and one silently
dropped, and you will not be told which.

Then the request that feels rude and outperforms all five: "do not write any code yet, tell
me your plan first" ([e3](#e3-plan-mode)).

## Full

### Before and after

| Instead of | Ask for |
|---|---|
| "make it better" | "the `render` function is 200 lines. Split it into smaller functions without changing what it does, then run the tests." |
| "fix all the bugs" | "here is one failing test and its full output. Make it pass without editing the test." |
| "optimize this" | "this endpoint takes four seconds. Find out why before changing anything, then tell me what you found." |
| "add tests" | "add one test in `tests/test_parser.py` for the case where the input list is empty." |
| "does this look right?" | "read `src/queue.py` and list anything that would break if two workers ran at the same time." |

The pattern across all five: the right-hand column names a place, a scope, and a
verifiable outcome. The left-hand column asks the agent to decide what you meant, and it
will decide.

### Give it what it cannot see

The agent sees the files it reads and the text you paste. It does not see your browser,
your other terminal window, or the thing you noticed and did not mention. For anything
broken, include all five of these:

1. The exact command you ran.
2. The full output, unedited.
3. What you expected to happen.
4. What actually happened.
5. What you already tried, and what that did.

The fifth is the one people skip and the one that saves the most time. Without it, the
first suggestion you get is very often the thing you tried an hour ago.

### Ask for the thing before the thing

Some of the most useful requests produce no code:

- **A map.** "Read `src/` and tell me where sessions are created. Do not change anything."
- **Options.** "Give me three approaches and the tradeoff of each."
- **Grounding.** "Explain what this file does, then stop."
- **A diagnosis.** "What would have to be true for this to be a caching problem?"

That last one is worth more than "are you sure?" Asking whether it is sure reliably gets
you agreement rather than a second look, which is the sycophancy problem in
[e7](#e7-agent-failure-modes). A question that demands evidence gets you either evidence or
an admission, and both are useful.

### Correcting mid-stream

When it goes wrong, resist piling a correction on top of a confused transcript. Stop it,
state the current state of the world plainly, and restate the goal:

> Stop. Right now `login.ts` contains both the old handler and the new one. Delete the old
> one. Change nothing else in that file.

Layered corrections produce layered confusion. Three of them in a row with no diagnosis
means the agent is guessing, and [f6](#f6-when-the-agent-loops) is the card for that.

### Templates worth stealing

For a change:

```text
Goal: <one sentence>
Where: <file or folder>
Constraint: do not change <thing>
Done when: <command> passes, or <observable thing> happens
```

For a bug:

```text
Command I ran: <exact command>
Expected: <what should have happened>
Got: <full output, pasted whole>
Already tried: <what you did and what it changed>
Do not change: <the thing that is fine>
```

Neither one is long. Both take ten seconds once they are habit, and both cut out an entire
round trip of the agent guessing at context you already had.

### The ceiling, honestly

There is a point where a request becomes so precise that you are writing the code in
English, which is slower than writing the code. That ceiling is real and you will feel it.

The way through is not more words per message. It is smaller tasks per message, plus a
`CLAUDE.md` carrying the standing constraints so you stop repeating them in every request
([e4](#e4-claude-md-and-agents-md)). Anything you have typed three times belongs in that
file, and getting it out of your prompts is what keeps them short enough to stay sharp.
