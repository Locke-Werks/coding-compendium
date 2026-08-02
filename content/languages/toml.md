---
id: toml
title: TOML
type: language
verified: 2026-08-02
volatility: low

name: TOML
aka: [toml, "tom's obvious minimal language", cargo.toml, pyproject]
family: config
likelihood: certain
extensions: ['.toml']

tells:
  - pattern: '^\s*[\w."-]+\s*=\s*'
    kind: regex
    weight: 8
    note: >
      A bare key, an equals sign, then the value. YAML uses a colon and JSON uses
      a quoted key with a colon, so the equals sign is the whole giveaway.
  - pattern: '^\s*\[[\w.-]+\]\s*$'
    kind: regex
    weight: 8
    note: >
      A word inside single square brackets alone on a line is a table heading, as
      in `[dependencies]`. JSON opens a brace here, and YAML just indents.
  - pattern: '^\s*\[\[[\w.-]+\]\]\s*$'
    kind: regex
    weight: 10
    note: >
      Doubled square brackets are TOML's array-of-tables form and appear in no
      other format on this deck. It means "another one of these".
  - pattern: '\{\s*\w+\s*=\s*'
    kind: regex
    weight: 6
    note: >
      Braces on one line are an inline table, as in
      `serde = { version = "1.0" }`. It looks like JSON but keeps the equals
      sign and leaves the key unquoted.
  - pattern: '^\s*#'
    kind: regex
    weight: 2
    note: >
      Comments start with `#`, the same as YAML and INI. JSON has no comments, so
      this rules JSON out but decides nothing between the other three.
  - pattern: '^\s*\w+\s*=\s*\d{4}-\d{2}-\d{2}'
    kind: regex
    weight: 6
    note: >
      TOML has a real date type, so a bare unquoted `2026-08-02` on the right of
      an equals sign is valid. JSON has no dates and would need that in quotes.

rules_out:
  - pattern: ':\s'
    kind: regex
    because: YAML or JSON. TOML separates a key from its value with `=`, never with a colon.
  - pattern: '^\s*-\s'
    kind: regex
    because: YAML, where a leading dash is a list entry. TOML writes lists inside square brackets on one line.
  - pattern: '<'
    kind: sigil
    because: XML or HTML, which nest with angle-bracket tags
  - pattern: '^\s*\{'
    kind: regex
    because: JSON, if the file itself opens with a brace

project_fingerprint:
  manifests:
    - file: Cargo.toml
      decisive: true
      note: >
        The manifest for every Rust project. Names the package, its version, and
        its dependencies. If this file is in the folder root, the project is
        Rust and the file is TOML.
    - file: pyproject.toml
      decisive: true
      note: >
        The modern Python project file. Replaces `setup.py` and holds the
        dependency list, the build backend, and most tool settings.
    - file: config.toml
      note: >
        Codex keeps its settings here, under
        `C:\Users\<yourname>\.codex\config.toml`. Hugo and several other tools
        use the same filename for their own settings.
    - file: rustfmt.toml
      note: Formatting rules for a Rust project. Sits beside `Cargo.toml`.
    - file: netlify.toml
      note: Build and deploy settings for a site hosted on Netlify.
    - file: '*.toml'
      decisive: true
      note: The extension is unambiguous. Nothing else uses it.
  lockfiles: [Cargo.lock, poetry.lock, uv.lock]

shape:
  blocks: none
  statement_end: newline
  comment_line: '#'
  string_quotes: >
    Double quotes for ordinary strings, single quotes for literal strings where
    backslashes stay backslashes. Triple quotes give you a multi-line string.
  naming: >
    lower-case keys joined by dashes or underscores. Rust projects use
    `snake_case`, Python projects use both, and nothing enforces either.
  import_keyword: 'none, though Cargo and uv add their own workspace and include keys'

confusable_with:
  - language: ini
    settle_it: >
      They look nearly identical: `[section]` headings and `key = value` lines.
      TOML quotes its strings and has real types, so `name = "app"` sits next to
      `port = 8080`. INI quotes nothing and treats every value as text.
    tiebreak: { pattern: '^\s*[\w.-]+\s*=\s*"', kind: regex, favors: toml }
  - language: yaml
    settle_it: >
      TOML writes `key = value` and groups with `[section]` headings that
      indentation cannot change. YAML writes `key: value` and the indentation is
      the structure. If moving a line left would change the meaning, it is YAML.
    tiebreak: { pattern: '^\s*[\w.-]+:\s', kind: regex, favors: yaml }
  - language: json
    settle_it: >
      JSON quotes every key and wraps the file in braces. TOML leaves keys bare,
      uses an equals sign, and has no outer braces at all. TOML also allows `#`
      comments, which JSON forbids.
    tiebreak: { pattern: '"[^"\n]+"\s*:', kind: regex, favors: json }

errors_look_like:
  sample: |
    error: failed to parse manifest at `C:\Users\<yourname>\projects\app\Cargo.toml`

    Caused by:
      TOML parse error at line 4, column 8
        |
      4 | version: "1.0.0"
        |        ^
      expected `.`, `=`
  recognize_by: >
    The words "TOML parse error" with a line and column, and a little pointer
    drawn under the offending character. `expected =` is the common one and it
    means you wrote `key: value` out of YAML habit in a file that wants
    `key = value`.
  patterns:
    - 'TOML parse error at line \d+'
    - 'expected `\.`, `=`'
    - 'failed to parse manifest at'
    - 'invalid (TOML )?value'

meet_it_when: >
  You open `Cargo.toml` to add a dependency to a Rust project, you edit
  `pyproject.toml` in a modern Python project, or you change a setting in
  `config.toml` for Codex. It is also what a Rust build error points at when the
  manifest itself is malformed.

what_agents_get_wrong: >
  The `[section]` boundary is the one they miss. An agent appends a new
  `key = value` to the bottom of the file, and because a heading stays in force
  until the next heading, that key lands inside whatever table happened to come
  last. The file parses fine and the setting does nothing. Agents also reach for
  YAML syntax in a TOML file and write `key: value`, which at least fails loudly.
  The quiet failures are the expensive ones: a quoted number where the tool wants
  a real number, `opt-level = "3"` instead of `opt-level = 3`, and invented keys
  in `Cargo.toml` that Cargo warns about once in a wall of output and then
  ignores. In a diff, check which heading each added line falls under, and check
  whether new values are quoted when they should not be.

version_landscape: >
  TOML reached 1.0 in 2021 and is stable, so old answers still apply. The one
  historical trap is Python: `pyproject.toml` grew new sections over several
  years, so a tutorial from 2020 may put dependencies somewhere the current tools
  no longer read.

see_also:
  - json
  - yaml
  - ini
  - rust
  - j2-the-config-formats-nobody-explains
  - b9-where-settings-live
  - g2-package-managers

keywords: [cargo.toml, pyproject.toml, config.toml, array of tables, table heading, inline table, tom preston-werner]
---

TOML (Tom's Obvious, Minimal Language) is a text format for configuration files. It is not
a programming language. It has no logic, no functions, and nothing inside a `.toml` file
ever runs. It describes settings, and some other program reads them and behaves
accordingly.

The T really is a person's first name. The format was designed so you can read one without
having read the specification first, which is the whole argument for it.

## The shape

`key = value`, one per line, with an equals sign. Related keys are grouped under a heading
in square brackets, which the specification calls a table and everyone else calls a
section.

```toml
[package]
name = "my-app"
version = "1.0.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = "1"

[profile.release]
opt-level = 3
```

The contrast: JSON (JavaScript Object Notation) quotes every key and wraps the file in
braces. YAML (YAML Ain't Markup Language) writes `key: value` and nests by indenting. TOML
uses an equals sign, and its indentation means nothing at all. An equals sign narrows you
to TOML or INI (Initialization), and quoted strings beside unquoted numbers settle it as
TOML.

Strings need quotes. Numbers, `true`, and `false` do not. Comments start with `#`.

The heading is a path, not a container. `[profile.release]` means "inside `profile`, inside
`release`". Nothing closes a heading except the next heading: every `key = value` line
after `[dependencies]` belongs to `dependencies` until another `[` line appears.

## The array-of-tables form

This is the part that confuses everyone, so it gets its own section.

Doubled square brackets, `[[like this]]`, mean "here is another one". Each block adds one
more entry to a list.

```toml
[[bin]]
name = "server"
path = "src/server.rs"

[[bin]]
name = "worker"
path = "src/worker.rs"
```

Single brackets define one table. Double brackets append to an array of tables. The same
thing in JSON is a list of objects: `"bin": [ { }, { } ]`. TOML has no other way to write a
repeated block, so when you see the doubled brackets, read them as "and another".

## What it is for

`Cargo.toml` is the manifest for every Rust project. `pyproject.toml` is the modern Python
project file. Codex keeps its settings in `config.toml`. `rustfmt.toml`, `netlify.toml`,
and `.taplo.toml` all follow the same pattern: the tool's name, then `.toml`.

## The gotchas

**A heading applies until the next heading.** A key you paste at the bottom of the file
lands in whatever table came last, not at the top level. When a setting is being ignored,
the first thing to check is which `[section]` it fell into.

**Top-level keys go at the very top.** Any bare `key = value` that belongs to no section
must appear before the first `[table]` line. Put it lower and it silently joins that table.

**Dotted headings and dotted keys are the same thing.** `[profile.release]` with
`opt-level = 3` under it, and `profile.release.opt-level = 3` on a single line, set exactly
the same value. Both are legal, and tools write the first form.

**Quotes decide the type.** `port = "8080"` is text. `port = 8080` is a number. A tool
expecting a number will reject the quoted form, and its error message rarely says which of
the two it wanted.

## Reading its errors

```text
error: failed to parse manifest at `C:\Users\<yourname>\projects\app\Cargo.toml`

Caused by:
  TOML parse error at line 4, column 8
    |
  4 | version: "1.0.0"
    |        ^
  expected `.`, `=`
```

Recognize it by the phrase "TOML parse error" and a caret drawn under the character that
broke it. `expected =` means you wrote a colon where an equals sign belongs, which is the
YAML habit leaking across. The other frequent message is `invalid value`, which usually
means a string that forgot its quotes.
