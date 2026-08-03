---
id: pnpm-add
title: pnpm add
type: command
verified: 2026-08-02
volatility: quarterly

tool: pnpm
command: pnpm add <package-name>
shell: any

does: >
  Installs a new dependency and records it in the project's `package.json` and lockfile.

flags:
  - flag: "-D"
    means: >
      Short for `--save-dev`. Records the package under `devDependencies`, meaning it is needed
      to build or test the project rather than to run it.
  - flag: "-E"
    means: >
      Short for `--save-exact`. Pins the precise version instead of recording a range that
      allows automatic minor updates.
  - flag: "-w"
    means: >
      Short for `--workspace-root`. Adds the dependency to the root of a multi-package
      repository rather than to one package inside it. Without it, pnpm refuses at the root and
      tells you to choose.
  - flag: "-g"
    means: >
      Installs a command-line tool for your whole machine instead of this project. Rarely the
      right call for a library.
  - flag: "<package-name>@<version>"
    means: Installs a specific version, as in `pnpm add react@18.3.1`.

expect: >
  A summary of what changed, ending with a `dependencies:` block listing the package and the
  version it settled on, then `Done in 2.4s`.

see_also:
  - pnpm-install
  - pnpm-dlx
  - npm-install
  - g1-what-a-dependency-is
  - g7-dependency-risk

keywords:
  - add a package
  - install a library
  - new dependency
  - pnpm install package
---

Check the package name against the registry before running this, especially when an agent
suggested it. Agents invent plausible package names, and an attacker who registers one of those
names gets their code installed by anyone who trusts the suggestion.
