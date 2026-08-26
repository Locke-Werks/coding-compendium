---
id: f4-logs
title: Logs, and where to find them
type: section
track: F
order: 40
verified: 2026-08-02
volatility: quarterly
verify: Get-ChildItem "$env:LOCALAPPDATA\npm-cache\_logs" | Select-Object -Last 3
answer: >
  A log is the running commentary a program writes to a file while it works, so
  when the terminal has scrolled away or says nothing useful, find that file and
  read it for the first error rather than the last one.
owns:
  - log levels
  - log locations
  - tailing
see_also:
  - f3-exit-codes-and-streams
  - f1-how-to-read-an-error-message
  - f5-what-to-paste-and-what-not-to
  - g8-what-never-to-paste-into-a-chat
keywords:
  - where are the logs
  - log file
  - tail a log
  - npm debug log
  - complete log of this run
  - log levels
  - watch a log live
---

## More

Terminal output disappears. You close the window, or ten thousand lines scroll past, and the
one line that mattered is gone. A **log** is the same commentary written to a file instead,
so it is still there afterward.

Most logs tag each line with a **level**, which is how important the program thought that
line was. From quietest to loudest: `TRACE`, `DEBUG`, `INFO`, `WARN`, `ERROR`, `FATAL`. Tools
normally show you `INFO` and above and write `DEBUG` to the file, which is why the file has
far more in it than the screen did.

Where to look, on Windows:

| Tool | File |
|---|---|
| npm, on any failure | `C:\Users\<yourname>\AppData\Local\npm-cache\_logs\` |
| pip | `C:\Users\<yourname>\AppData\Local\pip\Cache\` |
| Most apps | `C:\Users\<yourname>\AppData\Roaming\<app name>\` |
| Most tools | `C:\Users\<yourname>\AppData\Local\<app name>\` |
| A project's own log | usually `logs\` in the project folder |

npm makes this easy by telling you. The last line of a failed install is almost always:

```text
npm ERR! A complete log of this run can be found in:
npm ERR!     C:\Users\you\AppData\Local\npm-cache\_logs\2026-08-02T14_22_09_881Z-debug-0.log
```

That path is the answer to "where are the logs", and people scroll past it every day.

To watch one while something runs:

```powershell
Get-Content .\logs\app.log -Wait -Tail 20
```

`-Tail 20` starts you at the last twenty lines instead of the beginning, and `-Wait` keeps
the window open and prints new lines as they arrive. Ctrl+C stops watching, and stops nothing
else.

One rule that saves the most time: read for the **first** error, not the last. One failure
usually knocks over five more things on its way down, so the bottom of the file is full of
consequences and the cause is further up.

## Full

### What the levels actually mean

- `TRACE` and `DEBUG`: internal detail, written for whoever wrote the program. Enormous
  volume. Off by default, and worth turning on when you are stuck.
- `INFO`: normal progress. "Server started", "connected", "3 files written."
- `WARN`: something is odd and the program continued. Deprecation notices live here.
- `ERROR`: an operation failed. The program may still be running.
- `FATAL`: the program is stopping.

An `ERROR` line in a log is not proof that your command failed. Plenty of programs log an
error, retry, and succeed. The exit code decides ([f3](#f3-exit-codes-and-streams)).

Turning up the detail is usually a flag or an environment variable:

```powershell
$env:DEBUG = "*"; npm run dev
```

That is the convention for a large family of Node tools. Others use `--verbose`, `-vvv`, or
`RUST_LOG`. The tool's own documentation has the right one, and `--help` usually lists it.

### Finding something in a large file

```powershell
Select-String -Path .\logs\app.log -Pattern "error" -Context 3
```

`-Pattern` is the text to find, and `-Context 3` prints three lines either side of each hit,
which is what makes the result readable. Without it you get a list of matches with no
surroundings and no way to tell which one started the trouble.

```powershell
Get-Content .\logs\app.log -Tail 200
```

The last 200 lines, for when you just want to see how it ended.

```powershell
Select-String -Path .\logs\app.log -Pattern "error" | Select-Object -First 1
```

The first error in the file. This is the one you want, for the reason above.

### Reading a timestamped log

Two habits.

**Match the time to your run.** A log is append-only, so the top of the file can be from
weeks ago. Note the clock when you reproduce the problem, then find that minute in the file.

**Check the time zone.** Servers and containers log in UTC (Coordinated Universal Time),
which will be several hours off your clock. A log that appears to contain nothing from your
run usually contains everything from your run, five hours earlier.

### Logs that are one object per line

Server tools increasingly write JSON (JavaScript Object Notation), one object per line:

```text
{"level":"error","time":"2026-08-02T14:22:09Z","msg":"connection refused","port":5432}
```

Unpleasant to read and easy to search, which is the trade they made deliberately. The `msg`
field is the human part. Read that first and the surrounding fields second.

### Logs that rotate

A long-running program renames its log periodically and starts a new one, so you find
`app.log`, `app.log.1`, and `app.log.2026-08-01.gz` sitting together. `app.log` is always the
current one. The numbered and dated ones are older, and the compressed ones are older still.
If today's log seems to start in the middle of something, the beginning is in the file next
to it.

### The Windows Event Viewer

Windows keeps its own log for services and crashes, and it is where a program that died
without printing anything sometimes leaves a note. Press Start, type `Event Viewer`, and look
under Windows Logs, then Application. Sort by time and find your minute.

Worth checking exactly once per problem, when a program vanished with no output at all. It is
rarely the fastest route to anything else.

### What to do with what you find

Copy the relevant slice, not the file. A log is frequently tens of thousands of lines and
pasting all of it into an agent wastes the context it needs for your actual code
([e2](#e2-context-windows)). The useful slice is: the first `ERROR` line, everything from
there to the end of that stack trace, and about twenty lines before it for what the program
was doing at the time.

Two cautions before you paste.

**Logs contain credentials more often than any other file.** Connection strings, tokens in
request headers, and full URLs with keys in them all end up in logs routinely. Check before
sending anything anywhere. [g8](#g8-what-never-to-paste-into-a-chat) is the list.

**Logs contain other people's data.** Email addresses, names, and record identifiers get
logged constantly. The same rule applies.

[f5](#f5-what-to-paste-and-what-not-to) covers what a useful paste looks like once it is
clean.
