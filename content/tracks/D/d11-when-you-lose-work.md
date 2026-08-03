---
id: d11-when-you-lose-work
title: When you think you lost work
type: section
track: D
order: 110
verified: 2026-08-02
volatility: low
verify: git reflog -5
danger: >
  This card explains recovery from `git reset --hard`, which is the command that
  causes most of the panics it treats. `git reset --hard` destroys every
  uncommitted change in your folder and those are the one thing reflog cannot
  bring back. The safe alternative before any sharp command is
  `git stash push -u`.
answer: >
  Almost nothing you have committed is ever really gone. `git reflog` lists every
  position you have been in for the last ninety days and you can make a branch at
  any of them. Uncommitted changes are the one real exception.
owns:
  - reflog recovery
  - dangling commits
  - the recovery mindset
see_also:
  - d10-undo-everything
  - d4-commit-well
  - d5-branches
  - d3-the-three-places
keywords:
  - lost my work
  - reflog
  - my commits are gone
  - deleted the wrong branch
  - recover a commit
  - undo a reset
  - dangling commit
  - i lost everything
---

## More

Start from the correct assumption: if the work was ever in a commit, it is almost certainly
still on your machine right now. Git does not delete commits when you reset, rebase, or
delete a branch. It stops pointing at them. The commit sits in the repository, unreferenced,
for about ninety days before garbage collection touches it, and until then it is one command
away.

The command nobody tells beginners about is this one.

```powershell
git reflog
```

The **reflog** is a private log of every position `HEAD` has occupied in this repository:
every commit, every branch switch, every reset, every merge, every rebase step. It is local
to your machine, it is not pushed anywhere, and it records things that no longer appear in
`git log`. That is the whole point of it.

The output looks like this:

```text
9c1f0aa (HEAD -> main) HEAD@{0}: reset: moving to HEAD~1
4b7e2d3 HEAD@{1}: commit: feat: add password reset
1a55c0e HEAD@{2}: commit: feat: add login form
```

Read it top down, newest first. The left column is the commit hash. `HEAD@{1}` is a
nickname meaning "one move ago." The text after the colon is what you did.

In that example the work called `add password reset` looks gone from `git log`, because the
reset moved past it. It is right there at `4b7e2d3`. Getting it back is one command, and it
destroys nothing:

```powershell
git switch -c rescue-work 4b7e2d3
```

That makes a new branch called `rescue-work` sitting exactly where you were before the
mistake, and puts you on it. Verify with `git log --oneline -3`: your commit is back at the
top. Nothing else in the repository changed, so if you grabbed the wrong entry you can
delete the branch and try another.

The exception, stated honestly: **uncommitted changes are not in the reflog**, because they
were never in a commit. If `git reset --hard` or `git clean -f` or `git restore` wiped work
you had never committed, git has nothing to recover from. That is the one real hole, and it
is the argument for committing often. A commit every twenty minutes turns almost every
possible disaster into an inconvenience.

## Full

### Work that is not lost at all

Before recovering anything, rule out the three cases where the work is fine and you are
looking in the wrong place. All three are more common than actual loss.

```powershell
git branch --show-current
```

Prints the branch you are on. If it prints nothing at all, you are not on a branch, you are
parked on a specific commit, and your work is probably on the branch you left.

```powershell
git log --oneline --all -20
```

`--all` shows commits on every branch, not the one you are standing on. Your work often
appears here immediately, on a branch you forgot you made.

```powershell
git stash list
```

Shows shelved work. Agents stash without much ceremony, and a stash makes your files look
exactly as though your changes evaporated. If a line appears here, `git stash apply` brings
it back.

### Reading the reflog properly

```powershell
git reflog -30
```

Shows the last thirty moves. Each line ends with what caused it, and those labels are how
you find the moment before the mistake:

- `commit:` you made a commit.
- `checkout: moving from X to Y` you switched branches.
- `reset: moving to <target>` you reset. The entry directly below this one is where you
  were before it.
- `rebase (finish):` a rebase completed. Entries above it are the rewritten history.
- `merge <branch>:` you merged.
- `pull:` you pulled.

Find the last line that describes the state you want. Take the hash from the left column.
Make a branch at it. That is the whole method.

You can also use the nickname directly, but on Windows it needs quotes:

```powershell
git switch -c rescue-work "HEAD@{4}"
```

PowerShell treats `@{` as the start of a hashtable, so without the quotes you get a
confusing parse error rather than a git error. Quoting it always works.

### Recovering a branch you deleted

Deleting a branch does not delete its commits. It deletes the name.

```powershell
git reflog
```

Find the last entry from that branch, take the hash, and recreate the name pointing at it:

```powershell
git branch feature/login 4b7e2d3
```

Verify with `git log --oneline feature/login -5`. If the commits are there, the branch is
back exactly as it was. If the branch was pushed before you deleted it locally, you have an
even easier route: it is still on GitHub, and `git fetch origin` followed by
`git switch feature/login` brings it down.

### When the reflog does not show it

The reflog follows `HEAD`, so work committed in a state you never visited with `HEAD` can be
missing from it. Rare, but it happens after a botched rebase.

```powershell
git fsck --lost-found
```

Walks the entire object database and reports everything not reachable from any branch or
tag. Output looks like `dangling commit 7c9d21a`. Inspect a candidate before you trust it:

```powershell
git show 7c9d21a
```

That prints the commit's message and its diff. When you find the right one, recover it the
same way as anything else:

```powershell
git switch -c rescue-work 7c9d21a
```

This command is slow on a large repository and its output is noisy, including dangling blobs
that are just intermediate saves. Read past those and look for `dangling commit`.

### Recovering a stash you dropped

A dropped stash is unreferenced, not deleted, so the same trick works.

```powershell
git fsck --unreachable | Select-String "commit"
```

`Select-String` is PowerShell's text search, so this filters the noise down to commit
objects. Check each with `git show <hash>` until you find your work, then:

```powershell
git stash apply 7c9d21a
```

A stash is stored as a commit, so `git stash apply` accepts a raw hash.

### The one thing that is actually gone

Work that was never committed and never stashed does not exist anywhere inside git. If it
was destroyed by `git reset --hard`, `git clean -f`, `git restore`, or an agent overwriting
the file, git has no copy and no command will produce one.

Three places outside git are worth checking before you give up.

**Your editor's local history.** Visual Studio Code keeps its own record of every save,
independent of git. Open the file, then open the Timeline section at the bottom of the
Explorer panel. Every save is listed and you can view or restore any of them. This has
rescued more people than any git command. JetBrains editors have the same feature under
Local History.

**The Recycle Bin.** If the file was deleted through File Explorer rather than by a command,
it is there.

```powershell
Start-Process shell:RecycleBinFolder
```

**File version history.** If the folder is inside OneDrive, right-click the file on the
OneDrive website and choose Version history. If Windows File History is switched on, the
Previous Versions tab in the file's Properties dialog does the same job.

If none of those has it, it is gone, and the honest response is to say so and rebuild rather
than to keep running recovery commands hoping.

### The habit that makes this card unnecessary

Every recovery above works because the work was in a commit. The gap in coverage is exactly
the window between your last commit and now.

Commit small and commit often. A commit is not a publication and it does not have to be
finished or correct. `git commit -m "wip: half of the login form"` costs three seconds and
converts everything after it from unrecoverable to recoverable. You can always tidy the
history later with a squash merge when the branch lands: [d8](#d8-pull-requests).

Before any sharp command, take the two-second insurance:

```powershell
git stash push -u -m "insurance"
```

Then run the thing you were worried about. If it goes wrong, `git stash apply` puts you back.
[d10](#d10-undo-everything) ranks every undo command by exactly how much it destroys.
