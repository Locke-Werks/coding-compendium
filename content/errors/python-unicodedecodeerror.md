---
id: python-unicodedecodeerror
title: "UnicodeDecodeError: 'charmap' codec can't decode byte"
type: error
verified: 2026-08-02
volatility: low

language: python
category: broke-at-runtime

# Reads the file with an explicit encoding. Printing "ok" means the file is
# valid UTF-8 and the default encoding was the problem.
verify: python -c "open('data.csv', encoding='utf-8').read(); print('ok')"

sample: |
  PS C:\Users\you\dev\scraper> python main.py
  Traceback (most recent call last):
    File "C:\Users\you\dev\scraper\main.py", line 4, in <module>
      text = open("data.csv").read()
             ~~~~~~~~~~~~~~~~~~~~~~~
    File "C:\Users\you\AppData\Local\Programs\Python\Python312\Lib\encodings\cp1252.py", line 23, in decode
      return codecs.charmap_decode(input,self.errors,decoding_table)[0]
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  UnicodeDecodeError: 'charmap' codec can't decode byte 0x8f in position 1425: character maps to <undefined>

patterns:
  - "UnicodeDecodeError"
  - "codec can't decode byte"
  - "character maps to"
  - "'charmap' codec"
  - "'utf-8' codec can't decode"

means: >
  Text files store characters as numbers, and an encoding is the table that maps between the
  two. Python on Windows opens text files with the old Windows table, `cp1252`, unless you say
  otherwise. The file uses a different table, almost always UTF-8, so a byte in it has no
  meaning in `cp1252` and Python stops. The file is not corrupt. It is being read with the
  wrong table.

fix_ladder:
  - try: Say which encoding the file uses when you open it.
    command: open("data.csv", encoding="utf-8")
    shell: powershell
    why: >
      Assumes the file is UTF-8, which it is the overwhelming majority of the time. Every text
      file created by a modern editor, exported from a website, or written by another program
      on any system uses it. Always pass `encoding=` rather than relying on the default.

  - try: Try the encoding with a byte-order mark if that still fails.
    command: open("data.csv", encoding="utf-8-sig")
    shell: powershell
    why: >
      Assumes the file starts with three extra bytes that mark it as UTF-8. Excel and Notepad
      both add them when saving. The plain `utf-8` reader leaves those bytes in your text as a
      strange character on the first line, and `utf-8-sig` strips them.

  - try: Check what the file actually is.
    command: Get-Content data.csv -TotalCount 3
    shell: powershell
    why: >
      Assumes you cannot tell text from binary. If this prints readable lines, it is a text
      file with an encoding question. If it prints nonsense, the file is a spreadsheet, an
      image, or a compressed archive, and no encoding will help.

  - try: Open it as binary if it is not text.
    command: open("data.xlsx", "rb")
    shell: powershell
    why: >
      Assumes the file is not text at all. `.xlsx`, `.pdf`, `.png`, and `.zip` files are binary
      and reading them as text is meaningless. Use a library that understands the format, such
      as `openpyxl` for Excel files.

  - try: Skip the bytes that will not decode, as a last resort.
    command: open("data.csv", encoding="utf-8", errors="replace")
    shell: powershell
    why: >
      Assumes the file is a mix of encodings, which happens with data assembled by hand over
      years. `errors="replace"` puts a placeholder character where decoding fails and keeps
      going. You lose those characters, so do this knowingly rather than to make an error
      stop.

if_none_worked: >
  Paste the whole traceback including the byte value and position, and the output of
  `Get-Content <file> -TotalCount 3`. The byte value is the piece that gets trimmed and it
  identifies the real encoding: bytes in the `0x80` to `0x9f` range usually mean the file is
  UTF-8 being read as `cp1252`.

see_also:
  - c8-line-endings-and-encoding
  - f2-stack-traces
  - python

keywords:
  - UnicodeDecodeError
  - charmap codec
  - cp1252
  - utf-8 encoding
  - character maps to undefined
---

This error is almost entirely a Windows experience. On macOS and Linux, Python defaults to
UTF-8 and the same code works. On Windows it defaults to the local Windows code page, which
for a United States or Western European install is `cp1252`.

That is why an agent writes `open("data.csv")` with no encoding and it works on its own test
and fails on your machine. The code is not wrong anywhere else. It is incomplete, and Windows
is where the gap shows.

The habit worth building is passing `encoding="utf-8"` to every `open()` call you write. It
costs nothing when the default is already right and saves the whole class of problem when it
is not.

There is a related message pointing the other way. `'utf-8' codec can't decode byte` means the
file is not UTF-8, and `cp1252` or `latin-1` is worth trying next. `latin-1` never fails on
any byte, which makes it useful for inspecting a file and a poor choice for reading one
properly.
