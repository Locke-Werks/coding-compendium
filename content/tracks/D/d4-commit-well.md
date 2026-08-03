---
id: d4-commit-well
title: Committing, and what makes a good commit
type: section
track: D
order: 40
verified: 2026-08-02
volatility: low
answer: >
  Stage what you want with git add, record it with git commit -m "fix: stop
  logout on session timeout", and keep each commit to one change. Commit often:
  every commit is a point you can get back to.
owns:
  - commit
  - conventional commits
  - commit message style
see_also:
  - d3-the-three-places
  - a4-the-loop
  - b3-tell-git-who-you-are
  - d1-what-git-actually-stores
keywords:
  - git commit
  - commit message
  - feat fix chore
  - conventional commits
  - how often should i commit
  - checkpoint
  - git add then commit
---

## More

Saving work in git is two moves. Choose what goes in, then record it.

```powershell
git add src/auth.js src/session.js
```

Names the files whose current state you want in the next commit. Naming them beats
`git add .`, which sweeps in everything that changed, including the file you were
experimenting in.
Verify with `git status`: those files now sit under "Changes to be committed"
([d3](#d3-the-three-places)).

```powershell
git commit -m "fix: stop logout on session timeout"
```

Records a snapshot of everything staged, with that message attached. `-m` supplies the
message inline. Verify with `git log --oneline -1`, which prints the new commit's short
hash and your message.

The message uses **conventional commits**, a convention where the message opens with a type
and a colon:

- `feat:` a new feature
- `fix:` a bug fix
- `docs:` documentation only
- `refactor:` restructuring that changes no behavior
- `test:` adding or repairing tests
- `chore:` maintenance, dependencies, configuration

Keep the summary under about 72 characters, write it in the present tense, and describe the
effect rather than the mechanics. `fix: stop logout on session timeout` tells you what
changed for a person using the thing. `fix: update auth.js` tells you nothing you could not
get from the file list.

One change per commit. When a message needs the word "and", you probably have two commits.
This is not tidiness for its own sake: a commit that does one thing is a commit you can
undo on its own ([d10](#d10-undo-everything)), and a history of them is one you can read
later to find where something broke.

Commit far more often than feels natural. Before you try something risky, after anything
works, at the end of every step an agent completes. A commit costs a second and creates a
point you can return to, which is the entire safety net underneath working with an agent
that moves faster than you can read. Uncommitted work is the only work git cannot get back
for you.

## Full

### The message, in full form

The `-m` flag is for short messages. When a change needs explaining, leave `-m` off and git
opens an editor for a longer message, which has a fixed shape:

```text
fix: stop logout on session timeout

The session token was compared with the refresh token's expiry
rather than its own, so any session older than the refresh window
was treated as expired.

Closes #42
```

A summary line under about 72 characters. A blank line, which is mandatory: git treats the
first line as the title everywhere it displays a commit, and without the blank line your
whole paragraph becomes the title. Then the body, explaining why the change was needed. The
diff already shows what changed, so the body is where the reason goes.

Referencing an issue number as `Closes #42` makes GitHub close that issue when the commit
reaches the default branch.

### The editor trap

Run `git commit` without `-m` and Git for Windows opens whatever editor it was configured
with at install time, sometimes Vim, which fills your terminal with a file and no visible
way out. To escape without committing: press `Esc`, type `:q!`, press Enter. To save and
commit: press `Esc`, type `:wq`, press Enter. To avoid the situation entirely, keep using
`-m`.

### Checking what you actually committed

```powershell
git show --stat HEAD
```

Prints the newest commit's message and a list of the files it touched with a count of lines
added and removed per file. It is the fastest way to catch a commit that swept in a file you
did not mean to include. `HEAD` means the commit you are on ([d3](#d3-the-three-places)).

```powershell
git log --oneline -10
```

The last ten commits, one line each. Read this after any session where an agent committed on
your behalf. You are checking that the messages match what you asked for and that no
attribution line was added ([b8](#b8-turn-off-ai-attribution)).

### Two failures you will meet in the first week

`nothing to commit, working tree clean` means git sees no difference from the last commit.
Either you forgot the `git add` step, or you saved the file in your editor and think you
changed it but did not.

`Author identity unknown` or `Please tell me who you are` means git has no name and email
to stamp the commit with, and it stops rather than guessing. This is a one-time setup step,
covered in [b3](#b3-tell-git-who-you-are).

### The shortcut and its trap

```powershell
git commit -am "chore: update dependencies"
```

`-a` stages every tracked file that changed, then commits, skipping `git add` entirely. Two
things it will not do: it ignores untracked files, so a brand new file is silently left out,
and it makes no distinction between the change you reviewed and the three you had not
looked at yet. Use it when you know the whole working folder is one change. Use `git add`
by name the rest of the time.

### Fixing the commit you just made

```powershell
git commit --amend -m "fix: stop logout on session timeout"
```

Replaces the newest commit with a new one carrying the corrected message, and stages
anything you have added since. Because a commit's hash is computed from its contents, the
replacement is a different commit with a different hash, not an edited version of the old
one ([d1](#d1-what-git-actually-stores)).

That is the catch. Amend a commit you have already pushed and your local history no longer
matches GitHub's, and the next push is rejected. Amend freely before pushing. After
pushing, treat it as the history rewrite it is: [d6](#d6-merge-and-rebase) explains what
that costs and how to do it deliberately.

### Committing when an agent is doing the work

Ask for the commit at the end of each step, not at the end of the session. A session that
produced one enormous commit gives you one undo button that throws away four hours. A
session that produced nine gives you nine.

Put the convention in your instructions file so you stop repeating it: a line saying
"conventional commit messages, one logical change per commit, no attribution trailers" in
`CLAUDE.md` or `AGENTS.md` gets followed automatically ([e4](#e4-claude-md-and-agents-md)).

Review before you accept the commit, not after. Reading `git diff --staged` takes a minute
and is the last cheap moment to catch a change nobody asked for. Once it is committed you
are into undo territory, which is a longer conversation
([h3](#h3-reviewing-a-diff-you-cannot-fully-read)).
