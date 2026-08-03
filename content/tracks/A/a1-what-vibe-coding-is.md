---
id: a1-what-vibe-coding-is
title: What vibe coding actually is
type: section
track: A
order: 10
verified: 2026-08-02
volatility: low
answer: >
  Vibe coding is describing what you want in plain language and letting an agent
  write, run, and revise the code while you judge the running result instead of
  the source. Andrej Karpathy named it in early 2025.
owns:
  - vibe coding
  - the term's origin
see_also:
  - a2-the-honest-version
  - a4-the-loop
  - e1-what-an-agent-is
keywords:
  - karpathy
  - what does vibe coding mean
  - ai coding
  - prompt to app
  - is vibe coding a real thing
  - coding with ai
---

## More

The term comes from Andrej Karpathy, who used it in early 2025 for a way of working he had
fallen into: talk to the model, take what it writes, run the thing, describe the next change,
repeat. He was describing throwaway weekend projects. The phrase escaped and now gets applied
to everything up to and including production software.

The mechanical part is small. You write a sentence in plain English. An agent, meaning an AI (Artificial Intelligence)
(Artificial Intelligence) program that reads your files, writes new ones, and runs commands
rather than only answering questions, turns that sentence into edits on disk. It runs the
code, reads whatever the code printed, and tries again. What you look at is the result: the
page, the output, the test that went green. [e1](#e1-what-an-agent-is) covers what an agent
can reach and what it cannot.

The shift is in where your attention goes. Writing code by hand puts your attention on the
source text one line at a time, because typing the line is the only way to get the line. Vibe
coding puts it on the outcome and the shape of the change, because typing stopped being the
slow part. You still read code. You read less of it and you read it for different reasons.

Three things it is not:

- **Not no-code.** There is a folder, a terminal, real files you can open in Notepad, and a
  git history. The tools are the ones every developer uses, which is why tracks B through D
  of this app exist.
- **Not a product.** Nothing is named Vibe Coding. It describes a habit, and people use it to
  mean anything from "I built a toy in an afternoon" to a straight insult.
- **Not hands-off.** The agent stops constantly to ask, and the answers are yours.

That is the fun version, stated plainly. [a2](#a2-the-honest-version) is the correction, and
[a4](#a4-the-loop) is the rhythm this turns into once you are working on something you care
about.

## Full

### Where the term came from

Karpathy posted the phrase in early 2025, describing a mode where you give in to the vibes,
accept the changes without reading them, paste errors back in without looking at them, and
forget that the code exists as text at all. Read in context it is a description of one
specific low-stakes mode, on projects where the worst outcome is deleting the folder. It was
never proposed as a methodology, and most of the arguments about the term come from people
treating it as one.

The word stuck because it named something a lot of people had started doing at once.

### What made it possible

Three changes had to land together, and all three did within about a year.

1. **Models that write code that runs.** Necessary, and on its own not enough.
2. **Agents that can act.** The model can run the build, read the error it caused, and fix
   it without you as the messenger. The loop closes. This is the piece that turned a chat
   window into a tool, and it is [e1](#e1-what-an-agent-is).
3. **Enough context to see the project.** The model can hold a real number of your files at
   once instead of the one you pasted. [e2](#e2-context-windows) covers the limits of that.

### What a session actually looks like

You type a sentence. The agent narrates what it touches:

```text
> add a dark mode toggle to the settings page

  Read  src/pages/Settings.tsx
  Edit  src/pages/Settings.tsx   (+24 -3)
  Edit  src/theme.ts             (+11 -0)
  Run   npm run build            exit 0

Added a toggle that saves the choice in the browser and applies it on load.
```

Four lines of report for a change you would have spent an hour on. You did not name
`theme.ts`; it found it. That is the appeal, and the second file it touched without being
asked is also the reason [a2](#a2-the-honest-version) exists.

### The three meanings, so you know which one is being used

The same two words carry three different claims, and most arguments about vibe coding are
two people using different ones.

1. **The literal sense.** Do not read the code at all. Karpathy's original. Fine for a
   throwaway, and the stakes rise fast outside one.
2. **The working sense.** An agent writes most of the lines, a person directs, reviews, and
   owns the result. This is what nearly everyone doing it professionally means, and it is
   what this compendium teaches.
3. **The insult.** "That is vibe coded" meaning nobody understands it, nothing is tested,
   and the person who typed the prompts has left.

### What does not change

The parts underneath are the same as they were before any of this existed, which is good
news, because they are stable and worth learning once:

- Code is text in files, in a folder on your machine. [c1](#c1-what-a-program-is).
- Running it means a specific command per ecosystem, and there is no universal run button.
  [c3](#c3-what-running-means).
- Git records versions of that folder, and GitHub stores a copy.
  [d1](#d1-what-git-actually-stores).
- When something breaks, the terminal prints why, in a fixed shape you can learn to read.
  [f1](#f1-how-to-read-an-error-message).

An agent is fluent in all of it and will do most of it for you. It will also occasionally
tell you the wrong thing about it with total confidence, which is why the words are worth
knowing yourself.

### Where this card stops

Everything above is the version people put in the demo video, and it is true. It is also
half the picture. The other half, the part that decides whether this produces working
software or a folder nobody can fix, is [a2](#a2-the-honest-version). Read it next.
