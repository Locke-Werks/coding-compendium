---
id: a2-the-honest-version
title: The honest version, and what keeps you out of trouble
type: section
track: A
order: 20
verified: 2026-08-02
volatility: low
answer: >
  The agent is fast and confident and wrong often enough that you cannot trust it
  blindly. What makes this safe is not the AI: it is version control, tests, and
  small reviewable steps, because those check the work without you reading it.
owns:
  - why scaffolding matters
  - the safety net argument
see_also:
  - a4-the-loop
  - h1-what-a-test-is
  - d10-undo-everything
  - e7-agent-failure-modes
  - a5-what-you-still-have-to-know
keywords:
  - is vibe coding safe
  - can i trust the ai
  - why do i need git
  - why tests
  - ai wrote bad code
  - safety net
  - hallucination
---

## More

Three things are true at once, and everything else in this app sits on top of them.

**The agent is fast.** It writes in a minute what would take you an afternoon. That part is
real and it is why you are here.

**The agent is confident.** Correct output and wrong output arrive in the same voice, at the
same speed, with an equally reasonable explanation attached. There is no tell in the text.
Nothing about how a piece of generated code reads tells you whether it works.

**The agent is wrong often enough to matter.** It calls library functions that do not exist.
It edits files you never mentioned. It deletes a failing test rather than fixing the thing
the test caught. The named list, with the symptom that catches each one, is
[e7](#e7-agent-failure-modes).

Put those together and the real problem shows up. You cannot check its work by reading it,
because reading code closely enough to catch a subtle error is exactly the skill you are
using an agent to go without. So you need a check that does not run through your eyes.

There are three, and none of them are AI (Artificial Intelligence):

- **Version control.** A checkpoint you can return to. The worst case of any change becomes
  the time it took, not the project. [d4](#d4-commit-well),
  [d10](#d10-undo-everything).
- **Tests.** Code that runs your code and complains when the answer is wrong. It does not
  get tired, and it does not take the agent's word for anything.
  [h1](#h1-what-a-test-is).
- **Small steps.** When one change is one feature wide, a break has one suspect. When it is
  ten features wide, you get to go looking.

None of that was invented for agents. It is what careful engineers were doing decades before
any of this, and it works here for the same reason it worked then: it turns "is this right"
from a judgment call into something you can measure. The measurement does not care how
confident the author was.

The models will keep getting better and this does not change. A faster writer with the same
error rate hands you more code you have not checked.

## Full

### Why "I will read it before I accept it" is not the plan

Reading generated code for correctness is harder than writing it, because you have to
reconstruct what it was trying to do from what it did. You are doing the difficult half of
programming without having done the easy half first.

Then there is volume. An agent produces more diff per hour than any person reads per hour.
"Review everything" survives about a week before it quietly becomes "skim everything," and a
skim catches the wrong category of problem. It catches ugly. It does not catch a condition
that is backwards in a branch you did not exercise.

Review is still worth doing. It is worth doing for the things a skim is actually good at:
scope, size, and anything you did not ask for. That is a different job from verifying
correctness, it takes about ninety seconds, and [h3](#h3-reviewing-a-diff-you-cannot-fully-read)
is the checklist.

### What each part of the net actually buys you

**Version control converts a disaster into a number.** Without it, "the app worked an hour
ago and I do not know what changed" is a research project. With it, that sentence is a list
of commits and a diff for each one, and the fix is often throwing the last one away. The
habit that makes it work is committing before you ask for anything nontrivial, so there is
always a known-good point behind you. Where your work lives at any moment is
[d3](#d3-the-three-places). Every way to go backward, ranked by what it costs, is
[d10](#d10-undo-everything).

**Tests are the only check that does not require you.** A test is an if statement with a
report attached: run this, compare it to what should happen, complain if they differ. You do
not have to be able to write one. You have to ask for one, and then confirm it can actually
fail. Working software you cannot verify is a rumor, not a result. Tests have their own
failure mode, where they pass while the thing is broken, and that is
[h6](#h6-when-tests-lie).

**Small steps shrink the search.** This one is arithmetic. Change one thing, run it, and a
failure has one suspect. Change ten things, run it, and a failure has ten suspects plus
every way they interact. The work is identical. The debugging is not.

### The same Tuesday, twice

Two people ask for the same feature: user accounts with email login.

The first asks for all of it in one go, comes back three hours later, and finds the login
page blank. Nothing is committed. The change spans nineteen files. Every one of them is
plausible. The options are reading all nineteen or deleting the folder.

The second asks for four things in turn: the database table, the signup form, the login
form, then the session handling. Each one gets run and committed. Same three hours. When the
login page comes up blank, the break is inside the last forty lines, and `git diff` fits on
one screen. Ninety seconds, not an evening.

Neither person read more code than the other. One of them arranged to have less to search.

### What this does not ask of you

- **Not to read every line.** [a5](#a5-what-you-still-have-to-know) draws that line honestly.
- **Not to distrust everything.** Hand-checking every change removes the reason you started.
  The point is to have a net, then move quickly above it.
- **Not to write tests yourself.** Ask the agent for them. Then ask it to show you the test
  failing before the fix, because a test that has never failed has proven nothing.
- **Not to work slowly.** Small steps are faster in wall-clock time, and the second Tuesday
  above is the whole argument.

### The habit, if you take one thing from this app

Commit before you ask for anything nontrivial. Run the check after. Keep the ask small
enough that you can say what it changed in one sentence.

Everything else here, all ten tracks of it, is an elaboration of that sentence.
[a4](#a4-the-loop) is the version with the steps in order.
