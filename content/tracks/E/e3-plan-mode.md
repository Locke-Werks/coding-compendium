---
id: e3-plan-mode
title: Plan mode, and why to use it constantly
type: section
track: E
order: 30
verified: 2026-08-02
volatility: weekly
answer: >
  Plan mode makes the agent show you an ordered list of steps before it edits
  anything, and it is the cheapest review you will do, because a wrong
  assumption is obvious in thirty lines of English and invisible in four hundred
  lines of code.
owns:
  - plan mode
see_also:
  - e5-prompting-that-works
  - a4-the-loop
  - h3-reviewing-a-diff-you-cannot-fully-read
  - e7-agent-failure-modes
keywords:
  - shift tab
  - make a plan first
  - dont write code yet
  - approve the plan
  - planning mode
---

## More

Plan mode is a state where the agent may read and think but may not write. It investigates,
hands you an ordered list of steps, and waits. You approve it, correct it, or throw it out.
Only then does it touch a file.

In Claude Code, press **Shift+Tab** to cycle into it. In any tool, including Claude Code,
the sentence works just as well: "make a plan first, do not write any code yet." Learn the
sentence rather than the keystroke. Keystrokes change between versions and the sentence
never will.

The reason this is the highest-value habit available to you: a plan is thirty lines of
English you can read in a minute. The diff it turns into is four hundred lines of code you
cannot fully read ([h3](#h3-reviewing-a-diff-you-cannot-fully-read)). A wrong assumption is
plainly visible in a plan and nearly invisible in a diff, where it looks like more code
that works.

Four things to check in a plan, in this order:

1. **The file list.** How many files, and are they the ones you expected. Nine files for a
   one-line change is the tell for scope creep ([e7](#e7-agent-failure-modes)).
2. **Assumptions.** Anything it says exists that you do not recognize. Half the time it
   invented the thing.
3. **Verbs you did not use.** "Refactor," "migrate," "standardize," "clean up." Each one is
   work you did not ask for, riding along.
4. **The check.** Does the last step say how you will know it worked. If it does not, ask
   for one ([h1](#h1-what-a-test-is)).

Reject a plan by saying what is wrong, not by saying no. "Steps 1 and 2 are right. Drop
step 3, I do not want the config file moved. In step 4, use the existing `db.query` helper
instead of adding a second client."

## Full

### What it costs

Plan mode is slower. It is two round trips for something that could have been one, and for
a typo fix that is pure ceremony. Skip it there.

Use it for anything that touches more than one file, anything you cannot describe in a
single sentence, and anything in code you did not write. Those three cover most of the work
where a bad idea gets expensive.

### A bad plan and a corrected one

The request: "the login form should show an error when the password is wrong."

What comes back:

```text
1. Refactor auth handling in src/auth/ into a shared AuthService
2. Add error state to LoginForm, RegisterForm, and ResetForm
3. Migrate all three forms to the new useAuth hook
4. Update the affected tests
5. Update the auth section of the README
```

Five steps, three forms, a new hook, and a refactor, for one message on one form. Every
item is defensible on its own. Together they are a different project, and the diff would be
unreviewable.

What you send back:

```text
1. In src/components/LoginForm.tsx, add an error line under the password field
2. Show it when the login request comes back 401
3. Add one test: wrong password renders the message
```

Three steps, one file plus one test. The real value shows up later: because you know
exactly what should have changed, the diff review takes ninety seconds instead of an
afternoon.

### Plan mode as a way to ask questions

Some of the best uses produce no code at all. Because the agent cannot write in this mode,
you can point it at unfamiliar code without any risk to the files:

- "Do not change anything. Tell me where user sessions are created and everything that
  reads them."
- "Give me three ways to do this and the tradeoff of each. No code."
- "Read this folder and tell me what would break if I deleted it."

You get a map of code you did not write, and nothing on disk moved.
[j4](#j4-reading-a-repo-you-did-not-write) is the longer version of that procedure.

### Write the plan down for anything longer than one sitting

Once a plan is approved, ask for it in a file: `docs\PLAN.md`, committed. Two payoffs. A
context reset now costs nothing, because the plan outlives the session that produced it
([e6](#e6-when-to-reset-context)). And a second agent can review the plan against the diff
later, which is the strongest version of the trick in [e10](#e10-using-two-agents).

### Where plan mode does not save you

A good plan is not a guarantee. The agent can plan correctly and implement badly: a
function that does not exist, a test quietly weakened to get green, a library call that was
right two years ago. All of those live in [e7](#e7-agent-failure-modes) and none of them
are visible in a plan.

Plan mode moves the review earlier and makes the later review cheaper. It does not remove
it. The full rhythm stays: plan, approve, build, read the diff
([h3](#h3-reviewing-a-diff-you-cannot-fully-read)), run the thing, commit
([d4](#d4-commit-well)). [a4](#a4-the-loop) owns that loop.

### The one-question version

If you remember nothing else from this card: before the agent writes anything, ask which
files it will change and why each one.

That is plan mode compressed into a sentence. It works in every tool, in every version,
with no keystroke to remember, and it catches most of what a formal plan catches. The
answer takes one turn to read, and a surprising share of the time it contains a file that
has no business being there.
