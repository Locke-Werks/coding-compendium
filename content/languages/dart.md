---
id: dart
title: Dart
type: language
verified: 2026-08-02
volatility: low

name: Dart
aka: [dartlang, flutter, dart2]
family: compiled
likelihood: possible
extensions: ['.dart']

tells:
  - pattern: 'BuildContext'
    kind: token
    weight: 9
    note: >
      Appears in every Flutter screen as `Widget build(BuildContext context)`.
      It belongs to the framework rather than the language, and it appears in no
      other language in this deck, which makes it the fastest tell here.
  - pattern: '\brequired\s+this\.'
    kind: regex
    weight: 9
    note: >
      Dart's named-parameter syntax, as in `const Card({required this.title})`.
      TypeScript marks the opposite case with `?:` for optional, JavaScript has no
      named parameters at all, and nothing else writes `this.` in a parameter list.
  - pattern: '@override'
    kind: regex
    weight: 7
    note: >
      Lowercase. Java writes the same annotation as `@Override` with a capital O,
      and Python writes `@decorator` for something unrelated. The capitalization is
      the whole difference between Dart and Java here.
  - pattern: '\blate\s+\w'
    kind: regex
    weight: 7
    note: >
      `late` promises a value will be set before anything reads it. No other
      language in this deck has the keyword. Kotlin's nearest equivalent is spelled
      `lateinit`.
  - pattern: '^\s*(final|var|const)\s+\w+\s*='
    kind: regex
    weight: 4
    note: >
      Dart puts the type before the name (`String name = "nyx"`) or omits it with
      `var` and `final`. TypeScript puts the type after the name with a colon, which
      is the fastest way to separate the two.

rules_out:
  - pattern: '\bfunction\b'
    because: JavaScript or TypeScript. Dart declares a function by writing its return type first.
  - pattern: 'def'
    because: Python or Ruby
  - pattern: 'fn'
    because: Rust
  - pattern: 'console\.log'
    kind: regex
    because: JavaScript or TypeScript. Dart prints with `print()`.
  - pattern: 'System\.out\.println'
    kind: regex
    because: Java, which Dart resembles closely enough to matter.

project_fingerprint:
  manifests:
    - file: pubspec.yaml
      decisive: true
      note: >
        Names the project and its packages. If it is at the root, the project is
        Dart, almost always Flutter. The single most reliable check on this card.
  lockfiles: [pubspec.lock]
  build_dirs: [build/, .dart_tool/]
  entry_points: [lib/main.dart, bin/main.dart]

shape:
  blocks: braces
  statement_end: semicolon
  comment_line: '//'
  comment_block: '/* */'
  string_quotes: >
    Single quotes by convention, double quotes work identically. Substitution is
    `$name` or `${expression}`.
  naming: lowerCamelCase for functions and variables, UpperCamelCase for classes, snake_case for filenames
  import_keyword: import

tooling:
  package_manager: pub
  registry: pub.dev
  runtime: compiles ahead of time to a native app, or to JavaScript for the web
  install_command: dart pub add <package-name>
  run_command: flutter run
  test_command: flutter test

confusable_with:
  - language: typescript
    settle_it: >
      Both annotate types. Dart writes the type before the name (`String name`),
      TypeScript writes it after a colon (`name: string`). Dart files sit beside a
      `pubspec.yaml`, TypeScript files beside a `tsconfig.json`.
    tiebreak: { pattern: ':\s*(string|number|boolean)\b', kind: regex, favors: typescript }
  - language: java
    settle_it: >
      Both put the type first and use `@override`. Java capitalizes it as
      `@Override`, opens files with `package com.example;`, and prints with
      `System.out.println`. Dart uses lowercase `@override` and `print()`.
    tiebreak: { pattern: 'System\.out\.println', kind: regex, favors: java }
  - language: javascript
    settle_it: >
      Both use braces, semicolons, and `=>`. Dart's `=>` shortens a named function
      that already has a return type in front of it. JavaScript's `=>` creates an
      anonymous function with no type anywhere.
    tiebreak: { pattern: '\bfunction\b', kind: regex, favors: javascript }

errors_look_like:
  sample: |
    lib/main.dart:14:7: Error: The argument type 'String' can't be assigned to
    the parameter type 'int'.
      addToCart(item, "2");
          ^
  recognize_by: >
    A path ending in `.dart` with a line and column, then the word `Error:` spelled
    out rather than a numbered code. Rust numbers its errors as `error[E0382]` and
    TypeScript as `TS2345`, so a Dart error is recognizable by the absence of a
    code.
  patterns:
    - '\.dart:\d+:\d+'
    - "can't be assigned to the parameter type"
    - 'package:flutter/'

meet_it_when: >
  You are looking at a phone app built with Flutter, which is the only thing most
  people use Dart for. An agent picks it when you ask for an app that runs on both
  Android and iPhone from one codebase.

what_agents_get_wrong: >
  Deeply nested widget trees that work and are unreadable, because Flutter's layout
  is expressed by nesting rather than by styling. Also watch for packages: pub.dev
  moves quickly and an agent will confidently name a package version that does not
  exist, which surfaces as a `pub get` failure rather than as broken code.

see_also:
  - javascript
  - typescript
  - java
  - yaml
  - j1-how-to-recognize-a-language

keywords: [flutter, pub, pubspec, widget, mobile app, cross-platform]
---

Dart is a compiled language with one real use: Flutter, Google's framework for
building a phone app that runs on Android and iPhone from a single codebase.

The shape is close to Java and to TypeScript. Braces for blocks, semicolons at the
end of every statement, `//` for comments. The type goes before the name, which is
the fastest way to separate it from TypeScript, where the type goes after a colon.

```dart
class Cart {
  final String owner;
  late int total;

  Cart({required this.owner});

  int add(int price) => total += price;
}
```

Two things in that snippet exist nowhere else in this deck: `required this.owner`
inside the parameter braces, and `late`. Either one is enough on its own.

The folder settles it faster than the code does. A `pubspec.yaml` at the project
root means Dart, and the entry point is `lib/main.dart`. If you find a `build/` or
`.dart_tool/` folder beside it, both are generated output and both are safe to
delete, because the next build recreates them.
