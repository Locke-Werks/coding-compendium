---
id: python-attributeerror-no-attribute
title: "AttributeError: object has no attribute 'x'"
type: error
verified: 2026-08-02
volatility: low

language: python
category: broke-at-runtime

sample: |
  PS C:\Users\you\dev\scraper> python main.py
  Traceback (most recent call last):
    File "C:\Users\you\dev\scraper\main.py", line 18, in <module>
      data = session.fetch_json(url)
             ^^^^^^^^^^^^^^^^^^
  AttributeError: 'Session' object has no attribute 'fetch_json'. Did you mean: 'get_json'?

patterns:
  - "AttributeError"
  - "object has no attribute"
  - "has no attribute"
  - "module .* has no attribute"

means: >
  You asked an object for a name it does not have. In Python, everything after a dot is looked
  up on the object to the left of the dot, and this one has no such method or property. The
  object exists and is the type you expected, or close to it. The specific name you used is
  wrong, or belongs to a different version of the library, or was never real.

fix_ladder:
  - try: Read the rest of the message.
    why: >
      Assumes Python already found the intended name. Version 3.10 and later add
      "Did you mean:" with the closest match on the object. When that suggestion appears, it is
      right nearly every time and the fix is a one-word edit.

  - try: List what the object actually offers.
    command: print([n for n in dir(session) if not n.startswith("_")])
    shell: powershell
    why: >
      Assumes you need the real list. `dir()` returns every name on an object, and filtering out
      the ones starting with an underscore leaves the ones meant for you. This beats searching
      documentation, because it describes the version you actually have installed.

  - try: Check whether the agent invented the method.
    command: python -m pip show <package>
    shell: powershell
    why: >
      Assumes a hallucinated interface. Agents produce method names that sound exactly right and
      have never existed, and this error is what that looks like from the outside. Compare the
      installed version against the library's own documentation for that version.

  - try: Check whether the object is what you think it is.
    command: print(type(session))
    shell: powershell
    why: >
      Assumes the variable holds something unexpected. `'NoneType' object has no attribute`
      means the value is `None` and the real problem is further upstream, which is a different
      card. `'str' object has no attribute` means you have a string where you expected a parsed
      object.

  - try: Check the library version against the code.
    command: python -m pip list
    shell: powershell
    why: >
      Assumes the method existed once and was renamed or removed. Libraries drop methods
      between major versions, and code written against an older release fails exactly this way.
      Either upgrade the code or pin the older version deliberately.

  - try: Check for a name that belongs to the module rather than the object.
    why: >
      Assumes a mix-up between the two. `module 'x' has no attribute 'y'` means you called
      something on the module itself that lives on one of its classes, or the other way round.
      It is the same error with a different left-hand side.

if_none_worked: >
  Paste the whole traceback, the output of `python -m pip show <package>` for the library
  involved, and the line of code that failed. The installed version number is what people leave
  out, and half of these are a version difference rather than a mistake in the code.

see_also:
  - e7-agent-failure-modes
  - f2-stack-traces
  - g7-dependency-risk
  - python

keywords:
  - AttributeError
  - object has no attribute
  - method does not exist
  - hallucinated method
  - did you mean python
---

This is the error that catches agents inventing things.

A model that has read a great deal of Python knows what library methods usually look like, and
will produce a name that fits the pattern perfectly without ever checking whether it exists.
`session.fetch_json()` sounds exactly as real as `session.get_json()`. Only one of them is in
the library.

The tell is that the code looks completely reasonable and fails on the first run at the exact
line that does the interesting work. Compare that with a typo, which usually fails somewhere
duller.

`dir()` settles it faster than any amount of searching, because it reports the version
installed on your machine rather than whatever version the documentation on screen describes.

The `'NoneType' object has no attribute` variant is worth recognizing on sight. That one is
not about a wrong name at all. The value is `None`, and something upstream returned nothing.
