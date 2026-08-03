---
id: csharp
title: "C# (C Sharp)"
type: language
verified: 2026-08-02
volatility: low

name: "C#"
aka: [c-sharp, cs, dotnet, dot net, net, aspnet]
family: bytecode
likelihood: likely
extensions: ['.cs', '.csproj', '.sln', '.csx']

# Java is the nearest neighbor on nearly every line here, and the contrast is
# stated the same way on the Java card. If you change one, change both.
tells:
  - pattern: 'Console\.WriteLine'
    kind: regex
    weight: 10
    note: >
      C# prints with `Console.WriteLine`, capital W and capital L. Java prints
      with `System.out.println`, Kotlin and Scala with a bare `println`, Swift
      with `print`.
  - pattern: 'static void Main\(string\[\] args\)'
    kind: regex
    weight: 10
    note: >
      C# capitalizes the entry point and lowercases the string type. Java writes
      `public static void main(String[] args)` with a lowercase m and a capital S.
      That one letter of case is the whole difference.
  - pattern: 'using System'
    kind: line_start
    weight: 9
    note: >
      C# files open with `using System;`. Java opens with `import java.util.List;`
      and has no `using` keyword. C++ writes `using namespace std;`, which comes
      after a block of `#include` lines that C# never has.
  - pattern: 'get;\s*set;'
    kind: regex
    weight: 9
    note: >
      An auto-property, as in `public string Name { get; set; }`. Java writes a
      private field plus `getName()` and `setName()` methods by hand. Kotlin
      declares `var name` and generates both silently.
  - pattern: '\$"'
    kind: sigil
    weight: 8
    note: >
      A dollar sign immediately before a quote starts an interpolated string,
      `$"hello {name}"`. Kotlin puts the dollar inside the quotes as
      `"hello $name"`, and Swift writes `"hello \(name)"`.
  - pattern: 'namespace'
    kind: token
    weight: 7
    note: >
      C# groups code with `namespace Example.App`. Java uses `package` and ends
      the line with a semicolon. TypeScript has a legacy `namespace` too, so pair
      this with `Console.WriteLine` before deciding.
  - pattern: '\bstring \w+ ='
    kind: regex
    weight: 7
    note: >
      The lowercase `string` type. Java always capitalizes it as `String`. C#
      accepts the capital form as an alias, so lowercase proves C# and capital
      proves nothing.
  - pattern: '=>'
    kind: operator
    weight: 5
    note: >
      C# writes lambdas and one-line members with a fat arrow, `x => x * 2`. Java
      uses a thin arrow, `x -> x * 2`. One character apart, and in a file full of
      short functions it is the loudest signal on screen.

rules_out:
  - pattern: 'System\.out\.println'
    kind: regex
    because: Java
  - pattern: 'public static void main'
    kind: regex
    because: "Java. C# capitalizes Main and drops the public"
  - pattern: 'import'
    kind: line_start
    because: "Java, Kotlin, Python, or JavaScript. C# has no import keyword"
  - pattern: 'implements'
    kind: token
    because: "Java. C# uses a single colon for both base classes and interfaces"
  - pattern: 'fun'
    kind: token
    because: Kotlin
  - pattern: 'func'
    kind: token
    because: Swift or Go
  - pattern: '#include'
    kind: line_start
    because: C or C++

project_fingerprint:
  manifests:
    - file: '*.csproj'
      decisive: true
      note: >
        The project file, written in XML (Extensible Markup Language). One of
        these in a folder settles the language on its own, because the sibling
        formats are named `.fsproj` and `.vbproj` for other languages. It also
        names the framework version, in a `<TargetFramework>` line such as
        `net8.0`.
    - file: '*.sln'
      decisive: false
      note: >
        A solution file, which is a list of project files that open together. It
        proves the project belongs to Microsoft's ecosystem without proving the
        language, so open it and read which project files it points at.
    - file: Directory.Build.props
      decisive: false
      note: >
        Settings shared by every project file under this folder. Worth reading
        when a setting seems to come from nowhere.
  lockfiles: ['packages.lock.json']
  build_dirs: ['bin/', 'obj/']
  entry_points: ['Program.cs', 'Startup.cs', 'MauiProgram.cs']

shape:
  blocks: braces
  statement_end: semicolon
  comment_line: '//'
  comment_block: '/* */'
  string_quotes: >
    Double quotes for text, single quotes for one character. A leading `$` makes
    the string interpolated and a leading `@` makes it verbatim, so backslashes
    in a Windows path stay literal.
  naming: PascalCase for methods and classes, camelCase for locals, _camelCase for private fields
  import_keyword: using

tooling:
  package_manager: NuGet
  registry: nuget.org
  runtime: '.NET, installed once and used by every project'
  install_command: 'dotnet add package <package-name>'
  run_command: dotnet run
  test_command: dotnet test

confusable_with:
  - language: java
    settle_it: >
      Check the string type and the print call. C# writes `string` lowercase and
      prints with `Console.WriteLine`. Java capitalizes `String` and prints with
      `System.out.println`. In the folder, a `.csproj` file means C# and a
      `pom.xml` or `build.gradle` means Java.
    tiebreak: { pattern: 'System\.out\.println', kind: regex, favors: java }
  - language: typescript
    settle_it: >
      Both write `public class`, `interface`, `=>`, and lowercase `string`.
      TypeScript imports with `import ... from "..."` and declares functions with
      `function` or `const`. C# opens with `using System;` and never writes
      `function`.
    tiebreak: { pattern: 'import .* from', kind: regex, favors: typescript }
  - language: kotlin
    settle_it: >
      Both use `?.` for safe access and both drop boilerplate Java needs. Kotlin
      declares with `fun`, `val`, and `var name: String`, putting the type after
      the name. C# puts the type first and ends every line with a semicolon.
    tiebreak: { pattern: 'fun', kind: token, favors: kotlin }
  - language: powershell
    settle_it: >
      PowerShell can call the same libraries C# uses, so a line like
      `[System.IO.Path]::GetFullPath($p)` appears in both. C# files open with
      `using System;`, put everything inside a `class`, and end every statement with a
      semicolon. PowerShell has a `$` on every variable and no class around the code.
    tiebreak: { pattern: '$env:', kind: sigil, favors: powershell }

errors_look_like:
  sample: |
    Unhandled exception. System.NullReferenceException: Object reference not set
    to an instance of an object.
       at Example.App.Program.Greet(String name) in C:\Users\nyx\app\Program.cs:line 14
       at Example.App.Program.Main(String[] args) in C:\Users\nyx\app\Program.cs:line 7
  recognize_by: >
    The first line is the words `Unhandled exception.` followed by a type
    starting with `System.`. Every frame below reads `at Namespace.Class.Method`
    and then `in <full path>:line 14`. Java writes the same location in
    parentheses as `(App.java:7)` and never uses the word `line`. Build errors
    are different again and carry a code like `CS0103`, where the `CS` prefix
    means the C# compiler.
  patterns:
    - 'Unhandled exception\.'
    - 'System\.\w+Exception'
    - '^\s+at .+ in .+:line \d+'
    - '\bCS\d{4}\b'

meet_it_when: >
  It is the native language of the machine you are already on. An agent picks it
  for a Windows desktop app or a web backend, Unity games are scripted in it, and
  a large share of business software you will ever be paid to touch is written in
  it.

what_agents_get_wrong: >
  Start with null. Modern projects turn on nullable reference types, which makes
  the compiler treat `string` and `string?` as different, and an agent writing
  pre-2019 style code produces a wall of `CS8600` and `CS8602` warnings that
  build fine and then throw `NullReferenceException` in front of a user. If the
  build output scrolls past with warnings nobody read, that is the seam. Next,
  the entry point. Since .NET (Microsoft's runtime and standard library, said out
  loud as dot net) version 6, a new `Program.cs` holds bare statements with no
  class around them, so an agent that helpfully adds `static void Main` produces
  `CS0017`, the program has more than one entry point. Third, stale libraries:
  `Newtonsoft.Json` where the project uses `System.Text.Json`, `WebClient`
  instead of `HttpClient`, and a fresh `new HttpClient()` inside a method, which
  runs out of sockets under load and works perfectly in every test. Last, watch
  for Java leaking in. `System.out.println` will not compile, but a lowercase
  `main` compiles fine as an ordinary method and leaves you with `CS5001`, no
  suitable entry point, which reads like a project problem and is a typo.

version_landscape: >
  The runtime split in two and rejoined. The old Windows-only .NET Framework
  stopped at 4.8 and is maintained rather than developed. Everything current is
  the cross-platform line, numbered 5, 6, 7, 8, and up, and an answer written for
  .NET Framework often will not work there. Check `<TargetFramework>` in the
  `.csproj` file first: `net8.0` is current, `net48` is the old world. The
  language itself adds features fast and removes almost nothing, so old C# code
  still compiles.

see_also:
  - java
  - kotlin
  - typescript
  - c2-compiled-vs-interpreted
  - f2-stack-traces
  - g2-package-managers
  - j1-how-to-recognize-a-language

keywords: [dotnet, nuget, csproj, solution file, visual studio, unity, asp net core, nullreferenceexception]
---

Microsoft's answer to Java, twenty-five years old, and the default language of the
operating system you are sitting on. It compiles to bytecode and runs on .NET (Microsoft's
runtime and standard library, said out loud as dot net).

The name is spoken "see sharp". The sharp sign is borrowed from music, where it moves a
note up one step, which was as close to "the successor to C++" as marketing could get
without saying it.

## The shape

Blocks use curly braces. Every statement ends with a semicolon. The type comes before the
name: `string name = "nyx";` reads as type, name, value.

Capitalization carries real meaning here, more than in any neighboring language. Methods
and classes are PascalCase, so `Console.WriteLine` and `Main` are capitalized where Java
would write `println` and `main`.

```csharp
using System;
using System.Collections.Generic;

namespace Example.App;

public class Program
{
    public string Name { get; set; } = "nyx";

    static void Main(string[] args)
    {
        var names = new List<string>();
        names.Add("nyx");
        Console.WriteLine($"hello {names[0]}");
    }
}
```

Comments are `//` for a line and `/* */` for a block. Three slashes, `///`, starts a
documentation comment that the editor reads back to you as a tooltip.

Since version 6, a small program can skip all of it. A `Program.cs` holding two bare
lines with no class and no `Main` is legal and common, so do not read a missing entry
point as a broken file.

## Telling it from Java

These two are the closest pair in the deck. Four checks, fastest first.

| You see | It is |
|---|---|
| `string name` all lowercase | C# |
| `String name` with a capital S | Java |
| `Console.WriteLine("hi")` | C# |
| `System.out.println("hi")` | Java |
| `namespace Example;` then `using System;` | C# |
| `package com.example;` then `import java.util.List;` | Java |
| `class Dog : Animal, IPet` | C# |
| `class Dog extends Animal implements Pet` | Java |

The entry point settles it alone. C# writes `static void Main(string[] args)` with a
capital M. Java writes `public static void main(String[] args)` with a lowercase m.

One catch: capital `String` is legal C# too, because it is an alias for the same type.
Lowercase `string` proves C#, capital `String` proves nothing on its own, so look at the
print call next.

In the folder, a `.csproj` or `.sln` file means C#, and `pom.xml` or `build.gradle` means
Java.

## What it is for

Windows desktop applications, web backends, Unity games, and the internal line-of-business
software that keeps companies running. It also builds for macOS and Linux now, which was
not true for its first fifteen years.

Everything runs through one command-line tool called `dotnet`: it creates projects, adds
packages, builds, runs, and tests. If you learn one command here, learn `dotnet run`.

## Reading its errors

There are two kinds and they look nothing alike. A build error is one line with a code
like `CS0103: The name 'foo' does not exist in the current context`, and the code is worth
pasting into a search box because the numbering is stable and documented.

A crash at runtime starts with `Unhandled exception.` and then a stack, newest call first.
Read down to the first line naming a file inside your own project. The full Windows path
sits right there on the frame, which is more than most languages give you.
