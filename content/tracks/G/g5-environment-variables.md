---
id: g5-environment-variables
title: Environment variables and .env files
type: section
track: G
order: 50
verified: 2026-08-02
volatility: low
verify: "Get-ChildItem Env:"
answer: >
  An environment variable is a named value the operating system hands to every
  program it starts. `$env:NAME = "value"` sets one for this terminal only,
  `setx NAME "value"` makes it permanent, and only new windows see it.
owns:
  - environment variable
  - .env
  - setx
  - session vs permanent
see_also:
  - g6-secrets-and-what-never-to-commit
  - c4-path-and-command-not-found
  - d12-gitignore-and-what-not-to-commit
  - b1-terminal-shell-command-line
  - g4-environments-and-isolation
keywords:
  - env var
  - setx
  - dotenv
  - set an environment variable windows
  - it says the variable is not set
  - where does my api key go
  - environment variable not found
---

## More

Programs read their configuration from outside themselves: which database to talk to, which
key to authenticate with, whether to print debug output. The mechanism is the **environment
variable**, a named value the operating system hands to every program it starts. Keeping the
value out of your code is the point, because the same code then runs on your machine and on
a server with different values and no edits.

There are three lifetimes on Windows and mixing them up is most of the confusion.

```powershell
$env:MY_API_KEY = "<your-key-here>"
```

This terminal window, until you close it. Nothing else on your machine can see it. Good for
a quick test, useless tomorrow.

```powershell
setx MY_API_KEY "<your-key-here>"
```

Permanent for your Windows user account. The window you typed it in does not see it, and
that surprises everyone: `setx` writes the value where new processes read it from, and your
current session already read its copy at startup. Open a new terminal, then check:

```powershell
$env:MY_API_KEY
```

If it prints your value, it worked. If it prints a blank line, you are still in the old
window.

The third lifetime is a `.env` file in the project folder. That is not a Windows feature at
all. It is a plain text file of `KEY=value` lines that a library inside your project reads
when the program starts. If nothing in the project loads it, the file does nothing
whatsoever, which is worth knowing before you spend an hour on it.

The failure you will hit most: you set the variable and the program still says it is
missing. Every process gets its own copy of the environment at the moment it starts. Your
editor, your dev server, and your agent all need restarting before they can see a value you
set after they launched.

A `.env` file is never committed ([d12](#d12-gitignore-and-what-not-to-commit)) and never
pasted into a chat ([g8](#g8-what-never-to-paste-into-a-chat)).

## Full

### Reading, setting, and clearing

```powershell
Get-ChildItem Env:
```

Lists every variable in this session, name and value. It is a long list; most of it is
Windows talking to itself.

`$env:NAME` reads one. `$env:NAME = "value"` sets one for the session.
`Remove-Item Env:NAME` clears one for the session. All three are PowerShell syntax and none
of them work in Command Prompt, which uses `%NAME%` and `set NAME=value`, or in Git Bash,
which uses `$NAME` and `export NAME=value`. Same variables underneath, three spellings.
[b1](#b1-terminal-shell-command-line) tells the shells apart.

### The two surprises in setx

**It truncates at 1024 characters.** A long token can be silently cut in half, and the
symptom is an authentication failure that looks nothing like a length problem.

**Never point it at PATH.** `setx PATH "..."` has permanently mangled a lot of Windows
installations, because it writes back a flattened, truncated copy of a value that was
assembled from two places. Edit PATH through the Windows environment variables dialog
instead. [c4](#c4-path-and-command-not-found) covers what PATH is and why editing it is
delicate.

To remove a permanent variable properly, rather than setting it to an empty string:

```powershell
[Environment]::SetEnvironmentVariable("MY_API_KEY", $null, "User")
```

`$null` deletes the entry, and `"User"` means your account rather than the whole machine.
Verify in a new terminal: `$env:MY_API_KEY` should print nothing at all.

### The .env file, and what actually reads it

The format is deliberately dull:

```ini
DATABASE_URL=postgres://localhost:5432/myapp
MY_API_KEY=<your-key-here>
DEBUG=true
```

One per line, no spaces around the `=`, no `export` in front, quotes only when the value
contains spaces. Lines starting with `#` are comments.

Something has to read it. In Node projects that is usually the `dotenv` package; in Python,
`python-dotenv`; in Rust, `dotenvy`. Web frameworks such as Vite and Next.js load the file
themselves with no package required. If your project has none of those, add the loader or
set the variables in your shell, because the file alone is inert.

One safety detail specific to browser frameworks: Vite only exposes variables whose names
begin with `VITE_` to the code that ships to the browser, and Next.js uses
`NEXT_PUBLIC_`. Anything exposed that way is readable by anyone who opens your site.
A prefix like that is a decision to publish the value, so a real key never gets one.

### The example file everybody forgets

Commit a `.env.example` next to the ignored `.env`, holding the same keys with empty or
obviously fake values:

```ini
DATABASE_URL=
MY_API_KEY=
DEBUG=false
```

It costs nothing and it answers the only question a person cloning the project cannot
answer for themselves, which is which variables the thing needs. That person is usually you
on a new machine at eleven at night.

### Which value wins

Most loaders leave an already-set variable alone. If `MY_API_KEY` exists in your shell and
also in `.env`, the shell value usually wins and the file is ignored for that key. This is
sensible and it produces one confusing situation: you edit `.env`, restart, and see the old
value, because a stale variable in your session is shadowing it. `$env:MY_API_KEY` tells you
in one line whether that is what is happening.

### Where each kind of value belongs

- **Your machine, while developing.** A `.env` file that `.gitignore` covers, or session
  variables.
- **The test runner on a push.** Repository secrets configured on GitHub, which the
  workflow reads as environment variables and which never appear in the log
  ([h5](#h5-ci-cd)).
- **A deployed app.** The hosting provider's configuration screen. Same idea, different
  button ([i1](#i1-what-deployment-means)).

The values in the second and third places are real credentials, and
[g6](#g6-secrets-and-what-never-to-commit) is the card on handling them.

### The agent-specific warning

Agents read files in your project, including `.env`, and they quote what they read back into
the transcript. That transcript goes to a server and often to a session file on disk. Two
habits keep it clean: tell your agent in the instruction file never to open or print `.env`
([e4](#e4-claude-md-and-agents-md)), and check what `git add .` picked up before committing,
because an unignored `.env` gets swept in silently.
