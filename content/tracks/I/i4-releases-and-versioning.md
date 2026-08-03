---
id: i4-releases-and-versioning
title: Releases and version numbers
type: section
track: I
order: 40
verified: 2026-08-02
volatility: low
answer: >
  In `2.1.3` the first number breaks things, the second adds things, and the
  third fixes things. A release is one version packaged for other people: the
  number in the manifest, a tag on the commit, and the built files attached.
owns:
  - semantic versioning
  - release
  - changelog
see_also:
  - d13-tags-releases-and-history
  - i3-builds-and-artifacts
  - g3-lockfiles
  - d4-commit-well
  - i5-shipping-a-desktop-app
keywords:
  - semver
  - version number meaning
  - caret version
  - breaking change
  - changelog
  - release notes
  - v1.0.0
---

## More

Most version numbers you meet are three numbers separated by dots, and the scheme is called
semantic versioning. Read `2.1.3` as `MAJOR.MINOR.PATCH`.

- **Patch**, the last number. Something was broken and now works. Nothing else changed.
  Upgrading is meant to be free.
- **Minor**, the middle number. Something was added. Everything that worked before still
  works. Upgrading is meant to be safe.
- **Major**, the first number. Something that used to work stopped working. Upgrading means
  reading what changed and possibly editing your own code.

That is the whole promise, and it is a promise made to whoever depends on your project
rather than to you. When a library goes from `4.9.2` to `5.0.0`, the author is saying "I
broke something on purpose." When it goes to `4.10.0`, they are saying "safe to take."

Two conventions that trip people up. Version `0.x.y` means nothing is promised at all: below
`1.0.0` the author is allowed to break anything in any release, and plenty of widely used
software has stayed there for years. And a suffix like `1.0.0-beta.2` or `1.0.0-rc.1` marks
a pre-release, which sorts *before* the plain `1.0.0`, not after.

A **release** is one version handed to other people. Three separate things carry the number
and all three have to agree:

1. The **manifest** holds the version as text, in `package.json`, `Cargo.toml`, or
   `pyproject.toml`.
2. A **tag** marks the exact commit that version was built from, conventionally written
   `v1.0.0` with the leading letter. Tags belong to [d13](#d13-tags-releases-and-history).
3. The **release page** on GitHub packages that tag with notes and the built files from
   [i3](#i3-builds-and-artifacts) attached, so people can download without building
   anything.

If nobody depends on your project yet, none of this is enforced by anything. Pick the scheme
anyway, because the day someone does depend on it, the history of numbers you already
published is the only evidence of what you meant.

## Full

### Deciding which number to bump

Ask one question: would this change break somebody who is using the current version without
reading anything?

| Change | Bump | Example |
|---|---|---|
| Fixed a crash, behavior otherwise identical | Patch | `1.4.2` to `1.4.3` |
| Added a new option, existing calls unchanged | Minor | `1.4.3` to `1.5.0` |
| Renamed or removed something people call | Major | `1.5.0` to `2.0.0` |
| Changed a default value | Major | Silent behavior changes are the cruelest break |
| Updated a dependency, no visible effect | Patch | Unless the dependency's break shows through |
| Rewrote the internals, same interface | Patch or minor | Nobody can see the internals |

The fourth row is the one people get wrong. A changed default breaks working setups without
producing a single error message, which makes it worse than a removed function, and it earns
a major bump every time.

Two numbers reset when the one to their left moves. After `1.9.4` comes `2.0.0`, never
`2.9.4`. And the numbers are not decimals: `1.10.0` is newer than `1.9.0`, exactly the way
software versions have confused people since the 1990s.

### Reading version ranges in a manifest

The manifest rarely names one exact version. It names a range, and the operators are worth
recognizing on sight:

```json
{
  "dependencies": {
    "react": "^19.2.0",
    "vite": "~8.2.0",
    "left-pad": "1.3.0"
  }
}
```

- `^19.2.0` means "19.2.0 or newer, but nothing that starts with 20." Take minor and patch
  updates, refuse major ones. This is the default npm writes for you.
- `~8.2.0` means "8.2.0 or newer, but nothing that starts with 8.3." Patch updates only.
- `1.3.0` with no operator means exactly that version and nothing else.

The range is the intent. The lockfile records which version you actually got and pins it for
everyone else, which is the whole argument of [g3](#g3-lockfiles).

### The other schemes you will see

**Calendar versioning.** The number is a date: Ubuntu `24.04` shipped in April 2024, and
some tools use `2026.8.1`. It tells you how old the software is and says nothing about
compatibility.

**No scheme at all.** Windows 11, macOS Tahoe, and every product with a marketing department
version by feeling. This is fine for a product nothing else depends on and useless for a
library.

**Build numbers.** A counter that goes up every time the build machine runs, often glued to
the end: `1.4.2+build.881`. Everything after the plus sign is ignored when comparing
versions.

### The changelog

A changelog is a file at the root of the project, usually `CHANGELOG.md`, that lists what
changed in each version, newest at the top. It is written for the person deciding whether to
upgrade, which makes it a different document from your commit history.

In the file itself each version is a level-two Markdown heading and each group below it is a
level-three heading, which renders like this:

```text
1.2.0 - 2026-08-02

Added
  - Offline search across every card.

Fixed
  - Blank window on first launch when the database had not been built yet.

Changed
  - The panic key is now Ctrl+Shift+P. The old binding no longer works.
```

Four groups cover nearly everything: Added, Changed, Fixed, Removed. The rule that makes
one useful is that every line describes what a person can now see, do, or stop working
around. "Refactored the search module" belongs in the commit message, not here.

If your commits follow the conventional prefixes from [d4](#d4-commit-well), a tool can
draft this from the log. Draft is the right word: generated changelogs read like commit
messages, and they need a pass by hand before anyone else sees them.

### The release, start to finish

1. Decide the number using the question at the top of this tier.
2. Update the version in the manifest. Update the changelog.
3. Commit that, on its own: `chore: release v1.2.0`.
4. Tag the commit and push the tag ([d13](#d13-tags-releases-and-history)).
5. Build the artifacts ([i3](#i3-builds-and-artifacts)).
6. Create the release, paste the changelog entry as the notes, attach the built files.

Steps 4 through 6 are what release automation does for you when a tag gets pushed. Doing it
by hand once first is worth the twenty minutes, because when the automation breaks you will
know which of the six steps it stopped at.

### The gotcha this app has

The version number lives in more than one file here: `package.json` for the front end,
`Cargo.toml` for the Rust side, and `tauri.conf.json` for the installer. The one that ends
up in the installer's filename and in the Windows uninstall list is the one in
`tauri.conf.json`. Nothing checks that the three agree, so they drift, and you find out when
a user reports a version number you have never heard of.

Any project assembled from two ecosystems has this problem. Find every file holding the
number before your first release, write them down, and change them together.
