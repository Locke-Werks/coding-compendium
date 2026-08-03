---
id: "secret"
title: "Secret"
type: glossary
verified: "2026-08-02"
volatility: low
term: "Secret"
aliases: ["secrets", "sensitive value"]
short_def: "Any value that grants access and must never be committed or pasted: a key, a password, a token, a connection string."
not_to_be_confused_with: ["credential"]
canonical_section: "g6-secrets-and-what-never-to-commit"
---

Deleting a secret from a file and committing the deletion does not remove it. The old version is still in the history and still on GitHub. Once a secret has been pushed, rotate it. Assume it is compromised from the moment it left your machine.
