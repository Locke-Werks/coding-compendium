---
id: c
title: C language
type: language
verified: 2026-08-02
volatility: low

name: C
aka: [ansi-c, c99, c11, c17, clang, k-and-r]
family: compiled
likelihood: possible
extensions: ['.c', '.h']

# Every `note` below is the evidence line the identifier shows, so each one is
# written against the neighbor it gets mistaken for. C and C++ share most of
# their surface, so the C tells that matter are the ones C++ replaced.
tells:
  - pattern: '#include'
    kind: line_start
    weight: 7
    note: >
      A line starting with `#include` is C or C++ and nothing else in this deck.
      Rust says `use`, Python and Java say `import`, Go puts `import` inside
      parentheses. It does not separate C from C++ on its own, so check the header
      name next.
  - pattern: '#include <\w+\.h>'
    kind: regex
    weight: 6
    note: >
      A standard header ending in `.h`, as in `<stdio.h>`, is the C form. C++
      standard headers have no extension at all: `<vector>`, `<string>`,
      `<iostream>`.
  - pattern: 'malloc'
    kind: token
    weight: 8
    note: >
      C asks for memory by hand with `malloc` and gives it back with `free`. C++
      uses `new` or `std::make_unique`, Rust uses `Box::new`, and Go, Python, and
      JavaScript do it for you without a word.
  - pattern: 'printf'
    kind: token
    weight: 6
    note: >
      C prints with `printf` and a format string. C++ prefers `std::cout <<`, Rust
      uses `println!`, Go uses `fmt.Println`, Python uses `print`.
  - pattern: 'void \*'
    kind: regex
    weight: 7
    note: >
      A `void` pointer means "an address, type unknown, you keep track". C++ has
      templates instead and rarely needs one. Rust, Go, Java, and C# have no such
      thing in ordinary code.
  - pattern: '#define'
    kind: line_start
    weight: 6
    note: >
      The C preprocessor substitutes text before the compiler ever runs. C++
      inherits it and modern C++ avoids it in favor of `constexpr`. No other
      language in this deck has a preprocessor at all.
  - pattern: 'int main\('
    kind: regex
    weight: 5
    note: >
      C and C++ both start at `int main`. Java and C# spell it
      `public static void main`, Go uses `func main()`, Rust uses `fn main()`.
  - pattern: 'typedef'
    kind: token
    weight: 5
    note: >
      C names a type with `typedef struct { ... } thing_t;`. C++ can do this and
      normally writes `using`, Rust writes `type`, Go writes `type Thing struct`.

rules_out:
  - pattern: '::'
    kind: operator
    because: C++, Rust, or PHP. C has no scope resolution operator anywhere in the language.
  - pattern: 'class'
    because: C++, Java, C#, Python, or TypeScript. C has `struct` and nothing else.
  - pattern: 'template'
    because: C++
  - pattern: 'namespace'
    because: C++ or C#
  - pattern: 'std::'
    kind: sigil
    because: C++
  - pattern: 'func'
    because: Go or Swift
  - pattern: 'fn'
    because: Rust or Zig
  - pattern: '@import'
    kind: sigil
    because: Zig

project_fingerprint:
  manifests:
    - file: Makefile
      decisive: false
      note: >
        The classic C build file, but Makefiles also build C++, Go, and assembly.
        Read the `CC` and `CFLAGS` lines inside: `CC = gcc` with `.c` sources is C,
        `CXX` with `.cpp` sources is C++.
    - file: CMakeLists.txt
      decisive: false
      note: >
        CMake builds both C and C++ and cannot be trusted on its own. The
        `project(name C)` line or a `LANGUAGES C` argument settles it, and so does
        looking at whether the sources end in `.c` or `.cpp`.
    - file: configure.ac
      decisive: false
      note: >
        Autotools, alongside `Makefile.am` and a generated `configure` script.
        Almost always a C project, usually one older than CMake.
    - file: compile_commands.json
      decisive: false
      note: >
        Generated, not written. Lists the exact compiler command for every source
        file, which is the fastest way to see whether the project is C or C++.
  build_dirs: [build/, obj/, .libs/, cmake-build-debug/]
  entry_points: [main.c, src/main.c]

shape:
  blocks: braces
  statement_end: semicolon
  comment_line: '//'
  comment_block: '/* */'
  string_quotes: >
    Double quotes make a string. Single quotes make exactly one character, which C
    treats as a small number. Python and JavaScript let you use either quote for a
    string, so this catches people coming from those languages constantly.
  naming: snake_case for functions and variables, ALL_CAPS for macros and constants, a _t suffix on typedef names
  import_keyword: '#include'

tooling:
  package_manager: none in common use. Dependencies are copied into the repo or installed system-wide.
  registry: none
  runtime: none, it compiles to a standalone .exe
  install_command: none. You install a whole toolchain instead, usually Visual Studio Build Tools on Windows.
  run_command: cl main.c then main.exe, or gcc main.c -o main.exe then .\main.exe
  test_command: none built in. Projects pick Unity, Check, or CMake's ctest.

confusable_with:
  - language: cpp
    settle_it: >
      Any `::` in the file means C++. C has no scope resolution operator at all, so
      one is enough. Backup check: C++ standard headers drop the extension
      (`#include <vector>`) and C's always keep it (`#include <stdio.h>`).
    tiebreak: { pattern: '::', kind: operator, favors: cpp }
  - language: go
    settle_it: >
      Both are plain brace languages. C ends every statement with a semicolon and
      opens with `#include`. Go has no semicolons at line ends, no `#include`, and
      declares with `:=` and `func`.
    tiebreak: { pattern: '#include', kind: line_start, favors: c }
  - language: java
    settle_it: >
      Java's entry point is `public static void main(String[] args)` sitting inside
      a `class`. C's is a bare `int main(void)` with no class in the file. Java also
      says `import`, never `#include`, and has no pointers.
    tiebreak: { pattern: 'public static void', favors: java }

errors_look_like:
  sample: |
    main.c:12:5: warning: implicit declaration of function 'strlen' [-Wimplicit-function-declaration]
       12 |     strlen(name);
          |     ^~~~~~
    /usr/bin/ld: main.o: in function `main':
    main.c:(.text+0x1f): undefined reference to `helper'
    collect2: error: ld returned 1 exit status
  recognize_by: >
    A `file.c:line:column:` prefix, a bracketed compiler flag like
    `[-Wimplicit-function-declaration]`, and the phrase `undefined reference to` from
    the linker. Rust prints an error code like `error[E0382]`, Go prints
    `./main.go:9:2:` with no flag in brackets. At runtime C fails with
    `Segmentation fault` on Linux or a silent exit with code `0xC0000005` on Windows,
    and no stack trace either way.
  patterns:
    - '\.c:\d+:\d+: (error|warning):'
    - 'undefined reference to'
    - 'implicit declaration of function'
    - '[Ss]egmentation fault'

meet_it_when: >
  A dependency fails to build on Windows and the wall of output turns out to be a C
  compiler complaining. You clone a small tool from GitHub to build yourself. You
  read the source of something you use every day, because a surprising amount of it
  is C. An agent picks it when you ask for something tiny with no runtime attached.

what_agents_get_wrong: >
  Agents write C that compiles clean, runs clean in the demo, and corrupts memory on
  the first input nobody thought about. The compiler will not warn you, because C's
  contract is that you already checked. Four function names are the reliable smell in
  a diff: `strcpy`, `strcat`, `sprintf`, and `gets`. None of them take a size, so
  none of them can stop at the end of your buffer. The sized versions are `strncpy`
  and `snprintf`, and the size has to be passed explicitly. Second thing to check:
  every `malloc` has a matching `free` on every path out of the function, including
  the error paths the agent added last. Third: thrown-away return values. `malloc`
  can hand back a null pointer, `fopen` can hand back nothing, and `scanf` reports
  how many fields it actually read. Agents assign those results and never test them.
  Any line where a function's answer is discarded is a question worth asking.

version_landscape: >
  C barely moves. Most projects still target the 1999 standard, and the 2011 and 2017
  revisions changed little you would notice. An answer from 2005 usually still
  compiles today. What rots is the Windows half: nearly every C answer online assumes
  Linux and `gcc`, and getting the same code to build with Microsoft's compiler is a
  separate problem the answer will not mention.

see_also:
  - cpp
  - rust
  - go
  - c2-compiled-vs-interpreted
  - j1-how-to-recognize-a-language
  - f1-how-to-read-an-error-message

keywords: [pointer, malloc, free, segfault, header file, gcc, clang, msvc, undefined behavior, buffer overflow]
---

A small compiled language from 1972 that still sits underneath almost everything else
on your machine.

C has about thirty keywords and no safety net. Growable lists, strings that know their
own length, memory that cleans itself up: each of those is something you write yourself
or borrow from a library.

## The shape

Blocks use curly braces. Every statement ends with a semicolon. Indentation means
nothing to the compiler, unlike Python where it means everything.

Files come in pairs. A `.c` file holds the code. A `.h` file, called a header, lists
what that code offers so other files can call it. `#include` pastes a header in
literally before compiling, as a text-level copy and paste, which is why headers carry
guards against being pasted twice.

```c
#include <stdio.h>      /* angle brackets: a system header */
#include "mylib.h"      /* quotes: a header inside this project */

int add(int a, int b) {
    return a + b;       /* the return is mandatory, C works out nothing */
}
```

Types come before the name: `int count`, never `count: int`. That ordering separates C
from Rust, Go, TypeScript, and Kotlin at a glance, since all four put the type after
the name and most put a colon in front of it.

Comments are `/* */` everywhere and `//` since 1999. The program starts at
`int main(void)`. That is the entry point.

## Six lines of it

```c
#include <stdio.h>
#include <stdlib.h>

int main(void) {
    char *name = malloc(16);
    snprintf(name, 16, "nyx");
    printf("hello %s\n", name);
    free(name);
    return 0;
}
```

Two lines there have no equivalent in Python or JavaScript. `malloc` asks the operating
system for sixteen bytes and hands back the address of them. `free` gives them back. If
you forget the `free`, the program holds that memory until it exits, which is a leak. If
you `free` the same address twice, the program may crash later somewhere unrelated to
either line.

## What it is for

Operating system kernels, device drivers, embedded chips, and the insides of things you
use daily. Windows, Linux, SQLite, curl, and the reference Python interpreter are
largely C. When a Python package fails to install on Windows and prints pages of
compiler output, a C compiler is what failed.

C compiles to a standalone `.exe` with no runtime to install on the target machine. It
is also the common tongue between languages: Rust, Go, Python, and C# can all call C
libraries, so C headers turn up in projects that contain no C files of their own.

## Reading its errors

The compiler tells you where the syntax broke and almost nothing about whether the
program is correct. A C program that reads past the end of an array compiles without a
word, then behaves strangely, or works fine for a year and fails on a Tuesday. This is
the single most important fact about the language.

Two failures look different from each other and mean different things.
`main.c:12:5: error:` came from the compiler and means the code is not valid C.
`undefined reference to 'helper'` came from the linker, which runs afterward, and means
the code is valid but the thing it calls was never supplied. The second one usually
points at a missing library rather than a typo.
