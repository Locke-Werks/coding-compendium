---
id: objective-c
title: Objective-C
type: language
verified: 2026-08-02
volatility: low

name: Objective-C
aka: [objc, obj-c, objective c, cocoa]
family: compiled
likelihood: unlikely
extensions: ['.m', '.h', '.mm']

tells:
  - pattern: '@end'
    kind: regex
    weight: 10
    note: >
      Every class body closes with `@end` on a line of its own. Nothing else in
      the deck does this. Swift closes with a plain brace.
  - pattern: '@interface'
    kind: regex
    weight: 9
    note: >
      `@interface Foo : NSObject` declares a class. Java also writes
      `@interface`, for annotation types, but Java never follows it with a colon
      and a base class and never pairs it with `@end`.
  - pattern: '\[\w+ \w+'
    kind: regex
    weight: 8
    note: >
      Method calls sit inside square brackets, as `[button setTitle:@"Go"]`.
      Swift writes `button.setTitle("Go")`. Square brackets around a call is the
      loudest tell here.
  - pattern: '@"'
    kind: sigil
    weight: 9
    note: >
      Strings carry a leading at-sign, `@"hello"`. A bare `"hello"` in the same
      file is a plain C string and a different type. Swift and C# write neither
      prefix.
  - pattern: '#import'
    kind: line_start
    weight: 9
    note: >
      Objective-C includes headers with `#import "Foo.h"`. C and C++ write
      `#include`. Swift writes `import` with no hash and no quotes.

rules_out:
  - pattern: 'func'
    kind: token
    because: "Swift, which replaced this language on the same platforms"
  - pattern: 'std::'
    kind: operator
    because: C++
  - pattern: 'System\.out\.println'
    kind: regex
    because: Java

project_fingerprint:
  manifests:
    - file: '*.xcodeproj'
      decisive: false
      note: >
        An Xcode project. It means an Apple platform without naming the language,
        because Swift projects look identical from outside. The file extensions
        inside settle it: `.m` and `.h` mean Objective-C, `.swift` means Swift.
    - file: Podfile
      decisive: false
      note: >
        CocoaPods, the dependency manager most Objective-C projects used. Its
        presence points at an older project, whatever language it holds.
  build_dirs: ['build/', 'DerivedData/', 'Pods/']
  entry_points: ['main.m', 'AppDelegate.m']

shape:
  blocks: braces
  statement_end: semicolon
  comment_line: '//'
  comment_block: '/* */'
  string_quotes: 'A leading at-sign then double quotes, `@"text"`. Bare double quotes are a C string'
  naming: camelCase for methods, PascalCase with a two-letter prefix for classes
  import_keyword: '#import'

tooling:
  package_manager: CocoaPods or Swift Package Manager
  runtime: 'the Objective-C runtime, present on every Apple system'
  run_command: 'xcodebuild, or the run button in Xcode'

confusable_with:
  - language: swift
    settle_it: >
      They live in the same projects and call each other. Objective-C uses square
      brackets for calls, `@"` for strings, and `@end` to close a class. Swift
      uses dots, plain quotes, and a closing brace.
    tiebreak: { pattern: '@end', kind: regex, favors: objective-c }
  - language: cpp
    settle_it: >
      Both mix C with objects and both use `.h` headers. Objective-C writes
      `#import` and square-bracket calls. C++ writes `#include` and `std::`, and
      never uses an at-sign.
    tiebreak: { pattern: 'std::', kind: operator, favors: cpp }

errors_look_like:
  sample: |
    *** Terminating app due to uncaught exception 'NSInvalidArgumentException',
    reason: '-[__NSCFString count]: unrecognized selector sent to instance 0x600000'
  recognize_by: >
    Three stars at the start of the line, class names beginning with a capital
    `N` and `S`, and the phrase `unrecognized selector sent to instance`, which
    means a method was called on an object that has no such method. Swift crashes
    say `Fatal error:` instead.
  patterns:
    - 'unrecognized selector sent to instance'
    - '\*\*\* Terminating app due to uncaught exception'
    - "'NS\\w+Exception'"

meet_it_when: >
  Reading an older iPhone or Mac codebase, or reading a crash report from a
  library that has not been rewritten yet. You will not be asked to start
  something new in it.

what_agents_get_wrong: >
  Memory management is the trap. Code written before 2011 calls `retain`,
  `release`, and `autorelease` by hand, and every project since then compiles
  with automatic reference counting turned on, where those calls are a build
  error. An agent trained on old tutorials writes them anyway. The second thing
  to check is nullability and headers: an agent will happily invent a method that
  does not exist on a class, and because a message sent to an object that cannot
  answer it compiles cleanly, you find out when the app dies with `unrecognized
  selector`. Ask for the header file that declares any method you do not
  recognize.

see_also:
  - swift
  - c
  - cpp
  - j1-how-to-recognize-a-language

keywords: [objc, cocoa, xcode, nsstring, selector, arc, ios legacy]
---

The language Apple used before Swift. It is C with a message-passing object system bolted
on, and it looks like nothing else you will meet.

```objectivec
#import "User.h"

@implementation User

- (NSString *)greet {
    NSString *name = self.name ?: @"stranger";
    return [NSString stringWithFormat:@"hello %@", name];
}

@end
```

Four tells in nine lines. `#import` instead of `#include`. Square brackets around the
call. An at-sign in front of every string. And `@end` closing the block, which no other
language in this deck writes.

The `NS` prefix on class names is a fossil: it stands for NeXTSTEP, the operating system
Apple bought in 1997 and built everything since on top of.

If the folder holds an `.xcodeproj` and the files end in `.m` and `.h`, this is what you
have. If they end in `.swift`, see the Swift card instead.
