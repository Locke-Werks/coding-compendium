---
id: scala
title: Scala
type: language
verified: 2026-08-02
volatility: low

name: Scala
aka: [scala3, sbt, spark, akka]
family: bytecode
likelihood: unlikely
extensions: ['.scala', '.sc', '.sbt']

tells:
  - pattern: 'case class'
    kind: regex
    weight: 9
    note: >
      `case class User(id: Int)` is Scala. Kotlin writes `data class` for the
      same idea, Java writes `record`, and neither uses the word `case` outside a
      switch.
  - pattern: 'def'
    kind: token
    weight: 6
    note: >
      Scala declares methods with `def`, borrowed from Python. Kotlin says `fun`,
      Java and C# name the return type first with no keyword at all.
  - pattern: 'implicit|given'
    kind: regex
    weight: 9
    note: >
      `implicit` in Scala 2 and `given` in Scala 3 let the compiler supply an
      argument you never passed. No other language in this deck has either word,
      and both are why Scala code can be hard to follow by eye.
  - pattern: 'object [A-Z]'
    kind: line_start
    weight: 7
    note: >
      A top-level `object Main` is a class with exactly one instance, standing in
      for Java's `static`. Kotlin also has `object`, so check for `def` before
      deciding.
  - pattern: '=>'
    kind: operator
    weight: 4
    note: >
      Scala writes anonymous functions with a fat arrow, as C# does. Java uses a
      thin `->`. On its own this settles nothing, so pair it with `val` or `def`.

rules_out:
  - pattern: 'fun'
    kind: token
    because: Kotlin
  - pattern: 'System\.out\.println'
    kind: regex
    because: "Java. Scala calls println on its own"
  - pattern: 'Console\.WriteLine'
    kind: regex
    because: "C#"
  - pattern: 'fn'
    kind: token
    because: Rust

project_fingerprint:
  manifests:
    - file: build.sbt
      decisive: true
      note: >
        The build file for sbt, the Scala build tool. Its presence settles the
        language, and it is itself written in Scala.
    - file: project/build.properties
      decisive: false
      note: >
        Pins the sbt version. Sits beside `build.sbt` in every sbt project.
    - file: pom.xml
      decisive: false
      note: >
        Some Scala projects build with Maven instead. Check for a
        `src/main/scala` folder before assuming Java.
  build_dirs: ['target/', 'project/target/']
  entry_points: ['src/main/scala/', 'src/main/scala/Main.scala']

shape:
  blocks: braces
  statement_end: optional_semicolon
  comment_line: '//'
  comment_block: '/* */'
  string_quotes: 'Double quotes, with an `s` prefix for interpolation as in `s"hello $name"`'
  naming: camelCase for methods and values, PascalCase for types
  import_keyword: import

tooling:
  package_manager: sbt
  registry: Maven Central
  runtime: 'the JVM, the same one Java uses'
  run_command: sbt run
  test_command: sbt test

confusable_with:
  - language: java
    settle_it: >
      Both compile to the same bytecode and share the folder layout. Scala
      declares with `def` and `val`, writes `case class`, and leaves semicolons
      off. Java writes the type first and ends every statement with one.
    tiebreak: { pattern: 'def', kind: token, favors: scala }
  - language: kotlin
    settle_it: >
      Both use `val`, `var`, and `object`, and both drop semicolons. Scala says
      `def` and `case class`. Kotlin says `fun` and `data class`.
    tiebreak: { pattern: 'fun', kind: token, favors: kotlin }

errors_look_like:
  sample: |
    Exception in thread "main" scala.MatchError: 7 (of class java.lang.Integer)
        at Main$.classify(Main.scala:9)
  recognize_by: >
    A Java stack trace with `.scala` filenames and class names ending in a dollar
    sign, such as `Main$`. `scala.MatchError` is the signature failure, thrown
    when a `match` block has no branch for the value it was handed.
  patterns:
    - '^\s+at .+\(\w+\.scala:\d+\)'
    - 'scala\.\w*(Exception|Error)'

meet_it_when: >
  Data pipelines built on Spark, and backends written by teams that wanted more
  than Java allowed in 2013. You are most likely to meet it reading someone
  else's repository rather than starting one.

what_agents_get_wrong: >
  Agents mix Scala 2 and Scala 3 in one file. The versions differ in how blocks
  are written, braces in 2 and optional indentation in 3, and in how the compiler
  is told to supply arguments, `implicit` in 2 against `given` and `using` in 3.
  The result compiles under neither. Check the Scala version in `build.sbt`
  first, then check that the file matches it. The second habit to watch is
  library sprawl: agents reach for Cats, ZIO, or Akka mid-file, and each drags a
  different style of writing the same program into a codebase that already chose
  one.

see_also:
  - java
  - kotlin
  - j1-how-to-recognize-a-language

keywords: [sbt, spark, akka, functional, jvm, case class]
---

Java's clever cousin. It runs on the same runtime and calls the same libraries, and it
lets you write in a much denser style, which is either the attraction or the problem
depending on who inherited the code.

```scala
case class User(id: Int, name: String)

object Main:
  def greet(user: User): String = s"hello ${user.name}"

  def main(args: Array[String]): Unit =
    println(greet(User(1, "nyx")))
```

Three things in there settle it. `case class` is Scala where Kotlin says `data class`.
`def` declares a method. The `s` before a string turns on interpolation, so `${...}` gets
filled in.

The folder is faster still: `build.sbt` at the root means Scala and nothing else.

Read the version at the top of `build.sbt` before trusting anything you find online. Scala
3 changed enough that a 2 answer can be actively wrong.
