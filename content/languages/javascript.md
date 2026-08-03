---
id: javascript
title: JavaScript
type: language
verified: 2026-08-02
volatility: low

name: JavaScript
aka: [js, ecmascript, node, nodejs, vanilla js]
family: interpreted
likelihood: certain
extensions: ['.js', '.mjs', '.cjs', '.jsx']

# TypeScript is JavaScript plus type annotations, so nearly every token below is
# also a TypeScript token. That is why nothing on this card scores above 8: the
# high weights belong to the two tells TypeScript source almost never uses,
# `module.exports` and JSDoc types. Everything else identifies the family, and
# the file extension or a tsconfig.json settles which member of it.
tells:
  - pattern: '=>'
    kind: operator
    weight: 4
    note: >
      JavaScript's arrow function: `(a, b) => a + b` is a whole function. Rust uses
      `->` before a return type, PHP uses `->` for member access, and C# uses `=>`
      for an expression-bodied method. Three different meanings, two near-identical
      shapes. TypeScript uses `=>` the same way, so this narrows to the family and
      no further.
  - pattern: '\$\{[^}]*\}'
    kind: regex
    weight: 6
    note: >
      A dollar sign and braces inside backticks is a template literal:
      `` `Total: ${sum}` ``. Python writes `f"{sum}"`, C# writes `$"{sum}"`, Ruby
      writes `#{sum}`. The backtick is the part nobody else uses.
  - pattern: '==='
    kind: operator
    weight: 6
    note: >
      Three equals signs test value and type together. Only PHP shares it. Python,
      Java, Rust, Go, and C# stop at `==` and have no third form.
  - pattern: '\b(const|let)\s+\w+\s*='
    kind: regex
    weight: 4
    note: >
      `const` and `let` in the same file is the modern JavaScript pair. Rust writes
      `let` and `let mut`, Go writes `:=`, Python writes the name with no keyword
      at all. TypeScript writes the same pair, usually with a type after the name.
  - pattern: 'module\.exports'
    kind: regex
    weight: 8
    note: >
      Node's older export form. TypeScript source almost always writes
      `export default` instead, so this points at plain JavaScript rather than the
      family. Python has no equivalent, Rust marks the item `pub` in place.
  - pattern: '@param\s*\{'
    kind: regex
    weight: 7
    note: >
      A type in braces inside a comment, as in `@param {string} name`, is JSDoc.
      It exists because the file is JavaScript and cannot write the type in the
      code. TypeScript puts the same type on the parameter itself, so a JSDoc type
      block is evidence against TypeScript.
  - pattern: '\brequire\s*\('
    kind: regex
    weight: 6
    note: >
      Node's older import: `const fs = require("fs")`. Ruby has `require` too and
      writes it without parentheses and without assigning it, as `require 'json'`.
      Python uses `import`, Rust uses `use`.
  - pattern: 'undefined'
    kind: token
    weight: 5
    note: >
      JavaScript has two different kinds of nothing, `null` and `undefined`, and
      uses both. Python has only `None`, Java and C# have only `null`, Rust refuses
      to have either and uses `Option` instead.

rules_out:
  - pattern: '\binterface\b'
    kind: regex
    because: TypeScript, Java, Go, or C#. JavaScript has no `interface` keyword at all.
  - pattern: ':\s*(string|number|boolean)\b'
    kind: regex
    because: TypeScript. A type after a colon is a syntax error in a .js file.
  - pattern: '<\?php'
    kind: regex
    because: PHP, which also has `===` and `->` and is easy to mistake for this at a glance.
  - pattern: 'def'
    because: Python or Ruby
  - pattern: 'fn'
    because: Rust
  - pattern: 'func'
    because: Go or Swift
  - pattern: '#include'
    kind: line_start
    because: C or C++
  - pattern: 'System\.out\.println'
    kind: regex
    because: Java

project_fingerprint:
  manifests:
    - file: package.json
      decisive: false
      note: >
        Marks a Node or browser project and names its dependencies. It does NOT
        settle JavaScript against TypeScript, because TypeScript projects carry an
        identical one. This pair is the reason `decisive` exists as a field.
    - file: jsconfig.json
      decisive: true
      note: >
        The JavaScript counterpart of tsconfig.json. Present only when the project
        has deliberately chosen plain JavaScript, so it settles the question the
        way package.json cannot.
    - file: tsconfig.json
      decisive: false
      note: >
        Its presence means the project is TypeScript, so on this card it is
        negative evidence. A folder with package.json and no tsconfig.json is
        JavaScript.
    - file: vite.config.js
      decisive: false
      note: >
        The build tool's own config. The extension is the tell: `.js` here,
        `vite.config.ts` in a TypeScript project.
  lockfiles: [pnpm-lock.yaml, package-lock.json, yarn.lock, bun.lockb]
  build_dirs: [node_modules/, dist/, build/, .next/, .vite/]
  entry_points: [index.js, src/index.js, src/main.js, server.js]

shape:
  blocks: braces
  statement_end: optional_semicolon
  comment_line: '//'
  comment_block: '/* */'
  string_quotes: >
    Single and double quotes are identical in meaning. Backticks are a third kind
    that allows `${...}` substitution and real line breaks.
  naming: camelCase for functions and variables, PascalCase for classes and React components, SCREAMING_SNAKE_CASE for constants
  import_keyword: import (older code uses require)

tooling:
  package_manager: npm, pnpm, or yarn
  registry: npmjs.com
  runtime: Node.js on your machine, the browser on a web page
  install_command: pnpm add <package-name>
  run_command: node index.js
  test_command: pnpm test

confusable_with:
  - language: typescript
    settle_it: >
      Look for a type after a colon: `function greet(name: string)`. If it is
      there, the file is TypeScript, because that line will not run as JavaScript.
      If it is absent, check the extension: `.js` is JavaScript, `.ts` is
      TypeScript. Almost every other token is shared.
    tiebreak: { pattern: ':\s*(string|number|boolean|any|unknown)\b', kind: regex, favors: typescript }
  - language: json
    settle_it: >
      A JavaScript object and a JSON file look nearly identical. JSON quotes every
      key and allows no comments, no trailing comma, and no function. If the braces
      contain `//`, a bare key, or a `=>`, it is JavaScript.
    tiebreak: { pattern: '(=>|function|//|const )', kind: regex, favors: javascript }
  - language: php
    settle_it: >
      Both use `===`, braces, and `function`. PHP puts a dollar sign on every
      variable (`$total`) and opens the file with `<?php`. JavaScript uses a dollar
      sign only inside a template literal.
    tiebreak: { pattern: '<\?php', kind: regex, favors: php }
  - language: dart
    settle_it: >
      Both use braces, semicolons, and `=>`. Dart puts the type before the name
      (`String name = 'nyx'`) where JavaScript puts no type at all, and Dart files
      end in `.dart` beside a `pubspec.yaml`.
    tiebreak: { pattern: '\brequired\s+this\.', kind: regex, favors: dart }
  - language: css
    settle_it: >
      A CSS rule looks like a JavaScript object: braces full of `key: value`.
      JavaScript separates the pairs with commas and quotes its string values. CSS
      ends every pair with a semicolon and quotes almost nothing. A brace opening
      after a bare word, a dot, or a hash is CSS.
    tiebreak: { pattern: '^\s*[.#][\w-]+\s*\{', kind: regex, favors: css }
  - language: html
    settle_it: >
      A React file mixes tags into JavaScript and looks like HTML from a distance.
      It writes `className=` instead of `class=`, wraps values in braces
      (`href={url}`), and has `import` at the top. Real HTML has neither.
    tiebreak: { pattern: '<!DOCTYPE html', kind: regex, favors: html }
  - language: python
    settle_it: >
      Both are dynamically typed and both are everywhere. JavaScript wraps every
      block in curly braces and declares with `const`, `let`, or `function`. Python
      has no declaration keyword at all, opens blocks with a colon, and closes them
      by outdenting.
    tiebreak: { pattern: 'def \w+\(.*\):', kind: regex, favors: python }

errors_look_like:
  sample: |
    C:\Users\nyx\shop\server.js:12
      const total = cart.items.length;
                         ^

    TypeError: Cannot read properties of undefined (reading 'items')
        at checkout (C:\Users\nyx\shop\server.js:12:22)
        at Object.<anonymous> (C:\Users\nyx\shop\index.js:4:1)
        at Module._compile (node:internal/modules/cjs/loader:1356:14)
  recognize_by: >
    Lines beginning with `at `, each ending in `file:line:column`, stacked newest
    first. Anything mentioning `node:internal/` is Node. In the browser console the
    same trace appears in red with the file name on the right-hand side. Python
    prints its stack oldest first and labels it `Traceback`, which is the fastest
    way to tell the two apart.
  patterns:
    - '^\s+at\s+.*:\d+:\d+'
    - 'Cannot read propert(y|ies) of (undefined|null)'
    - '\b(ReferenceError|TypeError): \w+ is not (defined|a function)'
    - 'node:internal/'
    - 'Uncaught \(in promise\)'

meet_it_when: >
  Every web page you have ever opened runs some. An agent reaches for it for
  anything with a browser in it, any quick script, and most servers. It is also
  what the config files ending in `.js` are written in, and what the build tools
  themselves are written in.

what_agents_get_wrong: >
  Three things, in order of how often they appear. Old idioms: `var` instead of
  `const` and `let`, and jQuery calls like `$("#cart")` for work the browser has
  done natively since 2015. It runs. It also tells you the agent pulled from very
  old material, so read the rest of that file harder. Invented packages: a new
  line in package.json naming something that does not exist, or that somebody
  registered last month after noticing agents ask for it by name. Check every
  added dependency against npmjs.com before you install. A missing `await`: the
  code reads correctly, the variable holds a pending Promise instead of the value,
  and you get `undefined` or `[object Promise]` at runtime with no error anywhere
  near the mistake.

version_landscape: >
  The 2015 release split the language in two. Before it: `var`, callbacks nested
  inside callbacks, `require`. After it: `const` and `let`, arrow functions,
  `async`/`await`, `import`. Old code still runs, which is the trap, because an
  answer you find from 2013 works and looks nothing like the code beside it. If a
  suggestion uses `var` or jQuery, it is old.

see_also:
  - typescript
  - json
  - html
  - css
  - f2-stack-traces
  - g2-package-managers
  - j1-how-to-recognize-a-language

keywords: [node, npm, pnpm, es6, esm, commonjs, ecmascript, vanilla, browser console]
---

The language every web browser runs, and the one most build tooling is written in.

It has nothing to do with Java. The name was a marketing decision in 1995 and it
has been confusing people ever since.

## The shape

Blocks use curly braces. Semicolons at the end of a statement are optional, which
should worry you slightly: the interpreter inserts them for you and is right almost
every time. Indentation means nothing to the interpreter, unlike Python where it
means everything.

```javascript
const total = 3;                 // cannot be reassigned
let count = 0;                   // can be reassigned
var old = 1;                     // the pre-2015 form, see below

const add = (a, b) => a + b;     // arrow function, one expression
function subtract(a, b) {        // the older, longer form
  return a - b;
}
```

Comments are `//` for a line and `/* */` for a block. There is no third kind.

## Six lines of it

```javascript
const items = [
  { name: "candle", price: 12 },
  { name: "ink", price: 4 },
];
const total = items.reduce((sum, item) => sum + item.price, 0);
console.log(`Total: ${total}`);
```

## Why TypeScript is always standing next to it

Valid JavaScript is usually valid TypeScript. TypeScript is this language with type
annotations bolted on, and it strips them back off before anything runs. The
practical consequence catches people out: nearly every tell on this card is also a
TypeScript tell, so an honest identifier will sometimes tell you "JavaScript or
TypeScript" rather than pick.

Three things settle it, in order of how quickly you can check them.

1. The extension. `.js`, `.mjs`, and `.cjs` are JavaScript. `.ts` and `.tsx` are
   TypeScript.
2. A `tsconfig.json` in the project root. Nothing but the TypeScript compiler reads
   that file, so its presence is decisive and `package.json` never is, because both
   languages ship one.
3. A type in the code: `function greet(name: string)`. Save that line in a `.js`
   file and it will not run.

## What it is for

Anything in a browser: buttons that react, forms that check themselves, pages that
change without reloading. On the server it runs under Node.js, which is the same
language with file and network access added. It also drives most desktop app
shells and nearly every build tool you will meet.

It needs a runtime present to run at all, either the browser or Node.js. There is
no `.exe` at the end of a JavaScript project, which is the main practical
difference from Rust or Go.

## The three web languages, and which does what

HTML (HyperText Markup Language) is the structure of a page. CSS (Cascading Style
Sheets) is how that structure looks. JavaScript is what happens when you click
something. All three usually sit in the same folder and are frequently edited in
the same change, so an agent saying "I updated the front end" may have touched all
three.

JavaScript reaches the page through the DOM (Document Object Model), which is the
browser's live in-memory version of the HTML. `document.querySelector("#cart")`
finds an element in it; changing that element changes what is on screen
immediately.

## Reading its errors

Two places to look, and they are different places. Code run with `node` prints its
error in the terminal you started it from. Code running in a page prints to the
browser console, which you open with F12 and the Console tab. A page that looks
broken with a silent terminal means you are looking in the wrong one.

Read the stack from the top. JavaScript prints the newest call first, so the first
`at` line is where it actually broke and the lines below it are how it got there.
Find the first path inside your own project and start there.
