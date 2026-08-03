---
id: a5-what-you-still-have-to-know
title: What you still have to know yourself
type: section
track: A
order: 50
verified: 2026-08-02
volatility: low
answer: >
  You do not need to write code from memory. You do need to notice when something
  is wrong, read a diff well enough to see what it touched, undo anything, and
  say what "done" means before you start.
owns:
  - the irreducible minimum
see_also:
  - h3-reviewing-a-diff-you-cannot-fully-read
  - e7-agent-failure-modes
  - d10-undo-everything
  - f1-how-to-read-an-error-message
  - a2-the-honest-version
keywords:
  - do i need to learn to code
  - what should i learn first
  - minimum knowledge
  - can i skip learning syntax
  - what does the ai not do
---

## More

Start with what you can genuinely skip, because the list is longer than people expect.

You do not need to write a loop from memory. You do not need to remember which of `map`,
`filter`, and `reduce` does what, or the order of arguments to any function, or the flags of
any command, or how to spell a language's import statement. You do not need to have opinions
about frameworks. Every one of those is a lookup, and the agent is a very good lookup.

What does not delegate is a short list.

1. **Noticing that something is wrong.** The app does not do what you asked, the change is
   ten times the size of the request, the test count went down, or the agent's summary and
   the diff disagree. Nothing catches these for you.
2. **Reading a diff for shape.** Which files, how big, anything you did not ask about. This
   is ninety seconds and it is the highest-value skill in this whole app.
   [h3](#h3-reviewing-a-diff-you-cannot-fully-read).
3. **Undoing.** Knowing that you can go back, and roughly what each way back costs, is what
   makes moving fast rational. [d10](#d10-undo-everything).
4. **Reading an error far enough to paste the right thing.** Not solving it. Recognizing
   whether it is "cannot find" or "found it and it broke," and handing over the whole text
   rather than your summary of it. [f1](#f1-how-to-read-an-error-message).
5. **Saying what done looks like, before you start.** The agent will happily build the thing
   you described instead of the thing you wanted, and it cannot tell the difference.

That is the floor. It is a week of attention, not a degree, and none of it is about syntax.

## Full

### The list again, with what each one actually costs

| What | Roughly how long to get usable | Where |
|---|---|---|
| Reading a diff for scope | An afternoon | [h3](#h3-reviewing-a-diff-you-cannot-fully-read) |
| The four or five git commands you use daily | A day, then habit | [d3](#d3-the-three-places), [d4](#d4-commit-well) |
| Undo, and what each kind destroys | An hour to read, then look it up | [d10](#d10-undo-everything) |
| Reading an error message | Twenty minutes | [f1](#f1-how-to-read-an-error-message) |
| Knowing the agent's usual failures by name | Twenty minutes | [e7](#e7-agent-failure-modes) |
| Telling PowerShell from Git Bash | Five minutes | [b1](#b1-terminal-shell-command-line) |

Nothing on that list is a programming skill. All of it is operating skill: knowing where you
are, what changed, and how to get back.

### What "noticing something is wrong" looks like in practice

Four tells, in the order you will meet them.

- **The thing does not do what you asked.** Obvious and still the most common. It happens
  because the agent built what your sentence said rather than what you meant.
- **The change is much bigger than the ask.** You asked for a button and nineteen files
  changed. Sometimes that is legitimate refactoring. It is always worth one question.
- **The tests got smaller.** A test file shrank or vanished during a bug fix. That is the
  single most reliable sign that something was made to pass rather than made to work.
  [h6](#h6-when-tests-lie).
- **The summary and the diff disagree.** The agent says it added validation. The diff shows
  a deleted check and a new comment. Believe the diff.

None of these require reading code. They require looking at the report and the file list,
which is why they are learnable in an afternoon.

### The thing that never delegates

Deciding what right means.

An agent can tell you whether the code compiles, whether the tests pass, and whether the
approach is conventional. It cannot tell you whether the export should include archived rows,
whether an empty cart is an error or a normal state, or whether that email should go out
twice when someone double-clicks. Those are product decisions wearing technical clothes, and
they are where nearly every "it works and it is still wrong" bug comes from.

When an agent asks you a clarifying question, that is usually one of these. Answer it
properly instead of saying "use your judgment," because it does not have any about your
project.

### What you will pick up whether you plan to or not

Reading comes before writing, in any language, human or otherwise. After a month of looking
at diffs you will follow most changes in the languages your project uses, without ever
having sat down to study one. That is normal and it is the cheapest path in. Track J is
built for exactly this: recognition first, fluency never required.
[j1](#j1-how-to-recognize-a-language) is where that starts.

### Where this stops being enough

Honest boundaries, because the alternative is finding them the expensive way.

- **Anything handling money, health data, or other people's personal information.** The
  failure mode is not a broken page, it is a breach or a lawsuit, and it needs someone who
  can read every line and knows the rules that apply.
- **Anything running on machines you do not own,** or that other people depend on being up.
- **Anything where you cannot test the failure case.** If you cannot make it break on
  purpose, you cannot tell whether it was fixed. [f7](#f7-reproducing-a-bug).

Between those and the throwaway weekend project sits a very wide band of genuinely useful
software: internal tools, personal apps, prototypes, scripts, small sites. That band is what
this compendium is scoped to, and the list at the top of this card is what it takes to work
in it safely.

### The one that sounds soft and is not

Know what you do not know, and say so to the agent. "I do not know what a migration is,
explain it before you run one" costs you thirty seconds and prevents the class of problem
where you approve something whose consequences you could not have predicted. The agent will
not detect that you were guessing, and it will not slow down on its own.
