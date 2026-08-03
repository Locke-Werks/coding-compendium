---
id: git-lf-will-be-replaced-by-crlf
title: "warning: LF will be replaced by CRLF"
type: error
verified: 2026-08-02
volatility: low

category: config

# Prints the current line-ending setting. On Windows the usual value is true.
verify: git config --get core.autocrlf

sample: |
  PS C:\Users\nyx\dev\scraper> git add .
  warning: in the working copy of 'src/app.py', LF will be replaced by CRLF the next time Git touches it
  warning: in the working copy of 'README.md', LF will be replaced by CRLF the next time Git touches it

patterns:
  - "LF will be replaced by CRLF"
  - "CRLF will be replaced by LF"
  - "in the working copy of"

means: >
  This is a warning, not an error, and the command it appeared during worked. Text files mark
  the end of each line with an invisible character. Windows uses two of them, a carriage
  return followed by a line feed, written CRLF. Almost everything else uses one, the line feed
  alone, written LF. Git is set to store LF in the repository and hand you CRLF in your
  folder, and it is telling you it is about to do the conversion.

fix_ladder:
  - try: Do nothing and carry on.
    why: >
      Assumes the conversion is working as intended, which it is on a standard Git for Windows
      install. Your commit went through. Your files are fine. Every editor written this
      decade handles both kinds of line ending without noticing.

  - try: Check what the setting actually is.
    command: git config --get core.autocrlf
    shell: powershell
    why: >
      Assumes you want to know what is doing this. `true` converts to CRLF in your folder and
      back to LF on the way into a commit, which is the Windows default and the source of this
      warning. `input` stores LF and leaves your folder alone. `false` turns conversion off.

  - try: Turn the warning off without changing the behavior.
    command: git config --global core.safecrlf warn
    shell: powershell
    why: >
      Assumes the message is only noise to you. This affects what git prints, not what it
      stores. Nothing about your files changes.

  - try: Settle the rule in the project instead of on your machine.
    why: >
      Assumes you want the same result on any machine that clones this project. Add a file
      named `.gitattributes` at the repository root containing `* text=auto`. That records
      the decision in the repository, where it applies to everyone, rather than in your
      personal config. See c8-line-endings-and-encoding.

  - try: Look at whether a whole file is showing as changed.
    command: git diff --stat
    shell: powershell
    why: >
      Assumes the real problem is a diff claiming every line changed when you edited one. That
      is line endings being rewritten wholesale by a tool that disagreed with git, and it is
      the version of this that actually costs you something.

if_none_worked: >
  Paste the warnings, the output of `git config --list --show-origin | Select-String crlf`, and
  the contents of `.gitattributes` if the project has one. The origin column shows which config
  file each setting came from, and a project-level setting quietly disagreeing with your global
  one is the case that makes this genuinely confusing.

see_also:
  - c8-line-endings-and-encoding
  - d9-reading-a-diff
  - b2-install-git

keywords:
  - LF will be replaced by CRLF
  - line endings
  - autocrlf
  - gitattributes
  - whole file shows as changed
---

The first thing to know is that this line begins with `warning:` and your command succeeded.
Nothing was rejected and nothing needs fixing.

The reason git converts at all is that the rest of the world stores one line feed character
at the end of each line. If your Windows machine committed the two-character carriage return
and line feed pair, every collaborator on any other system would see spurious changes on
every line. Converting on the way in and out keeps the stored version consistent while giving
you what Windows expects locally.

The version worth caring about is when a diff shows every line of a file changed after you
edited one line. That means something rewrote the whole file with different line endings,
which usually means one tool in the project disagrees with git's setting. A `.gitattributes`
file at the repository root fixes it for everyone at once, which a personal setting cannot do.
