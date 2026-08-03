---
id: "rebase"
title: "Rebase"
type: glossary
verified: "2026-08-02"
volatility: low
term: "Rebase"
aliases: ["rebases", "rebasing", "rebased", "git rebase"]
short_def: "Replaying your commits on top of another branch's latest state, producing a straight-line history instead of a merge commit."
not_to_be_confused_with: ["merge"]
canonical_section: "d6-merge-and-rebase"
---

Merge keeps what actually happened and adds a join. Rebase rewrites your commits so it looks like you started from the newer code all along. Rebasing gives cleaner history and creates new commit identifiers for everything it moves, which is why you do not rebase anything you have already pushed and shared.
