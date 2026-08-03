---
id: j4-reading-a-repo-you-did-not-write
title: Walking into a repo you did not write
type: section
track: J
order: 40
verified: 2026-08-02
volatility: low
verify: git log --oneline -20
answer: >
  Read five things in order: the readme, the manifest, the entry point, the test
  folder, and the last twenty commits. That gives you what the project does, how
  to run it, and where the work is happening, without reading the code.
owns:
  - orientation procedure for unfamiliar code
see_also:
  - j3-project-layouts
  - d13-tags-releases-and-history
  - j1-how-to-recognize-a-language
  - c3-what-running-means
  - g1-what-a-dependency-is
  - h2-kinds-of-tests
keywords:
  - unfamiliar codebase
  - inherited project
  - where do i start
  - understand a repo
  - onboarding
  - cloned a repo now what
---

## More

You cloned something, or an agent did, and now there are four hundred files on your screen.
Do not open `src` yet. Reading code is the slowest possible way to learn what a project is,
and it is the fourth thing that happens here, not the first.

Twenty minutes, five steps, in this order.

**1. The readme, five minutes.** Every project's front page. You want three answers: what
this thing is, how to install it, how to run it. Copy the run command somewhere. If the
readme has a section headed Development or Contributing, that is usually the only part
written for someone who will change the code.

**2. The manifest, three minutes.** `package.json`, `Cargo.toml`, `pyproject.toml`, whichever
the project has ([j1](#j1-how-to-recognize-a-language) tells you which). Two parts matter.
The `scripts` block, or its equivalent, lists every command the project expects you to run.
The dependency list tells you what kind of program this is: a web framework in there means a
web app, a testing library means somebody cared, forty dependencies for a small tool means
somebody did not.

**3. The entry point, five minutes.** The file where execution starts: `src\main.rs`,
`src\index.ts`, `main.py`, `Program.cs`. Read it top to bottom, once. You are not studying
it. You are collecting names, because the things it sets up are the things the project is
made of.

**4. The tests, five minutes.** Open the `tests` folder and read the test names, not the test
bodies. A test called `rejects_expired_token` tells you the system has tokens, that they
expire, and that somebody thought about it. Test names are the most honest description of
intended behavior in any repository, because unlike the readme they fail when they go stale.

**5. The recent history, two minutes.** The last twenty commits tell you what is being worked
on right now and which files are moving.

```powershell
git log --oneline -20
```

`--oneline` prints one commit per line, `-20` stops after twenty. More on reading history is
in [d13](#d13-tags-releases-and-history).

At the end you should be able to say what the project does, how to run it, and roughly where
you would go to change one thing. That is a working mental model, and it is enough to start
asking an agent useful questions.

## Full

### Step 1, expanded: what a readme actually tells you

Read the top third and the section on running it locally. Skip the badges and the
screenshots.

What you are extracting:

- The one-sentence description. If there is not one, the project is either very early or
  very confident.
- The install and run commands, verbatim. Paste them into a note.
- Requirements: a language version, a database, an API key. This is where you find out the
  thing needs Postgres before you spend an hour wondering why it will not start.
- The license, which tells you what you are allowed to do with it.

When there is no readme, or it is three lines and a logo, jump to the manifest and treat the
absence as information: nobody expected an outsider to run this.

### Step 2, expanded: reading a manifest for what it implies

```json
{
  "scripts": {
    "dev": "vite",
    "build": "tsc -b && vite build",
    "test": "vitest run",
    "lint": "eslint ."
  }
}
```

That block answers "how do I run this" in four lines, and every one of those is a command
you can type as `pnpm dev`, `pnpm test`, and so on. Rust puts the same information in
convention instead: `cargo run`, `cargo test`, `cargo build`, always the same
([c3](#c3-what-running-means)).

The dependency list is a description of the project written by its choices. A name you do
not recognize is worth ten seconds in a search box, because knowing that the project uses a
particular web framework or database library tells you more about its shape than any file
in it will.

### Step 3, expanded: the entry point is a table of contents

The entry point is short in almost every project. It sets up the things the program needs,
wires them together, and starts something. Read it for the nouns.

In a web server, you are looking for the list of routes: those are the things the program
can be asked to do. In a command-line tool, you are looking for the list of subcommands. In
a desktop app, you are looking for the window setup and the list of commands the front end
can call.

You will not understand the implementation and you are not trying to. You are building a
map of where things are, so that when you want to change one, you know which end of the
project to point an agent at.

### Step 4, expanded: what the tests tell you for free

- **How many there are.** Zero tests is a fact about the project you want to know before you
  change anything, because nothing will tell you when you break it.
- **What kind they are.** Fast checks of single functions, or slow ones that start a real
  database ([h2](#h2-kinds-of-tests)).
- **What the edge cases are.** Every test named for a failure is a bug somebody already hit.
- **Whether they pass right now.** Run them before you change one line. A test suite that
  was already red before you arrived is not your fault, and finding that out later is
  miserable.

### Step 5, expanded: the history answers questions the code cannot

```powershell
git log --oneline -20 --stat
```

`--stat` adds the list of files each commit touched, which is how you spot the parts of the
project that actually move.

Two more that are worth the keystrokes:

```powershell
git log -1 --format=%cd
```

Prints the date of the most recent commit. A project last touched three years ago is a
different proposition from one touched yesterday, whatever the readme claims.

```powershell
git shortlog -sn --all
```

Counts commits per author, most first. One name at ninety percent means one person's
opinions are in every file. Ten names evenly spread means conventions were negotiated and
are probably written down somewhere.

### Is this project alive

Four checks, thirty seconds:

1. Date of the last commit.
2. Date of the last release ([d13](#d13-tags-releases-and-history)).
3. Open issues with no reply for months.
4. Dependency versions in the lockfile, compared with the current ones.

None of these is disqualifying on its own. A small tool that does one thing can be finished
rather than abandoned. All four pointing the same way means you are adopting maintenance
along with the code.

### Handing it to an agent, usefully

Once you have done the five steps, an agent can fill in the middle. The prompts that work
are the specific ones:

- "Map this repository: list the entry point, the main modules, and what each one is
  responsible for. Do not summarize the readme, I have read it."
- "I want to change how X behaves. Which files would that touch, and why those?"
- "What does this project do that is unusual compared with a standard project of this kind?"

The prompt that does not work is "explain this codebase," which reliably produces a
paraphrase of the readme and the file tree you were already looking at
([e5](#e5-prompting-that-works)).

### The exit test

You are done when you can answer these without opening a file:

- What this thing does, in one sentence.
- The command that runs it, and the command that tests it.
- Roughly where you would go to change the thing you came here to change.

If the third one is still blank after twenty minutes, that is a finding about the project
rather than about you. Say so in the note you leave for yourself, and go find the person who
wrote it, or the commit that introduced the part you cannot place.
