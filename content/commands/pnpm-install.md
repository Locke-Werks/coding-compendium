---
id: pnpm-install
title: pnpm install
type: command
verified: 2026-08-02
volatility: quarterly

tool: pnpm
command: pnpm install
shell: any

verify: pnpm --version

does: >
  Downloads every dependency listed in the project's `package.json`, storing one copy of each
  package on your machine and linking it into the project instead of copying it.

flags:
  - flag: "--frozen-lockfile"
    means: >
      Fails rather than updating `pnpm-lock.yaml` if the lockfile does not match
      `package.json`. This is the equivalent of `npm ci` and the right choice when you want a
      reproducible install.
  - flag: "-r"
    means: >
      Short for `--recursive`. Installs for every package in a workspace, which is a repository
      holding several related projects in one tree.
  - flag: "--offline"
    means: Installs only from the local store, with no network access. Fails if anything is missing.
  - flag: "--prod"
    means: Skips development dependencies. For building something to ship, not for working locally.

expect: >
  A progress line such as
  `Progress: resolved 312, reused 300, downloaded 12, added 312`, then `Done in 4.1s`. A
  `node_modules` folder appears, full of links rather than copies.

see_also:
  - pnpm-add
  - pnpm-dlx
  - npm-install
  - g3-lockfiles
  - g2-package-managers

keywords:
  - install dependencies pnpm
  - pnpm lockfile
  - node_modules missing
  - faster npm
---

`pnpm` does the same job as `npm` with a different storage strategy: one copy of each package
version lives in a shared store on your drive, and each project links to it. Installs are
faster and take far less disk space.

A project chooses one package manager and sticks to it. The lockfile name tells you which:
`pnpm-lock.yaml` for pnpm, `package-lock.json` for npm. Mixing them produces conflicting
dependency trees.
