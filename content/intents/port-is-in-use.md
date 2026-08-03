---
id: port-is-in-use
title: "The port is already in use"
type: intent
verified: 2026-08-02
volatility: low

goal: "It says the port is already in use and will not start."
target: c6-ports-and-localhost
urgency: panic

phrasings:
  - "address already in use"
  - "port 3000 is busy"
  - "eaddrinuse"
  - "something is already running on that port"
  - "kill whatever is on the port"
  - "my dev server wont start"
  - "port already taken"
  - "listen tcp bind failed"
---
