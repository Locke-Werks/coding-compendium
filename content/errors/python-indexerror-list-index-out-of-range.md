---
id: python-indexerror-list-index-out-of-range
title: "IndexError: list index out of range"
type: error
verified: 2026-08-02
volatility: low

language: python
category: broke-at-runtime

sample: |
  PS C:\Users\you\dev\scraper> python main.py
  Traceback (most recent call last):
    File "C:\Users\you\dev\scraper\main.py", line 11, in <module>
      first_row = rows[0]
                  ~~~~^^^
  IndexError: list index out of range

patterns:
  - "IndexError"
  - "list index out of range"
  - "string index out of range"
  - "tuple index out of range"

means: >
  You asked for an item at a position the list does not have. Python counts from zero, so a
  list of three items has positions 0, 1, and 2, and asking for position 3 is past the end.
  When the position is 0, the list is empty, and that is a different problem with a different
  cause: something produced no results at all.

fix_ladder:
  - try: Print the length of the list before the failing line.
    command: print("rows has", len(rows), "items")
    shell: powershell
    why: >
      Assumes you do not know how many items there are. Zero means the list is empty and the
      question is why nothing was found. Any other number means an off-by-one mistake, and the
      two need completely different fixes.

  - try: Check whether you are off by one.
    why: >
      Assumes the list has items and you counted from one. The last item of a list of three is
      at position 2, not 3. `rows[-1]` is the reliable way to get the last item whatever the
      length, and `rows[len(rows) - 1]` is the version people write by hand and get wrong.

  - try: Find out why the list is empty.
    why: >
      Assumes zero items and a cause further upstream. A file that was not found and read as
      empty, a web request that returned no results, a filter that matched nothing, or a split
      on the wrong separator all produce an empty list quietly. The error is at the read, and
      the bug is at the build.

  - try: Loop over the list instead of indexing into it.
    command: 'for row in rows: print(row)'
    shell: powershell
    why: >
      Assumes you are walking the list by number. Looping directly cannot go out of range and
      handles the empty case by doing nothing, which is usually what you want. Most manual
      index arithmetic in Python is a habit from other languages.

  - try: Check the empty case on purpose.
    command: 'if not rows: raise ValueError("no rows found in the file")'
    shell: powershell
    why: >
      Assumes empty means something is wrong and you want to say so clearly. This turns a
      confusing `IndexError` at line 11 into a statement about the actual problem at the point
      where the data was supposed to arrive.

if_none_worked: >
  Paste the whole traceback, the code that builds the list, and the printed length and contents
  of the list. The code that builds it is what people leave out because the error points at the
  line that reads it, and an empty list is always caused somewhere else.

see_also:
  - f2-stack-traces
  - f7-reproducing-a-bug
  - python

keywords:
  - IndexError
  - list index out of range
  - empty list
  - off by one
  - string index out of range
---

Two errors share one message, and telling them apart is the whole skill.

Index 0 failing means the list is empty. That is never a counting mistake. Something that was
supposed to produce data produced nothing, and the fix is upstream. A file read that found no
lines, a scrape that matched no elements, a query that returned no rows.

Any other index failing is a counting mistake, and Python counting from zero is why. The first
item is at 0 and the last is at one less than the length.

A related trap: negative indexes are legal and count backward from the end, so `rows[-1]` is
the last item. That means a wrong calculation producing `-1` does not raise an error at all.
It quietly returns the wrong item, which is worse than crashing.

Slices behave differently again. `rows[0:5]` on a two-item list gives you both items without
complaining, because slicing clamps to what exists.
