---
id: git-dubious-ownership
title: "fatal: detected dubious ownership in repository"
type: error
verified: 2026-08-02
volatility: low

category: permission

# Lists every folder git has been told to trust. The one you just added should
# appear in the output.
verify: git config --global --get-all safe.directory

sample: |
  PS C:\Users\nyx\dev\scraper> git status
  fatal: detected dubious ownership in repository at 'C:/Users/nyx/dev/scraper'
  'C:/Users/nyx/dev/scraper' is owned by:
          'S-1-5-21-1004336348-1177238915-682003330-1001'
  but the current user is:
          'S-1-5-21-1004336348-1177238915-682003330-500'
  To add an exception for this directory, call:

          git config --global --add safe.directory C:/Users/nyx/dev/scraper

patterns:
  - "detected dubious ownership"
  - "safe.directory"
  - "is owned by"

means: >
  Windows records an owner for every folder, and the owner of this repository is a different
  account from the one running git. Git treats that as a risk, because a repository can
  contain settings that run programs, and refuses to touch it until you say the folder is
  fine. Nothing is damaged. Git has not looked inside the repository at all.

fix_ladder:
  - try: Tell git this specific folder is fine.
    command: git config --global --add safe.directory C:/Users/<yourname>/dev/<project>
    shell: powershell
    why: >
      Assumes you know where the folder came from and trust it. Copy the path out of the
      error message exactly, including the forward slashes, which git wants even on Windows.
      This is the command the error itself suggests and it is the right answer nearly every
      time.

  - try: Work out why the owner differs before you trust it.
    command: (Get-Acl .).Owner
    shell: powershell
    why: >
      Assumes you want to know what happened. The usual causes are harmless: the folder was
      created by an administrator window, restored from a backup, copied from another
      machine, or is sitting on an external drive. An unfamiliar answer here is worth
      pausing over.

  - try: Give the folder back to your own account.
    command: takeown /F . /R /D Y
    shell: powershell
    why: >
      Assumes you would rather fix the ownership than add an exception. This walks the whole
      folder tree and can be slow on a repository with a large `node_modules` folder. It
      changes only ownership, never file contents.

  - try: Check whether the repository lives somewhere awkward.
    command: Get-Location
    shell: powershell
    why: >
      Assumes the project is on a network drive, a shared folder, or a path inside another
      user's home folder. Those produce mismatched ownership every time. Moving the project
      under `C:\Users\<yourname>\dev\` makes the problem stop happening rather than papering
      over it.

if_none_worked: >
  Paste the whole error including both long identifier strings, the output of
  `(Get-Acl .).Owner`, and the full path of the repository. The two identifiers are what
  people trim because they look like noise, and comparing them is how you tell an
  administrator-created folder apart from a genuinely foreign one.

see_also:
  - c7-files-folders-and-paths
  - b9-where-settings-live
  - b2-install-git

keywords:
  - dubious ownership
  - safe.directory
  - git refuses to run
  - folder owner mismatch
  - takeown
---

Git added this check after a real attack. A repository can carry configuration that runs
commands during ordinary git operations, so opening one that another account controls used
to be enough to get code running as you.

On a personal Windows machine the mismatch is almost always self-inflicted and boring. You
ran an installer or a script from an administrator terminal, and it created the folder as
the administrator account rather than as you.

The identifier strings in the message are Windows security identifiers. You do not need to
read them, but you can tell the accounts apart by the number after the last dash: `1001` is
usually your normal account, `500` is the built-in administrator.

Adding a `safe.directory` exception is a per-folder decision on purpose. There is a wildcard
form that trusts everything, and it turns the check off entirely across your whole machine,
which gives back the exact hole the check exists to close.
