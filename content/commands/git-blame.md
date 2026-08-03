---
id: git-blame
title: git blame
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git blame <file>
shell: any

does: >
  Prints a file with every line labeled by the commit, author, and date that last changed
  that line.

flags:
  - flag: "-L <start>,<end>"
    means: >
      Limits the output to a range of lines, as in `git blame -L 40,60 <file>`. Essential on
      a long file, where the full output is unreadable.
  - flag: "-w"
    means: >
      Ignores whitespace-only changes, so a reformatting pass does not claim credit for
      every line in the file. Almost always what you want.
  - flag: "-C"
    means: >
      Follows lines that were moved or copied from another file, which finds the real origin
      of code that was relocated.
  - flag: "-- <file>"
    means: The bare `--` separates the filename from options, needed when a file is named like a flag.

expect: >
  One line per line of the file, prefixed with a short hash, the author name in parentheses,
  a date, and the line number. Press `q` to leave the pager.

see_also:
  - git-log
  - git-show
  - d13-tags-releases-and-history

keywords:
  - who wrote this line
  - when did this line change
  - find the commit that broke this
  - annotate file
---

The name is unfortunate. The real use is investigation: you found a strange line, and you
want the commit that introduced it so you can read the message and the rest of that change.

Take the hash from the left column and run `git show <that-hash>` to see the whole commit in
context.
