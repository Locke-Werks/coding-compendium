---
id: cpp
title: C++
type: language
verified: 2026-08-02
volatility: low

name: C++
aka: [cplusplus, cpp, c-plus-plus, cxx, cc]
family: compiled
likelihood: possible
extensions: ['.cpp', '.hpp', '.cc', '.cxx', '.h', '.hh']

# C++ is a superset of C, so almost every C tell is also true here. The tells
# that earn their weight are the ones C does not have at all, which is why `::`
# and `std::` sit at the top and `#include` sits near the bottom.
tells:
  - pattern: 'std::'
    kind: sigil
    weight: 10
    note: >
      `std::` in front of anything is the C++ standard library. C has no `std` and
      no `::` at all, Rust writes `std::` too but pairs it with `fn` and `let`
      rather than `#include`.
  - pattern: '::'
    kind: operator
    weight: 7
    note: >
      The scope resolution operator, separating a namespace or class from what
      lives inside it. Rust uses it as well. C, Go, Java, Python, and JavaScript
      never do, so a single one rules C out on its own.
  - pattern: 'class'
    kind: token
    weight: 5
    note: >
      C++ has classes with `public:` and `private:` labels on their own lines. C
      has only `struct` and no access labels. Java and C# also use `class` but
      write the access word on every member instead.
  - pattern: 'template\s*<'
    kind: regex
    weight: 9
    note: >
      `template<typename T>` above a function or class is C++ generics. Rust and
      Java write the angle brackets after the name instead, and C has no generics
      at all.
  - pattern: 'std::cout <<'
    kind: regex
    weight: 9
    note: >
      Output by shifting into a stream. C prints with `printf`, Rust with
      `println!`, Go with `fmt.Println`. The double angle bracket used for printing
      belongs to C++ alone.
  - pattern: '#include <\w+>'
    kind: regex
    weight: 7
    note: >
      A standard header with no `.h` on the end, as in `<vector>` or `<iostream>`,
      is C++. C's standard headers always carry the extension: `<stdio.h>`.
  - pattern: 'nullptr'
    kind: token
    weight: 8
    note: >
      C++11 replaced C's `NULL` with `nullptr`. Seeing `nullptr` dates the file to
      2011 or later and rules out C, which never gained the keyword.
  - pattern: 'namespace'
    kind: token
    weight: 6
    note: >
      `namespace foo { ... }` groups names to stop collisions. C# uses the same
      word, but pairs it with `using System;` rather than `#include`. C has no
      namespaces.

rules_out:
  - pattern: 'func'
    because: Go or Swift
  - pattern: 'fn'
    because: Rust or Zig
  - pattern: 'def'
    because: Python or Ruby
  - pattern: 'public static void'
    because: Java or C#
  - pattern: 'println!'
    kind: regex
    because: Rust
  - pattern: '@import'
    kind: sigil
    because: Zig
  - pattern: 'console\.log'
    kind: regex
    because: JavaScript or TypeScript

project_fingerprint:
  manifests:
    - file: CMakeLists.txt
      decisive: false
      note: >
        The most common C++ build file by a wide margin, but CMake builds C too.
        The deciding line is `project(name CXX)` or `set(CMAKE_CXX_STANDARD 20)`.
        A CMake project with only `.c` sources is C.
    - file: '*.vcxproj'
      decisive: true
      note: >
        A Visual Studio C++ project file. Visual Studio uses `.csproj` for C# and
        `.vbproj` for Visual Basic, so the `vcx` spelling settles it. Comes with a
        `.sln` solution file beside it.
    - file: conanfile.txt
      decisive: true
      note: >
        Conan is a C++ package manager. Nothing else uses this file, so it settles
        the question by itself.
    - file: vcpkg.json
      decisive: true
      note: >
        Microsoft's C++ package manager, the one you meet most often on Windows.
        Lists dependencies by name and is C++ only.
    - file: Makefile
      decisive: false
      note: >
        Read the `CXX` and `CXXFLAGS` lines inside. `CXX = g++` with `.cpp` sources
        is C++, `CC = gcc` with `.c` sources is C.
  lockfiles: [conan.lock, vcpkg-configuration.json]
  build_dirs: [build/, x64/, Debug/, Release/, cmake-build-debug/, out/]
  entry_points: [main.cpp, src/main.cpp]

shape:
  blocks: braces
  statement_end: semicolon
  comment_line: '//'
  comment_block: '/* */'
  string_quotes: >
    Double quotes make a string. Single quotes make one character. A `R"(raw
    string)"` form exists for text with backslashes in it, which no other language
    in this deck spells that way.
  naming: snake_case in the standard library, CamelCase or camelCase in application code depending on the project, ALL_CAPS for macros
  import_keyword: '#include'

tooling:
  package_manager: vcpkg or Conan, and plenty of projects use neither
  registry: vcpkg registry, ConanCenter
  runtime: none, it compiles to a standalone .exe, though it may need the Visual C++ redistributable
  install_command: vcpkg install <package-name>
  run_command: cmake --build build, then run the .exe the build produced
  test_command: ctest --test-dir build

confusable_with:
  - language: c
    settle_it: >
      Any `::` in the file means C++. C has no scope resolution operator at all, so
      one is enough. Backup check: C++ standard headers drop the extension
      (`#include <vector>`) and C's always keep it (`#include <stdio.h>`). No
      `class`, `namespace`, or `template` anywhere plus `malloc` and `printf` means
      you are looking at C.
    tiebreak: { pattern: '::', kind: operator, favors: cpp }
  - language: rust
    settle_it: >
      Both use `::` and angle brackets. C++ has `#include <...>` at the top and
      `std::cout <<` for output. Rust has `use ...;` and `println!`. The exclamation
      mark on a call has no C++ equivalent.
    tiebreak: { pattern: '#include', kind: line_start, favors: cpp }
  - language: java
    settle_it: >
      Both have `class` and `new`. C++ puts `public:` on a line by itself as a label
      and starts at a bare `int main()`. Java writes `public` on every single member
      and starts at `public static void main(String[] args)` inside a class. Java
      also has no `#include` and no pointers.
    tiebreak: { pattern: 'public static void', favors: java }

errors_look_like:
  sample: |
    main.cpp:7:17: error: no matching function for call to 'std::vector<int>::push_back(const char [4])'
        7 |     numbers.push_back("nyx");
          |     ~~~~~~~~~~~~~~~~~^~~~~~~
    /usr/include/c++/13/bits/stl_vector.h:1287:7: note: candidate: 'void std::vector<_Tp, _Alloc>::push_back(const value_type&)'
  recognize_by: >
    Errors that are hundreds of lines long, full of `std::` and angle brackets, with
    `note: candidate:` lines listing every function the compiler considered. The
    phrase `no matching function for call to` is the most common opening. At runtime
    an unhandled exception prints
    `terminate called after throwing an instance of 'std::runtime_error'`. Rust
    prints a numbered `error[E0382]` instead, and C's errors are short.
  patterns:
    - '\.(cpp|cc|cxx|hpp):\d+:\d+: error:'
    - 'no matching function for call to'
    - 'terminate called after throwing an instance of'
    - 'std::__cxx11'

meet_it_when: >
  A Python package with fast internals fails to install on Windows and the output is
  from a C++ compiler asking for build tools. You clone a game engine or a desktop
  application to build. You read the source of a program you use, since most large
  desktop software with a window is C++ underneath.

what_agents_get_wrong: >
  Agents produce C++ from three different decades inside one file, and all of it
  compiles. Four things to check in a diff. First, a bare `new` with no matching
  `delete`, which leaks, or a `delete` that only runs on one branch, which is worse
  than leaking. Modern C++ hands that job to `std::make_unique` and
  `std::shared_ptr` so the cleanup happens on its own. Second, anything that returns
  a reference, a `std::string_view`, or a `std::span` pointing at a local variable.
  That compiles without a warning and prints garbage at random intervals, which is
  the hardest kind of bug in this deck to reproduce. Third, `v[i]` does no bounds
  checking and `v.at(i)` does. When an agent writes an index by hand, ask where the
  check is. Fourth, it will write C++20 code into a project set to C++14, then fix
  the build by raising the project's standard rather than by changing the code. That
  is a real decision and it should be yours.

version_landscape: >
  This one matters more than for any other language on the card. C++11 split the
  language in half. Code written before it uses raw `new`, raw pointers, and manual
  loops; code written after uses `auto`, smart pointers, and range-based `for`. An
  answer from 2009 will work and will look nothing like the rest of a modern
  codebase. Check which standard the project targets before trusting anything: the
  `CMAKE_CXX_STANDARD` line in `CMakeLists.txt`, or the `/std:c++20` flag in a
  Visual Studio project.

see_also:
  - c
  - rust
  - java
  - c2-compiled-vs-interpreted
  - j1-how-to-recognize-a-language
  - f1-how-to-read-an-error-message

keywords: [stl, template, smart pointer, unique_ptr, header, cmake, vcpkg, msvc, raii, segfault]
---

C with classes added in 1985, then forty more years of features, none of which were
ever taken back out.

C++ is enormous. Nobody uses all of it, and two C++ projects can read like two
different languages depending on which decade their authors learned it in.

## The shape

Blocks use curly braces. Every statement ends with a semicolon. Types come before the
name, exactly as in C: `int count`, never `count: int`.

The tell that separates C++ from C in one glance is `::`, the scope resolution
operator. It sits between a namespace and whatever lives inside it, so `std::vector`
means "the `vector` that belongs to the standard library". C has no `::` anywhere in
the language, which makes a single one decisive.

```cpp
#include <iostream>     // no .h on the end: a C++ standard header
#include <vector>
#include <string>

int main() {
    std::vector<std::string> names{"nyx"};
    names.push_back("locke");
    std::cout << names.size() << "\n";
    return 0;
}
```

`<<` pushes a value into a stream and prints it. It is the same symbol C uses for
shifting bits left, borrowed for output, and that line is the most recognizable single
line in the language.

Comments are `//` and `/* */`. Code goes in `.cpp` files and headers go in `.hpp` or
`.h`, the same split C uses.

## Six lines of the modern dialect

```cpp
#include <memory>
#include <string>

struct Session {
    std::string user;
    explicit Session(std::string u) : user(std::move(u)) {}
};

auto s = std::make_unique<Session>("nyx");
```

`auto` tells the compiler to work out the type from the right-hand side.
`std::make_unique` allocates the object and promises to destroy it when `s` goes out
of scope, so there is no `delete` to forget. Both arrived in C++11, and both are what
people mean when they say "modern C++".

## What it is for

Game engines, browsers, desktop applications, trading systems, and anything wanting
C's speed with a way to organize a large program. Chrome, Photoshop, and Unreal Engine
are C++.

You will also meet it sideways. Installing a Python or Node package that contains fast
code often compiles C++ on your machine first, and on Windows that step fails until
Microsoft's build tools are installed.

## Reading its errors

C++ errors are famously long. One wrong type inside a template can produce two hundred
lines naming types you never wrote.

Read the first error only. Inside it, find the line that names a file you wrote and
ignore the rest, which is the compiler showing its work. Fixing the first error
frequently deletes the other hundred and ninety-nine.

Two failures are worth telling apart. `error:` with a file and line came from the
compiler and means the code is not valid C++. `undefined reference to` came from the
linker, which runs afterward, and means a library was never handed to the build.
