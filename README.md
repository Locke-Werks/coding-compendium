# Coding Compendium

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
pnpm fetch:vendor         # downloads the model files and llama.cpp (large, once)
pnpm build:content        # compiles content/ into the searchable database
pnpm tauri dev            # runs the app
```

`pnpm tauri dev` opens the app with live reload: change a file, the app updates. To build the
installer you would hand to someone else:

```powershell
pnpm tauri build
```

The `.msi` lands in `src-tauri/target/release/bundle/msi/`.

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
├── tools/
│   ├── build/            compiles content/ into a SQLite database
│   └── lint/             the voice linter, run in CI
├── src-tauri/          the Rust half: search, embeddings, the identifier
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

## The local model

The app ships with a small language model, about 380MB, that runs on your own machine
through llama.cpp. It is used for one narrow job: when the search finds relevant cards but
you asked something the cards do not directly answer, it reads those cards and writes a
short answer citing which ones it used.

Two things about this were deliberate.

**It is optional by construction.** The model is a file on disk. Delete it and the app keeps
working, minus that one feature. Nothing else in the app knows the model exists. That is
enforced in the code by a trait boundary in `src-tauri/src/synth/`, which is a good example
of a design pattern worth recognizing: make the optional thing implement an interface, and
give the interface a do-nothing implementation for when it is absent.

**It was chosen for honesty, not intelligence.** Small models confabulate: they produce
confident, fluent, wrong answers. A general-purpose model roughly four times this one's size
still invents an answer on nearly half of the questions its source material does not cover.
That is unacceptable for a reference tool, because you cannot check the answer, which is why
you are asking.

So the model here is one trained specifically to say "the sources do not cover this," and it
emits that verdict as a machine-readable field the Rust code checks rather than a phrase we
hope shows up in the prose. On top of that, the answer is constrained by a grammar generated
per request, which makes citing a source that was never provided *impossible* rather than
merely discouraged.

And the retrieved cards are always shown above the generated answer. If the model fails and
if the model abstains, you see the same thing: the real cards. A failure can never turn into
a wrong answer.

---

## License and attribution

Private. Locke Werks.
