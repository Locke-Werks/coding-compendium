---
id: graphql
title: GraphQL
type: language
verified: 2026-08-02
volatility: low

name: GraphQL
aka: [gql, graph query language, apollo]
family: query
likelihood: possible
extensions: ['.graphql', '.gql']

tells:
  - pattern: '(query|mutation|subscription)\s+\w*\s*[({]'
    kind: regex
    weight: 9
    note: >
      A request opens with `query`, `mutation`, or `subscription` and then a
      brace. SQL, the other query language here, opens with `SELECT` and has a
      `FROM` clause. JSON has no keyword before its brace at all.
  - pattern: '^type\s+\w+\s*\{'
    kind: regex
    weight: 8
    note: >
      A schema file declares `type User { id: ID! }`. TypeScript writes
      `type User = { ... }` with an equals sign, or `interface User {`.
  - pattern: ':\s*\w+!'
    kind: regex
    weight: 8
    note: >
      An exclamation mark after a type means the field can never be null:
      `id: ID!`. TypeScript marks the opposite case, with a `?` for optional.
  - pattern: '\.\.\.\w+'
    kind: regex
    weight: 6
    note: >
      A fragment spread, `...UserFields`, pulls in a named block of fields.
      JavaScript uses `...` too, but on arrays and objects inside code, never
      alone on a line followed by a bare name.
  - pattern: '^\s*\w+\s*$'
    kind: regex
    weight: 3
    note: >
      Selections are bare field names with no values and no commas, so the block
      reads like JSON with everything after the colons deleted. JSON always has
      quoted keys and commas.

rules_out:
  - pattern: 'SELECT'
    kind: token
    because: >
      SQL. GraphQL has no `SELECT` and no `FROM`.
  - pattern: 'function'
    kind: token
    because: >
      JavaScript. The resolver code around a query is not the query.
  - pattern: '^\s*"[^"]+":'
    kind: regex
    because: >
      JSON. Quoted keys with colons mean data, not a GraphQL document.

project_fingerprint:
  manifests:
    - file: 'schema.graphql'
      decisive: true
      note: >
        The contract: every type the server exposes. Read this first, because a
        query that asks for anything not in here fails.
    - file: '*.gql'
      note: >
        The same thing with the short extension. Some tools generate one and some
        the other.
    - file: 'codegen.ts'
      note: >
        A GraphQL Code Generator config. Its presence means TypeScript types in
        the project are generated from the schema and should not be hand-edited.
    - file: 'apollo.config.js'
      note: >
        Apollo client or server tooling, so the project talks GraphQL even if you
        have not found a `.graphql` file yet.
  build_dirs: ['generated/', 'src/gql/']
  entry_points: ['schema.graphql']

shape:
  blocks: braces
  statement_end: none
  comment_line: '#'
  string_quotes: >
    Double quotes for values, triple double quotes for the descriptions that show
    up in documentation.
  naming: camelCase fields, PascalCase types, SCREAMING_SNAKE enum values
  import_keyword: >
    None in the language itself. Tools bolt on a `#import` comment.

confusable_with:
  - language: json
    settle_it: >
      JSON has quoted keys, colons, values, and commas everywhere. A GraphQL
      query has bare names, no commas, and no values, and it starts with the word
      `query` or `{`.
    tiebreak: { pattern: '^\s*"[^"]+":', kind: regex, favors: json }
  - language: sql
    settle_it: >
      Both fetch data. GraphQL nests braces and names fields. SQL is flat clauses
      in a fixed order and always has `FROM`.
    tiebreak: { pattern: 'FROM', kind: token, favors: sql }

errors_look_like:
  sample: |
    {"errors":[{"message":"Cannot query field \"emial\" on type \"User\". Did you mean \"email\"?","locations":[{"line":3,"column":5}]}],"data":null}
  recognize_by: >
    The error arrives as JSON with an `errors` array and a `locations` list giving
    line and column. The status code is usually 200, because the request itself
    succeeded and only the query failed.
  patterns:
    - '"errors":\s*\['
    - 'Cannot query field'
    - '"locations":\s*\[\{"line"'

meet_it_when: >
  You point an agent at a service whose interface is GraphQL: Shopify, Hasura,
  Linear, or GitHub, which offers a GraphQL version of its interface alongside the
  plain one. You will read it far more often than you write it.

what_agents_get_wrong: >
  Agents invent fields. A query asking for `user { avatarUrl }` looks correct and
  fails at runtime if the schema calls it `avatar`, because nothing checks the
  query against the schema until the server sees it. The trap on top of that:
  a failed GraphQL request usually returns HTTP status 200 with an `errors` array
  in the body, so code that checks the status code reports success while the data
  is null. Ask for the schema file, check the field names against it, and make
  sure error handling reads the body rather than the status.

see_also:
  - sql
  - json
  - typescript
  - j6-web-basics

keywords: [gql, apollo, schema, resolver, mutation, query language]
---

A query language for interfaces where the caller picks the fields. One request,
one round trip, and you get back exactly the shape you asked for. The server
publishes a schema listing every available type and field, and anything not in the
schema is rejected.

Two kinds of file look similar and do opposite jobs. A schema defines what exists.
A query asks for some of it.

```graphql
# the query: bare field names, no commas, no values
query GetUser {
  user(id: "42") {
    name
    email
    orders(last: 3) {
      total
    }
  }
}
```

The response comes back as JSON (JavaScript Object Notation) with the same
nesting, which is the point of the design. If you are looking at a block of braces and cannot tell which language it
is, check for values: GraphQL has none on the left-hand side, only names. See
[JSON](#json) for the data that comes back.
