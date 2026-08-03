---
id: a6-how-to-use-this-app
title: How to use this app
type: section
track: A
order: 60
verified: 2026-08-02
volatility: weekly
answer: >
  There is one box. Type a question and it searches, or paste anything you do not
  recognize and it identifies it. Enter opens a result, Escape backs out one
  level, and everything runs on your machine.
owns:
  - the app's own surfaces
see_also:
  - j1-how-to-recognize-a-language
  - f1-how-to-read-an-error-message
  - d10-undo-everything
keywords:
  - how do i use this
  - whats the hotkey
  - stale badge
  - may be out of date
  - yellow badge
  - paste to identify
  - panic button
  - sidecar
---

## More

One box, two behaviors, and it picks between them for you.

**Type a question** and it searches: "what is a branch", "how do I undo the last commit",
"why is my command not found". Two engines run at once, one matching your words and one
matching your meaning, so a question phrased nothing like the answer still lands. Results
appear as you type.

**Paste anything you do not recognize** and it identifies instead: code, an error message, a
command, a config file. It tells you what it is and shows the evidence it used, which is the
part worth reading. Being told "that is Rust" answers today's question. Being shown that only
Rust spells a function `fn` means you recognize the next one without opening anything.
[j1](#j1-how-to-recognize-a-language) is the method it uses.

The switch is automatic. Anything containing a line break is treated as pasted material,
because nobody types a newline into a search box. If it guesses wrong on a single line,
adding a line break forces the identify path.

**The keys**, all of them: up and down move through results, Enter opens the selected one,
Escape backs out one level, closing the card first and clearing the box second. The box is
focused the moment the window opens, so there is nothing to click before you start typing.

**Every card is written in three depths.** The one-sentence answer sits at the top, then
`More` for enough to act on, then `Full` for the edge cases and the worked example. Reading
only the answer line is a supported way to use this app, not a shortcut.

**It is all local.** No account, no network, no key. It works on a plane and it works when
GitHub is down, which is one of the times you are most likely to need it.

## Full

### The stale badge, honestly

Some cards show a small amber badge reading `may be out of date`. Here is exactly what
produces it, because a warning you cannot interpret is worse than none.

Every card carries two fields written by whoever wrote it:

```yaml
verified: 2026-08-02
volatility: weekly
```

`verified` is the date a person last checked the card against reality. `volatility` is that
person's judgment about how fast the card's claims rot, and it sets the card's own budget:

| volatility | Goes yellow after about | Typical content |
|---|---|---|
| `weekly` | one month | install commands, tool flags, model names |
| `quarterly` | six months | file layouts, config key names, conventions |
| `low` | two years | git's data model, what a compiler is, how a language declares a function |

The badge appears only when a card passes its own budget. That is the whole design. Stamping
a date on every card produces noise nobody reads, and a badge on everything means nothing.
An install command for a coding agent can genuinely change in a fortnight, and git's
snapshot model has not moved in fifteen years, so those two claims should not expire on the
same schedule. This card is marked `weekly`, because it describes shipping software that
changes.

What to do when you see the badge: read the card anyway, then check the one thing most likely
to have moved, which is usually a command or a version. Cards that install something carry a
`verify` command you can run to test the claim yourself in one line. Cards about a
fast-moving tool name the official docs, and those are the source of truth, not this app.

### When git has gone wrong and you cannot name what happened

Search for what you did rather than what you want. "I committed to the wrong branch",
"I think I lost my changes", "it will not let me push". Those land on the panic cards, which
work differently from the rest: one question at a time, in plain language, and every option
states what it destroys before you choose it. Several of them offer to make a backup branch
first. [d10](#d10-undo-everything) is the same material organized by cost instead of by
symptom, for when you are calm.

### The sidecar

The main window is a full reading surface. There is also a narrow strip you can dock to the
edge of your screen and leave open while you work in your terminal. Alt-tabbing to look
something up is a habit that takes weeks to build. Having it already on screen takes none.

### What this app cannot do

- **It does not know your project.** It has never seen your files. When a card says "check
  which shell you are in," you check.
- **It cannot run anything for you.** Every command here is text to copy. That is
  deliberate: a reference that could execute things is an agent, and an agent is
  [e1](#e1-what-an-agent-is).
- **It can be wrong.** People wrote it. The defenses are the freshness budget above, the
  `verify` command on install cards, and a link to the official documentation on anything
  that moves. When this app and the official docs disagree, the docs win.
- **It does not teach you a language.** Track J teaches recognition, which is a different
  and much smaller thing. The language cards tell you what you are looking at, how to tell it
  apart from its nearest neighbor, and what agents typically get wrong writing it.

### If a card is wrong

Everything in this app is plain markdown files with a small header, compiled into the
searchable database at build time. There is a file per card under `content/`, readable in any
text editor, and fixing a typo means fixing the markdown and rebuilding. The content is the
source of truth and the database is a build artifact, which is worth knowing because it means
nothing here is locked behind the app.

### Getting back to something you read before

Search the phrase you remember. The semantic half of the search does not need your wording to
match, so a half-remembered idea usually finds the card. Failing that, every card lists its
neighbors at the bottom, and the tracks run in order: A orientation, B setup, C how code runs,
D git, E agents, F failure, G dependencies, H proving it works, I shipping, J recognition.
