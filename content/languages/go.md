---
id: go
title: Go language
type: language
verified: 2026-08-02
volatility: low

name: Go
aka: [golang, go-lang, gopher]
family: compiled
likelihood: likely
extensions: ['.go']

# Go's tells are mostly about what is missing. It is a brace language with no
# semicolons, no `::`, no classes, and no exceptions, and each of those absences
# points somewhere specific, which is why the notes below name the language that
# has the thing Go lacks.
tells:
  - pattern: ':='
    kind: operator
    weight: 10
    note: >
      Declare and assign in one step, with the type worked out from the value. Only
      Go uses `:=` this way. Rust writes `let`, JavaScript writes `const`, Python
      writes nothing at all.
  - pattern: 'func'
    kind: token
    weight: 9
    note: >
      Go spells it exactly `func`. Rust uses `fn`, Kotlin uses `fun`, Swift also
      uses `func` but pairs it with `import Foundation` rather than `package main`.
  - pattern: 'package \w+'
    kind: regex
    weight: 9
    note: >
      Every Go file opens with `package something` on the first non-comment line.
      Java has a `package` line too, but writes it with a dotted path and a
      semicolon: `package com.example.app;`.
  - pattern: 'err != nil'
    kind: regex
    weight: 10
    note: >
      The most repeated three-word phrase in the language. Go returns errors as
      values and checks them inline. Rust returns a `Result`, Python and Java throw
      exceptions, so none of them ever write this.
  - pattern: 'fmt\.'
    kind: regex
    weight: 8
    note: >
      `fmt.Println` and `fmt.Sprintf` are Go's standard printing. C uses `printf`,
      C++ uses `std::cout`, Rust uses `println!`.
  - pattern: 'defer'
    kind: token
    weight: 6
    note: >
      Schedules a line to run when the function ends. Zig has `defer` too, but runs
      it at the end of the enclosing block instead, and pairs it with `fn` rather
      than `func`. C, Java, and Python have nothing like it.
  - pattern: 'go func\('
    kind: regex
    weight: 9
    note: >
      Starts a goroutine, which is Go's lightweight thread. The word `go` in front
      of a call exists in no other language in this deck.
  - pattern: 'chan '
    kind: regex
    weight: 8
    note: >
      A channel, Go's pipe between goroutines, usually seen with the `<-` arrow.
      Rust spells the same idea `mpsc::channel()` and Python uses `queue.Queue`.

rules_out:
  - pattern: ';\s*$'
    kind: regex
    because: >
      C, C++, Rust, Java, or C#. Go inserts semicolons for you, and its formatter
      strips any you type, so a line ending in one is not Go.
  - pattern: 'let'
    because: Rust, JavaScript, TypeScript, or Swift. Go has no `let` keyword.
  - pattern: '::'
    kind: operator
    because: C++ or Rust. Go has no scope resolution operator.
  - pattern: 'class'
    because: C++, Java, C#, Python, or TypeScript. Go has `struct` and interfaces and no classes.
  - pattern: '#include'
    kind: line_start
    because: C or C++
  - pattern: 'fn'
    because: Rust or Zig
  - pattern: 'try'
    because: >
      Java, C#, Python, JavaScript, or Zig. Go has no `try` and no exceptions for
      ordinary failures.

project_fingerprint:
  manifests:
    - file: go.mod
      decisive: true
      note: >
        A `go.mod` at the root means the project is Go, full stop. First line is
        `module <name>`, second is the Go version. This is the single most reliable
        identifier on this card.
    - file: go.work
      decisive: true
      note: >
        A workspace file tying several Go modules together. Less common than
        `go.mod` and equally decisive.
    - file: Makefile
      decisive: false
      note: >
        Plenty of Go projects wrap `go build` in a Makefile out of habit. It tells
        you nothing on its own. Look for `go.mod` beside it.
  lockfiles: [go.sum]
  build_dirs: [bin/, dist/]
  entry_points: [main.go, cmd/<name>/main.go]

shape:
  blocks: braces
  statement_end: newline
  comment_line: '//'
  comment_block: '/* */'
  string_quotes: >
    Double quotes for a normal string, backticks for a raw string that keeps its
    newlines and backslashes, single quotes for exactly one character. The backtick
    string is a good secondary tell, since Rust and C do not have one.
  naming: camelCase for private names, CapitalizedCase for anything other packages can use, which is a language rule rather than a convention
  import_keyword: import

tooling:
  package_manager: the go command itself, no separate tool
  registry: pkg.go.dev, which indexes modules straight from their git repositories
  runtime: none, it compiles to a standalone .exe
  install_command: go get <module-path>
  run_command: go run .
  test_command: go test ./...

confusable_with:
  - language: rust
    settle_it: >
      Go declares with `:=` and functions with `func`. Rust uses `let` and `fn`, and
      uses `::`, which Go never does. A brace language with no semicolons at the ends
      of lines is Go, because Rust ends its statements with them.
    tiebreak: { pattern: ':=', kind: operator, favors: go }
  - language: c
    settle_it: >
      Both are plain brace languages. C ends every statement with a semicolon and
      opens with `#include`. Go has no semicolons at line ends, no `#include`, and
      declares with `:=` and `func`.
    tiebreak: { pattern: '#include', kind: line_start, favors: c }
  - language: zig
    settle_it: >
      Both have `defer`, which is the thing that causes the mix-up. Go writes `func`
      and `:=` and imports with `import "fmt"`. Zig writes `fn` and gets everything
      through `@import`, and any `@` in front of a name is Zig.
    tiebreak: { pattern: '@import', kind: sigil, favors: zig }

errors_look_like:
  sample: |
    ./main.go:9:2: declared and not used: total

    panic: runtime error: index out of range [3] with length 3

    goroutine 1 [running]:
    main.main()
            C:/Users/you/hello/main.go:14 +0x1d
    exit status 2
  recognize_by: >
    One word does it: `goroutine`. Nothing else in this deck prints it. A crash
    starts with `panic:` and then lists goroutines with their state in square
    brackets. Rust also panics but writes `thread 'main' panicked at`, and Zig writes
    `thread 12345 panic:` with a number. On the compile side, `declared and not used`
    is a Go error rather than a warning, which no other language in this deck treats
    as fatal.
  patterns:
    - '^goroutine \d+ \[[a-z ]+\]:'
    - 'panic: runtime error:'
    - 'declared and not used'
    - '\.go:\d+:\d+: '

meet_it_when: >
  You download a command-line tool that arrives as one `.exe` with nothing to install
  beside it, which is usually Go. An agent picks it for a web service or a small
  server. You clone something like the `gh` tool or Terraform to build it yourself,
  or you read the source of the infrastructure software everyone runs.

what_agents_get_wrong: >
  Go's failure mode is quiet, and the diff is where you catch it. First: an agent
  writes `value, _ := doThing()` and the underscore throws the error away, so the
  program carries on with an empty value and breaks somewhere unrelated. Search a Go
  diff for an underscore on the left of `:=` or `=` and ask about every single one.
  Second: `defer file.Close()` on a file that was written to. Closing can fail, the
  deferred form discards that failure, and the last bytes quietly never reach disk.
  Third: goroutines with no way to stop them. A `go func()` with no context, no
  channel to signal it, and no `sync.WaitGroup` runs until the process exits, and one
  started inside a loop is a leak that grows. Fourth: agents trained on older code add
  a `v := v` line inside range loops to copy the loop variable. Go 1.22 made that
  unnecessary. It is harmless on its own, and it tells you the agent is working from
  old material, so read the rest of that file harder.

version_landscape: >
  Go promises not to break your code and has kept the promise. A program from 2013
  still compiles. Two changes date an answer. Modules arrived in Go 1.11 and made
  every older answer about `GOPATH` and putting your code in one specific folder
  wrong. Generics arrived in 1.18. If an answer tells you where on disk your project
  has to live, it is from before 2018 and you can skip it.

see_also:
  - rust
  - c
  - zig
  - g2-package-managers
  - c3-what-running-means
  - f2-stack-traces

keywords: [golang, goroutine, channel, go.mod, gofmt, nil, panic, defer, interface]
---

A compiled language from Google, designed in 2009 to be boring on purpose.

Go has twenty-five keywords and usually one way to do a thing. The language was built
so that a new hire could read a stranger's code on their first day, and that goal shows
in every corner of it.

## The shape

Blocks use curly braces. Statements do not end with semicolons, which makes Go the
easiest brace language to spot: C, C++, Rust, Java, and C# all finish their lines with
`;` and Go does not.

The opening brace has to sit on the same line as the thing it opens. Move it to the
next line and the program stops compiling, because Go inserts an invisible semicolon at
the end of the previous line for you.

```go
package main

import "fmt"

func main() {
    name := "ada"       // := declares and assigns in one step
    var count int = 3   // the long form, for when you want to state the type
    fmt.Println(name, count)
}
```

`:=` is the single most Go thing on the page. Rust writes `let`, JavaScript writes
`const`, Python writes nothing at all.

Types come after the name: `var count int`. C and Java put the type first. Rust and
TypeScript also put it after, with a colon in between, and Go leaves the colon out.

Comments are `//` and `/* */`. Every file opens with `package something`. The program
starts at `func main()` inside `package main`.

## The error habit

Go has no exceptions for ordinary failures. A function that can fail returns two
things, the answer and an error, and you check the error every time.

```go
data, err := os.ReadFile("config.json")
if err != nil {
    return err
}
```

Those three lines appear hundreds of times in a real Go program. Python and JavaScript
would throw instead. Rust returns a `Result` and refuses to compile until you deal with
it. Go asks politely and lets you ignore it, which is exactly where the bugs live.

## What it is for

Command-line tools, web servers, and infrastructure. Docker, Kubernetes, Terraform, and
the `gh` command you use to talk to GitHub are all Go.

Go compiles to one self-contained `.exe` with no runtime to install on the target
machine, which is why so many tools you download as a single file turn out to be Go.
Building for a different operating system takes one environment variable, which is the
other reason.

## Reading its errors

Two kinds, and they look nothing alike. A compile error reads
`./main.go:9:2: declared and not used: total`. Go refuses to build a program that has
an unused variable or an unused import, which catches a whole class of leftover code.

A crash at runtime prints `panic:` and then the word `goroutine`. Nothing else in this
deck prints goroutine, so that word on its own identifies the language.
