---
id: f3-exit-codes-and-streams
title: Exit codes, stdout, and stderr
type: section
track: F
order: 30
verified: 2026-08-02
volatility: low
verify: $LASTEXITCODE
answer: >
  Every command finishes with a number, zero meaning success and anything else
  meaning failure, and it writes ordinary output and errors down two separate
  channels, which is why sending output to a file can leave the error out of the
  file.
owns:
  - exit code
  - stdout
  - stderr
  - redirection
see_also:
  - f1-how-to-read-an-error-message
  - h5-ci-cd
  - f4-logs
  - c5-processes-and-killing-them
keywords:
  - exit code
  - lastexitcode
  - stdout stderr
  - 2>&1
  - redirect output
  - why is the log empty
  - exit code 1
  - exit code 137
  - it failed but printed nothing
---

## More

When a program finishes it hands the shell a number: zero for success, anything else for
failure, and the specific number sometimes says how. You never see it unless you ask:

```powershell
$LASTEXITCODE
```

Prints the number from the last external program you ran. Run it immediately after the thing
you are asking about, because the next command overwrites it.

This matters because automation reads that number and nothing else. A build server does not
read your log and decide whether it looks bad. It runs the command, checks for zero, and
turns the check red if it is anything else ([h5](#h5-ci-cd)). A step can therefore fail while
the output on screen looks completely normal, and that is not a glitch.

The second half is that programs write to two separate channels, and they both land in the
same terminal window, so they look like one thing.

- **stdout**, standard output, is where ordinary results go.
- **stderr**, standard error, is where errors and warnings go.

Keeping them apart is the point: you can capture a program's real output without the
complaints mixed in. What surprises people is the consequence:

```powershell
npm run build > log.txt
```

That captures stdout only. The error is not in `log.txt`, which is a genuinely maddening
twenty minutes if you do not know this rule.

```powershell
npm run build > log.txt 2>&1
```

`2>&1` means "send channel 2, stderr, to wherever channel 1, stdout, is going." Now both are
in the file. This appears in nearly every set of build instructions ever written and it means
the same thing every time.

## Full

### Checking the number, correctly

PowerShell has two things that look interchangeable and are not.

```powershell
git push; $LASTEXITCODE
```

`$LASTEXITCODE` is the exit code from the last **external program**: git, node, npm, cargo.
This is the one you want almost always.

```powershell
$?
```

`$?` is `True` or `False` for whether PowerShell thinks the last command succeeded, including
its own built-in commands. It is convenient and it lies more often, because some programs
write to stderr while succeeding and PowerShell has historically treated that as failure.
When the two disagree, believe `$LASTEXITCODE`.

### The numbers worth recognizing

| Code | Means |
|---|---|
| 0 | Success. The only one that means success. |
| 1 | Generic failure. Most tools use this for everything, so it tells you nothing. |
| 2 | Usually bad arguments or a usage error. |
| 127 | Command not found. See [c4](#c4-path-and-command-not-found). |
| 130 | You pressed Ctrl+C. |
| 137 | Killed from outside, nearly always for running out of memory. |
| 139 | Crashed hard. In a language that manages memory for you, this is a bug in a library. |

Node adds its own conventions and npm wraps failures in `ELIFECYCLE`, which means "the script
this project defined exited non-zero" and never means anything more specific. Look above it
for the real error.

The number 137 is worth knowing on sight. It shows up when a build runs out of memory inside
a container or on a small build server, and the log usually ends mid-sentence with no error
at all, because the program was removed rather than allowed to complain.

### Which channel a message came out of

Warnings, progress bars, and download spinners are written to stderr on purpose, so that they
do not corrupt output you might be piping somewhere. Two things follow.

**Red text is not proof of failure.** `npm WARN deprecated` arrives on stderr and the command
succeeded. The exit code is the proof, and warnings are covered in
[f1](#f1-how-to-read-an-error-message).

**A silent command may still have failed.** Check the number.

### Redirection, the whole set

```powershell
npm run build > log.txt        # stdout to a new file, overwriting it
npm run build >> log.txt       # stdout appended to the end of the file
npm run build 2> errors.txt    # stderr only, to its own file
npm run build > log.txt 2>&1   # both, into one file, in the order they happened
npm run build *> log.txt       # PowerShell shorthand for every channel at once
```

The order in the fourth line matters and is easy to get backward. `> log.txt` first points
stdout at the file, then `2>&1` points stderr at wherever stdout is now going. Reversed, you
get stderr aimed at the terminal and only stdout in the file.

To see it and save it at the same time:

```powershell
npm run build 2>&1 | Tee-Object -FilePath log.txt
```

`Tee-Object` writes to the file and passes everything through to your screen, named after a
T-shaped pipe fitting.

### The pipe carries stdout only

```powershell
npm run build | Select-String "error"
```

That finds nothing, and the conclusion "so there is no error" is wrong. The error went down
stderr and the pipe never saw it. Merge first:

```powershell
npm run build 2>&1 | Select-String "error"
```

Same trap in reverse: a command that prints a lot and pipes cleanly can still have failed.

### Running one command only if the last one worked

```powershell
npm run build && npm test
```

`&&` runs the second command only if the first exited zero. `||` runs it only if the first
did not. Both work in PowerShell 7 and in Git Bash. They do not work in Windows PowerShell
5.1, the version that ships in the box, which is one more reason to install PowerShell 7.

This is exactly how a build pipeline is written, and it is why one failing step stops
everything after it.

### The failure with no message at all

A command exits non-zero and prints nothing. In order:

1. Re-run it with a verbosity flag: `--verbose`, `-v`, or `--debug`. Most tools have one.
2. Merge the channels with `2>&1` in case you were filtering the error out yourself.
3. Check the tool's own log file, which is usually mentioned in the last line of output when
   there is any output at all ([f4](#f4-logs)).
4. Check `$LASTEXITCODE`. A `137` says it was killed rather than failed, which points at
   memory rather than at your code.

"This command exits with code 3 and prints nothing" is a specific, useful bug report. "It
doesn't work" is not.
