---
id: i3-builds-and-artifacts
title: Builds and artifacts
type: section
track: I
order: 30
verified: 2026-08-02
volatility: low
danger: >
  `Remove-Item -Recurse -Force` deletes a folder and everything under it with no
  Recycle Bin and no undo. Pointed at a build folder it costs you one rebuild.
  Pointed at the wrong folder it costs you your work. Read the path in the
  command before pressing Enter, and use the tool's own clean command instead
  wherever one exists.
answer: >
  A build turns the files you edit into the files that run, and the artifact is
  what comes out. It lands in a generated folder like `target` or `dist`, which
  is git-ignored and always safe to delete, because the next build recreates it.
owns:
  - build
  - artifact
  - build directory
see_also:
  - c2-compiled-vs-interpreted
  - d12-gitignore-and-what-not-to-commit
  - i1-what-deployment-means
  - i4-releases-and-versioning
  - g3-lockfiles
keywords:
  - dist folder
  - target folder
  - build output
  - what is an artifact
  - clean build
  - stale build
  - bundle
---

## More

The output of a **build** is called an **artifact**: a single `.exe`, a folder of minified
JavaScript, an installer, a container image. The build is the step that reads the files you
edit and writes those instead.

Not every project has one. A Python script runs from the file you typed, so there is nothing
to build. A Rust program has to be compiled before anything can run at all. The dividing
line is [c2](#c2-compiled-vs-interpreted), and it explains why some projects have a `target`
folder and others never grow one.

What a build does, depending on the ecosystem:

- **Compiles.** Source into machine code or bytecode.
- **Bundles.** Hundreds of small source files into a handful of large ones, so a browser
  makes three requests instead of eight hundred.
- **Transforms.** TypeScript into JavaScript, modern syntax into older syntax, image files
  into smaller image files.
- **Copies.** Static files that needed nothing done to them into the output folder alongside
  everything else.

The output has a conventional home and the name varies by ecosystem: `target` in Rust,
`dist` or `build` in JavaScript, `bin` and `obj` in C#, `build` in Java and Python
packaging. Whatever it is called, three things are true of it. It is generated, so nothing
in it was written by a person. It is listed in `.gitignore`, so it never reaches GitHub
([d12](#d12-gitignore-and-what-not-to-commit)). And it is disposable, because everything
inside it can be rebuilt from the source that is committed.

One folder that looks like build output and is not: `node_modules`. That is downloaded
dependencies, not something your build produced ([g1](#g1-what-a-dependency-is)). It is
also git-ignored and also safe to delete, for a different reason: the package manager can
fetch it all again from the lockfile ([g3](#g3-lockfiles)).

## Full

### The command per ecosystem, and where the output lands

| Ecosystem | Build command | Output folder | Artifact |
|---|---|---|---|
| Rust | `cargo build --release` | `target\release\` | A single `.exe` |
| Node with Vite | `pnpm build` | `dist\` | Static files a host can serve |
| TypeScript alone | `pnpm exec tsc` | `dist\` or beside the source | `.js` files |
| C# | `dotnet build -c Release` | `bin\Release\` | `.dll` and `.exe` |
| Python package | `python -m build` | `dist\` | A `.whl` and a `.tar.gz` |
| Tauri desktop app | `pnpm tauri build` | `src-tauri\target\release\bundle\` | An installer |

The build command is written down in the project's manifest or its scripts block, which is
why [c3](#c3-what-running-means) tells you to look there first rather than guessing.

### Debug builds and release builds

Nearly every compiler has two modes and the difference is large enough to notice.

A **debug** build compiles fast, runs slowly, and keeps the information a debugger needs to
tell you which line of your source a crash came from. This is what you get by default and
what you want while working.

A **release** build compiles slowly, runs fast, and strips most of that information out. The
binary is smaller and the error messages are less specific. This is what you ship. If you
ever see someone report that a Rust program is slow, the first question is whether they
built it with `--release`, and about a third of the time that is the entire problem.

### Why the first build takes ten minutes and the second takes four seconds

Build tools cache. The first run compiles every dependency you have, which for a Rust
project can be several hundred crates. The results are stored in the output folder, and the
next build recompiles only what changed. A `target` folder reaching several gigabytes is
normal and is not a mistake.

This is also why deleting the output folder is safe but not free: safe because everything in
it is reproducible, expensive because you pay the ten minutes again.

### The stale build, and the symptom that gives it away

You change a file, run the program, and the old behavior is still there. Nothing in the
terminal looks wrong.

Usually this means you ran the artifact rather than rebuilding it, or the dev server did not
notice the file change. Occasionally the cache is genuinely confused. The ladder, cheapest
first:

1. Stop the program and start it again. Many dev servers watch only some folders.
2. Do a hard refresh in the browser with `Ctrl+Shift+R`, which ignores the browser's own
   cache of the old files.
3. Run the tool's clean command, then build again.

```powershell
cargo clean
```

Deletes the Rust build folder and everything cached in it. `dotnet clean` and
`pnpm exec rimraf dist` are the equivalents elsewhere. Where no clean command exists, delete
the folder by hand:

```powershell
Remove-Item -Recurse -Force .\dist
```

`-Recurse` means include everything inside, `-Force` means do not ask. This does not use the
Recycle Bin, so read the path before you run it. Aimed at `dist` you lose a rebuild. Aimed
at `src` you lose your work.

If a clean rebuild fixes the problem, the cache was stale. If it does not, the problem is in
your code and you have ruled out an entire category in ninety seconds.

### Artifacts in a pipeline, and why the word appears there too

CI (Continuous Integration) is the automation that builds and tests your project when you
push ([h5](#h5-ci-cd)). It runs on a machine that is destroyed when the job finishes, so
anything you want to keep has to be uploaded, and the button that keeps it is labeled
"artifacts." That is the same word for the same thing: the files the build produced.

This is how a project gives you a downloadable `.exe` without anyone building it by hand,
and it is how a release gets its attached files ([i4](#i4-releases-and-versioning)).

### The rule underneath all of this

**Anything that can be regenerated from what is committed does not get committed.** Build
output, downloaded dependencies, caches, and logs are all regenerable. Source files, the
manifest, and the lockfile are not.

The test when you are unsure: delete it, run the build, and see whether it comes back. If it
does, it belongs in `.gitignore`. If it does not, you have found something that should have
been committed all along.
