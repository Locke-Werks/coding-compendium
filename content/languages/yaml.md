---
id: yaml
title: YAML
type: language
verified: 2026-08-02
volatility: low

name: YAML
aka: [yml, "yaml ain't markup language", docker-compose, github actions workflow]
family: config
likelihood: certain
extensions: ['.yml', '.yaml']

tells:
  - pattern: '^\s*[A-Za-z_][\w.-]*:\s'
    kind: regex
    weight: 8
    note: >
      A bare unquoted key, a colon, then a space and the value. JSON quotes the
      key and TOML separates with an equals sign, so an unquoted `key: value` is
      YAML.
  - pattern: '^\s*-\s'
    kind: regex
    weight: 6
    note: >
      A dash and a space at the start of a line is one list entry. JSON writes a
      list inside `[ ]`, and TOML writes repeated `[[table]]` headings.
  - pattern: '^---\s*$'
    kind: regex
    weight: 7
    note: >
      Three dashes alone on a line open a document or separate two of them. In
      Markdown the same three dashes at the top of a file open a frontmatter
      block, which is itself YAML.
  - pattern: '^\s*#'
    kind: regex
    weight: 3
    note: >
      Comments start with `#`, shared with TOML, INI, and shell scripts. JSON has
      no comments at all, so a `#` rules JSON out immediately.
  - pattern: '\|\s*$'
    kind: regex
    weight: 6
    note: >
      A trailing pipe or `>` opens a multi-line string that keeps or folds its
      newlines. No other config format has this, and it is why long shell scripts
      end up living inside YAML files.
  - pattern: '\$\{\{'
    kind: regex
    weight: 7
    note: >
      Double curly braces after a dollar sign is GitHub Actions expression
      syntax, as in `${{ secrets.TOKEN }}`. It appears in workflow YAML and
      nowhere else on this deck.
  - pattern: '^\s*\w+:\s*$'
    kind: regex
    weight: 5
    note: >
      A key with nothing after the colon means the value is the indented block
      below it. JSON would open a brace here and TOML would start a new
      `[section]` heading.

rules_out:
  - pattern: '='
    kind: operator
    because: TOML or INI, which assign with an equals sign rather than a colon
  - pattern: "\t"
    kind: sigil
    because: not YAML at all. A tab character is illegal in YAML, so a tab-indented file is something else.
  - pattern: '^\s*\{'
    kind: regex
    because: JSON, if the whole file is wrapped in braces
  - pattern: '<'
    kind: sigil
    because: XML or HTML, which nest with angle-bracket tags
  - pattern: '^\s*\w+\s*\(\)'
    kind: regex
    because: a programming language. YAML has no functions to call.

project_fingerprint:
  manifests:
    - file: docker-compose.yml
      note: >
        Describes a set of containers, their images, ports, and volumes. The
        Dockerfile builds one image; this file says how to run several together.
    - file: .github/workflows/*.yml
      decisive: true
      note: >
        Anything under this folder is a GitHub Actions workflow, and it is always
        YAML. This is where most people meet the format for the first time and
        immediately break it on indentation.
    - file: '*.yaml'
      decisive: true
      note: Both extensions mean the same thing. `.yml` is more common, `.yaml` is the official spelling.
    - file: pubspec.yaml
      note: The manifest for a Dart or Flutter project.
    - file: .pre-commit-config.yaml
      note: Git hook configuration, common in Python projects.
    - file: openapi.yaml
      note: A description of a web service's endpoints. Frequently thousands of lines.

shape:
  blocks: indentation
  statement_end: newline
  comment_line: '#'
  string_quotes: >
    Quotes are optional and usually left off. Single and double both work, and
    double quotes are the ones that understand escapes like `\n`.
  naming: >
    Mostly lower-case keys with dashes or underscores. Kubernetes uses camelCase,
    GitHub Actions uses dashes, and nothing enforces either.
  import_keyword: 'none, though many tools add their own include or extends key'

confusable_with:
  - language: json
    settle_it: >
      JSON wraps everything in braces and double-quotes every key. YAML has no
      braces and no quotes. If the file is one big `{ ... }`, it is JSON, and it
      is also technically valid YAML, because YAML is a superset of JSON.
    tiebreak: { pattern: '"[^"\n]+"\s*:', kind: regex, favors: json }
  - language: toml
    settle_it: >
      Both are indentation-friendly config formats with `#` comments. YAML uses a
      colon and nests by indenting. TOML uses an equals sign and nests with
      `[section.subsection]` headings, and its indentation means nothing.
    tiebreak: { pattern: '=', kind: operator, favors: toml }
  - language: markdown
    settle_it: >
      A Markdown file that opens with three dashes has a YAML frontmatter block
      at the top and Markdown below it. Inside the dashes, YAML rules apply.
      Below them, prose and `#` headings mean Markdown.
    tiebreak: { pattern: '^#{1,6}\s+\S', kind: regex, favors: markdown }
  - language: ini
    settle_it: >
      INI groups keys under `[section]` headings and joins them with `=`. YAML
      has no section headings and joins with `: `. Both allow `#` comments, so
      the separator is the thing to look at.
    tiebreak: { pattern: '^\s*\[[\w.-]+\]\s*$', kind: regex, favors: ini }
  - language: css
    settle_it: >
      Both use `key: value` pairs and both look like settings. YAML uses indentation
      with no braces and no semicolons. CSS wraps every group in braces and ends
      every line with a semicolon.
    tiebreak: { pattern: '\{', kind: regex, favors: css }
  - language: dockerfile
    settle_it: >
      They travel together and do different jobs. The Dockerfile builds one image and
      opens every line with an uppercase keyword. `docker-compose.yml` is YAML and
      says how to run several containers together, in `key: value` pairs.
    tiebreak: { pattern: '^FROM ', kind: line_start, favors: dockerfile }

errors_look_like:
  sample: |
    yaml.scanner.ScannerError: mapping values are not allowed in this context
      in "docker-compose.yml", line 7, column 14

    Error: .github/workflows/ci.yml (Line: 12, Col: 5): Unexpected value ''
  recognize_by: >
    The word `yaml` inside the error type, a filename ending `.yml`, and a line
    and column pair. The reported line is where the parser noticed the problem,
    which is usually one or two lines past the line you actually broke, so read
    upward from it.
  patterns:
    - 'yaml\.(scanner|parser)\.\w+Error'
    - 'mapping values are not allowed'
    - 'did not find expected key'
    - 'found character .\\t. that cannot start any token'
    - '\.ya?ml.*[Ll]ine:? ?\d+'

meet_it_when: >
  You open `docker-compose.yml` to run a project locally, you add a step to a
  GitHub Actions workflow so tests run on every push, or an agent generates a
  deployment file and you have to check it. It is also the frontmatter block at
  the top of most Markdown files in a documentation site.

what_agents_get_wrong: >
  Indentation is the one an agent breaks most, because it is generating text
  rather than a tree: a nested key comes back one level too shallow, the file
  still parses, and the tool reads a setting that is now in the wrong place.
  Nothing errors. Agents also emit tab characters when reformatting, which YAML
  rejects outright, and they quote inconsistently, so `version: 3.10` becomes the
  number 3.1 in a file that wanted the text "3.10". The invented-key failure is
  the expensive one: a plausible but nonexistent key in a GitHub Actions workflow
  is silently ignored, the workflow runs green, and the step you asked for never
  happened. In a diff, check the indentation level of every changed line against
  its neighbors, and check every new key against the tool's documentation.

version_landscape: >
  YAML 1.2 came out in 2009 and tightened the type rules, but many parsers still
  behave like 1.1, which is where the `no` becomes false problem lives. You
  cannot tell which one a tool uses by looking. Quote anything you care about and
  the difference stops mattering.

see_also:
  - json
  - toml
  - markdown
  - dockerfile
  - j2-the-config-formats-nobody-explains
  - h5-ci-cd
  - b9-where-settings-live

keywords: [yml, docker-compose, github actions, workflow, indentation error, norway problem, frontmatter, kubernetes]
---

YAML (YAML Ain't Markup Language) is a text format for storing data, written with
indentation instead of brackets. It is not a programming language. It has no logic, no
functions, and nothing inside a `.yml` file ever runs. It describes data, and some other
program reads it and acts on it.

The name is a joke about itself. It started life as "Yet Another Markup Language" and was
renamed to insist that it is not one.

## The shape

A key, a colon, a space, then the value. Nesting is done by indenting, two spaces per
level by convention. A list is a run of lines each beginning with a dash.

```yaml
name: my-app
version: "1.0.0"
services:
  web:
    image: nginx
    ports:
      - "8080:80"
    environment:
      NODE_ENV: production
```

The contrast that settles it: JSON (JavaScript Object Notation) wraps that same data in
braces, quotes every key, and separates entries with commas. TOML (Tom's Obvious, Minimal
Language) writes `key = value` with an equals sign under `[section]` headings. YAML has no
braces, no commas between entries, and no equals sign. A bare word, a colon, and a space is
YAML and nothing else.

Comments start with `#` and run to the end of the line. JSON has no comments at all, which
is a large part of why so many tools moved to YAML.

Every JSON file is also a valid YAML file, because YAML is a superset of it. That is why a
`.yml` file is allowed to contain `{ "a": 1 }`, and why "it looks like JSON" does not prove
it is JSON.

## What it is for

Pipelines and orchestration, mostly. `docker-compose.yml` describes a set of containers and
how they connect. `.github/workflows/ci.yml` describes what GitHub runs when you push.
Kubernetes configuration is YAML from top to bottom. Static site generators, and this app,
put a YAML block at the top of each Markdown file, fenced by `---`, called frontmatter.

## The gotchas

This is the format that will actually cost you an afternoon, so read this part twice.

**Indentation is the structure, and tabs are illegal.** Spaces only. A tab character is a
hard error in the specification, and the message you get back rarely says the word "tab".
Your editor may be inserting them without telling you. If a YAML file broke right after you
pressed Tab, that is why.

**One wrong space changes the meaning without breaking the file.** Indent a key two spaces
further and it becomes a child of the line above rather than its sibling. The file still
parses. The tool reads it, finds nothing where it expected something, and either uses a
default or does nothing at all.

**The Norway problem.** An unquoted `no` parses as the boolean false, and so do `yes`,
`on`, `off`, `y`, and `n`. A list of country codes containing Norway used to quietly lose
Norway. YAML 1.2 narrowed the rule to `true` and `false` only, but plenty of parsers still
run the old behavior. Quote it: `"no"`.

**Version numbers turn into decimals.** `version: 1.0` is the number one. `version: 1.10`
is the number 1.1, and the zero is gone forever. `version: 1.0.0` stays text, because it is
not a valid number. Quote every version you write: `version: "1.10"`.

**There are two syntaxes for everything.** A list can be written block style with dashes or
flow style in square brackets. Both are correct, and you will meet both inside the same
file.

```yaml
ports:
  - "8080:80"
  - "8443:443"

ports: ["8080:80", "8443:443"]
```

## Reading its errors

```text
yaml.scanner.ScannerError: mapping values are not allowed in this context
  in "docker-compose.yml", line 7, column 14
```

Recognize it by the word `yaml` in the error type and a line-and-column pair naming a
`.yml` file. The line it reports is where the parser noticed something wrong, which is
often one or two lines below the line you actually broke. Read upward from it.

That particular message nearly always means an unquoted colon inside a value. This breaks:

```yaml
title: The loop: describe, review
```

This works:

```yaml
title: "The loop: describe, review"
```

When you cannot find it by eye, paste the file to an agent and ask which line is wrong.
Whitespace bugs are invisible to you and obvious to a parser.
