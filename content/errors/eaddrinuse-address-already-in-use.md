---
id: eaddrinuse-address-already-in-use
title: "EADDRINUSE: address already in use"
type: error
verified: 2026-08-02
volatility: low

language: javascript
category: conflict

# Change the number to your port. No output means the port is free.
verify: Get-NetTCPConnection -LocalPort 3000 -ErrorAction SilentlyContinue

sample: |
  PS C:\Users\nyx\dev\site> npm run dev

  > site@0.1.0 dev
  > node server.js

  node:events:496
        throw er;
        ^

  Error: listen EADDRINUSE: address already in use :::3000
      at Server.setupListenHandle [as _listen2] (node:net:1898:16)
      at listenInCluster (node:net:1946:12) {
    code: 'EADDRINUSE',
    errno: -4091,
    syscall: 'listen',
    address: '::',
    port: 3000
  }

patterns:
  - "EADDRINUSE"
  - "address already in use"
  - "listen EADDRINUSE"

means: >
  A port is a numbered door on your machine, and only one program can hold a given number at
  a time. Something already holds port 3000, so your server cannot open it and exits
  immediately. The holder is almost always the previous run of this same server, still alive
  in another terminal or left behind by a session that ended without stopping it.

fix_ladder:
  - try: Look for another terminal already running the server, and press Ctrl+C in it.
    why: >
      Assumes you started it twice. Editors, agents, and split terminal panes make this easy
      to do without noticing. Ctrl+C in the window that owns the server releases the port
      immediately and costs nothing.

  - try: Find out which program holds the port.
    command: Get-Process -Id (Get-NetTCPConnection -LocalPort 3000).OwningProcess
    shell: powershell
    why: >
      Assumes the holder has no visible window. This prints the process name and its id. If
      the name is `node`, it is a leftover server. If it is something else entirely, changing
      your own port is the better move.

  - try: Stop the process holding it.
    command: Stop-Process -Id <id> -Force
    shell: powershell
    why: >
      Assumes the previous step named a leftover of your own. Use the id from that output.
      This ends the program without warning, so check the name first rather than stopping
      whatever the command returns.

  - try: Run on a different port instead.
    command: $env:PORT = 3001; npm run dev
    shell: powershell
    why: >
      Assumes the port is genuinely spoken for and you want to get on with your work. Most
      dev servers read a `PORT` environment variable. Vite uses `--port 3001` on the command
      line instead, and Next.js accepts both.

  - try: Check whether Windows has reserved the port range.
    command: netsh int ipv4 show excludedportrange protocol=tcp
    shell: powershell
    why: >
      Assumes nothing is running and the port still will not open. Hyper-V and Windows
      Subsystem for Linux reserve blocks of ports at startup, and anything inside those
      ranges is unusable by normal programs until a restart. If your port falls inside a
      listed range, pick a different one.

if_none_worked: >
  Paste the whole error including the `port:` and `address:` lines, the command you ran, and
  the output of `Get-NetTCPConnection -LocalPort <port>`. The port number is obvious to you
  and invisible to an agent reading a trimmed message, and the connection listing names the
  process id that actually matters.

see_also:
  - c6-ports-and-localhost
  - c5-processes-and-killing-them
  - javascript

keywords:
  - EADDRINUSE
  - port already in use
  - port 3000 taken
  - address already in use
  - kill dev server
---

The number in the error is the port. `:::3000` is a way of writing "port 3000 on every
network interface", and the three colons are an empty address rather than a typo.

Agent-run dev servers are the modern cause. An agent starts one in a background terminal to
test something, the turn ends, and the process keeps running with nothing on screen to show
for it. The next run collides with a server you did not know existed.

There is one case with no visible holder at all. A server that crashed can leave the port in
a waiting state for a couple of minutes while Windows finishes closing the connection.
Waiting is the fix. If the port frees up on its own after a short pause, that was it.
