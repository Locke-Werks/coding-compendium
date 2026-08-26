---
id: python
title: Python
type: language
verified: 2026-08-02
volatility: low
verify: python --version

name: Python
aka: [py, py3, python3, cpython]
family: interpreted
likelihood: certain
extensions: ['.py', '.pyi', '.pyw']

# Every `note` is rendered as the evidence line in the identifier, so each one
# names the neighbor it is being told apart from. Python's nearest neighbor is
# Ruby almost every time, which is why Ruby appears in half of these.
tells:
  - pattern: 'def \w+\(.*\):'
    kind: regex
    weight: 8
    note: >
      Python and Ruby both declare with `def`. Python ends the line with a colon
      and indents the body. Ruby uses no colon and closes with `end`.
  - pattern: '\(self[,)]'
    kind: regex
    weight: 8
    note: >
      Python writes the object out as the first parameter of its own method.
      Ruby and JavaScript hand it over invisibly as `self` and `this`.
  - pattern: '__name__ == .__main__.'
    kind: regex
    weight: 10
    note: >
      The double-underscore guard at the bottom of a script exists in no other
      language on this deck. One sighting settles it.
  - pattern: 'from \w+ import'
    kind: regex
    weight: 9
    note: >
      Python puts the package first and the names second. JavaScript reverses it
      as `import { x } from "pkg"`. Ruby says `require`, PHP says `use`.
  - pattern: 'elif'
    kind: token
    weight: 8
    note: >
      Ruby spells it `elsif`, PHP spells it `elseif`, the brace languages write
      `else if`. Bash also uses `elif`, but a Bash block closes with `fi`.
  - pattern: 'f"'
    kind: regex
    weight: 8
    note: >
      An `f` in front of a quote is an f-string and `{name}` inside it gets
      substituted. Ruby writes `#{name}`, JavaScript uses backticks and
      `${name}`, C# uses `$"`.
  - pattern: '\b(True|False|None)\b'
    kind: regex
    weight: 8
    note: >
      Python capitalizes its booleans and calls nothing `None`. JavaScript and
      PHP write `true`, `false`, and `null` in lowercase. Ruby writes `nil`.
  - pattern: '^\s*@\w+'
    kind: regex
    weight: 6
    note: >
      A line that is only `@something`, sitting directly above a `def`, is a
      decorator. Java puts `@Override` above a method too, but Java wraps the
      body in braces and Python indents it.

rules_out:
  - pattern: 'elsif'
    because: Ruby, which spells Python's `elif` with an s.
  - pattern: '^\s*end\s*$'
    kind: regex
    because: Ruby, Lua, or Elixir. Python closes a block by outdenting, never with a word.
  - pattern: 'function'
    because: JavaScript, TypeScript, or PHP.
  - pattern: '=>'
    kind: operator
    because: A JavaScript arrow function or a C# lambda. Python's `lambda` has no arrow.
  - pattern: '\$\w+'
    kind: regex
    because: PHP, Bash, or PowerShell, which all mark variables with a sigil. Python never does.
  - pattern: '<\?php'
    kind: regex
    because: PHP.
  - pattern: 'nil'
    because: Ruby, Lua, or Go. Python spells it `None`.

project_fingerprint:
  manifests:
    - file: pyproject.toml
      decisive: true
      note: >
        The modern manifest. The file is written in TOML (Tom's Obvious Minimal
        Language), so the manifest and the code it describes are two different
        languages living in one folder. The `.toml` extension tells you the file
        format, the `[project]` table inside tells you the project is Python.
    - file: requirements.txt
      decisive: true
      note: >
        A flat list of package names, one per line, sometimes pinned with `==`.
        Older and dumber than pyproject.toml and still everywhere. Nothing keeps
        it in step with what is actually installed, which is why it drifts.
    - file: setup.py
      decisive: true
      note: >
        The oldest of the three, and Python source code rather than config, so it
        executes when the package installs. A project whose only manifest is this
        one has not been touched in years.
    - file: Pipfile
      decisive: true
      note: >
        Pipenv's manifest, TOML again, always paired with Pipfile.lock. Less
        common than it was in 2018, common enough to recognize.
  lockfiles: [poetry.lock, Pipfile.lock, uv.lock, pdm.lock]
  build_dirs: ['__pycache__/', '.venv/', 'venv/', 'build/', 'dist/', '*.egg-info/']
  entry_points: [main.py, app.py, manage.py, '__main__.py']

shape:
  blocks: indentation
  statement_end: newline
  comment_line: '#'
  comment_block: >
    None. Three quotes in a row open a multi-line string, and one sitting under a
    `def` is a docstring, which is documentation rather than a comment.
  string_quotes: >
    Single and double quotes mean exactly the same thing. Three of either spans
    lines. An `f` in front interpolates `{variables}`.
  naming: snake_case for functions and variables, CamelCase for classes, SCREAMING_SNAKE_CASE for constants, a leading underscore for internal things
  import_keyword: import

tooling:
  package_manager: pip, with uv and Poetry as faster front ends over the same registry
  registry: PyPI, the Python Package Index
  runtime: CPython, installed separately. On Windows the command is `python`, or `py` when several versions are installed. `python3` is the Mac and Linux spelling and often fails here.
  install_command: pip install <package-name>
  run_command: python <file>.py
  test_command: pytest

confusable_with:
  - language: ruby
    settle_it: >
      Both declare with `def` and both read like English. Python puts a colon
      after the signature and indents the body; Ruby needs no colon and closes
      with `end`. Python writes `elif` and takes `self` as the first parameter;
      Ruby writes `elsif` and marks instance variables `@name`.
    tiebreak: { pattern: '^\s*end\s*$', kind: regex, favors: ruby }
  - language: javascript
    settle_it: >
      Both are dynamically typed and both are everywhere. JavaScript wraps every
      block in curly braces and declares with `const`, `let`, or `function`.
      Python has no declaration keyword at all, opens blocks with a colon, and
      closes them by outdenting.
    tiebreak: { pattern: '=>', kind: operator, favors: javascript }
  - language: r
    settle_it: >
      They sit side by side in data work and both comment with `#`. R assigns
      with `<-` and declares with `function(x)`. Python assigns with `=` and
      declares with `def`.
    tiebreak: { pattern: '<-', kind: operator, favors: r }
  - language: sql
    settle_it: >
      A Python file often holds SQL inside a quoted string, so uppercase `SELECT` and
      `FROM` on the screen do not make the file SQL. If the lines around them contain
      `def`, `import`, or an `=`, the file is Python and the SQL is a string it hands
      to a database at runtime. A standalone `.sql` file has none of that around it.
    tiebreak: { pattern: 'def ', kind: regex, favors: python }

errors_look_like:
  sample: |
    Traceback (most recent call last):
      File "C:\Users\<yourname>\project\app.py", line 20, in <module>
        main()
      File "C:\Users\<yourname>\project\app.py", line 12, in total
        return price * quantity
               ~~~~~~^~~~~~~~~~
    TypeError: can't multiply sequence by non-int of type 'str'
  recognize_by: >
    The word `Traceback` alone on the first line, then indented pairs of
    `File "...", line N, in <name>` and the source line itself, then the error
    type and message on the last line. Python 3.11 and later also draw `~~~^~~~`
    carets under the exact expression. The order is the giveaway: the oldest call
    is printed first and the failure is printed last, which is the reverse of a
    JavaScript stack trace, where the message comes first.
  patterns:
    - '^Traceback \(most recent call last\):'
    - '^\s+File "[^"]+", line \d+'
    - '^\w*(Error|Exception): '

meet_it_when: >
  An agent reaches for Python by default whenever you ask for a script, a data
  cleanup, a scraper, or a small service, because it needs no build step and has
  a library for everything. You also meet it when a tool you installed fails and
  prints a traceback, and when a Jupyter notebook lands in a repo you cloned.

what_agents_get_wrong: >
  Six things, in rough order of how often they bite. A default argument that is a
  list or a dict, as in `def add(item, items=[])`, is created once and shared by
  every call, so the list quietly grows between calls; the fix is `None` as the
  default. A bare `except:` swallows every error including your Ctrl+C, turning a
  crash into a hang or a silently wrong answer, so search any diff for `except:`
  and for `except Exception: pass`. Invented packages: an agent will write
  `import requests_toolkit` with total confidence, and pip either fails or
  installs whatever a stranger registered under that exact name. Python 2 idioms
  leak in from old training data, usually `print` with no parentheses. Agents
  forget the virtual environment and install into your global Python, which works
  today and breaks the next project. And the one to check every single time: a
  new `import` line at the top of a file with nothing added to requirements.txt
  or pyproject.toml. It runs on your machine because the package is already
  there, and it fails for everyone else.

version_landscape: >
  Python 2 and Python 3 are different languages that share a name, and this is
  the reason to check the date on every answer you find. Python 2 reached its end
  of life in 2020, but a page from 2015 is still the top search result for plenty
  of questions, and it can be actively wrong rather than merely old. `print "hi"`
  was correct Python 2 and is a syntax error now. A snippet with `print` missing
  its parentheses, or one importing `urllib2`, is Python 2: keep scrolling. Inside
  Python 3 the minor version still matters, because f-strings need 3.6, `match`
  needs 3.10, and libraries drop old minors quickly. Run `python --version` before
  you trust anything.

see_also:
  - ruby
  - javascript
  - r
  - g4-environments-and-isolation
  - f2-stack-traces
  - g7-dependency-risk
  - c2-compiled-vs-interpreted
  - j1-how-to-recognize-a-language

keywords: [pip, venv, virtualenv, pypi, traceback, pytest, django, flask, jupyter, notebook, conda]
---

The language an agent reaches for first. Dynamically typed, no build step, and a
library for nearly everything.

Named after Monty Python, not the snake. The logo has two snakes on it anyway.

## The shape

Indentation is the block. Python is the only language in this deck where
whitespace changes what the program means: four spaces to the right puts a line
inside the `if`, four spaces back takes it out. There are no braces to close and
no `end` to write.

A line that opens a block ends in a colon. The colon says the next line is
indented.

Statements end at the end of the line. Semicolons are legal and nobody writes
them.

```python
count = 3                     # no declaration keyword: no let, no var, no const
MAX_ITEMS = 100               # a constant by naming convention only

def add(a: int, b: int) -> int:
    return a + b              # indented, so it sits inside the function

if count > 2:
    print(f"{count} is plenty")   # the f makes {count} substitute
elif count == 2:
    print("two")
else:
    print("not enough")
```

Comments are `#` to the end of the line, and there is no block comment form.
Three quotes in a row open a string that spans lines, and one sitting directly
under a `def` is a docstring, which is documentation the tooling reads.

Two more things appear on every page of real Python. A method takes `self` as its
written-out first parameter, because Python passes an object to its own method
explicitly where Ruby and JavaScript do it invisibly. And a line that is only
`@something` above a `def` is a decorator: it wraps the function underneath in
extra behavior without touching the body.

## Six lines of it

```python
import json

def main() -> None:
    scores = {"ada": 10}
    print(json.dumps(scores))

if __name__ == "__main__":
    main()
```

Those last two lines mean "run this only if I was started directly, not if
something imported me." Nothing else in common use has that guard, so one glance
at it ends the argument.

## What it is for

Scripts, automation, scraping, data analysis, machine learning, and web backends
through Django or FastAPI.

Python needs Python installed to run, which is why it wins on servers and on your
own machine and loses to Rust and Go for desktop apps. It compiles to bytecode in
a `__pycache__` folder on the way, so that folder is generated, git-ignored, and
safe to delete.

Packages install into a virtual environment: a per-project folder, usually
`.venv`, holding that project's own copies. Skip it and everything lands in one
global pile where two projects fight over one version of one library. See
[g4-environments-and-isolation](#g4-environments-and-isolation).

## Reading its errors

Python prints a **traceback**, the most recognizable error format in software.

```text
Traceback (most recent call last):
  File "C:\Users\<yourname>\project\app.py", line 12, in total
    return price * quantity
TypeError: can't multiply sequence by non-int of type 'str'
```

Read the last line first. It names the error type and says what went wrong. Then
walk up the `File` lines and stop at the topmost one that points inside your own
project rather than inside a library.

The order is what catches people. The oldest call is at the top and the thing
that actually broke is at the bottom. A JavaScript stack trace prints it the
other way around.
