---
id: c2-compiled-vs-interpreted
title: Compiled, interpreted, and the middle ground
type: section
track: C
order: 20
verified: 2026-08-02
volatility: low
answer: >
  A compiled language turns your whole program into a machine-code file before
  anything runs, so you can hand someone one file, while an interpreted language
  translates as it goes, so every machine that runs it needs the interpreter
  installed first.
owns:
  - compiler
  - interpreter
  - bytecode
  - JIT
  - runtime
see_also:
  - c1-what-a-program-is
  - c3-what-running-means
  - j1-how-to-recognize-a-language
  - g2-package-managers
  - i3-builds-and-artifacts
keywords:
  - what is a compiler
  - what is an interpreter
  - why does python need python installed
  - bytecode
  - virtual machine
  - build step
  - why is there no exe
---

## More

Both approaches solve the same problem: your text has to become instructions the processor
can execute. They differ on when that happens.

**A compiler does it all at once, before you run anything.** You run a build command, it
reads every file, checks the whole thing, and writes out a finished program. Rust, Go, C,
and C++ work this way. What you get is a `.exe` on Windows that runs on a machine with none
of the tools installed. The build takes time, seconds to minutes, and you pay it every time
you change the code.

**An interpreter does it while running.** There is no build step. You point the interpreter
at your file, it works through the code as it goes, and it stops when it reaches something
broken. Python and Ruby work this way, and JavaScript does under Node. What you get is
instant startup and no `.exe` at all, which is why running a Python script on a machine
without Python installed produces `'python' is not recognized` rather than anything useful.

The consequence you feel first is when errors show up.

A compiler refuses to produce a program at all if any part of your code is wrong, so a typo
in a function you never call still stops the build. That is annoying on Tuesday and
excellent on Friday. An interpreter finds the same typo only when execution reaches that
line, which can be after twenty minutes of real work, or in the one branch a user hits at
midnight.

Between the two sits **bytecode**: a compressed intermediate form that is not machine code
and not your source. Java and C# compile to it, then a **virtual machine** executes it.
That buys them one build that runs on Windows, macOS, and Linux, at the cost of needing the
virtual machine installed. Python quietly does a version of this too, which is what the
`__pycache__` folder full of `.pyc` files is.

None of this changes how you start the thing. That is [c3](#c3-what-running-means), and the
answer is always in the project's own files.

## Full

### The compiled path, step by step

You run `cargo build` or `go build`. The compiler parses every source file, checks types and
references across all of them, optimizes, and writes one executable. Then you run that
executable, and the compiler is no longer involved in any way.

Three things follow:

- **Distribution is easy.** One file, no prerequisites. This is why command-line tools you
  install and forget about, `git` and `rg` and `gh`, tend to be written in compiled
  languages.
- **The feedback loop is slower.** Change one line, wait for the build. Large projects use
  incremental builds to rebuild only what changed, which helps and does not eliminate it.
- **The build output is disposable.** It lives in `target\` or `bin\`, git ignores it, and
  deleting it costs you nothing but a rebuild ([i3](#i3-builds-and-artifacts)).

### The interpreted path, step by step

You run `python main.py`. The `python` program starts, reads your file, and begins
executing. Your file is data to it, in the same way a `.docx` is data to Word.

- **There is nothing to ship but the source.** Which is also the problem: the target machine
  needs the right interpreter, at a compatible version, with your libraries installed. Most
  of track G exists because of this sentence.
- **Half a program can run before it fails.** Files get written, rows get inserted, and then
  line 200 raises. This is normal and it is why a failed run can leave a mess behind.
- **The version matters more than you expect.** Python 3.11 and 3.12 are different programs.
  Code using newer syntax fails on the older one with a `SyntaxError` that looks like your
  mistake.

### The middle: bytecode and a virtual machine

Java compiles `.java` into `.class` files full of bytecode, and the JVM (Java Virtual
Machine) executes those. C# compiles to a similar intermediate form that the .NET runtime
executes. The bytecode is portable; the virtual machine is the part that has to be built for
your operating system.

Recognizing this explains a category of error message. Anything mentioning a version
mismatch between a class file and a runtime, or a missing runtime entirely, is this stage
complaining. It is a setup problem, not a code problem.

### Compiling while it runs, which blurs the line

Modern JavaScript engines and the JVM both use JIT (Just In Time) compilation. They start by
interpreting, watch which functions run most, and compile those to real machine code while
the program is running. So "interpreted" is a description of the workflow rather than a
promise about speed.

Practical effect: the first run of a hot loop is slower than the hundredth. If you are
timing something, the first measurement is a lie.

### "Runtime" means two different things

Both are common and context tells them apart.

- **The runtime** is a program or library your code needs while it executes. Node is a
  JavaScript runtime. The .NET runtime runs C# programs.
- **At runtime** means while the program is running, as opposed to at build time. "A runtime
  error" is one that happened during execution rather than during compilation.

### Telling which one you are looking at

The fastest signal is the project's own files, which is the same signal used for identifying
the language at all ([j1](#j1-how-to-recognize-a-language)).

- A `Cargo.toml`, `go.mod`, or `CMakeLists.txt` means compiled. Expect a build step and a
  build folder.
- A `requirements.txt`, `pyproject.toml`, or `Gemfile` means interpreted. Expect the
  interpreter to be a prerequisite.
- A `package.json` means JavaScript or TypeScript on Node, which is interpreted, except that
  TypeScript itself is compiled to JavaScript first by a step called transpiling. Both facts
  are true at once.
- A `pom.xml`, `build.gradle`, or `.csproj` means bytecode and a virtual machine.

Whatever the answer, the command that starts it is written down in that same file, and
[c3](#c3-what-running-means) is how to find it.
