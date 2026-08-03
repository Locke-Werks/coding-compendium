---
id: pnpm-dlx
title: pnpm dlx
type: command
verified: 2026-08-02
volatility: quarterly

tool: pnpm
command: pnpm dlx <package-name>
shell: any

does: >
  Downloads a command-line tool, runs it once, and discards it, without adding anything to your
  project or installing anything permanently.

flags:
  - flag: "dlx"
    means: >
      Short for download and execute. The pnpm equivalent of `npx`. The package lands in a
      temporary cache rather than in `node_modules` or `package.json`.
  - flag: "<package-name>@<version>"
    means: >
      Runs a specific version, as in `pnpm dlx create-vite@latest`. Worth pinning when a tool's
      newest release changes its questions.
  - flag: "-- <arguments>"
    means: Everything after the bare `--` is passed to the tool rather than read by pnpm.

expect: >
  A short download line, then the tool's own output. Nothing is added to `package.json` and no
  lockfile changes, which you can confirm with `git status`.

see_also:
  - npx
  - pnpm-add
  - pnpm-install
  - g2-package-managers

keywords:
  - run a tool without installing
  - npx equivalent
  - scaffold a project
  - one off command
---

This is how project generators are meant to be run. `pnpm dlx create-vite@latest` builds a new
project without leaving the generator installed anywhere afterward.

It downloads and executes code from the internet on your machine. Check the package name is the
one you meant, since a single transposed letter can be somebody else's package.
