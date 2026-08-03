---
id: d7-merge-conflicts
title: Merge conflicts, and how to resolve one
type: section
track: D
order: 70
verified: 2026-08-02
volatility: low
answer: >
  A merge conflict means two branches changed the same lines and git will not
  choose for you. Open the file, keep the version you want, delete the other and
  the three marker lines, then git add it and commit. Aborting is always
  available.
danger: >
  git merge --abort and git rebase --abort are the safe exits: they put every
  file back exactly as it was before the merge started. git reset --hard also
  ends a conflict, and additionally throws away every uncommitted change in your
  working folder with no undo, including work that had nothing to do with the
  conflict. Abort first. Read d10 before reaching for reset.
owns:
  - merge conflict
  - conflict markers
  - abort
see_also:
  - d6-merge-and-rebase
  - d10-undo-everything
  - d3-the-three-places
  - g3-lockfiles
keywords:
  - merge conflict
  - conflict markers
  - fix conflicts and then commit the result
  - both modified
  - unmerged paths
  - git merge abort
  - arrows in my file
---

## More

The word "conflict" oversells it. Git already combined everything it could work out on its
own, and stopped at the lines where both branches changed the same thing, because guessing
which version you meant is the one thing it will not do.

On screen it looks like this:

```text
Auto-merging src/config.js
CONFLICT (content): Merge conflict in src/config.js
Automatic merge failed; fix conflicts and then commit the result.
```

Nothing is broken and nothing is lost. The merge is paused, and your job is to say what the
file should say.

Open the named file and find the markers git wrote into it:

```text
<<<<<<< HEAD
const timeout = 30;
=======
const timeout = 60;
>>>>>>> feature/login
```

Three marker lines, two versions. Between `<<<<<<<` and `=======` is the version on the
branch you are standing on, labeled `HEAD`. Between `=======` and `>>>>>>>` is the version
arriving from the branch named on the last line.

Edit the file so it says what you want. Usually that means keeping one side, deleting the
other, and deleting all three marker lines. Sometimes the right answer is a line that
neither side had. Git does not care which you pick; it only cares that the markers are gone.

Then tell git the file is settled and finish the merge:

```powershell
git add src/config.js
```

```powershell
git commit
```

`git commit` with no `-m` here is deliberate: git has already written the merge message and
just wants you to confirm it. If an editor opens showing that message, save and close it, or
press `Esc` then type `:wq` and Enter if it is Vim.

Verify with `git status`, which should now say `nothing to commit, working tree clean`. If
it still lists "Unmerged paths", a file is left unresolved and you can see which one.

Not ready to deal with it at all:

```powershell
git merge --abort
```

Puts every file back exactly as it was before you started. Use `git rebase --abort` if you
hit the conflict during a rebase instead ([d6](#d6-merge-and-rebase)).

## Full

### Why some overlapping edits merge fine and others do not

Git compares three versions of the file: your side, their side, and the common ancestor,
which is the last commit both branches shared. Any line only one side changed is taken
automatically, because the ancestor proves the other side left it alone. A conflict is
raised only where both sides moved away from the ancestor in the same region.

This is why edits to two different functions in one file usually merge silently, and why two
people fixing the same typo in different ways always conflict.

### Making the markers more useful

The default markers show two versions. You can have git show the ancestor as well, which
turns "which of these do I want" into "what was each side trying to change":

```powershell
git config --global merge.conflictStyle zdiff3
```

Verify with `git config --get merge.conflictStyle`, which prints `zdiff3`. Conflicts now
look like this:

```text
<<<<<<< HEAD
const timeout = 30;
||||||| 7c8b9a0
const timeout = 45;
=======
const timeout = 60;
>>>>>>> feature/login
```

The middle section is what the line said before either branch touched it. Now you can see
that one side lowered the timeout and the other raised it, which is a real decision, rather
than two numbers with no context.

### Reading git status mid-conflict

```powershell
git status
```

```text
On branch main
You have unmerged paths.
  (fix conflicts and run "git commit")
  (use "git merge --abort" to abort the merge)

Unmerged paths:
  (use "git add <file>..." to mark resolution)
        both modified:   src/config.js
```

"Unmerged paths" is the list of files still waiting on you. The label on each one describes
the kind of conflict: `both modified` is the ordinary case, `deleted by them` means the
other branch removed a file you edited, and `both added` means each branch independently
created a file with the same name. The last two are resolved the same way, by deciding
whether the file should exist and then running `git add` on it, or `git rm` if the answer is
no.

### Taking one whole side

When you know the entire file should come from one branch:

```powershell
git checkout --ours package-lock.json
```

`--ours` is the branch you are standing on, `--theirs` is the one you are merging in. The
newer spelling is `git restore --ours <file>`, and both still need a `git add` afterward to
mark the file resolved.

The confusing part, and it catches experienced people: during a **rebase**, those two words
swap. A rebase replays your commits onto the other branch, so git considers the other branch
"ours" and your own commit "theirs". If you are mid-rebase, read the labels in the file
rather than trusting the word.

### The file types where the answer is never "pick a side"

Lockfiles conflict constantly, because any two branches that added a dependency both rewrote
the same generated file. Never hand-edit one. Take either side, then regenerate it by
running your package manager's install command, and commit the result
([g3](#g3-lockfiles)).

The same logic covers any generated file: build output, compiled assets, anything a tool
produces. Resolve by rerunning the tool, not by editing its output.

### What to hand the agent, and what to decide yourself

Hand it over when the conflict is mechanical: two import lists that both grew, formatting
differences, a generated file that needs regenerating, the same change applied twice in
slightly different words. Paste the conflicted section whole, including all the markers, and
say which branch is which.

Decide yourself when the two sides represent two different intentions. If one branch made
sessions expire faster and the other made them expire slower, no amount of code reading
tells you which was wanted. That is a decision, and an agent asked to resolve it will make
one confidently and without mentioning that it did.

Two rules whatever you do. Never let an agent resolve a conflict in a file you have not
opened yourself. And after it reports success, run `git diff --staged` and read what it
kept ([d9](#d9-reading-a-diff)).

### The failure that reaches production

The classic conflict disaster is committing the markers. The file still contains `<<<<<<<`
and both versions, git records it happily because to git it is only text, and the program
fails to start with a syntax error at that line. Before you commit a resolution:

```powershell
git grep -n "<<<<<<<"
```

Searches every tracked file and prints the file and line number of any leftover marker. No
output means you are clean. Run it once and the habit costs nothing.

### Making conflicts rarer

Short branches, merged back within a day or two. Pull `main` into your branch before you
start a new piece of work rather than after a week of drift. Small commits, so that when a
conflict does come up it covers ten lines instead of four hundred. Conflicts scale with how
long two lines of work stayed apart, and nothing else you do affects them nearly as much.
