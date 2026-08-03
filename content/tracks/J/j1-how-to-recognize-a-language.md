---
id: j1-how-to-recognize-a-language
title: How to tell what language you are looking at
type: section
track: J
order: 10
verified: 2026-08-02
volatility: low
answer: >
  Check five things in this order: the file extension, the manifest sitting in
  the folder, whether blocks use braces or indentation, the word that declares a
  function, and the word that imports. The first two settle most files alone.
owns:
  - the recognition method
  - the identification order
see_also:
  - j2-the-config-formats-nobody-explains
  - j3-project-layouts
  - j4-reading-a-repo-you-did-not-write
  - c2-compiled-vs-interpreted
  - a6-how-to-use-this-app
  - j7-the-ones-you-will-not-meet
keywords:
  - what language is this
  - identify code
  - unknown file type
  - which language
  - code recognition
  - what is this file
---

## More

Paste the snippet into this app's identify box and you get the answer in a second, with the
evidence it used. This section is the method behind that box, written out for the times the
app is not open: a code review in a browser, a screenshot somebody sent you, a file on a
machine that is not yours.

Five signals, ordered by how decisive each one is. Stop as soon as you have an answer.

**1. The filename extension.** `.rs` is [Rust](#rust). `.py` is [Python](#python). `.go` is
[Go](#go). This settles it more often than anything else and takes no thought at all. It
fails only when the file has no name, which is the case for pasted snippets.

**2. The manifest in the folder.** A manifest is the file that describes a project to its
package manager ([g2](#g2-package-managers)). `Cargo.toml` means the project is Rust, full
stop, no second opinion needed. `go.mod` means Go. `Gemfile` means [Ruby](#ruby).
`package.json` is the common one that does *not* settle it: it means
[JavaScript](#javascript) or [TypeScript](#typescript), and you need one more look to know
which.

**3. The block delimiters.** Look at how the language marks where a chunk of code begins and
ends. Curly braces put it in a large family that includes Rust, Go, [Java](#java),
[C#](#csharp), and JavaScript. Indentation alone means Python. A literal `end` keyword means
Ruby, [Lua](#lua), or [Elixir](#elixir).

**4. The function keyword.** The word in front of a function name is close to a fingerprint.
`fn` is Rust. `func` is Go or [Swift](#swift). `fun` is [Kotlin](#kotlin). `def` is Python or
Ruby. `function` is JavaScript, [PowerShell](#powershell), or
[PHP (PHP: Hypertext Preprocessor)](#php).

**5. The import syntax.** How the file pulls in other code. `use` is Rust. `import` is
Python, Java, Go, and JavaScript. `#include` is [C](#c) or [C++](#cpp). `using` is C#.
`require` is Ruby or older JavaScript.

Two signals agreeing is enough. Three is certainty.

## Full

### Step 1: extensions, including the ones that lie

| Extension | Language | Note |
|---|---|---|
| `.rs` | Rust | Never anything else |
| `.py` | Python | |
| `.go` | Go | |
| `.rb` | Ruby | |
| `.js` `.mjs` `.cjs` | JavaScript | |
| `.ts` | TypeScript | Also a video format, so a `.ts` file that will not open in an editor is not code |
| `.tsx` `.jsx` | TypeScript or JavaScript with markup inside | React components |
| `.java` | Java | |
| `.kt` | Kotlin | |
| `.cs` | C# | |
| `.php` | PHP | |
| `.swift` | Swift | |
| `.c` | C | |
| `.cpp` `.cc` `.cxx` | C++ | |
| `.h` | C or C++ | Genuinely ambiguous. Look for `class` or `::`, which mean C++ |
| `.m` | Objective-C or MATLAB (Matrix Laboratory) | Look for `#import` and square brackets, which mean Objective-C |
| `.ps1` | PowerShell | |
| `.sh` | Bash or another Unix shell | |
| `.html` | HTML (Hypertext Markup Language) | |
| `.sql` | SQL (Structured Query Language) | |
| `.md` | Markdown | Prose, not code |

The trap worth naming: a file called `script` with no extension at all is normal on Linux
and common in repositories. Open it and read the first line. A line beginning `#!` names the
program that runs it, so `#!/usr/bin/env python3` tells you the answer outright.

### Step 2: the manifest, and how decisive each one is

| File in the folder | Means | Decisive |
|---|---|---|
| `Cargo.toml` | Rust | Yes |
| `go.mod` | Go | Yes |
| `Gemfile` | Ruby | Yes |
| `composer.json` | PHP | Yes |
| `Package.swift` | Swift | Yes |
| `pyproject.toml`, `requirements.txt` | Python | Yes |
| `pom.xml`, `build.gradle` | Java or Kotlin | No, check the source extensions |
| `*.csproj`, `*.sln` | C# | Yes, in practice |
| `package.json` | JavaScript or TypeScript | No |

For the last row, the tiebreaker is one more file. A `tsconfig.json` beside it means
TypeScript. Files ending `.ts` in the source folder mean TypeScript. Neither present means
plain JavaScript.

Manifests are also how you find out what the project needs and how to run it, which is why
they are step two of walking into an unfamiliar repository as well
([j4](#j4-reading-a-repo-you-did-not-write)).

### Steps 3 to 5: identifying a snippet with nothing around it

This is the hard case. No filename, no folder, twelve lines in a chat window. Work down the
same list in order.

**Blocks.** Braces narrow you to a large family. Indentation with a colon at the end of the
opening line means Python and almost nothing else. A bare `end` on its own line means Ruby,
Lua, or Elixir.

**The declaration keywords.** Read the first word of any line that introduces something:

| You see | It is |
|---|---|
| `fn name(...)` | Rust |
| `func name(...)` | Go, or Swift if the file also has `let` and no `:=` |
| `fun name(...)` | Kotlin |
| `def name(...)` | Python if the line ends in a colon, Ruby if the block closes with `end` |
| `function name(...)` | JavaScript, PHP, or PowerShell |
| `public static void` | Java or C# |
| `sub name` | Perl |

**The import line.** Almost every file starts with them, so this is often the first thing on
screen:

| You see | It is |
|---|---|
| `use std::...` | Rust |
| `import ... from '...'` | JavaScript or TypeScript |
| `from x import y` | Python |
| `import "fmt"` | Go |
| `#include <...>` | C or C++ |
| `using System;` | C# |
| `require 'x'` | Ruby |
| `const x = require('x')` | Older JavaScript running under Node |

### The finishing touches when it is still close

- **Semicolons at the end of every line** point at C, C++, Java, C#, PHP, or Rust. Go and
  Python have none. JavaScript is inconsistent about them and that inconsistency is itself a
  tell.
- **`::` between names** means Rust or C++, and nothing else common.
- **A dollar sign in front of variables** means PHP, or PowerShell if the file also has
  hyphenated command names like `Get-ChildItem`.
- **Types written after the name**, as in `name: String`, means Rust, TypeScript, Kotlin, or
  Swift. Types written before the name, as in `String name`, means Java, C#, or C.

### Two languages on the screen at once

This catches people constantly. A Python file can hold a SQL query inside a string. A
JavaScript file can hold HTML inside backticks. A YAML (YAML Ain't Markup Language) pipeline
file holds shell commands inside `run:` blocks.

When something looks like two languages, it usually is. Identify the outer one from the file
extension and the inner one from the shape of the string, and remember that an error can
come from either. A broken query inside a working Python program produces a Python error
about a database, which sends people looking in the wrong file for an hour.

### What the app does that this list does not

The identify box scores every signal at once instead of stopping at the first answer, weighs
them against each other, and shows you which ones it used. That is worth doing when a
snippet is genuinely ambiguous, and it is faster than this page in every case where you have
the app open. The reason to know the manual method anyway is that recognition is a skill you
carry into the browser, the pull request, and the screenshot, and those are exactly the
places you cannot paste anything.
