---
id: d10-undo-everything
title: Undoing things, ranked by how much they destroy
type: section
track: D
order: 100
verified: 2026-08-02
volatility: low
verify: git status
danger: >
  This card contains the sharp commands. `git restore` throws away uncommitted
  edits to a file. `git reset --hard` throws away a commit and every uncommitted
  change in your folder at the same time. `git clean` deletes untracked files off
  disk. None of those three has an undo for uncommitted work. Run
  `git stash push -u` first if there is any chance you want it back, and use
  `git revert` instead of `git reset` for anything already pushed.
answer: >
  Pick the undo by what you want back: `git restore` for file edits,
  `git restore --staged` to unstage, `git reset --soft HEAD~1` to undo a commit
  and keep the work, `git revert` for anything already pushed.
owns:
  - restore
  - revert
  - reset
  - stash
  - reflog
  - the undo decision
see_also:
  - d3-the-three-places
  - d11-when-you-lose-work
  - d4-commit-well
  - d7-merge-conflicts
  - d5-branches
  - e11-what-to-never-let-an-agent-do
keywords:
  - undo
  - take it back
  - revert
  - reset
  - unstage
  - throw away my changes
  - go back to before
  - stash
  - i broke it
---

## More

There is no single undo in git. There are six, and picking the right one is a question of
what you want back, not how git works. Find your row.

| What you want back | Command | What it destroys |
|---|---|---|
| The file, the way it was at your last commit | `git restore <file>` | every uncommitted edit to that file |
| A file out of the staging area, contents untouched | `git restore --staged <file>` | nothing |
| The commit undone, the work kept | `git reset --soft HEAD~1` | nothing |
| The commit and the work both gone | `git reset --hard HEAD~1` | the commit and every uncommitted change |
| A commit undone that is already on GitHub | `git revert <hash>` | nothing, it adds a new commit |
| Your work parked somewhere safe for an hour | `git stash push -u` | nothing, it is recoverable |

Two rules make the choice for you most of the time.

**If it is already pushed, use `revert`.** Revert is safe. It does not remove anything. It
writes a new commit that is the mirror image of an old one, so the history still shows both
what happened and that you took it back. Nobody else's copy breaks.

**If it is not pushed, `reset` is available, and `reset` is sharp.** It moves your branch
pointer backward. `--soft` keeps everything, plain `reset` keeps your files but unstages
them, and `--hard` wipes your files back to match. Only `--hard` destroys anything, and it
destroys a lot.

Before any of them, run this:

```powershell
git status
```

It tells you which of the three places your work is currently in, which is the thing the
command you are about to run depends on. [d3](#d3-the-three-places) reads the output.

And if you undo too much, you are probably still fine. Git keeps a log of every position
you have been in, and [d11](#d11-when-you-lose-work) is how you get back. The one thing it
cannot recover is uncommitted work destroyed by `--hard`.

## Full

### Undo edits to a file you have not committed

You changed a file, or an agent did, and you want the last committed version back.

This throws away every edit to that file since your last commit. There is no undo for it.
The file goes back to its committed state and your changes are gone off disk.

```powershell
git restore src/login.js
```

Verify with `git status`: the file no longer appears under changes. Verify with `git diff`:
it prints nothing for that file.

Restoring everything at once is the same command with a dot, and the same warning applies
to every changed file in the project at once:

```powershell
git restore .
```

If you want it back but are not certain, shelve instead of restoring. `git stash push -u`
gets you the same clean folder and keeps a copy.

If you want the version from further back rather than the last commit:

```powershell
git restore --source=HEAD~2 src/login.js
```

`HEAD` is where you are now and `HEAD~2` is two commits earlier. That pulls the old contents
into your file without moving your branch anywhere.

Older documentation says `git checkout -- src/login.js` for this. It does the same job.
`restore` arrived in git 2.23 to split the overloaded `checkout` command in two, and it is
the one to use because it cannot accidentally switch your branch.

### Unstage something

You ran `git add` on a file you did not mean to include.

This destroys nothing. Your file's contents do not change at all. The only thing that moves
is whether the change is queued for the next commit.

```powershell
git restore --staged src/login.js
```

Verify with `git status`: the file moves from the staged group to the unstaged group. The
edits are still there.

`git reset src/login.js` does the same thing and is what you will see in older answers. Both
work. `restore --staged` says what it means.

### Undo a commit but keep the work

You committed too early, or with the wrong message, or with a file you did not mean to
include.

This destroys nothing. The commit is removed from your branch and every change it contained
lands back in your staging area, ready to be committed again.

```powershell
git reset --soft HEAD~1
```

Verify with `git log --oneline -3`: the commit is gone from the top. Verify with
`git status`: its contents are sitting there staged. Fix what you need to fix, then commit
again.

`HEAD~1` means one commit back. `HEAD~3` means three. For only the message, there is a
narrower tool:

```powershell
git commit --amend -m "fix: correct the login redirect"
```

That replaces the last commit with a new one carrying the new message. Do not amend a commit
you have already pushed, because the pushed copy and your copy stop matching and the next
push is rejected.

Plain `git reset HEAD~1`, with no flag, is the middle setting. The commit goes away, your
files keep every change, and nothing is staged. Use it when you want to rebuild the commit
from scratch in different pieces.

### Undo a commit and the work with it

You want the last commit gone and you do not want its changes back in any form.

This is the sharp one. It destroys the commit, and it destroys every uncommitted change
anywhere in your folder, including ones that have nothing to do with that commit. There is
no undo for the uncommitted part.

```powershell
git reset --hard HEAD~1
```

Before you run it, run `git stash push -u` instead if there is any chance you want any of it
back. Stashing takes two seconds and makes the whole thing reversible.

Verify with `git log --oneline -3` and `git status`: the commit is gone and the folder is
clean.

The commit itself is not really gone for about ninety days. It is unreachable, not deleted,
and [d11](#d11-when-you-lose-work) walks it back. The uncommitted changes are the part that
is genuinely unrecoverable, and that is the entire argument for committing often.

### Undo a commit that is already pushed

Once a commit is on GitHub, other copies of it may exist, including your own on another
machine. Rewriting history under them causes a different and worse problem than the one you
started with.

This destroys nothing. It creates a new commit whose content is the exact opposite of the
one you name.

```powershell
git revert --no-edit HEAD
```

`--no-edit` accepts the generated message instead of opening an editor. To undo something
further back, name it by its hash from `git log --oneline`:

```powershell
git revert --no-edit 4f2c1ab
```

Verify with `git log --oneline -3`: you now have one more commit, called
`Revert "the original message"`. Verify the file itself looks the way you wanted. Then push
normally.

If the commit you are reverting was a merge, git needs to know which side to keep and will
refuse without `-m 1`. That case is [d6](#d6-merge-and-rebase).

Reverting a revert is legal and works exactly the way you would guess.

### Shelve work you are not ready to commit

You need a clean folder right now, to switch branches or try something, and the work is not
finished enough to commit.

This destroys nothing. It takes every change out of your files and stores it, then gives you
a clean working folder.

```powershell
git stash push -u -m "half-done login form"
```

`-u` includes untracked files, which are new files git has never seen. Without it they stay
behind, which is the most common way people lose work to stash. `-m` labels the entry so
`git stash list` is readable a week later.

Verify with `git status`: clean. Verify with `git stash list`: your entry is there as
`stash@{0}`.

Getting it back:

```powershell
git stash apply
```

Copies the work back into your files and keeps the stored copy as a safety net.
`git stash pop` does the same and deletes the stored copy on success, which is fine once you
trust it. Clear an entry you are done with using `git stash drop`.

Stash is per repository, not per branch. You can stash on one branch and apply on another,
which is exactly how you fix work started in the wrong place.

### Delete files git is not tracking

Untracked junk, usually generated files or leftovers from a failed build, that `git restore`
will not touch because git never tracked it.

This deletes files off disk permanently. They were never committed, so nothing in git can
bring them back. Always run the preview first.

```powershell
git clean -n -d
```

`-n` means show me, do not do it. `-d` includes directories. Read that list carefully. Then,
only if the list is correct:

```powershell
git clean -f -d
```

`-f` means force, and git requires it because this command has no undo. Verify with
`git status`: the untracked section is empty.

### The whole ladder, sorted by damage

1. `git stash push -u` destroys nothing and is reversible.
2. `git restore --staged` destroys nothing.
3. `git reset --soft HEAD~1` destroys nothing.
4. `git revert` destroys nothing and is the only safe option once pushed.
5. `git reset HEAD~1` destroys nothing, but unstages, so you have to rebuild the commit.
6. `git restore <file>` destroys uncommitted edits to that file.
7. `git reset --hard` destroys the commit and every uncommitted change in the folder.
8. `git clean -f -d` destroys untracked files off disk.

Anything at position 6 or below deserves a `git stash push -u` first. It costs two seconds
and it converts an unrecoverable mistake into a recoverable one.

### If you undid too much

Go to [d11](#d11-when-you-lose-work) and run `git reflog`. It lists every position your
branch has been in, including the ones you reset away from, and you can make a branch at any
of them. Committed work is almost never actually gone.
