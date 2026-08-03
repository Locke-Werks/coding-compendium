---
id: python-typeerror-nonetype-not-subscriptable
title: "TypeError: 'NoneType' object is not subscriptable"
type: error
verified: 2026-08-02
volatility: low

language: python
category: broke-at-runtime

sample: |
  PS C:\Users\nyx\dev\scraper> python main.py
  Traceback (most recent call last):
    File "C:\Users\nyx\dev\scraper\main.py", line 14, in <module>
      print(data["items"][0])
            ~~~~^^^^^^^^^
  TypeError: 'NoneType' object is not subscriptable

patterns:
  - "'NoneType' object is not subscriptable"
  - "'NoneType' object is not"
  - "object is not subscriptable"
  - "NoneType"

means: >
  A value you treated as a container is actually `None`. `None` is Python's word for nothing at
  all, and it appears in three main ways: a function that finished without returning anything
  gives back `None`, a search that found no match returns `None`, and a variable set to `None`
  as a placeholder was never replaced. Square brackets on nothing is meaningless, so Python
  stops there. Everything before that line ran normally.

fix_ladder:
  - try: Print the value on the line above the failure.
    command: print("data is", data)
    shell: powershell
    why: >
      Assumes you need to see what is actually there rather than what you expect. This costs
      one line and answers the question directly. `None` confirms it, and anything else means
      you are looking at the wrong variable.

  - try: Find where the value came from and check that function returns something.
    why: >
      Assumes a missing `return`. A Python function without a `return` statement gives back
      `None`, silently and legally. A `return` sitting inside an `if` that did not run has the
      same effect. This is the most common cause by a wide margin.

  - try: Check whether a lookup found nothing.
    why: >
      Assumes a search came back empty. `re.match`, `dict.get`, and most database and web
      library calls return `None` when there is no result, rather than raising an error.
      Chaining straight into the result assumes success.

  - try: Check for a function that changes things in place.
    why: >
      Assumes you assigned the result of a method that returns nothing. `list.sort()`,
      `list.append()`, and `dict.update()` change the object and return `None`, so
      `rows = rows.sort()` throws away your list. Use `sorted(rows)` when you want a value
      back.

  - try: Handle the empty case rather than assuming it away.
    command: 'if data is None: raise ValueError("no data returned")'
    shell: powershell
    why: >
      Assumes `None` is a real possibility you have to plan for. Failing on purpose with a
      clear message beats failing three lines later with a confusing one. Do this once you
      know where the `None` comes from, never as a way to skip finding out.

if_none_worked: >
  Paste the whole traceback including the `~~~~^^^^` markers, the function that produced the
  value, and the line that calls it. The producing function is what people leave out because
  the error points somewhere else, and it is where the answer lives nearly every time.

see_also:
  - f2-stack-traces
  - f1-how-to-read-an-error-message
  - python

keywords:
  - NoneType is not subscriptable
  - object is not subscriptable
  - none type error
  - missing return
  - python None
---

Read the traceback from the bottom. The last line is the error, and the lines above it are the
path your program took to get there, oldest at the top. JavaScript prints its stack the other
way around, which trips people who move between the two.

The markers under the failing line are worth knowing. Python 3.11 and newer draw `~~~~` under
the part that evaluated fine and `^^^^` under the part that broke. In the sample that tells you
`data` is the problem and `["items"]` is where it surfaced.

The whole family behaves the same way. "not subscriptable" means you used square brackets.
"not callable" means you used parentheses. "not iterable" means you used it in a `for` loop.
All three mean the value is `None` and the fix is upstream of the line in the error.

The forgotten `return` is the one to check first. Python does not warn about a function that
falls off the end, so the mistake is invisible until something downstream tries to use the
nothing it produced.
