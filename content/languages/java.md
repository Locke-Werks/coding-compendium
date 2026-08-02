---
id: java
title: Java
type: language
verified: 2026-08-02
volatility: low

name: Java
aka: [jdk, jre, openjdk, java8, java17, java21, jvm]
family: bytecode
likelihood: likely
extensions: ['.java', '.jar', '.class']

# Every note below contrasts against the nearest neighbor, which for Java is C#
# on almost every line. The two were designed a few years apart with the same
# shape, so a tell that does not name the difference teaches nothing.
tells:
  - pattern: 'public static void main\(String\[\] args\)'
    kind: regex
    weight: 10
    note: >
      Java's entry point, character for character. C# writes
      `static void Main(string[] args)`, with a capital M, a lowercase `string`,
      and no `public`. Kotlin writes `fun main()`. Nothing else spells it this way.
  - pattern: 'System\.out\.println'
    kind: regex
    weight: 10
    note: >
      Java prints with `System.out.println`. C# prints with `Console.WriteLine`,
      Kotlin and Scala with a bare `println`, Swift with `print`.
  - pattern: 'package [\w.]+;'
    kind: line_start
    weight: 7
    note: >
      Java's first line names the package and ends in a semicolon. C# says
      `namespace` for the same idea. Kotlin writes the same `package` line and
      leaves the semicolon off.
  - pattern: 'implements'
    kind: token
    weight: 8
    note: >
      Java splits inheritance into two words, `extends` for a class and
      `implements` for an interface. C#, Kotlin, Swift, and Scala all use a
      single colon for both.
  - pattern: '@Override'
    kind: sigil
    weight: 8
    note: >
      Java metadata is an at-sign plus a capitalized name, sitting on its own
      line above the thing it describes. C# writes the same idea in square
      brackets, as `[Test]`, and spells override as a plain keyword.
  - pattern: 'throws \w+Exception'
    kind: regex
    weight: 9
    note: >
      A signature ending in `throws IOException` is a checked exception, which
      the compiler forces the caller to handle. Only Java has these. C# has no
      `throws` clause at all.
  - pattern: 'import java\.'
    kind: regex
    weight: 8
    note: >
      Java pulls in its standard library with `import java.util.List;`. C# has no
      `import` keyword and writes `using System.Collections.Generic;` instead.
  - pattern: '<>\s*\('
    kind: regex
    weight: 6
    note: >
      The empty diamond in `new ArrayList<>()` tells Java to infer the type
      inside. C# repeats it as `new List<string>()` or shortens the whole thing
      to `new()`.

rules_out:
  - pattern: 'Console\.WriteLine'
    kind: regex
    because: "C#, the only other language in the deck that looks this much like Java"
  - pattern: 'using System'
    kind: line_start
    because: "C#"
  - pattern: 'namespace'
    kind: token
    because: "C#. Java has no namespace keyword, it says package"
  - pattern: 'fun'
    kind: token
    because: Kotlin
  - pattern: 'val'
    kind: token
    because: Kotlin or Scala
  - pattern: 'func'
    kind: token
    because: Swift or Go
  - pattern: '#include'
    kind: line_start
    because: C or C++

project_fingerprint:
  manifests:
    - file: pom.xml
      decisive: false
      note: >
        Maven's build file, written in XML (Extensible Markup Language) around a
        `<dependencies>` block. It proves the project runs on the JVM (Java
        Virtual Machine) without proving the language, because Kotlin and Scala
        build with Maven too. The source folder settles it.
    - file: build.gradle
      decisive: false
      note: >
        Gradle's build file, written in Groovy. Standard for Android and common
        for server Java. The same file spelled `build.gradle.kts` is written in
        Kotlin, which hints at a Kotlin project without settling it.
    - file: gradlew.bat
      decisive: false
      note: >
        The Gradle wrapper for Windows. Its presence means you build with
        `.\gradlew.bat` rather than installing Gradle yourself.
  build_dirs: ['target/', 'build/', 'out/']
  entry_points: ['src/main/java/', 'src/main/java/**/Main.java', 'src/main/java/**/App.java']

shape:
  blocks: braces
  statement_end: semicolon
  comment_line: '//'
  comment_block: '/* */'
  string_quotes: >
    Double quotes for text. Single quotes mean exactly one character, which is a
    different type called `char`. C# follows the same rule, Python and JavaScript
    do not.
  naming: camelCase for methods and variables, PascalCase for classes, SCREAMING_SNAKE_CASE for constants
  import_keyword: import

tooling:
  package_manager: Maven or Gradle
  registry: Maven Central
  runtime: the JVM, installed separately as a JDK
  install_command: 'edit pom.xml or build.gradle by hand, there is no add command'
  run_command: 'mvn exec:java, or java -jar target/app.jar'
  test_command: 'mvn test, or .\gradlew.bat test'

confusable_with:
  - language: csharp
    settle_it: >
      Check the string type and the print call. Java capitalizes `String` and
      prints with `System.out.println`. C# writes `string` lowercase and prints
      with `Console.WriteLine`. In the folder, `pom.xml` or `build.gradle` means
      Java and a `.csproj` file means C#.
    tiebreak: { pattern: 'Console\.WriteLine', kind: regex, favors: csharp }
  - language: kotlin
    settle_it: >
      Kotlin runs on the same runtime and reads much shorter. If lines end in
      semicolons and functions declare a return type first, it is Java. If you
      see `fun`, `val`, or a line with no semicolon at the end, it is Kotlin.
    tiebreak: { pattern: 'fun', kind: token, favors: kotlin }
  - language: scala
    settle_it: >
      Both compile to the same bytecode and both use `class` and `extends`.
      Scala declares methods with `def` and uses `case class`. Java has neither
      word.
    tiebreak: { pattern: 'def', kind: token, favors: scala }

errors_look_like:
  sample: |
    Exception in thread "main" java.lang.NullPointerException: Cannot invoke
    "String.length()" because "name" is null
        at com.example.app.App.greet(App.java:14)
        at com.example.app.App.main(App.java:7)
  recognize_by: >
    Three things together. The first line starts `Exception in thread "main"`.
    The type is a dotted path beginning `java.lang.` or `java.util.`. Every line
    below starts with the word `at` and ends with a filename and line number in
    parentheses, as `(App.java:7)`. C# frames also start with `at`, but they
    finish with `in C:\path\Program.cs:line 12` and never use parentheses for
    the location.
  patterns:
    - 'Exception in thread "[^"]+"'
    - '^\s+at [\w.$]+\(\w+\.java:\d+\)'
    - 'java\.(lang|util|io)\.\w+(Exception|Error)'

meet_it_when: >
  An agent picks it for an Android app or a company backend. You also meet it
  sideways, when a tool you installed falls over and prints a stack trace at you:
  Elasticsearch, Kafka, Jenkins, and most of the plumbing under a build server
  are Java.

what_agents_get_wrong: >
  Three specific things. First, agents write Java that was current in 2014. Watch
  a diff for `SimpleDateFormat`, `Date`, and `Calendar`, which the `java.time`
  package replaced years ago and which break in ways that only show up under
  load. Second, null. Java has no compile-time null checking, so an agent
  returning `null` from a helper produces code that builds cleanly and throws
  `NullPointerException` on the first real input. Third, and most expensive, is
  framework drift. Spring Boot 3 renamed every `javax.` package to `jakarta.`, so
  an agent trained on older code writes `import javax.persistence.Entity;` into a
  project whose other files all say `jakarta.persistence`. It looks correct next
  to a hundred examples online and it will not compile. If you see `javax.` and
  `jakarta.` in the same project, that is the bug. Also check that the agent has
  not mixed C# into the file: a capital `Main`, a bare `override`, or the word
  `namespace` means it lost track of which language it was writing.

version_landscape: >
  Java almost never breaks old code, so a 2015 answer usually still compiles. The
  version numbers are the confusing part. Java 8 was the world for a decade and
  is still everywhere; 11, 17, and 21 are the long-support versions after it. A
  tutorial written for 8 works on 21. The reverse is not true, because `var`,
  records, and switch expressions are all newer than 8.

see_also:
  - csharp
  - kotlin
  - scala
  - c2-compiled-vs-interpreted
  - f2-stack-traces
  - g2-package-managers
  - j1-how-to-recognize-a-language

keywords: [jvm, maven, gradle, spring boot, jar file, bytecode, openjdk, android, nullpointerexception]
---

A language that compiles to bytecode and runs on the JVM (Java Virtual Machine), which
is why one build of a Java program runs on Windows, macOS, and Linux with no rebuild.

Verbose on purpose. Everything lives inside a class, every class sits in a package, and
the line that starts the program is six words long before it reaches a name you chose.

## The shape

Blocks use curly braces. Every statement ends with a semicolon. Indentation is four
spaces and means nothing to the compiler, unlike Python where it means everything.

The type comes before the name: `String name = "nyx";` reads as type, name, value. Hold
on to that reading order, because it is what separates Java from Kotlin and Swift, which
put the name first and the type after a colon.

```java
package com.example.app;

import java.util.ArrayList;
import java.util.List;

public class App {
    public static void main(String[] args) {
        String name = "nyx";
        final int max = 100;               // final means it cannot be reassigned
        List<String> names = new ArrayList<>();
        names.add(name);
        System.out.println("hello " + name);
    }
}
```

Comments are `//` for a line and `/* */` for a block. A block opening with `/**` is a doc
comment, which tooling reads to build documentation pages.

## Telling it from C#

These two are the closest pair in the deck. They were designed a few years apart to look
the same, and at a glance they succeed. Four checks, fastest first.

| You see | It is |
|---|---|
| `String name` with a capital S | Java |
| `string name` all lowercase | C# |
| `System.out.println("hi")` | Java |
| `Console.WriteLine("hi")` | C# |
| `package com.example;` then `import java.util.List;` | Java |
| `namespace Example;` then `using System;` | C# |
| `class Dog extends Animal implements Pet` | Java |
| `class Dog : Animal, IPet` | C# |

The entry point settles it alone. Java writes `public static void main(String[] args)`
with a lowercase m. C# writes `static void Main(string[] args)` with a capital M.

One catch worth knowing: capital `String` is legal in C# as well, because it is an alias
for the same type. Lowercase `string` proves C#. Capital `String` proves nothing on its
own, so check the print call next.

In the folder, `pom.xml` or `build.gradle` means Java, and a `.csproj` or `.sln` file
means C#.

## What it is for

Android apps, banking and insurance systems, Minecraft and its mod ecosystem, and large
company backends that have been running for twenty years and will run for twenty more.

Java needs the runtime installed on whatever machine runs it, so there is no single `.exe`
to hand someone. You install the JDK (Java Development Kit) to build, and it includes the
runtime you need to run.

Maven and Gradle are the two build tools, and a project uses one or the other. Maven has
no lockfile, which lands oddly if you arrived from `npm` or `cargo`: the version you get
is whatever the range in `pom.xml` resolves to today.

## Reading its errors

The first line names the exception type and, in recent versions, the exact variable that
was null. Everything under it is the call stack, newest call first. Read down until you
hit the first line naming a file inside your own project, because that is where your part
of the problem is.
