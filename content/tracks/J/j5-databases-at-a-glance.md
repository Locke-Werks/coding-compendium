---
id: j5-databases-at-a-glance
title: Databases, at a recognition level
type: section
track: J
order: 50
verified: 2026-08-02
volatility: low
danger: >
  `DROP TABLE` deletes a table and every row in it, and a migration containing
  one runs without asking. `DELETE` with no `WHERE` clause empties a table just
  as completely. Neither uses a Recycle Bin. Before running either, run the
  `SELECT` version of the same condition to count the rows you are about to
  lose, and take a backup before anything that changes the shape of a live
  database.
answer: >
  A database is a program that stores rows in named tables and answers questions
  about them. A migration is a committed file that changes the shape of those
  tables, and running one against real data is the part with no undo.
owns:
  - table
  - row
  - query
  - SQL vs NoSQL
  - migration
see_also:
  - sql
  - j6-web-basics
  - g6-secrets-and-what-never-to-commit
  - j1-how-to-recognize-a-language
  - i1-what-deployment-means
  - j3-project-layouts
keywords:
  - database basics
  - what is a table
  - postgres vs mongodb
  - schema
  - migration
  - sqlite
  - orm
---

## More

A database is a program whose whole job is to hold data safely and answer questions about it
quickly. Your code asks, the database answers. That separation is the point: your program
can crash, restart, and be replaced, and the data is still there.

The vocabulary, which is smaller than it looks:

- A **table** is one kind of thing the system stores. `users`. `orders`. Think of a
  spreadsheet tab.
- A **column** is one property every row of that table has, with a fixed type. `email`,
  which is text. `created_at`, which is a timestamp.
- A **row** is one of the things. One user. One order.
- A **primary key** is the column that uniquely names a row, usually called `id`.
- A **foreign key** is a column holding another table's key, which is how an order knows
  which user placed it. This is the "relational" in relational database.
- A **query** is a question, written in SQL (Structured Query Language). `SELECT` asks,
  `INSERT` adds, `UPDATE` changes, `DELETE` removes. The [SQL](#sql) card covers the syntax.
- A **schema** is the shape: which tables exist, which columns they have, which types.
- An **index** is a lookup shortcut that makes one kind of question fast. Missing indexes are
  the usual reason a page that was quick with fifty rows is slow with fifty thousand.

Two families exist. **Relational** databases (PostgreSQL, MySQL, SQLite, SQL Server) store
tables with a fixed schema and speak SQL. **Document** databases, usually called NoSQL
(MongoDB, DynamoDB, Firestore), store records shaped like JSON (JavaScript Object Notation)
with no fixed schema and speak their own query languages.

Start relational unless you have a specific reason, and the reason is almost never
performance. A fixed schema is the database refusing to store nonsense, which is a feature
you want on the day your code has a bug.

The last word: a **migration** is a file, committed to the repository, that changes the
schema. Adding a column, renaming one, creating a table. They run in order, and their order
is what keeps every copy of the database in the same shape.

## Full

### Where the database actually is

Three arrangements, and telling them apart tells you what you are dealing with.

**A file on disk.** SQLite is a single `.db` or `.sqlite` file with no server, no login, and
no configuration. Delete the file and the data is gone. Copy it and you have copied the
database. This app uses one: the entire compiled content of every card is a SQLite file
sitting inside the installed program.

**A server on your machine.** PostgreSQL or MySQL running locally, usually in a container,
reachable at something like `localhost:5432` ([c6](#c6-ports-and-localhost)).

**A server somebody else runs.** A managed database at a hosting provider. Same thing, with
backups and a bill ([i2](#i2-servers-and-hosting)).

### The connection string, and why it is a secret

Every database that is not a file is reached through one line of text:

```text
postgresql://appuser:hunter2@db.example.com:5432/production
```

Username, password, host, port, database name, all in one string. That is a password with
extra fields around it. It belongs in an environment variable
([g5](#g5-environment-variables)), never in the code, never in a commit, and never pasted
into a chat window ([g6](#g6-secrets-and-what-never-to-commit)). Leaking one gives away
every row you have.

### ORMs, and why your project may contain no SQL at all

An ORM (Object-Relational Mapper) is a library that lets you write your own language and get
SQL out the other end. Prisma and Drizzle in TypeScript, SQLAlchemy in Python, Diesel in
Rust, Entity Framework in C#.

```typescript
const user = await db.user.findFirst({ where: { email: "ada@example.com" } })
```

That produces a `SELECT` statement you never see. Two things follow. First, when you go
looking for the queries in a repository and find none, look for one of these library names
in the manifest. Second, the generated query is real and can be slow or wrong, and every
ORM has a way to print what it sent. Finding that switch is worth ten minutes on the day
something is mysteriously slow.

The schema also usually lives in the ORM's own file, such as `prisma/schema.prisma`, rather
than in SQL. That file is the source of truth, and the migrations are generated from it.

### Migrations, in the detail that matters

A migration is a numbered, committed file describing one change to the schema.

```text
migrations/
  0001_create_users.sql
  0002_add_email_index.sql
  0003_add_orders_table.sql
```

The numbers are the order. The database records which ones it has already applied, so
running the migration command applies only the new ones. That record is why the same command
is safe to run repeatedly and why editing an already-applied migration file does nothing:
the database has ticked it off and will not look at it again. Change the schema by adding
`0004`, never by editing `0003`.

Some tools write each migration as a pair: an `up` that makes the change and a `down` that
reverses it. The `down` is real and useful in development. In production it is a good
intention, because a `down` that removes a column cannot bring back the data that was in it.

Migrations are committed, reviewed, and run as part of deploying
([i1](#i1-what-deployment-means)). They are the reason a fresh copy of the project can build
a correct empty database from nothing.

### The part with no undo

Code is reversible. You can revert a commit and redeploy the old version in two minutes.
Data is not. A migration that drops a column deletes what was in it, permanently, in the
same second it runs.

Three habits, and they are cheap:

- Read every migration before it runs against anything real. This is a short file and you
  can read all of it.
- Take a backup before any migration that removes or renames something. Managed databases
  offer this as one button.
- Prefer additive changes. Add the new column, write to both for a while, stop reading the
  old one, and drop it in a later release once you are sure. Slower, and it has no bad day.

### What agents get wrong here

Asked to rename a field, an agent will frequently generate a migration that drops the old
column and creates a new one. That is technically a rename and it deletes every existing
value. The tell in a diff is the word `DROP` or `remove` inside a migration file, and it is
worth a hard stop every single time.

The other one: an agent asked to fix a slow query will often propose adding an index, which
is usually correct, and will occasionally propose changing the query so it returns different
rows, which is not. Check that the result set is meant to be the same.

### Finding the database in a repository you did not write

Look for these, in order: a `migrations` or `db` folder, a `schema.sql` or `schema.prisma`
file, a `DATABASE_URL` entry in `.env.example`, and a database service in
`docker-compose.yml`. Any one of them tells you which database, and the migrations folder
tells you the whole history of the schema in the order it was built
([j4](#j4-reading-a-repo-you-did-not-write)).
