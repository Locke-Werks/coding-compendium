---
id: protobuf
title: Protocol Buffers
type: language
verified: 2026-08-02
volatility: low

name: Protocol Buffers
aka: [protobuf, proto, proto3, grpc, pb]
family: config
likelihood: possible
extensions: ['.proto']

danger: >
  Changing or reusing a field's number is destructive in a way no compiler will
  stop. The number, not the name, is what travels on the wire, so a renumbered
  field makes every program still running the old code read the wrong value out of
  saved data and live messages. Delete a field by marking it `reserved 4;`
  instead, which permanently blocks that number from being handed to something
  else.

tells:
  - pattern: '^syntax\s*=\s*"proto[23]"'
    kind: regex
    weight: 10
    note: >
      Almost every file opens with this line. No other language in the deck starts
      with an assignment to a bare word called `syntax`.
  - pattern: '^message\s+\w+\s*\{'
    kind: regex
    weight: 9
    note: >
      A record type is a `message`. Rust and Go say `struct`, TypeScript says
      `interface`, SQL says `CREATE TABLE`.
  - pattern: '^\s*(repeated\s+)?[\w.]+\s+\w+\s*=\s*\d+;'
    kind: regex
    weight: 9
    note: >
      Every field ends with `= <number>;`. That number is the field's identity in
      the transmitted bytes. No other language numbers its fields, and no other
      language cares if you renumber them.
  - pattern: 'repeated'
    kind: token
    weight: 8
    note: >
      Marks a list: `repeated string tags = 3;`. TypeScript writes `string[]`,
      Rust writes `Vec<String>`, Java writes `List<String>`.
  - pattern: '\brpc\s+\w+\s*\('
    kind: regex
    weight: 8
    note: >
      Inside a `service` block, `rpc SayHello (Req) returns (Res);` defines one
      callable method. The word `returns` on the same line as `rpc` appears
      nowhere else.

rules_out:
  - pattern: 'interface'
    kind: token
    because: >
      TypeScript, Java, or Go. Protocol Buffers use `message` and `service`.
  - pattern: 'struct'
    kind: token
    because: >
      Rust, Go, or C. Same idea, different language.
  - pattern: '^\s*"[^"]+":'
    kind: regex
    because: >
      JSON. That is data in the shape a `.proto` describes, not the description.

project_fingerprint:
  manifests:
    - file: '*.proto'
      decisive: true
      note: >
        The definition of a message format, shared by every program that speaks
        it. Usually the only hand-written file in the chain.
    - file: 'buf.yaml'
      note: >
        Config for buf, the modern tool that lints `.proto` files and checks
        whether a change breaks existing clients.
    - file: 'protos/'
      note: >
        A folder of `.proto` files, often shared across several services or
        published as its own repository.
  build_dirs: ['gen/', 'generated/']
  entry_points: ['*.proto']

shape:
  blocks: braces
  statement_end: semicolon
  comment_line: '//'
  comment_block: '/* */'
  string_quotes: >
    Double quotes, and they appear in only a few places: the syntax line, imports,
    and option values.
  naming: PascalCase messages and services, snake_case fields, SCREAMING_SNAKE enum values
  import_keyword: import

confusable_with:
  - language: json
    settle_it: >
      A `.proto` file describes a shape; JSON is one instance of that shape filled
      in. Protocol Buffers have types before names, field numbers, and semicolons.
      JSON has quoted keys, colons, and real values.
    tiebreak: { pattern: '=\s*\d+;', kind: regex, favors: protobuf }
  - language: typescript
    settle_it: >
      Both declare the shape of data with braces. TypeScript writes
      `name: string;` with the type after a colon. Protocol Buffers write
      `string name = 1;` with the type first and a number at the end.
    tiebreak: { pattern: ':\s*(string|number|boolean)', kind: regex, favors: typescript }

errors_look_like:
  sample: |
    user.proto:7:3: Expected "required", "optional", or "repeated".
    user.proto:12:1: Field number 3 has already been used in "User" by field "email".
  recognize_by: >
    The compiler is `protoc`, and it reports `file:line:column:` followed by a
    sentence. Complaints about a field number that is already in use belong to no
    other language.
  patterns:
    - '\.proto:\d+:\d+:'
    - 'Field number \d+ has already been used'
    - 'Expected "required", "optional", or "repeated"'

meet_it_when: >
  A project talks to another service over gRPC, or stores data in a compact binary
  format. You meet the `.proto` file when a field needs adding and you meet the
  generated code, which you never edit, everywhere else.

what_agents_get_wrong: >
  Agents renumber fields. Editing a message to add something in the middle and
  shifting the numbers below it looks tidy in a diff and silently corrupts every
  message written by code that has not been redeployed. Field numbers are
  permanent; deleted fields get `reserved` rather than reused. The second miss is
  procedural: a `.proto` change does nothing until the generator runs, so a diff
  that touches only the `.proto` and no generated file usually means the build is
  still using the old shape. Check both in the same commit.

see_also:
  - json
  - typescript
  - j2-the-config-formats-nobody-explains

keywords: [protobuf, proto3, grpc, protoc, buf, serialization, wire format]
---

A way to describe the shape of a message once, then generate matching code for
every language that has to read or write it. The `.proto` file is the contract; the
code in your project is generated from it and should never be edited by hand.

The point is size and speed. The same data that takes 200 bytes as text takes a
fraction of that as Protocol Buffers, because field names never travel. Only the
numbers do.

```proto
syntax = "proto3";

message User {
  string email = 1;      // 1 is this field's permanent identity
  int32  age   = 2;
  repeated string tags = 3;
}
```

Those numbers are the whole trick and the whole hazard. Reading a saved message
means matching number 1 to `email`, so changing 1 to 4 later means every program
still running the old code reads the wrong thing. Add new fields with new numbers,
never recycle an old one.
