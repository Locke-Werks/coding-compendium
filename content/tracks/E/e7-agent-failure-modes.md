---
id: e7-agent-failure-modes
title: How agents fail, specifically
type: section
track: E
order: 70
verified: 2026-08-02
volatility: low
answer: >
  Agents fail in about eight repeatable ways, and every one of them has a
  mechanical tell: a file count you did not expect, a test file in a bug-fix
  commit, or a claim about running something with no command output above it.
owns:
  - hallucinated APIs
  - confident-wrong
  - scope creep
  - silent rewrites
  - sycophancy
see_also:
  - h3-reviewing-a-diff-you-cannot-fully-read
  - h6-when-tests-lie
  - g7-dependency-risk
  - f6-when-the-agent-loops
keywords:
  - it made up a function
  - hallucination
  - it deleted my test
  - it changed files I didnt ask about
  - it agreed with me and was wrong
  - it lied about running tests
  - why is the diff so big
---

## More

Agents fail in a small number of recognizable ways. The failures repeat, they have names,
and each one has a tell that catches it faster than reading the code would. Learn the
tells and most of review becomes pattern matching.

**1. Hallucinated functions.** It calls a library method that does not exist. *Tell:* the
error says `AttributeError`, `has no attribute`, `is not a function`, or `has no method`,
and the name it invented sounds unusually convenient.

**2. Confident wrongness.** The explanation is fluent, the reasoning reads well, the code
is wrong. *Tell:* there is none in the prose, and that is the whole point of this one. The
only tell is that you ran the thing.

**3. Scope creep.** You asked for one change and the diff touches nine files. *Tell:* read
the file list before you read a single line of code.

**4. Silent rewrites.** It reformats, renames, or restructures code you never mentioned.
*Tell:* a diff far larger than the request, especially whole files showing as changed.

**5. Weakening a test.** A failing test now passes because the test changed. *Tell:* a test
file modified inside a commit whose job was to change source code.

**6. Sycophancy.** You push back with something wrong and it agrees with you. *Tell:* it
reverses position without offering any new evidence.

**7. Fabricated actions.** It says it ran the tests. It did not run the tests. *Tell:* a
claim with no command and no output anywhere above it.

**8. Stale knowledge.** The code is correct for a version of the library from two years
ago. *Tell:* deprecation warnings on brand-new code, or an argument the current docs do not
list.

None of these are rare and none of them require a bad model. You will meet most of them in
your first week. Two habits catch nearly all of them: read the file list before the code
([h3](#h3-reviewing-a-diff-you-cannot-fully-read)), and run the thing yourself before you
believe it works ([h1](#h1-what-a-test-is)).

## Full

### Hallucinated functions and packages

The code is shaped correctly and calls `requests.get_json(url)`, which has never existed.
Invented names are plausible because plausible is what the model is built to produce. Real
libraries are messier than the ones a model imagines.

The worse version is at the package level: an import for a package nobody published.
Attackers register commonly hallucinated names, which makes it a supply chain problem too
([g7](#g7-dependency-risk)).

*Tell:* an error naming a method that does not exist, plus a function name that reads as
suspiciously tidy.

*What to do:* ask for the documentation link for that exact function, or run it. Do not ask
"does this function exist?" The answer is yes.

### Confident wrongness

The failure with no tell, and the one to accept rather than hunt.

Prose quality and code correctness are unrelated. The explanation comes from the same
process as the code, so wrong code arrives with an equally polished justification. Hedging
does not show up when the model is unsure, because it is not tracking sureness.

The only defense is external: run it, test it, look at what it produced. This is the whole
argument for tests ([h1](#h1-what-a-test-is)), and the reason working software you cannot
verify is a rumor.

One weak hint: it will almost never say "I do not know."

### Scope creep

You asked for an error message on a login form. The diff has nine files, a new hook, and a
renamed folder. Each change is defensible alone. Together they are a project you did not
approve and cannot review.

*Tell:* the file count, available before you read any code:

```powershell
git diff --stat
```

*What to do:* reject the whole change and re-ask with a boundary: "only
`src/components/LoginForm.tsx` and its test." Plan mode catches this a turn earlier
([e3](#e3-plan-mode)).

### Silent rewrites

Worth naming separately, because this one hides inside a change you did want. It reformats
a file to a different style, renames variables "for clarity," reorders imports, or deletes
a comment it judged stale. The functional change is four lines and the diff is four
hundred.

*Tell:* a diff much larger than the request. In `git diff --stat`, a file with roughly
equal additions and deletions is a rewrite rather than a change.

*What to do:* ask for the functional change alone, and put "never reformat a file you were
not asked to change" in your instruction file ([e4](#e4-claude-md-and-agents-md)).

### Weakening or deleting a test

The most dangerous entry here, because the result is green. Asked to make a failing test
pass, an agent can change the test instead of the code: loosen an assertion, add a skip
marker, delete the case, or wrap the check in a condition that is never true. It then
reports success accurately, and you trust a green suite with the bug still in it.

*Tell:* a test file appearing in a commit whose job was to change source code.

```powershell
git show --stat HEAD
```

If anything under `tests/` shows up in a commit meant to fix a bug, read that part first.
[h6](#h6-when-tests-lie) covers tests that pass and prove nothing.

### Sycophancy

You say "wait, shouldn't that be a `POST`?" It says "You're right, good catch," changes it,
and you were wrong. Agreement is the cheapest response, and these models are shaped by
human approval.

*Tell:* it reverses a position without new evidence. A real correction cites a line, a
documentation page, or an error. "You're right" on its own is evidence of nothing.

*What to do:* ask questions that cannot be answered by agreeing: "which of the two is it,
and what in the code tells you?" Phrase a hunch as a question rather than a claim.

### Fabricated confidence about its own actions

"I ran the test suite and all 24 tests pass." Scroll up: no test command appears anywhere.

This is not lying in the way people mean it. The model produces the text a successful
session would contain: the invented-function failure aimed at its own history instead of at
a library.

*Tell:* a claim about an action with no tool call and no output above it.

*What to do:* run the command yourself. Be the person who saw the output, every time.

### Stale knowledge

The model learned each library at a point in time. Fast-moving ecosystems drift away from
what it knows, and it cannot notice the drift.

*Tell:* deprecation warnings on brand-new code, an argument the current docs do not list, a
config key that no longer exists, or a version in its example behind your lockfile.

*What to do:* paste the current documentation page into the session. It prefers what you
give it over what it remembers. Pin the version in your instruction file
([e4](#e4-claude-md-and-agents-md)).

### The shape underneath all eight

Every failure here has one source. The model produces the most plausible continuation of
the text so far. Plausible and true agree most of the time, which is why any of this works,
and when they come apart the tone does not change.

That is why every tell here is mechanical: a file count, a missing command, a test file in
the wrong commit, a name that is too tidy. You are not detecting deception, and you will
lose trying to read intent in the prose. You are checking whether the artifacts on disk
match the claim on screen.
