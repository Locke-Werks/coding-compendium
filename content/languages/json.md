---
id: json
title: JSON
type: language
verified: 2026-08-02
volatility: low

name: JSON
aka: [javascript object notation, jsonc, json5, package.json]
family: config
likelihood: certain
extensions: ['.json', '.jsonc', '.json5', '.geojson']

# Every note contrasts against the format standing next to it in a real project
# folder. "JSON quotes its keys" is trivia. "JSON quotes its keys, YAML does not,
# TOML uses an equals sign" is the thing that lets a reader tell three files apart.
tells:
  - pattern: '"[^"\n]+"\s*:'
    kind: regex
    weight: 9
    note: >
      A key wrapped in double quotes and followed by a colon. Only JSON quotes
      its keys. YAML writes `name:` bare, TOML writes `name = "x"`, INI writes
      `name=x`.
  - pattern: '^\s*\{'
    kind: regex
    weight: 4
    note: >
      The file opens with a curly brace, because the whole document is one value.
      A YAML file opens with a bare key and a TOML file opens with a `[section]`
      heading.
  - pattern: ',\s*\n'
    kind: regex
    weight: 5
    note: >
      Entries are separated by a comma at the end of the line. YAML separates
      with the newline alone, and TOML never puts a comma between two keys.
  - pattern: 'null'
    kind: token
    weight: 3
    note: >
      The empty value is spelled `null` in lowercase. Python writes `None`, YAML
      also accepts `~` or nothing at all, and TOML has no empty value.
  - pattern: '\[\s*\{'
    kind: regex
    weight: 4
    note: >
      A list of objects opens with `[{`. TOML writes that same idea as repeated
      `[[table]]` headings, and YAML writes a dash at the start of each entry.
  - pattern: '"\$schema"'
    kind: regex
    weight: 6
    note: >
      A `"$schema"` key at the top of a config file is a JSON habit. YAML and
      TOML files point at their schema from a comment, since they are allowed
      comments.

rules_out:
  - pattern: '#'
    kind: line_start
    because: YAML, TOML, or INI. JSON has no comment syntax, so a `#` line is one of those.
  - pattern: '//'
    kind: line_start
    because: JSONC or JavaScript. Strict JSON rejects `//`, even where an editor accepts it.
  - pattern: '='
    kind: operator
    because: TOML or INI, which both assign with an equals sign
  - pattern: '---'
    kind: line_start
    because: YAML, where three dashes open or separate a document
  - pattern: "'"
    kind: sigil
    because: YAML, Python, or JavaScript. JSON has no single-quoted string.

project_fingerprint:
  manifests:
    - file: package.json
      decisive: false
      note: >
        The manifest at the root of every Node.js project. Names the project,
        lists its dependencies, and holds the `scripts` you run with `npm run`.
        Not decisive, because it says nothing about whether the code is
        JavaScript or TypeScript.
    - file: tsconfig.json
      note: >
        TypeScript compiler settings. Real ones frequently carry `//` comments,
        which is legal only because the TypeScript tooling parses JSONC.
    - file: .eslintrc.json
      note: Linter rules for a JavaScript or TypeScript project.
    - file: settings.json
      note: >
        Editor and agent settings. Lives under `.vscode\` inside a project, or
        `C:\Users\<yourname>\.claude\` for Claude Code.
    - file: composer.json
      note: The manifest for a PHP project. Same shape, different ecosystem.
    - file: '*.json'
      decisive: true
      note: The extension never lies. A file ending `.json` is JSON.
  lockfiles: [package-lock.json]

shape:
  blocks: braces
  statement_end: none
  comment_line: 'none, JSON has no comment syntax of any kind'
  string_quotes: >
    Double quotes only, on keys and on string values alike. A single quote is a
    syntax error.
  naming: >
    camelCase keys in JavaScript projects, snake_case in Python ones. The format
    itself has no opinion.
  import_keyword: 'none, a JSON file cannot reference another file'

confusable_with:
  - language: yaml
    settle_it: >
      JSON wraps the document in braces and puts double quotes around every key.
      YAML has neither: `name: my-app` sits alone on its line. Every JSON file is
      also valid YAML, so the confusion only runs one direction.
    tiebreak: { pattern: '"[^"\n]+"\s*:', kind: regex, favors: json }
  - language: toml
    settle_it: >
      TOML assigns with an equals sign under a `[section]` heading. JSON assigns
      with a colon inside braces. One equals sign anywhere means it is not JSON.
    tiebreak: { pattern: '=', kind: operator, favors: toml }
  - language: javascript
    settle_it: >
      A JavaScript object literal looks nearly identical, but it leaves keys
      unquoted, allows single quotes, and sits to the right of something like
      `const config =`. A JSON file is the value alone with no code around it.
    tiebreak: { pattern: 'const |=>|function ', kind: regex, favors: javascript }
  - language: css
    settle_it: >
      Both are braces full of pairs. JSON quotes every single key, separates pairs
      with commas, and allows no comments. CSS quotes no keys, ends each pair with a
      semicolon, and allows `/* */`. One quoted key settles it for JSON.
    tiebreak: { pattern: '\b\d+(px|rem|em|vh|vw)\b', kind: regex, favors: css }

errors_look_like:
  sample: |
    SyntaxError: Unexpected token } in JSON at position 47
        at JSON.parse (<anonymous>)
        at Object.<anonymous> (C:\Users\<yourname>\projects\app\index.js:4:20)
  recognize_by: >
    The phrase "in JSON at position" followed by a number. The number counts
    characters from the start of the file rather than lines, which is why it
    feels useless. The character it names is where the parser gave up, and the
    actual mistake is almost always just before it.
  patterns:
    - 'in JSON at position \d+'
    - 'JSON\.parse'
    - 'Unexpected token .* in JSON'
    - 'Expected .* in JSON at position'
    - 'Unexpected end of JSON input'

meet_it_when: >
  It is the first file an agent asks you to edit. `package.json` sits at the root
  of every web project, `settings.json` holds your editor and agent
  configuration, and nearly every web service answers in JSON, so it is also what
  you paste when a request comes back with the wrong data.

what_agents_get_wrong: >
  Three failures, in the order you will hit them. Agents write comments into
  JSON, because they are explaining the change they just made, and the file stops
  parsing. They append an entry to a list and leave the previous comma behind,
  which is the most common JSON error there is. Worst of the three, because
  nothing complains: they invent configuration keys. A made-up key in
  `settings.json` or `tsconfig.json` is still valid JSON, so the file loads, the
  tool ignores the key it does not recognize, and the setting you asked for never
  takes effect. In a diff, look for a comma sitting before a closing `}` or `]`,
  any `//`, and any key you cannot find in the tool's own documentation.

version_landscape: >
  JSON was frozen in 2013 and has not changed since, so an answer you find from
  2015 is still correct. What moves is the dialects. JSONC allows `//` comments
  and trailing commas and is what Visual Studio Code actually parses. JSON5 goes
  further and allows unquoted keys. Neither one is JSON, and a strict parser
  rejects both.

see_also:
  - yaml
  - toml
  - javascript
  - j2-the-config-formats-nobody-explains
  - b9-where-settings-live
  - g3-lockfiles

keywords: [package.json, tsconfig, trailing comma, JSON.parse, jsonc, json5, settings.json, unexpected token]
---

JSON (JavaScript Object Notation) is a text format for storing data. It is not a
programming language. It has no logic, no functions, no loops, and nothing inside a
`.json` file ever runs. It describes data, and some other program reads it.

Two structures make up the entire format. An object, written `{ }`, holds named entries.
An array, written `[ ]`, holds an ordered list. Everything else is a string, a number,
`true`, `false`, or `null`.

## The shape

Every key is wrapped in double quotes and followed by a colon. Entries are separated by
commas. The whole file is a single value, usually an object, so it opens with `{` and
closes with `}`.

```json
{
  "name": "my-app",
  "version": "1.0.0",
  "private": true,
  "scripts": {
    "dev": "vite",
    "build": "vite build"
  },
  "keywords": ["cli", "windows"]
}
```

Here is the same data in the two formats it sits beside on disk. YAML (YAML Ain't Markup
Language) drops every brace, comma, and quote mark and uses indentation instead.

```yaml
name: my-app
version: "1.0.0"
private: true
```

TOML (Tom's Obvious, Minimal Language) writes `key = value` with an equals sign.

```toml
name = "my-app"
version = "1.0.0"
private = true
```

A quoted key and a colon is JSON. A bare key and a colon is YAML. An equals sign is TOML
or INI (Initialization). That one test settles nearly every config file you will open.

Numbers carry no quotes. Strings always do. `"1.0.0"` is a string, because a version
number is not a number, and writing `1.0.0` unquoted is a syntax error.

## What it is for

Configuration files, and data moving between programs. `package.json` describes a Node.js
project. `tsconfig.json` configures the TypeScript compiler. `settings.json` holds the
settings for Visual Studio Code and for Claude Code. Nearly every web API (Application
Programming Interface) answers in JSON, so a response you paste into an agent is usually
this.

## The gotchas

Three, and between them they cause almost every JSON error you will ever see.

**No comments. None, anywhere, ever.** There is no `#`, no `//`, no `/* */`. If you want
to explain a key, the accepted workaround is to add a key called `"_comment"` and hope the
next person reads it. This is the format's most complained-about property and fifteen years
of complaining has not moved it.

**No trailing comma.** The last entry before a `}` or a `]` gets no comma after it. Adding
a dependency to the end of a list and leaving the old comma behind is the single most
common JSON mistake in existence.

```json
{
  "a": 1,
  "b": 2,
}
```

That file is broken. Delete the comma after `2`.

**Keys must be double-quoted.** `{name: "x"}` is a perfectly good JavaScript object and an
invalid JSON file. Single quotes fail the same way: `'x'` is fine in Python and JavaScript
and illegal here.

One exception is worth knowing, because it is where the myth comes from. `settings.json`
in Visual Studio Code accepts `//` comments and trailing commas, because the editor parses
JSONC (JSON with Comments) rather than JSON. That file is the reason people believe
comments are allowed. They are not, anywhere else.

## Reading its errors

```text
SyntaxError: Unexpected token } in JSON at position 47
```

The position counts characters from the start of the file, not lines. Most editors will
jump to a character offset if you ask. The character it names is where the parser gave up,
and the real mistake is nearly always just before it: a comma with nothing following it, or
a string that was never closed.

Newer versions of Node.js print a kinder version, such as `Expected double-quoted property
name in JSON at position 47`. That one is telling you a key is missing its quotes.

If a JSON file will not parse and you cannot see why, paste the whole file to an agent and
ask it to find the syntax error. Counting brackets is a task machines do better than people.
