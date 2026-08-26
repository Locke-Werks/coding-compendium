---
id: k6-history-completion-and-keys
title: Tab, arrow keys, and the rest of the keyboard
type: section
track: K
order: 60
verified: 2026-08-25
volatility: low
verify: Get-History
answer: >
  Tab completes what you are typing, the up arrow brings back what you typed before, and
  Ctrl+R searches everything you have ever run, which together remove most of the typing and
  nearly all of the typos.
owns:
  - command history
  - tab completion
  - the console key bindings
  - copy and paste in a terminal
see_also:
  - c5-processes-and-killing-them
  - k2-anatomy-of-a-command
  - k3-the-first-ten-commands
  - c7-files-folders-and-paths
keywords:
  - tab completion
  - command history
  - up arrow terminal
  - ctrl r search history
  - copy paste in terminal
  - keyboard shortcuts console
  - previous command
  - psreadline
---

## More

The console rewards a handful of keys more than it rewards typing accurately. These are the
ones that matter.

**Tab completes.** Type the first few characters of a file, folder, or command and press
Tab. The shell finishes it. Press Tab again to cycle through the other matches.

This is worth more than the keystrokes it saves. A path you completed with Tab is spelled
correctly by construction, so half of all "cannot find the path specified" errors never
happen. Tab also completes PowerShell parameter names: type `Get-ChildItem -Rec` and press
Tab to get `-Recurse` written out in full.

**Up arrow repeats.** Brings back the last command. Press it repeatedly to walk further
back. Edit the line and press Enter to run the modified version. Down arrow walks forward
again.

**Ctrl+R searches.** Press it, then type any fragment of a command you ran before. The shell
shows the most recent match as you type. Press Enter to run it, or the right arrow to put it
on the line for editing. Press Ctrl+R again to step to an older match.

This is the one that changes how the console feels. You do not have to remember the command
you used to build the installer three weeks ago. You have to remember one word in it.

**Ctrl+C stops what is running.** The single most useful key when something has run away
from you. It interrupts the current command and hands the prompt back.
[c5](#c5-processes-and-killing-them) covers what to do when it does not work.

**Escape clears the line** you are typing without running it, which is the polite way out of
a command you decided against.

## Full

### The rest of the line-editing keys

Most of these are the same in PowerShell, bash and zsh. Two are not, and they are the two
you will reach for from a tutorial.

| Key | Does | Where |
|---|---|---|
| Home / End | Jump to the start or end of the line | All |
| Ctrl+Left / Ctrl+Right | Move one word at a time | All |
| Ctrl+W | Delete the word left of the cursor | All |
| Ctrl+L | Clear the screen, keeping what you have typed | All |
| Escape | Throw away the line you are typing | PowerShell |
| Ctrl+Home / Ctrl+End | Delete everything left or right of the cursor | PowerShell |
| Ctrl+U / Ctrl+K | Delete everything left or right of the cursor | Bash and Zsh |
| Alt+F7 | Clear this window's history | PowerShell |

`Ctrl+U` is the one people bring over from bash and find missing. In PowerShell the same job
is `Ctrl+Home`, or `Escape` if you want the whole line gone. `Ctrl+L` is worth the muscle
memory in any shell, because it beats typing `cls` and it does not lose your half-finished
command.

### Copy and paste, which is the one thing Windows does differently

The console does not use Ctrl+C for copy, because Ctrl+C already means "stop the running
command" and that meaning is older. What you get instead:

- **Copy.** Select text with the mouse, then press Enter, or right-click. In Windows
  Terminal, Ctrl+Shift+C also works and is the safe habit.
- **Paste.** Right-click, or Ctrl+V, or Ctrl+Shift+V. All three work in Windows Terminal.
- **Select a word** by double-clicking, **a line** by triple-clicking.

One warning. Pasting several lines at once runs them one after another, immediately, with no
chance to read them. When an agent hands you a block of five commands, paste them one at a
time. The cost is four extra keystrokes and the benefit is that you get to stop after the
one that failed.

Drag and drop also works: dragging a file from File Explorer into the console types its full
path, correctly quoted. That is the fastest way to get a long path onto the line without
spelling it.

### Where history is kept

Two different histories exist and people confuse them constantly.

**The session history** is the commands in this window. `Get-History` lists them numbered,
and it disappears when the window closes.

**The persistent history** is the file that survives a reboot, which is what Ctrl+R actually
searches. On PowerShell it lives here:

```powershell
(Get-PSReadLineOption).HistorySavePath
```

Prints the full path, which is normally under
`C:\Users\<yourname>\AppData\Roaming\Microsoft\Windows\PowerShell\PSReadLine`. It is a plain
text file. You can open it, read it, and edit it.

In bash and zsh the equivalents are `~/.bash_history` and `~/.zsh_history`, same idea, same
plain text.

That the file is plain text has one consequence worth acting on: **anything you type on a
command line is written down**. A password or a token passed as an argument is now sitting
in a file in your profile, in the clear, for as long as the history keeps it. See
[g6](#g6-secrets-and-what-never-to-commit) for what to do with secrets instead. If one has
already gone into a command line, clear that entry rather than hoping:

```powershell
Clear-History
```

Clears the session list. The persistent file needs editing or deleting separately, at the
path the command above printed.

### Making tab completion better

PowerShell's completion is good out of the box and better with two settings.

```powershell
Set-PSReadLineKeyHandler -Key Tab -Function MenuComplete
```

Changes Tab from "cycle through matches one at a time" to "show all the matches as a menu
and let me arrow around it". Faster to use and much easier to see what your options were.

```powershell
Set-PSReadLineOption -PredictionSource History
```

Turns on inline suggestions from your own history: as you type, the rest of a command you
have run before appears in gray, and the right arrow accepts it. It is the best
quality-of-life change available in the console.

Both of these last only for the current window. To keep them, put the two lines in your
PowerShell profile, which is the file that runs every time a window opens.
[b9](#b9-where-settings-live) covers where that lives.
