<div align="center">

<img src="assets/coding-compendium.ico" width="96" alt="Coding Compendium">

# Coding Compendium

**Offline Windows reference that answers development questions faster than opening a browser tab**

[![license](https://img.shields.io/badge/license-Proprietary-d6262a?style=flat-square)](LICENSE)
![platform](https://img.shields.io/badge/platform-Windows%2011-d6262a?style=flat-square)

</div>

---

A Windows desktop app that answers questions about software development faster than opening
a browser tab. Everything runs on your machine: the search, the language identifier, and the
answers. No account, no API key, no internet needed once it is installed.

It started as one 44KB markdown file, `vibecoding-knowledge.md`, still in this repo as the
seed. This is that document turned into something you can actually use at the moment you are
stuck, plus about eleven times more of it.

---

## What it does

**Search that understands what you meant.** Type "how do I undo the thing I just did" and get
the right answer, even though the guide calls it `git revert`. Two search engines run at once:
one matches your literal words, one matches your meaning, and the results are merged. More on
how that works below, because it is a genuinely interesting piece of engineering.

**Identify anything you paste.** Code, an error message, a terminal command, a config file.
It tells you what it is **and shows its work**:

```
Rust, 92%
  because  fn        only Rust uses exactly `fn`. Go uses `func`,
                     Kotlin uses `fun`, Python uses `def`
           let mut   Rust variables cannot change by default
           ::        the separator between modules and types
           println!  the `!` means macro, nothing else looks like this
```

The evidence is the point. Being told "this is Rust" answers the question once. Being shown
*why* means next time you recognize it yourself without opening the app.

**A panic button for git.** When you have broken something and do not know what, it asks one
question at a time, in plain language. Every option tells you what it will destroy before you
pick it, and offers to make a backup branch first.

**Language cards.** Twenty-two of them, covering the languages you will actually meet, plus
the config formats (JSON, YAML, TOML, Dockerfile) that everyone hits and nobody explains
because they are "not really code." The goal is not to teach you Python. It is to let you
look at a file and know what you are looking at.

**A sidecar window.** A narrow strip you can dock to the edge of the screen and leave open
while you work. Alt-tabbing to look something up is a habit you have to build. Having the
answer already on screen is not.

---

## Running it

You need these installed once:

```powershell
winget install OpenJS.NodeJS.LTS
winget install Rustlang.Rustup
npm install -g pnpm
```

Confirm each one worked. If these print version numbers, you are good:

```powershell
node --version
cargo --version
pnpm --version
```

Then, from this folder:

```powershell
pnpm install              # downloads the frontend dependencies
pnpm build:content        # compiles content/ into the searchable database
pnpm tauri dev            # runs the app
```

`pnpm build:content` downloads the 66MB embedding model the first time it runs, and never
again. It is the only step that touches the network, and the built app does not.

`pnpm tauri dev` opens the app with live reload: change a file, the app updates.

To build the installer you would hand to someone else:

```powershell
pnpm package
```

That runs the whole path in `scripts/build-installer.ps1`: frontend, corpus, release binary,
sign the binary, stage the payload, forge, sign the installer. The result lands in
`installer/dist/`.

Signing needs `AZURE_TENANT_ID`, `AZURE_CLIENT_ID` and `AZURE_CLIENT_SECRET` in the
environment. Without them, `pnpm package -- -Dev` builds an unsigned installer that the stub
labels as a development build.

Two orderings in that script are load-bearing and are why it is a script rather than a list
of commands. The payload binary is signed *before* it is staged, because payload members are
extracted verbatim: an executable that goes in unsigned comes out unsigned, and signing the
installer afterwards does nothing for it. The installer is signed *last*, because the
signature is what makes the bytes immutable.

---

## How the repo is laid out

```
Coding Compendium/
├── content/            Everything the app knows, as markdown files
│   ├── tracks/           the long explanations, organized A through J
│   ├── languages/        the 22 language fingerprint cards
│   ├── errors/           50 error messages and what they mean
│   ├── commands/         80 terminal commands, flag by flag
│   ├── intents/          plain-language goals mapped to answers
│   ├── glossary/         260 terms
│   ├── panic/            the git disaster decision trees
│   └── _meta/            the frozen glossary and section id lists
├── schema/             JSON Schemas: the required shape of every card type
├── tools/lint/         the voice linter, run in CI
├── scripts/            icons, payload staging, the installer build
├── installer/          the Forge config that packages the app
├── docs/               the decision record, and the local-model gate report
├── src-tauri/          the Rust half: search, embeddings, the identifier, the compiler
└── src/                the React half: everything you see
```

**`content/` is the source of truth.** It is plain markdown with a YAML header. You can read
it in any text editor, and every change shows up as a readable diff in git. The database the
app searches is *compiled* from these files and is never edited by hand. If you want to fix a
typo in the app, you fix the markdown and rebuild.

`AUTHORING.md` is the style guide every one of those files is written to. It is worth reading
if you ever want to add a card.

---

## How the search actually works

Worth understanding, because the same pattern shows up everywhere in modern software.

There are two ways to find text, and they fail in opposite directions.

**Lexical search** matches the words you typed. It is what Ctrl+F does, with better ranking.
The engine here is SQLite's FTS5 with BM25 ranking, which is the same family of algorithm
that ran search engines for twenty years. It is fast, exact, and completely literal. Search
for "detached HEAD" and it finds every card containing those words. Search for "my commits
went somewhere weird" and it finds nothing, because none of those words appear anywhere.

**Semantic search** matches meaning. Every card is run through a small neural network that
turns text into a list of 384 numbers, called an embedding. Text that means similar things
produces similar numbers. Your query gets the same treatment, and then it is arithmetic:
find the cards whose numbers are closest to your query's numbers. "My commits went somewhere
weird" lands near the detached HEAD card because the *meanings* are close, even with zero
words in common. The failure mode is the mirror image: it is fuzzy, so an exact search for
`git reset --hard` may drift to merely related cards.

Each one covers the other's blind spot, so the app runs both and merges the two ranked lists
using **Reciprocal Rank Fusion**. RRF ignores the raw scores, which are not comparable
between the two engines, and looks only at *position*. A card ranked 3rd by one engine and
2nd by the other beats a card ranked 1st by one and 40th by the other. Simple, and hard to
fool.

All of it runs locally and finishes in under 50 milliseconds, which is faster than you can
notice.

---

## Why nothing here writes you an answer

The app never generates prose. When search finds relevant cards, it highlights the sentences
that match your question and pulls the two or three most relevant into a quote block with the
card's name attached. Everything you read was written by a person and is sitting in
`content/` where you can go check it.

This was going to work differently. The plan was to ship a small language model, about 380MB,
that would read the retrieved cards and write a short cited answer for questions the cards do
not directly cover. A model trained specifically to abstain was benchmarked before committing
to it, on 50 questions, 15 of which the corpus deliberately does not answer.

It mostly worked. It abstained on 14 or 15 of the 15, which cleared the bar it was measured
against. It did not ship anyway, and the reason is worth knowing because it generalizes.

Asked how to connect Prisma to Postgres, which the corpus does not cover, it answered from a
card whose entire purpose is to warn against pasting connection strings into a chat window. It
lifted the redacted example out of the warning and presented it as instruction. Well-formed,
correctly cited, literally quoted, and it had inverted a security warning into advice. One
question in fifteen.

That is the failure mode you cannot defend against by being careful, because it looks exactly
like success. And it matters most for the person least able to catch it: you are asking
because you do not already know, which is precisely why you cannot check the answer.

So: extraction, not generation. It cannot invert a warning into advice, because it can only
show you what a card already says. The full measurement is in `docs/PHASE0-LLM-GATE.md`, and
the trait boundary in `src-tauri/src/synth/` is built so this can be revisited later without
disturbing anything else.

The 66MB embedding model that powers semantic search is a different thing entirely and does
ship. It turns text into numbers so meanings can be compared. It does not write anything.

---

## License and attribution

Private. Locke Werks.
