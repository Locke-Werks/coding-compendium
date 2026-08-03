---
id: it-deleted-a-test-to-make-it-pass
title: "I think it cheated to get the tests green"
type: intent
verified: 2026-08-02
volatility: low

goal: "The tests went green and I think the agent cheated to get there."
target: e7-agent-failure-modes
urgency: panic

phrasings:
  - "it deleted my test"
  - "the test passes now but nothing changed"
  - "it commented out the assertion"
  - "green for the wrong reason"
  - "did it fake the fix"
  - "it skipped the failing test"
  - "suspicious green check"
---
