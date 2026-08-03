---
id: a-file-is-locked-by-another-program
title: "A file is locked by another program"
type: intent
verified: 2026-08-02
volatility: low

goal: "Windows says a file is in use and I cannot delete or replace it."
target: file-in-use-by-another-process
urgency: stuck

phrasings:
  - "the process cannot access the file"
  - "file is in use by another process"
  - "i cant delete this file"
  - "the file is locked"
  - "close the program using this file"
  - "cant overwrite the file"
  - "something is holding onto it"
---
