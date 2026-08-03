---
id: git-rm-cached
title: git rm --cached
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git rm --cached <file>
shell: any

does: >
  Tells git to stop tracking a file while leaving the file itself sitting on your disk,
  which is how you undo committing something that should have been ignored.

flags:
  - flag: "--cached"
    means: >
      Remove from git's index only, not from the filesystem. Leave this flag off and
      `git rm <file>` deletes the actual file from your disk as well. One word is the entire
      difference.
  - flag: "-r"
    means: >
      Recursive, required for a folder. `git rm -r --cached node_modules` is the usual form,
      and it stages a deletion for every file underneath.
  - flag: "--dry-run"
    means: Lists what would be untracked without changing anything. Cheap and worth it on a folder.

destructive: true

danger: >
  Your copy of the file is safe, and everyone else's is not. This stages a deletion, so once
  you commit and push, the next person to pull has that file deleted from their working
  folder. If the file held their local configuration, it is gone from their machine. Leaving
  off `--cached` deletes the file from your disk immediately.

destroys: >
  Nothing on your disk, as long as `--cached` is present. After you commit and push, it
  deletes that file from every other clone on the next pull, and that copy is recoverable
  only from an earlier commit. It does not remove the file from past history: a committed
  secret stays readable in every earlier commit forever, which is why the answer to a leaked
  credential is always to rotate it.

safer_first: >
  Run `git rm -r --cached --dry-run <file>` and read the list. Add the path to `.gitignore`
  in the same commit, or the very next `git add .` puts the file straight back.

undo: >
  Before committing, `git reset HEAD <file>` restores the tracking. After committing,
  `git revert <that-commit-hash>` puts the file back into the repository.

expect: >
  One `rm '<path>'` line per file. Then run `git status`: the file should appear under
  `Changes to be committed` as `deleted`, and also under `Untracked files`, which is correct
  and means git has let go of it while your disk copy survives.

see_also:
  - git-status
  - d12-gitignore-and-what-not-to-commit
  - g6-secrets-and-what-never-to-commit

keywords:
  - stop tracking a file
  - committed node_modules by mistake
  - remove from git but keep file
  - untrack a file
---
