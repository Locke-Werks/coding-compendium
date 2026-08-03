---
id: b8-turn-off-ai-attribution
title: Turn off AI attribution
type: section
track: B
order: 80
verified: 2026-08-02
volatility: weekly
verify: git log -1 --format=%B
answer: >
  Set the `attribution` block to empty strings in Claude Code's settings file and
  `commit_attribution = ""` in Codex's config, then check with
  `git log -1 --format=%B` that nothing extra is appended to your commit message.
owns:
  - attribution settings
  - the commit-msg hook
see_also:
  - d4-commit-well
  - b9-where-settings-live
  - e4-claude-md-and-agents-md
keywords:
  - co-authored-by
  - remove claude from commits
  - commit trailer
  - attribution setting
  - commit-msg hook
  - clean git history
  - generated with
---

## More

Left alone, some of these tools sign their work. A co-author trailer gets appended to your
commit messages, a footer gets added to pull request descriptions, and a link back to the
session sometimes comes along with it. If you want a git history that reads as though you
wrote it, four settings turn all of that off. Set them once, on the machine, and every
project inherits them.

**Claude Code.** Open or create `C:\Users\<yourname>\.claude\settings.json` and put this in
it:

```json
{
  "attribution": {
    "commit": "",
    "pr": "",
    "sessionUrl": false
  }
}
```

Empty strings mean nothing is appended. `sessionUrl: false` stops the link back to the
session from being added.

**Codex.** In `C:\Users\<yourname>\.codex\config.toml`:

```toml
commit_attribution = ""
```

**Claude Desktop.** Its coding work runs the same engine and reads the same settings file, so
the block above already covers it. For plain chat and documents, open Settings and put a
standing line in your profile Preferences telling it never to sign or credit its output.

**Both instruction files.** Add one plain-language line to each project's `CLAUDE.md` and
`AGENTS.md` ([e4](#e4-claude-md-and-agents-md)). A settings file can go missing or a key can
be renamed; an instruction in prose survives both.

Then confirm on your next commit:

```powershell
git log -1 --format=%B
```

`-1` means the most recent commit and `--format=%B` prints its message body in full. What you
typed, and nothing under it.

## Full

### Claude Code, in detail

The mechanism changed once already. The old `includeCoAuthoredBy` key is deprecated, meaning
retired and no longer the right way to do this, and the `attribution` block replaced it and
takes precedence over it. If you find the old key in an old set of instructions, use the
block instead.

The three fields:

| Field | What it controls | Set it to |
|---|---|---|
| `commit` | text appended to commit messages | `""` |
| `pr` | text appended to pull request descriptions | `""` |
| `sessionUrl` | whether a link back to the session is added | `false` |

Putting the block in `C:\Users\<yourname>\.claude\settings.json` applies it to every project
on the machine, which is what you want. Putting the same block in a project's
`.claude\settings.json` commits the rule alongside the code, which is what you want when
other people work on the repository. Both can exist and the narrower one wins, which is
[b9](#b9-where-settings-live).

The official reference is https://code.claude.com/docs/en/settings, in the attribution
section. Check it if a commit ever comes out signed anyway.

### Codex, in detail

Codex added nothing at all for most of its life, then gained a single config key. When that
key is active and unset, it inserts a co-author trailer naming Codex. Setting it to an empty
string guarantees nothing is added:

```toml
# C:\Users\<yourname>\.codex\config.toml
commit_attribution = ""
```

Its commit behavior has shifted across versions and can be tied to a feature flag, so the
plain-language line in `AGENTS.md` is doing real work here rather than repeating the setting.
Together they cover you regardless of which version is installed.

### The line to put in the instruction files

Paste this into `CLAUDE.md` and `AGENTS.md` at the root of each project:

```text
Never add co-author lines, "generated with" footers, or any AI attribution to
commits, pull requests, or git metadata.
```

It reads as an instruction to the model rather than a switch on the program, which is exactly
why it is worth having. It applies to things no settings key covers, like a footer on a file
it writes or a note in a pull request body it composes.

### The guarantee that sits below every tool

If you want a hard stop that does not depend on any tool's settings, add a git hook. A hook is
a script git runs automatically at a set moment. This one runs every time a commit message is
written in that repository, strips any co-author trailer, and does not care which program
produced it.

It is bash, so run it in **Git Bash**, not PowerShell, from inside the project folder:

```bash
cat > .git/hooks/commit-msg << 'EOF'
#!/bin/sh
sed -i.bak '/^Co-Authored-By:/d' "$1"
rm -f "$1.bak"
EOF
chmod +x .git/hooks/commit-msg
```

`cat > file << 'EOF'` writes everything up to the next `EOF` line into that file. `sed -i`
edits a file in place, and the pattern deletes any line starting with that trailer. `chmod +x`
marks the file executable, which is what makes git willing to run it.

Two things to know about hooks. They live in `.git\hooks\` and git does not share them, so
they are per-project and a fresh clone will not have them. And git on Windows runs hooks
through its own bundled shell, so a bash script works even though the rest of your work is in
PowerShell.

For most people the two settings above are enough and the hook is for the genuinely paranoid.

### Checking your history

The last commit:

```powershell
git log -1 --format=%B
```

The last twenty, searched for anything that slipped through:

```powershell
git log -20 --format=%B | Select-String -Pattern "Co-Authored|Generated with"
```

`Select-String` is PowerShell's text search. No output means nothing matched, which is the
result you want. If something did match, the setting was added after those commits were made;
they keep whatever message they were created with, and rewriting old commit messages changes
every hash from that point forward ([d1](#d1-what-git-actually-stores)). On a private
project not yet shared, that is a fine trade. On anything already pushed and shared, leave the
old commits alone and let the setting take care of the new ones.

### Why this card expires quickly

The Claude Code setting has already changed once, from a single boolean key to a block. The
Codex key arrived after the tool had shipped without one for a year. Both tools ship changes
weekly, which is why this card carries a weekly freshness budget and shows a staleness badge
sooner than most. The check command at the top is the real defense: it takes two seconds and
it tests the actual outcome rather than the configuration.
