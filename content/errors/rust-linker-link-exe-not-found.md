---
id: rust-linker-link-exe-not-found
title: "error: linker `link.exe` not found"
type: error
verified: 2026-08-02
volatility: quarterly

language: rust
category: wont-compile

# Builds without running. Success ends with "Finished" and no error block.
verify: cargo build

sample: |
  PS C:\Users\you\dev\tool> cargo run
     Compiling tool v0.1.0 (C:\Users\you\dev\tool)
  error: linker `link.exe` not found
    |
    = note: program not found

  note: the msvc targets depend on the msvc linker but `link.exe` was not found

  note: please ensure that Visual Studio 2017 or later, or Build Tools for Visual Studio were installed with the Visual C++ option.

  note: VS Code is a different product, and is not sufficient.

  error: aborting due to 1 previous error

  error: could not compile `tool` (bin "tool") due to 1 previous error

patterns:
  - "linker `link.exe` not found"
  - "the msvc targets depend on the msvc linker"
  - "Build Tools for Visual Studio"
  - "program not found"

means: >
  Rust compiles your code into pieces and then needs a linker to join those pieces into a single
  program. On Windows, Rust uses Microsoft's linker, `link.exe`, which ships with Visual Studio
  rather than with Rust. It is not installed on this machine. Your code compiled. The last step
  had no tool to run.

fix_ladder:
  - try: Install the Microsoft build tools.
    command: winget install Microsoft.VisualStudio.2022.BuildTools --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
    shell: powershell
    why: >
      Assumes nothing suitable is installed, which is the case on a fresh Windows machine. The
      override text picks the one workload Rust needs and skips the rest of Visual Studio. It is
      a large download, around two gigabytes, and it needs no restart.

  - try: Close the terminal, open a new one, and build again.
    command: cargo build
    shell: powershell
    why: >
      Assumes the install finished and this window has stale PATH. The installer adds folders to
      PATH, and a terminal reads PATH once when it opens. This is the step people skip before
      concluding the install failed.

  - try: Confirm the linker is now findable.
    command: Get-Command link.exe
    shell: powershell
    why: >
      Assumes you want certainty before another long build. A path in the output means the tool
      is there. Nothing means the workload was not included, which happens when Visual Studio is
      installed without the C++ option ticked.

  - try: Add the C++ workload to an existing Visual Studio install.
    why: >
      Assumes Visual Studio is already on the machine without the right parts. Open Visual Studio
      Installer from the Start menu, click Modify, and tick "Desktop development with C++". This
      is much smaller than a fresh install because most of it is already there.

  - try: Switch Rust to the toolchain that brings its own linker.
    command: rustup default stable-x86_64-pc-windows-gnu
    shell: powershell
    why: >
      Assumes you cannot install the Microsoft tools, on a locked-down work machine for example.
      The GNU toolchain links without them. It is less well supported on Windows and some crates
      expect the Microsoft one, so treat it as a workaround rather than the default choice.

if_none_worked: >
  Paste every `note:` line from the error, the output of `rustup show`, and the output of
  `Get-Command link.exe`. The notes are what people cut because they read like boilerplate, and
  `rustup show` names which toolchain you are on, which decides whether a linker is even expected
  to be there.

see_also:
  - c2-compiled-vs-interpreted
  - c4-path-and-command-not-found
  - i3-builds-and-artifacts
  - rust

keywords:
  - link.exe not found
  - msvc linker
  - build tools for visual studio
  - rust windows setup
  - cargo build fails
---

Every new Rust developer on Windows meets this, usually within an hour of installing Rust.

The reason is a licensing and packaging one rather than a technical one. Rust cannot bundle
Microsoft's linker, so it uses the one on your machine, and a machine that has never built C or
C++ code does not have one.

Note the line in the error that says Visual Studio Code is a different product and is not
sufficient. The names are confusingly similar. Visual Studio Code is an editor. Visual Studio is
a much larger development suite, and Build Tools for Visual Studio is the command-line part of
it without the editor, which is all Rust needs.

Once installed it stays installed, and this never happens again on that machine. It also fixes
the same error for anything else that compiles native code, including some Python and Node
packages that build parts of themselves during install.
