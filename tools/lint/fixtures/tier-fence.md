---
id: tier-fence
title: A section whose code fence contains a heading
type: section
track: A
verified: 2026-08-02
volatility: low
answer: >
  This fixture proves the tier splitter ignores headings inside fenced code
  blocks, which a plain regex cannot do.
---

## More

A `##` line inside a fence is a shell comment or a markdown example, never a
heading. A regex-based splitter cannot tell the difference and truncates the
section at that line.

```bash
## not a heading, a shell comment
echo "still inside More"
```

This paragraph sits after the fence. A broken splitter loses it.

## Full

The failure mode is nasty because it is silent. The tier keeps rendering, just
shorter, and the only symptom is a warning saying the last tier is shorter than
the one before it. That reads as an authoring mistake rather than a tooling bug,
so the author rewrites perfectly good prose trying to satisfy it.

```markdown
## An example heading inside a markdown sample
Some text under it.
```

This paragraph is what makes the test meaningful: it comes after a fence that
contains a heading, so it is only reachable if the splitter tracked the fence.
Without that, `Full` ends at the fence and measures shorter than `More`, and the
tier-depth warning fires on a card that is correct.

Padding to keep this tier comfortably longer than the one above it, so a failure
here means the fence logic broke rather than the word counts drifting.
