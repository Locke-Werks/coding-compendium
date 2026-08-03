---
id: e10-using-two-agents
title: Using two agents against each other
type: section
track: E
order: 100
verified: 2026-08-02
volatility: quarterly
answer: >
  Build with one agent and review with the other, because the second model has
  no stake in the first one's reasoning and reads the diff as evidence instead
  of defending its own conclusion.
owns:
  - cross-review between tools
see_also:
  - h3-reviewing-a-diff-you-cannot-fully-read
  - e7-agent-failure-modes
  - e3-plan-mode
  - f6-when-the-agent-loops
keywords:
  - claude and codex together
  - second opinion
  - cross review
  - two tools
  - which agent is better
  - ai reviewing ai
---

## More

Build with one agent, review with the other. That is the whole technique, and it works
better than it has any right to.

The reason is not that one tool is smarter. It is that the reviewing model has no stake in
the first one's reasoning and no memory of writing the code. It reads the diff as evidence.
The model that wrote the code will defend a wrong choice fluently, because defending it is
a natural continuation of the text that produced it. A second engine has nothing to
continue.

The mechanics, in order:

1. **Commit the work first** ([d4](#d4-commit-well)). You need something stable to point
   at.
2. **Open the second tool in the same folder.**
3. **Give it the original request and the diff**, and ask for specific things rather than a
   general opinion.

A prompt that produces useful output:

> Review the changes in the last commit against `main`. The request was: show an error
> message on the login form when the password is wrong. For each file changed, say whether
> it needed to change. Flag anything reformatted or renamed without a functional change,
> any test that was modified, any new dependency, and anything doing something other than
> what the commit message says.

That prompt asks for the tells in [e7](#e7-agent-failure-modes) by name, which is why it
beats "review this."

Two things it is not. It is not a test: two models can be confidently wrong about the same
stale library ([h1](#h1-what-a-test-is)). And it is not free: two subscriptions, or two
bills ([e8](#e8-tokens-and-cost)). If you have only one tool, a fresh session that has no
memory of writing the code gets you most of the benefit
([e6](#e6-when-to-reset-context)).

## Full

### Three uses, in order of value

**Review a committed diff.** The main one, using the prompt above. Do it after the commit
and before the merge, so rejecting the whole thing costs you nothing
([d8](#d8-pull-requests)).

**Second opinion on a plan.** Cheaper than reviewing code, because a plan is thirty lines
([e3](#e3-plan-mode)). "Here is a plan another agent produced for this request. What is
missing, what is over-scoped, and what would you do differently?"

**Tie-breaker on a loop.** When one agent has tried three fixes without ever stating a
diagnosis ([f6](#f6-when-the-agent-loops)), hand the problem to the other one cold: the
error, the file, and what has already been tried. A fresh reading beats a fourth guess, and
it costs one turn to find out.

### Handling disagreement

The reviewer will find things. Some will be real, some will be style, and some will be
confidently wrong in the opposite direction from the first agent. Do not automatically take
the newer opinion. It is not more likely to be right. It is only more recent.

Make the disagreement testable instead:

> Agent A says this is safe when two requests arrive at the same time. Agent B says it is
> not. Point me at the specific line and walk me through what happens, in what order.

A claim that survives that demand is worth acting on. A claim that dissolves into "it is a
best practice" is not.

When it stays unresolved, write a test that fails if the reviewer is right
([h1](#h1-what-a-test-is)). That settles it permanently, which is more than either agent
can do for you.

### The bias this technique introduces

A model asked to find problems will find problems. Criticism reads as insight, and the
second opinion always sounds sharper because it is judging rather than building. Left
alone, this trains you to distrust whichever tool you happened to build with.

Two guards, both one sentence long:

- **Ask what is fine.** "First list what is correct and should stay as it is, then list
  your concerns."
- **Ask for severity.** "Rank each item: would break in production, would confuse a future
  reader, or personal preference." The third category is usually the largest, and it is the
  one to ignore.

### Which tool for which job

Do not overthink the pairing. The two leading tools are close enough in capability that the
split matters far less than the habit of splitting at all. Pick one to build with, keep the
other for review, and swap if the arrangement annoys you.

Both change fast enough that any specific recommendation printed here would be stale before
you read it, which is itself a useful thing to know about this whole category of advice.

### The single-tool version

Everything above works with one tool and two sessions, minus the benefit of a genuinely
different model. Finish the work, commit it, clear the session
([e6](#e6-when-to-reset-context)), then open a fresh one and say:

> Review the last commit. I did not write it and I do not know what it was meant to do
> beyond this: <paste the original request>.

A model with no transcript of having written the code behaves noticeably more like a
reviewer. What you lose is the second engine's different blind spots. What you keep is the
loss of ownership, which is most of where the value was.

### What it replaces, and what it does not

It replaces some of the reading you cannot realistically do
([h3](#h3-reviewing-a-diff-you-cannot-fully-read)). It does not replace running the code
([h1](#h1-what-a-test-is)), checking the file list yourself, or the list in
[e11](#e11-what-to-never-let-an-agent-do).

Two agents agreeing that something works are two predictions that agree. Running it is an
observation. Those are different categories of evidence and no amount of agreement moves
one into the other.
