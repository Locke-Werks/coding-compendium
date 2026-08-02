---
id: sql
title: SQL
type: language
verified: 2026-08-02
volatility: low

name: SQL
aka: [structured query language, sequel, postgres, postgresql, mysql, sqlite, t-sql, tsql, plsql]
family: query
likelihood: likely
extensions: ['.sql']

danger: >
  `DROP TABLE orders` deletes the table and every row in it. `DROP DATABASE`
  deletes all of them at once. `DELETE FROM orders` with no `WHERE` clause empties
  the table just as completely. None of these go to a Recycle Bin and none of them
  can be undone once committed. Before any of them, run the `SELECT` version of
  the same statement to see exactly which rows you are about to lose, and take a
  backup before anything starting with `DROP`. Inside a transaction you can
  `ROLLBACK`, which is the closest thing to an undo this language has.

# Notes contrast against the neighbors she will actually confuse it with: the
# other query language in the deck, and the host language it is embedded in.
tells:
  - pattern: 'SELECT\s+[\s\S]{0,200}?\bFROM\b'
    kind: regex
    weight: 9
    note: >
      `SELECT` and `FROM` in the same statement is the signature of the language.
      GraphQL, the other query language in this deck, has neither word: it opens
      with `query {` and lists bare field names.
  - pattern: 'CREATE TABLE'
    kind: regex
    weight: 9
    note: >
      Defines a table and its columns. Protocol Buffers say `message`, TypeScript
      says `interface`, Rust says `struct`. Only a database says `CREATE TABLE`.
  - pattern: 'INSERT INTO'
    kind: regex
    weight: 8
    note: >
      Adds rows. The pairing of two uppercase English words as one command is
      normal here and looks wrong everywhere else.
  - pattern: 'JOIN\b[\s\S]{0,120}?\bON\b'
    kind: regex
    weight: 8
    note: >
      `LEFT JOIN orders ON orders.user_id = users.id` stitches two tables
      together. No general-purpose language uses `JOIN` and `ON` as keywords.
  - pattern: 'IS NULL'
    kind: regex
    weight: 6
    note: >
      Missing values are tested with `IS NULL`, never `= NULL`, because comparing
      to nothing gives neither true nor false. Python writes `is None`, JavaScript
      writes `=== null`, Rust has no null at all.
  - pattern: 'GROUP BY'
    kind: regex
    weight: 6
    note: >
      Collapses rows into totals per group. Spreadsheets call it a pivot. No other
      language in the deck has a two-word keyword for it.
  - pattern: '--'
    kind: operator
    weight: 2
    note: >
      Two hyphens start a comment that runs to the end of the line. Lua and
      Haskell use `--` too, and C-family languages read it as a decrement, so this
      only counts once you have also seen `SELECT`.

rules_out:
  - pattern: 'def'
    kind: token
    because: >
      Python or Ruby. If the query text is quoted inside that file, the file is
      Python and the SQL is a string it sends to a database.
  - pattern: 'function'
    kind: token
    because: >
      JavaScript, TypeScript, or PHP. SQL spells it `CREATE FUNCTION`.
  - pattern: '=>'
    kind: operator
    because: >
      A JavaScript arrow function or a C# lambda. SQL has no anonymous functions.
  - pattern: '#include'
    kind: line_start
    because: >
      C or C++.
  - pattern: '$env:'
    kind: sigil
    because: >
      PowerShell. A connection string in a script, not a query.

project_fingerprint:
  manifests:
    - file: '*.sql'
      decisive: true
      note: >
        A file of statements, usually either a schema definition or one step of a
        migration history.
    - file: 'migrations/'
      note: >
        A folder of numbered or dated files like `0003_add_orders.sql`. This is
        the database's version history. You never edit a migration that has
        already run anywhere, because the database only replays new ones.
    - file: 'schema.sql'
      note: >
        The whole shape of the database in one file: every table, column, and
        index. The fastest way to understand a project's data.
    - file: 'seed.sql'
      note: >
        Sample rows for a fresh development database. Safe to run, safe to delete.
    - file: 'prisma/schema.prisma'
      note: >
        Not SQL, but it generates SQL. Its presence means the project talks to a
        database through a translator, so you may never see a query in the source.
  entry_points: ['schema.sql', 'migrations/', 'seed.sql']

shape:
  blocks: keyword
  statement_end: semicolon
  comment_line: '--'
  comment_block: '/* */'
  string_quotes: >
    Single quotes hold text values. Double quotes mean an identifier, a table or
    column name, which is the reverse of most languages. MySQL uses backticks for
    identifiers and SQL Server uses square brackets.
  naming: snake_case tables and columns, keywords conventionally uppercase, table names usually plural
  import_keyword: >
    None. Other files are pulled in by the client program, for example `\i` in
    psql or `SOURCE` in the MySQL client.

tooling:
  package_manager: >
    None. The database server is the runtime, and extensions install with SQL
    statements.
  runtime: >
    A database: PostgreSQL, MySQL, SQL Server, or SQLite, which is a single file
    on disk with no server at all.
  run_command: psql -f schema.sql, or paste it into a database client
  test_command: >
    None standard. The usual approach is to run the statement inside a transaction
    and roll it back.

confusable_with:
  - language: graphql
    settle_it: >
      Both ask a server for data. SQL is flat clauses in a fixed order and always
      contains `FROM`. GraphQL is nested braces of bare field names, opens with
      `query {`, and has no `FROM` anywhere.
    tiebreak: { pattern: 'query\s*\{', kind: regex, favors: graphql }
  - language: python
    settle_it: >
      If the uppercase words sit inside quotes and the lines around them contain
      `def`, `import`, or an `=`, the file is Python and the SQL is a string being
      handed to a database at runtime. This is where you meet SQL most often.
    tiebreak: { pattern: 'def ', kind: regex, favors: python }
  - language: typescript
    settle_it: >
      Same situation, different host. SQL inside backtick template literals, next
      to `const`, `await`, or type annotations after a colon, is TypeScript code
      holding a query.
    tiebreak: { pattern: 'const ', kind: regex, favors: typescript }

errors_look_like:
  sample: |
    ERROR:  relation "user" does not exist
    LINE 1: SELECT * FROM user;
                          ^
  recognize_by: >
    The dialect names itself in the error. PostgreSQL says `ERROR:` and then
    reprints your query under `LINE 1:` with a caret pointing at the exact
    character, and it calls a table a relation. MySQL says
    `ERROR 1146 (42S02): Table 'app.user' doesn't exist`, with a number and a
    five-character code in parentheses. SQLite says `Error: no such table: user`
    and stops. Oracle prefixes everything with `ORA-` and five digits.
  patterns:
    - '^ERROR:\s+relation ".*" does not exist'
    - '^ERROR \d{4} \([0-9A-Z]{5}\)'
    - 'no such table:'
    - 'syntax error at or near'
    - 'ORA-\d{5}'

meet_it_when: >
  An agent adds a database to your project and writes a migration file. You open
  `supabase/migrations` or `db/migrate` in a repo you cloned. Most often you meet
  it without a file at all: three uppercase words inside a quoted string in
  Python, TypeScript, or C#, being sent to a database while the program runs.

what_agents_get_wrong: >
  Three things, in order of how much they cost you. First, string concatenation
  instead of parameters. An agent writes
  `"SELECT * FROM users WHERE id = " + user_id`, or the same thing as an f-string,
  and now anyone who can influence that value can rewrite the query. This is SQL
  injection, it is the oldest bug on the web, and the fix is a placeholder: `?`,
  `$1`, or `%s`, with the values passed separately as arguments. In a diff, treat
  any quote sitting next to a `+` or an `f"` prefix as a defect until proven
  otherwise. Second, `UPDATE` or `DELETE` with no `WHERE` clause, which silently
  applies to every row in the table; a statement that touches a table and has no
  `WHERE` on the same line deserves a second look. Third, dialect drift: an agent
  writes `AUTO_INCREMENT` into a PostgreSQL migration or `GETDATE()` into SQLite,
  because it learned all the dialects at once and does not know which server you
  are running.

version_landscape: >
  SQL is a standard that every database ignores in a different place. The core is
  stable and portable: `SELECT`, `FROM`, `WHERE`, `JOIN`, `GROUP BY`, and
  `ORDER BY` work everywhere and have for decades. Everything around the edges
  differs: auto-numbering columns, string concatenation, date functions, row
  limits, quoting rules, and inserting a row that might already exist. An answer
  you copied that fails with a syntax error is usually correct SQL for the wrong
  database rather than wrong SQL. Version numbers matter much less than which
  engine you are on.

see_also:
  - graphql
  - python
  - typescript
  - j5-databases-at-a-glance
  - g6-secrets-and-what-never-to-commit
  - h3-reviewing-a-diff-you-cannot-fully-read

keywords: [query, database, postgres, mysql, sqlite, migration, table, join, injection]
---

SQL (Structured Query Language) is how you ask a database for rows, and how you
change them. It is a query language rather than a general-purpose one: you cannot
build an application in it, and it is not trying to let you. You describe the
result you want and the database works out how to get it.

Pronounced either "sequel" or spelled out. Both are correct and people argue about
it anyway.

## Where you actually meet it

Far more often inside another language than in a file of its own. A Python function
holds a three-line string of uppercase words and hands it to a database driver. A
TypeScript file does the same inside backticks. The `.sql` files in a project are
usually the schema and the migration history, not the queries the app runs day to
day.

That matters for recognition. When you paste a snippet into the identifier and it
comes back as Python, look again at the string inside it. Two languages are on your
screen, and the bug may be in the quiet one.

## The shape

Statements end with a semicolon. Keywords are written in uppercase by convention
only; the database does not care. Comments are `--` for a line and `/* */` for a
block. Names of tables and columns are conventionally snake_case.

```sql
-- everyone who signed up this month, newest first
SELECT u.id, u.email, COUNT(o.id) AS order_count
FROM users AS u
LEFT JOIN orders AS o ON o.user_id = u.id
WHERE u.created_at >= '2026-08-01'
GROUP BY u.id, u.email
ORDER BY u.created_at DESC
LIMIT 50;
```

The clauses have to appear in that order. `SELECT` picks the columns, `FROM` names
the table, `JOIN` attaches another one, `WHERE` filters rows, `GROUP BY` collapses
them, `ORDER BY` sorts, `LIMIT` cuts the list short. `AS` renames something for the
rest of the statement.

## Which dialect am I looking at

Every database implements the same core and then invents its own edges. Copied
answers fail across that line constantly, so learn to spot which one is on screen.

| If you see | You are looking at |
|---|---|
| `SERIAL`, `$1` placeholders, `now()`, `ON CONFLICT`, `ILIKE` | PostgreSQL |
| `AUTO_INCREMENT`, backticks around names, `ON DUPLICATE KEY UPDATE` | MySQL or MariaDB |
| `AUTOINCREMENT`, `INTEGER PRIMARY KEY`, no server in the setup | SQLite |
| `[square brackets]`, `TOP 10`, `GETDATE()`, `NVARCHAR`, `GO` alone on a line | SQL Server, whose dialect is called T-SQL |
| `VARCHAR2`, `SYSDATE`, `ROWNUM`, errors starting `ORA-` | Oracle |

Same query, three spellings of "give me ten rows": `LIMIT 10` in PostgreSQL, MySQL,
and SQLite, `TOP 10` in SQL Server, `FETCH FIRST 10 ROWS ONLY` in the actual
standard nobody follows.

## Before you change rows

Every `UPDATE` and `DELETE` starts life as a `SELECT`. Write the `WHERE` clause,
run it as `SELECT * FROM orders WHERE ...` first, and count the rows that come
back. If that number is what you expected, swap the front of the statement. A
`DELETE` with no `WHERE` clause empties the whole table and the database will not
ask whether you meant it.

## Reading its errors

```text
ERROR:  relation "user" does not exist
LINE 1: SELECT * FROM user;
                      ^
```

The caret points at the exact character that upset the parser, which is usually one
token past the real mistake. Here the table is called `users` and the query asked
for `user`. The word `relation` where you expected `table` tells you this is
PostgreSQL without reading anything else.

Two messages worth recognizing on sight. `syntax error at or near` means the
database did not understand the statement, and the quoted token is where it gave
up. `permission denied for table` means the statement was fine and the account you
connected with is not allowed to do that, which is a credentials problem rather
than a query problem.
