---
id: ruby
title: Ruby
type: language
verified: 2026-08-02
volatility: low
verify: ruby --version

name: Ruby
aka: [rb, ruby-lang, mri]
family: interpreted
likelihood: possible
extensions: ['.rb', '.erb', '.rake', '.gemspec', '.ru']

# Ruby's nearest neighbor is Python: same `def`, same dynamic typing, same
# prose-like reading. Every note below is written to be the thing that separates
# the two in one glance.
tells:
  - pattern: '^\s*end\s*$'
    kind: regex
    weight: 7
    note: >
      A line holding nothing but `end` closes a block. Python outdents instead
      and JavaScript writes `}`. Lua and Elixir also close with `end`, but Lua
      opens with `local` and `then`, and Elixir opens with `defmodule` and `do`.
  - pattern: 'elsif'
    kind: token
    weight: 9
    note: >
      Ruby is the only language on this deck that spells it `elsif`. Python uses
      `elif`, PHP and Lua use `elseif`, the brace languages write `else if`.
  - pattern: '@\w+'
    kind: sigil
    weight: 8
    note: >
      An `@` glued to a name is an instance variable, as in `@name`. Python
      writes `self.name` and JavaScript writes `this.name`. Python also uses `@`,
      but alone on its own line as a decorator, never stuck to a variable.
  - pattern: 'do \|\w+\|'
    kind: regex
    weight: 9
    note: >
      Block parameters go between vertical bars, as in `items.each do |item|`.
      Rust puts closure parameters between bars too, but never after the word
      `do`.
  - pattern: 'puts'
    kind: token
    weight: 8
    note: >
      Ruby prints with `puts`. Python and Lua use `print`, PHP uses `echo`,
      JavaScript uses `console.log`.
  - pattern: 'def \w+[!?]'
    kind: regex
    weight: 8
    note: >
      A method name ending in `?` or `!` is legal in Ruby and a syntax error in
      Python, which shares the `def` keyword and nothing else about this line.
  - pattern: 'require'
    kind: token
    weight: 5
    note: >
      Ruby writes `require "json"` with no parentheses and no assignment. Node
      writes `const x = require("pkg")` with both. Python says `import`.
  - pattern: 'nil'
    kind: token
    weight: 5
    note: >
      Ruby's nothing is `nil`. Python spells it `None`, JavaScript has both
      `null` and `undefined`, PHP uses `null`. Lua and Go also say `nil`, so this
      one corroborates rather than decides.

rules_out:
  - pattern: 'elif'
    because: Python or Bash. Ruby always writes `elsif`.
  - pattern: 'def \w+\(.*\):'
    kind: regex
    because: Python. The colon after the signature is the tell.
  - pattern: 'None'
    because: Python.
  - pattern: 'function'
    because: JavaScript, TypeScript, PHP, or Lua.
  - pattern: '\$\w+'
    kind: regex
    because: PHP, Bash, or PowerShell. Ruby has global variables with `$`, but they are rare enough that a page full of them is not Ruby.
  - pattern: 'defmodule'
    because: Elixir, which borrowed `do` and `end` from Ruby and nothing else.
  - pattern: '<\?php'
    kind: regex
    because: PHP.

project_fingerprint:
  manifests:
    - file: Gemfile
      decisive: true
      note: >
        The dependency manifest, read by Bundler. No extension, capital G. It is
        Ruby source code rather than a config format, so it has real lines like
        `source "https://rubygems.org"` and `gem "rails", "~> 7.1"`.
    - file: '*.gemspec'
      decisive: true
      note: >
        Describes a package that is meant to be published to RubyGems. Also Ruby
        source. If you see one, the folder is a library rather than an app.
    - file: Rakefile
      decisive: true
      note: >
        Ruby's make. Holds named tasks you run with `rake test`. Ruby source
        again, which is the running theme: Ruby projects configure themselves in
        Ruby rather than in TOML (Tom's Obvious Minimal Language) or JSON.
    - file: config/routes.rb
      decisive: false
      note: >
        Not a manifest, but seeing it means the project is Rails specifically
        rather than plain Ruby, and that changes which answers apply.
  lockfiles: [Gemfile.lock]
  build_dirs: ['vendor/bundle/', '.bundle/', 'tmp/', 'log/']
  entry_points: [main.rb, app.rb, config.ru, 'bin/rails']

shape:
  blocks: keyword
  statement_end: newline
  comment_line: '#'
  comment_block: '=begin and =end, both at column zero. Almost nobody uses them.'
  string_quotes: >
    Double quotes interpolate `#{name}` and understand escapes. Single quotes are
    literal. Ruby cares about the difference where Python does not.
  naming: snake_case for methods and variables, CamelCase for classes, SCREAMING_SNAKE_CASE for constants, a trailing `?` for questions and a trailing `!` for the dangerous version
  import_keyword: require

tooling:
  package_manager: Bundler, over the gem command
  registry: RubyGems.org
  runtime: the ruby interpreter, installed separately
  install_command: bundle add <gem-name>
  run_command: ruby <file>.rb
  test_command: bundle exec rspec

confusable_with:
  - language: python
    settle_it: >
      Both declare with `def` and both read like English. Ruby needs no colon
      after the signature and closes the body with `end`; Python ends the line
      with a colon and closes by outdenting. Ruby writes `elsif` and marks
      instance variables `@name`; Python writes `elif` and takes `self` as the
      first parameter.
    tiebreak: { pattern: '^\s*end\s*$', kind: regex, favors: ruby }
  - language: elixir
    settle_it: >
      Elixir took `do`, `end`, and `:atoms` straight from Ruby, so the outline
      looks identical. Elixir opens files with `defmodule`, pipes with `|>`, and
      has no `class` keyword at all. Ruby has `class`, `@ivars`, and no pipe.
    tiebreak: { pattern: 'defmodule', kind: token, favors: elixir }
  - language: javascript
    settle_it: >
      Both use `=>` and it means opposite things. In Ruby it sits between a key
      and a value inside a hash, as in `{ :a => 1 }`. In JavaScript it sits
      between parameters and a function body, as in `(a) => a + 1`.
    tiebreak: { pattern: 'console\.log', kind: regex, favors: javascript }

errors_look_like:
  sample: |
    app.rb:12:in `total': undefined method `+' for nil (NoMethodError)
    	from app.rb:20:in `<main>'
  recognize_by: >
    The first line starts with `file.rb:line:in`, method names are wrapped in a
    backtick on the left and a straight quote on the right, and the exception
    class sits in parentheses at the end of that same line. Later frames are
    indented and begin with the word `from`. Python puts its error class at the
    start of the last line; Ruby puts it in parentheses at the end of the first.
  patterns:
    - '^[\w./\\-]+\.rb:\d+:in '
    - '\((\w+)?(Error|Exception)\)\s*$'
    - '^\s+from .*:\d+:in '

meet_it_when: >
  You clone something with a Gemfile in it, a static site turns out to be built
  with Jekyll, or a tool you want is distributed as a gem. Agents rarely pick Ruby
  on their own now. When you meet it, the project usually chose it years ago and
  is probably Rails.

what_agents_get_wrong: >
  Agents write Rails even when the project is plain Ruby. Methods like `blank?`,
  `present?`, and `2.days.ago` come from Rails rather than from the language, so
  generated code calls them and dies with `NoMethodError` in a script that never
  loaded Rails. The second habit is monkey patching: reopening a core class like
  `String` to add a method, which changes behavior for the whole program from a
  file nobody thinks to look in. Third, the bang convention gets ignored. `save`
  returns false and carries on when it fails, `save!` stops the program, and an
  agent that picked the wrong one produces a chain that silently does nothing.
  Fourth, `rescue` with no class attached catches every ordinary error, and
  `rescue Exception` catches even your Ctrl+C. Last, watch for a new
  `require "something"` with no matching `gem` line added to the Gemfile: it runs
  for the agent and fails on a clean checkout.

version_landscape: >
  The language itself is stable. Ruby 3 arrived in 2020 and old code mostly kept
  working, so a 2019 answer about Ruby syntax is usually still correct. The split
  that matters is Rails, not Ruby: an answer written for Rails 5 can be wrong for
  Rails 7, and the version is in the Gemfile. One newer wrinkle is cosmetic and
  confusing anyway: Ruby 3.4 changed how error messages quote method names, so
  the traceback you search for may not match the one on your screen character for
  character.

see_also:
  - python
  - elixir
  - javascript
  - g2-package-managers
  - g3-lockfiles
  - f2-stack-traces
  - j1-how-to-recognize-a-language

keywords: [gem, gems, bundler, rails, rspec, rake, jekyll, monkey patch, activerecord, irb]
---

A dynamic language designed to be pleasant to read, which it achieves by giving
you six ways to write the same line. Its packages are called **gems** and its
package manager is **Bundler**.

## The shape

Blocks open with a keyword and close with `end`. Indentation is two spaces by
convention and means nothing to the interpreter, unlike Python, where it means
everything.

Statements end at the end of the line. Parentheses around arguments are optional,
which is why Ruby often reads like a sentence and why it is sometimes hard to
tell a method call from a variable.

```ruby
count = 3                       # no declaration keyword
MAX_ITEMS = 100                 # capitalized, so Ruby treats it as a constant

def add(a, b)                   # no colon, no types
  a + b                         # last line is the return value
end

if count > 2
  puts "#{count} is plenty"     # #{} substitutes inside double quotes
elsif count == 2
  puts "two"
else
  puts "not enough"
end
```

Comments are `#` to the end of the line, the same character Python uses.

Three habits give Ruby away faster than anything else. Instance variables carry
an `@` in front, so `@name` is state that belongs to the object. Method names may
end in `?` or `!`, where `?` means it answers a question and `!` means it is the
version that modifies things or blows up. And blocks of work get handed to
methods between vertical bars.

## Six lines of it

```ruby
require "json"

scores = { "nyx" => 10 }

scores.each do |name, points|
  puts "#{name}: #{points}"
end

puts JSON.generate(scores)
```

The `do |name, points|` part is a block: a chunk of code passed into `each` and
run once per entry. Vertical bars around parameters, after the word `do`, is a
shape no other language on this deck produces.

## What it is for

Web applications through Rails, static sites through Jekyll, build and deploy
scripts, and command-line tools. Rails is the reason most Ruby exists, and a
large share of the Ruby you meet is really Rails wearing Ruby's clothes.

Ruby needs the Ruby interpreter installed to run. On Windows that means
RubyInstaller, which is a heavier lift than Python and part of why you meet Ruby
less often here.

## Reading its errors

Ruby leads with the location and finishes with the error class.

```text
app.rb:12:in `total': undefined method `+' for nil (NoMethodError)
	from app.rb:20:in `<main>'
```

Read the first line: file, line number, the method it was inside, then the
complaint, then the class of error in parentheses. The indented `from` lines
underneath are who called it, newest first.

`undefined method ... for nil` is the one you will hit most. It means something
you expected to hold a value held `nil` instead, and the real bug is wherever
that value was supposed to be set.
