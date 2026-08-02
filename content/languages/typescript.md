---
id: typescript
title: TypeScript
type: language
verified: 2026-08-02
volatility: low

name: TypeScript
aka: [ts, tsx, typescriptlang]
family: compiled
likelihood: certain
extensions: ['.ts', '.tsx', '.mts', '.cts', '.d.ts']

# TypeScript is JavaScript plus type annotations, so the JavaScript card cannot
# claim the tokens both share and this card must not repeat them. Everything
# below is syntax that a .js file cannot contain: put any of it in a .js file and
# the file stops running. That is what makes these weights honest.
tells:
  - pattern: ':\s*(string|number|boolean|void|any|unknown|never)\b'
    kind: regex
    weight: 9
    note: >
      A lowercase type after a colon, as in `function greet(name: string)`. This is
      the single most reliable tell on the card. Python annotates the same way and
      spells the types `str`, `int`, `bool`. Kotlin and Swift capitalize theirs as
      `String` and `Int`. Only TypeScript writes them lowercase in full.
  - pattern: 'interface'
    kind: token
    weight: 6
    note: >
      Declares a shape that objects must match. JavaScript has no such keyword, so
      this settles the pair on its own. Java, C#, Go, and PHP also have `interface`,
      and none of them put `=>` and `const` in the same file.
  - pattern: '^\s*(export\s+)?type\s+\w+\s*='
    kind: regex
    weight: 8
    note: >
      A type alias: `type UserId = string`. Rust has `type` aliases too and writes
      `fn` and `let mut` beside them; Go writes `type User struct` with no equals
      sign. JavaScript has nothing like it.
  - pattern: '\w+\?\s*:'
    kind: regex
    weight: 8
    note: >
      A question mark before the colon means the field is optional:
      `email?: string`. Do not confuse it with the ternary `ready ? "yes" : "no"`,
      which is ordinary JavaScript and reads left-to-right. Kotlin and Swift mark
      optional with a `?` after the type instead, as `String?`.
  - pattern: '\bas\s+(const|[A-Z]\w*)'
    kind: regex
    weight: 7
    note: >
      A type assertion: `input as HTMLInputElement` tells the compiler to stop
      arguing. Rust's `as` converts a number for real at runtime, Python's `as`
      renames an import, and C#'s `as` performs a checked cast. TypeScript's does
      nothing at runtime whatsoever.
  - pattern: 'enum'
    kind: token
    weight: 5
    note: >
      A named set of fixed values. JavaScript has no `enum`, so this settles the
      pair, but Java, C#, Rust, and Swift all have one, so it settles nothing else.
  - pattern: 'function\s+\w+\s*<[^>]+>\s*\('
    kind: regex
    weight: 7
    note: >
      Angle brackets between the function name and its parameters is a generic:
      `function first<T>(items: T[])`. Rust writes `fn first<T>`, Java and C# put
      `public static` in front. JavaScript cannot express this at all.
  - pattern: 'constructor\s*\(\s*(private|public|protected|readonly)'
    kind: regex
    weight: 9
    note: >
      An access keyword on a constructor parameter declares and assigns the field
      in one move. No other language in this deck has this shortcut, and JavaScript
      classes have no `private` or `readonly` keyword to put there.

rules_out:
  - pattern: 'module\.exports'
    kind: regex
    because: JavaScript using Node's older export form. TypeScript source writes `export` instead.
  - pattern: '@param\s*\{'
    kind: regex
    because: JavaScript with JSDoc types. A TypeScript file puts the type on the parameter, not in a comment.
  - pattern: ':\s*(str|int|bool|float)\b'
    kind: regex
    because: Python type hints, which look almost identical and use the short spellings.
  - pattern: '<\?php'
    kind: regex
    because: PHP
  - pattern: 'fn'
    because: Rust
  - pattern: 'func'
    because: Go or Swift
  - pattern: 'System\.out\.println'
    kind: regex
    because: Java
  - pattern: '#include'
    kind: line_start
    because: C or C++

project_fingerprint:
  manifests:
    - file: tsconfig.json
      decisive: true
      note: >
        The TypeScript compiler's settings file. Nothing else on your machine reads
        it, so a tsconfig.json at the project root settles the question by itself.
        This is the decisive one and the reason to look in the folder before
        reading the code.
    - file: package.json
      decisive: false
      note: >
        Every TypeScript project has one and so does every JavaScript project, so
        it never separates the two. It does tell you which package manager to use:
        a pnpm-lock.yaml beside it means run `pnpm`, not `npm`.
    - file: vite.config.ts
      decisive: false
      note: >
        The build tool's own config written in TypeScript. The extension is the
        whole tell: `vite.config.ts` here, `vite.config.js` in a JavaScript
        project. The same applies to `next.config.ts` and `vitest.config.ts`.
    - file: '*.d.ts'
      decisive: false
      note: >
        A declaration file: types only, no code. Usually generated or shipped by a
        library so that TypeScript can describe JavaScript it did not compile.
  lockfiles: [pnpm-lock.yaml, package-lock.json, yarn.lock, bun.lockb]
  build_dirs: [node_modules/, dist/, build/, .next/, .turbo/, .tsbuildinfo]
  entry_points: [src/index.ts, src/main.ts, src/main.tsx, src/App.tsx]

shape:
  blocks: braces
  statement_end: optional_semicolon
  comment_line: '//'
  comment_block: '/* */'
  string_quotes: >
    Single and double quotes mean the same thing. Backticks allow `${...}`
    substitution and real line breaks, exactly as in JavaScript.
  naming: camelCase for functions and variables, PascalCase for types, interfaces, and React components, SCREAMING_SNAKE_CASE for constants
  import_keyword: import

tooling:
  package_manager: npm, pnpm, or yarn
  registry: npmjs.com
  runtime: Node.js or the browser, after the types are stripped out
  install_command: pnpm add -D typescript
  run_command: pnpm tsc && node dist/index.js
  test_command: pnpm test

confusable_with:
  - language: javascript
    settle_it: >
      Look for a type after a colon: `name: string`. Present means TypeScript,
      because a `.js` file cannot contain it. Absent means check the extension and
      the folder: `.ts` or a tsconfig.json is TypeScript, `.js` with no tsconfig.json
      is JavaScript. Everything else in the file is shared.
    tiebreak: { pattern: '(interface\s+\w+|:\s*(string|number|boolean)\b|\bas\s+const\b)', kind: regex, favors: typescript }
  - language: csharp
    settle_it: >
      Both use `=>`, `interface`, `enum`, and `readonly`. C# capitalizes its types
      (`string` is an alias for `String`), opens with `using System;`, and puts
      `public class` at the top of nearly every file. TypeScript imports with
      `import ... from "..."` and has no `namespace` around ordinary code.
    tiebreak: { pattern: '^\s*using\s+System', kind: regex, favors: csharp }
  - language: dart
    settle_it: >
      Both annotate types and both use `=>`. Dart puts the type before the name
      (`String name`), TypeScript puts it after with a colon (`name: string`). Dart
      files sit beside a `pubspec.yaml`, TypeScript files beside a `tsconfig.json`.
    tiebreak: { pattern: '\brequired\s+this\.', kind: regex, favors: dart }
  - language: rust
    settle_it: >
      Both write the type after a colon. Rust declares functions with `fn`, imports
      with `use`, and uses `::` everywhere. TypeScript writes `function` or `=>`,
      imports with `import ... from`, and never uses `::`.
    tiebreak: { pattern: 'fn\s+\w+\s*\(', kind: regex, favors: rust }

errors_look_like:
  sample: |
    src/cart.ts:12:18 - error TS2345: Argument of type 'string' is not assignable
    to parameter of type 'number'.

    12   addToCart(item, "2");
                          ~~~

    Found 1 error in src/cart.ts:12
  recognize_by: >
    The letters `TS` followed by four digits. No other language in this deck
    numbers its errors that way. Rust numbers its own as `error[E0382]` and puts
    the code in square brackets, so the two are easy to keep apart. The tildes
    underlining the offending value are also a TypeScript habit. At runtime there
    are no TypeScript errors at all, only JavaScript ones, because the types were
    deleted before the code ran.
  patterns:
    - 'error TS\d{4}:'
    - '\bTS\d{4}\b'
    - "is not assignable to (parameter of )?type"
    - '\.tsx?:\d+:\d+ - error'
    - "Object is possibly '(null|undefined)'"

meet_it_when: >
  Any front-end project started in the last several years is probably this rather
  than JavaScript. An agent picks it by default for a web app, a browser extension,
  or a Node service, and it is what the app you are reading this in is written in.

what_agents_get_wrong: >
  Watch for `any`. It is TypeScript's escape hatch and it means "stop checking this
  value", which switches off every check downstream of it as well. An agent that
  cannot work out a type reaches for `any`, the build goes green, and the safety
  you were paying for is gone in that file. Search a diff for `: any`, `as any`,
  and the double assertion `as unknown as`. Three related moves buy a green build
  the same way: a `// @ts-ignore` or `// @ts-expect-error` comment above the
  failing line, a `!` promising a value is not null (`user!.email`), and an edit to
  `tsconfig.json` turning `strict` off. Any change to `tsconfig.json` inside a diff
  that was supposed to be about a feature deserves a question before you merge it.

version_landscape: >
  Version 5 is current and the language has been stable for years, so an answer
  from 2021 usually still applies. Two things date material: `enum` and namespaces
  are now discouraged in favor of plain object literals and modules, and `strict`
  mode is on by default in new projects where older tutorials assume it is off.

see_also:
  - javascript
  - csharp
  - json
  - f1-how-to-read-an-error-message
  - h3-reviewing-a-diff-you-cannot-fully-read
  - g3-lockfiles
  - j1-how-to-recognize-a-language

keywords: [tsc, tsconfig, strict mode, type annotation, generics, any, tsx, superset]
---

JavaScript with type labels added, plus a compiler that reads the labels and
refuses to build when they disagree.

The labels do nothing at runtime. They are deleted on the way out, and what
actually runs is plain JavaScript.

## The one fact that explains everything else

Valid JavaScript is usually valid TypeScript. Rename a working `.js` file to `.ts`
and it will still run. That is deliberate: TypeScript was designed as a layer over
a language that already existed, not as a replacement for it.

Two consequences follow, and both matter when you are trying to work out what you
are looking at.

Nearly every JavaScript tell is also a TypeScript tell. `const`, `=>`, `===`,
backtick strings, `console.log`, `async`/`await`: all shared, all useless for
telling the two apart. An identifier that says "JavaScript or TypeScript" is being
honest rather than lazy.

Only the annotations are unique to TypeScript, so those are what to look for.

## What only TypeScript has

```typescript
interface User {
  name: string;
  email?: string;          // the ? means this one is optional
}

type UserId = string;      // a name for an existing type

function greet(user: User): string {
  return `Hello ${user.name}`;
}

const el = document.getElementById("cart") as HTMLDivElement;
```

Every line above is a syntax error in a `.js` file. The colon-then-type on a
parameter, `interface`, `type X =`, the `?` before a colon, `enum`, `as`, and
angle-bracket generics such as `Array<string>` are the whole list. If you see any
of them, you are looking at TypeScript.

## The shape

Identical to JavaScript. Braces for blocks, optional semicolons, `//` and `/* */`
for comments, camelCase for values and PascalCase for types. The only structural
difference is the annotations, and there is one honest oddity: JavaScript's own
`?:` ternary (`ready ? "yes" : "no"`) looks a lot like an optional field
(`email?: string`) and means something completely unrelated. Position tells them
apart. A ternary sits inside an expression, an optional field sits in a type
declaration.

## Compiled, with an asterisk

This deck files TypeScript as compiled, which is true and needs one qualification.
Rust compiles to a standalone `.exe` you can hand to someone. TypeScript compiles
to JavaScript, which still needs Node.js or a browser to run. The compiler is
`tsc`, and modern build tools often strip the types without checking them at all
for speed, then run `tsc --noEmit` separately as the actual check.

This is why a project can fail its type check and still start up fine in dev mode.
Both facts can be true at once and it confuses everybody the first time.

## Reading its errors

A TypeScript error has a code, `TS2345` and friends, and points at a file, line,
and column. Read the last clause of the message first: it usually names the two
types that failed to line up, and one of them is what you actually wanted.

Nothing on this card can save you at runtime. A `.json` file arriving from the
network with the wrong shape will sail past every type in your project, because
the types were deleted before the program started. TypeScript checks the code you
wrote against itself, and it stops at the edges of your program.
