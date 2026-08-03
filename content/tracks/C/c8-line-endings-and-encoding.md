---
id: c8-line-endings-and-encoding
title: Line endings and text encoding
type: section
track: C
order: 80
verified: 2026-08-02
volatility: low
verify: git config --get core.autocrlf
answer: >
  Windows ends a line with two invisible characters where everything else uses
  one, which is what git's `LF will be replaced by CRLF` warning means, and why
  a diff sometimes claims every line in a file changed when nothing visible did.
owns:
  - CRLF vs LF
  - UTF-8
  - BOM
  - .gitattributes
see_also:
  - d9-reading-a-diff
  - c7-files-folders-and-paths
  - f1-how-to-read-an-error-message
  - git-lf-will-be-replaced-by-crlf
keywords:
  - LF will be replaced by CRLF
  - line endings
  - crlf
  - whole file shows as changed
  - bad interpreter
  - encoding
  - utf-8
  - weird characters
  - mojibake
---

## More

Windows ends every line of a text file with CRLF (Carriage Return then Line Feed), a pair of
invisible characters. Linux, macOS, and effectively every tool in software use LF (Line
Feed) on its own. The CR (Carriage Return) is the extra one, and it is there because early
printers needed one instruction to move down a line and another to return to the left edge.

Nothing in your editor shows you which one a file uses, and every tool has an opinion.

Git for Windows handles it by converting. Its default setting, `core.autocrlf=true`, means
files get CRLF when they land in your folder and are converted back to LF when you commit.
The repository stays LF for everyone; your machine gets what Windows expects. That
conversion is exactly what this is telling you:

```text
warning: in the working copy of 'src/app.ts', LF will be replaced by CRLF the next time Git touches it
```

It is a warning, not an error. Nothing failed, nothing is broken, and you can ignore it
forever. See the [error card](#git-lf-will-be-replaced-by-crlf) if you want the short
version.

The problem worth knowing about is the other symptom. Open a file in a tool that writes the
other kind of ending, save it, and every single line technically changed. Git shows the whole
file as rewritten, a two-line fix becomes a 400-line diff, and nobody can review it
([d9](#d9-reading-a-diff)). Confirm it in one command:

```powershell
git diff --ignore-cr-at-eol
```

If that prints only the lines you actually edited, the rest was line endings. If it prints
the same wall of changes, something really did rewrite the file.

The permanent fix is a `.gitattributes` file that settles the question for the project
instead of for each machine, and it is in Full.

## Full

### Check what git is doing on your machine

```powershell
git config --get core.autocrlf
```

`true` is the Git for Windows default: convert to CRLF on the way out, LF on the way in.
`input` converts on the way in only, which is the setting most Linux and macOS machines use.
`false` means no conversion at all. Nothing printed means it was never set, which behaves as
`false`.

You do not need to change this. The project-level file below is better, because it applies
to everyone who touches the repository rather than to whoever remembered to run a command.

### .gitattributes, the fix that travels with the project

Create a file called `.gitattributes` in the root of the repository, next to `.gitignore`:

```text
* text=auto
*.sh text eol=lf
*.ps1 text eol=crlf
*.png binary
*.jpg binary
```

Line by line. `* text=auto` tells git to treat everything as text and normalize it to LF in
the repository, which is the whole ballgame. `*.sh text eol=lf` forces shell scripts to LF
on disk as well, because they break otherwise. `*.ps1 text eol=crlf` does the opposite for
PowerShell scripts, which prefer CRLF. The `binary` lines tell git to leave images alone
entirely, since converting bytes inside a picture would destroy it.

Commit that file. Existing files already in the repository keep their current endings until
they are touched, so if you want to fix them all at once, ask your agent to run git's
renormalize step and review the result as its own commit.

### The shell script that fails with a nonsense character

```text
bash: ./setup.sh: /bin/bash^M: bad interpreter: No such file or directory
```

That `^M` is a carriage return that got saved into the file. Bash read the first line, took
everything up to the end of it as the name of the interpreter, and went looking for a
program called `bash` with an invisible character stuck on the end. The `eol=lf` line above
prevents it. To fix one file now, open it in your editor and switch its line endings to LF,
then save.

This is worth recognizing on sight, because the error names a file that obviously exists and
sounds like a broken install.

### Encoding, which is the other invisible property

A text file is bytes. An **encoding** is the agreement about which bytes mean which
characters. UTF-8 is the answer to that question everywhere in software, it covers every
language and emoji, and plain English text in it is byte-for-byte identical to the old ASCII
files, which is why the problem stays hidden until somebody types a curly quote.

Two things go wrong.

**Mojibake.** You see `â€™` where an apostrophe should be, or `Ã©` instead of `é`. That is a
UTF-8 file being read by something that assumed the old Windows-1252 encoding. The file is
usually fine and the reader is wrong.

**A crash on a specific file.** Python raises `UnicodeDecodeError` naming a byte position;
other languages raise something similar. It means the file is not the encoding the program
assumed. See the [Python error card](#python-unicodedecodeerror).

### The byte order mark

A BOM (Byte Order Mark) is three invisible bytes some tools write at the very start of a
file to announce "this is UTF-8". Most things ignore it. The ones that do not produce
memorable failures: a `.json` file that fails to parse at character 1, a shell script whose
first line is not recognized, and a `.csv` whose first column header has an invisible prefix
so no lookup matches it.

Windows tools write it more often than anything else does. Prefer UTF-8 without a BOM for
everything, which is what editors mean by "UTF-8" versus "UTF-8 with BOM".

### PowerShell writes files in a surprising encoding

In PowerShell 5.1, the version that ships with Windows, redirecting output with `>` or
`Out-File` writes UTF-16, which doubles the file size and looks like garbage to most tools.
PowerShell 7 fixed this and writes UTF-8 without a BOM.

When it matters, be explicit:

```powershell
"hello" | Set-Content -Path .\notes.txt -Encoding utf8
```

`-Encoding utf8` removes the guesswork. This is worth remembering the first time a log file
you captured is unreadable to the thing you captured it for.

### Seeing both properties at a glance

Open the file in Visual Studio Code and look at the status bar along the bottom right. It
shows the encoding (`UTF-8`) and the line ending (`LF` or `CRLF`) for the file you are
looking at. Both are buttons. Click the line ending to convert the file, then save.

That is the fastest way to answer "is this actually the problem", and it takes about three
seconds.
