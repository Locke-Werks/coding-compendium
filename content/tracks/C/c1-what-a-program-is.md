---
id: c1-what-a-program-is
title: What a program actually is
type: section
track: C
order: 10
verified: 2026-08-02
volatility: low
answer: >
  A program is text you wrote, a translator that turns that text into
  instructions the processor can execute, and a process that exists only while
  it runs, and almost every error names which of those three stages failed.
owns:
  - source code vs executable
  - the basic model
see_also:
  - c2-compiled-vs-interpreted
  - c3-what-running-means
  - c5-processes-and-killing-them
  - f1-how-to-read-an-error-message
keywords:
  - what is source code
  - what is an executable
  - is code just text
  - binary file
  - what does the computer actually run
  - exe file
---

## More

Everything you open in an editor is a plain text file. `app.py`, `main.rs`, `server.js`:
open any of them in Notepad and you see the whole program, every character of it. Nothing is
hidden inside. That text is called **source code**, and by itself it does nothing.

Two more stages sit between the text and anything happening.

**A translator.** Some other program reads your text and turns it into instructions the
processor understands. Sometimes it does that once, ahead of time, and writes the result
into a `.exe` file you can double-click. Sometimes it does it fresh every time you run the
thing. Which of those your language does decides a lot about your day, and
[c2](#c2-compiled-vs-interpreted) is the card for it.

**A process.** When the instructions are actually executing, the operating system hands them
a slice of memory, a working folder, a copy of your environment settings, and a number to be
known by. That bundle is the process, it exists only while the program runs, and it
disappears the moment the program exits. [c5](#c5-processes-and-killing-them) covers
processes and how to end one that will not stop.

This model earns its keep the first time something breaks, because an error message almost
always belongs to exactly one of the three stages:

- **Translation failed.** `SyntaxError`, `expected ';'`, `error[E0308]: mismatched types`.
  Your text was not valid, so nothing ever ran.
- **The program could not be assembled.** `No module named 'requests'`, `Cannot find
  module`, `command not found`. Something it needed was missing before the work started.
- **It ran and hit a case it did not expect.** `TypeError`, `IndexError`,
  `NullReferenceException`. Real work happened first, and possibly a lot of it.

Sorting an error into one of those three narrows the fix enormously, because the first is
in your file, the second is in your setup, and the third is in your logic.
[f1](#f1-how-to-read-an-error-message) is the full method for reading the message.

One consequence that trips people constantly: opening a file in an editor does not run it,
and saving does not run it either. Something has to start the program on purpose.
[c3](#c3-what-running-means) is how.

## Full

### The three things people all call "the program"

They are different files and they behave differently.

1. **The source.** Text you or an agent wrote. This is what git tracks and what a diff
   shows. Small, readable, and the only one that matters to version control.
2. **The build output.** Whatever the translator produced: a `.exe`, a `dist` folder, a
   `target` folder, a `.jar`. Generated from the source, regenerated on demand, and
   deliberately excluded from git ([d12](#d12-gitignore-and-what-not-to-commit)). Deleting
   it is safe.
3. **The installed program.** Somebody else's build output, sitting in `C:\Program Files`
   or `C:\Users\<yourname>\AppData\Local\Programs`, that you run by name. `git`, `node`,
   and `python` are all this.

When an instruction says "build it", it means make number two out of number one. When it
says "run it", it means start a process from number two or number three.

### What "instructions the processor understands" actually means

At the bottom, a processor executes numbered operations: move this value here, add these
two, jump to that address if the result was zero. That is **machine code**, it is stored as
raw bytes rather than letters, and you will never read or write it. Open a `.exe` in
Notepad and you get pages of garbage characters, which is what text looks like when it was
never text.

The point of every programming language is to let you write something readable that becomes
those bytes. That is the entire job.

### A program needs a place to start

Source code is a pile of definitions. Something has to say which one runs first, and that is
the **entry point**. Rust and C look for a function called `main`. Node runs the file you
name on the command line. Python runs the file top to bottom, which is why you see
`if __name__ == "__main__":` guarding the part that should only fire when the file is the
one being run.

This is why "there is no error, it prints nothing" is a common false alarm. A file full of
function definitions with nothing calling them will run perfectly and produce silence.

### The same text, a different machine

Source code is portable. Everything around it is not. The same file that works on your
machine fails on another when:

- The translator is a different version, so syntax your code uses does not exist yet.
- A library your code imports is installed here and not there
  ([g1](#g1-what-a-dependency-is)).
- The program is looked up by name and that name is not found
  ([c4](#c4-path-and-command-not-found)).
- A setting the program reads from the environment is missing
  ([g5](#g5-environment-variables)).

None of those are bugs in the source, which is why "it works on my machine" is a statement
about the machine.

### Data is not a program

You will meet a lot of files that look like code and are not: `package.json`,
`tsconfig.json`, `settings.json`, `.yaml`, `.toml`, `.env`. Those are configuration, read by
a program that already exists. They have no entry point and cannot be run. A mistake in one
produces a complaint from whatever was reading it, usually naming a line and column, and
never a stack trace through your own code. [j2](#j2-the-config-formats-nobody-explains)
covers the formats and the traps in each.
