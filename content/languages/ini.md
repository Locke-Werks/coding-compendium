---
id: ini
title: INI and .env files
type: language
verified: 2026-08-02
volatility: low

name: INI and .env
aka: [ini, dotenv, env file, conf, cfg, properties]
family: config
likelihood: certain
extensions: ['.ini', '.env', '.cfg', '.conf', '.properties']

tells:
  - pattern: '^\s*[A-Z_][A-Z0-9_]*=\S'
    kind: regex
    weight: 8
    note: >
      A shouting key, an equals sign, no spaces around it, as in
      `DATABASE_URL=postgres://localhost`. That is a `.env` line. TOML would put
      spaces around the equals sign and quote the value.
  - pattern: '^\s*\[[\w.\s-]+\]\s*$'
    kind: regex
    weight: 5
    note: >
      A section heading in square brackets. TOML uses the same mark, so this
      narrows you to those two and settles nothing on its own.
  - pattern: '^\s*[\w.-]+\s*=\s*[^"''\n]+$'
    kind: regex
    weight: 6
    note: >
      An unquoted value after an equals sign. INI treats everything as text, so
      nothing needs quotes. TOML requires quotes on every string, so an unquoted
      word points here.
  - pattern: '^\s*;'
    kind: regex
    weight: 7
    note: >
      A semicolon starting a comment is pure INI. TOML and YAML use `#`, and both
      styles appear in INI files depending on who wrote the parser.
  - pattern: '^\s*\w+\s*:\s*\S'
    kind: regex
    weight: 2
    note: >
      Some INI dialects accept a colon instead of an equals sign, which is why a
      file with both separators mixed together is INI rather than YAML or TOML.

rules_out:
  - pattern: '^\s*\[\['
    kind: regex
    because: TOML, whose doubled brackets are an array of tables. INI has no such form.
  - pattern: '^\s*-\s'
    kind: regex
    because: YAML, where a leading dash is a list entry. INI has no lists.
  - pattern: '^\s*\{'
    kind: regex
    because: JSON
  - pattern: '<'
    kind: sigil
    because: XML or HTML

project_fingerprint:
  manifests:
    - file: .env
      decisive: true
      note: >
        Environment variables for local development, one `KEY=value` per line. It
        usually holds secrets, which is why `.gitignore` almost always names it.
    - file: .env.example
      note: >
        The same file with the values removed, committed on purpose so you know
        which keys to fill in. Copy it to `.env` and edit.
    - file: .gitconfig
      decisive: false
      note: Git's own settings, in INI format, at `C:\Users\<yourname>\.gitconfig`.
    - file: .editorconfig
      note: Indentation and line-ending rules your editor picks up automatically.
    - file: tox.ini
      note: Test automation settings in a Python project.
    - file: setup.cfg
      note: Older Python packaging settings, now mostly replaced by `pyproject.toml`.
    - file: php.ini
      note: The settings file for a PHP installation. The format is named after files like this one.

shape:
  blocks: none
  statement_end: newline
  comment_line: '# or ;'
  string_quotes: >
    Usually none. Everything after the equals sign is text, including the spaces,
    which is why a stray trailing space becomes part of the value.
  naming: SCREAMING_SNAKE_CASE in .env files, lower-case with dots or dashes in .ini files
  import_keyword: 'none, though some tools add an include directive of their own'

confusable_with:
  - language: toml
    settle_it: >
      They look almost the same: `[section]` headings and `key = value` lines.
      TOML quotes its strings and has real types, so `name = "app"` sits beside
      `port = 8080`. INI quotes nothing, because every value is text.
    tiebreak: { pattern: '^\s*[\w.-]+\s*=\s*"', kind: regex, favors: toml }
  - language: yaml
    settle_it: >
      YAML separates a key from its value with a colon and a space and nests by
      indenting. INI uses an equals sign and has no nesting at all beyond one
      level of `[section]`.
    tiebreak: { pattern: '^\s*[\w.-]+:\s', kind: regex, favors: yaml }
  - language: gitignore
    settle_it: >
      Both are flat line-oriented files with `#` comments. Every meaningful line
      in an INI or `.env` file has an equals sign. A `.gitignore` has none, only
      paths and glob characters.
    tiebreak: { pattern: '=', kind: operator, favors: ini }

errors_look_like:
  sample: |
    configparser.MissingSectionHeaderError: File contains no section headers.
    file: 'tox.ini', line: 1
    'name = app\n'
  recognize_by: >
    There is barely any error to read. Most `.env` loaders never report a
    problem: a malformed line is skipped in silence and the variable comes back
    empty, so the failure shows up later as a missing setting. Python's
    `configparser` is the exception and names the file and line.
  patterns:
    - 'configparser\.\w+Error'
    - 'MissingSectionHeaderError'
    - 'ParsingError'

meet_it_when: >
  You copy `.env.example` to `.env` before running a project for the first time,
  you paste an API key into it, or you open `.gitconfig` to check which email git
  is stamping on your commits.

what_agents_get_wrong: >
  The failure mode here is silence. There is no INI standard, so every parser
  behaves slightly differently and almost none of them report a bad line: the key
  is skipped, the value comes back empty, and your application fails later
  somewhere unrelated. Agents write `KEY = value` with spaces in a `.env` file,
  and several loaders keep the space as part of the value. They write
  `KEY="value"` and some loaders keep the quote marks. They add inline comments
  after a value, which certain parsers treat as text. The one to watch hardest in
  a diff: an agent that writes a real key into `.env.example`, or that adds
  `.env` to a commit, has just published a secret.

see_also:
  - toml
  - yaml
  - gitignore
  - g5-environment-variables
  - g6-secrets-and-what-never-to-commit

keywords: [dotenv, .env, environment variable, gitconfig, editorconfig, section header, configparser]
---

INI (Initialization) files are the oldest configuration format still in daily use:
`key = value`, one per line, grouped under `[section]` headings. There is no standard for
them, so every program's version differs a little. Nothing inside one runs.

A `.env` file is the same idea with the sections removed. One `KEY=value` per line, loaded
into environment variables by tools like Docker Compose and `dotenv`.

```ini
[database]
host = localhost
port = 5432

[logging]
level = debug
```

```ini
# .env, which never gets committed
DATABASE_URL=postgres://localhost:5432/app
API_KEY=replace-me
```

Settle it against TOML (Tom's Obvious, Minimal Language), which it resembles almost
exactly: TOML quotes its strings and has real types, INI treats every value as text. If
`name = "app"` sits beside `port = 8080`, it is TOML. If nothing is quoted anywhere, it is
INI. Settle it against YAML (YAML Ain't Markup Language): YAML separates with a colon, INI
with an equals sign.

You meet `.env` first and constantly. It holds secrets, which is why `.gitignore` almost
always names it, and why a `.env` file appearing in a diff is worth stopping for.
