---
id: h6-when-tests-lie
title: When tests pass and the thing is still broken
type: section
track: H
order: 60
verified: 2026-08-02
volatility: low
verify: npm test
answer: >
  A test can pass because it asserts nothing, because an agent weakened it,
  because it checks a mock instead of your code, or because it never ran at all.
  If you have never seen a test fail, you do not know that it works.
owns:
  - false confidence
  - tests that assert nothing
  - deleted tests
  - flakiness
see_also:
  - e7-agent-failure-modes
  - h1-what-a-test-is
  - h3-reviewing-a-diff-you-cannot-fully-read
  - h2-kinds-of-tests
  - f7-reproducing-a-bug
keywords:
  - tests pass but its broken
  - green but wrong
  - the test doesnt actually check anything
  - false confidence
  - flaky test
  - why did the test not catch this
  - it deleted a test to make it pass
  - coverage is high but
---

## More

Working software you cannot verify is a rumor, not a result. A green test suite is the
usual way to verify, which makes a green suite that proves nothing worse than no suite at
all: it converts "I do not know" into "I checked."

Four ways a test passes while the thing is broken.

**It asserts nothing useful.** The test calls your function and checks that it did not
crash. Or it asserts that the result is not null. Or it checks that a fake was called, which
tests the fake. The code ran, and nobody ever wrote down what the answer should have been.

**An agent weakened it to get green.** Asked to make a failing test pass, changing the test
is easier than fixing the code. An assertion gets loosened, a case gets skipped, an expected
value gets replaced with whatever the code currently produces. The report is accurate and
the bug is still there ([e7](#e7-agent-failure-modes)).

**It passes for the wrong reason.** The function name has a typo so the runner never
collected it. The assertion sits after an early return and never executes. The mock behaves
nothing like the real service. Or the test was written by reading the code, so it asserts
what the code does rather than what it should do, bugs included.

**It is flaky.** It passes and fails on identical code, because of timing, shared state,
test ordering, a real network call, or an unseeded random value. This one is corrosive
beyond the individual test, because it teaches you to rerun instead of read, and once you
rerun until green you have stopped testing anything.

One check catches all four:

**Break the code on purpose and confirm the test goes red.** Change a return value, invert
a condition, delete a line. Run the tests. If they stay green, the test is decorative. Put
the code back. Thirty seconds, and it is the only thing that proves a test is real.

## Full

### Tests that assert nothing

```python
def test_get_user():
    user = get_user(1)
    assert user is not None
```

This passes if `get_user` returns the wrong user, an empty object, or the string "banana."
It proves the function returned something. The stronger version names the answer:
`assert user.email == "nyx@example.com"`.

```javascript
expect(saveMock).toHaveBeenCalled();
```

This asserts that the test's own fake was called. It says nothing about what was saved or
whether the real save works. Fine as one line among several, hollow as the only assertion
in a file.

Snapshot tests add a step: when one fails, the runner offers to update the recording, and
accepting turns a caught regression into a documented one.

This is also why **coverage** is not evidence: it measures which lines ran, not whether
anything about them was checked ([coverage](#coverage)).

### The suite an agent weakened

The specific markers, worth recognizing on sight:

| Marker | Language | Effect |
|---|---|---|
| `@pytest.mark.skip` | Python | the test never runs, and the report says so quietly |
| `it.skip` or `xit` | JavaScript | same |
| `.only` | JavaScript | every other test in that file stops running |
| `#[ignore]` | Rust | same as skip |
| `assert True` | any | an assertion that cannot fail |
| a widened tolerance | any | `abs(x - y) < 0.5` where it used to be `0.001` |

Checking for these belongs in every review
([h3](#h3-reviewing-a-diff-you-cannot-fully-read)). Git answers directly:

```powershell
git log --oneline -- tests/
```

Lists every commit that touched your test folder. A bug-fix commit in that list is worth
opening.

### Passing for the wrong reason

**The runner never collected it.** `pytest` only runs functions whose names begin with
`test_`. A function named `check_login` is dead code that nobody flags. The tell is the
count: you wrote six tests and the report says five passed.

**The assertion never executed.** An early return, a condition that is never true, or a
`try` block that swallows the failure ([h4](#h4-what-good-looks-like) on swallowed errors).
The test passes because nothing in it ran.

**The mock diverged from reality.** Your fake payment service returns success; the real one
started requiring a field six months ago. Every test passes and every payment fails. A mock
freezes an assumption, and nothing tells you when it expires
([h2](#h2-kinds-of-tests)).

**The test describes the bug.** This is the one specific to generated tests. Ask an agent to
"write tests for this file" and it reads the implementation and asserts what it sees. If the
function returns 90 where it should return 85, the test now demands 90, and it will block
the fix later.

The defense is the order of operations from [h1](#h1-what-a-test-is): the test gets written
from the requirement or from the bug report, and it fails before the fix exists.

### Flaky tests

A flaky test passes and fails on identical code. The usual causes, in rough order:

- **Time.** A `sleep(1)` that is usually long enough. A test that breaks at midnight or in
  another timezone.
- **Shared state.** One test leaves a row in the database and the next one counts rows.
- **Order.** The suite passes in order and fails when a runner shuffles or parallelizes.
- **The real network.** Anything reaching a live service will eventually be slow.
- **Randomness.** Generated data with no fixed seed, so one run in forty hits the case
  nobody handled.

The thing not to do is rerun until it passes and move on. Two weeks of that and nobody reads
a red result at all.

Fix it or delete it. A deleted flaky test is honest about what you know. A flaky test kept
around is a test you have already agreed to ignore.

One exception worth respecting: an end-to-end test that fails intermittently is sometimes
reporting a real race condition, on the same days your users hit it. Before calling a test
flaky, check whether it is right ([f7](#f7-reproducing-a-bug)).

### Making a test prove itself

The mutation check, in full:

1. Note the current result: all green.
2. Open the code the test is supposed to cover and break one thing. Turn
   `return total * 0.9` into `return total`. One line.
3. Run the tests.
4. Red means the test was watching. Green means it was not, whatever its name says.
5. Undo your change with `git restore <file>` and confirm you are back to green.

Do this once per important test and never again. Half a minute, and it is the difference
between a suite you trust and a suite you hope about.

### The three numbers to glance at

**The test count** should go up over time, so a change that reduced it needs an explanation.
**The skipped count** is printed on every run and skimmed past by everyone, and a skipped
test is a test you are not running. **The duration** falling by a factor of five means
something stopped happening.

### What to ask an agent

- "Show me the test failing before you fix it."
- "Which tests would fail if I emptied the body of `applyDiscount`?"
- "List every assertion in this file and what each one actually proves."
- "Are any tests skipped, and why?"

The second is the strongest: a specific answer is checkable, and a vague one tells you the
suite is not covering what you assumed.
