---
id: swift
title: Swift
type: language
verified: 2026-08-02
volatility: quarterly

name: Swift
aka: [swiftlang, swiftui, apple swift, ios]
family: compiled
likelihood: possible
extensions: ['.swift']

# Kotlin is the nearest neighbor for shape and Rust for punctuation, so every
# note names which of the two the token would be mistaken for.
tells:
  - pattern: 'guard let'
    kind: regex
    weight: 10
    note: >
      `guard let name = name else { return }` unwraps a value or leaves the
      function. No other language in the deck has `guard`. Kotlin gets the same
      job done with `?:` and an early return.
  - pattern: '\\\('
    kind: regex
    weight: 9
    note: >
      A backslash and an open parenthesis inside a string is Swift putting a
      value into text, as in `"hello \(name)"`. Kotlin writes `"hello $name"` and
      C# writes `$"hello {name}"`.
  - pattern: '@(State|Published|ObservedObject|StateObject|MainActor|Environment)'
    kind: regex
    weight: 9
    note: >
      Property wrappers, the at-sign markers that drive Apple's user interface
      framework. Java annotations also start with an at-sign, but Java never
      writes `@State` and always puts its annotations on a line of their own.
  - pattern: 'func'
    kind: token
    weight: 7
    note: >
      Swift and Go both declare with `func`. Kotlin uses `fun`, Rust uses `fn`.
      Swift writes the return type after `->` and annotates parameters with a
      colon, where Go writes the return type bare with no arrow.
  - pattern: 'some View'
    kind: regex
    weight: 8
    note: >
      `var body: some View` is how a screen is declared in Apple's newer user
      interface framework. Kotlin's equivalent is a function marked
      `@Composable`, which reads nothing like this.
  - pattern: 'nil'
    kind: token
    weight: 6
    note: >
      Swift and Objective-C write `nil` for nothing at all. Java, C#, Kotlin, and
      JavaScript write `null`. Go also says `nil`, but Go has no `?` on its types.
  - pattern: 'let [\w]+ ='
    kind: regex
    weight: 5
    note: >
      `let` is a value that will not change and `var` is one that will. Rust and
      JavaScript also write `let`, Kotlin writes `val` for the same idea, and
      Java and C# have nothing like it.
  - pattern: 'if let'
    kind: regex
    weight: 5
    note: >
      Unwraps an optional for the length of a block. Rust writes `if let` too,
      but Rust always pairs it with a pattern such as `Some(x)`, where Swift
      names the variable on its own.

rules_out:
  - pattern: 'fun'
    kind: token
    because: Kotlin
  - pattern: 'fn'
    kind: token
    because: Rust
  - pattern: 'null'
    kind: token
    because: "Java, C#, Kotlin, or JavaScript. Swift says nil"
  - pattern: '@interface'
    kind: regex
    because: Objective-C or a Java annotation type
  - pattern: '#include'
    kind: line_start
    because: C or C++
  - pattern: 'public static void main'
    kind: regex
    because: Java

project_fingerprint:
  manifests:
    - file: Package.swift
      decisive: true
      note: >
        The manifest for Swift Package Manager, and unusual in that it is itself
        a Swift program rather than a data file. Its first line is a comment
        naming the tool version, such as `// swift-tools-version:5.9`. This file
        settles the language on its own.
    - file: '*.xcodeproj'
      decisive: false
      note: >
        An Xcode project, which is a folder pretending to be a file. It means an
        Apple platform without naming the language, because Objective-C projects
        look identical from outside. Open the source folder and check whether the
        files end in `.swift` or `.m`.
    - file: '*.xcworkspace'
      decisive: false
      note: >
        A workspace holding several projects together, usually because a
        dependency manager generated it. Same rule as above, the file extensions
        inside settle it.
  lockfiles: ['Package.resolved']
  build_dirs: ['.build/', 'DerivedData/', 'build/']
  entry_points: ['Sources/', 'main.swift', 'App.swift', 'ContentView.swift']

shape:
  blocks: braces
  statement_end: optional_semicolon
  comment_line: '//'
  comment_block: '/* */'
  string_quotes: >
    Double quotes only, with `\(value)` inside them to insert a value. Triple
    quotes hold text across several lines. There is no single-quote string, which
    separates it from Java and C# at a glance.
  naming: camelCase for functions and variables, PascalCase for types, camelCase for enum cases
  import_keyword: import

tooling:
  package_manager: Swift Package Manager
  registry: 'the Swift Package Index, which points at GitHub rather than hosting anything'
  runtime: 'none, it compiles to a native binary'
  install_command: 'add a dependency to the array in Package.swift'
  run_command: swift run
  test_command: swift test

confusable_with:
  - language: kotlin
    settle_it: >
      Both drop semicolons, both put the type after a colon, both mark optional
      values with a question mark. Swift says `func`, `let`, and `nil`. Kotlin
      says `fun`, `val`, and `null`. Three words, two seconds.
    tiebreak: { pattern: 'fun', kind: token, favors: kotlin }
  - language: objective-c
    settle_it: >
      They share a runtime and often a project. Objective-C puts calls inside
      square brackets as `[view addSubview:button]`, prefixes strings with an
      at-sign, and ends class bodies with `@end`. Swift has none of that and
      calls methods with a dot.
    tiebreak: { pattern: '@end', kind: regex, favors: objective-c }
  - language: rust
    settle_it: >
      Both use `let`, `->`, `enum`, and `match` or `switch` on shapes of data.
      Rust declares with `fn` and separates paths with `::`. Swift declares with
      `func` and never writes `::`.
    tiebreak: { pattern: '::', kind: operator, favors: rust }

errors_look_like:
  sample: |
    Fatal error: Unexpectedly found nil while unwrapping an Optional value
    ContentView.swift:42: Fatal error
    Compile error, separately:
    Sources/App/main.swift:12:9: error: value of optional type 'String?' must be
    unwrapped to a value of type 'String'
  recognize_by: >
    The word `Optional` with a capital O, and `nil` rather than `null`. A crash
    starts with `Fatal error:` and usually names an optional that was empty. A
    compile error is `file.swift:12:9: error:` with both a line and a column, and
    it often ends with a suggested fix. Kotlin's version of the same crash says
    `NullPointerException` and prints a stack of `at` lines below it.
  patterns:
    - 'Fatal error: Unexpectedly found nil'
    - '\w+\.swift:\d+:\d+: error:'
    - 'Optional\(.*\)'
    - 'EXC_BAD_INSTRUCTION'

meet_it_when: >
  Almost always by reading rather than running, because building it needs a Mac
  and you are on Windows. An agent picks it when you ask for an iPhone or Mac
  app, and you meet it in the wild any time you look at how a phone app you like
  was put together.

what_agents_get_wrong: >
  The exclamation mark is the whole story. Swift makes you say what happens when
  a value is missing, and an agent under pressure writes `!` to force it, or
  `try!` to force an operation that can fail, and both turn a compiler question
  into a crash that reads `Fatal error: Unexpectedly found nil`. Scan a diff for
  `!` on the end of a name and ask about every one. The second seam is
  concurrency. Swift 6 turns data races into build errors, so an agent writing
  code that was correct under Swift 5 now fails to compile, and the usual fix it
  offers is to scatter `@MainActor` around until the errors stop, which quietly
  moves your work onto the screen thread. The third is interface drift, because
  Apple deprecates in public: `NavigationView` became `NavigationStack`,
  `.onChange` changed shape, and `@ObservedObject` is wrong where `@StateObject`
  belongs. Code from a 2021 tutorial compiles with warnings and behaves subtly
  differently. Check the first line of `Package.swift` for the tools version and
  ask the agent to target that. Left alone, it writes for whatever version it
  saw most of during training.

version_landscape: >
  Swift broke source compatibility hard between versions 1, 2, and 3, which is
  why answers from 2016 are useless. From version 5 onward it is stable, and 5 to
  6 is mostly a tightening of concurrency rules rather than a rewrite. The
  frameworks move faster than the language: something written for iOS 15 often
  needs edits for iOS 18, and the deprecation warning tells you which line.

see_also:
  - kotlin
  - objective-c
  - rust
  - c2-compiled-vs-interpreted
  - g2-package-managers
  - j1-how-to-recognize-a-language

keywords: [ios, macos, xcode, swiftui, optionals, apple, spm, package resolved]
---

Apple's language for its own platforms, made to replace Objective-C and largely
finished doing it. It compiles to a native binary with no runtime to install.

You will meet this by reading it far more often than by running it, because the build
tools live on macOS. That is a real limit and worth knowing before you ask an agent for
an iPhone app.

## The shape

Blocks use curly braces. Lines end without a semicolon, which puts Swift and Kotlin in a
small club together. The name comes first and the type follows a colon, and most of the
time the type is left out because the compiler works it out.

The feature that shapes everything else is the optional. A type written `String` always
holds a string. A type written `String?` might hold nothing, and the compiler will not let
you use it until you have said what happens in that case.

```swift
import Foundation

struct User {
    let id: Int
    var name: String?
}

func greet(_ user: User) -> String {
    guard let name = user.name else {
        return "hello stranger"
    }
    return "hello \(name)"
}

let ada = User(id: 1, name: "ada")
print(greet(ada))
```

Comments are `//` for a line and `/* */` for a block. The backslash inside the last string
is how a value gets inserted into text, and it is one of the loudest Swift tells there is.

`guard` is the other one. It reads as "make sure of this or leave", and no other language
in this deck has the word.

## Telling it from Kotlin

These two are close enough that people who write both mix them up daily. The folder
settles it instantly if you have one: `Package.swift` or a `.xcodeproj` means Swift, and
`build.gradle.kts` means Kotlin.

In a bare snippet, three words decide it.

| You see | It is |
|---|---|
| `func greet()` | Swift |
| `fun greet()` | Kotlin |
| `let count = 3` | Swift |
| `val count = 3` | Kotlin |
| `nil` | Swift |
| `null` | Kotlin |

## What it is for

iPhone, iPad, Mac, Watch, and Vision apps. It also runs server code on Linux, which is
real and rare enough that you should assume an Apple platform until proven wrong.

The screens are usually written in SwiftUI, where a view is a `struct` with a `body`
property and the at-sign markers such as `@State` hold the values the screen redraws for.

## Reading its errors

Compile errors carry a file, a line, and a column, and they frequently end with a fix you
can apply directly. Read the last line first.

Runtime crashes are blunter. The common one is `Fatal error: Unexpectedly found nil while
unwrapping an Optional value`, which means something in the code promised a value was
present by writing `!` after it, and it was not. The fix is never to add another
exclamation mark.
