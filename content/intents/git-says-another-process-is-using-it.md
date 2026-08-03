---
id: git-says-another-process-is-using-it
title: "Git says another process is using the repository"
type: intent
verified: 2026-08-02
volatility: low

goal: "Git says a lock file exists and will not run anything."
target: git-index-lock-file-exists
urgency: panic

phrasings:
  - "index lock file exists"
  - "another git process seems to be running"
  - "git is stuck and wont do anything"
  - "unable to create index lock"
  - "every git command fails now"
  - "can i delete the lock file"
  - "git is frozen"
---
