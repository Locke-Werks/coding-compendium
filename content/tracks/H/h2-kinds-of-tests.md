---
id: h2-kinds-of-tests
title: Unit, integration, and end-to-end
type: section
track: H
order: 20
verified: 2026-08-02
volatility: low
answer: >
  A unit test checks one function with everything around it faked, an
  integration test checks real pieces working together, and an end-to-end test
  drives the whole app the way a person would. Speed drops and realism rises as
  you go down that list.
owns:
  - unit test
  - integration test
  - e2e test
  - the tradeoff between them
see_also:
  - h1-what-a-test-is
  - h6-when-tests-lie
  - h5-ci-cd
  - f7-reproducing-a-bug
  - h3-reviewing-a-diff-you-cannot-fully-read
keywords:
  - unit or integration
  - what kind of test do i need
  - end to end test
  - playwright
  - mocking
  - what should i test
  - test pyramid
---

## More

Tests come in three sizes, and the only real difference is how much genuine machinery each
one runs.

**A unit test** checks one function on its own, with everything around it faked. It runs in
milliseconds, you will have hundreds, and when one fails it names the exact function that
broke. It tells you nothing about whether your pieces fit together.

**An integration test** checks several real components working as a group: your code plus a
real database, or two modules talking to each other for real. Seconds rather than
milliseconds, dozens rather than hundreds. It catches the bugs that live in the gaps
between parts, which is where a large share of them live.

**An end-to-end test** drives the whole system the way a person would, usually through a
real browser: open the page, type an email, click sign in, expect the dashboard. Minutes,
and a handful at most. It is the closest thing to the truth that you can automate, and it
is also the slowest, the most expensive to maintain, and the most likely to fail for
reasons that have nothing to do with your code.

The tradeoff in one line: as the test gets more real, it gets slower and it gets vaguer
about what broke.

Which to ask for:

- A bug in one function: a unit test that reproduces it, then the fix
  ([f7](#f7-reproducing-a-bug)).
- Anything crossing a boundary, such as saving to a database or calling an API
  (Application Programming Interface): an integration test.
- The path your project exists for, such as sign up, buy, or submit: one end-to-end test.

The usual ratio is many unit tests, some integration tests, a few end-to-end tests. That is
economics rather than doctrine. The cheap ones you can afford to run on every save; the
expensive ones you run before you ship.

## Full

### The same feature, tested three ways

A sign-in form that rejects a wrong password.

**Unit.** Call `validatePassword("hunter2", storedHash)` directly and assert it returns
false. No form, no database, no network. Runs in under a millisecond. Proves the comparison
works and proves nothing about whether anything calls it.

**Integration.** Call the sign-in handler with a fake request against a real test database
holding one real user, and assert the response is a 401 status. Runs in a second or two.
Proves the handler, the lookup, and the comparison work together. Still no browser.

**End-to-end.** Launch a browser, load the app, type the email and a wrong password, click
sign in, and assert the error message appears on screen. Runs in twenty seconds. Proves the
whole thing works, including the part where somebody forgot to wire the error message into
the page.

Only the third one catches a broken form. Only the first one tells you in one line exactly
which function is wrong. That is the entire argument for having some of each.

### The tradeoff, laid out

| | Unit | Integration | End-to-end |
|---|---|---|---|
| Speed | milliseconds | seconds | minutes |
| How many | hundreds | dozens | a handful |
| When it fails, you know | the exact function | the area | that something is broken |
| Catches | logic errors | wiring between parts | anything a user would hit |
| Misses | anything between parts | anything in the interface | nothing, when it works |
| Breaks for no reason | almost never | sometimes | regularly |

That last row is the reason nobody builds a suite entirely from end-to-end tests, however
appealing that sounds. See [h6](#h6-when-tests-lie) for what a test that fails at random
does to your judgment.

### Mocks, and what they cost

A **mock** is a stand-in a test uses in place of something real and slow: a payment
provider, an email service, a database. The test hands your code a fake that returns a
fixed answer, which keeps the test fast and predictable.

Unit tests need them. They are also the main way a test drifts away from reality: your code
can pass every test against a mock that behaves nothing like the real service. If a
generated test mocks the very thing you were trying to check, the test is verifying the
mock. That failure has its own section in [h6](#h6-when-tests-lie).

Practical rule: mock things you do not own, such as a payment provider. Use the real thing
for what you do own, including your own database, which is what integration tests are for.

### Names you will hear

- **Smoke test.** A tiny end-to-end check that the app starts and the main page loads.
  Cheap, and worth having before anything else.
- **Regression test.** Any test written because a specific bug happened once, to make sure
  it does not come back. Usually a unit test with a good comment.
- **Snapshot test.** Records the output once and compares future runs against the
  recording. Useful for markup, and hazardous, because updating the snapshot when it fails
  is one keystroke and defeats the purpose.
- **Property test.** Generates hundreds of random inputs and asserts something that must
  hold for all of them. Excellent for parsers and math.
- **Load test.** Measures behavior under many simultaneous users. Not correctness, and not
  your problem yet.

### What to ask an agent for, by situation

- "Add a unit test for `<function>` covering the empty input and the boundary case."
- "Add an integration test that hits the real database and asserts the row is written."
- "Add one end-to-end test for the checkout flow, using the browser tooling this project
  already has."
- "This is the bug. Write a failing test that reproduces it before you change anything."

Naming the kind matters, because "add tests" produces whatever the agent feels like, which
is usually a pile of unit tests with mocks for everything, and those are the tests most
likely to pass while the app is broken.

### If you only have room for one

For a small project, one end-to-end test that opens the app and does the main thing catches
more real breakage than fifty unit tests written to raise a coverage number. It runs slowly,
it needs occasional maintenance, and it fails when your project is genuinely broken.

Start there, then add unit tests for the specific functions that turn out to be fiddly.
Every one of these runs in automation on push, which is [h5](#h5-ci-cd).
