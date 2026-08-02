---
id: gitignore
title: .gitignore
type: language
verified: 2026-08-02
volatility: low

name: .gitignore
aka: [gitignore, ignore file, dockerignore, npmignore, glob patterns]
family: config
likelihood: certain
extensions: ['.gitignore', '.dockerignore', '.npmignore', '.prettierignore']

tells:
  - pattern: '^[\w.*/-]+/\s*$'
    kind: regex
    weight: 7
    note: >
      A bare path ending in a slash, alone on the line, with no equals sign and
      no colon. INI and TOML lines always assign something; these lines assign
      nothing because they are patterns.
  - pattern: '^!\S'
    kind: regex
    weight: 9
    note: >
      A leading exclamation mark un-ignores something an earlier line ignored. No
      other line-oriented format uses `!` at the start of a line for anything.
  - pattern: '^\*\.\w+\s*$'
    kind: regex
    weight: 8
    note: >
      A star, a dot, an extension, and nothing else, as in `*.log`. A shell
      script would do something with that pattern; here the pattern is the whole
      line.
  - pattern: '^node_modules/?\s*$'
    kind: regex
    weight: 6
    note: >
      The single most common line in the file. Seeing a folder name with nothing
      around it, rather than `folder = something`, rules out every config format.
  - pattern: '^\*\*/'
    kind: regex
    weight: 7
    note: >
      Double star then slash means "at any depth". It is glob syntax, shared with
      shell globbing, and it never appears in JSON, YAML, or TOML.

rules_out:
  - pattern: '='
    kind: operator
    because: INI, TOML, or a .env file. A gitignore line never assigns a value.
  - pattern: ':\s'
    kind: regex
    because: YAML or JSON
  - pattern: '^#!'
    kind: line_start
    because: a shell script
  - pattern: '<'
    kind: sigil
    because: XML or HTML

project_fingerprint:
  manifests:
    - file: .gitignore
      decisive: true
      note: >
        Sits in the project root, and often in subfolders too, where it applies
        to that folder and everything under it.
    - file: .dockerignore
      note: >
        Keeps files out of the build context that `docker build` uploads. Same
        syntax, different tool.
    - file: .npmignore
      note: Keeps files out of a published npm package. Same syntax again.
    - file: .prettierignore
      note: Files the formatter leaves alone. Also `.eslintignore`, and the pattern continues.
    - file: .git/info/exclude
      note: >
        Personal ignores that are never committed, for things only your machine
        creates. Same syntax, and nobody knows it exists.

shape:
  blocks: none
  statement_end: newline
  comment_line: '#'
  string_quotes: 'none, a line is a path pattern and quoting it would break it'
  naming: >
    Patterns match paths, so they follow whatever the files are called. A
    trailing slash means folders only.
  import_keyword: 'none, though a .gitignore in a subfolder adds to the one above it'

confusable_with:
  - language: ini
    settle_it: >
      Both are flat lists of lines with `#` comments. Every meaningful line in an
      INI or `.env` file has an equals sign in it. A `.gitignore` line has no
      equals sign and no colon, only a path and glob characters.
    tiebreak: { pattern: '=', kind: operator, favors: ini }
  - language: bash
    settle_it: >
      Both contain bare words and `*` wildcards. A shell script runs commands, so
      its lines start with a verb such as `rm` or `echo`, and it opens with
      `#!/bin/bash`. A gitignore line is a noun.
    tiebreak: { pattern: '^#!/', kind: regex, favors: bash }

errors_look_like:
  sample: |
    $ git status
    On branch main
    nothing to commit, working tree clean
  recognize_by: >
    There are no parse errors. A malformed pattern is just a pattern that matches
    nothing, so the failure looks like a file you expected to be ignored showing
    up in `git status`, or a file you needed being invisible to git.
  patterns:
    - 'The following paths are ignored by one of your \.gitignore files'
    - 'use -f if you really want to add them'

meet_it_when: >
  Every repository has one, and you edit it the first time a folder you do not
  want tracked shows up in `git status`. You will also meet `.dockerignore` the
  first time a container build takes four minutes because it uploaded
  `node_modules`.

what_agents_get_wrong: >
  Agents generate a huge boilerplate ignore file covering nine languages the
  project does not use, which is harmless clutter, and then miss the one entry
  that mattered. The real trap is one they cannot fix by editing this file at
  all: `.gitignore` only applies to files git is not already tracking. If `.env`
  was committed once, adding it here changes nothing, the file stays tracked, and
  it keeps getting pushed. An agent that says "I added it to gitignore, you are
  covered" is wrong, and the secret is still in the history. Watch also for an
  ignore rule that hides something a build needs, since the failure appears only
  on a fresh clone or on the build server, where nobody has the untracked copy.

see_also:
  - ini
  - dockerfile
  - bash
  - d12-gitignore-and-what-not-to-commit
  - g6-secrets-and-what-never-to-commit

keywords: [gitignore, dockerignore, ignore, glob, node_modules, untracked, exclude]
---

A `.gitignore` file is a list of file patterns git will refuse to track. One pattern per
line. It is not a programming language and nothing inside it runs. It is a filter.

```gitignore
node_modules/
target/
dist/

.env
*.log

!important.log
```

Four rules cover nearly everything. A bare name matches that file or folder anywhere in
the tree. A trailing `/` means folders only. `*` matches any run of characters inside one
path segment, and `**/` means at any depth. A leading `!` un-ignores something an earlier
line ignored.

Settle it against a shell script or a Dockerfile: those run commands, and this holds only
paths and glob characters. Settle it against `.env` and INI (Initialization) files: those
are `KEY=value` pairs, and a gitignore line has no equals sign anywhere. A `#` starts a
comment in all of them.

The thing that catches everyone: this file only affects files git is not already tracking.
Adding `.env` to it after you committed `.env` changes nothing at all. See
[d12-gitignore-and-what-not-to-commit](#d12-gitignore-and-what-not-to-commit) for the fix,
and [g6-secrets-and-what-never-to-commit](#g6-secrets-and-what-never-to-commit) if that
file held a secret.

The same syntax runs `.dockerignore`, `.npmignore`, and `.prettierignore`.
