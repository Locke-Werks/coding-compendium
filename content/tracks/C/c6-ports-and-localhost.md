---
id: c6-ports-and-localhost
title: Ports, localhost, and "address already in use"
type: section
track: C
order: 60
verified: 2026-08-02
volatility: low
verify: Get-NetTCPConnection -LocalPort 3000 -ErrorAction SilentlyContinue
danger: >
  `Stop-Process` ends a program instantly with no chance to save or clean up.
  Look at the process name before you stop it, because the id you get back from a
  port lookup is whatever holds the port, which is not always the server you were
  thinking of. The safe alternative is to press Ctrl+C in the terminal that owns
  the server, or to start your own server on a different port and leave the
  other one alone.
answer: >
  A port is a numbered door on your own machine and only one program can hold a
  given number at a time, so `EADDRINUSE` means something already holds yours,
  and it is almost always the same dev server you started earlier and forgot.
owns:
  - ports
  - localhost
  - 127.0.0.1
  - EADDRINUSE
see_also:
  - c5-processes-and-killing-them
  - j6-web-basics
  - c3-what-running-means
  - eaddrinuse-address-already-in-use
keywords:
  - localhost
  - port 3000
  - address already in use
  - EADDRINUSE
  - what is a port
  - 127.0.0.1
  - cant reach this page
  - change port
---

## More

**localhost** means this machine, right here. When a dev server prints
`http://localhost:3000`, it is telling you the site is being served by a program on your own
computer. Nothing is published, nobody else can reach it, and it works with the internet
unplugged. `127.0.0.1` is the numeric form of the same thing and the two are interchangeable.

A **port** is a numbered door on that machine, from 1 to 65535. One machine runs many
programs that all listen for connections, so each one takes a different number and the number
is how traffic knows where to go. `3000` in the address above is the port.

The rule that generates all the trouble: only one program can hold a given port at a time.
Start a second thing on 3000 and it fails immediately with `EADDRINUSE`, which is how Node
spells "address already in use".

Find what is holding it:

```powershell
Get-Process -Id (Get-NetTCPConnection -LocalPort 3000).OwningProcess
```

`Get-NetTCPConnection` looks up what is listening on that port, `.OwningProcess` is the
process id it returns, and `Get-Process` turns that number into a name you can recognize.
Change `3000` to your port. If it prints `node`, that is a leftover dev server of yours. If
it prints something you do not recognize, leave it alone and use a different port.

Stop it, using the id from that output:

```powershell
Stop-Process -Id 12345
```

Before that, look for the terminal window that is already running the server and press
Ctrl+C in it. That is the same fix at no cost, and it works more often than you would think,
because the usual cause is a window you have behind something else.

## Full

### The address, piece by piece

```text
http://localhost:3000/api/users
\__/   \_______/ \__/ \_______/
  |        |       |      |
scheme    host    port   path
```

The **scheme** says which protocol to speak. The **host** says which machine. The **port**
says which program on that machine. The **path** says which part of that program. Leave the
port out and the browser assumes 80 for `http` and 443 for `https`, which is why ordinary
websites do not show one. [j6](#j6-web-basics) covers the protocol side.

One Windows-specific annoyance: typing `localhost:3000` into the address bar often runs a
web search instead, because the browser reads it as a word rather than an address. Type
`http://localhost:3000` and it works.

### The ports you will actually meet

| Port | Usually |
|---|---|
| 3000 | Node, Next.js, Express, Create React App |
| 5173 | Vite |
| 8080 | A generic second choice when 3000 is taken |
| 8000 | Python, Django, `python -m http.server` |
| 5000 | Flask, and on some machines a Windows service |
| 5432 | PostgreSQL |
| 27017 | MongoDB |

Nothing enforces any of this. They are conventions, and a project can use any number it
likes.

### Looking at what is listening

```powershell
Get-NetTCPConnection -LocalPort 3000 -State Listen
```

TCP (Transmission Control Protocol) is the connection type nearly everything you will meet
uses. `-State Listen` filters out finished connections that are still being tidied up, which
otherwise clutter the output and make an idle port look busy.

```powershell
Get-NetTCPConnection -State Listen | Select-Object LocalPort, OwningProcess | Sort-Object LocalPort
```

Everything currently listening on the machine, in port order. Useful when you know a server
is up and cannot remember which number it chose.

### Just change your port instead

Often the faster answer. Most dev servers read a `PORT` environment variable:

```powershell
$env:PORT = 3001; npm run dev
```

`$env:PORT` sets the variable for this terminal only, and it is gone when you close the
window ([g5](#g5-environment-variables)). Vite wants a command-line flag instead,
`npm run dev -- --port 3001`, where the bare `--` separates arguments meant for npm from
arguments meant for the tool it runs.

### localhost, 127.0.0.1, 0.0.0.0, and the three colons

- `localhost` is a name that resolves to your own machine.
- `127.0.0.1` is the numeric IP (Internet Protocol) address it resolves to.
- `::1` is the same idea in the newer address format, which is why a Node error sometimes
  says `:::3000`. Those three colons are an empty address, not a typo.
- `0.0.0.0` means "listen on every network connection this machine has", which is what you
  set when you want your phone on the same wifi to reach the dev server. It also means
  Windows Firewall will ask permission the first time.

### When the browser cannot connect

`ERR_CONNECTION_REFUSED` or "This site can't be reached" on a localhost address means
nothing is listening on that port. In order of likelihood: the server is not running, the
server crashed after printing its address, you typed a different port than it chose, or it
is listening on a different address than you asked for. Look at the terminal where you
started it, because whatever went wrong is printed there.

### The port that frees itself if you wait

A server that crashed rather than exiting cleanly can leave its port unusable for a minute or
two while Windows finishes closing the connections. There is no process to find and nothing
to stop. Waiting is the whole fix, and if the port comes back on its own, that was it.

### The port that will never work

Hyper-V, Docker, and Windows Subsystem for Linux reserve blocks of ports at startup, and
nothing else can use anything inside those blocks until a restart. The symptom is
`EADDRINUSE` with no process holding the port at all.

```powershell
netsh int ipv4 show excludedportrange protocol=tcp
```

Prints the reserved ranges. If your port sits inside one of them, pick a different number.
This is worth knowing purely because the alternative is an hour spent hunting a process that
does not exist.

### The reason this happens so often to you specifically

Agents leave dev servers running in the background after a turn ends
([c5](#c5-processes-and-killing-them)), so your next run collides with a server you never
knew existed. The [EADDRINUSE error card](#eaddrinuse-address-already-in-use) has the full
fix ladder.
