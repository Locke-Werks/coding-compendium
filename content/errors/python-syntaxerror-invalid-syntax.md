---
id: python-syntaxerror-invalid-syntax
title: "SyntaxError: invalid syntax"
type: error
verified: 2026-08-02
volatility: low

language: python
category: wont-compile

# Parses without running. Silence means the file is legal Python now.
verify: python -m py_compile main.py

sample: |
  PS C:\Users\you\dev\scraper> python main.py
    File "C:\Users\you\dev\scraper\main.py", line 8
      if count = 3:
         ^^^^^^^^^
  SyntaxError: invalid syntax. Maybe you meant '==' or ':=' instead of '='?

patterns:
  - "SyntaxError"
  - "invalid syntax"
  - "was never closed"
  - "EOL while scanning string literal"
  - "unmatched"

means: >
  Python could not make sense of the text as code. It stopped at the first place the grammar
  broke, so nothing in the file ran, including the parts that are fine. The reported line is
  where Python gave up rather than where the mistake is, and those are often different lines
  when the cause is an unclosed bracket or quote.

fix_ladder:
  - try: Read the rest of the message after "invalid syntax".
    why: >
      Assumes Python already worked out what you meant, which modern versions often do. The
      sample says "Maybe you meant '==' or ':=' instead of '='". A single `=` assigns a value
      and a double `==` compares two, and mixing them up is the most common cause of this
      error.

  - try: Look at the line above the one reported.
    why: >
      Assumes an unclosed bracket or quote. Python keeps reading across lines while a bracket
      is open, so it only complains when it hits something impossible on the next line. The
      real mistake is a missing `)`, `]`, `}`, or closing quote further up.

  - try: Count the brackets on the previous few lines.
    why: >
      Assumes you are looking in the right place and need to find the mismatch. Newer Python
      says "'(' was never closed" and points at the opening bracket, which is much more useful.
      An editor that highlights matching brackets does the same job by eye.

  - try: Check for a missing colon.
    why: >
      Assumes a block header is incomplete. Every `if`, `for`, `while`, `def`, `class`, `try`,
      and `else` line ends with a colon. Python reports the error on the following line when
      one is missing, which sends you looking in the wrong place.

  - try: Check whether the code is Python 2.
    why: >
      Assumes the code came from an old tutorial. `print "hello"` without parentheses is
      Python 2 and produces this error under Python 3. Anything using `raw_input` or
      `except Exception, e` is from the same era, and there is a lot of it still online.

  - try: Parse the file without running it.
    command: python -m py_compile main.py
    shell: powershell
    why: >
      Assumes you have made an edit and want to know whether it parses. This checks the syntax
      and produces no output when the file is legal, which is faster than running a program
      that does real work.

if_none_worked: >
  Paste the whole error including the caret markers, and the fifteen lines above the reported
  line. The lines above are what people cut because the error names one line, and for an
  unclosed bracket the reported line is genuinely innocent.

see_also:
  - f1-how-to-read-an-error-message
  - j1-how-to-recognize-a-language
  - python

keywords:
  - SyntaxError
  - invalid syntax
  - unclosed bracket
  - missing colon
  - python 2 print
---

The single most useful habit with this error is to distrust the line number.

Python reports where it could no longer continue, which is where the mistake shows up rather
than where it is. An unclosed parenthesis on line 6 produces an error on line 7, or on line
20 if the lines in between happen to be legal continuations.

The caret markers under the line are more reliable than the line number. They point at the
exact text Python choked on. When they sit under something that looks perfectly fine, that is
your signal to look upward.

Newer Python versions are considerably better at this. Version 3.10 onward names the specific
mistake and points at the opening bracket that was never closed. If you are getting the bare
"invalid syntax" with no hint, check `python --version`, because an old interpreter is doing
you no favors.
