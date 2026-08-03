---
id: select-string
title: Select-String
type: command
verified: 2026-08-02
volatility: low

tool: powershell
command: Select-String -Path <file> -Pattern "<text>"
shell: powershell

does: >
  Searches inside files for lines matching a piece of text and prints each match with its
  filename and line number.

flags:
  - flag: '-Pattern "<text>"'
    means: >
      What to look for. It is treated as a regular expression by default, so characters like
      `.`, `*`, `(`, and `[` carry special meaning rather than matching themselves.
  - flag: "-Path <file>"
    means: >
      Which files to search. Wildcards work: `-Path .\src\*.ts`. For a whole tree, pipe
      `Get-ChildItem -Recurse` into this command instead.
  - flag: "-SimpleMatch"
    means: >
      Turns off the regular expression handling and matches the text literally. Use it any
      time your search string contains punctuation and you keep getting no results.
  - flag: "-CaseSensitive"
    means: The search ignores capitalization unless you add this.
  - flag: "-Context <before>,<after>"
    means: >
      Prints surrounding lines as well, as in `-Context 2,2`. Invaluable when the match itself
      does not tell you enough.
  - flag: "-List"
    means: Stops at the first match per file, which turns the output into a list of which files contain the thing.

expect: >
  One line per match in the form `path\to\file.ts:42:the matching line`. Nothing printed means
  no match, which is a real answer rather than an error.

see_also:
  - get-content
  - get-childitem
  - j4-reading-a-repo-you-did-not-write

keywords:
  - grep in powershell
  - search inside files
  - find text in a file
  - which file contains
---

This is the PowerShell answer to `grep`. To search a whole project:

```powershell
Get-ChildItem -Recurse -Filter *.ts | Select-String -Pattern "loginRedirect"
```

The pipe sends every matching file into the search. Add `-Exclude node_modules` to the first
half if the results drown in dependency code.
