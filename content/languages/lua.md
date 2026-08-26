---
id: lua
title: Lua
type: language
verified: 2026-08-02
volatility: low
verify: lua -v

name: Lua
aka: [luajit, lua5]
family: interpreted
likelihood: possible
extensions: ['.lua']

tells:
  - pattern: '~='
    kind: operator
    weight: 10
    note: >
      Lua writes "not equal" as `~=`. Every other language on this deck writes
      `!=`. Nothing else uses the tilde this way, so one sighting is enough.
  - pattern: 'local'
    kind: token
    weight: 8
    note: >
      Lua declares with `local x = 1`. Rust and JavaScript use `let`, Go uses
      `:=`, Python uses no keyword at all. Only Lua says `local`.
  - pattern: '\.\.'
    kind: operator
    weight: 8
    note: >
      Two dots join strings, as in `"a" .. "b"`. Ruby uses `..` for a range of
      numbers instead, which is a completely different thing that looks the same.
  - pattern: 'then'
    kind: token
    weight: 7
    note: >
      An `if` line ends in `then` and the block closes with `end`. Bash also uses
      `then`, but Bash closes with `fi` and Ruby uses no `then` at all.
  - pattern: '^\s*--'
    kind: regex
    weight: 6
    note: >
      Comments are two hyphens. SQL and Haskell comment the same way, but neither
      one has `local` or `end` sitting nearby.

rules_out:
  - pattern: 'def'
    because: Python or Ruby. Lua declares functions with `function`.
  - pattern: 'elsif'
    because: Ruby. Lua spells it `elseif`, one word, the way PHP does.
  - pattern: 'elif'
    because: Python or Bash.
  - pattern: '\$\w+'
    kind: regex
    because: PHP, Bash, or PowerShell. Lua never marks a variable.

project_fingerprint:
  manifests:
    - file: '*.rockspec'
      decisive: true
      note: >
        LuaRocks package metadata, and Lua source code itself rather than a
        config format.
    - file: init.lua
      decisive: false
      note: >
        In a folder under `~/AppData/Local/nvim`, this is a Neovim configuration.
        Elsewhere it is the entry point of a Lua module.
    - file: conf.lua
      decisive: true
      note: >
        The settings file for the Love2D game framework. Nothing else uses it.
  lockfiles: []
  build_dirs: []
  entry_points: [main.lua, init.lua]

shape:
  blocks: keyword
  statement_end: optional_semicolon
  comment_line: '--'
  comment_block: '--[[ ]]'
  string_quotes: Single and double are identical. Double square brackets open a string that spans lines.
  naming: snake_case throughout, with `M` as the conventional name for the table a module returns
  import_keyword: require

tooling:
  package_manager: LuaRocks
  registry: luarocks.org
  runtime: the lua binary, or the host program that embeds it
  install_command: luarocks install <rock-name>
  run_command: lua <file>.lua
  test_command: busted

confusable_with:
  - language: ruby
    settle_it: >
      Both close blocks with a bare `end`. Ruby has `def`, `puts`, and `@name`
      instance variables. Lua has `local`, `function`, and `~=` for not-equal.
    tiebreak: { pattern: '~=', kind: operator, favors: lua }
  - language: javascript
    settle_it: >
      Both declare functions with the word `function` and both are dynamically
      typed. JavaScript wraps the body in braces and ends the file without
      ceremony. Lua closes every block with `end` and comments with `--`.
    tiebreak: { pattern: '^\s*end\s*$', kind: regex, favors: lua }

meet_it_when: >
  You configure Neovim, mod a game, write a Redis script, or open the plugin
  folder of a larger application that embeds Lua as its scripting layer.

what_agents_get_wrong: >
  Lua counts from 1, not 0. An agent that has written a million Python loops will
  produce `for i = 0, #items do` and quietly skip the first entry or run one past
  the end. Second, a variable declared without `local` becomes global for the
  whole program, and agents drop the keyword constantly; the bug shows up as one
  file changing another file's value. Third, `#t` on a table with a gap in it
  returns an undefined length, so counting a list that had something removed from
  the middle gives a number nobody can predict.

version_landscape: >
  Lua 5.1, 5.2, 5.3, and 5.4 are all still in use because host programs pin them.
  LuaJIT, which many games and Neovim plugins rely on, is 5.1 with extras. An
  answer online is worth nothing until you know which one you are running, so
  check with `lua -v` first.

see_also:
  - ruby
  - javascript
  - python
  - j1-how-to-recognize-a-language

keywords: [luarocks, luajit, neovim, nvim, roblox, love2d, wow addon]
---

A small scripting language designed to be embedded inside other programs. When an
application lets you write plugins, the language is often this one.

## The shape

Blocks close with `end`, as in Ruby. Variables need `local` or they go global.
Comments are two hyphens. Not-equal is `~=`, which no other language does.

```lua
local scores = { ada = 10 }

for name, points in pairs(scores) do
  if points ~= 0 then
    print(name .. ": " .. points)   -- .. joins strings
  end
end
```

Tables do all the work. A Lua table is the array, the dictionary, and the object
all at once, and it is the only data structure the language has.

## Reading its errors

An error names the file, the line, and the complaint, then prints a stack
traceback underneath.

```text
lua: app.lua:4: attempt to concatenate a nil value (local 'points')
stack traceback:
        [C]: in function 'error'
```

`attempt to index a nil value` is the one you will hit most. It means the table
you reached into was never created.
