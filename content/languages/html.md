---
id: html
title: HTML
type: language
verified: 2026-08-02
volatility: low

name: HTML
aka: [html5, hypertext, markup, htm, web page source]
family: markup
likelihood: certain
extensions: ['.html', '.htm']

# HTML is not a programming language and the card says so in the first line,
# because nobody ever tells beginners and the distinction changes what you expect
# from a diff. The tells here separate it from the three things it is actually
# mistaken for: XML, JSX inside a React file, and a templating language.
tells:
  - pattern: '<!DOCTYPE html'
    kind: regex
    weight: 10
    note: >
      The first line of essentially every web page. XML opens with
      `<?xml version="1.0"?>` instead, and nothing else in this deck opens a file
      with an exclamation mark inside an angle bracket. Decisive on its own.
  - pattern: '<(html|head|body)\b'
    kind: regex
    weight: 9
    note: >
      The three fixed section tags. XML invents its own tag names for whatever it
      describes and has no reserved vocabulary, so `<body>` appearing at all is
      HTML. Markdown produces these tags but never contains them.
  - pattern: '<(script|link)\b[^>]*(src|href)='
    kind: regex
    weight: 8
    note: >
      The two lines that pull in the other web languages: `<script src="app.js">`
      and `<link rel="stylesheet" href="styles.css">`. XML has no concept of either.
      A file containing these is the page, not the code and not the styling.
  - pattern: '<(br|hr|img|input|meta)\b'
    kind: regex
    weight: 7
    note: >
      Tags that never close. HTML permits `<br>` on its own; XML requires every tag
      to close and would write `<br/>`, and so does JSX inside a React file. A bare
      unclosed `<br>` is HTML.
  - pattern: '\bclass\s*=\s*"'
    kind: regex
    weight: 6
    note: >
      Attributes are `name="value"` with an equals sign and quotes. The word `class`
      is the giveaway against JSX, which is forced to write `className=` because
      `class` is a reserved word in JavaScript.
  - pattern: '&[a-z]+;'
    kind: regex
    weight: 6
    note: >
      An entity such as `&amp;` or `&nbsp;`, which is how a literal angle bracket or
      a non-breaking space gets written. XML shares the five basic ones and has no
      `&nbsp;`. No programming language in this deck uses them at all.
  - pattern: '</\w+>'
    kind: regex
    weight: 6
    note: >
      A closing tag with a slash after the opening bracket. This is the family
      signature shared with XML and JSX, so it puts you in the right neighborhood
      and settles nothing on its own.
  - pattern: '<!--'
    kind: regex
    weight: 4
    note: >
      Comments are `<!-- like this -->`, with no single-line form. JavaScript uses
      `//`, CSS uses `/* */`, Python uses `#`. XML uses the same `<!--`, so this is
      family-level evidence.

rules_out:
  - pattern: '<\?xml'
    kind: regex
    because: XML, which uses the same angle brackets and a completely different opening line.
  - pattern: '<\?php'
    kind: regex
    because: PHP, which generates HTML and is not HTML.
  - pattern: 'className='
    kind: regex
    because: JSX inside a JavaScript or TypeScript file, where `class` is unavailable.
  - pattern: '\{%'
    kind: regex
    because: A template language such as Jinja, Django, or Liquid, which fills HTML in before sending it.
  - pattern: '@media'
    kind: regex
    because: CSS, which never uses angle brackets and always uses braces.
  - pattern: '\bfunction\s+\w+\s*\('
    kind: regex
    because: JavaScript. It can sit inside a `<script>` block in an HTML file, so weigh this one lightly.

project_fingerprint:
  manifests:
    - file: index.html
      decisive: false
      note: >
        The file a web server hands out when nobody asks for anything specific. It
        marks the folder as a website, and it is not decisive about the project,
        because a React or Vue project has one too and does almost nothing in it.
    - file: '*.html'
      decisive: false
      note: >
        A folder of these and nothing else is a plain static site with no build
        step. That is the simplest thing on the web and it still works.
    - file: package.json
      decisive: false
      note: >
        Present means the site is built by tooling and the HTML you see may be
        generated. Absent means the files on disk are the files the browser gets.
  lockfiles: []
  build_dirs: [dist/, build/, _site/, .next/, out/]
  entry_points: [index.html, public/index.html, src/index.html]

shape:
  blocks: tags
  statement_end: none
  comment_line: '<!-- -->'
  comment_block: '<!-- -->'
  string_quotes: >
    Double quotes around attribute values by convention. Single quotes work and
    look wrong to everyone.
  naming: lowercase tags, kebab-case for id and class values, kebab-case for data attributes
  import_keyword: none, it pulls files in with `<script src>` and `<link href>`

tooling:
  package_manager: none
  registry: none
  runtime: any web browser
  install_command: none, it is text a browser reads
  run_command: open the file in a browser, or serve the folder with a dev server
  test_command: none, though validator.w3.org will check it for structural mistakes

confusable_with:
  - language: xml
    settle_it: >
      Both are angle-bracket tags. HTML uses a fixed vocabulary you can recognize:
      `<div>`, `<body>`, `<a href>`. XML invents tag names for whatever it happens
      to describe. Check the first line: `<!DOCTYPE html>` is HTML,
      `<?xml version="1.0"?>` is XML.
    tiebreak: { pattern: '<\?xml', kind: regex, favors: xml }
  - language: javascript
    settle_it: >
      A React file mixes tags into JavaScript and looks like HTML from a distance.
      It writes `className=` instead of `class=`, wraps values in braces
      (`href={url}`), and sits in a `.jsx` or `.tsx` file with `import` at the top.
      Real HTML has no braces and no `import`.
    tiebreak: { pattern: '(className=|=\{\w+\})', kind: regex, favors: javascript }
  - language: markdown
    settle_it: >
      Markdown is the shorthand that becomes HTML, and it allows raw HTML inside
      itself, so a file can contain both. Lines starting with `#`, `-`, or `>` and
      links written as `[text](url)` are Markdown. Angle brackets around every
      element are HTML.
    tiebreak: { pattern: '^#{1,6}\s', kind: regex, favors: markdown }
  - language: css
    settle_it: >
      They travel together and look nothing alike. HTML is angle brackets and
      attributes. CSS is a selector followed by braces with `property: value;`
      pairs inside. If the file has no `<` anywhere, it is not HTML.
    tiebreak: { pattern: '^\s*[.#][\w-]+\s*\{', kind: regex, favors: css }

errors_look_like:
  sample: |
    Error: End tag "div" seen, but there were open elements.
    From line 24, column 3; to line 24, column 8

    GET http://localhost:5173/app.js
    Failed to load resource: the server responded with a status of 404 (Not Found)
  recognize_by: >
    HTML has no errors in the ordinary sense, which is the single most important
    thing to know about it. A browser given broken markup guesses, silently fixes
    it, and renders something. The page looks wrong and nothing is printed
    anywhere. The two places a message does appear: the validator at
    validator.w3.org, which talks about open elements and stray end tags, and the
    browser console, which reports a 404 when a `<script src>` or `<link href>`
    points at a file that is not there.
  patterns:
    - 'End tag .* seen, but there were open elements'
    - 'Stray end tag'
    - 'Element .* not allowed as child of element'
    - 'Failed to load resource.*404'
    - 'Unclosed element'

meet_it_when: >
  Any time you open a web page and choose View Source. An agent produces it for
  every user interface that runs in a browser, and it is the file you edit when
  the words on a page are wrong, the button is in the wrong place, or an image is
  missing.

what_agents_get_wrong: >
  Generated markup looks correct in a browser and is unusable to anyone without a
  mouse and working eyes, and none of it shows up as an error, which is why it
  survives review. Four specific things to search a diff for. An `<img>` with no
  `alt` attribute, which leaves a screen reader with nothing to announce. An
  `<input>` with no matching `<label>`, which makes the field unlabeled for
  everyone who cannot see where it sits on screen. A `<div onclick=...>` where a
  `<button>` belongs, because a div cannot be reached with the Tab key or pressed
  with the space bar. And `<html>` with no `lang="en"`, which leaves screen readers
  guessing at pronunciation. Agents also nest `<div>` six deep where `<header>`,
  `<nav>`, `<main>`, and `<footer>` would say the same thing and tell assistive
  software what each region is for.

version_landscape: >
  The current version is called HTML5 and has been current since 2014. It is a
  living standard now, meaning it gains features without changing its number, so
  there is no version cliff to fall off. Genuinely old material shows itself with
  `<font>`, `<center>`, tables used for page layout, and a doctype line four lines
  long. Anything using those is at least fifteen years out of date.

see_also:
  - css
  - javascript
  - xml
  - markdown
  - j6-web-basics
  - j1-how-to-recognize-a-language

keywords: [markup, tag, element, attribute, doctype, semantic html, accessibility, alt text, view source]
---

HTML (HyperText Markup Language) is the structure of a web page: what is a
heading, what is a paragraph, what is a button, in what order.

It is not a programming language. Nobody tells beginners this and it matters. HTML
has no variables, no conditions, no loops, and no way to make a decision. It cannot
add two numbers. It describes a document, and a browser reads the description and
draws it. When an agent says it "wrote some HTML", it changed what is on the page,
not what the program does.

## The shape

Everything is an element wrapped in tags: an opening `<p>`, some content, a closing
`</p>`. Elements nest inside each other and the nesting is the whole structure.
Attributes live in the opening tag as `name="value"`.

Indentation means nothing. Line breaks mean nothing. A page written on one enormous
line renders identically, which is exactly what the minified files in a `dist/`
folder are.

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Nyx's shop</title>
    <link rel="stylesheet" href="styles.css" />
  </head>
  <body>
    <h1 class="page-title">Candles</h1>
    <img src="candle.png" alt="A lit beeswax candle" />
    <button id="buy">Buy one</button>
    <script src="app.js"></script>
  </body>
</html>
```

Comments are `<!-- like this -->`. There is no single-line form.

## The three web languages, and the division of labor

HTML is structure. CSS (Cascading Style Sheets) is how that structure looks: color,
spacing, size, layout. JavaScript is what happens when you click the button. All
three usually live in the same folder, and a change described as "front-end work"
often touches all three at once.

The two lines in the snippet above are where they meet. `<link rel="stylesheet">`
pulls in the CSS. `<script src>` pulls in the JavaScript. Delete either line and the
page still loads, just unstyled or inert, which is a fast way to find out which file
is causing a problem.

## The three things it gets mistaken for

XML (Extensible Markup Language) looks identical at a glance and is a different
thing: a format for describing data, with tag names invented per file. HTML opens
with `<!DOCTYPE html>`, XML opens with `<?xml version="1.0"?>`.

JSX (JavaScript XML) is HTML-shaped syntax written inside a React component. It
lives in a `.jsx` or `.tsx` file, writes `className=` where HTML writes `class=`,
and puts values in braces as `href={url}`. If there is an `import` line at the top,
it is JavaScript or TypeScript, not HTML.

Template languages such as Jinja and Handlebars are HTML with holes in it. Look for
`{{ name }}` or `{% if %}`, which HTML has no meaning for.

## What the browser does with it

The browser parses the file into the DOM (Document Object Model), a live tree it
keeps in memory. JavaScript then edits that tree, which is why the page you inspect
in developer tools frequently differs from the file on disk. Press F12 and open the
Elements tab to see the tree as it currently stands, including everything the
browser silently repaired on the way in.

## Finding the part you want to change

You rarely need to read a whole page. Right-click the thing on screen that is
wrong and choose Inspect. The Elements panel opens with that exact element
highlighted, and its tags and attributes are visible.

Copy the `id` or `class` value from it, then search your project folder for that
text. That is the line to change, or the line to hand an agent. Naming the element
this way is far more useful than describing it, because "the blue button near the
top" matches four things and `class="checkout-cta"` matches one.

Two cautions when you do this. What Inspect shows you is the live tree, so if
JavaScript built that element the text will not exist anywhere in the `.html` file
and you will need to search the `.js` or `.tsx` files instead. And edits typed
directly into the Elements panel change the page in front of you and nothing on
disk, so they vanish on reload. That is useful for trying something out and is
never a fix.
