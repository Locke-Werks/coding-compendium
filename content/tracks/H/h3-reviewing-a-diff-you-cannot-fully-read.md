---
id: h3-reviewing-a-diff-you-cannot-fully-read
title: Reviewing a diff when you cannot read every line
type: section
track: H
order: 30
verified: 2026-08-02
volatility: low
verify: git diff --stat
answer: >
  Review the shape of the change rather than the syntax: check that it touched
  the files you expected, at the size you expected, without weakening a test,
  adding a dependency you did not ask for, or committing a secret.
owns:
  - review heuristics
  - scope checking
  - the review checklist
see_also:
  - d9-reading-a-diff
  - e7-agent-failure-modes
  - h4-what-good-looks-like
  - h6-when-tests-lie
  - d8-pull-requests
  - g6-secrets-and-what-never-to-commit
keywords:
  - how do i review code i dont understand
  - review checklist
  - what do i look for
  - i cant read the code
  - is this change safe
  - review a big diff
  - how do i approve this
  - did it change too much
---

## More

You are not auditing syntax. Reading a hundred lines of an unfamiliar language line by line
would not tell you much even if you did it, and it is not the job. The job is checking the
shape of the change against what you asked for, and six questions cover it in about ninety
seconds. [d9](#d9-reading-a-diff) explains what the symbols in a diff mean. This is what to
look for.

Start with one command:

```powershell
git diff --stat
```

One line per file with counts of what changed, and no code at all. It answers the first
three questions before you have read anything.

**1. Is this the file I expected?** You asked for a change to the login form, so the login
form should be in the list. A file you have never heard of is the single most reliable
warning sign on this page.

**2. Is the change the size I expected?** A one-line request that produced four hundred
changed lines is a different change from the one you approved. Roughly equal additions and
deletions in one file usually means it was rewritten rather than edited.

**3. Did it touch anything I did not ask about?** Config files, workflow files, the
formatter's settings, folders unrelated to the request. Each change may be defensible on
its own. Together they are a second project you did not review.

**4. Did it delete or weaken a test?** A test file appearing in a commit whose job was to
change source code is the highest-value warning in this list, because the result is green
and wrong.

**5. Are there new dependencies?** `package.json`, `requirements.txt`, `Cargo.toml`. A new
package is a permanent decision, and it gets made in passing.

**6. Are there secrets?** A key, a token, a `.env` file that no ignore rule covers. Rare,
expensive, and cheap to check.

When an answer comes back wrong, the response is not "fix it." Reject the change and ask
again with a boundary: "only `src/LoginForm.tsx`, no new packages, do not touch the tests."
Re-running a small request costs less than untangling a large diff.

## Full

### The whole pass, in four commands

```powershell
git diff --stat
git diff -- package.json
git diff --stat -- tests/
git diff | Select-String -Pattern "key|secret|token|password|BEGIN "
```

File list and size, dependency changes, test changes, credential scan. Swap `package.json`
for whichever manifest your project uses. Run these against staged work with `--staged`, or
against a finished commit with `git show --stat` instead of the first one.

That is the entire review for a change you cannot read. Everything below is what to do when
one of them says something interesting.

### 1. The file list

Read the paths, not the numbers, and compare them against the sentence you typed. Three
patterns matter:

- **A file you did not expect at all.** Ask what it is doing there before anything else.
  The answer is sometimes good ("it needed a new component") and sometimes reveals that the
  agent solved a different problem.
- **More files than the request implies.** One behavior change touching nine files is worth
  a question. It might be a rename that rippled, and it might be scope creep
  ([e7](#e7-agent-failure-modes)).
- **Files nobody edits by hand.** Lockfiles, generated types, build output. A lockfile
  changing when no manifest changed means something ran an update
  ([g3](#g3-lockfiles)).

### 2. The size

`--stat` prints something like `src/cart.js | 12 ++++++++----`. Twelve lines touched, eight
added, four removed. Two readings are worth having:

**Much bigger than expected.** Ask for the functional change alone, or ask what the extra
lines do. A four-line fix inside a four-hundred-line diff is a reformat wearing a fix as a
disguise.

**Additions and deletions almost exactly equal, across a whole file.** That is a rewrite.
Sometimes it is a formatter that ran on save, in which case this makes it readable:

```powershell
git diff --ignore-all-space
```

If the diff mostly disappears, it was whitespace, and on Windows it may have been line
endings rather than anything anyone did ([c8](#c8-line-endings-and-encoding)).

### 3. Territory you did not ask about

The specific paths worth a second look every time:

- `.github/workflows/` : the automation that decides whether your tests must pass
  ([h5](#h5-ci-cd))
- `.gitignore` : quietly hiding files from review
- `package.json` scripts : changing what `npm test` actually runs
- `tsconfig.json`, `.eslintrc`, `pyproject.toml` : loosening a rule instead of fixing what
  the rule caught

None of these are forbidden. All of them change the rules of the game rather than the game,
which is why they deserve an explicit "I asked for this" before they land.

### 4. Tests

The most valuable check on this page, and the one nobody thinks to run.

```powershell
git diff --stat -- tests/
```

If a commit meant to fix a bug touches a test file, read that part in full even if you skip
everything else. What you are looking for:

- A test **deleted** outright.
- A skip marker added: `@pytest.mark.skip`, `it.skip`, `#[ignore]`, `@Ignore`.
- `.only` added to one test, which silently stops every other test in the file from running.
- An assertion loosened: an exact match becoming "contains", or a specific value becoming
  "not null".
- The expected value changed to whatever the code now produces, which turns the test into a
  recording of the bug.

Each of these makes a red suite green without fixing anything, and each is a documented
agent behavior ([e7](#e7-agent-failure-modes)). [h6](#h6-when-tests-lie) covers what to do
about a suite you no longer trust.

### 5. New dependencies

```powershell
git diff -- package.json
```

A new line under `dependencies` means a package you now own: it ships, it needs updating,
and it brought its own dependencies with it ([g1](#g1-what-a-dependency-is)). Ask two
questions. What does it do that we could not do with what we have, and is that name exactly
right ([g7](#g7-dependency-risk)).

### 6. Secrets

```powershell
git diff --staged | Select-String -Pattern "key|secret|token|password|BEGIN "
```

`Select-String` is PowerShell's text search and `-Pattern` takes the words to look for. This
produces false positives constantly, which is fine, because the output is short and you are
scanning it rather than reading it. A real hit means stop and go to
[g6](#g6-secrets-and-what-never-to-commit) before you commit anything.

### Doing this on GitHub instead

A PR (Pull Request) page runs the same checklist with a mouse
([d8](#d8-pull-requests)). The **Files changed** tab is the file list, the count at the top
is the size, the settings menu there has a hide-whitespace option, and each file has a
**Viewed** checkbox that collapses it so you can work through a long list without losing
your place.

### What this does not catch

Logic. A change can pass all six checks and still be wrong, because correctness does not
show up in the shape of a diff. That is what running the thing is for
([h1](#h1-what-a-test-is)), and it is why the loop is review, then test, then commit rather
than review and commit ([a4](#a4-the-loop)).
