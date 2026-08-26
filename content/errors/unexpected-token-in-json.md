---
id: unexpected-token-in-json
title: "Unexpected token } in JSON at position N"
type: error
verified: 2026-08-02
volatility: low

language: json
category: wont-compile

# Prints the parsed contents as an object when the file is valid, and a clear
# error with a position when it is not.
verify: Get-Content package.json -Raw | ConvertFrom-Json

sample: |
  PS C:\Users\you\dev\site> npm install
  npm error code EJSONPARSE
  npm error path C:\Users\you\dev\site\package.json
  npm error JSON.parse Unexpected token "}" (0x7D) in JSON at position 412 while parsing near "...\"vite\": \"^8.2.0\",\n  }\n}"
  npm error JSON.parse Failed to parse JSON data.
  npm error JSON.parse Note: package.json must be actual JSON, not just JavaScript.

patterns:
  - "in JSON at position"
  - "is not valid JSON"
  - "EJSONPARSE"
  - "JSON.parse"
  - "Unexpected end of JSON input"

means: >
  Something tried to read a JSON (JavaScript Object Notation) file and the text is not valid
  JSON. The parser reports the character that broke it and how far into the file it got. JSON
  is stricter than it looks: no trailing commas, no comments, double quotes only, and every
  bracket paired. One stray character makes the whole file unreadable, so nothing that depends
  on it can start.

fix_ladder:
  - try: Look for a comma before a closing brace or bracket.
    why: >
      Assumes a trailing comma, which causes this more than every other mistake combined. The
      error quotes the text around the break, and in the sample you can see `"^8.2.0",` with a
      comma and then `}` immediately after. JavaScript allows that comma. JSON does not.

  - try: Open the file in Visual Studio Code and look for the red underline.
    why: >
      Assumes you cannot find it by eye, which is normal in a long file. The editor
      understands JSON natively and marks the exact character. The Problems panel lists it
      with a line number, which the raw error does not give you.

  - try: Check the file for comments.
    why: >
      Assumes someone added a `//` note. JSON has no comment syntax at all. `tsconfig.json`
      and Visual Studio Code's own settings files are a special relaxed variant that allows
      them, and `package.json` is not, so a comment copied between the two breaks it.

  - try: Check for single quotes and unquoted keys.
    why: >
      Assumes the text was written as JavaScript. JSON requires double quotes around every
      key and every string value. `{name: 'site'}` is valid JavaScript and invalid JSON, and
      agents produce it when they are thinking in one language and writing in the other.

  - try: Parse the file yourself to confirm it is fixed.
    command: Get-Content package.json -Raw | ConvertFrom-Json
    shell: powershell
    why: >
      Assumes you have made an edit and want certainty before rerunning a slow install. This
      prints the contents as an object when the file is valid. It is faster than a full
      install and gives a clearer position when it still fails.

  - try: Restore the last committed version of the file.
    command: git checkout -- package.json
    shell: powershell
    why: >
      Assumes an agent mangled a file that was fine an hour ago. This throws away every
      uncommitted change to that one file, so read `git diff package.json` first to see what
      you would be giving up.

if_none_worked: >
  Paste the error and the entire contents of the file it names. The file is the piece people
  leave out, because the error looks self-contained. It is not: the position number is
  meaningless without the text it is counting into, and an agent given both finds the
  character in seconds.

danger: >
  The last step runs `git checkout -- package.json`, which discards your uncommitted edits to
  that file with no undo. Run `git diff package.json` first. If the file has never been
  committed, this deletes it outright.

see_also:
  - j2-the-config-formats-nobody-explains
  - b9-where-settings-live
  - f1-how-to-read-an-error-message
  - json

keywords:
  - unexpected token json
  - EJSONPARSE
  - trailing comma
  - invalid json
  - package.json broken
---

The position number counts characters from the start of the file, not lines, which makes it
nearly useless for finding the spot by hand. Ignore it and use the quoted fragment instead:
parsers print the text surrounding the break, and that you can search for.

Node changed the wording partway through version 20. Older builds name the character that
broke it, newer ones say `Expected double-quoted property name`. The newer message is more
specific and both mean the same thing.

Four rules cover nearly every case. No comma after the last item. No comments. Double quotes
around everything, including keys. Every opening brace has a closing one.

Related and worth knowing: `Unexpected end of JSON input` means the file stopped early, which
usually means it is empty or a download was cut off partway.
