---
id: git-clean-fd
title: git clean -fd
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git clean -fd
shell: any

does: >
  Deletes every untracked file and untracked folder in your repository, meaning everything
  git has never been told to look after.

flags:
  - flag: "-f"
    means: >
      Force. Git refuses to delete anything without it, on purpose, because this command has
      no undo. Typing `-f` is you confirming.
  - flag: "-d"
    means: >
      Also remove untracked directories. Without it, git deletes loose files and leaves the
      folders, which is rarely what anyone meant.
  - flag: "-n"
    means: >
      Dry run. Lists exactly what would be deleted and deletes nothing. Run this first,
      every time, with no exceptions.
  - flag: "-x"
    means: >
      Also delete files that `.gitignore` is protecting. That includes `node_modules`,
      `target`, build output, and, critically, your `.env` file with your credentials in it.
      Far more dangerous than the base command.

destructive: true

danger: >
  This permanently deletes files from disk. Untracked files were never in a commit, so git
  has no copy of them anywhere. On Windows they do not go to the Recycle Bin. Adding `-x`
  extends the deletion to your ignored files, which is where secrets and local configuration
  live.

destroys: >
  Every untracked file and folder under the repository: new files you wrote but never
  staged, scratch notes, downloaded fixtures. There is no undo and no reflog entry, because
  none of it was ever a commit. With `-x`, it also destroys `.env` files and any local
  configuration that `.gitignore` excludes.

safer_first: >
  Run `git clean -nd` and read the list. Every line is a file that is about to be deleted. If
  any of them matter, move them out of the repository or run `git add` and commit them first.

undo: >
  You cannot. Nothing here was ever tracked, so no git command can reach it. A Windows File
  History or OneDrive backup is your only remaining option.

expect: >
  One `Removing <path>` line per deleted item. The dry run with `-n` prints
  `Would remove <path>` instead, which is the version you should see first.

see_also:
  - git-status
  - git-reset-hard
  - d12-gitignore-and-what-not-to-commit
  - d11-when-you-lose-work

keywords:
  - delete untracked files
  - clean the repo
  - remove build junk
  - get rid of new files
---
