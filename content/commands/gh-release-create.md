---
id: gh-release-create
title: gh release create
type: command
verified: 2026-08-02
volatility: quarterly

tool: gh
command: gh release create <tag-name> --title "<title>" --notes "<notes>"
shell: powershell

does: >
  Publishes a release page on GitHub for a given version tag, with notes and optionally the
  installer or build output people are meant to download.

flags:
  - flag: "<tag-name>"
    means: >
      The version tag, such as `v1.2.0`. If the tag does not exist yet, this command creates
      it at your current commit and pushes it.
  - flag: '--notes "<notes>"'
    means: The release description. Use `--notes-file <file>` for anything more than a line.
  - flag: "--generate-notes"
    means: >
      Writes the notes for you from the pull requests merged since the previous release.
      Usually a better starting point than anything you would type by hand.
  - flag: "--draft"
    means: Creates it unpublished so you can check the page before anyone sees it.
  - flag: "--prerelease"
    means: Marks it as not production-ready, which keeps it out of the "latest release" slot.
  - flag: "<file-path>, added at the end"
    means: >
      Any file paths after the flags are uploaded as downloadable assets, as in
      `gh release create v1.2.0 .\dist\setup.exe`.

expect: >
  The address of the release page printed on its own line, such as
  `https://github.com/nyxlocke/myproject/releases/tag/v1.2.0`.

see_also:
  - git-tag
  - gh-pr-merge
  - i4-releases-and-versioning
  - d13-tags-releases-and-history

keywords:
  - publish a release
  - upload an installer
  - github release
  - release notes
---

A tag marks a commit. A release is the page built around that tag, with notes and files
attached. You can have a tag without a release, and this command is how you turn one into the
other.
