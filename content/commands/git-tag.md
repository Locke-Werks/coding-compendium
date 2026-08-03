---
id: git-tag
title: git tag
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git tag -a <tag-name> -m "<message>"
shell: any

does: >
  Puts a permanent readable label on a commit, which is how a version number gets attached
  to a specific point in history.

flags:
  - flag: "-a"
    means: >
      Annotated. Stores the tag as a real object with your name, the date, and a message.
      Without `-a` you get a lightweight tag, which is a bare pointer with no record of who
      made it or when. Use `-a` for anything you release.
  - flag: '-m "<message>"'
    means: The tag's message, such as `first public release`. Required in practice for an annotated tag.
  - flag: "<tag-name>"
    means: >
      The label itself, conventionally a version like `v1.2.0`. Tag names share a namespace
      with branches, so avoid naming a tag after a branch.
  - flag: "-l"
    means: Lists existing tags. `git tag -l "v1.*"` filters them by pattern.
  - flag: "-d <tag-name>"
    means: Deletes a local tag. Removing one already pushed also needs `git push origin --delete <tag-name>`.

expect: >
  Nothing printed. Confirm with `git tag -l`, which should now list your tag, or
  `git show <tag-name>` to see the message and the commit it points at.

see_also:
  - git-push
  - git-log
  - gh-release-create
  - i4-releases-and-versioning
  - d13-tags-releases-and-history

keywords:
  - version number
  - tag a release
  - mark a version
  - tag not on github
---

Tags are not uploaded by an ordinary `git push`. This is why a tag you made never shows up
on GitHub. Push it explicitly with `git push origin <tag-name>`, or push all of them with
`git push --tags`.
