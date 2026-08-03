---
id: d5-branches
title: Branches
type: section
track: D
order: 50
verified: 2026-08-02
volatility: low
answer: >
  A branch is a movable label pointing at one commit, so making one costs nothing
  and throwing one away costs nothing. git switch -c feature/login creates a
  branch and moves you onto it; git switch main moves you back.
danger: >
  git branch -D deletes a branch even when its commits were never merged
  anywhere, which leaves that work with no label pointing at it. Use git branch
  -d instead, which refuses to delete unmerged work and tells you so. If you
  already ran the capital version, the commits usually still exist and d11
  covers getting them back.
owns:
  - branch
  - main
  - switch
  - checkout
  - branch naming
see_also:
  - d6-merge-and-rebase
  - d8-pull-requests
  - d1-what-git-actually-stores
  - d10-undo-everything
keywords:
  - git branch
  - git switch
  - git checkout
  - main branch
  - feature branch
  - branch naming
  - delete a branch
---

## More

Most people picture a branch as a copy of the project, and that picture causes every
misunderstanding that follows. A **branch** is a label that points at one commit and moves
forward as you commit. Nothing is copied and no folder is created. Making one writes a
single line to a file inside `.git`, which is why branching is instant on a project of any
size ([d1](#d1-what-git-actually-stores)).

**main** is the label on your stable line of work. The name is a convention, set once when
git creates a repository ([b3](#b3-tell-git-who-you-are)), and older projects use `master`
for the same thing.

Make one and move onto it:

```powershell
git switch -c feature/login
```

`-c` means create. Verify with `git status`, whose first line now reads
`On branch feature/login`. Commits you make from here move the `feature/login` label
forward and leave `main` exactly where it was.

Go back:

```powershell
git switch main
```

Verify the same way, or run `git branch` to list every branch with a `*` marking the one you
are on. Press `q` if that listing opens a scrolling view.

Name branches with a category, a slash, and a short description in lowercase with hyphens:
`feature/login`, `fix/session-timeout`, `chore/upgrade-deps`. The slash is part of the name
and creates no folder. No spaces, ever.

One behavior that catches everyone: uncommitted changes travel with you when you switch
branches. If a switch would overwrite an edited file, git refuses instead:

```text
error: Your local changes to the following files would be overwritten by checkout:
        src/app.js
Please commit your changes or stash them before you switch branches.
Aborting
```

Nothing has broken and nothing was lost. Git is telling you to deal with the edit first,
by committing it ([d4](#d4-commit-well)) or shelving it
([d10](#d10-undo-everything)) before you move.

## Full

### What switching actually does to your disk

Git rewrites the files in your working folder to match the snapshot the target branch points
at. Files that exist only on the branch you left disappear from the folder. Files that exist
only on the branch you arrived at appear.

This looks exactly like your work being deleted, and it is not. The files are stored in the
commits on the branch you left, and switching back brings them all straight back. Verify it
the first time on purpose: switch away, look at the folder, switch back, and watch the files
return.

Anything git never tracked is untouched by all of this, which is why `node_modules` and your
`.env` file stay put while everything else changes around them
([d12](#d12-gitignore-and-what-not-to-commit)).

### switch, checkout, and why there are two commands

`git checkout` is the original command. It switches branches, and it also restores files,
and it also moves you to an arbitrary commit, and the difference between those is which
arguments you gave it. Getting one wrong is how people used to discard a day of work while
believing they were changing branches.

Git split it in two: `git switch` changes branches, `git restore` changes files. Use
`switch`. `checkout` still works everywhere and every older tutorial uses it, so you will
read it constantly, and when you do, it means one of the two newer commands.

### The branch on GitHub is a different label

After you push, you will see names like `origin/main`. That is a **remote-tracking branch**:
your machine's record of where `main` was on GitHub the last time it looked. It is not live.
It updates when you run `git fetch` or `git pull`, and it can be days out of date without
anything appearing wrong ([d2](#d2-repo-remote-clone-origin)).

So `git branch` lists your local labels only. To see the ones on GitHub too:

```powershell
git branch -a
```

`-a` means all. Remote-tracking entries appear with the `remotes/origin/` prefix.

A new local branch has no counterpart on GitHub until you push it:

```powershell
git push -u origin feature/login
```

`-u` links the two so later pushes on this branch are a bare `git push`. Verify with
`git status`, which now reports the branch as up to date with `origin/feature/login`.

### Three more commands worth knowing

```powershell
git switch -
```

Back to the branch you were on before, the way `cd -` works in a shell. Useful when you are
bouncing between two.

```powershell
git branch -m fix/tpyo fix/typo
```

`-m` renames a branch. If it is the branch you are on, you can leave out the old name. A
branch you already pushed keeps the old name on GitHub until you push the new one and delete
the old one there.

```powershell
git branch -d feature/login
```

`-d` deletes a branch, and refuses if the branch holds commits that are not merged anywhere
else. That refusal is the feature. Deleting a merged branch deletes only the label, since
the commits themselves now live on `main` as well. The capital `-D` variant deletes
regardless, which is what the `danger` note at the top of this card is about.

### When to branch, and for how long

Branch per piece of work, and merge it back within a day or two. A branch is cheap because
it is a label; the cost arrives when it sits open for three weeks while `main` moves on, and
the two lines of work drift far enough apart that combining them raises conflicts in files
you had forgotten about ([d7](#d7-merge-conflicts)).

Working directly on `main` is legitimate for a solo project small enough to hold in your
head. You give up two things by doing it: pull requests, which need a branch to propose
([d8](#d8-pull-requests)), and the ability to abandon an experiment by deleting a label
rather than by unpicking commits.

The branch habit matters more with an agent than without one. When you tell an agent to
attempt something you are not sure about, do it on a branch. If the attempt is wrong, you
switch back to `main` and the wrong version is somewhere else entirely rather than tangled
into your working history.

### Finding out where you are, from cold

Three commands answer it, in increasing detail:

```powershell
git status
```

The first line names your branch. Run this before anything else, always.

```powershell
git branch
```

Every local branch, with `*` on the current one.

```powershell
git log --oneline --graph --all -15
```

The last fifteen commits across every branch, with the branch labels drawn in and the
connections between commits on the left. This is the picture the other two are summarizing.
