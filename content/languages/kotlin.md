---
id: kotlin
title: Kotlin
type: language
verified: 2026-08-02
volatility: low

name: Kotlin
aka: [kt, kotlinlang, kotlin jvm, android kotlin]
family: bytecode
likelihood: possible
extensions: ['.kt', '.kts']

# Kotlin sits between Java and Swift, and it borrows from both. Every note below
# names which neighbor the token would be mistaken for.
tells:
  - pattern: 'fun'
    kind: token
    weight: 9
    note: >
      Kotlin declares functions with exactly `fun`. Swift and Go use `func`, Rust
      uses `fn`, Python and Scala use `def`. Java and C# name the return type
      first and use no keyword at all.
  - pattern: '!!'
    kind: sigil
    weight: 10
    note: >
      Two exclamation marks force a value that might be null to be treated as if
      it is not, and crash on the spot when it is. Only Kotlin has this operator.
      Finding one is both an identification and a warning.
  - pattern: '\?:'
    kind: operator
    weight: 9
    note: >
      The Elvis operator supplies a fallback when the left side is null, as in
      `name ?: "unknown"`. C# spells the same idea `??`. Java has no operator for
      it and writes an `if` instead.
  - pattern: 'data class'
    kind: regex
    weight: 9
    note: >
      `data class User(val id: Int)` generates the comparison and printing code
      for you. Java's nearest equivalent is `record` and C#'s is also `record`.
      The two words `data class` together are Kotlin only.
  - pattern: 'companion object'
    kind: regex
    weight: 9
    note: >
      Kotlin has no `static` keyword, so shared members live in a
      `companion object` inside the class. Java and C# both write `static` and
      have nothing that looks like this.
  - pattern: 'val [\w]+ ='
    kind: regex
    weight: 6
    note: >
      `val` cannot be reassigned, `var` can. Scala uses the same pair, so this
      does not settle Kotlin on its own. Java and C# have no `val`, and Swift
      writes `let` for the same idea.
  - pattern: 'package [\w.]+$'
    kind: line_start
    weight: 6
    note: >
      Kotlin's package line has no semicolon on the end. Java's does. Same
      keyword, one character apart, and it is visible from across the room.
  - pattern: '\?\.'
    kind: operator
    weight: 4
    note: >
      The safe call, `user?.name`, gives back null rather than crashing when
      `user` is null. C# and Swift have the same operator, so pair it with `fun`
      or `val` before you decide.

rules_out:
  - pattern: 'public static void main'
    kind: regex
    because: "Java. Kotlin writes fun main()"
  - pattern: 'System\.out\.println'
    kind: regex
    because: "Java. Kotlin calls println on its own"
  - pattern: 'Console\.WriteLine'
    kind: regex
    because: "C#"
  - pattern: 'func'
    kind: token
    because: Swift or Go
  - pattern: 'fn'
    kind: token
    because: Rust
  - pattern: 'def'
    kind: token
    because: Python, Ruby, or Scala
  - pattern: 'implements'
    kind: token
    because: "Java. Kotlin uses a single colon for both"

project_fingerprint:
  manifests:
    - file: build.gradle.kts
      decisive: false
      note: >
        Gradle's build file written in Kotlin, which is itself a Kotlin file. It
        points hard at a Kotlin project without settling it, because a Java
        project can use the Kotlin build language. The line that settles it is
        `kotlin("jvm")` or `id("org.jetbrains.kotlin.android")` in the `plugins`
        block.
    - file: build.gradle
      decisive: false
      note: >
        The Groovy spelling of the same file. Common on older Android projects
        that are written in Kotlin anyway, so read the plugins block rather than
        trusting the filename.
    - file: gradle/libs.versions.toml
      decisive: false
      note: >
        A version catalog listing every dependency in one TOML (Tom's Obvious
        Minimal Language) file. Recent Gradle projects keep versions here instead
        of in the build file.
  build_dirs: ['build/', '.gradle/', 'out/']
  entry_points: ['src/main/kotlin/', 'src/main/kotlin/Main.kt', 'app/src/main/java/']

shape:
  blocks: braces
  statement_end: optional_semicolon
  comment_line: '//'
  comment_block: '/* */'
  string_quotes: >
    Double quotes, with `$name` inside them for values and `${a + b}` for
    expressions. Triple quotes hold text across several lines. Single quotes mean
    one character.
  naming: camelCase for functions and variables, PascalCase for classes, SCREAMING_SNAKE_CASE for constants
  import_keyword: import

tooling:
  package_manager: Gradle
  registry: Maven Central
  runtime: 'the JVM, or Android, or a native binary'
  install_command: 'add a line to the dependencies block in build.gradle.kts'
  run_command: '.\gradlew.bat run'
  test_command: '.\gradlew.bat test'

confusable_with:
  - language: java
    settle_it: >
      They compile to the same bytecode and share a folder layout. Kotlin writes
      `fun`, `val`, and types after the name, and leaves semicolons off the ends
      of lines. Java writes the type first and ends every statement with a
      semicolon.
    tiebreak: { pattern: 'public static void main', kind: regex, favors: java }
  - language: swift
    settle_it: >
      Both drop semicolons, both put the type after a colon, and both read
      almost identically. Kotlin says `fun` and `val`, Swift says `func` and
      `let`. If null is written `null` it is Kotlin; Swift writes `nil`.
    tiebreak: { pattern: 'func', kind: token, favors: swift }
  - language: scala
    settle_it: >
      Both run on the same runtime and both use `val` and `object`. Scala
      declares methods with `def` and writes `case class`. Kotlin declares with
      `fun` and writes `data class`.
    tiebreak: { pattern: 'def', kind: token, favors: scala }

errors_look_like:
  sample: |
    Exception in thread "main" java.lang.NullPointerException
        at MainKt.greet(Main.kt:12)
        at MainKt.main(Main.kt:4)
  recognize_by: >
    A stack trace that looks like Java's, because it is one, with two Kotlin
    giveaways. The filenames end in `.kt` rather than `.java`, and the class
    names carry a `Kt` suffix such as `MainKt`, which is the wrapper the compiler
    invents for functions that sit outside any class. Compile errors are plainer
    than Java's and name the file and column, as
    `Main.kt:12:9: error: unresolved reference: prnt`.
  patterns:
    - '^\s+at [\w.$]+\(\w+\.kt:\d+\)'
    - '\w+\.kt:\d+:\d+: error:'
    - 'kotlin\.\w*(Exception|Error)'

meet_it_when: >
  You meet it in a build file before you meet it in an app, because
  `build.gradle.kts` is Kotlin. After that it is Android, where it has been the
  default language since 2019, and any backend team that got tired of Java and
  had a runtime they could not leave.

what_agents_get_wrong: >
  The whole point of Kotlin is that the compiler tracks which values can be null,
  and the whole risk is that an agent silences it. Watch a diff for `!!`, which
  means "trust me, this is not null" and turns a compile-time check into a
  runtime crash. Watch for `lateinit var` used to dodge writing a constructor.
  Both compile, both look tidy, both fail on a real device. The second seam is
  coroutines, Kotlin's way of doing background work: agents reach for
  `GlobalScope.launch`, which starts work nothing is left holding and which no
  current guide recommends, or wrap the call in `runBlocking` and freeze the
  screen. The third is Android drift. Code from before 2022 imports
  `kotlinx.android.synthetic`, a shortcut that was deleted, and it will not
  build. Anything referring to `findViewById` in a project using Compose is the
  agent mixing two eras of Android in one file. Last, Java leaking through: a
  Kotlin file full of getters, setters, and semicolons is Java that an agent
  translated word for word rather than rewrote.

version_landscape: >
  Kotlin moves faster than Java and breaks small things at each step, but the
  language itself is stable enough that a 2021 answer usually still compiles. The
  churn is in the ecosystem around it: Gradle plugin versions, the Compose
  compiler, and Android build tooling all have to agree with each other, and a
  version mismatch there produces errors that look like the code is wrong when it
  is not.

see_also:
  - java
  - swift
  - scala
  - csharp
  - c2-compiled-vs-interpreted
  - g2-package-managers
  - j1-how-to-recognize-a-language

keywords: [android, gradle, jetbrains, coroutines, compose, null safety, kts, data class]
---

Java's replacement, made by the company that makes the editor most Java is written in.
It runs on the same runtime, calls the same libraries, and says the same things in about
half the lines.

Files end in `.kt`. Pronounced like the island, with the stress on the first syllable.

## The shape

Blocks use curly braces. Statements end at the end of the line, and a semicolon is legal
but nobody writes one, so a brace language with no semicolons is either this or Swift.

The type comes after the name, following a colon, and most of the time you leave it out
entirely because the compiler works it out. This is backwards from Java, and it is the
single fastest way to tell the two apart on screen.

```kotlin
package com.example.app

data class User(val id: Int, val name: String)

fun greet(user: User?): String {
    val name = user?.name ?: "stranger"
    return "hello $name"
}

fun main() {
    val users = mutableListOf(User(1, "ada"))
    println(greet(users.firstOrNull()))
}
```

Comments are `//` for a line and `/* */` for a block.

Two things in that snippet do the heavy lifting. `User?` with a question mark means the
value is allowed to be null, and Kotlin will not let you touch it without saying what
happens when it is. `?:` supplies the fallback. Java has neither, which is why Java
programs meet `NullPointerException` and Kotlin programs mostly do not.

## Telling it from Java

Both compile to the same bytecode and sit in the same folders, so the folder gives you
nothing. Read the lines instead.

| You see | It is |
|---|---|
| `fun greet(name: String): String` | Kotlin |
| `public String greet(String name)` | Java |
| `val count = 3` or `var count = 3` | Kotlin |
| `final int count = 3;` | Java |
| No semicolons at line ends | Kotlin |
| Semicolon on every statement | Java |
| `println("hi")` | Kotlin |
| `System.out.println("hi")` | Java |

A mixed project is normal rather than a mistake. Kotlin and Java files live side by side
in one build and call each other freely, so finding both under `src/main` means the team
is part way through a migration.

## Telling it from Swift

These two look more alike than either does to anything else. Both drop semicolons, both
put the type after a colon, both use a question mark for values that may be missing.

Kotlin says `fun`, `val`, and `null`. Swift says `func`, `let`, and `nil`. That is the
whole test and it takes two seconds.

## What it is for

Android, first and mostly. It is also used for server backends, for Gradle build scripts,
and for code shared across platforms through Kotlin Multiplatform.

You can build Android apps on Windows with Android Studio, so this is one of the few
Apple-adjacent corners of the deck that is fully open to you.

## Reading its errors

At runtime you get a Java stack trace, because it is a Java stack trace. Look for `.kt`
filenames and class names ending in `Kt` to know you are in Kotlin code rather than in a
library underneath it.

Compile errors name the file, the line, and the column, and they are usually specific
enough to fix without a search. The exception is Gradle, which fails long before the
compiler does and buries the real message in fifty lines of build output. Search that
output for the word `error` and read the first hit.
