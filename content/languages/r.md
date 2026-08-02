---
id: r
title: R language
type: language
verified: 2026-08-02
volatility: low
verify: R --version

name: R
aka: [rlang, r-lang, rstats, cran]
family: interpreted
likelihood: unlikely
extensions: ['.R', '.r', '.Rmd', '.Rproj']

tells:
  - pattern: '<-'
    kind: operator
    weight: 9
    note: >
      R assigns with a left arrow, `x <- 5`, on nearly every line. Go uses `<-`
      to send on a channel and Haskell uses it inside a `do` block. Neither one
      puts a plain variable name to its left.
  - pattern: '%>%'
    kind: operator
    weight: 10
    note: >
      The pipe from the magrittr package, wrapped in percent signs. Any operator
      fenced by `%` is R and nothing else.
  - pattern: 'library\('
    kind: regex
    weight: 9
    note: >
      R loads a package with `library(dplyr)`, a function call rather than a
      keyword. Python writes `import`, Ruby writes `require`, PHP writes `use`.
  - pattern: 'c\('
    kind: regex
    weight: 8
    note: >
      A list of values is built with the function `c`, as in `c(1, 2, 3)`. Every
      other language uses brackets. This one letter appears in almost every R
      script written.
  - pattern: '\b(TRUE|FALSE|NULL|NA)\b'
    kind: regex
    weight: 7
    note: >
      R shouts its booleans in full capitals. Python capitalizes only the first
      letter, as `True`. JavaScript and PHP write them entirely lowercase.

rules_out:
  - pattern: 'def'
    because: Python or Ruby. R declares with `name <- function(x)`.
  - pattern: 'elif'
    because: Python or Bash. R writes `else if`.
  - pattern: '^\s*end\s*$'
    kind: regex
    because: Ruby, Lua, or Elixir. R closes with a brace.
  - pattern: '^import '
    kind: line_start
    because: Python or JavaScript.

project_fingerprint:
  manifests:
    - file: DESCRIPTION
      decisive: true
      note: >
        A file called DESCRIPTION with no extension at the root of a folder means
        an R package. It lists the package name, version, and dependencies in a
        format used nowhere else. NAMESPACE usually sits beside it.
    - file: '*.Rproj'
      decisive: true
      note: >
        An RStudio project file. It holds editor settings rather than
        dependencies, but only R projects have one.
    - file: renv.lock
      decisive: true
      note: >
        The lockfile from renv, R's answer to a Python virtual environment. It is
        JSON (JavaScript Object Notation) despite the name.
  lockfiles: [renv.lock]
  build_dirs: ['renv/library/', '.Rproj.user/']
  entry_points: [app.R, server.R, 'R/']

shape:
  blocks: braces
  statement_end: newline
  comment_line: '#'
  comment_block: 'None. Comment each line, or fold the block into a string nobody reads.'
  string_quotes: Single and double are identical, as in Python.
  naming: dot.case and snake_case both appear, sometimes in the same file, because the language predates the argument
  import_keyword: library

tooling:
  package_manager: install.packages, with renv for per-project isolation
  registry: CRAN, the central R package archive
  runtime: the R interpreter, usually driven from RStudio or Positron
  install_command: install.packages("<package-name>")
  run_command: Rscript <file>.R
  test_command: testthat::test_dir("tests")

confusable_with:
  - language: python
    settle_it: >
      They sit side by side in data work and both comment with `#`. R assigns
      with `<-`, loads packages with `library(x)`, and wraps blocks in braces.
      Python assigns with `=`, writes `import x`, and uses a colon plus
      indentation.
    tiebreak: { pattern: '<-', kind: operator, favors: r }
  - language: elixir
    settle_it: >
      Both pipe with `|>`, since R 4.1 added a native one. Elixir's pipe follows
      `defmodule` and `def ... do`. R's follows `library(...)` and lines full of
      `<-`.
    tiebreak: { pattern: 'defmodule', kind: token, favors: elixir }

meet_it_when: >
  You open a statistics, bioinformatics, or economics repository, or someone
  sends you an analysis script or a report file ending in `.Rmd`.

what_agents_get_wrong: >
  R counts from 1, so an agent porting Python code lands one position off in
  every loop and every slice. Worse, R recycles: adding a vector of three numbers
  to a vector of six does not fail, it repeats the short one twice and returns an
  answer. That turns a real bug into plausible output, which is the hardest kind
  to catch. Agents also mix base R with tidyverse idioms in the same script, so
  half the code uses `df$col` and the other half uses `%>%` and `mutate`, and
  both run. And `=` inside a function call names an argument while `<-` assigns a
  variable, so `f(x <- 1)` and `f(x = 1)` do different things and look nearly
  identical in a diff.

version_landscape: >
  R 4.0 in 2020 changed a long-standing default so that text columns stay text
  instead of turning into factors. Older answers assume the opposite and will
  hand you results that look wrong for no visible reason. Package versions matter
  more than the language version here, which is what renv exists to pin.

see_also:
  - python
  - elixir
  - g4-environments-and-isolation
  - j1-how-to-recognize-a-language

keywords: [cran, rstudio, tidyverse, dplyr, ggplot2, renv, rmarkdown, rscript, data frame]
---

The language of statistics. Written by statisticians rather than by software
engineers, which shows in every design decision it made.

## The shape

Blocks use braces, but assignment uses a left arrow rather than an equals sign.
Everything is a vector, so an operation on one number and an operation on ten
thousand look the same.

```r
library(dplyr)

scores <- c(10, 8, 6)
names <- c("nyx", "ada", "grace")

results <- data.frame(names, scores) %>%
  filter(scores > 6)

print(results)
```

The `%>%` is a pipe: it takes what is on the left and feeds it as the first
argument to what is on the right. Newer code uses the built-in `|>` instead, and
both mean the same thing.

## Reading its errors

An error names the call that failed, then the complaint, on one line.

```text
Error in filter(., scores > 6) : object 'scores' not found
```

There is no stack trace by default. `object '...' not found` is the common one
and means the name was never assigned, or was assigned in a different session
that has since been cleared.
