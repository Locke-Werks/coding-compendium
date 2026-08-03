---
id: npx
title: npx
type: command
verified: 2026-08-02
volatility: quarterly

tool: npm
command: npx <package-name>
shell: any

does: >
  Runs a command-line tool from the npm registry without permanently installing it, fetching it
  first if you do not already have it.

flags:
  - flag: "<package-name>@<version>"
    means: >
      Runs a specific version, as in `npx create-vite@latest`. Worth pinning, since project
      generators change their questions between releases.
  - flag: "-y"
    means: >
      Answers yes to the "need to install the following packages" prompt. Convenient, and it
      removes the last moment you would have noticed a wrong package name.
  - flag: "--no-install"
    means: Runs the tool only if it is already available locally, and fails rather than downloading anything.
  - flag: "-- <arguments>"
    means: Everything after the bare `--` is passed to the tool rather than read by npx.

expect: >
  Possibly a prompt asking to install, then the tool's own output. Nothing is added to
  `package.json`, which you can confirm with `git status`.

see_also:
  - pnpm-dlx
  - npm-install
  - npm-run
  - g2-package-managers
  - g7-dependency-risk

keywords:
  - run without installing
  - create react app
  - scaffold a project
  - npx command not found
---

`npx` prefers a copy already inside the project's `node_modules`, and only downloads when there
is none. That is why the same command can be instant in one folder and slow in another.

It downloads and runs code from the internet. Read the package name carefully before you accept
the prompt, since one wrong letter can be a different author's package entirely.
