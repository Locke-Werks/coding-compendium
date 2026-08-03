---
id: d13-tags-releases-and-history
title: Tags, releases, and reading history
type: section
track: D
order: 130
verified: 2026-08-02
volatility: quarterly
verify: git log --oneline -5
danger: >
  Deleting a tag that has already been pushed removes a version marker other
  people and other machines may already have downloaded. Nothing in your code is
  lost, but anything referencing that tag breaks. Prefer publishing a corrected
  new tag over deleting a published one.
answer: >
  A tag is a permanent name for one commit, usually a version like `v1.2.0`, and
  a release is GitHub packaging that tag with notes and files. `git log` and
  `git blame` answer when something broke and which commit changed it.
owns:
  - tag
  - release
  - git log
  - blame
see_also:
  - i4-releases-and-versioning
  - d9-reading-a-diff
  - j4-reading-a-repo-you-did-not-write
  - d1-what-git-actually-stores
  - i3-builds-and-artifacts
keywords:
  - tag
  - release
  - version number
  - git log
  - git blame
  - who changed this
  - when did this break
  - history
  - what changed and when
---

## More

A **tag** is a permanent label stuck to one specific commit. Branches move as you work;
tags do not. That is the entire difference and it is why tags mark versions.

```powershell
git tag -a v1.0.0 -m "First working version"
```

`-a` makes an annotated tag, which stores who made it, when, and the message. The other
kind, made by `git tag v1.0.0` with no flags, is a bare name with no author or date. Use
`-a` for anything you publish.

Tags do not travel with `git push`. Send them on purpose:

```powershell
git push origin v1.0.0
```

Verify with `git tag`, which lists your tags locally, and by looking at the Tags section on
the repository page.

A **release** is GitHub's layer on top of a tag: a title, notes, and optional downloadable
files, on their own page.

```powershell
gh release create v1.0.0 --generate-notes
```

`--generate-notes` writes the release notes from the commit messages between this tag and
the previous one, which is a strong argument for writing readable commit messages.
[i4](#i4-releases-and-versioning) covers what the numbers in `v1.2.0` promise.

The other half of this card is reading history, which is how you answer "when did this
break." Three commands cover most of it.

```powershell
git log --oneline -20
```

The last twenty commits, one line each: short hash, then message.

```powershell
git log --oneline -- src/login.js
```

Only commits that touched that one file. The bare `--` separates the path from everything
else, so git does not mistake a filename for a branch name.

```powershell
git blame src/login.js
```

Every line of the file with the commit, author, and date that last changed it.

All three go through a pager. Press the spacebar to page down and `q` to get back to your
prompt.

## Full

### Making, listing, and moving tags

```powershell
git tag
```

Lists every tag in the repository, sorted alphabetically, which means `v10.0.0` sorts before
`v9.0.0`. For version order use `git tag --sort=-v:refname`, which puts the newest first.

```powershell
git tag -a v1.2.0 -m "Add password reset"
```

Tags the commit you are standing on right now. To tag an older commit, name it:

```powershell
git tag -a v1.1.0 4f2c1ab -m "Retroactive tag for the release we shipped"
```

```powershell
git show v1.2.0
```

Shows the tag's message and the commit it points at, including the full diff.

To push every tag you have made at once:

```powershell
git push origin --tags
```

Deleting a tag takes two commands, one for each copy, and this is where care is needed.
Locally:

```powershell
git tag -d v1.2.0
```

And on GitHub:

```powershell
git push origin --delete v1.2.0
```

Nothing in your code is lost either way, because a tag is only a name. What breaks is
anything that referenced it: an install command pinned to that version, a build pipeline, a
release page. If a published version was wrong, publishing a corrected `v1.2.1` is almost
always better than deleting `v1.2.0`.

### Where am I relative to the last tag

```powershell
git describe --tags
```

Prints something like `v1.2.0-14-g4f2c1ab`, meaning: the most recent tag was `v1.2.0`, you
are 14 commits past it, and you are on commit `4f2c1ab`. Build scripts use this to stamp a
version into a program automatically. It is also the fastest way to answer "how much has
happened since the last release."

### Turning a tag into a release

```powershell
gh release create v1.0.0 --generate-notes
```

Creates the release page from an existing tag. If the tag does not exist yet, this command
creates it for you at your current commit.

To attach files people can download, list them after the tag:

```powershell
gh release create v1.0.0 --generate-notes .\dist\setup.exe
```

Verify with `gh release list` or by opening the Releases section of the repository. If the
attached file is a Windows installer, [i5](#i5-shipping-a-desktop-app) covers the warning
users will see when they run it.

### Reading history: four questions and their commands

**What has been happening lately.**

```powershell
git log --oneline --graph --all -20
```

`--graph` draws the branch structure on the left as lines and asterisks, `--all` includes
every branch rather than only the one you are on. This is the best twenty seconds you can
spend in a repository you have not opened in a month.

**What happened to this one file.**

```powershell
git log --oneline --follow -- src/login.js
```

`--follow` keeps tracking the file through renames, which plain `git log` does not do
because git stores snapshots rather than file identities. [d1](#d1-what-git-actually-stores)
explains why that is not an oversight.

**When did this exact text appear or disappear.**

```powershell
git log -S "TIMEOUT_MS" --oneline
```

This is the most under-used command in git. It searches history for commits that changed the
number of times that string appears, so it finds the commit that introduced a setting, and
the commit that deleted it. When you are staring at code wondering where a value came from,
this answers it in one shot. Add `-p` to see the diff of each match.

**What did that specific commit actually do.**

```powershell
git show 4f2c1ab
```

Message, author, date, and the full diff. [d9](#d9-reading-a-diff) reads the diff format.

Two more filters worth knowing:

```powershell
git log --since="3 days ago" --oneline
```

```powershell
git log --author="nyxlocke" --oneline
```

### Who changed this line, and when

```powershell
git blame src/login.js
```

Each line of the file is prefixed with the short hash, the author, and the date of the
commit that last touched it. On a long file, restrict it to the region you care about:

```powershell
git blame -L 40,60 src/login.js
```

`-L 40,60` means lines 40 through 60.

Blame has one predictable failure: a formatting pass or a rename shows one commit as the
author of every line, and the real history is underneath it. Two flags dig past that.

```powershell
git blame -w src/login.js
```

`-w` ignores whitespace-only changes, so a reindentation stops claiming credit.

```powershell
git blame --ignore-rev 4f2c1ab src/login.js
```

Skips one specific commit, which is what you want when a single formatting commit rewrote
the whole file.

Once blame gives you a hash, `git show <hash>` tells you what that commit was for. That pair
is the entire investigation: blame finds the commit, show explains it.

Blame reads as accusatory and is not. On a project where an agent wrote most of the lines,
the author column is you either way. Read it as "which change introduced this."

### Finding the commit that broke something

When you know it worked at some point and does not now, and there are eighty commits in
between, git can find the culprit by bisection: it checks out a commit halfway along, you
say whether it works, and it halves the remaining range each time. Eighty commits take about
seven answers.

```powershell
git bisect start
git bisect bad
git bisect good v1.1.0
```

That says: right now is broken, `v1.1.0` was fine, start searching. Git moves you to a
commit in the middle. Test it, then say `git bisect good` or `git bisect bad`. Repeat until
git prints the first bad commit.

When you are done, always run this:

```powershell
git bisect reset
```

It puts you back on the branch you started from. Without it you are left parked on an old
commit with no branch involved, which is the detached HEAD state in
[d3](#d3-the-three-places) and looks alarming.

### Living with the pager

Every command on this page can produce more output than fits on screen, and git pipes those
through a pager that takes over the window. It is not frozen.

- Spacebar pages down, `b` pages back.
- `q` quits and returns you to the prompt.
- `/word` searches, `n` jumps to the next match.

If you would rather it never happened, `git --no-pager log --oneline -20` prints straight to
the terminal for one command.
