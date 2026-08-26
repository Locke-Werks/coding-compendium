# Authoring guide

Everything in `content/` is written to this guide. Read it before writing a single card.
It ships inside every author's prompt, and the linter in `tools/lint/` enforces the
mechanical half of it as a merge gate.

The goal is that 127,000 words written by forty different authors read as though one person
wrote them in one sitting.

---

## 1. Who you are writing for

One reader. You do not know their name, their pronouns, or their job, and you write as
though you are talking to exactly one of them rather than to an audience.

They have touched code before. They have not lived inside git, the terminal, or AI coding
agents day to day. They are on Windows 11 and will never be told to do something the Mac
way.

They are not stupid and they are not a child. Do not simplify by being vague. Simplify by
being concrete, by defining words at the moment you use them, and by never assuming a prior
section was read. Most people arrive at a card from a search box in a moment of confusion,
not from the top of a document.

Write to them in the second person. "You" in a card means the reader, always. The corpus
never names them, never guesses at their gender, and never assumes anything about them
beyond the paragraph above. This guide says "the reader" and "they" for the same reason.

Two consequences that shape everything:

- **The reader cannot verify your claims.** If you write something wrong, they will act on
  it. This is why every instruction gets a check step and why hedging is worse than useless:
  "this should probably work" gives them nothing to do.
- **They usually arrive mid-crisis.** Something broke, or an agent did something they do not
  understand. Answer first. Explain second. Never make them read a preamble to reach the
  fix.

---

## 2. The seed document is the voice

`vibecoding-knowledge.md` in the repo root is the reference. When in doubt about tone,
cadence, or how much to explain, open it and match it. Every rule below was extracted from
it, with its own examples.

### 2.1 The ten rules

**1. Second person, present tense. Imperative for instructions.**
> Open a new Windows Terminal window and confirm.

**2. Define the word in the same sentence you use it.** Never send the reader to the
glossary mid-sentence.
> A **diff** is the line-by-line view of what changed.

**3. Expand every acronym on first use in *this card*.** Not first use in the corpus. In a
searchable app there is no "first," so every card stands alone. Hover handles repeats.
> CLI (Command-Line Interface)

**4. Every instruction gets a verification step.** The reader needs to know they
succeeded.
> If it prints a version number, you are done.

**5. Name the failure before it happens, and say what it looks like on screen.**
> If you see `'irm' is not recognized`, you are in Command Prompt, not PowerShell.

**6. State the tradeoff plainly.** Do not sell.
> One tradeoff: the winget version does not auto-update.

**7. Admit the limits and point at the source of truth.**
> These tools ship changes weekly, so if a command behaves differently, the official docs
> linked in each section are the source of truth.

**8. Short declarative sentences.** A one-sentence paragraph for emphasis, rarely.
> It sounds intimidating and is not.

**9. Dry humor at most once per section.** Always at the technology's expense. Never at
the reader's.
> Working software you cannot verify is a rumor, not a result.

**10. Windows first, always.** Concrete paths as `C:\Users\<yourname>\...`, with the
PowerShell `~/` equivalent given once where it helps.

### 2.2 Do and don't

| Don't | Do |
|---|---|
| "Git is a powerful and flexible distributed version control system." | "Git is the version-control program. It is separate from GitHub, the website. Git runs on your machine; GitHub stores copies online." |
| "Simply run the installer." | "Download 'Git for Windows' from https://git-scm.com and run the installer. The defaults are fine, accept them all the way through." |
| "This should work." | "If it prints a version number, you are done." |
| "There are several approaches you might consider." | "Use this unless you have a specific reason not to." |
| "Errors can be frustrating!" | "The AI is fast and confident and wrong often enough that you cannot trust it blindly." |
| Re-explain git because your section touches git. | Link to `D1`. Own your section, link to your neighbors. |
| Use an acronym and move on. | "CLI (Command-Line Interface)" on first use in this card, then plain `CLI`. |
| End with a summary restating the section. | End with the next action, or stop. |
| "Let's explore what happens when..." | "Here is what happens when..." |
| Bury the answer in paragraph four. | Put it in the `answer` field in one sentence. |

---

## 3. Banned

The linter fails your card on every item here. No exceptions, no discussion.

**Punctuation.** Em dashes, anywhere, for any reason. Use a colon, a comma, a period, or
restructure the sentence. Emoji, anywhere.

**Words and phrases.** delve · leverage (as a verb) · robust · seamless · comprehensive ·
it's worth noting · that said · in today's landscape · key takeaway · moving forward ·
dive into · unpack · simply

**Constructions.**
- "not just X, but Y" in any form
- A rhetorical question answered by the next sentence
- Tricolons where two items do the job
- Any AI attribution, disclaimer, or "generated with" note, in any file, comment, or
  docstring

**One precision note on "just".** Banned when it means "this is trivial": "just configure
the remote." Allowed when it means "only this one thing": "just close and reopen the
terminal." The seed doc uses it correctly both times it appears. "Simply" is always banned
because it only ever carries the first meaning.

**US spelling throughout.** behavior, favor, capitalization, recognize, organize, color.
The corpus is Windows-first and American, and a card that drifts into British spelling reads
as though a different person wrote it, which is exactly the failure this guide exists to
prevent. The linter enforces this.

The first draft of the three exemplars in this repo violated two of the rules above,
including one written on the same day. Run the linter. Do not assume you are the exception.

---

## 4. The three-tier structure

Every prose section is one file containing three depth tiers. This is what makes
answer-first search results possible, so it is not optional and it is not a formality.

| Tier | Length | Rendered in | Job |
|---|---|---|---|
| `answer` | 1 sentence, **under 45 words** | The palette | The thing they needed, with no preamble |
| `more` | 200-400 words | The reader, on open | Enough to act correctly and know why |
| `full` | 400-900 words | On request | The whole story, edge cases, the worked example |

**Write `answer` last.** After `full` exists, after `more` exists. A distillation written
afterward sounds like a distillation. An opening statement written first sounds like a
thesis, and forty authors writing thesis statements produce forty different voices. This is
a small rule with a disproportionate effect on whether the corpus coheres.

Each tier must stand alone. Someone who reads only `answer` gets something correct and
useful, not a teaser. `more` does not begin by restating `answer`.

---

## 5. Ownership

You own the ids you were assigned. You do not own anything else.

The failure mode across parallel authors is that everyone re-explains git, because
everything touches git. The result is eleven slightly different explanations of a branch,
all of them fine, none of them consistent. Do not do this.

When your section needs a concept another card owns, link to it and move on:

> Work happens on a branch. See [D1](#d1) if that word is new.

The frozen id list is in `content/_meta/sections.yml`. Every id in it exists before anyone
starts writing, so your links resolve. The linter fails any `see_also` or inline card link
that does not resolve.

The frozen glossary is in `content/_meta/glossary.yml`, terms and one-line definitions only.
Read it before writing. If a term is in there, you link to it, you do not redefine it. If a
term you need is missing, add it in the same pull request rather than defining it inline.

---

## 6. Commands

Every command block is copy-ready. They will paste it without reading it, so it has to be
correct standing alone.

    ```powershell
    git switch -c feature/login
    ```
    Makes a new branch called `feature/login` and moves you onto it.
    `-c` means create.

Rules:

- Tag the shell: `powershell`, `bash`, or `cmd`. Never leave it bare. The seed doc is
  careful about which shell each command needs and so are you.
- One command per block unless they genuinely must run together.
- Explain every flag that is not obvious. Assume the reader does not know what `-u` does.
- Never put a placeholder the reader might paste literally without marking it: `<yourname>`,
  not `yourname`.

**Destructive commands carry a `danger:` annotation in frontmatter, and the linter fails
the card without one.** A destructive command is anything that can lose work: `git reset
--hard`, `git push --force`, `git clean`, `rm`, `checkout --`. The card must say what it
destroys, in the same breath, and offer the safe alternative first.

> `git reset --hard` throws away every uncommitted change in your working folder. There is
> no undo. Run `git stash` first if there is any chance you want it back.

---

## 7. Freshness

Every card carries two frontmatter fields:

```yaml
verified: 2026-08-02
volatility: low | quarterly | weekly
```

`volatility` is your judgment about how fast the claim rots:

- `low`: git's data model, what a compiler is, how Python declares a function
- `quarterly`: file layouts, config key names, conventions
- `weekly`: install commands, model names, CLI flags for Claude Code and Codex, anything
  where the seed doc already warns that these tools ship changes weekly

The app shows a stale badge only when a card passes its own budget, so a `low` card from
2026 stays clean and a `weekly` card goes yellow fast. A date on every card is noise.

**Cards with install or check commands also carry a `verify:` command** that the reader can
run to test the claim themselves. This generalizes what the seed doc already does well:

```yaml
verify: git --version
```

---

## 8. Card types

Full JSON Schemas are in `schema/`. The linter validates frontmatter against them. Summary:

| Type | Path | Shape |
|---|---|---|
| Prose section | `content/tracks/<track>/<id>.md` | Three tiers, `see_also`, freshness |
| Language card | `content/languages/<id>.md` | Fixed field set, see §9 |
| Error card | `content/errors/<id>.md` | The text, what it means, the fix ladder |
| Command card | `content/commands/<id>.md` | What it does, every flag, what it destroys |
| Intent | `content/intents/<id>.md` | Plain-language goal, heavy synonyms, target card |
| Glossary term | `content/glossary/<id>.md` | One-line def, then optional expansion |
| Panic tree | `content/panic/<id>.md` | Question, branches, each with command and cost |

---

## 9. Language cards specifically

These are the marquee feature and they have the tightest contract, because the identifier
compiles its scoring weights out of them at build time. The card is the source of truth for
both the prose the reader sees and the classifier that guesses on their behalf.

The rule that makes them work: **state every tell against its nearest neighbor.**

Not this:

> `fn` declares a function.

This:

> `fn` declares a function. Only Rust uses exactly `fn`. Go uses `func`, Kotlin uses `fun`,
> Python uses `def`.

The second version is what teaches recognition. The first is trivia. Every entry in `tells`
and every entry in `confusable_with` follows this pattern.

`confusable_with` gets a one-line settle-it rule per pair, phrased as a decision the reader
can make in two seconds:

> **Go.** Settle it: Go declares with `:=` and functions with `func`. Rust uses `let` and
> `fn`, and uses `::`, which Go never does.

Also required on every language card: `what_agents_get_wrong`. What the agent produces in
this language that looks fine and is not, and what to watch for in a diff. This is the field
that makes the deck useful to a vibe coder rather than to a student.

---

## 10. What the linter checks

Run `pnpm lint:content` before you finish. It fails on:

- Em dash anywhere
- Any banned word or phrase from §3
- `not just .{0,40} but`
- Any emoji codepoint
- Frontmatter that does not validate against the card's schema
- `answer` over 45 words
- An acronym used without a parenthetical expansion at its first occurrence in that card,
  unless it is on the assumed-known list in `content/_meta/assumed.yml`
- Any `see_also` or inline card link that does not resolve
- A command block with no shell tag
- A destructive command with no `danger:` annotation

And warns on:

- Average sentence length over 22 words
- A card with no `verify:` that contains an install command
- A `full` tier shorter than its `more` tier

Warnings do not block a merge. Take them seriously anyway.
