---
id: d3-the-three-places
title: The three places your work can be
type: section
track: D
order: 30
verified: 2026-08-02
volatility: low
answer: >
  Your work sits in one of three places: the working folder on disk, the staging
  area holding what goes into the next commit, and the committed history.
  Reading git status tells you which place each changed file is in.
owns:
  - working directory
  - staging area
  - index
  - HEAD
  - git status
see_also:
  - d10-undo-everything
  - d4-commit-well
  - d1-what-git-actually-stores
  - d9-reading-a-diff
keywords:
  - staging area
  - git add
  - what does git status mean
  - untracked files
  - changes not staged for commit
  - index
  - where is my work
---

## More

Git does not record your work when you save the file. A change has to travel through three
stops before it is safely stored, and nearly every git command you will ever run exists to
move it from one stop to the next.

1. **The working folder.** The files on your disk, exactly as they are right now. This is
   what your editor shows you and what the program runs. Edits land here first, and
   nothing else knows about them.
2. **The staging area.** A list of the changes you have chosen to include in your next
   commit. `git add` puts a change here. Nothing is saved permanently yet.
3. **The committed history.** The permanent record. `git commit` takes everything in the
   staging area and freezes it as a snapshot ([d1](#d1-what-git-actually-stores)).

Work moves one way through those three, and one command does each step:

```powershell
git add notes.md
```

Copies the current state of `notes.md` into the staging area. Verify with `git status`:
the file moves from the "Changes not staged for commit" list to the "Changes to be
committed" list.

```powershell
git commit -m "docs: add project notes"
```

Freezes everything staged into a snapshot. Verify with `git status` again: the file is now
in none of the lists, and git says `nothing to commit, working tree clean`.
[d4](#d4-commit-well) covers what to put in the message.

`git status` is the answer to "where am I", and it is worth running constantly. It tells
you your current branch, then sorts every changed file into a list by which place it is
in:

- **Changes to be committed.** Staged, going into the next commit.
- **Changes not staged for commit.** Edited on disk, tracked by git, but not staged.
- **Untracked files.** On disk and git has never been told about them. A new file starts
  here.

The wording is stiff, and it is also literal. Once you can map those three headings onto
the three places, `git status` stops being noise and becomes the only orientation tool you
need. **HEAD** is git's fourth piece of vocabulary here: the pointer to where you are in
history, which is normally the newest commit on your current branch and the one your next
commit will attach to.

## Full

### The worked example, with real output

Start in a repository with nothing in progress. Create two files, stage one of them, and
watch where each one shows up.

```powershell
git status
```

```text
On branch main
Changes to be committed:
  (use "git restore --staged <file>..." to unstage)
        modified:   notes.md

Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)
  (use "git restore <file>..." to discard changes in working directory)
        modified:   src/app.js

Untracked files:
  (use "git add <file>..." to include in what will be committed)
        scratch.txt
```

Read it as three answers to one question. `notes.md` is staged and will be in the next
commit. `src/app.js` has been edited and will not be, unless you add it. `scratch.txt` is
new and git is ignoring it until told otherwise.

Git prints the command for each move in the parenthesized hints, which is genuinely
helpful and also the reason people paste `git restore` at moments they meant something
else. What each undo command destroys is ranked in [d10](#d10-undo-everything). Read that
before running any of them.

### One file, four possible states

- **Untracked.** Git has never stored this file. It is invisible to commits until you
  `git add` it once.
- **Unmodified.** Tracked, and identical to the last commit. It shows up nowhere in
  `git status`, which is why a clean status is a short status.
- **Modified.** Tracked, and different from the last commit, with the difference sitting
  only in your working folder.
- **Staged.** The current version has been copied into the staging area and will be part
  of the next commit.

A file can be modified and staged at once. Stage a file, edit it again, and `git status`
lists it under both headings, because the staged copy and the disk copy are now two
different things. Committing at that point saves the staged version and leaves the newer
edit behind, which is confusing exactly once.

### Which comparison each diff command shows

```powershell
git diff
```

Working folder against the staging area. What you would still have to add.

```powershell
git diff --staged
```

Staging area against the last commit. What you are about to commit, which is the version
worth reading before you type `git commit`. Reading the output itself, the plus and minus
lines and the chunk headers, is [d9](#d9-reading-a-diff).

### Why the staging area exists at all

It lets you commit a piece of your work rather than all of it. You fixed a bug and also
renamed a variable in a second file. Stage only the bug fix and commit it, then stage the
rename and commit that separately, and your history has two clear entries instead of one
muddled one. This matters more when an agent has touched eight files and you agree with
five of them.

The cost is one more concept between you and a saved change, which is why `git commit -a`
exists to skip it. That shortcut stages every tracked file that changed, including the
ones you had not looked at yet ([d4](#d4-commit-well)).

### The staging area has three names

Git calls it the staging area in messages, the **index** in its documentation and error
text, and the **cache** in older flags such as `git diff --cached`, which does exactly what
`--staged` does. All three words mean the same thing. Git kept every name it ever used and
expects you to keep up.

### HEAD, and the state where it goes loose

HEAD points at your current position, and normally it points at a branch, which in turn
points at a commit. Everything you do assumes that chain.

Move directly to a specific commit instead of a branch, usually by running
`git checkout <hash>` to look at an old version, and git prints several paragraphs about
being in **detached HEAD** state. It means HEAD is pointing straight at a commit with no
branch involved. Looking around is safe. Committing there creates commits no branch points
at, and they are easy to lose track of. Getting back:

```powershell
git switch -
```

Returns you to the branch you were on before. Verify with `git status`, which should say
`On branch main` rather than `HEAD detached at 9f2c1ab`. If you did commit while detached
and want that work, [d11](#d11-when-you-lose-work) is the card that recovers it.

### The fourth place, which is not on your machine

Committed history lives on your disk. The copy on GitHub is a separate place again, and
`git push` is the only thing that moves work into it. A commit you have not pushed exists
on exactly one computer, which is fine right up until the moment it is not.
[d2](#d2-repo-remote-clone-origin) covers remotes.

### The short form, once the long form is familiar

```powershell
git status -s
```

```text
M  notes.md
 M src/app.js
?? scratch.txt
```

Two columns. The left column is the staging area, the right column is the working folder,
and `??` means untracked. `M ` with the letter on the left is staged, ` M` with the letter
on the right is modified but not staged. It is the same information in one screen, and it
is unreadable until the long form has clicked.
