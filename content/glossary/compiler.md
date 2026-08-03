---
id: "compiler"
title: "Compiler"
type: glossary
verified: "2026-08-02"
volatility: low
term: "Compiler"
aliases: ["compilers", "compile", "compiled", "compiling", "compilation"]
short_def: "A program that translates your source code into machine instructions ahead of time, producing a file that runs without the compiler present."
not_to_be_confused_with: ["interpreter"]
canonical_section: "c2-compiled-vs-interpreted"
---

A compiler does the translation once, before anyone runs anything, and hands you a finished program. An interpreter does it every time, line by line, while the program runs. This is why a Rust build produces an exe file you can hand to someone, and a Python script needs Python installed on their machine first.
