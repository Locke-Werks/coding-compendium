---
id: g3-lockfiles
title: Lockfiles, and why you commit them
type: section
track: G
order: 30
verified: 2026-08-02
volatility: low
verify: git ls-files package-lock.json pnpm-lock.yaml Cargo.lock
answer: >
  Your manifest says roughly which versions you want and the lockfile records
  exactly which ones you got, down to every transitive package, so the lockfile
  is what makes an install repeatable. Commit it. Never edit it by hand.
owns:
  - lockfile
  - exact versions
  - reproducible installs
see_also:
  - g2-package-managers
  - g1-what-a-dependency-is
  - d12-gitignore-and-what-not-to-commit
  - h3-reviewing-a-diff-you-cannot-fully-read
  - i4-releases-and-versioning
keywords:
  - package-lock.json
  - pnpm-lock.yaml
  - Cargo.lock
  - should i commit the lockfile
  - the lockfile changed on its own
  - can i delete the lock file
  - npm ci
---

## More

Your manifest asks for `"express": "^4.18.2"`, which means "4.18.2 or anything newer below
5.0.0." That is a range, and a range resolves to a different version depending on what day
you install. The **lockfile** is the record of what the range actually resolved to: express
at 4.18.2 exactly, plus the exact version of all 57 packages express dragged in with it,
plus a checksum for each one so a tampered download is caught.

Manifest: what you asked for. Lockfile: what you got.

Three rules cover almost every situation.

**Commit it.** It belongs in the repository next to the manifest. Without it, you and your
machine at work and the server running your tests can each install a different set of
versions from the same project, which is the mechanical cause of a large share of "works on
my machine" ([g4](#g4-environments-and-isolation)). Anyone telling you to gitignore a
lockfile is talking about a published library, which is not what you are building.

**Never edit it by hand.** It is generated. Change a number in it and you have told your
tools a lie, which surfaces later as an install that cannot be reproduced. To change what
is in it, change the manifest and run the install command, which rewrites the lockfile for
you.

**Do not read it.** It is thousands of lines and it is machine output. When a change shows
up with 3,000 modified lines in `package-lock.json`, that is not something to review line by
line. Look at the manifest diff instead, which is a handful of lines and tells you what
actually changed ([h3](#h3-reviewing-a-diff-you-cannot-fully-read)).

If the lockfile changed and you did not touch it, something ran an install. That is worth
noticing, and the next tier explains the four reasons it happens.

## Full

### What is actually inside one

For every package in the entire tree, direct and transitive:

- the exact resolved version
- where it was downloaded from
- a hash of the contents, so a package that was altered after publication fails the install
- which packages required it, and under what version range

That last part is why the file is enormous. It is not a list of your dependencies. It is a
complete map of the dependency graph with every edge written down.

The names, per ecosystem: `package-lock.json` for npm, `pnpm-lock.yaml` for pnpm,
`yarn.lock` for yarn, `Cargo.lock` for Rust, `poetry.lock` and `uv.lock` for those Python
tools. Plain `pip` has no lockfile at all, which is why a pinned `requirements.txt` with
exact `==` versions is the usual substitute and why `uv` is worth the switch.

### Why the lockfile changed when you did not touch it

Four causes, in order of how often they happen:

1. **Something ran an install that added or removed a package.** Usually your agent, doing
   what you asked. The manifest changed too, and that is the diff to read.
2. **Somebody ran an update.** `npm update` and `cargo update` resolve every range again
   and take the newest allowed version of everything. This is a deliberate act with a real
   blast radius, which is why it is on the unsupervised blocklist
   ([e11](#e11-what-to-never-let-an-agent-do)).
3. **A different version of the package manager rewrote it.** npm 9 and npm 10 format the
   file differently, so a teammate on another version can produce a huge diff that changes
   nothing.
4. **The manifest and the lockfile disagreed** and the manager quietly fixed it during an
   ordinary install.

Case 1 is normal. Case 2 you want to have chosen. Cases 3 and 4 are noise you can accept.

### The install that respects the lockfile

```powershell
npm ci
```

Deletes `node_modules`, installs precisely what the lockfile says, and fails outright if
the lockfile and the manifest disagree. Plain `npm install` is allowed to update the
lockfile to resolve a difference; this one is not, which is exactly what you want on a
build server ([h5](#h5-ci-cd)). Use it locally too when you want certainty that you are
running the same versions as everyone else.

The equivalents: `pnpm install --frozen-lockfile`, `cargo build --locked`,
`uv sync --frozen`, `dotnet restore --locked-mode`.

### Resolving a lockfile conflict

Two branches both added a dependency, so both rewrote the lockfile, and now git shows a
merge conflict in 400 places. Do not try to pick lines
([d7](#d7-merge-conflicts) covers conflicts in general, and this is the one case where the
normal advice does not apply).

Take either side whole, then rebuild the file from the manifest:

```powershell
git checkout --theirs package-lock.json
npm install
git add package-lock.json
```

`--theirs` here means take the incoming branch's copy, which is arbitrary and fine, because
the next command regenerates it from the merged manifest anyway. Verify by checking that
`git status` is clean and the app still starts.

### Deleting one, and when that is reasonable

Deleting the lockfile does not break your project. The next install regenerates it from the
manifest, resolving every range to whatever is newest today. That is an upgrade of your
entire dependency tree in one step, disguised as a cleanup, so treat it as the decision it
is rather than a way to clear an error.

The legitimate uses are narrow: the file is genuinely corrupt, or you have decided to
upgrade everything and are ready to test the result. Commit first, so
[d10](#d10-undo-everything) can put it back.

### Reviewing a change that includes one

Ignore the lockfile diff itself and read the manifest diff. A well-behaved change adds one
line to `package.json` and several hundred to `package-lock.json`. If the manifest gained
nothing and the lockfile changed anyway, an update ran. If the manifest gained three
packages you did not ask for, that is scope creep in its most expensive form, because those
packages are now yours ([g7](#g7-dependency-risk)).

```powershell
git diff -- package.json
```

One command, a few lines of output, and it answers the only question the lockfile diff was
ever going to answer.
