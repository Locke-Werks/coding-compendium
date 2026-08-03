---
id: "parameter"
title: "Parameter"
type: glossary
verified: "2026-08-02"
volatility: low
term: "Parameter"
aliases: ["parameters", "param", "params", "formal parameter"]
short_def: "The name a function gives one of its inputs, written where the function is defined. It is the slot, not the value."
not_to_be_confused_with: ["argument"]
canonical_section: "c1-what-a-program-is"
---

The parameter is the empty slot in the definition. The argument is what you drop into it at the call. Most people use the two words interchangeably and nothing bad happens, but error messages do not: a complaint about a missing argument means the call site, not the function.
