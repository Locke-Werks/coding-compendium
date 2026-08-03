---
id: d12-gitignore-and-what-not-to-commit
title: .gitignore, and what must never be committed
type: section
track: D
order: 120
verified: 2026-08-02
volatility: low
verify: git status --ignored --short
danger: >
  `git rm --cached` stops git tracking a file and leaves it on disk, which is
  safe. Dropping `--cached` deletes the file. `git clean -X -f` deletes every
  ignored file off disk with no undo, so run it with `-n` first to see the list.
answer: >
  `.gitignore` is a list of path patterns git will not offer to track, one per
  line, committed with the project. It only works on files git is not already
  tracking, which is why adding a rule for a file you committed changes nothing.
owns:
  - gitignore
  - build output
  - node_modules
see_also:
  - g6-secrets-and-what-never-to-commit
  - gitignore
  - g3-lockfiles
  - i3-builds-and-artifacts
  - g5-environment-variables
  - d3-the-three-places
keywords:
  - gitignore
  - node_modules committed
  - stop tracking a file
  - untrack
  - dont commit
  - ignored file still showing
  - git rm cached
---

## More

`.gitignore` is a plain text file in your project root listing paths git should pretend it
cannot see. One pattern per line. Anything matching a pattern never appears in `git status`,
never gets picked up by `git add .`, and never reaches GitHub.

The file itself is committed. That is deliberate: everyone who clones the project should
ignore the same junk.

Three categories belong in it, and the reasoning is different for each.

- **Dependencies.** `node_modules/`, `.venv/`, `target/`, `vendor/`. Other people's code,
  reinstallable from the manifest in one command, often hundreds of megabytes. Commit the
  lockfile instead, which is [g3](#g3-lockfiles).
- **Build output.** `dist/`, `build/`, `out/`, `*.exe`. Generated from your source, so
  committing it stores the same information twice and guarantees the two versions disagree.
  [i3](#i3-builds-and-artifacts) covers why deleting it is always safe.
- **Secrets and local settings.** `.env`, `*.pem`, `secrets.json`, local database files.
  These must never leave your machine. [g6](#g6-secrets-and-what-never-to-commit) is the
  card that matters if one already has.

Now the part that catches everyone, including agents:

**`.gitignore` only applies to files git is not already tracking.** If `.env` was committed
once, adding `.env` to `.gitignore` does nothing at all. Git keeps tracking it, keeps
showing its changes, and keeps pushing them. The ignore rule is not a retroactive eraser.

The fix is one command, and it leaves the file on your disk:

```powershell
git rm --cached .env
```

`--cached` is doing the work: remove it from git's tracking, leave the actual file alone.
Without that flag the command deletes the file. Then commit the removal. Verify with
`git status`: the file should now be invisible rather than listed as deleted.

That untracks it from this point forward. It does not remove it from past commits, and if
the file held a credential, the credential is still readable in the history and must be
treated as compromised. Go to [g6](#g6-secrets-and-what-never-to-commit).

## Full

### Where the file goes and how far it reaches

The main `.gitignore` sits in the repository root, next to the `.git` folder. It applies to
everything below it.

You can also put a `.gitignore` inside any subfolder, where it applies to that folder and
its children and adds to whatever the parent files already said. This is useful in exactly
one situation: a folder with its own generated output that nobody else's rules should have
to know about. Otherwise keep one file at the root, because ignore rules scattered across
six folders are unreadable when something goes missing.

### The patterns you actually need

| Pattern | Matches |
|---|---|
| `node_modules/` | a folder called `node_modules`, anywhere in the tree |
| `*.log` | any file ending in `.log`, anywhere |
| `/dist` | `dist` in the root only, not `src/dist` |
| `build/*.tmp` | `.tmp` files directly inside `build`, not deeper |
| `**/cache/` | a folder called `cache` at any depth |
| `!keep.log` | un-ignores `keep.log` after a broader rule caught it |
| `# comment` | a comment, ignored by git |

A leading slash anchors the pattern to the repository root. A trailing slash restricts it to
folders. No slash at all means "match this name anywhere." The
[gitignore card](#gitignore) has the full syntax.

One gotcha with `!`: you cannot un-ignore a file if one of its parent folders is ignored.
Git never looks inside an excluded folder, so it never sees the file to reconsider it. If
you need one file out of an ignored folder, exclude the folder's contents rather than the
folder:

```gitignore
logs/*
!logs/keep-this.log
```

### Untracking something already committed

The single-file version is `git rm --cached <file>`. For a folder that has been getting
committed for weeks, usually `node_modules`, the recursive form:

```powershell
git rm -r --cached node_modules
```

`-r` means recurse into the folder. `--cached` means untrack it, do not delete it. Then add
the rule to `.gitignore` if it is not there and commit both changes together:

```powershell
git commit -m "chore: stop tracking node_modules"
```

Verify with `git status`, which should be clean, and with `git ls-files node_modules`, which
should print nothing. `git ls-files` lists what git is tracking, so silence is the answer
you want.

The repository does not get smaller. Every past commit still contains those files and the
`.git` folder still stores them. Cleaning that requires rewriting history, which is worth
doing only for a repository that has become genuinely unusable. It is not the fix for a
leaked credential: revoke that instead ([g6](#g6-secrets-and-what-never-to-commit)).

### Why a file you ignored is still showing up

```powershell
git check-ignore -v path/to/file
```

This answers the question directly. It prints the file that contains the matching rule, the
line number, and the rule itself:

```text
.gitignore:4:node_modules/	node_modules/express/index.js
```

If it prints nothing, no rule matches and that is your answer. The two usual causes are a
rule that is subtly wrong, and a file that was already tracked before the rule existed. The
second is far more common. `git ls-files <path>` tells you which case you are in: if the
file is listed, it is tracked and the ignore rule was never going to help.

To see everything currently being ignored:

```powershell
git status --ignored --short
```

Useful once, to confirm the rules do what you think. It is a long list on any project with
dependencies installed.

### Your own ignores, not the project's

Some things you want ignored are yours rather than the project's: your editor's settings, a
scratch folder, notes. Putting those in the shared `.gitignore` clutters it for everyone.

Two places take personal rules instead.

```powershell
git config --global core.excludesFile "$env:USERPROFILE\.gitignore_global"
```

Creates a machine-wide ignore file at `C:\Users\<yourname>\.gitignore_global`, applied to
every repository you touch. Good home for `Thumbs.db`, `desktop.ini`, and your editor's
folder. Verify with `git config --global core.excludesFile`, which should echo the path
back.

For rules that apply to one project only and should not be committed, edit
`.git\info\exclude` in that project. Same syntax, never pushed, and almost nobody knows it
exists.

### What people wrongly ignore

**Lockfiles.** `package-lock.json`, `pnpm-lock.yaml`, `Cargo.lock`, `poetry.lock`. These are
the record of exactly which versions you got, and they belong in the repository.
[g3](#g3-lockfiles) explains why.

**The whole `.vscode` folder.** Some of it is personal and some of it is the project's:
recommended extensions and debug configurations help anyone who opens the project. Ignoring
the folder wholesale throws away the useful part.

**Configuration files with a placeholder.** Ignore `.env`, then commit a `.env.example`
holding the same keys with empty or fake values. That way the next person, including you on
a new machine, knows which variables the project needs.
[g5](#g5-environment-variables) covers what those files hold.

### Starting from a decent file

GitHub maintains a template per language at `github.com/github/gitignore`. When you create a
repository through the website you can select one. For an existing project, ask your agent
for the file matching your stack and then read what it produced. Agents tend to generate
enormous ignore files covering nine ecosystems the project does not use, which is clutter
rather than a problem, and then miss the one entry that mattered. Check that your build
output folder and your `.env` are both in there.

### Deleting the ignored junk

Ignored files pile up. To clear them out, preview first:

```powershell
git clean -n -X -d
```

`-n` shows what would be deleted without doing it, `-X` limits it to ignored files only, and
`-d` includes folders. Read the list. Then, if it is right, swap `-n` for `-f`. There is no
undo: these files were never tracked, so nothing in git can restore them. Everything in the
list is regenerable by definition, but that is only true if you actually read the list first.
