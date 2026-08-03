---
id: c3-what-running-means
title: What "run it" actually means
type: section
track: C
order: 30
verified: 2026-08-02
volatility: quarterly
verify: npm run
answer: >
  There is no universal run button, because the command that starts a project is
  written down inside the project itself, so open its manifest file and read the
  command out of there instead of guessing.
owns:
  - entry points
  - the run command per ecosystem
see_also:
  - c2-compiled-vs-interpreted
  - j3-project-layouts
  - c6-ports-and-localhost
  - c4-path-and-command-not-found
  - j4-reading-a-repo-you-did-not-write
keywords:
  - how do i run this
  - npm run dev
  - cargo run
  - how to start the project
  - which command starts it
  - dev server
  - it just hangs
---

## More

"Run it" is two questions wearing one coat: which command, and from which folder.

The command is a property of the project, not the language. Two Python projects on your disk
can start with two different commands, and both are correct. This is why nobody can tell you
the answer without looking, and why the answer is always written down in a file called the
**manifest**: the one file that describes the project to its own tooling.

| You see this file | The project is | Start it with |
|---|---|---|
| `package.json` | JavaScript or TypeScript | `npm run dev`, or whatever its `scripts` block says |
| `Cargo.toml` | Rust | `cargo run` |
| `pyproject.toml` or `requirements.txt` | Python | `python main.py`, or the command in the README |
| `go.mod` | Go | `go run .` |
| `.csproj` or `.sln` | C# | `dotnet run` |
| `pom.xml` or `build.gradle` | Java | `mvn spring-boot:run` or `./gradlew bootRun` |

For a JavaScript project you do not have to guess which script, because the tool will list
them:

```powershell
npm run
```

Run with no script name, it prints every script the project defines. `dev` is the
conventional name for the one you want while working, `start` for the production one, and
`build` produces files rather than running anything.

The second question matters as much. Run the command from the folder that contains the
manifest. From anywhere else you get `Could not read package.json` or `error: could not find
Cargo.toml`, which reads like the project is broken and means you are standing in the wrong
place.

What success looks like depends on the kind of program. A script prints something and gives
you your prompt back. A **dev server** prints an address like `http://localhost:3000` and
then sits there doing nothing visible. The second one is not frozen. That is what running
looks like for a server, and [c6](#c6-ports-and-localhost) covers the address it printed.

## Full

### The order to look in

1. **The README.** It is usually right and it is usually stale in exactly one place, the
   version numbers.
2. **The manifest.** `package.json` has a `scripts` block that is the literal truth about
   what this project can do.
3. **The CI (Continuous Integration) config**, in `.github\workflows\`. It contains the
   exact commands that run on a clean machine, which makes it the least wrong file in the
   repository. [h5](#h5-ci-cd) covers reading it.

### Reading a scripts block

```json
{
  "scripts": {
    "dev": "vite",
    "build": "tsc -b && vite build",
    "test": "vitest run",
    "lint": "eslint ."
  }
}
```

`npm run dev` does not mean anything on its own. It means "look up `dev` in this block and
run whatever string is there", which here is `vite`. That indirection is the whole point: the
project can change how it starts without changing what you type.

Two shortcuts exist for historical reasons. `npm test` and `npm start` work without the word
`run`. Everything else needs it, and `npm dev` is a common typo that produces
`Unknown command`.

### Before the first run, once per project

A freshly cloned project has source code and no libraries. The first command is the one that
downloads them into the project folder:

```powershell
npm install
```

Rust and Go do this as part of the build, so `cargo run` fetches what it needs on its own.
Python needs an isolated environment set up first, which is [g4](#g4-environments-and-isolation).
[g2](#g2-package-managers) is the map of which tool does this in which ecosystem.

Skipping this step is the most common cause of `Cannot find module` on a project that
demonstrably works for everyone else.

### The entry point

Inside the code, one specific place runs first.

- Rust, Go, C, and Java start at a function named `main`.
- Node starts at the file named on the command line, or the one named in the `main` field of
  `package.json`.
- Python runs the named file from the top down, executing every line as it meets it. The
  `if __name__ == "__main__":` block near the bottom of many files is a guard meaning "only
  do this when I am the file being run, not when I am being imported by something else."

You rarely need to touch the entry point. Knowing it exists explains why a file full of
correct code can produce no output at all: nothing called it.

### The server that will not exit is working

When a dev server starts it stays running on purpose. It holds a port, watches your files,
and rebuilds when you save. Three things follow that surprise people:

- **Ctrl+C in that window stops it.** Closing the tab usually does too, and not always. See
  [c5](#c5-processes-and-killing-them).
- **You need a second terminal** to run anything else while it is up. Windows Terminal opens
  a new tab with Ctrl+Shift+T.
- **Starting it twice fails** with `EADDRINUSE`, because the first one still holds the port.
  That one is a certainty rather than a risk, and [c6](#c6-ports-and-localhost) is the fix.

### When the command fails before the program starts

Sort it by which stage broke.

- `'npm' is not recognized` or `'cargo' is not recognized`: the tool itself was not found.
  Nothing about your project is involved. [c4](#c4-path-and-command-not-found).
- `Could not read package.json` or `no such file or directory`: wrong folder. Check where
  you are with `Get-Location` and see [c7](#c7-files-folders-and-paths).
- `Missing script: "dev"`: the tool ran, read the manifest, and the script genuinely is not
  there. Run `npm run` to see the real list.
- A wall of red from inside `node_modules\` on the first run after cloning: you skipped the
  install step above.

### Agents and the run command

An agent will usually work this out for itself by reading the manifest, and it will
occasionally invent a command that sounds right and is not, especially `npm run start` for a
project that only defines `dev`. When it claims a project runs, ask which file it read the
command out of. That question costs one line and catches the invented answer immediately
([e7](#e7-agent-failure-modes)).
