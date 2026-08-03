---
id: dotnet-build
title: dotnet build
type: command
verified: 2026-08-02
volatility: low

tool: dotnet
command: dotnet build
shell: any

does: >
  Compiles a C# project or solution and everything it depends on, writing the result into the
  `bin` folder.

flags:
  - flag: "-c Release"
    means: >
      Short for `--configuration`. Builds the optimized version instead of the debug one. The
      output moves from `bin\Debug` to `bin\Release`, and debug symbols are trimmed. Use it for
      anything you hand to someone else.
  - flag: "--no-restore"
    means: >
      Skips downloading dependencies, assuming they are already present. Slightly faster, and it
      fails confusingly if they are not.
  - flag: "-v <level>"
    means: >
      Short for `--verbosity`. Accepts `quiet`, `minimal`, `normal`, `detailed`, `diagnostic`.
      Raise it when a build fails without telling you why.
  - flag: "-o <folder>"
    means: Writes the output somewhere other than the default `bin` path.

expect: >
  `Determining projects to restore...`, compilation lines, then
  `Build succeeded.` with warning and error counts. Errors are printed with the file, line, and
  a code such as `CS0103`.

see_also:
  - dotnet-run
  - csharp
  - i3-builds-and-artifacts
  - c2-compiled-vs-interpreted

keywords:
  - build c sharp
  - compile dotnet
  - build succeeded
  - bin folder
---

`dotnet build` implies a restore, so you rarely need `dotnet restore` on its own.

Run it from the folder holding the `.csproj` or `.sln` file. From anywhere else it reports that
it found no project to build, which reads like a broken installation and is not.
