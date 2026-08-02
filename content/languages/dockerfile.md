---
id: dockerfile
title: Dockerfile
type: language
verified: 2026-08-02
volatility: quarterly
verify: docker --version

name: Dockerfile
aka: [docker, containerfile, docker build, oci image]
family: config
likelihood: likely
extensions: ['.dockerfile']

tells:
  - pattern: '^FROM\s+\S'
    kind: regex
    weight: 10
    note: >
      Every Dockerfile starts with `FROM` on the first non-comment line. SQL also
      has a `FROM`, but it sits inside a query after `SELECT`, never alone at the
      start of a line naming an image.
  - pattern: '^RUN\s'
    kind: regex
    weight: 9
    note: >
      An uppercase `RUN` at the start of a line, followed by an ordinary shell
      command. A shell script would have the command with no keyword in front of
      it.
  - pattern: '^COPY\s'
    kind: regex
    weight: 9
    note: >
      `COPY source destination` brings files in from your project folder. Nothing
      else on this deck uses a bare uppercase `COPY` as a line-opening keyword.
  - pattern: '^(CMD|ENTRYPOINT)\s'
    kind: regex
    weight: 8
    note: >
      Says what runs when a container starts, usually as a bracketed list,
      `CMD ["node", "server.js"]`. That looks like JSON because it is JSON, and
      it is the only JSON in the file.
  - pattern: '^WORKDIR\s'
    kind: regex
    weight: 8
    note: >
      Sets the folder for everything below it. The path is always a Linux path
      with forward slashes, even when you are building on Windows.
  - pattern: '^(ENV|ARG)\s+\w+='
    kind: regex
    weight: 6
    note: >
      `ENV KEY=value` sets an environment variable inside the image. It looks
      like a `.env` file line with a keyword bolted on the front, and that is
      close to what it is.
  - pattern: '\sAS\s+\w+\s*$'
    kind: regex
    weight: 5
    note: >
      `FROM node:22 AS builder` names a build stage, so a later stage can copy
      files out of it. Seeing several `FROM` lines in one file means it is a
      multi-stage build rather than a mistake.

rules_out:
  - pattern: '^\s+\w+:\s'
    kind: regex
    because: YAML, most likely the docker-compose.yml sitting in the same folder
  - pattern: '^#!'
    kind: line_start
    because: a shell script. Dockerfiles have no shebang line.
  - pattern: '^\s*\[[\w.-]+\]\s*$'
    kind: regex
    because: TOML or INI, which head their sections with square brackets
  - pattern: '<'
    kind: sigil
    because: XML or HTML
  - pattern: '^\s*(def|fn|func|function)\s'
    kind: regex
    because: a programming language. A Dockerfile has no functions.

project_fingerprint:
  manifests:
    - file: Dockerfile
      decisive: true
      note: >
        Usually no extension at all, just the word, capital D, in the project
        root. If it is there, the project can be built into a container image.
    - file: Dockerfile.*
      note: >
        A variant for one situation, as in `Dockerfile.dev` or
        `Dockerfile.prod`. Same syntax, different target.
    - file: '*.dockerfile'
      decisive: true
      note: The reversed naming, as in `dev.dockerfile`. Editors pick up the syntax highlighting from this form.
    - file: Containerfile
      note: Podman's name for exactly the same file. The contents are identical.
    - file: .dockerignore
      note: >
        Lists what to keep out of the build. It is not Dockerfile syntax: it uses
        `.gitignore` patterns, and it belongs to the gitignore card.
  build_dirs: []

shape:
  blocks: none
  statement_end: newline
  comment_line: '#'
  string_quotes: >
    Double quotes inside the bracketed `CMD` and `ENTRYPOINT` forms, because that
    part is JSON. Everywhere else the argument is passed to a shell and quoting
    follows shell rules.
  naming: >
    Instructions are written in capitals by convention, and lowercase works.
    Every real Dockerfile uses capitals, so match that.
  import_keyword: 'FROM, which pulls in a base image rather than a library'

tooling:
  registry: Docker Hub, at hub.docker.com
  runtime: Docker Desktop, which has to be installed and running on Windows before any of this works
  run_command: docker build -t my-app .
  test_command: docker run --rm my-app

confusable_with:
  - language: makefile
    settle_it: >
      Both are lists of shell commands with something in front of them. A
      Dockerfile puts an uppercase keyword first, `RUN npm ci`. A Makefile puts a
      `target:` line first and indents the commands under it with a real tab.
    tiebreak: { pattern: '^[A-Za-z_.][\w.-]*:\s*$', kind: regex, favors: makefile }
  - language: bash
    settle_it: >
      Strip the keywords off a Dockerfile and you have a shell script, which is
      why they look related. A shell script opens with `#!/bin/bash` and has no
      uppercase keywords at the start of lines.
    tiebreak: { pattern: '^#!/', kind: regex, favors: bash }
  - language: yaml
    settle_it: >
      They travel together and do different jobs. The Dockerfile builds one
      image. `docker-compose.yml` is YAML and says how to run several containers
      together. Keywords in capitals means Dockerfile, `key: value` means the
      compose file.
    tiebreak: { pattern: '^\s*services:\s*$', kind: regex, favors: yaml }

errors_look_like:
  sample: |
    => ERROR [4/7] RUN pnpm install --frozen-lockfile               2.3s
    ------
    > [4/7] RUN pnpm install --frozen-lockfile:
    0.9 ERR_PNPM_OUTDATED_LOCKFILE  Cannot install with frozen-lockfile
    ------
    failed to solve: process "/bin/sh -c pnpm install --frozen-lockfile" did not complete successfully: exit code: 1
  recognize_by: >
    A step counter in square brackets such as `[4/7]`, rows of dashes fencing the
    output, and the line `failed to solve:` at the bottom. The step number counts
    instructions from the top of the file, so it tells you which line broke.
  patterns:
    - 'failed to solve:'
    - '=> ERROR \[\d+/\d+\]'
    - 'did not complete successfully: exit code'
    - 'COPY failed: file not found in build context'
    - 'Cannot connect to the Docker daemon'

meet_it_when: >
  You clone a project that ships one so it runs the same on your machine as on
  the server, an agent offers to containerize something you built, or a hosting
  platform asks for a Dockerfile before it will deploy anything. It usually
  arrives with a `docker-compose.yml` beside it.

what_agents_get_wrong: >
  Two failures dominate. The first is `FROM ubuntu:latest`, or `node:latest`, or
  anything else tagged `latest`. That pins nothing: the same file builds a
  different image next month, and a build that worked yesterday can break today
  for a reason that appears nowhere in your diff. Ask for a real version tag. The
  second is instruction order. An agent writes `COPY . .` before the install
  step, which is the natural way to describe it in a sentence and the wrong way
  to write it, because every edit to any file in your project then invalidates
  the cached install layer and reinstalls every dependency on every build. The
  fix is to copy the manifest, install, then copy the rest. Agents also leave the
  default user as root and quietly add packages your image does not need. In a
  diff, check the `FROM` tag, check that `COPY . .` comes after the install line,
  and read every new `RUN` for something you did not ask for.

version_landscape: >
  The instruction set has been stable for years, so old answers still mostly
  work. What changed is the builder underneath. BuildKit is the default now, and
  its output looks nothing like the old numbered `Step 4/7` lines, so a
  screenshot from 2019 will not match what you see. Multi-stage builds and cache
  mounts are modern additions worth asking for by name.

see_also:
  - yaml
  - makefile
  - bash
  - gitignore
  - i1-what-deployment-means
  - i3-builds-and-artifacts
  - c3-what-running-means

keywords: [docker, container, image, layer, build cache, latest tag, multi-stage, dockerignore, buildkit]
---

A Dockerfile is a list of instructions for building a container image. It is not a
programming language. It has no functions, no loops, and no variables you can compute
with. It is a recipe, read from top to bottom, once, by the command `docker build`.

Two words first, because nothing else makes sense without them. An **image** is a frozen
snapshot of a filesystem plus a note about what to run. A **container** is one running copy
of an image. The Dockerfile builds the image, and the image produces containers.

## The shape

One instruction per line. The instruction is a word in capitals at the start of the line,
and everything after it is its argument.

```dockerfile
FROM node:22-alpine

WORKDIR /app

COPY package.json pnpm-lock.yaml ./
RUN corepack enable && pnpm install --frozen-lockfile

COPY . .
RUN pnpm build

EXPOSE 3000
CMD ["node", "dist/server.js"]
```

The contrast: nothing else on this deck looks like that. A Makefile opens with `target:`
lines and indents its commands with a tab. A shell script opens with `#!/bin/bash` and has
no keywords in front of its commands. YAML (YAML Ain't Markup Language) writes
`key: value`. Lines beginning with a capitalized `FROM`, `RUN`, or `COPY` are a Dockerfile.

The file is normally named `Dockerfile` with no extension whatsoever. Comments start
with `#`.

Four instructions cover most files. `FROM` names the starting image and comes first. `RUN`
executes a command while the image is being built. `COPY` brings files in from your project
folder. `CMD` records what should run when a container starts, which is a different moment
entirely.

## What it is for

Packaging an application together with everything it needs, so it behaves the same on your
Windows machine and on a Linux server you have never logged into. When a project holds both
a `Dockerfile` and a `docker-compose.yml`, the Dockerfile builds the image and the compose
file, which is YAML, says how to run it alongside a database and anything else.

## The gotchas

**Every instruction creates a layer.** A layer is a saved filesystem difference, and the
image is the stack of them. Layers are cached: if nothing feeding an instruction has
changed, Docker reuses the previous layer and skips the work entirely.

**Order decides whether that cache helps you.** Copy the dependency manifest, install
dependencies, then copy the rest of the code.

```dockerfile
COPY package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile
COPY . .
```

Put `COPY . .` first and every edit to any file in your project invalidates the install
layer, so you reinstall every dependency on every build. This is the most common Dockerfile
mistake there is and it costs minutes each time.

**`latest` is not a version.** `FROM ubuntu:latest` pins nothing at all. It means "whatever
was newest at the moment this built", so the same file gives you a different image next
month. Pin a real tag: `FROM ubuntu:24.04`.

**`CMD` does not run during the build.** It is recorded in the image and runs when a
container starts. A build that succeeds tells you nothing about whether the program starts.

**You are on Linux inside the image.** Forward slashes, `/app` rather than `C:\app`, and
filenames are case-sensitive. A `COPY` that works from your Windows folder can fail in the
build because `Utils.js` and `utils.js` are two different files in there.

**The build context is the whole folder.** `docker build .` uploads everything in the
current folder to the builder before it starts. If `node_modules` is sitting there, that is
slow for no reason. A `.dockerignore` file fixes it and uses the same patterns as
`.gitignore`.

## Reading its errors

```text
 => ERROR [4/7] RUN pnpm install --frozen-lockfile               2.3s
------
 > [4/7] RUN pnpm install --frozen-lockfile:
0.9 ERR_PNPM_OUTDATED_LOCKFILE  Cannot install with frozen-lockfile
------
failed to solve: process "/bin/sh -c pnpm install --frozen-lockfile" did not complete
successfully: exit code: 1
```

Recognize it by the step counter, `[4/7]`, and the line beginning `failed to solve:` at the
bottom. The step number counts instructions from the top of the file, so it names the line
that broke. The real error is the indented block between the two rows of dashes, and it
came from the program you ran, not from Docker.

One error is worth knowing on sight: `Cannot connect to the Docker daemon` means Docker
Desktop is not running. Start it and try again.
