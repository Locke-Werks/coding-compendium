---
id: g1-what-a-dependency-is
title: What a dependency is
type: section
track: G
order: 10
verified: 2026-08-02
volatility: low
verify: npm ls --depth=0
danger: >
  `Remove-Item node_modules -Recurse -Force` deletes the entire dependency
  folder with no undo. It is safe here only because every file in it was
  downloaded from your manifest and lockfile, so the reinstall puts it all
  back. Never point that command at a folder holding code you wrote.
answer: >
  A dependency is code somebody else wrote that your project needs to run: you
  name a handful in your manifest, the package manager downloads those plus
  everything they depend on, and that is why the folder holds thousands of files.
owns:
  - dependency
  - library
  - transitive dependency
see_also:
  - g2-package-managers
  - g3-lockfiles
  - g7-dependency-risk
  - d12-gitignore-and-what-not-to-commit
  - g4-environments-and-isolation
keywords:
  - what is node modules
  - why is this folder so big
  - thousands of files appeared
  - site-packages
  - vendor folder
  - third party code
  - what is a library
---

## More

A date-formatting library, a web framework, a testing tool. Each one is a **dependency**,
code somebody else wrote that your project cannot run without. You name it in your project's
**manifest**, the file listing what the project requires, and a package manager downloads
it for you. [g2](#g2-package-managers) covers the managers, one per language.

You will name about twelve. You will get four hundred.

That gap is the thing to understand. Every library you install has its own dependencies,
and those have theirs, all the way down. A dependency of a dependency is a **transitive
dependency**: you never asked for it, you cannot name it, and it ships inside your project
anyway. Twelve direct dependencies pulling four hundred packages in total is an ordinary
result for a JavaScript project.

Which is why the folder is enormous. `node_modules` routinely holds tens of thousands of
files and a few hundred megabytes. Python puts the same thing in `site-packages`, Rust
keeps a shared cache under your user folder, and none of them are small. Nothing has gone
wrong. That folder is the installed form of the twelve lines you wrote.

Three consequences you will use constantly:

- **The folder is disposable.** Every byte in it came from a registry and can be
  downloaded again from your manifest and lockfile. Deleting it and reinstalling is a
  normal repair rather than a last resort.
- **It never goes into git.** It is huge, it is regenerable, and it is specific to your
  machine. `.gitignore` keeps it out, which is [d12](#d12-gitignore-and-what-not-to-commit).
  You commit the lockfile instead: [g3](#g3-lockfiles).
- **You own the manifest, not the tree.** You decide the twelve. The manager decides the
  rest, and it will change them under you when versions move.

Adding a dependency is a real decision with a real cost, and [g7](#g7-dependency-risk) is
the card about which ones are worth taking.

## Full

### The two files that describe your dependencies

The **manifest** is short, hand-edited, and yours: `package.json`, `pyproject.toml`,
`Cargo.toml`, `requirements.txt`, `Gemfile`. It names what you want, usually as a range
rather than an exact version.

The **lockfile** is long, machine-written, and nobody's. It records the exact version of
every package you actually received, transitive ones included, and it is
[g3](#g3-lockfiles).

The installed folder is the third thing, and it is derived from the other two. That is the
whole reason it is safe to delete.

### Where they land, per ecosystem

| Ecosystem | Manifest | Installed into |
|---|---|---|
| JavaScript | `package.json` | `node_modules\` in the project |
| Python | `pyproject.toml` or `requirements.txt` | `.venv\Lib\site-packages\` in the project |
| Rust | `Cargo.toml` | `target\` plus a shared cache in `C:\Users\<yourname>\.cargo\` |
| C# | the `.csproj` file | a shared cache in `C:\Users\<yourname>\.nuget\packages\` |
| Go | `go.mod` | a shared module cache |
| Ruby | `Gemfile` | a shared gem folder |

Notice the split. JavaScript and Python copy a full set of packages into every project,
which costs disk and keeps projects isolated from each other. Rust, C#, Go, and Ruby keep
one shared cache and point projects at it. Both designs work, and it explains why
`node_modules` feels alarming while `cargo` never seems to download anything twice.

### Direct, transitive, and why the count explodes

```powershell
npm ls --depth=0
```

Lists only the packages you asked for. Drop the flag and you get the whole tree, which
scrolls for a while.

The JavaScript ecosystem in particular favors tiny packages, sometimes one function each,
so the count runs high by design. A large number is not by itself a sign that something is
wrong, and a small number is not a sign of quality. What matters is which packages you
chose directly, because those are the only ones you decided on.

### Deleting and reinstalling is a normal repair

When installs behave strangely, when a package is present but not found, or when you have
switched branches into a different set of dependencies, the standard move is to throw the
folder away and let the manager rebuild it:

```powershell
Remove-Item node_modules -Recurse -Force
npm install
```

`-Recurse` means include everything inside, `-Force` means do not ask about read-only or
hidden files. There is no undo, and none is needed: the manager downloads all of it again
from your manifest and lockfile. Give it a minute or two. Verify by running the command
that was failing.

The Python version of the same repair is deleting the virtual environment and recreating
it, which is [g4](#g4-environments-and-isolation).

### What one dependency actually costs

It is not free, and the cost is not the download.

- It can break in a version you did not choose to install.
- It can be abandoned, leaving you with code nobody will fix.
- It brings its own dependencies, and you now ship all of them.
- It becomes yours to update, forever, or to remove later at some effort.

The honest other side: writing your own date parser or your own encryption is worse than
any of that. A useful line to draw is whether you could write the thing correctly. Ten
lines of string handling, write it. Time zones, encryption, network protocols, take the
library. [g7](#g7-dependency-risk) covers judging a specific package.

### The words people use for the same thing

**Library**, **package**, **dependency**, **crate** in Rust, **gem** in Ruby, **module**
in several places. In practice these are interchangeable when someone is talking about
code you installed. Only **module** means something narrower and language-specific, and it
usually refers to one file inside a package rather than the whole package.
