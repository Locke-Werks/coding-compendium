---
id: d1-what-git-actually-stores
title: What git actually stores
type: section
track: D
order: 10
verified: 2026-08-02
volatility: low
answer: >
  Every commit is a complete snapshot of your project, not a list of edits, and
  each one points at the commit before it, which makes the history a graph. Git
  runs on your machine; GitHub is a website that stores a copy.
owns:
  - commits as snapshots
  - the history graph
  - git vs GitHub
see_also:
  - d3-the-three-places
  - d5-branches
  - d2-repo-remote-clone-origin
  - d13-tags-releases-and-history
keywords:
  - what is a commit
  - snapshot or diff
  - git internals
  - is github the same as git
  - commit hash
  - version control
  - dot git folder
---

## More

Type `git commit` and git copies every file it is tracking into its own storage, exactly as
those files are at that second, then labels the bundle with your message, your name, the
time, and a pointer to the commit before it. That bundle is the commit: a photograph of the
whole project, not a note about what you changed.

That surprises people, because every tool you will meet shows a commit as a **diff**, the
line-by-line view of what changed. Git computes that view on demand by comparing two
snapshots. The diff is a display format. The snapshot is what got stored.

Two things follow, and you will use both constantly:

- Moving to any commit hands you the whole project as it stood then. Git never replays a
  chain of edits to rebuild it, so there is no half-built state to land in.
- No commit depends on its neighbors staying put. That is why git can reorder, drop, and
  replay commits without breaking the project.

The obvious worry is disk space. Git stores identical file content once and points at it
from every snapshot that contains it, so a commit that changes one line of one file costs
one file, not a second copy of everything.

Each commit is named by a 40-character **hash** computed from its contents. You will
usually see the first seven characters, like `9f2c1ab`. Change anything about a commit,
including its message, and the hash changes, which means you now have a different commit.

Because each commit names its parent, the history is a graph. Follow the pointers backward
from where you are and you get the path that led here. A branch is a label pointing at one
commit, covered in [d5](#d5-branches). A merge creates a commit with two parents, covered
in [d6](#d6-merge-and-rebase). A straight line of commits is a special case, not the rule.

One more, and it is the most common confusion here. **Git and GitHub are
different things.** Git is a program on your machine, and it keeps everything above in a
hidden folder called `.git` inside your project. GitHub is a website that holds a copy of
that folder so other machines can reach it. Git works with no internet connection and no
GitHub account. The commands that reach GitHub are `clone`, `fetch`, `pull`, and `push`.
Everything else is local, and all of it lives in [d2](#d2-repo-remote-clone-origin).

## Full

### What is actually inside one commit

Four things. Run this inside a project that has at least one commit:

```powershell
git cat-file -p HEAD
```

`cat-file` prints a stored object, `-p` means print it in readable form, and `HEAD` is
git's word for the commit you are sitting on right now. You get something shaped like
this:

```text
tree 91f2b4c8d3e5a6079182b3c4d5e6f708192a3b4c
parent 6f1a0c9c1b2d3e4f5061728394a5b6c7d8e9f0a1
author Nyx <nyx@example.com> 1785705600 -0500
committer Nyx <nyx@example.com> 1785705600 -0500

feat: add email and password login
```

The `tree` line points at the full listing of every tracked file and folder, with the
stored content of each. That is the snapshot. The `parent` line is the commit before this
one, and it is the entire mechanism behind the graph. Then who, when, and why.

If it prints `fatal: not a git repository`, you are in a folder git does not track. If it
prints `fatal: bad revision 'HEAD'`, the repository exists but has no commits in it yet.

### What "snapshot, not diff" changes about your expectations

- Undoing a commit does not rewind time. It records a new snapshot that happens to look
  like an older one, so your history gets longer rather than shorter. Every form of undo
  is in [d10](#d10-undo-everything).
- Some commands do work in diffs. Rebase and cherry-pick take the difference between a
  commit and its parent and reapply it elsewhere. That is a calculation performed on top
  of snapshots, and it explains why a rebase can raise a conflict once per commit instead
  of once in total ([d6](#d6-merge-and-rebase)).
- Renames are not stored. Git stores content. When a file's content turns up under a new
  path, git guesses it was a rename by comparing how similar the two are. Rename a file
  and heavily edit it in the same commit and git will show a delete plus an add. Nothing
  is broken when that happens.

### The .git folder is the repository

Every commit, every version of every file, every branch label, your remotes, and your
local config live in the `.git` folder at the root of your project. Windows hides it by
default. Two consequences worth knowing:

- Remove that folder and the working files stay exactly where they are while the entire
  history disappears. "This is not a git repository" means the folder is missing.
- Copy the project folder to a flash drive and you have copied the whole history with it,
  because the history was never anywhere else. No server is involved.

### Git and GitHub, in the detail that matters

Git is a version-control program, written in 2005, that runs offline on your machine.
GitHub is a website, owned by Microsoft, that stores copies of git repositories and adds
its own features on top: pull requests, issues, and Actions. None of those are git
features. GitLab, Bitbucket, and Codeberg do the same job for the same repositories, and
your history does not care which one you use, or whether you use one at all.

The practical test: anything you can do on a plane is git. Deleting a repository on GitHub
does not touch your machine, and deleting your folder does not touch GitHub, because each
one is a complete copy.

### Reading the graph yourself

```powershell
git log --oneline --graph --all
```

`--oneline` prints one line per commit, `--graph` draws the parent connections down the
left edge, and `--all` includes branches you are not currently on.

```text
*   8a1c9de (HEAD -> main) Merge branch 'feature/login'
|\
| * 4b2f7c1 (feature/login) feat: add password reset
| * 9f2c1ab feat: add email and password login
* | 1d3e5f7 docs: update README
|/
* 7c8b9a0 chore: initial commit
```

Read it bottom to top: oldest at the bottom, newest at the top. The forked lines are two
branches that existed at the same time, and the join at the top is the merge that brought
them back together. If the command prints nothing and returns you to the prompt, the
repository has no commits yet. If it opens a scrolling view you cannot escape, press `q`.

### What git does not store

- Anything you have not committed. Uncommitted work exists only on your disk, which is the
  whole argument of [d3](#d3-the-three-places).
- Anything listed in `.gitignore`, covered in [d12](#d12-gitignore-and-what-not-to-commit).
- Empty folders. Git tracks files, so an empty folder is invisible to it. This is why
  projects contain otherwise pointless files named `.gitkeep`.
- File permissions and timestamps, beyond a single flag for whether a file is executable.
  A freshly cloned file gets today's date, which makes "date modified" useless for working
  out when something changed. `git log` is the record instead, and
  [d13](#d13-tags-releases-and-history) covers reading it.
