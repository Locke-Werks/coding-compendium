---
id: haskell
title: Haskell
type: language
verified: 2026-08-02
volatility: low

name: Haskell
aka: [ghc, hs, cabal, stack]
family: compiled
likelihood: unlikely
extensions: ['.hs', '.lhs']

tells:
  - pattern: '^\w+ :: '
    kind: regex
    weight: 9
    note: >
      A name, a double colon, then a type, on a line by itself above the function.
      In Haskell `::` means "has type". C++ and Rust use `::` as a namespace
      separator inside an expression, never alone on its own line.
  - pattern: 'IO \(\)'
    kind: regex
    weight: 10
    note: >
      `IO ()` as a return type marks the one part of the program allowed to touch
      the outside world. No other language in this deck writes a type this way.
  - pattern: 'module \w+ where'
    kind: regex
    weight: 10
    note: >
      A Haskell file can open with `module Name where`. Go writes `package name`,
      Java writes `package com.example;`, Python writes nothing.
  - pattern: '<-'
    kind: operator
    weight: 7
    note: >
      A left arrow binds a result inside a `do` block. Go uses `<-` for channels,
      and nothing else in this deck uses it at all.
  - pattern: '\bwhere\b'
    kind: regex
    weight: 5
    note: >
      Haskell hangs local helpers off a `where` clause underneath the function.
      Rust uses `where` for type bounds after a signature; C, Go, and Python have no
      `where` at all.

rules_out:
  - pattern: 'def'
    because: Python or Ruby
  - pattern: 'func'
    because: Go or Swift
  - pattern: '#include'
    kind: line_start
    because: C or C++
  - pattern: 'println!'
    kind: regex
    because: Rust

project_fingerprint:
  manifests:
    - file: '*.cabal'
      decisive: true
      note: >
        A `.cabal` file at the root is a Haskell project. Nothing else uses the
        format or the name.
    - file: stack.yaml
      decisive: true
      note: >
        Stack is the other Haskell build tool. Only Haskell projects carry this file.
    - file: package.yaml
      decisive: false
      note: >
        Used by `hpack` to generate the `.cabal` file. Check for a `.cabal` or a
        `stack.yaml` beside it before trusting it.
  lockfiles: [cabal.project.freeze, stack.yaml.lock]
  build_dirs: [dist-newstyle/, .stack-work/]
  entry_points: [app/Main.hs, src/Main.hs]

shape:
  blocks: indentation
  statement_end: newline
  comment_line: '--'
  comment_block: '{- -}'
  string_quotes: Double quotes for strings, single quotes for one character, same split as C and Rust.
  naming: camelCase for functions and values, CapitalizedCase for types and constructors
  import_keyword: import

tooling:
  package_manager: Cabal or Stack
  registry: Hackage
  runtime: none, it compiles to a standalone .exe
  install_command: cabal install <package-name>
  run_command: cabal run
  test_command: cabal test

confusable_with:
  - language: rust
    settle_it: >
      Both use `->` and `::`. Rust puts `fn` in front of every function and ends
      statements with semicolons. Haskell has no `fn`, no semicolons, and puts its
      `::` alone on a signature line above the function.
    tiebreak: { pattern: 'fn', favors: rust }
  - language: elixir
    settle_it: >
      Both are functional and both look unfamiliar. Elixir writes `defmodule` and
      `def` and closes blocks with `end`. Haskell writes `module Name where`, has no
      `def`, and closes nothing.
    tiebreak: { pattern: 'defmodule', favors: elixir }

errors_look_like:
  sample: |
    Main.hs:4:11: error: [GHC-83865]
        * Couldn't match expected type 'Int' with actual type '[Char]'
        * In the first argument of 'double', namely 'name'
  recognize_by: >
    Bullet marks indenting each part of the message, a bracketed code like
    `[GHC-83865]`, and the phrase `Couldn't match expected type`. Rust uses
    `error[E0382]` with a different bracket shape and draws diagrams with pipes.
  patterns:
    - 'Couldn.t match expected type'
    - '\.hs:\d+:\d+: error:'
    - '\[GHC-\d+\]'

meet_it_when: >
  You clone a tool that happens to be written in it and need to build the thing. Pandoc
  and ShellCheck are the two you are most likely to hit. You will not write it and an
  agent will not choose it unless you ask for it by name.

what_agents_get_wrong: >
  Agents reach for `head`, `fromJust`, `read`, and `!!`. All four are partial
  functions: they work on the input the agent imagined and crash at runtime on an empty
  list, on a `Nothing`, or on a string that does not parse. The compiler says nothing,
  because Haskell's types do not track emptiness. In a diff, treat any of those four
  names as a question to ask. The safe forms are pattern matching, `listToMaybe`, and
  `readMaybe`.

version_landscape: >
  Stable. The 2010 standard still describes the core language, and most change since
  has arrived as optional extensions switched on per file with a
  `{-# LANGUAGE ... #-}` line at the top. An old answer usually still works.

see_also:
  - rust
  - elixir
  - j1-how-to-recognize-a-language

keywords: [ghc, cabal, stack, monad, hackage, functional, purity, type signature]
---

A purely functional compiled language. A function cannot touch the outside world unless
its type says so, which is why `IO` turns up in signatures everywhere.

The line above each function is its type signature, written with `::`. That double
colon means "has type" here and "namespace separator" in C++ and Rust, and it is the
fastest way to tell the three apart.

```haskell
double :: Int -> Int
double x = x * 2

main :: IO ()
main = putStrLn (show (double 21))
```

No braces, no semicolons, no `return`. Indentation groups the code, as in Python. The
`->` separates argument types, `=` defines the function, and there is no parameter list
in parentheses at all.

Comments are `--` for a line and `{- -}` for a block, which no other language in this
deck spells that way.

You meet Haskell by cloning something rather than by writing it. If a build asks for
`cabal` or `stack`, this is what you are building.
