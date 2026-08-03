---
id: violations
title: Every rule, deliberately broken
type: error
verified: 2026-08-02
volatility: low
category: not-found
patterns: ["this card is a test fixture and matches nothing real"]
means: >
  Exists so the self-test can assert the linter still fires. Every rule below is
  broken on purpose. If the linter reports this file as clean, the linter is
  broken, not this file.
fix_ladder:
  - try: Read tools/lint/selftest.ts
    why: It lists which rule each line here is meant to trip.
see_also: [this-id-does-not-exist]
---

## More

We should simply delve into this robust and seamless behaviour, which is comprehensive.

This claim is not just wrong, but very wrong, and moving forward that said it is a key
takeaway worth leveraging in today's landscape.

An em dash lives here — right there. An en dash lives here – too.

The API is mentioned without expanding it, and so is the SDK.

Colours and favourite organisation are British, and licence is too.

Run `git reset --hard` and `rm -rf /` with no danger annotation at all.

```
This fence has no language tag.
```

```powershell
# This fence is tagged, so it must NOT be reported.
git status
```

Inline `git reset --hard` inside a code span must not trip the prose rules, and neither
must a fenced block containing the word simply:

```text
simply delve robust seamless behaviour — colours
```

## Full

This tier exists so the tier splitter has something to find. It contains a code
fence with a heading-shaped line inside it, which must NOT end the section:

```bash
## this is a shell comment, not a heading
echo "still inside Full"
```

Text after the fence proves the splitter kept reading past that line. Without
fence awareness this paragraph vanishes and the tier reads as truncated.
