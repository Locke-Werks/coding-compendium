---
id: assembly
title: Assembly
type: language
verified: 2026-08-02
volatility: low

name: Assembly
aka: [asm, x86, x86-64, nasm, masm, gas, disassembly]
family: compiled
likelihood: unlikely
extensions: ['.s', '.S', '.asm', '.nasm', '.inc']

tells:
  - pattern: '\bmov\b'
    kind: regex
    weight: 8
    note: >
      `mov` copies a value into a register. No high-level language in this deck has
      an instruction called `mov`, and it appears more often than anything else in a
      real assembly file.
  - pattern: '%r[a-z]{2}'
    kind: regex
    weight: 10
    note: >
      A percent sign glued to a register name, as in `%rax`, is the assembler
      dialect the Linux toolchain uses by default. Nothing else writes `%rax`.
  - pattern: '\b(rax|rsp|rbp|rdi|rsi)\b'
    kind: regex
    weight: 9
    note: >
      Register names are fixed hardware slots, not variables. They are never
      declared anywhere, which is what makes them look wrong to anyone reading their
      first assembly file.
  - pattern: '^\s*section\s+\.'
    kind: regex
    weight: 9
    note: >
      `section .text` for code and `section .data` for constants split the file into
      parts the loader understands. No other language in this deck has sections.
  - pattern: '\b(syscall|int 0x80)\b'
    kind: regex
    weight: 8
    note: >
      The instruction that asks the operating system to do something. In C you would
      call `write` and let the library set this up; in assembly you fill the
      registers and issue the call yourself.

rules_out:
  - pattern: 'return'
    because: >
      Almost anything else. Assembly spells it `ret`, so the word `return` points at
      C, C++, Rust, Go, Java, Python, or JavaScript.
  - pattern: '{'
    kind: sigil
    because: C, C++, Rust, Go, Java, C#, or JavaScript. Assembly groups nothing with braces.
  - pattern: 'def'
    because: Python or Ruby

project_fingerprint:
  manifests:
    - file: Makefile
      decisive: false
      note: >
        Nearly every assembly project builds with a plain Makefile. It settles
        nothing on its own. Read the tool it calls: `nasm`, `as`, or `ml64` means
        assembly.
    - file: '*.ld'
      decisive: false
      note: >
        A linker script, saying where in memory each section lands. Bare-metal,
        kernel, and bootloader projects have one, and those are nearly always
        assembly plus a little C.
    - file: '*.asm'
      decisive: true
      note: >
        There is no manifest format for assembly, so the source extension is what
        you have. `.asm`, `.s`, and `.S` are all assembly, and a folder full of them
        settles it.
  build_dirs: [build/, obj/, bin/]
  entry_points: [boot.s, start.S, main.asm]

shape:
  blocks: none
  statement_end: newline
  comment_line: '; in NASM and MASM, # or // in the GNU assembler'
  string_quotes: Double or single quotes, both meaning a run of bytes rather than a text type.
  naming: lower-case instruction names, labels ending in a colon, ALL_CAPS for constants by convention
  import_keyword: '%include in NASM, .include in the GNU assembler'

tooling:
  package_manager: none
  registry: none
  runtime: none. It becomes the exact bytes the processor runs.
  install_command: none. The assembler ships with the compiler toolchain you already installed.
  run_command: nasm -f win64 main.asm, then link the resulting object file
  test_command: none

confusable_with:
  - language: c
    settle_it: >
      The two live in the same folders, so this comes up. C has curly braces,
      `#include`, and a semicolon ending every statement. Assembly has one
      instruction per line, labels ending in a colon, and `;` starting a comment
      rather than ending a line.
    tiebreak: { pattern: '{', kind: sigil, favors: c }
  - language: makefile
    settle_it: >
      Both are plain text with bare words ending in a colon. What follows decides it:
      a Makefile follows with tab-indented shell commands like `gcc -c foo.c`, and
      assembly follows with one-word instructions like `mov` and `jmp`. A Makefile
      also uses `$(VAR)`, which assembly never does.
    tiebreak: { pattern: '\$\(', kind: regex, favors: makefile }

errors_look_like:
  sample: |
    main.asm:5: error: invalid combination of opcode and operands
    main.s:12: Error: no such instruction: `mov rax, 1'
    /usr/bin/ld: warning: cannot find entry symbol _start; defaulting to 0000000000401000
  recognize_by: >
    A `file:line:` prefix with no column number, then `Error:` or `error:` and a
    complaint about an instruction or an operand. C and C++ always print a column as
    well. At runtime there is no message at all: the process dies where it stands,
    because nothing is left underneath to catch anything.
  patterns:
    - '\.(s|S|asm):\d+: (E|e)rror:'
    - 'no such instruction'
    - 'invalid combination of opcode and operands'
    - 'cannot find entry symbol'

meet_it_when: >
  You will not write it. You see it in a crash dump with no symbols attached, in a
  disassembler, in compiler output when someone is showing what a line of C became, or
  in the first few files of an embedded or operating system project.

what_agents_get_wrong: >
  Agents get calling conventions wrong and the result assembles cleanly, which is the
  worst combination available. Windows on 64-bit passes the first four arguments in
  `rcx`, `rdx`, `r8`, and `r9`. Linux passes them in `rdi`, `rsi`, `rdx`, `rcx`, `r8`,
  and `r9`. Most assembly written on the internet is the Linux form, so an agent will
  hand you Linux register usage for a Windows target and the program will read
  whatever happened to be sitting in those registers. Second thing to check: Windows
  wants 32 bytes of shadow space on the stack before a call, and the stack has to be
  16-byte aligned at the call. Agents skip both and the crash lands somewhere else
  entirely. Third: `rbx`, `rbp`, and `r12` through `r15` belong to the caller and have
  to be put back before returning. An agent that uses one without saving it corrupts
  code it never touched.

version_landscape: >
  The instruction set grows and almost never drops anything, so old assembly keeps
  working. What shifts underneath an answer is the calling convention, which answers
  rarely name. A snippet that works on Linux can fail on Windows with every
  instruction correct.

see_also:
  - c
  - makefile
  - j1-how-to-recognize-a-language

keywords: [asm, nasm, masm, register, opcode, disassembly, calling convention, stack, x86]
---

The instructions the processor actually runs, written one per line. Every other
language in this deck turns into this before anything happens.

One operation per line: an instruction name first, then whatever it acts on. No braces,
no semicolons at the ends of lines, no functions in the usual sense. A label is a bare
word ending in a colon and marks a place the code can jump to.

```asm
section .text
global _start
_start:
    mov rax, 60      ; 60 is the exit call on Linux
    mov rdi, 0       ; exit code zero
    syscall
```

`;` starts a comment here, the reverse of every curly-brace language, where `;` ends a
statement. `rax` and `rdi` are registers, fixed slots inside the processor, and there
are only about sixteen of them.

Two dialects exist for one processor. Intel style writes `mov rax, 60`, destination
first. The `as` assembler that ships with `gcc` writes `movq $60, %rax`, destination
last, with a `%` on every register. Same instruction, opposite order, which has cost
the world a great deal of time.
