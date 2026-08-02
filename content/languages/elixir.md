---
id: elixir
title: Elixir
type: language
verified: 2026-08-02
volatility: low
verify: elixir --version

name: Elixir
aka: [ex, exs, beam]
family: bytecode
likelihood: unlikely
extensions: ['.ex', '.exs', '.heex']

tells:
  - pattern: 'defmodule'
    kind: token
    weight: 10
    note: >
      Every Elixir file opens with `defmodule Name do`. No other language on this
      deck has the word. Ruby writes `class`, Python writes `class`, Java writes
      `public class`.
  - pattern: '%\{'
    kind: regex
    weight: 9
    note: >
      A percent sign followed by a brace opens a map, as in `%{name: "nyx"}`.
      Ruby and Python write a bare `{` for the same idea. Nothing else prefixes
      it.
  - pattern: 'def \w+.*\sdo$'
    kind: regex
    weight: 8
    note: >
      Elixir shares `def` with Python and Ruby. Its line ends in `do` and closes
      with `end`. Python ends in a colon and indents; Ruby ends in nothing.
  - pattern: '\|>'
    kind: operator
    weight: 8
    note: >
      The pipe feeds the value on the left in as the first argument on the right.
      R has a native `|>` too, so check for `defmodule` before deciding. No other
      language on this deck pipes at all.
  - pattern: '(?<![:\w]):\w+'
    kind: regex
    weight: 6
    note: >
      A leading colon marks an atom, a name that stands only for itself. Ruby
      symbols look identical, which is where they came from. Ruby will have
      `@ivars` and `puts` nearby; Elixir will have `defmodule` and `|>`.

rules_out:
  - pattern: 'class'
    because: Java, C#, Python, Ruby, or JavaScript. Elixir has no classes and no objects.
  - pattern: 'return'
    because: Almost anything else. Elixir has no `return` keyword; the last expression is the value.
  - pattern: 'elsif'
    because: Ruby. Elixir has no `elif` or `elsif` of any spelling and uses `cond` instead.
  - pattern: '\$\w+'
    kind: regex
    because: PHP, Bash, or PowerShell.
  - pattern: 'function'
    because: JavaScript, PHP, or Lua. Elixir writes `def` and `fn`.

project_fingerprint:
  manifests:
    - file: mix.exs
      decisive: true
      note: >
        Mix is the build tool and this is its manifest. The file is Elixir source
        code, opening with `defmodule MyApp.MixProject`, so the manifest is
        written in the same language as the project.
    - file: mix.lock
      decisive: true
      note: >
        The pinned versions of every dependency. Commit it.
    - file: 'lib/*_web/router.ex'
      decisive: false
      note: >
        Means the project is Phoenix, the web framework, which is why most people
        meet Elixir at all.
  lockfiles: [mix.lock]
  build_dirs: ['_build/', 'deps/']
  entry_points: ['lib/<app>.ex', 'lib/<app>/application.ex']

shape:
  blocks: keyword
  statement_end: newline
  comment_line: '#'
  comment_block: 'None. Module and function documentation goes in @moduledoc and @doc strings instead.'
  string_quotes: >
    Double quotes make a string. Single quotes make a charlist, which is a
    different type and a real trap for anyone arriving from another language.
  naming: snake_case for functions and variables, CamelCase for modules, a trailing `?` for questions and a trailing `!` for the version that fails loudly
  import_keyword: alias for shortening a module name, import for pulling functions in, require for macros

tooling:
  package_manager: Mix, over the Hex registry
  registry: hex.pm
  runtime: the Erlang virtual machine, which Elixir compiles down to
  install_command: mix deps.get
  run_command: mix run, or iex -S mix for an interactive session
  test_command: mix test

confusable_with:
  - language: ruby
    settle_it: >
      Elixir borrowed `do`, `end`, and `:atoms` from Ruby, so the outline is
      nearly identical. Elixir files start with `defmodule` and pipe with `|>`.
      Ruby files have `class`, `@name` instance variables, and `puts`.
    tiebreak: { pattern: 'defmodule', kind: token, favors: elixir }
  - language: r
    settle_it: >
      Both pipe with `|>`. R fills its lines with `<-` and `library(...)`.
      Elixir opens with `defmodule` and closes blocks with `end`.
    tiebreak: { pattern: '<-', kind: operator, favors: r }

meet_it_when: >
  A team picked it for something that has to stay up under load, usually a chat,
  a queue, or a live dashboard. You will see it as a Phoenix web application or
  not at all.

what_agents_get_wrong: >
  Data in Elixir cannot be changed, and agents write as though it can. They
  rebind a variable inside an `if` or a comprehension and expect the outer one to
  have changed, which never happens and produces code that runs and does nothing.
  They also reach for exceptions where Elixir expects tuples: real code returns
  `{:ok, value}` or `{:error, reason}` and matches on it with `case`, so a
  generated `try` and `rescue` block is the tell that the agent is writing Ruby
  in Elixir syntax. The dangerous one is `String.to_atom` on anything a user
  typed. Atoms are never cleaned up, so that line is a slow memory leak with no
  symptom until the machine falls over.

version_landscape: >
  Elixir 1.x has stayed compatible across its whole life, so an answer from 2019
  usually still runs. The version that bites is Phoenix: LiveView changed shape
  several times before 1.0, and older answers reference callbacks that no longer
  exist. Check the versions in mix.exs before trusting a snippet.

see_also:
  - ruby
  - r
  - python
  - g3-lockfiles
  - j1-how-to-recognize-a-language

keywords: [mix, phoenix, liveview, hex, erlang, otp, genserver, beam, iex]
---

A language for services that stay up. It runs on the Erlang virtual machine,
which is built to keep going while parts of it crash and restart.

## The shape

Blocks open with `do` and close with `end`, as in Ruby. Nothing has a class,
nothing has an object, and no value can be changed after it is made.

```elixir
defmodule Scores do
  def report(scores) do
    scores
    |> Enum.filter(fn {_name, points} -> points > 6 end)
    |> Enum.each(fn {name, points} -> IO.puts("#{name}: #{points}") end)
  end
end

Scores.report(%{"nyx" => 10, "ada" => 4})
```

Read the `|>` chain top to bottom: each line takes the result above it as its
first argument. That is the shape most Elixir code takes, and it is the fastest
way to recognize the language from across the room.

## Reading its errors

An error names the exception module, the message, and then the frames.

```text
** (ArithmeticError) bad argument in arithmetic expression
    (my_app 0.1.0) lib/scores.ex:4: Scores.report/1
```

The two stars at the start of the line are the giveaway. The `/1` after a
function name is how many arguments it takes, which is part of the name in this
language: `report/1` and `report/2` are two different functions.
