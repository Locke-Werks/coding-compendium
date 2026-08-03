---
id: git-filename-too-long
title: "error: Filename too long"
type: error
verified: 2026-08-02
volatility: low

category: config

# Prints true once long path support is on for git. It has to be set before the
# clone or checkout that failed.
verify: git config --get core.longpaths

sample: |
  PS C:\Users\nyx\dev> git clone https://github.com/nyxlocke/scraper.git
  Cloning into 'scraper'...
  remote: Enumerating objects: 1284, done.
  Receiving objects: 100% (1284/1284), 2.11 MiB | 4.02 MiB/s, done.
  error: unable to create file node_modules/.pnpm/@typescript-eslint+typescript-estree@8.18.1/node_modules/@typescript-eslint/typescript-estree/dist/create-program/shared.d.ts: Filename too long
  fatal: cannot checkout some of the files

patterns:
  - "Filename too long"
  - "unable to create file"
  - "cannot checkout some of the files"

means: >
  Windows has historically limited a full path to 260 characters, and git is using the old
  interface that enforces that limit. The repository contains a file whose path is longer
  than that once your folder location is added to the front. The files git could create are
  on disk and the ones it could not are missing, so the checkout is incomplete rather than
  failed outright.

fix_ladder:
  - try: Turn on long path support in git.
    command: git config --global core.longpaths true
    shell: powershell
    why: >
      Assumes git is the only thing in the way, which it usually is. This makes git use the
      Windows interface that has no 260-character limit. Set it before you clone again, since
      it does not repair a checkout that already failed.

  - try: Finish the checkout that stopped halfway.
    command: git checkout .
    shell: powershell
    why: >
      Assumes the clone downloaded everything and only the file creation failed. Once
      `core.longpaths` is set, this writes out the files that were skipped. Run it from
      inside the cloned folder.

  - try: Clone into a shorter folder path.
    command: git clone https://github.com/nyxlocke/<repo>.git C:\dev\<repo>
    shell: powershell
    why: >
      Assumes your folder location is eating the budget. `C:\Users\<yourname>\Documents\projects\`
      spends over forty characters before the repository even starts. `C:\dev\` spends seven.

  - try: Turn the limit off in Windows itself.
    command: New-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem" -Name LongPathsEnabled -Value 1 -PropertyType DWORD -Force
    shell: powershell
    why: >
      Assumes other tools are hitting the limit too, not only git. This needs an administrator
      window and a restart. It helps Node, Python, and build tools as well, since each one has
      its own version of this problem.

if_none_worked: >
  Paste the whole clone output including the long file path that failed, the folder you ran the
  clone from, and the output of `git config --get core.longpaths`. That file path is the piece
  people shorten because it is enormous, and its actual length is the entire question.

see_also:
  - c7-files-folders-and-paths
  - d2-repo-remote-clone-origin
  - j3-project-layouts

keywords:
  - filename too long
  - core.longpaths
  - 260 character limit
  - clone failed windows
  - long path
---

This is a Windows problem that only Windows users ever see, and it turns up most on
JavaScript projects, where nested dependency folders produce genuinely absurd paths.

Setting `core.longpaths` is a one-time thing worth doing on a new machine before you need it.
It changes nothing else about how git behaves.

Watch the trap in the middle of this. A clone that hits the limit still leaves a folder
behind with most of the files in it, and a partial checkout looks like a working project
until something imports the one file that never arrived. If a clone printed this error, do
not start working in that folder until `git status` is clean.
