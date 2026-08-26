---
id: k7-same-word-different-program
title: When the same word runs a different program
type: section
track: K
order: 70
verified: 2026-08-25
volatility: low
verify: Get-Command curl
answer: >
  A dozen common command names exist in more than one shell and do different things in each,
  so a copied command can fail with a strange complaint about a parameter rather than a clean
  "not found".
owns:
  - alias traps
  - the exe suffix
  - why a copied command behaves differently
see_also:
  - k4-finding-things
  - k1-four-shells-one-console
  - c4-path-and-command-not-found
  - k2-anatomy-of-a-command
keywords:
  - curl not working powershell
  - ls -la does not work
  - alias
  - powershell alias
  - command behaves differently
  - where vs where.exe
  - echo powershell
  - parameter cannot be found
---

## More

There is a failure that reads like the instructions were wrong and is something else
entirely. You paste a command that thousands of people use, and instead of a clean error you
get a complaint about a parameter you did not know you had used.

```text
Get-ChildItem: A parameter cannot be found that matches parameter name 'la'.
```

That is what `ls -la` produces in PowerShell. The command was found. `ls` exists there. It
is an alias pointing at `Get-ChildItem`, a completely different program that has never heard
of `-la`.

This is the trap. A name that does not exist gives you a clean "not recognized" and you know
what to do. A name that exists and means something else gives you a real error from the
wrong program, and the error is about the flag rather than the name, so it points away from
the actual problem.

The names that do this are a short list and worth knowing by sight:

| Name | In bash it is | In PowerShell it is |
|---|---|---|
| `ls` | the list program | an alias for `Get-ChildItem` |
| `cat` | the print program | an alias for `Get-Content` |
| `curl` | the download program | the real program in PowerShell 7, an alias for `Invoke-WebRequest` in 5.1 |
| `wget` | the download program | gone in PowerShell 7, an alias for `Invoke-WebRequest` in 5.1 |
| `echo` | prints its arguments | an alias for `Write-Output` |
| `sort` | sorts lines of text | an alias for `Sort-Object` |
| `rm` | delete | an alias for `Remove-Item` |
| `where` | not a command | an alias for `Where-Object`, which filters |

When a command behaves nothing like its documentation, check what the name resolves to
before you check anything else.

```powershell
Get-Command curl
```

If the `CommandType` column says `Alias`, you have found your problem.

## Full

### The two that actually cost time

Most aliases are close enough to their Unix namesake that the difference never surfaces. Two
are not.

**`curl`.** In bash, `curl` is a real program with about two hundred flags, and every API
(Application Programming Interface) example on the internet uses it. In Windows PowerShell
5.1, `curl` is an alias for `Invoke-WebRequest`, which does a similar job with entirely
different spelling. A pasted `curl -H "Authorization: Bearer <token>" https://example.com`
fails there with a parameter complaint.

The fix is one character:

```powershell
curl.exe -H "Authorization: Bearer <token>" https://example.com
```

Adding `.exe` forces PowerShell past its own alias and runs the real program, which has
shipped with Windows since 2018. The same trick works for anything shadowed by an alias.

PowerShell 7 removed the `curl` and `wget` aliases, so on a modern install `curl` is already
the real thing. Which behavior you get depends on the version you are running, which is
exactly the kind of difference that makes this hard to search for.

**`where`.** In Command Prompt, `where node` finds a program. In PowerShell, `where` is an
alias for `Where-Object`, which filters a pipeline and has nothing to do with finding
programs. Typing `where node` in PowerShell does not error usefully. It waits, or it returns
nothing, because you have asked it to filter with a condition that makes no sense.

Always write `where.exe` in PowerShell. See [k4](#k4-finding-things) for what to use
instead.

### Finding out what a name really is

Three commands answer this, one per shell, and all three are worth knowing.

```powershell
Get-Command ls
```

PowerShell. The `CommandType` column tells you what it is: `Alias`, `Function`, `Cmdlet`, or
`Application`. Only `Application` means a real program on disk.

```powershell
Get-Alias -Name ls
```

Prints what the alias points at, when the answer above said `Alias`. Run
`Get-Alias` with nothing after it to see the whole list, which is a useful five minutes.

```bash
type ls
```

Bash and zsh. Prints one of "is aliased to", "is a shell builtin", or "is /usr/bin/ls". The
distinction matters for the same reason it does in PowerShell.

### Builtins, which are the other half of the same problem

Some commands are not programs at all. `cd` is not a file on disk anywhere. It is built into
the shell, because changing the current folder is something only the shell itself can do.

The practical consequence is that builtins cannot be found by a search of your PATH, and
they cannot be replaced by installing something. When `Get-Command` says `Cmdlet` or
`Function`, or bash says "shell builtin", you are looking at a piece of the shell rather
than a program, and the documentation you want is the shell's, not the internet's.

This is also why [c4](#c4-path-and-command-not-found) and this card are different problems
with similar symptoms. A missing program is a PATH question. A shadowed name is a language
question. The tell is whether the error names the command or names a parameter.

### The habit

Two moves cover nearly all of it.

When a copied command fails with a parameter error rather than a missing-command error, run
`Get-Command <name>` before changing anything about the command itself.

When you know you want the real Unix program on Windows and not PowerShell's version of it,
write the `.exe`. `curl.exe`, `where.exe`, `sort.exe`. It costs four characters and removes
the ambiguity entirely.
