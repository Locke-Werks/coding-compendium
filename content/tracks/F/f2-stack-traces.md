---
id: f2-stack-traces
title: Stack traces
type: section
track: F
order: 20
verified: 2026-08-02
volatility: low
answer: >
  A stack trace is the list of function calls that were waiting on each other
  when something broke, and which end carries the actual message depends on the
  language: Python prints it last, JavaScript and Java print it first.
owns:
  - stack trace
  - call stack
  - reading order per language
see_also:
  - f1-how-to-read-an-error-message
  - f5-what-to-paste-and-what-not-to
  - j1-how-to-recognize-a-language
  - python
  - javascript
  - java
keywords:
  - stack trace
  - traceback
  - call stack
  - at lines
  - which line do i read
  - caused by
  - most recent call last
  - wall of red text
---

## More

Programs call functions, and those functions call other functions. While the inner one is
working, every function above it is stopped part way through, waiting. That waiting queue is
the **call stack**, and when something fails the language prints the whole thing so you can
see the route it took to get there. That printout is the **stack trace**.

Most of the skill is knowing which end to start at, and the answer is different per language.

| Language | The message is | Read from | Spot it by |
|---|---|---|---|
| Python | at the **bottom**, on the last line | bottom up | the word `Traceback` on line one |
| JavaScript and Node | at the **top**, above the frames | top down | lines starting with `at`, ending `file:line:column` |
| Java | at the **top**, on the first line | top down | `Exception in thread`, then `at` lines |
| C# | at the **top** | top down | `at` lines ending `in C:\path\File.cs:line 12` |
| Rust | at the **top** | top down | `thread 'main' panicked at` |

Python is the odd one out, and it tells you so. `Traceback (most recent call last)` is a
literal instruction: the calls are listed oldest first, so the thing that actually broke is
at the end. Every other trace you will meet leads with the message.

Once you have the message, the second move is the same in every language: **find the topmost
frame that points into your own project.** Frames mentioning `node_modules\`,
`site-packages\`, `node:internal/`, or `java.base/` are somebody else's code. The failure
surfaced there. The cause is almost always the first line of yours above or below it.

A trace is not a bug report and you do not have to understand every line. Two lines carry
almost all the information: the message and the first frame that is yours. The rest is
context you paste along with them ([f5](#f5-what-to-paste-and-what-not-to)).

## Full

### Python: the answer is on the last line

```text
Traceback (most recent call last):
  File "C:\Users\you\project\app.py", line 20, in <module>
    main()
  File "C:\Users\you\project\app.py", line 12, in total
    return price * quantity
TypeError: can't multiply sequence by non-int of type 'str'
```

Read it bottom to top.

- **Last line.** `TypeError: can't multiply sequence by non-int of type 'str'`. That is what
  went wrong. Python multiplied a piece of text by something that is not a whole number.
- **Two lines above it.** `app.py, line 12, in total`, and then the actual line of code,
  `return price * quantity`. That is where it went wrong.
- **Above that.** `line 20, in <module>` calling `main()`. That is how it got there.
  `<module>` means the top level of the file rather than a function.

Reading downward instead means starting with `line 20`, which is correct and useless: line 20
is fine, it called something that was not.

Python 3.11 and later also draw `~~~~^^^^` carets under the exact expression that failed,
which helps when one line contains four function calls.

### JavaScript and Node: the answer is at the top

```text
C:\Users\you\shop\server.js:12
  const total = cart.items.length;
                     ^

TypeError: Cannot read properties of undefined (reading 'items')
    at checkout (C:\Users\you\shop\server.js:12:22)
    at Object.<anonymous> (C:\Users\you\shop\index.js:4:1)
    at Module._compile (node:internal/modules/cjs/loader:1356:14)
```

Read it top to bottom.

- **The header block.** Node quotes the offending file, line, and the source itself, with a
  caret under the part that broke. This is the fastest read in the whole family.
- **The message line.** `TypeError: Cannot read properties of undefined (reading 'items')`.
  This one is worth translating: `cart` exists, `cart.items` is `undefined`, and you asked
  `undefined` for its `length`.
- **The `at` lines, newest first.** `checkout` broke, `checkout` was called from `index.js`
  line 4, and that was called by Node's own module loader. The numbers on the end are
  `line:column`.
- **Anything with `node:internal/`** is Node itself. Stop reading there.

Browsers print the same shape in the console, with the stack collapsed behind a triangle you
have to click and the file name on the right of each row.

### Java: the answer is at the top, and then check the bottom

```text
Exception in thread "main" java.lang.NullPointerException: Cannot invoke
"String.length()" because "name" is null
    at com.example.app.App.greet(App.java:14)
    at com.example.app.App.main(App.java:7)
```

Read the first line for the type and the message, then the first `at` line for the location.
The type is a dotted path, `java.lang.NullPointerException`, and the last segment is the part
worth searching for. Frames are newest first, same as JavaScript, and end with a file name
and line number in parentheses.

Java adds one thing nothing else does, and it changes the answer:

```text
Exception in thread "main" java.lang.IllegalStateException: Failed to start
    at com.example.app.Server.start(Server.java:41)
Caused by: java.net.BindException: Address already in use
    at java.base/sun.nio.ch.Net.bind(Net.java:555)
```

When you see `Caused by:`, the real failure is the **last** one in the chain. The lines above
it are wrappers, each one catching the error beneath and rethrowing it with its own label.
Scroll to the final `Caused by:` and read that. Everything above is packaging.

### The other two you will meet

**C#** looks almost exactly like Java. Its frames end with `in C:\path\Program.cs:line 12`
instead of putting the location in parentheses, which is the fastest way to tell the two
apart.

**Rust** does not print a stack by default. It gives one line, `thread 'main' panicked at
src/main.rs:14:9`, the message, then a note telling you to set an environment variable for
the rest:

```powershell
$env:RUST_BACKTRACE = 1; cargo run
```

That sets the variable for this terminal only and gives you the full trace on the next run.

### Frames that are not yours

Whatever the language, most of a long trace is library code. The tells:

- `node_modules\` or `node:internal/` for JavaScript.
- `site-packages\` or `lib\python3.12\` for Python.
- `java.base/`, `org.springframework.`, or anything with a package name that is not yours.

You are looking for the frame with your own folder in it. If there is not one, a framework
called your code indirectly and the message is the useful clue.

### Traces that are shortened or wrong

**Truncated.** `... 12 more` in a Java trace, or `Show 15 more frames` in a browser console,
means repeated frames were hidden. Expand them only if the visible part names nothing of
yours.

**Async.** Code that ran later, after a promise or a callback, often has a short and
unhelpful stack, because the functions that queued the work already returned and are no
longer on the stack. The trace is honest and the useful history is missing. Log what you know
at the point the work is queued rather than fighting the trace ([f4](#f4-logs)).

**Minified.** A browser trace pointing at `main.a3f9c.js:1:24601` is production JavaScript
compressed onto one line. It is not readable. Reproduce it against the development build
instead ([f7](#f7-reproducing-a-bug)).

**Deep and repeating.** Hundreds of copies of the same two frames, ending in
`RecursionError` or `Maximum call stack size exceeded`, means a function called itself with
no way out. The fix is at the one place those frames loop.

### What to do with it

Copy the whole thing, not the part you think matters: every frame, including the ones from
libraries you did not write, plus the command you ran.
[f5](#f5-what-to-paste-and-what-not-to) explains why the parts you would cut are the parts
the agent uses.
