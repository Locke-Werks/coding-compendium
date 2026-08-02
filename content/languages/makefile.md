---
id: makefile
title: Makefile
type: language
verified: 2026-08-02
volatility: low

name: Makefile
aka: [make, gnu make, gmake, mk]
family: config
likelihood: possible
extensions: ['.mk', '.make']

tells:
  - pattern: '^[A-Za-z_.][\w.-]*:\s*$'
    kind: regex
    weight: 8
    note: >
      A bare name, a colon, and nothing after it, with commands indented below.
      YAML puts a value or an indented block after its colon; a Makefile target
      is followed by shell commands.
  - pattern: '^\t\S'
    kind: regex
    weight: 9
    note: >
      A real tab character starting a command line. YAML forbids tabs outright
      and Python treats them as a mistake, so a required literal tab is a
      Makefile and nothing else.
  - pattern: '\$\((\w+)\)'
    kind: regex
    weight: 7
    note: >
      Variables are read as `$(NAME)`. Shell scripts write `$NAME` or `${NAME}`,
      so the round brackets are the tell.
  - pattern: '^\.PHONY:'
    kind: regex
    weight: 10
    note: >
      Declares targets that are commands rather than files. It appears in
      Makefiles and in no other format on this deck.
  - pattern: '^[\w.-]+\s*:?=\s'
    kind: regex
    weight: 5
    note: >
      Assignment with `:=` or `=` at the top of the file. TOML also uses `=`, but
      TOML has no tab-indented command lines under a target.

rules_out:
  - pattern: '^#!'
    kind: line_start
    because: a shell script. A Makefile has no shebang.
  - pattern: '^\s*-\s\w+:'
    kind: regex
    because: YAML, most likely a workflow file
  - pattern: '^FROM\s'
    kind: regex
    because: a Dockerfile
  - pattern: '^\s*\{'
    kind: regex
    because: JSON

project_fingerprint:
  manifests:
    - file: Makefile
      decisive: true
      note: >
        The standard name, capital M, in the project root. Run `make` with no
        arguments and the first target in the file is what happens.
    - file: makefile
      note: Lowercase works too. On Windows the two names are the same file anyway.
    - file: GNUmakefile
      note: Read in preference to `Makefile` when the contents rely on GNU extensions.
    - file: '*.mk'
      note: A fragment included by a larger Makefile. Same syntax.
    - file: Justfile
      note: >
        Not a Makefile, but the modern replacement people reach for, and it does
        not need tabs. If you see one, read it the same way.

shape:
  blocks: indentation
  statement_end: newline
  comment_line: '#'
  string_quotes: >
    None of its own. The command lines are handed to a shell, so quoting follows
    shell rules once make has finished expanding its variables.
  naming: >
    Lower-case target names such as `build`, `test`, and `clean`. Variables are
    SCREAMING_SNAKE_CASE by convention.
  import_keyword: include

tooling:
  runtime: GNU make, which Windows does not ship. Git Bash does not include it either.
  run_command: make build
  test_command: make test

confusable_with:
  - language: yaml
    settle_it: >
      Both use colons and indentation. A Makefile writes `build:` with nothing
      after the colon and tab-indented shell commands below. YAML always has a
      value or a nested block after the colon, and forbids tabs completely.
    tiebreak: { pattern: '^\t\S', kind: regex, favors: makefile }
  - language: bash
    settle_it: >
      The indented lines in a Makefile are shell commands, which is why it reads
      like a script. A shell script opens with `#!/bin/bash`, has no `target:`
      lines, and writes variables as `$NAME` rather than `$(NAME)`.
    tiebreak: { pattern: '^#!/', kind: regex, favors: bash }

errors_look_like:
  sample: |
    Makefile:4: *** missing separator.  Stop.
    make: *** No rule to make target 'buld'.  Stop.
  recognize_by: >
    Three asterisks and the word `Stop.` at the end of the line, with the
    filename and line number in front. `missing separator` means spaces where a
    tab belongs, and it is the error every beginner meets first.
  patterns:
    - 'missing separator'
    - '^make(file)?(\[\d+\])?: \*\*\*'
    - 'No rule to make target'
    - '^Makefile:\d+: \*\*\*'

meet_it_when: >
  You clone a project that grew up on Linux and the README tells you to run
  `make build`. On Windows this is the moment you discover `make` is not
  installed, so read the file and run the commands yourself instead.

what_agents_get_wrong: >
  Tabs. An agent generating a Makefile emits spaces for indentation, because that
  is what it does everywhere else, and the file fails immediately with
  `missing separator`. That one is loud and cheap to fix. The quiet failure is
  the shell: each recipe line runs in its own shell, so a `cd` on one line does
  not affect the next, and an agent that writes a multi-step recipe as separate
  lines produces something that runs in the wrong folder without saying so. Watch
  also for `$` in a shell command, which has to be doubled to `$$` to survive
  make's own variable expansion, and for recipes that were never listed under
  `.PHONY` and stop running once a file of the same name exists.

see_also:
  - bash
  - dockerfile
  - yaml
  - c3-what-running-means

keywords: [make, missing separator, phony, target, recipe, tab character, gnu make]
---

A Makefile is a list of named tasks and the shell commands that run them. Type
`make build` and it finds the `build:` target in the file and runs the lines underneath.
It is not a programming language in any ordinary sense, though it has enough variables and
conditionals to argue about at length.

```makefile
BINARY := server

build:
	go build -o $(BINARY) ./cmd/server

test:
	go test ./...

.PHONY: build test
```

Every command line under a target has to begin with a real tab character, not spaces.
Nothing else in common use demands a literal tab. When you see
`Makefile:4: *** missing separator.  Stop.`, that is what happened.

Settle it against YAML (YAML Ain't Markup Language), which also uses colons and
indentation: YAML forbids tabs entirely and always puts a value or a nested block after
the colon, while a Makefile leaves the colon bare and puts shell commands below it. Settle
it against a shell script: a script has no `target:` lines and opens with `#!/bin/bash`.

Variables read as `$(NAME)`. A `$` meant for the shell has to be written `$$`.

You meet this on projects born on Linux. Windows has no `make` command, so a project with
a Makefile means either installing one or reading the file and running the commands by
hand.
