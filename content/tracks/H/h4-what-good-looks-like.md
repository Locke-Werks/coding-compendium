---
id: h4-what-good-looks-like
title: What good looks like in generated code
type: section
track: H
order: 40
verified: 2026-08-02
volatility: low
answer: >
  You can judge generated code without reading it closely by looking for six
  things: functions longer than a screen, repeated blocks, swallowed errors,
  commented-out code, unexplained numbers, and leftover TODO markers.
owns:
  - code smell recognition for non-coders
see_also:
  - h3-reviewing-a-diff-you-cannot-fully-read
  - e7-agent-failure-modes
  - h6-when-tests-lie
  - g6-secrets-and-what-never-to-commit
  - e5-prompting-that-works
keywords:
  - is this good code
  - code smell
  - this function is really long
  - magic numbers
  - it left todo comments
  - copy pasted code
  - what does bad code look like
  - swallowed error
---

## More

This is recognition, not repair. You are looking for six shapes that show up in generated
code, and for each one you have a question to ask rather than a fix to make. The agent
answers accurately when the question is specific, and it agrees with anything when the
question is "is this good code."

**A function longer than your screen.** Ask: what is the one thing this function does? If
the honest answer needs the word "and," ask for it split.

**The same block of code two or three times.** Duplication is where bugs go to hide, because
somebody fixes one copy and not the other two. Ask: is this duplicated elsewhere, and should
it be one function?

**A swallowed error.** `except: pass`, an empty `catch {}`, a `.catch(() => {})` with
nothing inside. The code has been told to continue as though nothing happened. Ask: what
happens when this fails, and how would I find out?

**Commented-out code.** Ask for it deleted. Git already has every old version
([d1](#d1-what-git-actually-stores)), so a commented block is clutter that makes the next
reader wonder whether it matters.

**An unexplained number.** `86400`, `* 0.15`, `if (status == 3)`. Ask what it means and for
it to be a named constant. The number is fine; the mystery is not.

**A leftover `TODO` or `FIXME`.** Ask whether it is a known gap you should track or an
abandoned thought. Agents leave these behind when they run out of certainty, and they mark
the exact spot worth your attention.

None of these are bugs by themselves. Plenty of correct code has all six. They are the
places where asking a question is cheap and occasionally pays for the whole review.

One rule outranks all of them: new code should look like the code already around it. Code
that reads as though it arrived from somewhere else usually did.

## Full

### The six, with what they look like

**Long functions.** A function you cannot see the whole of is one you cannot reason about.
Around fifty lines is where most people start to lose the thread. The fix is always the
same shape: pull the middle out into a function with a name that says what it does.

**Duplication.** Three near-identical blocks with one value different is the classic. The
agent produced it honestly, because generating a variant is easier than restructuring. Ask
for one function with a parameter.

**Swallowed errors.** This is the one to care about most, because it converts a loud failure
into a silent wrong answer:

```python
try:
    user = fetch_user(user_id)
except:
    user = None
```

Any failure at all, including a typo in the function name, now produces `None` and no
message. The program keeps going and breaks somewhere else entirely, which is how you end
up debugging the wrong file for an hour. Catching a specific error and doing something
deliberate is fine. Catching everything and continuing is not.

**Commented-out code.** Delete it. If it turns out to have mattered, it is one command away
in the history.

**Magic numbers.** `86400` is the number of seconds in a day, and `SECONDS_PER_DAY` says so
in the code. `if (status == 3)` requires knowing what 3 means, forever, and the person who
knew has moved on.

**Leftover markers.** `TODO`, `FIXME`, `HACK`, and comments that read like an apology.
These are honest signals from the agent about where it was unsure, so they are worth
following rather than deleting.

### Three more that only show up in generated code

**Names that lie.** A function called `getUser` that also updates the database, or
`validateInput` that quietly rewrites the input. The name promised one thing and the body
does two. Ask what else it does besides what the name says.

**Abstraction with one caller.** A new interface, factory, or base class with exactly one
implementation. It is the shape of code from a large system, applied to a small one. Ask
whether the extra layer is doing anything, and be willing to accept "yes" as the answer.

**Machine-specific values.** A hardcoded `C:\Users\you\projects\...` path, a port number
written into three files, a real email address in a default. These work on your machine
today and nowhere else tomorrow. Anything that looks like a credential is a different and
more urgent problem ([g6](#g6-secrets-and-what-never-to-commit)).

### What is not worth your attention

- **Formatting.** Indentation, quote style, line length, trailing commas. A formatter
  settles all of it and nobody should be reading it.
- **Variable names in short loops.** `i` and `x` inside three lines are fine.
- **Anything a linter would catch.** Unused imports, unreachable code, a missing semicolon.
  Ask the agent to run the project's linter and formatter rather than reviewing for these
  yourself.
- **Style you would have written differently.** Consistency with the surrounding file beats
  your preference, and your preference is not yet informed.

### How to ask so you get a real answer

The phrasing matters more than the question, because a question that can be answered with
agreement will be ([e7](#e7-agent-failure-modes) on sycophancy).

Weak: "is this good code?" You will get yes.

Better:

- "What does this function do, in one sentence, without using the word 'and'?"
- "What happens when `fetch_user` raises here?"
- "What is 3 in this comparison?"
- "Is any of this duplicated elsewhere in the project?"
- "Which parts of this were you least sure about?"

That last one is unreasonably effective. It gets the same information a `TODO` would have,
for the parts where the agent did not leave one.

### Code that matches its surroundings

Read the new code next to the old code, and notice whether it belongs. Same naming style,
same error handling, same structure, same level of commenting. Code that matches its
surroundings was written with awareness of the project. Code that does not is a generic
solution dropped into a specific place, and that mismatch predicts problems better than any
individual item on this page.
