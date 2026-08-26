---
id: python-indentationerror-unexpected-indent
title: "IndentationError: unexpected indent"
type: error
verified: 2026-08-02
volatility: low

language: python
category: wont-compile

# Parses the file without running it. No output means the indentation is legal
# now. Swap in your own file name.
verify: python -m py_compile main.py

sample: |
  PS C:\Users\you\dev\scraper> python main.py
    File "C:\Users\you\dev\scraper\main.py", line 12
      total = total + item
      ^
  IndentationError: unexpected indent

patterns:
  - "IndentationError"
  - "unexpected indent"
  - "expected an indented block"
  - "unindent does not match any outer indentation level"
  - "TabError"

means: >
  Python decides which lines belong to which block by how far they are indented, so
  indentation is part of the grammar rather than a style choice. This line is indented further
  than the line above allows. Nothing in the file ran, because Python reads the whole file and
  builds the structure before executing a single line.

fix_ladder:
  - try: Look at the line number in the error and the line directly above it.
    why: >
      Assumes a stray space or an extra level. The reported line is where Python noticed the
      problem, and the line above it is what set the expected depth. Comparing the two side by
      side is usually enough.

  - try: Turn on whitespace display in your editor.
    why: >
      Assumes you cannot see the difference, which is the normal situation. In Visual Studio
      Code, open the command palette with Ctrl+Shift+P, type "Toggle Render Whitespace", and
      run it. Spaces become faint dots and tabs become arrows, and a mixed line becomes
      obvious immediately.

  - try: Convert the whole file to spaces.
    why: >
      Assumes tabs and spaces are mixed, which is what `TabError` means specifically. Python
      counts a tab differently from the spaces it looks like, so a file that lines up perfectly
      on screen can still be wrong. In Visual Studio Code, click the "Spaces" or "Tab Size"
      indicator in the bottom bar and choose Convert Indentation to Spaces. Four spaces per
      level is the convention.

  - try: Check the line above ends with a colon, if the message says "expected an indented block".
    why: >
      Assumes the opposite problem. `if`, `for`, `while`, `def`, and `class` lines end with a
      colon and the next line has to be indented under them. An empty block is not allowed at
      all, and `pass` is the placeholder that fills one legally.

  - try: Let a formatter fix the whole file.
    command: python -m pip install black; python -m black main.py
    shell: powershell
    why: >
      Assumes the file has drifted in several places, which happens when code is pasted in from
      different sources. `black` rewrites indentation to one consistent style. It will not run
      on a file that cannot be parsed at all, so fix the reported line first.

if_none_worked: >
  Paste the whole error including the caret line, and about ten lines of the file around the
  reported line rather than the single line itself. The surrounding lines are what people trim,
  and indentation only means anything relative to its neighbors, so one line on its own tells
  an agent nothing.

see_also:
  - f1-how-to-read-an-error-message
  - c8-line-endings-and-encoding
  - python

keywords:
  - IndentationError
  - unexpected indent
  - expected an indented block
  - TabError
  - tabs vs spaces
---

Python is unusual in caring about this. Most languages use braces to mark blocks and treat
indentation as decoration. Python has no braces, so the indentation is the structure.

The practical consequence is that pasted code breaks. Copying four lines out of a chat window
into the middle of a function brings whatever indentation the chat had, and the result can
look correct and parse wrong.

Three related messages, all in this family. "unexpected indent" means a line is indented
further than anything allows. "expected an indented block" means a line ending in a colon has
nothing indented under it. "unindent does not match any outer indentation level" means a line
came back out to a depth that matches none of the open blocks.

`TabError` is the nastiest of them, because the file looks right. A tab and eight spaces are
identical on screen and different to Python. Pick spaces, convert the file once, and it stops
happening.
