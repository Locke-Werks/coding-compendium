---
id: git-restore-staged
title: git restore --staged
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git restore --staged <file>
shell: any

does: >
  Takes a file back out of the staging area so it will not be part of the next commit,
  while leaving your edits to it completely intact.

flags:
  - flag: "--staged"
    means: >
      Operate on the staging area only. Without this flag the command overwrites your file
      instead, which destroys your edits. The flag is the entire difference between a safe
      command and a sharp one.
  - flag: "<file>"
    means: >
      The file to unstage. A dot unstages everything currently staged in the current folder
      and below.

expect: >
  Nothing printed. Run `git status`: the file should move from `Changes to be committed`
  down to `Changes not staged for commit`, still carrying your edits.

undo: >
  Run `git add <file>` again. Nothing was lost, so this is a free move in both directions.

see_also:
  - git-add
  - git-restore
  - git-status
  - d3-the-three-places

keywords:
  - unstage a file
  - undo git add
  - remove from staging
  - staged by mistake
---

This is the fix for `git add .` sweeping in a file you did not want. It is safe: the
staging area is a list, and taking a file off the list changes nothing on disk.

Older versions of git print `git reset HEAD <file>` as the hint instead. That does the same
job through a more dangerous command, so prefer this one.
