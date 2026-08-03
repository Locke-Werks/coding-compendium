---
id: winget-upgrade
title: winget upgrade
type: command
verified: 2026-08-02
volatility: quarterly

tool: winget
command: winget upgrade --all
shell: powershell

verify: winget --version

does: >
  Updates programs installed through Windows package management to their latest published
  versions.

flags:
  - flag: "--all"
    means: >
      Upgrades everything that has a newer version available. Without it, `winget upgrade`
      only lists what is out of date and changes nothing, which is the version to run first.
  - flag: "--id <package-id>"
    means: Upgrades one named package, as in `winget upgrade --id Git.Git`.
  - flag: "--include-unknown"
    means: >
      Also upgrades packages whose currently installed version winget cannot read. Those are
      skipped by default and can sit outdated for months without you noticing.
  - flag: "--silent"
    means: Hides the installer windows. Useful when upgrading a dozen packages at once.

expect: >
  A table of package name, current version, available version, and source, then installation
  output for each one. `No installed package found matching input criteria.` means everything
  is already current.

see_also:
  - winget-install
  - get-command
  - c4-path-and-command-not-found

keywords:
  - update my programs
  - upgrade everything
  - out of date packages
  - windows updates for apps
---

Run bare `winget upgrade` first and read the list. Upgrading everything at once includes
tools a project may be pinned to, and a version jump you did not plan is a difficult thing to
diagnose an hour later.

Programs installed by their own updater, such as Claude Code, are not managed here and will
not appear.
