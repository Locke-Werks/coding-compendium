---
id: xml
title: XML
type: language
verified: 2026-08-02
volatility: low

name: XML
aka: [extensible markup language, csproj, pom, xaml, svg]
family: markup
likelihood: possible
extensions: ['.xml', '.csproj', '.xaml', '.svg', '.xsd', '.plist']

tells:
  - pattern: '<\?xml'
    kind: regex
    weight: 10
    note: >
      The declaration `<?xml version="1.0"?>` on the first line settles it
      outright. HTML declares itself with `<!DOCTYPE html>` instead, and no
      config format has a declaration line at all.
  - pattern: '</[A-Za-z][\w.:-]*>'
    kind: regex
    weight: 7
    note: >
      A closing tag with a slash after the angle bracket. Every element must have
      one, which is the hard rule HTML relaxes and Markdown never had.
  - pattern: '<[A-Z][\w.]*>'
    kind: regex
    weight: 6
    note: >
      Capitalized invented tag names such as `<PropertyGroup>`. HTML tag names
      come from a fixed lowercase list, so a capital letter inside a tag points
      at XML.
  - pattern: '<!\[CDATA\['
    kind: regex
    weight: 9
    note: >
      A CDATA block holds raw text the parser must not read as markup. It exists
      only in XML and in the parts of HTML that inherited it.
  - pattern: 'xmlns='
    kind: regex
    weight: 8
    note: >
      A namespace declaration, which prevents two vocabularies from colliding.
      Nothing outside the XML family has namespaces.

rules_out:
  - pattern: '<!DOCTYPE html'
    kind: regex
    because: HTML, which is a specific vocabulary rather than XML in general
  - pattern: '^\s*[\w.-]+\s*=\s*"'
    kind: regex
    because: TOML or INI, where an assignment stands alone on a line
  - pattern: '^\s*\{'
    kind: regex
    because: JSON, if the file opens with a brace

project_fingerprint:
  manifests:
    - file: '*.csproj'
      decisive: true
      note: The project file for a C# project. Lists the target framework and every package reference.
    - file: pom.xml
      decisive: true
      note: Maven's build file for a Java project. Long, nested, and always XML.
    - file: AndroidManifest.xml
      note: Declares an Android app's permissions and screens.
    - file: '*.svg'
      note: A vector image. It is XML, which is why you can open one in a text editor and read it.
    - file: web.config
      note: Configuration for a site hosted on Windows under IIS.

shape:
  blocks: tags
  statement_end: none
  comment_line: 'none, use the block form'
  comment_block: '<!-- -->'
  string_quotes: 'Attribute values need quotes, single or double. Both are accepted and double is normal.'
  naming: PascalCase element names in Microsoft tooling, lowercase-with-dashes nearly everywhere else
  import_keyword: 'none in the format, though schemas use xs:import and build files use Import'

confusable_with:
  - language: html
    settle_it: >
      HTML uses a fixed set of lowercase tag names and forgives a missing closing
      tag. XML lets you invent any tag name and refuses to parse if one is left
      open. A `<?xml` first line, or capitalized tag names, means XML.
    tiebreak: { pattern: '<!DOCTYPE html', kind: regex, favors: html }
  - language: json
    settle_it: >
      Same job, different decade. JSON writes `"name": "app"` inside braces. XML
      writes `<name>app</name>` inside tags. If the file has angle brackets it is
      XML, and if it has braces and quoted keys it is JSON.
    tiebreak: { pattern: '"[^"\n]+"\s*:', kind: regex, favors: json }

errors_look_like:
  sample: |
    XMLSyntaxError: Opening and ending tag mismatch: PropertyGroup line 3 and Project, line 8, column 11
  recognize_by: >
    The phrase "tag mismatch" or "mismatched tag", with two tag names and a line
    number for each. The parser is telling you it opened one thing and found the
    close for another.
  patterns:
    - 'Opening and ending tag mismatch'
    - 'mismatched tag'
    - 'XMLSyntaxError|SAXParseException'
    - 'not well-formed \(invalid token\)'

meet_it_when: >
  You open a `.csproj` file in a C# project, a `pom.xml` in a Java one, or an
  Android manifest. Also whenever you look inside an `.svg` image, and in older
  web services that answer in XML rather than JSON.

what_agents_get_wrong: >
  Agents drop the closing tag when they edit a nested block, and unlike YAML or
  Markdown, XML fails loudly for that, so it is the cheap failure. The expensive
  one is namespaces: an element added without the right `xmlns` parses correctly
  and is then ignored by the tool reading it, with no warning anywhere. In a
  build file such as a `.csproj`, also watch for a package version invented to
  fit, since the file is valid whether or not that version exists and the failure
  surfaces later during restore.

see_also:
  - html
  - json
  - markdown
  - j2-the-config-formats-nobody-explains

keywords: [csproj, pom.xml, svg, xmlns, cdata, tag mismatch, maven, android manifest]
---

XML (Extensible Markup Language) stores data inside angle-bracket tags. It is not a
programming language. Nothing inside a `.xml` file runs. It describes data, and some other
program reads it.

Every value sits between an opening tag and a matching closing tag, and every tag must be
closed. That is the rule HTML (HyperText Markup Language) relaxes: HTML has a fixed set of
tag names and forgives a missing `</p>`, while XML lets you invent any tag name you like
and refuses the whole file if one tag is left open.

```xml
<?xml version="1.0" encoding="utf-8"?>
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <TargetFramework>net9.0</TargetFramework>
  </PropertyGroup>
</Project>
```

Settle it against JSON (JavaScript Object Notation): braces and quoted keys are JSON, angle
brackets are XML. Settle it against HTML: a `<?xml` declaration on the first line, or tag
names with capital letters in them, means XML.

You meet it in `.csproj` files for C# projects, in Java's `pom.xml`, in Android manifests,
and in any `.svg` image you open in a text editor.
