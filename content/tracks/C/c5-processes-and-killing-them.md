---
id: c5-processes-and-killing-them
title: Processes, and how to kill one that will not stop
type: section
track: C
order: 50
verified: 2026-08-02
volatility: low
verify: Get-Process node -ErrorAction SilentlyContinue
danger: >
  `Stop-Process` and `taskkill` end a program instantly and give it no chance to
  save, close a file, or finish a database write. Press Ctrl+C in the window that
  owns the process first, because that asks rather than tells. Never stop by name
  without listing the matches first: `Stop-Process -Name node` ends every process
  called node, including your editor's language server and anything an agent is
  running in the background.
answer: >
  A process is one running copy of a program with its own memory and its own
  number, so press Ctrl+C in the window that started it, and only look up its
  number and stop it by force when Ctrl+C gets no response.
owns:
  - processes
  - Ctrl+C
  - Task Manager
  - taskkill
see_also:
  - c6-ports-and-localhost
  - c1-what-a-program-is
  - c3-what-running-means
  - f4-logs
keywords:
  - how to kill a process
  - ctrl c not working
  - task manager
  - taskkill
  - stop-process
  - server still running
  - file is locked
  - it wont stop
---

## More

A **process** is one running copy of a program. The operating system gives it its own slice
of memory, its own working folder, its own copy of your environment settings, and a number
called the process id that everything else uses to refer to it.

The same program can be running four times at once, as four processes, each with a different
number and none of them aware of the others. That is why "Node is installed" and "a Node
process is running" are unrelated statements, and why you can start a dev server twice
without anything warning you until the port collides
([c6](#c6-ports-and-localhost)).

To stop one, in order of how gentle it is.

**Ctrl+C, in the window that started it.** This sends an interrupt, which is a request. The
program gets to close its files and shut down properly. It has to be the window that owns
the process; Ctrl+C in a different tab does nothing to it. Press it once and wait a couple of
seconds before pressing it again.

**Find it, then stop it by number.** When Ctrl+C gets no response, or the process was started
somewhere you cannot see:

```powershell
Get-Process node
```

Lists every running process called `node`, with its `Id` in the second column. Swap in
`python`, `cargo`, or whatever you are hunting.

```powershell
Stop-Process -Id 12345
```

Ends the one process with that id. Use the number from the previous command, never a guess.

That second command is where the care goes. It does not ask the program to stop, it removes
it, and anything the program was part way through writing stays part way through. For a dev
server that costs nothing. For anything holding a database or writing a file, it can cost
the file.

## Full

### What Ctrl+C is actually doing

It sends an interrupt signal to whatever is in the foreground of that terminal. Well-behaved
programs catch it, stop accepting new work, finish what they are doing, and exit. Some
ignore it deliberately. Some are stuck in a way that leaves nothing running to notice it.

Two habits worth having:

- Press it once and count to three. Mashing it can leave a program half shut down.
- If the prompt comes back but the program clearly is still going, the terminal handed you
  the prompt while a background child kept running. Look for it with `Get-Process`.

Closing the terminal window is a heavier version of the same idea, and it is not reliable
for anything started in the background. A dev server an agent launched detached will outlive
the window that launched it, which is the single most common source of a mystery process on
a vibe coder's machine.

### Finding the one you want

```powershell
Get-Process node | Select-Object Id, ProcessName, StartTime, Path
```

`Select-Object` picks the columns worth seeing. `StartTime` tells you which copy is the
leftover from an hour ago, and `Path` tells you which install of Node it is. If several rows
look identical, the oldest `StartTime` is almost always the orphan.

When you know the port instead of the name, go the other way and start from the port. That
is [c6](#c6-ports-and-localhost).

### Stopping it

```powershell
Stop-Process -Id 12345
```

The ordinary form. It asks Windows to end that one process.

```powershell
Stop-Process -Id 12345 -Force
```

`-Force` skips the confirmation prompt and does not take no for an answer. Reach for it only
after the plain form failed.

```powershell
taskkill /PID 12345 /T /F
```

The older Windows command, still useful for one reason: `/T` means "and every process this
one started". A dev server that spawned four workers goes away in one command. `/F` is
force. The Task Manager column labeled PID (Process Identifier) is the same number.

### The one to never type without looking first

```powershell
Stop-Process -Name node
```

That ends every process named `node` on the machine. On a developer's laptop that list
routinely includes your editor's language server, a documentation preview, a background task
an agent started, and the thing you actually wanted. Run `Get-Process node` first, read the
list, and stop the one id you meant.

### Task Manager, for when you would rather point at it

Ctrl+Shift+Esc opens it. Go to the Details tab, which lists real process names and their PID
values rather than the friendly grouping on the Processes tab. Right-click, End task.

It is the same operation as `Stop-Process -Force`, with the same cost. It is genuinely better
when you do not know the name of what you are looking for, because you can sort by memory or
by CPU and recognize the offender.

### The file that will not delete

`The process cannot access the file because it is being used by another process` is this
card wearing a different hat. Something is still running and still holds that file open. The
usual culprits are a dev server, a terminal sitting inside the folder you are trying to
delete, and Windows Explorer previewing the file. Stop the process and the file frees
immediately. See [file in use by another process](#file-in-use-by-another-process).

### What ending a process actually costs

- **Unsaved memory is gone.** Anything the program was holding and had not written is lost.
- **Half-written files stay half-written.** A build that was mid-write leaves a corrupt
  artifact, which is why the first move after killing a build is to delete the output folder
  and build again.
- **Locks can survive briefly.** A database or a port can stay held for a few seconds while
  Windows cleans up. Waiting ten seconds beats running the command twice.

Nothing here touches your source code, and nothing here touches git. A killed process cannot
lose a commit. It can lose the last edit you made in a program that had not saved it yet.

### Processes an agent left behind

Agents start servers, test watchers, and build tools in the background, and they are not
consistent about stopping them when a turn ends. When something behaves as though an old
version of your code is still running, this is usually why: an older process is still up and
still serving. Check for it before debugging the code, because you will otherwise spend an
hour fixing something that is already fixed.

```powershell
Get-Process node, python, cargo -ErrorAction SilentlyContinue | Select-Object Id, ProcessName, StartTime
```

`-ErrorAction SilentlyContinue` stops it complaining about the names that are not running.
