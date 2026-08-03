---
id: h1-what-a-test-is
title: What a test actually is
type: section
track: H
order: 10
verified: 2026-08-02
volatility: low
verify: npm test
answer: >
  A test is code that runs your code and complains when the answer is wrong. It
  is an if statement with a report attached, and the test runner is the program
  that finds every one of them and tells you which failed.
owns:
  - test
  - assertion
  - test runner
  - pass and fail
see_also:
  - h2-kinds-of-tests
  - h6-when-tests-lie
  - a2-the-honest-version
  - f3-exit-codes-and-streams
  - f7-reproducing-a-bug
keywords:
  - how do i run tests
  - what is an assertion
  - npm test
  - pytest
  - cargo test
  - what does passing mean
  - green and red
  - what does this test check
---

## More

A test is code that runs your code and complains when the answer is wrong. That is the
whole idea. There is no separate testing technology and nothing magic happening underneath:
a test is a small function that calls your function and checks what came back.

Here is a complete one, with no framework involved:

```python
def add(a, b):
    return a + b

result = add(2, 2)
if result != 4:
    print("failed: expected 4, got", result)
```

That is a real test. It runs the code and complains. Everything a testing framework adds is
convenience on top of exactly that: a way to find all your tests, one command to run them,
a report at the end, and a failure message better than the one you would have written.

The same test, written the way a framework wants it:

```python
def test_add_two_numbers():
    assert add(2, 2) == 4
```

`assert` is the **assertion**: the line stating what should be true. If the claim is false,
the test fails. If it is true, nothing happens and the test passes. That is the entire
mechanism, in every language, under every framework.

The **test runner** is the program that finds your tests and runs them. `pytest` for Python,
`cargo test` for Rust, `npm test` for JavaScript, which runs whichever runner the project
listed in its manifest. One command, one report, and a list of what failed.

A failing test suite is not a disaster. It is the product working. The failure arrived on
your screen instead of in front of somebody using the thing, which is the only difference
between the two and the reason any of this exists ([a2](#a2-the-honest-version)).

The runner finishes with an exit code of zero when everything passed and non-zero when
anything failed, which is what lets automation decide whether a push is good
([f3](#f3-exit-codes-and-streams)).

## Full

### The three steps inside every test

Nearly every test you will ever read has the same shape, whatever the language:

```javascript
test("applies the member discount", () => {
  const cart = { total: 100, member: true };     // set up the input
  const result = applyDiscount(cart);            // run the code
  expect(result.total).toBe(90);                 // check the answer
});
```

Set up the input, run the code, check the answer. When you look at an unfamiliar test and
want to know what it covers, read the last part first. The assertion is the claim, and
everything above it exists to make that claim checkable.

The names differ and the shape does not. Python uses `assert x == y`, JavaScript frameworks
use `expect(x).toBe(y)`, Rust uses `assert_eq!(x, y)`, C# uses `Assert.Equal(y, x)`. All
four are the same if statement.

### Running them

| Ecosystem | Command | Finds |
|---|---|---|
| Python | `pytest` | files named `test_*.py`, functions named `test_*` |
| JavaScript | `npm test` | whatever the `test` script in `package.json` runs |
| Rust | `cargo test` | functions marked `#[test]`, usually beside the code |
| C# | `dotnet test` | test projects in the solution |
| Go | `go test ./...` | files ending `_test.go` |

If `npm test` prints `Missing script: "test"`, the project has no tests configured, which is
a real answer to the question "are there tests." [c3](#c3-what-running-means) covers how a
project declares its commands.

### Reading the report

```text
tests/test_cart.py ..F..                                          [100%]

=================================== FAILURES ===================================
____________________________ test_member_discount ______________________________

    def test_member_discount():
        cart = {"total": 100, "member": True}
>       assert apply_discount(cart)["total"] == 90
E       assert 100 == 90

tests/test_cart.py:14: AssertionError
========================= 1 failed, 4 passed in 0.31s ==========================
```

Every part of that is readable once you know the layout. Each dot is a passing test and
each `F` is a failing one, printed as they run. The block underneath names the test that
failed. The line marked `>` is the assertion that broke. The line marked `E` is the
comparison it actually made: 100 came back where 90 was expected. The last line gives the
file and line number, and the footer is the count.

So: the discount was not applied. You now know what broke, where, and what the code
produced instead, without having read any of the source.

### What to ask an agent for

Two requests, and the order matters.

**"Write a test that fails on the current code, then fix the code."** This is how you get a
test that is worth having. A test written after the fix, against code that already works,
can pass without checking anything ([h6](#h6-when-tests-lie)). A test written to reproduce
the bug is proof the bug existed and proof it is gone.

**"Show me the test failing first."** Then the output above appears in your session, and
the claim that the fix works has evidence attached. An agent that says it ran the tests
without showing you the run is a known failure mode
([e7](#e7-agent-failure-modes)).

For a bug you can reproduce by hand, [f7](#f7-reproducing-a-bug) is the card on narrowing
it down to something a test can hold.

### What tests do not do

They do not prove your code is correct. They prove that the cases somebody wrote down
behave as that person expected. A function with ten tests and one unhandled empty-list case
has ten green tests and a bug.

They also do not test what you never asked about. This is the honest limit of the whole
practice, and the reason a green suite is a floor rather than a ceiling. Everything about
green suites that are lying to you is [h6](#h6-when-tests-lie).

### Where tests live

Most projects keep them in a `tests/` folder at the root, or beside each source file with a
matching name: `cart.js` and `cart.test.js`. Rust conventionally puts them in the same file
as the code, at the bottom, behind a marker that keeps them out of the shipped build.

You do not have to place them. The runner has a convention and your agent knows it. What is
worth knowing is that a test file appearing in a commit that was supposed to change source
code is one of the highest-value review signals you have
([h3](#h3-reviewing-a-diff-you-cannot-fully-read)).
