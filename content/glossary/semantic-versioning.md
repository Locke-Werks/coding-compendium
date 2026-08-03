---
id: "semantic-versioning"
title: "Semantic versioning"
type: glossary
verified: "2026-08-02"
volatility: low
term: "Semantic versioning"
aliases: ["semver", "semantic version", "version number"]
short_def: "The scheme where 2.1.3 means major 2, minor 1, patch 3, and a bump in each position promises a different amount of breakage."
canonical_section: "i4-releases-and-versioning"
---

Patch means a fix, nothing else moved. Minor means something was added and your existing code still works. Major means something you were using changed or went away, so read the notes before upgrading. The promise is a convention, and plenty of projects break it, so a major bump is a warning rather than a guarantee.
