---
id: markdown
title: Markdown
type: language
verified: 2026-08-02
volatility: low

name: Markdown
aka: [md, commonmark, gfm, github flavored markdown, mdx]
family: markup
likelihood: certain
extensions: ['.md', '.markdown', '.mdx']

tells:
  - pattern: '^#{1,6}\s+\S'
    kind: regex
    weight: 7
    note: >
      One to six hash marks, a space, then text is a heading. In YAML, TOML, and
      INI a `#` starts a comment instead, and those never require the space.
  - pattern: '\*\*[^*\n]+\*\*'
    kind: regex
    weight: 8
    note: >
      Doubled asterisks around a phrase mean bold. No configuration format has
      inline emphasis of any kind, because none of them are meant to be read as
      prose.
  - pattern: '\]\('
    kind: regex
    weight: 8
    note: >
      A closing square bracket touching an opening parenthesis is a link,
      `[text](url)`. HTML writes the same link as `<a href="url">text</a>` and
      nothing else uses this shape.
  - pattern: '^\|.*\|\s*$'
    kind: regex
    weight: 6
    note: >
      A line that starts and ends with a pipe is a table row. This comes from
      GitHub's dialect, not from the original format, which had no tables at all.
  - pattern: '^- \[[ xX]\]'
    kind: regex
    weight: 7
    note: >
      A dash, then square brackets holding a space or an x, is a task list
      checkbox. Also a GitHub extension, and it renders as literal brackets
      anywhere that does not support it.
  - pattern: '^\s*[-*+]\s+\S'
    kind: regex
    weight: 3
    note: >
      A dash and a space is a bullet. Weak on its own, because YAML uses exactly
      the same mark for list entries. The difference is that a YAML entry is a
      `key: value` pair and a Markdown bullet is a sentence.
  - pattern: '^> '
    kind: regex
    weight: 4
    note: >
      A greater-than sign at the start of a line is a block quote. In YAML the
      same character at the end of a line opens a folded string instead.

rules_out:
  - pattern: '^\s*[\w.-]+\s*=\s*"'
    kind: regex
    because: TOML or INI. A quoted assignment is configuration, not prose.
  - pattern: '<!DOCTYPE'
    kind: regex
    because: HTML, which declares its document type on the first line
  - pattern: '^\s*\{\s*$'
    kind: regex
    because: JSON, if the file opens with a lone brace
  - pattern: '#include'
    kind: line_start
    because: C or C++, where a hash starts a preprocessor directive rather than a heading

project_fingerprint:
  manifests:
    - file: README.md
      note: >
        The front door of every repository. GitHub renders it on the project
        page, and it is the first thing to read when you clone something you did
        not write.
    - file: CLAUDE.md
      note: >
        Standing instructions Claude Code reads at the start of every session in
        that project. Written in Markdown because a person has to read it too.
    - file: AGENTS.md
      note: The same idea for Codex and several other tools. Same format, different filename.
    - file: CHANGELOG.md
      note: What changed in each released version, newest first by convention.
    - file: CONTRIBUTING.md
      note: How a project wants patches submitted. Worth reading before opening a pull request.
    - file: docs/**/*.md
      note: A documentation folder. Nearly always Markdown, frequently built into a website.
    - file: '*.md'
      decisive: true
      note: The extension settles it. `.mdx` means Markdown with embedded components, which is a different thing.

shape:
  blocks: none
  statement_end: none
  comment_line: 'none, though most renderers pass an HTML comment through without displaying it'
  comment_block: '<!-- -->'
  string_quotes: 'none, everything in the file is text already'
  naming: >
    lower-case filenames with dashes, apart from README, LICENSE, CHANGELOG, and
    CLAUDE, which are capitalized by tradition so they sort to the top.
  import_keyword: 'none in the format itself, though static site tools add their own include syntax'

confusable_with:
  - language: html
    settle_it: >
      Both describe formatted text. Markdown marks it with punctuation,
      `**bold**` and `# Heading`. HTML marks it with tags, `<b>bold</b>` and
      `<h1>Heading</h1>`. Markdown allows raw HTML inside it, so a few tags do
      not make a file HTML.
    tiebreak: { pattern: '</[a-zA-Z][\w-]*>', kind: regex, favors: html }
  - language: yaml
    settle_it: >
      A block fenced by `---` at the very top of a `.md` file is frontmatter, and
      that block is YAML. Above the closing dashes, YAML rules apply. Below them,
      `#` means heading rather than comment.
    tiebreak: { pattern: '^#{1,6}\s+\S', kind: regex, favors: markdown }

errors_look_like:
  sample: |
    README.md:14:1 MD022/blanks-around-headings Headings should be surrounded by blank lines
    README.md:31:81 MD013/line-length Line length [Expected: 80; Actual: 104]
  recognize_by: >
    Markdown itself cannot fail to parse, so anything that looks like an error
    came from a linter rather than a renderer. Recognize those by a rule code of
    the form `MD` followed by three digits, next to a `.md` filename with a line
    and column.
  patterns:
    - 'MD\d{3}/[a-z-]+'
    - '\.mdx?:\d+:\d+'
    - 'markdownlint'

meet_it_when: >
  Constantly, and usually without noticing. Every `README.md` you open, the
  `CLAUDE.md` and `AGENTS.md` files your agents read before they touch anything,
  every pull request description, every issue comment, and the chat window you
  type into, which renders Markdown live as you write.

what_agents_get_wrong: >
  Agents write Markdown fluently and then break it in two specific ways. First,
  they mix dialects: a table or a task list that renders on GitHub does nothing
  in a plain CommonMark renderer, and the document silently degrades into rows of
  pipe characters. Second, they get code fences wrong when a snippet contains
  backticks, so the fence closes early and the rest of your document renders as
  code. There is no error message for either, because nothing validates a
  Markdown file. The other one to watch in a diff: an agent editing `CLAUDE.md`
  will happily reword instructions you wrote deliberately, and since the file is
  prose, the change reads as an improvement rather than a behavior change. Read
  those diffs line by line.

version_landscape: >
  The original 2004 version was a Perl script and a loose description, which is
  why dialects exist. CommonMark standardized the core in 2014 and most renderers
  follow it. GitHub Flavored Markdown adds tables, task lists, strikethrough, and
  autolinks on top. Answers from any year still apply to the core; anything about
  tables or checkboxes depends on which renderer you are using.

see_also:
  - html
  - yaml
  - xml
  - e4-claude-md-and-agents-md
  - j4-reading-a-repo-you-did-not-write
  - d8-pull-requests

keywords: [md, readme, commonmark, gfm, code fence, frontmatter, task list, table, markdownlint]
---

Markdown is a plain-text format for writing formatted documents. It is not a programming
language. It has no logic, no functions, and nothing inside a `.md` file ever runs. It
describes text: which words are bold, which line is a heading, which block is code.

The point of it is that the raw file stays readable. `**bold**` still says bold even when
nothing is rendering it.

## The shape

Marks at the start of a line control the structure. Marks inside a line control emphasis.

```markdown
# A heading
## A smaller heading

Ordinary text. **Bold** and *italic* and `inline code`.

- A bullet
- Another bullet

1. A numbered item
2. Another one

[A link](https://example.com)

> A quoted block
```

The contrast: HTML (HyperText Markup Language) does the same job with angle-bracket tags,
`<h1>A heading</h1>`. Markdown is what you write and HTML is frequently what it becomes.
YAML (YAML Ain't Markup Language) also uses `-` for lists and looks similar at a glance,
and the difference is that a YAML line is a `key: value` pair while a Markdown line is a
sentence.

A fenced code block is three backticks, a language name, your code, then three backticks
again. The language name is what colors the block and it is required by this project's
linter.

Blank lines carry meaning. Two lines of text with no blank line between them render as one
paragraph. That catches more people than any other rule in the format.

## What it is for

Every `README.md` on GitHub. `CLAUDE.md` and `AGENTS.md`, which are the standing
instructions your agents read at the start of each session. Issue and pull request
descriptions. Documentation sites, which are usually a folder of `.md` files run through a
generator. The chat box you type into is rendering Markdown as you go.

## The gotchas

Markdown has no single specification. It has dialects, and they disagree about precisely
the features you want.

**Tables are not standard Markdown.** They come from GFM (GitHub Flavored Markdown) and
work on GitHub, in Visual Studio Code, and in most modern renderers. In a strict
CommonMark renderer they come out as literal pipe characters.

**Task lists are not standard either.** `- [ ]` and `- [x]` become checkboxes on GitHub and
stay as literal square brackets in plenty of other places. Jira wants its own syntax
entirely.

**Code fences are the safest extension there is.** Three backticks are near-universal now.
The older way was to indent a block by four spaces, which still works, and it is why a
paragraph of yours occasionally renders as code: you indented it.

**Line breaks are hard to get on purpose.** A single newline inside a paragraph is thrown
away. To force a break you end the line with two trailing spaces, which are invisible, or
you write `<br>`. Most people give up and insert a blank line.

**Underscores inside identifiers.** `some_variable_name` can turn into italics in older
renderers, because an underscore is an emphasis mark. Wrap identifiers in backticks and the
problem disappears.

## Frontmatter

A block fenced by `---` at the very top of a Markdown file is called frontmatter, and it is
YAML, not Markdown. Static site generators and this app both use it to attach structured
data to a document.

```markdown
---
title: My post
date: 2026-08-02
---

The document itself starts here.
```

When a file opens with three dashes, the rules that apply to the next few lines are YAML's,
not Markdown's.

## Reading its errors

There are none. A Markdown file cannot fail to parse. Every renderer accepts every file and
produces something, so when it produces the wrong thing there is no message at all, only
output that looks off. That is the price of a format with no syntax errors.

When a document renders badly, the causes in order of likelihood are a missing blank line,
a code fence that never closed, and a feature your renderer does not support. Anything that
looks like a Markdown error message came from a linter such as `markdownlint`, which is
style advice rather than a parse failure.
