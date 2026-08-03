---
id: npm-err-elifecycle
title: "npm ERR! code ELIFECYCLE"
type: error
verified: 2026-08-02
volatility: quarterly

language: javascript
category: broke-at-runtime

# Rerun the script that failed. A clean exit prints nothing extra and returns
# you to the prompt.
verify: npm run <script-name>

sample: |
  PS C:\Users\nyx\dev\site> npm run build

  > site@0.1.0 build
  > tsc -b && vite build

  src/App.tsx:12:7 - error TS2304: Cannot find name 'usState'.

  Found 1 error.

  npm error code ELIFECYCLE
  npm error errno 2
  npm error path C:\Users\nyx\dev\site
  npm error command failed
  npm error command C:\WINDOWS\system32\cmd.exe /d /s /c tsc -b && vite build

patterns:
  - "ELIFECYCLE"
  - "npm ERR! code ELIFECYCLE"
  - "npm error code ELIFECYCLE"
  - "Failed at the"
  - "Exit status"

means: >
  The script npm ran finished with a failure code, so npm reports that the script failed. That
  is all this message means. npm did not fail and there is nothing wrong with npm. Whatever it
  launched, a compiler, a test runner, a build tool, printed its own error and gave up, and
  that error is further up the screen.

fix_ladder:
  - try: Scroll up and read the output above the npm error block.
    why: >
      Assumes the real error was printed and you are reading the wrong end of it. The npm
      block is the last thing on screen, so it is what your eye lands on. The line that
      matters is usually a few lines above it and mentions a file in your own project. In the
      sample, that line is the TypeScript error about `usState`.

  - try: Run the failing command directly, without npm in the way.
    command: npx tsc -b
    shell: powershell
    why: >
      Assumes npm's wrapping is making the output hard to read. The `npm error command` line
      names exactly what was run. Running that on its own gives you the tool's output with no
      npm framing around it.

  - try: Check that the script exists and does what you think.
    command: npm run
    shell: powershell
    why: >
      Assumes the script name or its contents are the problem. With no arguments, `npm run`
      lists every script in `package.json` along with the command each one runs. A script that
      calls a tool you never installed fails this way immediately.

  - try: Look for a missing dependency in the underlying error.
    command: npm install
    shell: powershell
    why: >
      Assumes the script failed because a tool it needs is not on disk. A build script that
      calls `tsc` needs TypeScript installed in the project. The underlying error says
      something about a command not being recognized rather than about your code.

  - try: Read the log file npm wrote.
    command: Get-Content $env:LOCALAPPDATA\npm-cache\_logs\*-debug-0.log -Tail 40
    shell: powershell
    why: >
      Assumes the real error scrolled away or the terminal buffer lost it. npm writes the full
      run to a log file and names the path in its output on newer versions. The last forty
      lines are almost always enough.

if_none_worked: >
  Paste everything from the `> script-name` line at the top down to the last npm line, not the
  npm block on its own. The npm block says a script failed and nothing else. The output above
  it is the actual error, and it is the part people cut because the red text at the bottom
  looks more important.

see_also:
  - f1-how-to-read-an-error-message
  - f3-exit-codes-and-streams
  - c3-what-running-means
  - javascript

keywords:
  - ELIFECYCLE
  - npm run failed
  - exit status 1
  - npm error code
  - build script failed
---

This is the most misleading error in the Node world, and only because of where it sits on
the screen.

Every program returns a number when it exits. Zero means success and anything else means
failure. npm starts your script, sees a non-zero number come back, and reports it. The
number in `errno` is that exit code, and it carries no meaning beyond "not zero".

So the fix is a reading habit rather than a command. Start at the `> build` line where npm
announces what it is running, and read downward for the first thing that mentions a file in
your project. That is the error. Everything below it is npm's paperwork.

Newer npm prints `npm error` where older versions printed `npm ERR!`. Same thing, and both
appear in the wild depending on which version is installed.
