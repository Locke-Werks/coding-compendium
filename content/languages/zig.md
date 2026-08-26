---
id: zig
title: Zig
type: language
verified: 2026-08-02
volatility: quarterly

name: Zig
aka: [ziglang, zig-lang]
family: compiled
likelihood: unlikely
extensions: ['.zig', '.zon']

# Zig shares `fn`, `const`, and `pub` with Rust and `defer` with Go, so the tells
# that carry weight are the ones neither of those has: the `@` builtins, the `!`
# in a return type, and the `.{}` literal.
tells:
  - pattern: '@import\('
    kind: regex
    weight: 10
    note: >
      `const std = @import("std");` is the first line of nearly every Zig file. C
      says `#include`, Rust says `use`, Go says `import`. Nobody else puts an at
      sign in front of it.
  - pattern: '@\w+\('
    kind: regex
    weight: 9
    note: >
      Anything starting with `@` is a builtin handled by the compiler itself:
      `@intCast`, `@sizeOf`, `@panic`. Python puts `@` on decorators above a
      function and Rust writes `#[...]` for the same job, so position alone tells
      them apart.
  - pattern: '!void'
    kind: regex
    weight: 10
    note: >
      A return type of `!void` means "nothing, or an error". The bang in front of a
      type is Zig's error union and exists in no other language here. Rust writes
      `Result<(), Error>` for the same idea.
  - pattern: 'pub fn'
    kind: regex
    weight: 6
    note: >
      Zig and Rust both write `pub fn`. Zig follows it with `@` builtins and
      `const`/`var`, Rust follows it with `let`, `impl`, and `#[derive]`.
  - pattern: '\.\{'
    kind: regex
    weight: 8
    note: >
      A dot before a brace, as in `.{ name, count }`, is an anonymous struct
      literal, used constantly for print arguments. Rust would write a tuple `(a,
      b)` and C would write a plain `{ }`.
  - pattern: 'try '
    kind: regex
    weight: 7
    note: >
      In Zig `try` is a prefix on one call: `try file.write(data)`. Java, C#, and
      Python write `try {` as a block opener, which reads completely differently.
  - pattern: 'comptime'
    kind: token
    weight: 10
    note: >
      Marks code that runs during compilation. C++ spells the nearest equivalent
      `constexpr` and Rust spells it `const fn`, so the word `comptime` belongs to
      Zig alone.
  - pattern: 'allocator'
    kind: token
    weight: 5
    note: >
      Zig passes an allocator into any function that needs memory, so the word turns
      up in signatures everywhere. C hides it inside `malloc`, and Go, Java, and
      Python never mention memory at all.

rules_out:
  - pattern: '#include'
    kind: line_start
    because: C or C++. Zig has no preprocessor and no include directive.
  - pattern: 'let'
    because: Rust, JavaScript, TypeScript, or Swift. Zig declares with `const` and `var` only.
  - pattern: 'func'
    because: Go or Swift
  - pattern: 'impl'
    because: Rust
  - pattern: 'println!'
    kind: regex
    because: Rust
  - pattern: 'class'
    because: C++, Java, C#, Python, or TypeScript. Zig has structs and no classes.

project_fingerprint:
  manifests:
    - file: build.zig
      decisive: true
      note: >
        The build script, written in Zig itself rather than in a config format. If
        this file is at the root, the project is Zig. Nothing else uses the name.
    - file: build.zig.zon
      decisive: true
      note: >
        The dependency list, in Zig Object Notation, which is Zig's own take on
        JSON. Equally decisive and usually sitting beside `build.zig`.
  lockfiles: []
  build_dirs: [zig-out/, zig-cache/, .zig-cache/]
  entry_points: [src/main.zig, build.zig]

shape:
  blocks: braces
  statement_end: semicolon
  comment_line: '//'
  comment_block: >
    none. Zig has no block comment at all, which is unusual enough to be a tell by
    itself: C, C++, Rust, Go, and Java all have one.
  string_quotes: >
    Double quotes for strings, single quotes for one character, and a `\\` prefix on
    each line for a multi-line string. That backslash form belongs to Zig.
  naming: snake_case for functions and variables, CamelCase for types, SCREAMING_SNAKE for constants
  import_keyword: '@import'

tooling:
  package_manager: the zig command itself, reading build.zig.zon
  registry: none central. Dependencies are named by URL and content hash.
  runtime: none, it compiles to a standalone .exe
  install_command: zig fetch --save <url>
  run_command: zig build run
  test_command: zig build test

confusable_with:
  - language: rust
    settle_it: >
      Both write `fn` and `pub fn`, which is the whole problem. Look for `@`: any
      `@import` or `@intCast` is Zig. Rust marks the same territory with `#[derive]`
      attributes and macros ending in `!`, and declares variables with `let`, which
      Zig does not have.
    tiebreak: { pattern: '@import', kind: sigil, favors: zig }
  - language: c
    settle_it: >
      Zig was built to replace C and reads like a tidied version of it. C opens with
      `#include` and has a preprocessor. Zig opens with `const std = @import("std")`
      and has no preprocessor at all, no `#define`, and no header files.
    tiebreak: { pattern: '#include', kind: line_start, favors: c }
  - language: go
    settle_it: >
      Both have `defer`, which is what causes the mix-up. Go writes `func` and `:=`
      and imports with `import "fmt"`. Zig writes `fn` and gets everything through
      `@import`, and any `@` in front of a name is Zig.
    tiebreak: { pattern: '@import', kind: sigil, favors: zig }

errors_look_like:
  sample: |
    src/main.zig:5:9: error: expected type 'u8', found 'i32'
        count = value;
                ^~~~~
    src/main.zig:3:16: note: u8 declared here

    thread 21484 panic: integer overflow
    src/main.zig:7:20: 0x2033a5 in main (demo.exe)
  recognize_by: >
    Compile errors carry a `.zig:` path with a line and column, then `note:` lines
    pointing at the declarations involved. A crash prints `thread 21484 panic:` with
    a bare number. Rust writes `thread 'main' panicked at` with the name in quotes,
    and Go writes `panic:` followed by `goroutine`. All three say panic, and the word
    directly after it tells you which language you are holding.
  patterns:
    - '\.zig:\d+:\d+: error:'
    - 'thread \d+ panic:'
    - 'error: expected type'
    - 'zig build-exe'

meet_it_when: >
  Rarely, and usually sideways. A project uses `zig cc` as its C compiler because it
  cross-compiles more easily than the alternatives, a small tool you want to build is
  written in it, or someone recommends it to you as the modern replacement for C.

what_agents_get_wrong: >
  The biggest thing agents get wrong about Zig is the version. Zig is before 1.0 and
  breaks on purpose between releases, and both the standard library and the build
  system moved in every one of 0.11, 0.12, 0.13, and 0.14. An agent will hand you a
  `build.zig` that was correct a year ago and does not compile now, and the error will
  point at a function that no longer exists rather than at anything you asked for. Run
  `zig version` first and tell the agent the number before it writes a line. Second
  thing to check in a diff: every allocation takes an allocator explicitly, and every
  one needs a matching `defer allocator.free(...)` or `defer thing.deinit()` beside
  it. Agents write the allocation and skip the defer. Third: `try` hands an error up
  to the caller, and `catch unreachable` tells the compiler the error can never
  happen. Agents write `catch unreachable` to get past a compile error they did not
  want to think about. It is a promise about the future, it is usually wrong, and it
  turns a handled error into a crash.

version_landscape: >
  The opposite of Rust. Zig is before 1.0 and breaks between releases deliberately, so
  an answer from six months ago may not compile at all. Every answer, snippet, and
  tutorial needs the version it was written for, and the first thing to run is
  `zig version`. This is the one language on the card where a search result's date
  matters more than its content.

see_also:
  - c
  - rust
  - go
  - c2-compiled-vs-interpreted
  - j1-how-to-recognize-a-language

keywords: [ziglang, comptime, allocator, error union, build.zig, zig cc, defer, unreachable]
---

A young compiled language aiming at the job C does, without the preprocessor and
without the parts of C that fail quietly.

Zig is not finished. It is before version 1.0, it breaks between releases on purpose,
and that fact matters more than any piece of syntax on this card.

## The shape

Blocks use curly braces. Statements end with semicolons. Functions are `fn`, which Zig
shares with Rust and with nothing else in this deck.

The giveaway is `@`. Anything starting with an at sign is a builtin, handled by the
compiler itself: `@import`, `@intCast`, `@sizeOf`. No other language here uses `@` that
way. Python puts `@` on a decorator line above a function, Rust writes `#[...]` for the
same job, and C# puts attributes in square brackets.

```zig
const std = @import("std");

pub fn main() !void {
    const name = "ada";
    var count: u8 = 3;
    count += 1;
    std.debug.print("{s} {d}\n", .{ name, count });
}
```

Three things in there are Zig and only Zig. `@import` where C would say `#include` and
Rust would say `use`. `!void` as a return type, meaning "returns nothing, or an error".
And `.{ ... }`, an anonymous struct literal, used here to pass the print arguments.

Variables are `const` when fixed and `var` when they change. Rust spells that pair
`let` and `let mut`, which is the quickest way to tell the two apart when both files
are full of `fn`.

Comments are `//` only. Zig has no block comment, which is unusual enough to be a tell
by itself.

## Where the memory comes from

Zig hides no allocation. Any function that needs memory takes an allocator as an
argument, so every allocation in the program is visible in a signature somewhere.

```zig
const buf = try allocator.alloc(u8, 64);
defer allocator.free(buf);
```

`try` passes an error up to whoever called you. `defer` runs its line when the current
block ends, which is how the memory finds its way back. Go has `defer` as well and runs
it at the end of the whole function instead of the block, so the two are not
interchangeable.

## What it is for

Small fast programs, embedded work, and replacing C in projects that had run out of
patience with C. Zig also ships a complete C compiler inside itself, so some projects
use `zig cc` as their build tool and contain no Zig code at all.

The `bun` JavaScript runtime is the largest Zig program most people have heard of.

## Reading its errors

A compile error looks like `src/main.zig:5:9: error: expected type 'u8', found 'i32'`,
followed by `note:` lines pointing at the declarations involved. It is closer to Rust's
style than to C's, though without the drawings.

A crash prints `thread 21484 panic:` with a bare number. Rust prints
`thread 'main' panicked at` with the name in quotes, and Go prints `panic:` followed by
`goroutine`. All three use the word panic, and the word directly after it is what tells
you which language just fell over.
