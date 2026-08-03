---
id: "reset"
title: "Reset"
type: glossary
verified: "2026-08-02"
volatility: low
term: "Reset"
aliases: ["resets", "resetting", "git reset"]
short_def: "Moving your branch pointer to a different commit. Some modes keep your edited files and one throws them away for good."
not_to_be_confused_with: ["revert", "restore"]
canonical_section: "d10-undo-everything"
danger: "The hard mode of reset deletes every uncommitted change in your working folder with no undo. Run git stash first if there is any chance you want that work back."
---

Reset moves where your branch points and optionally rewrites your files to match. The soft and mixed modes leave your edits on disk. The hard mode replaces them, and anything uncommitted is gone. Commit or stash before you reach for it.
