---
id: "line-ending"
title: "Line ending"
type: glossary
verified: "2026-08-02"
volatility: low
term: "Line ending"
aliases: ["line endings", "CRLF", "LF", "newline", "EOL", "carriage return"]
short_def: "The invisible character that ends each line of a text file. Windows uses two, everything else uses one, and git can rewrite them in transit."
canonical_section: "c8-line-endings-and-encoding"
---

A diff that shows every line changed when you only edited one is nearly always this. Windows tools write a carriage return and a line feed. Linux and Mac tools write only the line feed. Git can convert on the way in and out, and a gitattributes file is where you tell it what you want.
