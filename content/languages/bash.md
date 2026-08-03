---
id: bash
title: Bash
type: language
verified: 2026-08-02
volatility: low

name: Bash
aka: [sh, shell, shell script, bourne again shell, git bash, zsh]
family: shell
likelihood: certain
extensions: ['.sh', '.bash', '.zsh']
verify: bash --version

danger: >
  `rm -rf <path>` deletes a folder and everything under it immediately. There is
  no Recycle Bin, no confirmation, and no undo. When the path is built from a
  variable that turned out to be empty, the command deletes far more than you
  meant, which is the single most famous way to destroy a machine. Before you run
  it, run `ls <path>` with the same path to see exactly what is in there, and
  never let a variable sit next to a slash without quotes around it.

# Every note contrasts against PowerShell, because that is the prompt she is
# standing at when she pastes one of these. A tell with no contrast is trivia.
tells:
  - pattern: '#!/bin/bash'
    kind: line_start
    weight: 10
    note: >
      The first line of a shell script names the program that should run it. Read
      the word after the last slash: `bash`, `sh`, `zsh`, or `python3`. PowerShell
      scripts have no shebang line at all.
  - pattern: 'fi'
    kind: token
    weight: 9
    note: >
      An `if` block closes with `fi` and a `case` block closes with `esac`, the
      opening keyword spelled backwards. PowerShell and every C-family language
      close with a brace, and Python closes with nothing.
  - pattern: '2>/dev/null'
    kind: operator
    weight: 9
    note: >
      Throwing away error output. PowerShell writes `2>$null` with no slashes,
      because Windows has no `/dev` folder. Seeing this in a command someone gave
      you means it was written for Linux or Mac.
  - pattern: 'export'
    kind: line_start
    weight: 8
    note: >
      Sets an environment variable for this shell and everything it starts:
      `export API_KEY=abc`. PowerShell writes `$env:API_KEY = 'abc'`. JavaScript
      also has `export`, but it is followed by `function`, `const`, or `default`.
  - pattern: '\[\[ '
    kind: regex
    weight: 8
    note: >
      Double square brackets open a test: `if [[ -f config.json ]]`. PowerShell
      writes `if (Test-Path config.json)` with parentheses. The spaces just inside
      the brackets are mandatory, which is a common paste error.
  - pattern: '\$\{?[A-Z_][A-Z0-9_]*\}?'
    kind: regex
    weight: 6
    note: >
      Variables are read with a bare `$` and are conventionally SCREAMING_SNAKE:
      `$HOME`, `${BUILD_DIR}`. PowerShell writes `$Name` for a local and
      `$env:NAME` for an environment variable. PHP uses `$` too, but ends every
      statement with a semicolon.
  - pattern: 'elif'
    kind: token
    weight: 5
    note: >
      The else-if keyword. PowerShell writes `elseif`, JavaScript and C# write
      `else if`. Python also uses `elif`, so check for a colon at the end of the
      line: Python has one, bash does not.
  - pattern: '$('
    kind: operator
    weight: 4
    note: >
      Command substitution: `$(date)` runs `date` and drops its output in place.
      PowerShell uses `$( )` for expressions inside strings, so this one is
      corroborating rather than decisive.

rules_out:
  - pattern: '$env:'
    kind: sigil
    because: >
      PowerShell. Bash reads an environment variable as plain `$NAME`.
  - pattern: '\b(Get|Set|New|Remove)-[A-Z]'
    kind: regex
    because: >
      PowerShell. Verb-Noun command names exist in no Unix shell.
  - pattern: '@echo off'
    kind: regex
    because: >
      Batch. The file is a `.bat` or `.cmd`, the old Windows shell.
  - pattern: '%\w+%'
    kind: regex
    because: >
      Batch wraps variables in percent signs. In bash a bare `%` is job control
      or a modulo.
  - pattern: 'CategoryInfo'
    kind: token
    because: >
      PowerShell error output. Bash errors are a single lowercase line.

project_fingerprint:
  manifests:
    - file: '*.sh'
      decisive: true
      note: >
        A shell script. On Windows nothing runs it by double-click. You run it
        from Git Bash with `bash name.sh`.
    - file: 'install.sh'
      note: >
        The conventional name for a one-command installer, usually the thing on
        the other end of a `curl ... | bash` line. Read it before you run it,
        because piping a script from the internet straight into a shell runs
        whatever is on that server today.
    - file: 'entrypoint.sh'
      note: >
        The script a Docker container runs on start. Its presence means the
        project ships as a container.
    - file: '.bashrc'
      note: >
        The startup script in your home folder. Git Bash reads it at every new
        window, so it is where `PATH` edits and aliases hide.
    - file: 'scripts/'
      note: >
        A folder of `.sh` files is the usual home for a project's build, deploy,
        and setup helpers.
  entry_points: ['install.sh', 'entrypoint.sh', 'scripts/build.sh']

shape:
  blocks: keyword
  statement_end: newline
  comment_line: '#'
  comment_block: >
    None. Every commented line needs its own `#`.
  string_quotes: >
    Double quotes expand `$variables` inside them, single quotes are literal, and
    no quotes at all is a third behavior that splits the value on spaces. That
    third one is where most bash bugs come from.
  naming: lower_snake_case for functions and local variables, SCREAMING_SNAKE for environment variables
  import_keyword: source

tooling:
  package_manager: >
    None for the language. The shell is where you call apt, brew, npm, and winget.
  runtime: >
    bash itself. On Windows you get it as Git Bash, installed with Git for
    Windows, or inside WSL (Windows Subsystem for Linux).
  run_command: bash script.sh
  test_command: >
    bats for tests, shellcheck for static checking. Most projects run neither.

confusable_with:
  - language: powershell
    settle_it: >
      Read the variables. `$PATH` and `export` mean bash. `$env:PATH` and a
      `Verb-Noun` command mean PowerShell. A trailing backslash continuing a long
      line is bash; a trailing backtick is PowerShell.
    tiebreak: { pattern: '2>/dev/null', kind: operator, favors: bash }
  - language: batch-script
    settle_it: >
      Both are shell scripts and both look terse. Batch uses `%VAR%`, `REM` for
      comments, and usually opens with `@echo off`. Bash uses `$VAR`, `#` for
      comments, and opens with `#!/bin/bash`.
    tiebreak: { pattern: 'REM ', kind: line_start, favors: batch-script }
  - language: dockerfile
    settle_it: >
      A Dockerfile is full of bash, but every line starts with an uppercase
      instruction: `FROM`, `RUN`, `COPY`, `CMD`. The bash is the part after `RUN`.
      A `.sh` file has no uppercase instruction column.
    tiebreak: { pattern: '^FROM ', kind: line_start, favors: dockerfile }
  - language: makefile
    settle_it: >
      A Makefile is also mostly shell commands, wrapped in `target:` lines with
      tab-indented recipes underneath and `$(VAR)` for its own variables. A `.sh`
      file has no target lines and writes `$VAR` or `${VAR}`, never `$(VAR)`.
    tiebreak: { pattern: '^\w+:\s*$', kind: regex, favors: makefile }
  - language: gitignore
    settle_it: >
      Both contain bare words and `*` wildcards. A shell script runs commands, so its
      lines start with a verb such as `rm` or `echo`, and it opens with `#!/bin/bash`.
      A gitignore line is a noun, and a leading `!` on one is gitignore alone.
    tiebreak: { pattern: '^!\S', kind: regex, favors: gitignore }

errors_look_like:
  sample: |
    ./deploy.sh: line 12: cd: /opt/app: No such file or directory
    bash: jq: command not found
    ./deploy.sh: line 30: syntax error near unexpected token `fi'
  recognize_by: >
    One line. Lowercase, no color, no error code, in the shape
    `who: where: what`. The script name and line number sit at the front. There is
    no stack trace and nothing underlined. If you are looking at five red lines
    with `+ CategoryInfo` in them, that is PowerShell, not bash.
  patterns:
    - '^bash: .*: command not found'
    - '^[^:]*\.sh: line \d+:'
    - 'No such file or directory$'
    - 'syntax error near unexpected token'
    - 'Permission denied$'

meet_it_when: >
  Constantly, and mostly secondhand. Every install page on the internet gives you
  a bash command. It runs the `RUN` lines in a Dockerfile, the `run:` steps in a
  GitHub Actions workflow, and every `.sh` file in a repo you clone. Claude Code
  prefers Git Bash as its own shell, so the commands it runs for you are bash even
  when you typed the request in PowerShell.

what_agents_get_wrong: >
  Two problems, and the first one hits you every day. Agents hand you bash for a
  PowerShell prompt, because almost all of their training material is Linux and
  Mac. Watch for `export`, `rm -rf`, `2>/dev/null`, `sudo`, and `~/.config` paths
  arriving in an answer you are meant to paste into Windows Terminal. The second
  is inside the bash itself: unquoted variables. `rm -rf $BUILD_DIR/` is a
  catastrophe when `BUILD_DIR` is empty, and `cd $HOME/My Documents` splits at the
  space and fails. The fix is always double quotes: `"$BUILD_DIR"`. Agents also
  skip `set -euo pipefail`, so a script whose third command failed keeps running
  and reports success at the end. In a diff, look for any `$VARIABLE` that is not
  wrapped in double quotes, and any deletion whose path is assembled from
  variables.

version_landscape: >
  Bash 5 is current and is what Git Bash on Windows gives you. Mac ships bash 3.2
  from 2007 for license reasons, so an answer using `declare -A` or `${var,,}`
  fails there. The sharper trap is that `sh` is not bash: on Debian and Ubuntu
  `/bin/sh` is a smaller shell called dash that does not understand `[[ ]]`, so a
  script that works when run with `bash` breaks when run with `sh`. Answers using
  `sed` and `date` flags are usually written for Linux and behave differently on
  Mac.

see_also:
  - powershell
  - batch-script
  - dockerfile
  - makefile
  - b1-terminal-shell-command-line
  - g5-environment-variables
  - f3-exit-codes-and-streams

keywords: [sh, shell script, git bash, wsl, shebang, dotfiles, terminal]
---

The shell used on Linux and Mac: the program that reads what you type and runs it.
You are on Windows, so you do not live in bash. You read it constantly anyway,
because nearly every instruction on the internet is written for it.

On Windows you get bash as **Git Bash**, installed alongside Git for Windows. It is
a separate window with its own prompt and its own rules, and pasting its commands
into PowerShell is one of the most common ways to lose an afternoon.

## The shape

Comments are `#`. Variables are read with a bare `$` and set with no spaces around
the `=`, which is not a style choice: `NAME = value` is a syntax error. Blocks open
and close with keywords rather than braces, and the closing keyword is often the
opening one backwards.

```bash
#!/bin/bash
set -euo pipefail                 # stop on the first error, not the last

NAME="nyx"                        # no spaces around =
GREETING="hello ${NAME}"          # double quotes expand, single quotes do not

if [[ -f config.json ]]; then     # spaces inside the brackets are required
  echo "$GREETING"
fi
```

`set -euo pipefail` on line two is the mark of a script someone thought about. It
means stop at the first failing command, treat an unset variable as an error, and
do not let a failure in the middle of a pipe get swallowed. Without it, a script
runs happily past its own broken steps.

## Translating a bash command to PowerShell

This table is the practical payoff of the whole card. When an answer online gives
you the left column and your prompt starts with `PS`, you want the right one.

| You want to | bash | PowerShell |
|---|---|---|
| List files | `ls -la` | `Get-ChildItem`, also aliased to `ls` and `dir` |
| Read an environment variable | `$PATH` | `$env:PATH` |
| Set one for this session | `export KEY=abc` | `$env:KEY = 'abc'` |
| Delete a folder and its contents | `rm -rf build` | `Remove-Item build -Recurse -Force` |
| Copy a file | `cp a.txt b.txt` | `Copy-Item a.txt b.txt` |
| Throw away error output | `2>/dev/null` | `2>$null` |
| Continue a long line | trailing `\` | trailing backtick |
| Compare two numbers | `[ "$a" -eq "$b" ]` | `$a -eq $b` |
| Search inside files | `grep pattern *.md` | `Select-String pattern *.md` |
| Show a value | `echo "$NAME"` | `Write-Output $Name` |

Two of these are worth knowing by heart, because they are the ones that fail
quietly rather than loudly: `$VAR` against `$env:VAR`, and the escape character,
which is a backslash in bash and a backtick in PowerShell. See
[PowerShell](#powershell) for why the two shells differ this much underneath.

## What it is for

Gluing programs together. Bash is not where you build an application; it is where
you install one, start one, or run five commands in a row on a server. That is why
it turns up in installers, Docker images, and continuous integration steps rather
than in a project's source folder.

## Reading its errors

```text
./deploy.sh: line 12: cd: /opt/app: No such file or directory
bash: jq: command not found
```

One line, lowercase, no color. Read it from the right: the last piece is what went
wrong, and everything to the left of it narrows down where. `command not found`
means the program is missing or not on your `PATH`. See
[C4](#c4-path-and-command-not-found).

A bash script does not stop when a command fails unless it was told to, so the real
error is usually higher up your terminal than the last thing on screen. Scroll up.
