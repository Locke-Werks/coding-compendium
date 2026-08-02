---
id: css
title: CSS
type: language
verified: 2026-08-02
volatility: low

name: CSS
aka: [css3, stylesheet, styles, cascading style sheets]
family: markup
likelihood: certain
extensions: ['.css']

# CSS is not a programming language and the body says so plainly, because an
# agent reporting that it "wrote some CSS" has changed how something looks and
# nothing else, and knowing that changes what you check in the diff. The tells
# below separate it from the two things it is genuinely confused with: a
# JavaScript object literal and a JSON file, both of which are braces full of
# `key: value` pairs.
tells:
  - pattern: '@media'
    kind: regex
    weight: 9
    note: >
      An at-rule, usually `@media (max-width: 768px)`, which applies a block of
      styling only at certain screen sizes. Java writes `@Override` and Python
      writes `@decorator`, both followed by a bare name. Nothing else follows an
      at-sign with a parenthesized width.
  - pattern: '!important'
    kind: regex
    weight: 9
    note: >
      An exclamation mark in the middle of a declaration, as in
      `color: red !important`. It overrides everything else. No other language in
      this deck uses it, and its presence in a diff is worth a question on its own.
  - pattern: '--[\w-]+\s*:'
    kind: regex
    weight: 8
    note: >
      Two hyphens starting a name is a custom property, CSS's version of a
      variable: `--brand-color: #1a1a1a;`. Bash and PowerShell use `--` for command
      flags and never follow one with a colon. Sass uses `$brand-color` instead,
      which is how you tell plain CSS from Sass.
  - pattern: ':(hover|focus|active|root|first-child|nth-child|disabled)\b'
    kind: regex
    weight: 8
    note: >
      A pseudo-class: a colon glued to a selector that means "only in this state".
      TypeScript also puts a colon after a name and means a type, as in
      `name: string`. Position separates them: this one sits before a brace, never
      inside a parameter list.
  - pattern: '\b\d+(px|rem|em|vh|vw)\b'
    kind: regex
    weight: 8
    note: >
      A number glued to a unit with no space, as in `16px` or `1.5rem`. Every
      programming language in this deck would read that as a syntax error. Seeing
      `padding: 12px` is close to proof.
  - pattern: '\b(display|margin|padding|font-size|background-color)\s*:'
    kind: regex
    weight: 7
    note: >
      Property names are fixed vocabulary you can learn. A JavaScript object or a
      JSON file also holds `key: value` pairs, and the keys are whatever the author
      invented. These specific words are CSS.
  - pattern: '^\s*[.#][\w-]+\s*\{'
    kind: regex
    weight: 7
    note: >
      A dot or hash starting a line and a brace ending it is a selector: `.card {`
      targets a class, `#cart {` targets an id. JavaScript never opens a block after
      a bare name; it opens one after `function`, `if`, or `=>`.
  - pattern: '#[0-9a-fA-F]{6}\b'
    kind: regex
    weight: 6
    note: >
      A hash followed by six hex digits is a color, as in `#3b82f6`. In Python,
      Bash, YAML, and TOML a `#` starts a comment, so the same character means
      opposite things depending on the file you are in.

rules_out:
  - pattern: '\bfunction\b'
    because: JavaScript or TypeScript. CSS has no functions you can define.
  - pattern: '\breturn\b'
    because: Any programming language. CSS never returns a value, because nothing in it computes one.
  - pattern: '=>'
    kind: operator
    because: JavaScript or TypeScript, most likely styling written inside a component file.
  - pattern: '<div'
    kind: regex
    because: HTML. CSS never contains an angle bracket outside a string.
  - pattern: '@(mixin|include|extend)'
    kind: regex
    because: Sass, a CSS dialect that compiles down to CSS. Plain CSS has no such at-rules.
  - pattern: '\$[\w-]+\s*:'
    kind: regex
    because: Sass variables. Plain CSS writes custom properties as `--name` instead.
  - pattern: '^\s*//'
    kind: line_start
    because: Sass or JavaScript. Plain CSS has no line comment, only `/* */`.

project_fingerprint:
  manifests:
    - file: '*.css'
      decisive: true
      note: >
        The extension is the whole story. There is no manifest, no compiler, and
        no dependency list, because CSS has nothing to declare.
    - file: tailwind.config.js
      decisive: false
      note: >
        The project uses Tailwind, which means most styling lives in `class="..."`
        attributes inside the HTML rather than in any `.css` file. Look there
        before hunting for a stylesheet that barely exists.
    - file: postcss.config.js
      decisive: false
      note: >
        A build step processes the CSS before shipping it, so the file in `dist/`
        will not match the file you edited. Expect the browser to show you
        generated output.
    - file: package.json
      decisive: false
      note: >
        Absent means the `.css` files on disk are exactly what the browser gets.
        Present means something builds them first.
  lockfiles: []
  build_dirs: [dist/, build/, .next/static/css/, node_modules/]
  entry_points: [src/index.css, src/styles.css, styles/globals.css, public/style.css]

shape:
  blocks: braces
  statement_end: semicolon
  comment_line: '/* */'
  comment_block: '/* */'
  string_quotes: >
    Quotes are rare and mostly optional. They show up in `url("bg.png")`,
    `font-family: "Segoe UI"`, and `content: "..."`. Values are otherwise bare.
  naming: kebab-case throughout, for property names, class names, and custom properties
  import_keyword: '@import, though a `<link>` tag in the HTML is the normal way'

tooling:
  package_manager: none
  registry: none
  runtime: any web browser
  install_command: none, it is text a browser reads
  run_command: none, the browser applies it when the page loads
  test_command: none, though tools like Stylelint check it for mistakes

confusable_with:
  - language: javascript
    settle_it: >
      A JavaScript object looks like a CSS rule: braces full of `key: value`.
      JavaScript separates pairs with commas and quotes its string values. CSS
      ends every pair with a semicolon and quotes almost nothing. A brace opening
      after a bare word, a dot, or a hash is CSS.
    tiebreak: { pattern: '(=>|\bfunction\b|\bconst\b)', kind: regex, favors: javascript }
  - language: json
    settle_it: >
      Both are braces full of pairs. JSON quotes every single key, separates pairs
      with commas, and allows no comments. CSS quotes no keys, ends each pair with
      a semicolon, and allows `/* */`. One quoted key settles it for JSON.
    tiebreak: { pattern: '"\w+"\s*:', kind: regex, favors: json }
  - language: html
    settle_it: >
      They ship together and look nothing alike. HTML is angle brackets and
      attributes, CSS is selectors and braces. A file with no `<` in it is not
      HTML. A file with no `{` in it is not CSS.
    tiebreak: { pattern: '</\w+>', kind: regex, favors: html }
  - language: yaml
    settle_it: >
      Both use `key: value` pairs and both look like settings. YAML uses
      indentation with no braces and no semicolons. CSS wraps every group in braces
      and ends every line with a semicolon.
    tiebreak: { pattern: '\{', kind: regex, favors: css }

errors_look_like:
  sample: |
    [vite:css] [postcss] Unclosed block
    file: C:/Users/nyx/shop/src/styles.css:42:1

    GET http://localhost:5173/styles.css
    Failed to load resource: the server responded with a status of 404 (Not Found)
  recognize_by: >
    CSS almost never produces an error. A misspelled property or an invalid value
    is dropped silently and everything around it still applies, so the only symptom
    is that the page looks wrong. Two exceptions produce real text. A build tool
    such as PostCSS reports genuinely broken syntax, always naming a `.css` file
    with a line and column. And the browser console reports a 404 when the
    stylesheet itself fails to load, which is what an entirely unstyled page means.
    For the silent cases, open developer tools with F12, pick the element, and look
    at the Styles panel: dropped declarations appear struck through with a warning
    triangle.
  patterns:
    - '\[postcss\]'
    - '\.css:\d+:\d+'
    - 'Unclosed block'
    - 'Failed to load resource.*\.css'
    - 'Unexpected \}'

meet_it_when: >
  Any time something on a page is the wrong color, the wrong size, in the wrong
  place, or overlapping something else. An agent edits it constantly and reports it
  as "styling" or "polish", and it is the change most likely to look finished in a
  screenshot and be broken on a phone.

what_agents_get_wrong: >
  Two moves to search a diff for. The first is `!important`, which is CSS's
  override-everything flag. An agent that cannot work out why its rule is being
  ignored appends `!important` instead of finding the more specific rule that is
  winning. It works once. The second time, two rules both carry it and there is
  nothing left to escalate to, so the fix has made the next fix harder. The second
  is `style="color: red"` written directly onto an element in the HTML. It beats
  every stylesheet rule, cannot be reused, cannot be changed by a theme, and will
  not be found by anyone searching the `.css` file for the color. Also watch for a
  new block appended at the bottom of a file that duplicates a rule fifty lines
  above it, and for a hardcoded `#3b82f6` in a file that already defines
  `--color-primary` at the top.

version_landscape: >
  CSS gains features continuously and never breaks old files, so a stylesheet from
  2015 still works exactly as it did. What dates material is the layout technique:
  anything using `float` for page layout, clearfix hacks, or vendor prefixes such
  as `-webkit-box-shadow` predates flexbox and grid and should be replaced rather
  than extended. Custom properties, flexbox, and grid are all safe to use now.

see_also:
  - html
  - javascript
  - json
  - j6-web-basics
  - j1-how-to-recognize-a-language

keywords: [stylesheet, selector, specificity, cascade, flexbox, grid, media query, tailwind, responsive]
---

CSS (Cascading Style Sheets) is how a web page looks: color, spacing, size,
position, and what changes when the window gets narrow.

It is not a programming language. This deck files it beside HTML (HyperText Markup
Language) under markup, which is a filing decision rather than a precise one: CSS
is a stylesheet language, and the point of both labels is the same. There are no
variables you can compute with, no conditions, no loops, and no functions you write
yourself. It cannot make a decision or hold a value. An agent that says it "wrote
some CSS" changed how something appears and did not touch what the program does,
which tells you exactly what to check: open the page and look at it. Nothing else
will catch a styling mistake.

## The shape

A rule is a selector, then braces, then declarations. Each declaration is
`property: value;` and the semicolon is required, unlike JavaScript where it is
optional.

```css
:root {
  --brand: #3b82f6;          /* a custom property, reusable below */
}

.card {                      /* every element with class="card" */
  display: flex;
  padding: 12px;
  background-color: var(--brand);
}

#cart:hover {                /* the element with id="cart", while hovered */
  border-color: red;
}

@media (max-width: 600px) {  /* only on narrow screens */
  .card { display: block; }
}
```

Comments are `/* like this */` and there is no line comment. Writing `//` in a
`.css` file does nothing useful and the browser drops the line without telling you.
That trips up everyone arriving from JavaScript.

## What "cascading" means, and why it decides fights

Two rules can both target the same element. When their instructions disagree, CSS
picks a winner by specificity: an id beats a class, a class beats a plain tag name,
and when two rules are equally specific the one written later wins.

This is the source of nearly every "why is it still blue" moment. The rule you
edited is correct and something more specific is beating it. Open developer tools
with F12, select the element, and read the Styles panel: the losing rules are shown
struck through, with the winner at the top. That panel answers the question faster
than reading the file.

`!important` skips the whole contest. It is the reason the contest exists to skip,
and using it means the next person, including you next week, has no move left.

## Where it actually lives

Three places, in descending order of how much you want to see them.

A `.css` file, pulled in by `<link rel="stylesheet" href="styles.css">` in the
HTML. This is the version you can search, reuse, and change in one place.

A `class="..."` attribute, if the project uses Tailwind. Then the styling lives in
the markup as short class names such as `flex gap-2 p-3`, and there may be almost
nothing in any `.css` file. Confusing on arrival and completely normal.

A `style="..."` attribute directly on the element. This wins over both of the
above, applies to one element only, and is what an agent reaches for when it is in
a hurry.

## What it is not

A JavaScript object literal is braces full of `key: value` pairs, and so is a
JSON (JavaScript Object Notation) file, and so is a CSS rule. Three formats, one
silhouette. JSON quotes every key and separates with commas. JavaScript separates
with commas and quotes its strings. CSS quotes nothing and ends every line with a
semicolon.
