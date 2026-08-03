---
id: b9-where-settings-live
title: Where each tool keeps its settings
type: section
track: B
order: 90
verified: 2026-08-02
volatility: quarterly
verify: Test-Path ~/.claude/settings.json
answer: >
  Claude Code reads three settings files, one for your machine and two inside
  each project, and the narrower one wins. Codex reads one file,
  `C:\Users\<yourname>\.codex\config.toml`.
owns:
  - settings.json levels
  - config.toml location
  - precedence
see_also:
  - j2-the-config-formats-nobody-explains
  - e4-claude-md-and-agents-md
  - b8-turn-off-ai-attribution
  - b7-install-codex
keywords:
  - settings.json location
  - config.toml location
  - settings.local.json
  - which settings file wins
  - claude settings
  - codex config
  - json vs toml
---

## More

Two tools, two formats, and one rule that covers both.

**Claude Code** reads JSON (JavaScript Object Notation) from three places:

| File | Level | Who sees it |
|---|---|---|
| `C:\Users\<yourname>\.claude\settings.json` | user | you, in every project on this machine |
| `.claude\settings.json` inside a project | project | committed to git and shared with the repository |
| `.claude\settings.local.json` inside a project | local | you alone, git-ignored, never shared |

In PowerShell the first one is also written `~/.claude/settings.json`.

**Codex** reads TOML (Tom's Obvious, Minimal Language) from one place:
`C:\Users\<yourname>\.codex\config.toml`, also written `~/.codex/config.toml`.

**Narrower wins.** Local beats project, project beats user. So a machine-wide preference goes
in the user file, a rule the whole team should follow goes in the project file and gets
committed, and anything specific to your machine goes in the local file where it cannot
annoy anyone else.

That precedence rule is the answer to almost every "I set this and it did not take" moment: a
narrower file is quietly overriding you.

Settings files are switches the program reads. They are a different thing from `CLAUDE.md`
and `AGENTS.md`, which are prose the model reads at the start of a session. Both shape
behavior and only one of them is configuration. [e4](#e4-claude-md-and-agents-md) owns the
instruction files.

## Full

### Finding them

The folders start with a dot, which makes them easy to miss in File Explorer. From
PowerShell:

```powershell
explorer $env:USERPROFILE\.claude
```

Opens the folder in File Explorer. Swap `.claude` for `.codex` for the other one. To read a
file without opening an editor:

```powershell
Get-Content ~/.claude/settings.json
```

If the file does not exist yet, that is normal: both tools work fine with no settings file
and create one when something needs writing. You can also create it yourself, and an empty
one containing only `{}` is valid.

### The two formats, in one look

You will meet both constantly, so it is worth being able to tell them apart at a glance.

```json
{
  "attribution": {
    "commit": "",
    "sessionUrl": false
  }
}
```

JSON is braces, `"key": value` pairs, commas between them. Two rules break people: **no
comments are allowed**, at all, and **no trailing comma** after the last item in a list or
object. Both produce a parse error that names a line number, and the named line is often the
one after the real mistake.

```toml
# a comment, which JSON cannot have
approval_policy = "on-request"
sandbox_mode = "workspace-write"

[some_group]
a_setting = "value"
```

TOML is `key = value` lines, with `#` for comments and `[headings]` grouping the keys that
follow them. A key written after a heading belongs to that heading, which is the one thing
that catches people: a line you meant as a top-level setting, placed below a group heading,
is a different setting entirely. The two real keys above are Codex's, and
[b7](#b7-install-codex) explains what they do.

[j2](#j2-the-config-formats-nobody-explains) covers the wider family, including YAML (YAML
Ain't Markup Language), which you will meet in continuous integration files.

### What goes at which level

- **User level.** Preferences that are about you rather than about a project: the attribution
  block from [b8](#b8-turn-off-ai-attribution), an editor choice, anything you would want on
  every project you ever open. This is where most of your settings should live.
- **Project level.** Rules the repository itself needs, which everyone working on it should
  get automatically. Committed, reviewed, and visible in a diff like any other file.
- **Local level.** Anything containing a path only your machine has, and anything you want to
  try without committing. It is git-ignored by default, which also makes it the right place
  for anything you would not want published.

The local file is git-ignored for a reason and that reason is worth naming: it is the only
one of the three that does not end up on GitHub. Even so, credentials do not belong in any
of them. [g6](#g6-secrets-and-what-never-to-commit) covers where secrets actually go.

### Working out which file won

When a setting is being ignored, check in this order:

1. **Is the file valid?** A single misplaced comma stops the whole file from loading, and
   some tools fail quietly. Paste the contents into the identify box in this app, or ask an
   agent to check it.
2. **Is a narrower file overriding it?** Look for `.claude\settings.local.json` in the
   project. It is the usual culprit, because it is invisible in git and easy to forget.
3. **Is the key name current?** Settings keys get renamed and deprecated. An unknown key is
   usually ignored in silence rather than reported, which looks exactly like the setting not
   working. [b8](#b8-turn-off-ai-attribution) documents one key this has already happened to.
4. **Did you restart the session?** Both tools read settings at startup.

### The rest of what lives in these folders

The two dot-folders hold more than the settings file, and none of it needs your attention
until something goes wrong:

- Session history and logs, which is where to look when you want to recover what an agent
  actually did.
- Project-specific state keyed by folder path, which is why an agent remembers a project
  after you close the terminal.
- Custom commands and other extensions, once you add any.

Deleting either folder resets the tool to a fresh install, losing your settings and history
and nothing else. It is a reasonable last resort when a tool is behaving impossibly, and it
is a poor first one.

### The same idea, elsewhere

User level, project level, local override is not a Claude Code invention. Git does exactly the
same thing with system, global, and local config ([b3](#b3-tell-git-who-you-are)), and so do
most editors, linters, and build tools. Once you have the pattern, a new tool's settings
layout is usually a five-minute read rather than a mystery.
