---
id: npm-install
title: npm install
type: command
verified: 2026-08-02
volatility: quarterly

tool: npm
command: npm install
shell: any

verify: npm --version

does: >
  Downloads every dependency listed in the project's `package.json` into a `node_modules`
  folder, so the project can run.

flags:
  - flag: "<package-name>"
    means: >
      Adds one new dependency and records it in `package.json`, as in `npm install react`.
      With no package name, it installs what the file already lists.
  - flag: "-D"
    means: >
      Short for `--save-dev`. Records the package under `devDependencies`, meaning it is
      needed to build or test the project and not to run it. Test frameworks and type
      definitions belong here.
  - flag: "-g"
    means: >
      Installs the package for your whole machine rather than this project. Reserve it for
      command-line tools. A project dependency installed this way works on your machine and
      nowhere else.
  - flag: "--legacy-peer-deps"
    means: >
      Ignores conflicts between packages that disagree about which version of a shared
      dependency they need. It makes the error go away without resolving the disagreement, so
      treat it as a temporary answer.
  - flag: "--save-exact"
    means: Pins the exact version instead of recording a range that allows future minor updates.

expect: >
  A progress display, then a summary such as `added 312 packages in 8s`. A line about
  vulnerabilities is normal and rarely urgent.

see_also:
  - npm-ci
  - npm-run
  - npx
  - g1-what-a-dependency-is
  - g3-lockfiles

keywords:
  - install dependencies
  - node_modules missing
  - cannot find module
  - npm i
---

`npm i` is the same command, abbreviated.

The `node_modules` folder is enormous, regenerable, and must never be committed. If it is
missing, this command is the fix. `package-lock.json` is the opposite: it records the exact
versions you got, and it does get committed.
