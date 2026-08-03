---
id: d9-reading-a-diff
title: Reading a diff
type: section
track: D
order: 90
verified: 2026-08-02
volatility: low
verify: git diff --stat
answer: >
  In a diff, lines marked `-` are what the file used to say and lines marked `+`
  are what it says now. `git diff` shows edits you have not staged;
  `git diff --staged` shows what is about to go into your next commit.
owns:
  - diff
  - hunk
  - plus and minus lines
  - staged vs unstaged diff
see_also:
  - h3-reviewing-a-diff-you-cannot-fully-read
  - c8-line-endings-and-encoding
  - d3-the-three-places
  - d4-commit-well
  - d8-pull-requests
keywords:
  - what do the plus and minus mean
  - git diff
  - at at symbols
  - hunk header
  - red and green lines
  - diff staged
  - how do i see changes
---

## More

A **diff** is a line-by-line report of the difference between two versions of a file. Git
prints one whenever you ask what changed, and GitHub renders the same information with
colors on a pull request page. The format is forty years old and identical everywhere, so
learning it once covers every tool you will meet.

Four symbols carry all of it.

- A line beginning with `-` was there before and is not there now. Red on screen.
- A line beginning with `+` is there now and was not there before. Green on screen.
- A line beginning with a space is unchanged, shown only so you can see the surroundings.
- A line beginning with `@@` is a location header, not code.

A changed line shows up as a `-` immediately followed by a `+`. Git has no concept of
"edited a line." It only knows removed and added, so every edit reads as a deletion
followed by an insertion of the new version.

Which diff you get depends on which command you run, and the difference matters. Your work
lives in three places, explained in [d3](#d3-the-three-places).

```powershell
git diff
```

Edits in your files that you have not staged yet. If you have staged everything, this
prints nothing, which surprises people constantly.

```powershell
git diff --staged
```

What is staged and about to be committed. This is the one to run before you type
`git commit`, because it is exactly the change you are about to record.

```powershell
git diff HEAD
```

Everything, staged and unstaged together, compared against your last commit. Use this when
you want the honest total of what you have done since the last checkpoint.

Diffs go through a pager on Windows. Press the spacebar to move down a screen, `q` to quit
back to the prompt. If you are stuck in a screen you cannot type into, `q` is the way out.

This card covers what the symbols mean. What to actually look for in a diff an agent
produced is a different skill and it lives in
[h3](#h3-reviewing-a-diff-you-cannot-fully-read).

## Full

### A complete diff, labeled

```diff
diff --git a/src/login.js b/src/login.js
index 3f8a1c2..9b4e7d0 100644
--- a/src/login.js
+++ b/src/login.js
@@ -12,7 +12,8 @@ function handleSubmit(event) {
   event.preventDefault();
   const email = form.email.value;
-  const password = form.password.value;
+  const password = form.password.value.trim();
+  if (!password) return showError("Password is required");
 
   submit({ email, password });
 }
```

Line by line:

- `diff --git a/... b/...` names the file. `a/` is the old version, `b/` is the new one.
  Both paths are the same file unless it was renamed.
- `index 3f8a1c2..9b4e7d0` is git's internal bookkeeping. Ignore it.
- `--- a/src/login.js` and `+++ b/src/login.js` mark which side is which. Three dashes is
  the before, three pluses is the after. This is the only place a `-` line is a filename
  rather than deleted code.
- `@@ -12,7 +12,8 @@` is the hunk header, explained next.
- Then the content, where the first character of each line is the marker and the rest is
  the file.

Read this one as: the password line was replaced with a version that trims whitespace, and
one new line was added below it. Two lines added, one removed.

### The hunk header

A **hunk** is one contiguous region of change plus a few unchanged lines around it for
context. A file with three edits in three distant places produces three hunks in one diff.

```text
@@ -12,7 +12,8 @@ function handleSubmit(event) {
```

The two numbered groups are the before and after positions.

- `-12,7` means: in the old file, this hunk starts at line 12 and covers 7 lines.
- `+12,8` means: in the new file, it starts at line 12 and covers 8 lines.

Eight against seven is the arithmetic of one net added line, which matches the two `+` and
one `-` in the body.

The text after the second `@@` is a hint, not part of the format. Git guesses the enclosing
function or section and prints its name so you know roughly where you are. It guesses from
indentation and is sometimes wrong, so treat it as a label rather than a fact.

You never need to do anything with these numbers. Knowing they are line positions is enough
to stop them looking like an error message.

### Staged, unstaged, and the diff that shows nothing

The single most common confusion with `git diff` is running it, getting no output, and
concluding nothing changed.

Git compares two specific things depending on the flag:

| Command | Compares |
|---|---|
| `git diff` | your files against what is staged |
| `git diff --staged` | what is staged against your last commit |
| `git diff HEAD` | your files against your last commit, staged or not |
| `git diff main` | your current position against the tip of `main` |

So after `git add .`, plain `git diff` is empty by definition: nothing differs between your
files and the staging area, because you just made them match. The change did not vanish. It
moved one place to the right. `git diff --staged` shows it.

`--staged` and `--cached` are the same flag. `--cached` is the older spelling and still
works, so you will see both in older documentation.

### Seeing the shape before the detail

```powershell
git diff --stat
```

Prints one line per file with a count of lines added and removed, and a total at the bottom.
No code at all. This is the first command to run on any change you did not personally type,
because it answers the two questions that matter most in ten seconds: how many files, and
is the size roughly what you expected.

```powershell
git diff --name-only
```

Just the file paths, nothing else. Useful when the list is long enough that even `--stat`
scrolls.

### Diffs for things that are not edits

**A new file** shows every line as `+`, with `--- /dev/null` as the before side. `/dev/null`
is the conventional name for nothing.

**A deleted file** shows every line as `-`, with `+++ /dev/null` as the after side.

**A renamed file** shows as `similarity index 96%` followed by `rename from` and `rename to`
lines, with only the actual edits in the body. Git does not store renames; it detects them
by noticing that a deleted file and an added file are nearly identical.

**A binary file** shows as `Binary files a/logo.png and b/logo.png differ` and nothing else.
Git cannot produce line-by-line output for an image, so it tells you it changed and stops.

**A whitespace-only change** looks alarming: every line in the file removed and every line
added back, apparently identically. On Windows this is almost always line endings rather
than a real edit. [c8](#c8-line-endings-and-encoding) explains why, and this makes it
readable in the meantime:

```powershell
git diff --ignore-all-space
```

### Diffing one file, or one moment in time

```powershell
git diff -- src/login.js
```

Restricts the diff to one path. The bare `--` separates paths from other arguments, which
matters when a filename could be mistaken for a branch name. Get in the habit of including
it.

```powershell
git diff HEAD~1 HEAD
```

The change made by your most recent commit alone. `HEAD` is your current position and
`HEAD~1` is one commit before it, so this is the answer to "what did that last commit
actually do."

```powershell
git show
```

The same thing in nicer packaging: the last commit's message, author, and diff in one
output. [d13](#d13-tags-releases-and-history) covers reading further back.

### When the diff is unreadable

Two switches earn their keep on generated code.

```powershell
git diff --word-diff
```

Marks changes within a line instead of replacing the whole line. On a long line where one
argument changed, this is the difference between seeing the edit and hunting for it.

```powershell
git diff --color-words
```

The same idea, using color rather than brackets. Easier on the eyes, harder to copy out of.

If a diff is still unreadable after both, that is information: a change that cannot be read
line by line usually needs to be smaller. Ask the agent to split it, or read the file whole
instead of the diff.
