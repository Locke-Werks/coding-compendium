---
id: f7-reproducing-a-bug
title: Reproducing a bug on purpose
type: section
track: F
order: 70
verified: 2026-08-02
volatility: low
answer: >
  A bug you can trigger on command is most of the way to fixed, so write down the
  exact starting state and the exact steps first, then cut away every step that
  turns out not to be needed.
owns:
  - reproduction steps
  - minimal case
  - intermittent bugs
see_also:
  - h1-what-a-test-is
  - f5-what-to-paste-and-what-not-to
  - f6-when-the-agent-loops
  - h6-when-tests-lie
  - e5-prompting-that-works
keywords:
  - reproduce a bug
  - repro steps
  - it happens sometimes
  - flaky
  - minimal example
  - works on my machine
  - cant reproduce
  - intermittent
---

## More

If you cannot make the bug happen on demand, you cannot tell whether it is fixed. You get a
change, the symptom does not appear for an hour, and you have learned nothing.

A reproduction has five parts. Write them down, in a file, not in your head.

1. **Starting state.** Which branch, whether the database was empty, whether you were logged
   in, which folder you were standing in.
2. **The steps**, in order, exact. "Click Save" and "click Save twice quickly" are different
   bugs.
3. **What you expected.**
4. **What happened**, including the error text verbatim
   ([f5](#f5-what-to-paste-and-what-not-to)).
5. **The environment.** Windows 11, the version of Node or Python, the browser if it is a
   browser.

Then make it smaller. Take out a step and try again. If it still fails, that step was
irrelevant and it stays out. If it stops failing, put it back. Keep going until every
remaining step is load-bearing.

That reduction is worth real time. Ten steps involving three files becomes two steps and one
function, and at that size the cause is often visible without any debugging at all. It also
turns your bug report from a story into something an agent can act on directly.

The last move is to make the reproduction permanent by writing it as a test that fails now
and passes after the fix ([h1](#h1-what-a-test-is)). Then it is a bug that cannot come back
without something going red.

## Full

### A worked example of narrowing

The report: "the checkout page crashes sometimes."

- **Reproduce it once.** Add two items, apply a discount code, check out. Crash. Write down
  those three steps.
- **Remove a step.** Add two items, check out. No crash. So the discount code matters.
- **Change one thing.** One item plus the discount code. Crash. So the quantity does not
  matter.
- **Push on the one that does.** A different discount code. No crash. So it is that specific
  code.
- **Look at it.** That code is a percentage discount and the others are fixed amounts.

You now have "checkout crashes when a percentage discount is applied", which is a different
kind of sentence from where you started. Nothing here needed you to read the code.

### Cutting a large input in half

When the trigger is data rather than steps, use the same method on the data. Delete half the
rows and run it. Still fails, delete half of what is left. Stops failing, the problem was in
the half you removed, so put that back and cut the other half instead.

Ten rounds of that takes a 10,000 line file down to a single line. It feels crude and it is
the fastest method available.

### "It happens sometimes" and the four things it usually is

An intermittent bug is not random. It has a cause you have not spotted yet, and it is almost
always one of these.

**Order and timing.** Two things that normally finish in one order occasionally finish in the
other. Anything involving a network call, a file write, or a promise is a candidate. The tell
is that it fails more on a slow or busy machine, and more when everything runs at once.

**Leftover state.** The first run passes and the second fails, or the reverse. Something from
the previous run is still on disk, still in the database, or still in a cache. The tell is
that deleting a folder or restarting the database makes it go away for exactly one run.

**Something outside your machine.** A network hiccup, a rate limit, an external service. The
tell is that nothing about your code changed and the failure moved to a different day.

**Randomness.** Randomly generated identifiers, real timestamps, time zones, and results that
come back in a different order each time. The tell is a failure that depends on the clock:
tests that fail after 5pm, or only on the first of the month, are always this.

### Making an intermittent bug repeat

Run it a lot and count:

```powershell
$fails = 0
1..20 | ForEach-Object { npm test *> $null; if ($LASTEXITCODE -ne 0) { $fails++ } }
$fails
```

Three lines that must run together. The first sets a counter, the second runs the tests
twenty times and adds one to the counter each time the exit code is not zero
([f3](#f3-exit-codes-and-streams)), and `*> $null` throws away the output so you get a
number rather than twenty screens. The third prints the count.

Two out of twenty is a real bug with a real cause. Zero out of twenty means either you fixed
it or you have not found the condition yet, and those two look identical from here, which is
worth being honest with yourself about.

### Ruling out your own machine

"Works on my machine" is a statement about your machine, and the way to test it is to use a
different one. Short of that, a clean copy is close:

```powershell
git clone https://github.com/nyxlocke/site.git C:\Users\<yourname>\temp\site-check
```

Install and run in that folder. If it works there and not in your normal folder, the cause is
something not tracked by git: an uncommitted file, a `.env` that only you have, a cached
build, or an installed version that differs. That narrows it enormously and it takes two
minutes.

### The bug that only happens after deployment

Same method, different variables. The differences between your machine and the deployed one
are the suspect list: environment variables ([g5](#g5-environment-variables)), a real
database instead of a small local one, a build step that runs there and not here, and file
paths that are case-sensitive there and not here
([c7](#c7-files-folders-and-paths)). Work down that list rather than changing code.

### Do not accept a fix for a bug you cannot reproduce

An agent will happily produce a plausible fix for a symptom nobody can trigger. You will not
be able to test it, so you will accept it, and you will find out in a week whether it worked.

When you genuinely cannot reproduce something, say exactly that and ask for the smaller
thing: add logging at the three points where it could go wrong, then wait for it to happen
again with the evidence being recorded ([f4](#f4-logs)). That is slower and it converges.

### Write it down where it will still exist tomorrow

Put the reproduction in the issue, the pull request, or a scratch file in the repository. Two
reasons. It is the first thing anyone needs, including you in a month. And once the fix
lands, the reproduction is the checklist that proves it worked.
