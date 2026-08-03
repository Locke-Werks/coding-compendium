---
id: g2-package-managers
title: Package managers, one per ecosystem
type: section
track: G
order: 20
verified: 2026-08-02
volatility: quarterly
verify: npm --version
answer: >
  A package manager is the tool that downloads your project's dependencies and
  records them, and there is a different one per language: npm or pnpm for
  JavaScript, pip or uv for Python, cargo for Rust, NuGet for C#.
owns:
  - package manager
  - registry
  - install vs add
see_also:
  - g1-what-a-dependency-is
  - g3-lockfiles
  - g4-environments-and-isolation
  - j1-how-to-recognize-a-language
  - c3-what-running-means
keywords:
  - npm vs pnpm
  - pip
  - cargo
  - nuget
  - what installs my packages
  - package registry
  - yarn
  - uv
---

## More

Every language has one tool that downloads dependencies, keeps a record of what it
downloaded, and installs them where that language expects to find them. Only the names
differ.

| Language | Manager | Manifest | Add one package | Install everything |
|---|---|---|---|---|
| JavaScript | `npm`, `pnpm`, `yarn` | `package.json` | `npm install <name>` | `npm install` |
| Python | `pip`, `uv`, `poetry` | `requirements.txt` or `pyproject.toml` | `pip install <name>` | `pip install -r requirements.txt` |
| Rust | `cargo` | `Cargo.toml` | `cargo add <name>` | `cargo build` |
| C# | `dotnet` with NuGet | the `.csproj` file | `dotnet add package <name>` | `dotnet restore` |
| Go | `go` itself | `go.mod` | `go get <name>` | `go mod download` |
| Ruby | `bundler` | `Gemfile` | `bundle add <name>` | `bundle install` |

Two commands that look the same and are not:

- `npm install` with nothing after it means "install everything this project already
  lists." You run it after cloning a project or after switching branches.
- `npm install <name>` means "fetch this new package and write it into the manifest." That
  changes the project for everybody.

The trap is Python. `pip install requests` installs the package and records it nowhere. The
next person clones your project and does not get it, because nothing in the repository ever
mentioned it. If you use plain `pip`, you have to add the line to `requirements.txt`
yourself. `uv` and `poetry` exist largely to fix this, and they write the manifest for you.

The **registry** is the public server the manager downloads from: npmjs.com for JavaScript,
PyPI for Python, crates.io for Rust, nuget.org for C#. Anyone can publish to any of them,
and nobody checks what they publish, which is [g7](#g7-dependency-risk).

One thing these are not: `winget` installs Windows applications, not project dependencies.
Different job, different card ([b4](#b4-github-and-gh)).

## Full

### Which one this project uses

Look at the folder, not at your preferences. The lockfile answers it in one glance:
`package-lock.json` means `npm`, `pnpm-lock.yaml` means `pnpm`, `yarn.lock` means `yarn`,
`uv.lock` means `uv`, `poetry.lock` means `poetry`. Use the one the project already uses.
Mixing two managers in one JavaScript project produces two disagreeing lockfiles and a
tree that neither tool can reason about.

If there is no lockfile and no clear signal, `npm` is the safe default for JavaScript and
`pip` inside a virtual environment is the safe default for Python.
[j1](#j1-how-to-recognize-a-language) covers reading a project's shape in general.

### The JavaScript three

`npm` ships with Node and is the default. `pnpm` does the same job while storing one copy
of each package on disk and linking it into every project, which saves real disk space and
installs faster. `yarn` was the fast one first and is now mostly seen in older projects.

They are close enough that you can read `npm` instructions and run `pnpm` commands, with
one predictable difference: `pnpm add <name>` where `npm` says `npm install <name>`.
`pnpm install` with no arguments still means install everything.

### Installing a tool versus installing a library

These are different actions that share a verb, and confusing them is the source of a lot of
broken machines.

```powershell
npm install prettier
```

A **project** install. It lands in this project's `node_modules`, gets recorded in this
project's manifest, and is invisible to every other project.

```powershell
npm install -g prettier
```

A **global** install. `-g` puts it on your PATH as a command you can run anywhere. Correct
for tools you type at the terminal, wrong for anything your code imports, because now the
project works on your machine and nowhere else. Same story with `pip install --user`,
`cargo install`, and `dotnet tool install`. [g4](#g4-environments-and-isolation) is the
card about why this matters more in Python than anywhere else.

Rule of thumb: if you type its name at a prompt, global is fine. If your code imports it,
it belongs in the project.

### Reading what happened

Installs print a lot and most of it is noise. The parts worth reading:

- **`added 214 packages`**. You asked for one. The other 213 are transitive, and that is
  normal ([g1](#g1-what-a-dependency-is)).
- **`npm WARN deprecated`**. A package inside the tree is unmaintained. It installed, it
  works, and you usually cannot do anything about it because it is somebody else's
  dependency, not yours.
- **`found 0 vulnerabilities`** or a count. Advisory noise more often than a real problem.
  [g7](#g7-dependency-risk) explains how much attention it deserves.
- **A red block ending in `ERR!`**. That is a failure and the install did not happen. Read
  the last line first ([f1](#f1-how-to-read-an-error-message)).

An install that fails halfway leaves a partial tree. Delete the dependency folder and run
the install again before you debug anything else ([g1](#g1-what-a-dependency-is)).

### The commands you will actually use

```powershell
npm install
```

After every clone, after every branch switch that changed the manifest, and any time
imports stop resolving. It is idempotent, so running it when you did not need to costs
nothing but time.

```powershell
npm outdated
```

Lists packages with newer versions available, alongside what your manifest allows. Reading
it is free. Acting on it is a decision, and upgrading everything at once is on the list of
things not to hand an agent unsupervised ([e11](#e11-what-to-never-let-an-agent-do)).

```powershell
npm uninstall <name>
```

Removes the package and its line in the manifest. The equivalents are `pip uninstall`,
`cargo remove`, and `dotnet remove package`. Removing a package does not remove its
transitive dependencies if something else still needs them, which is correct behavior even
though the disk savings can be disappointing.

### Version numbers in the manifest

A manifest names a range rather than one exact version, written with a caret or a tilde in
front of the number. [i4](#i4-releases-and-versioning) reads the operators.

Those ranges are why two people can install the same manifest and get different code, and
they are the entire reason lockfiles exist. [g3](#g3-lockfiles) is next.
