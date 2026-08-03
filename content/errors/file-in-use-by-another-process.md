---
id: file-in-use-by-another-process
title: "The process cannot access the file because it is being used"
type: error
verified: 2026-08-02
volatility: low

category: permission

# Lists every process with that file open. Empty output means the lock is gone.
verify: Get-Process | Where-Object { $_.Modules.FileName -like "*<filename>*" }

sample: |
  The process cannot access the file 'C:\Users\nyx\dev\scraper\target\debug\scraper.exe' because it is being used by another process. (os error 32)
  error: could not compile `scraper` (bin "scraper") due to 1 previous error

patterns:
  - "cannot access the file because it is being used by another process"
  - "being used by another process"
  - "os error 32"

means: >
  Windows lets one program hold a file in a way that blocks everyone else from writing to
  it, moving it, or deleting it. Something is holding this one right now. The usual holder
  is the previous version of the program you are trying to rebuild, still running from the
  last time you started it. Nothing is corrupt and nothing needs reinstalling. A handle is
  open that should have closed.

fix_ladder:
  - try: Press Ctrl+C in whatever terminal is running the program, then build again.
    why: >
      Assumes the program you are rebuilding is still running in another window. This is
      the cause the overwhelming majority of the time. Ctrl+C tells a running program to
      stop, and the lock disappears with the process.

  - try: Kill the process by name.
    command: Stop-Process -Name <name> -Force
    shell: powershell
    why: >
      Assumes the program is running but did not respond to Ctrl+C, or it was started by
      something with no visible window. Use the executable name without the `.exe`. If it
      says no process was found, the holder is something else.

  - try: Find the actual holder instead of guessing.
    why: >
      Assumes the file is held by a program you would not have thought of. Press the
      Windows key, type `Resource Monitor`, open it, go to the CPU tab, and type the file
      name into the Associated Handles box. It names the process holding the file, which
      is often an editor, an antivirus scan, or a file indexer.

  - try: Close your editor and try once more.
    why: >
      Assumes the editor is the holder. Editors keep handles open on files in the folder
      they have open, and some extensions keep handles on build output. This is worth one
      attempt before anything heavier.

  - try: Rerun the same command with nothing changed.
    why: >
      Assumes an antivirus scan grabbed the file for a moment during the build. Real-time
      scanning opens a file the instant it is written. If the identical command fails at
      random and passes on a retry, that is the signature, and adding your project folder
      to the Windows Security exclusion list stops it.

if_none_worked: >
  Paste the full error including the `os error 32` part and the build line under it, the
  command you ran, and the output of the Resource Monitor search for that file name. The
  name of the holding process is the entire answer here, and it is the piece that never
  makes it into the paste.

see_also:
  - c5-processes-and-killing-them
  - i3-builds-and-artifacts
  - c6-ports-and-localhost

keywords:
  - file in use
  - os error 32
  - cannot delete exe
  - locked file
  - link.exe access denied
---

Windows locks files harder than the systems most build tools were written on. On Linux
you can delete a running program's file and the running copy carries on. On Windows the
lock is real and the build stops.

This is why the error shows up most on the second run. The first `cargo run` or
`npm run dev` starts the program. You edit code, run again, and the compiler tries to
overwrite an executable that is still running from the first time.

There is a version of this with no visible window at all. A dev server started by an agent
in a background terminal stays alive after the agent's turn ends. Nothing on screen shows
it. `Get-Process` and Resource Monitor find it, and Task Manager works too.
