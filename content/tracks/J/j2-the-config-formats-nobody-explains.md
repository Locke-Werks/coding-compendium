---
id: j2-the-config-formats-nobody-explains
title: JSON, YAML, TOML, and friends
type: section
track: J
order: 20
verified: 2026-08-02
volatility: low
answer: >
  These are configuration formats, not programming languages: no logic, no
  functions, and nothing in them runs. A quoted key with a colon is JSON, a bare
  key with a colon is YAML, and an equals sign is TOML or an INI file.
owns:
  - config formats as a family
  - why there are several
see_also:
  - b9-where-settings-live
  - j1-how-to-recognize-a-language
  - json
  - yaml
  - toml
  - g5-environment-variables
  - j3-project-layouts
keywords:
  - config file
  - yml vs yaml
  - what is a config format
  - settings file
  - docker-compose
  - frontmatter
  - is json code
---

## More

Start here, because nobody says it out loud: **these files are not code.** JSON (JavaScript
Object Notation), YAML (YAML Ain't Markup Language), and TOML (Tom's Obvious, Minimal
Language) have no logic, no functions, no loops, and no way to make a decision. Nothing
inside one ever runs. Each is a way of writing down settings or data so that a real program
can read them and behave accordingly.

That means a mistake in one of these files is never a bug in your program's thinking. It is
a typo, a wrong value, or a key the tool does not recognize.

Telling them apart takes one glance at any line that assigns something:

```text
"name": "my-app"      JSON: the key is quoted, the separator is a colon
name: my-app          YAML: the key is bare, the separator is a colon and a space
name = "my-app"       TOML: the separator is an equals sign
name=my-app           INI or .env: an equals sign, no quotes, no sections in brackets
<name>my-app</name>   XML: angle brackets wrapping everything
```

The three you will meet in your first week, and what each one is:

- **JSON.** `package.json`, `tsconfig.json`, `settings.json`. Machine-first, and the format
  most web services answer in.
- **YAML.** `docker-compose.yml`, `.github/workflows/ci.yml`, and the block at the top of
  every file in this app. Human-first, indentation carries the meaning.
- **TOML.** `Cargo.toml`, `pyproject.toml`, the Codex `config.toml`. Human-first, sections in
  square brackets, no indentation rules to get wrong.

Now the three gotchas, which are the whole practical value of knowing they are different:

1. **JSON allows no comments and no trailing commas.** No `#`, no `//`, and the last entry
   before a `}` gets no comma after it.
2. **YAML forbids tab characters and treats indentation as meaning.** Two spaces of
   difference changes which thing owns which.
3. **TOML writes `key = value` under a `[section]` heading**, and everything after a heading
   belongs to it until the next heading.

The [JSON](#json), [YAML](#yaml), and [TOML](#toml) cards each go through their own traps in
detail. This card is the family view: which one you are holding, and what kind of mistake
each one invites.

## Full

### Why there are several

They arrived in that order, each one reacting to the last.

JSON came out of JavaScript in the early 2000s as a way to send data between programs. It
was designed for machines to write and machines to read, which is why it has no comments:
nobody expected a person to open one. Then everyone started writing configuration in it by
hand, and the missing comment became the most complained-about feature in software.

YAML answered that. Comments, no braces, no quotes needed, indentation for structure. It is
pleasant to read and it has more special cases than any format on this page, because "guess
what the human meant" turns out to be a hard problem.

TOML answered YAML. Explicit sections, an equals sign, indentation that carries no meaning
at all. Rust and modern Python packaging both chose it, which is most of why you meet it.

INI (Initialization) predates all three by decades and never left, because it is the
simplest thing that works. A `.env` file is an INI file wearing different clothes.

XML (Extensible Markup Language) predates JSON and lost to it everywhere except the Java and
C# worlds, where `pom.xml` and `.csproj` files are still XML and still fine.

### The family, at a glance

| Format | Assigns with | Comments | Nesting by | You meet it in |
|---|---|---|---|---|
| JSON | `"key": value` | None | Braces | `package.json`, web responses |
| YAML | `key: value` | `#` | Indentation | Pipelines, containers, frontmatter |
| TOML | `key = value` | `#` | `[section]` headings | `Cargo.toml`, `pyproject.toml` |
| INI | `key=value` | `#` or `;` | `[section]` headings | `.gitconfig`, old Windows settings |
| `.env` | `KEY=value` | `#` | Nothing, it is flat | Local secrets ([g5](#g5-environment-variables)) |
| XML | `<key>value</key>` | `<!-- -->` | Nested tags | Java and C# projects |

Two more that look like this family and are not. A `Dockerfile` is a list of build
instructions that do run, in order ([Dockerfile](#dockerfile)). A `Makefile` is a set of
rules with shell commands inside them, and it is famously particular about tab characters
([Makefile](#makefile)).

### Two failures, and the second one is the dangerous one

**The file will not parse.** The tool refuses to start and prints a line number. This is
loud, annoying, and harmless: nothing ran, so nothing broke. The usual causes are the
gotchas above, and pasting the file into an agent and asking it to find the syntax error is
a reasonable use of thirty seconds.

**The file parses and the setting does nothing.** This is the one that costs an afternoon.
You added a key, the file is valid, the tool loads it, and the behavior does not change,
because the tool has never heard of that key and ignores what it does not recognize. No
error appears anywhere.

This is a favorite failure of coding agents. Asked to change a setting, an agent will
confidently invent a plausible key name, write it into valid JSON, and report success. The
check is not "does the file load." The check is "does the key exist in this tool's own
documentation," and the file cannot tell you.

### Editing one without breaking it

1. **Copy the shape of the line above.** Whatever the existing entries do about quotes,
   commas, and indentation, do that.
2. **Never use Tab in a YAML file.** Set your editor to insert spaces. A tab is a hard error
   in the specification and the message rarely contains the word "tab."
3. **Quote anything that looks like a number, a date, or a yes.** In YAML, `version: 1.10`
   becomes the number 1.1, and an unquoted `no` in some parsers becomes false. Quotes cost
   nothing.
4. **Save, then let the tool read it.** Restart the tool, or run its own validation command
   if it has one. Many do: `docker compose config` prints the file back to you if it parsed.
5. **When it fails, read the line number as a starting point, not a location.** These parsers
   report where they gave up, which is often a line or two past the line you actually broke.

### The one about file extensions

`.yml` and `.yaml` are the same format. Different projects picked different spellings and
neither is more correct. Docker uses `.yml`, Kubernetes tends to use `.yaml`, and a tool
that expects one will not always find the other, so match whatever the tool's documentation
shows.

`.json` files that contain `//` comments are not JSON. They are JSONC (JSON with Comments),
which Visual Studio Code accepts in its own settings and almost nothing else accepts
anywhere. If a comment in a `.json` file breaks a tool, that is the tool being correct.

### Where your own settings live

Each agent and editor keeps its configuration in one of these formats, at more than one
level, with the narrower level winning. Which file, and which level, is
[b9](#b9-where-settings-live). This card is only about reading the syntax once you have the
file open.
