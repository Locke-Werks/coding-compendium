---
id: python-keyerror
title: "KeyError: 'x'"
type: error
verified: 2026-08-02
volatility: low

language: python
category: broke-at-runtime

sample: |
  PS C:\Users\you\dev\scraper> python main.py
  Traceback (most recent call last):
    File "C:\Users\you\dev\scraper\main.py", line 9, in <module>
      timeout = config["timeout"]
                ~~~~~~^^^^^^^^^^^
  KeyError: 'timeout'

patterns:
  - "KeyError"
  - "KeyError:"

means: >
  You asked a dictionary for a key it does not have. A dictionary is a lookup table of names to
  values, and asking for a name that is not in it is an error rather than an empty answer. The
  name in the message is exactly what you asked for, quotes and capitals included. The
  dictionary itself is fine, and everything up to that line ran.

fix_ladder:
  - try: Print the keys that actually exist.
    command: print(list(config.keys()))
    shell: powershell
    why: >
      Assumes a spelling or capitalization difference. Dictionary keys are case sensitive and
      `"Timeout"` and `"timeout"` are two different keys. Seeing the real list next to the name
      you asked for answers this in one look, and it is faster than reading the file that
      built the dictionary.

  - try: Check whether the key is optional.
    command: timeout = config.get("timeout", 30)
    shell: powershell
    why: >
      Assumes the key is genuinely missing sometimes and that is allowed. `.get()` returns
      `None` instead of raising, and a second argument supplies a default. Use this where
      absence is normal, such as an optional setting.

  - try: Look at where the dictionary came from.
    why: >
      Assumes the data is not what you think. A dictionary built from a JSON (JavaScript Object
      Notation) file, a web response, or a database row has whatever shape the source gave it.
      Print the whole thing once and read it rather than trusting the documentation.

  - try: Check for a nesting level you skipped.
    why: >
      Assumes the key exists one layer deeper. Web responses commonly wrap results in an outer
      object, so the key you want is at `config["settings"]["timeout"]` rather than at the top
      level. Printing the whole dictionary shows the shape immediately.

  - try: Fail with a message that says what was missing.
    command: 'if "timeout" not in config: raise ValueError("config is missing timeout")'
    shell: powershell
    why: >
      Assumes the key should always be there and its absence means something upstream is
      broken. A clear error at the point of loading beats a bare `KeyError` twenty lines later,
      and it tells whoever reads it what to fix.

if_none_worked: >
  Paste the whole traceback and the printed contents of the dictionary, not a description of
  what it should contain. The actual contents are the piece people summarize, and the entire
  question is how the real data differs from what the code expects.

see_also:
  - f2-stack-traces
  - j2-the-config-formats-nobody-explains
  - python

keywords:
  - KeyError
  - dictionary key missing
  - dict get default
  - key not found python
  - json key error
---

A `KeyError` is a statement of fact and rarely a bug in the dictionary. Something built that
dictionary with different contents than the code reading it expects.

The three sources worth checking in order: an environment variable that is not set, a JSON (JavaScript Object Notation)
(JavaScript Object Notation) response whose shape changed, and a typo. All three produce an
identical message.

`.get()` is the right tool for optional values and the wrong tool for required ones. Replacing
a `KeyError` with `.get()` when the key really should be there converts a loud failure into a
silent `None` that breaks somewhere less obvious. Agents do this constantly when asked to make
an error go away.

`os.environ["API_KEY"]` deserves a specific mention. That raises `KeyError` when the variable
is not set, which on Windows usually means it was set in a different terminal window, or set
with `setx` and this window has not been reopened since.
