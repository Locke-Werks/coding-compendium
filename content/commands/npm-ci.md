---
id: npm-ci
title: npm ci
type: command
verified: 2026-08-02
volatility: quarterly

tool: npm
command: npm ci
shell: any

does: >
  Installs the exact dependency versions recorded in `package-lock.json`, deleting the existing
  `node_modules` folder first so the result is identical every time.

flags:
  - flag: "ci"
    means: >
      Stands for continuous integration, the automated system that runs your tests when you
      push. The name describes where it was designed to run, and it is equally useful locally
      when you want a clean, exactly reproducible install.
  - flag: "--omit=dev"
    means: >
      Skips development-only dependencies, giving a smaller install for a production build. Do
      not use it locally, since your test tools live there.

expect: >
  `added 312 packages in 5s`, usually faster than `npm install` because it does no version
  resolution. It prints no summary of changes to `package.json`, because it never changes it.

see_also:
  - npm-install
  - g3-lockfiles
  - h5-ci-cd

keywords:
  - clean install
  - reproducible install
  - lockfile out of sync
  - fix broken node_modules
---

Two differences from `npm install` matter. This command never writes to `package.json` or the
lockfile, and it fails outright if the two files disagree rather than quietly fixing them. That
failure is a feature: it tells you someone changed a dependency without committing the updated
lockfile.

Reach for it when a project behaves strangely after a dependency change and you want to rule
out a stale `node_modules` folder.
