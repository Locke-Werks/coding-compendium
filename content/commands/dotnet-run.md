---
id: dotnet-run
title: dotnet run
type: command
verified: 2026-08-02
volatility: low

tool: dotnet
command: dotnet run
shell: any

does: >
  Builds a C# project if anything changed and then runs it, without you having to find the
  compiled executable yourself.

flags:
  - flag: "-c Release"
    means: Builds and runs the optimized version rather than the debug one. Needed whenever you are judging speed.
  - flag: "--project <path>"
    means: Runs a project in another folder, so you do not have to move there first.
  - flag: "--no-build"
    means: Runs the existing compiled output and skips the build. Fast, and it silently runs stale code if you forgot to build.
  - flag: "-- <arguments>"
    means: >
      Everything after the bare `--` goes to your program rather than to the dotnet tool, as in
      `dotnet run -- --input data.csv`.

expect: >
  Build output first, then your program's own output. A web project prints the addresses it is
  listening on, such as `Now listening on: http://localhost:5000`, and holds the terminal until
  you press Ctrl+C.

see_also:
  - dotnet-build
  - csharp
  - c3-what-running-means
  - c6-ports-and-localhost

keywords:
  - run c sharp project
  - start dotnet app
  - dotnet run arguments
  - localhost 5000
---

If the folder contains more than one runnable project, the command stops and asks you to name
one with `--project`. That is not an error in your code.
