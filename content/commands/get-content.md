---
id: get-content
title: Get-Content
type: command
verified: 2026-08-02
volatility: low

tool: powershell
command: Get-Content <file>
shell: powershell

does: >
  Prints the contents of a text file to your terminal, one line at a time.

flags:
  - flag: "-TotalCount <count>"
    means: >
      Reads only the first N lines and stops. This is what `head` does on macOS and Linux, and
      the way to peek at a huge log without printing all of it.
  - flag: "-Tail <count>"
    means: >
      Reads only the last N lines. The equivalent of `tail`, and the usual way to see the end
      of a log where the error lives.
  - flag: "-Wait"
    means: >
      Keeps the file open and prints new lines as they are written. Combined with `-Tail 20`
      this is how you watch a log live. Press Ctrl+C to stop.
  - flag: "-Raw"
    means: >
      Returns the whole file as a single block of text instead of an array of lines. Needed
      when you are passing the content to something that expects one string.

expect: >
  The file's lines printed in order. `Cannot find path ... because it does not exist.` means
  a typo in the name or the wrong folder.

see_also:
  - select-string
  - get-childitem
  - f4-logs

keywords:
  - read a file
  - cat in powershell
  - print file contents
  - tail a log
  - watch a log file
---

`cat`, `gc`, and `type` are aliases for this command in PowerShell, so instructions written
for another shell usually work.

Do not print a large file. `Get-Content` on a hundred-megabyte log will fill your terminal for
several minutes. Use `-Tail 50` and look at the end, which is where the failure is.
