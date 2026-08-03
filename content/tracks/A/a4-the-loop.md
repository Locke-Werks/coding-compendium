---
id: a4-the-loop
title: "The loop: describe, review, test, checkpoint"
type: section
track: A
order: 40
verified: 2026-08-02
volatility: low
answer: >
  Describe one small piece, make it plan before it writes, read the diff for
  scope rather than syntax, run it, then commit. Repeat. The size of one turn is
  what makes the rest of it work.
owns:
  - the core working rhythm
see_also:
  - d4-commit-well
  - h3-reviewing-a-diff-you-cannot-fully-read
  - e3-plan-mode
  - a2-the-honest-version
  - d5-branches
keywords:
  - workflow
  - how do i actually work with an agent
  - the rhythm
  - what order do i do things
  - checkpoint
  - one feature at a time
---

## More

One turn of the loop, start to finish:

1. **Describe one piece.** One feature, not five. If you cannot say what changed in one
   sentence afterward, it was two pieces.
2. **Make it plan first.** Both tools will lay out the approach before touching a file.
   Reading a plan takes thirty seconds and reading the resulting diff takes ten minutes, so
   this is the cheapest place to catch a wrong assumption. [e3](#e3-plan-mode).
3. **Read the diff.** You are checking shape, not syntax: is this the file I expected, is it
   the size I expected, did it touch anything I did not ask about, did the test count go
   down. [h3](#h3-reviewing-a-diff-you-cannot-fully-read) is the checklist and
   [d9](#d9-reading-a-diff) is how to read the symbols.
4. **Run it.** The tests if there are tests, the app itself if there are not. This is the
   step people skip, and it is the only one that produces evidence.
   [h1](#h1-what-a-test-is).
5. **Commit.** A checkpoint with a message saying what changed.
   [d4](#d4-commit-well).

Then go again.

Two things sit around the loop rather than inside it. Work on a branch so the last known-good
version of `main` stays untouched, which is [d5](#d5-branches). Push and open a pull request
when the piece is done, which is [d8](#d8-pull-requests).

The reason this is a loop instead of a checklist is step size. Every property that makes the
work survivable comes from keeping one turn small: the diff stays readable, the failure has
one suspect, the checkpoint behind you is recent, and the agent's context stays about one
thing. Large turns break all four at once.
[a2](#a2-the-honest-version) is the argument for why that matters.

## Full

### Each step, with the way it goes wrong

**Describe.** The failure is vagueness. "Make the login better" gets you somebody's guess at
better. Name the file if you know it, state the constraint, say what done looks like.
[e5](#e5-prompting-that-works) is full of worked examples.

**Plan.** The failure is approving a plan you skimmed. Read for the assumptions rather than
the steps: a plan that says "migrate the existing users table" when you did not know there
was one has told you something valuable. Push back in plain language and ask for a new plan.

**Build.** The failure is watching it work. You do not need to. Let it finish the piece, and
spend the attention on the diff instead, where the evidence actually is.

**Review.** The failure is reading line by line, running out of patience at file four, and
approving the rest. Read the file list first. A change to a file you did not expect is worth
more of your attention than any single line inside a file you did.

**Run.** The failure is taking "all tests pass" from the agent as the result. Run it
yourself, or at minimum look at the output it printed. An agent that has quietly weakened a
test to get green will report green, honestly. [h6](#h6-when-tests-lie).

**Commit.** The failure is the giant commit at the end of the day, which throws away the
whole benefit of having checkpoints. [d4](#d4-commit-well) covers message format and size.

### How big is one turn

The rules of thumb, in order of usefulness:

- You can say what it does in one sentence with no "and".
- The diff is under a few hundred lines.
- There is one obvious way to tell whether it worked.
- It touches one area of the project.

When an ask fails those tests, split it before you send it. "Add user accounts" is four
turns: the data, the signup form, the login form, the session. Each one runs, each one gets
committed, and when the fourth breaks you are looking at the fourth.

### A worked turn

```powershell
git switch -c feature/csv-export
```

`-c` means create the branch and move onto it.

Ask for the piece. Something like: "Add a button on the reports page that downloads the
current table as a CSV (Comma-Separated Values) file. Do not change how the table is
loaded. Plan first."

Read the plan. Approve or redirect. Let it build. Then:

```powershell
git status
git diff
```

`git status` lists which files changed. `git diff` shows the lines. You are looking for the
four things in step 3, which takes about ninety seconds.

Run it. Click the button. Open the file it downloaded.

```powershell
git add -A
git commit -m "feat: add CSV export to the reports page"
```

`-A` stages every changed file. `-m` supplies the message inline. Then push and open the
pull request, which is [d8](#d8-pull-requests), or take the next turn on the same branch.

### When the loop stops working

Three specific stalls, each with its own card:

- **It fixes the error and a new one appears, three times running.** It is guessing. Stop
  it, make it state what it now thinks the cause is before it changes anything else.
  [f6](#f6-when-the-agent-loops).
- **It forgets a decision you made an hour ago, or reintroduces a bug you already fixed.**
  The session is full. Start a fresh one with a short handoff note.
  [e6](#e6-when-to-reset-context).
- **The change came back much bigger than the ask.** That is scope creep, it is a known
  failure mode, and the response is to throw the change away and ask again more narrowly
  rather than to salvage it. [e7](#e7-agent-failure-modes).

Throwing a turn away costs nothing when you committed before it started. That is the entire
reason the commit is in the loop.

### The two steps people drop first

Review and run, in that order, usually in week two, usually because a run of good turns made
them feel unnecessary. They fail the same way: nothing goes wrong today. The cost arrives
later, when the break is somewhere in three days of unreviewed, untested, unchecked work and
there is no recent point where the thing was known to be good.

The loop is cheap while it is a habit and expensive to restart after it lapses. Keep the
turns small and the rest of it stays easy.
