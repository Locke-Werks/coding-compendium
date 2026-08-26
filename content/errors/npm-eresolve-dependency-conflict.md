---
id: npm-eresolve-dependency-conflict
title: "npm error ERESOLVE unable to resolve dependency tree"
type: error
verified: 2026-08-02
volatility: quarterly

language: javascript
category: conflict

# Lists the installed tree one level deep. It reports any unmet or conflicting
# dependency it finds.
verify: npm ls --depth=0

danger: >
  One step deletes `node_modules` and `package-lock.json`. Nothing you wrote lives in either
  and `npm install` regenerates both. The real cost is that the new lockfile may pick
  different versions than before, so do it on a clean commit and check that the project still
  runs afterward.

sample: |
  PS C:\Users\you\dev\site> npm install react-chartjs-2
  npm error code ERESOLVE
  npm error ERESOLVE unable to resolve dependency tree
  npm error
  npm error While resolving: site@0.1.0
  npm error Found: react@19.2.8
  npm error node_modules/react
  npm error   react@"^19.2.8" from the root project
  npm error
  npm error Could not resolve dependency:
  npm error peer react@"^18.0.0" from react-chartjs-2@5.2.0
  npm error node_modules/react-chartjs-2
  npm error
  npm error Fix the upstream dependency conflict, or retry
  npm error this command with --force or --legacy-peer-deps

patterns:
  - "ERESOLVE"
  - "unable to resolve dependency tree"
  - "Could not resolve dependency"
  - "peer dep"
  - "legacy-peer-deps"

means: >
  Two packages disagree about which version of a third package they need. The one you are
  installing declares a peer dependency, which is its way of saying "I work with version 18 of
  React and you have to supply it". Your project has version 19. npm will not silently install
  a combination the packages themselves say is wrong, so it stops. Nothing was installed and
  nothing changed.

fix_ladder:
  - try: Read which two packages disagree and about what.
    why: >
      Assumes the wall of text is hiding a simple statement. `Found:` is what your project has.
      `Could not resolve dependency: peer` is what the new package wants. In the sample, one
      package wants React 18 and the project is on React 19, and everything else in the output
      is supporting detail.

  - try: Check whether a newer version of the package supports what you have.
    command: npm view <name> versions --json
    shell: powershell
    why: >
      Assumes you are installing an outdated release. Packages catch up with new framework
      versions on their own schedule, and installing the latest often makes the conflict
      disappear. Install a specific one with `npm install <name>@<version>`.

  - try: Install anyway, accepting the mismatch.
    command: npm install <name> --legacy-peer-deps
    shell: powershell
    why: >
      Assumes the packages are being cautious and the combination actually works, which is
      often true when a major version bumped without breaking much. The risk is real though:
      you are overruling the package author, and the failure that follows appears at runtime
      rather than at install time.

  - try: Look for a maintained replacement.
    command: npm view <name> time.modified
    shell: powershell
    why: >
      Assumes the package is abandoned. A peer dependency stuck two major versions behind
      usually means nobody has touched it in a long time. This prints the date of its last
      release, and a package untouched for two years is a decision worth making deliberately.

  - try: Clear the tree and install from scratch.
    command: Remove-Item -Recurse -Force node_modules, package-lock.json; npm install
    shell: powershell
    why: >
      Assumes the lockfile itself is inconsistent, usually because two package managers were
      both used in this project or an install was interrupted. This is the last resort because
      it can change versions across the whole project rather than only the one you were
      installing.

if_none_worked: >
  Paste the entire npm error block from `code ERESOLVE` down to the last line, plus your
  `dependencies` and `devDependencies` sections. The block is long and repetitive, which is
  exactly why people trim it, and the `Found:` and `peer` lines that carry the whole answer are
  usually in the part that gets cut.

see_also:
  - g1-what-a-dependency-is
  - g3-lockfiles
  - g7-dependency-risk
  - javascript

keywords:
  - ERESOLVE
  - peer dependency conflict
  - legacy-peer-deps
  - unable to resolve dependency tree
  - npm install fails
---

A peer dependency is a package saying "I need this, and you have to be the one to install it,
because we both have to use the same copy". React plugins work this way: two copies of React
in one project breaks in strange ways, so plugins ask you to supply the one you already have.

npm used to install the mismatch and print a warning. Version 7 changed it to a hard failure,
which is why old instructions say this is a warning and yours says it is an error.

`--legacy-peer-deps` restores the old behavior for that one command. It works often enough
that people reach for it first, and it is still overruling a claim the package author made
deliberately. Check for a newer version first, and if you do use the flag, run the app
afterward rather than assuming a successful install means a working project.

`--force` is the heavier hammer and does more than skip peer checks. Prefer
`--legacy-peer-deps` when you have a choice.
