-- Schema for content.db, the compiled corpus.
--
-- This file is applied by `compile-content`, which builds the database from
-- content/*.md. The database is a build artifact: it is never edited by hand and
-- never committed. To change what the app knows, change the markdown and rebuild.
--
-- The app opens this file READ-ONLY and ATTACHes a separate notes.db for
-- anything the user writes. Two files rather than one so that shipping an app
-- update can replace the corpus wholesale without touching the reader's notes, and so
-- that "the shipped content is unmodified" is a property of the filesystem
-- rather than a promise.

-- WAL for the build, which is write-heavy. `compile::seal` turns it back off
-- before the file ships: a WAL database creates a -shm file beside itself even
-- to be read, and the app installs somewhere it cannot write.
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;
PRAGMA foreign_keys = ON;

-- --------------------------------------------------------------------------
-- Cards
-- --------------------------------------------------------------------------

-- One row per markdown file in content/.
CREATE TABLE IF NOT EXISTS cards (
    id          TEXT PRIMARY KEY,     -- stable kebab-case id, also the filename
    type        TEXT NOT NULL,        -- section | language | error | command | intent | glossary | panic
    title       TEXT NOT NULL,
    track       TEXT,                 -- A..J, sections only
    ord         INTEGER,              -- position within a track, for the reader rail

    -- The one-sentence answer, rendered directly in the palette. Sections
    -- always have one; other card types may not, so it is nullable.
    answer      TEXT,

    -- The full markdown body, kept so the reader can render it without going
    -- back to disk. The corpus is ~1 MB of text, so storing it twice (here and
    -- in the FTS index) costs nothing worth optimizing.
    body        TEXT NOT NULL,

    -- True when `answer` was lifted from the body's opening paragraph rather
    -- than authored as its own field. Only prose sections declare one, so every
    -- other card type gets a derived answer to render in the palette. The reader
    -- checks this to avoid printing the same sentence twice, once as the callout
    -- and again as the first line of the body.
    answer_derived INTEGER NOT NULL DEFAULT 0,

    -- Extra search terms an author added deliberately: plurals, misspellings,
    -- and the wrong-but-common name for a thing. Stored as one space-joined
    -- string rather than inside `meta`, because an external-content FTS5 table
    -- maps its columns to the base table BY NAME. A column that exists only in
    -- the JSON blob cannot be indexed, and the 'rebuild' command fails outright
    -- rather than skipping it.
    keywords    TEXT NOT NULL DEFAULT '',

    volatility  TEXT NOT NULL,        -- low | quarterly | weekly
    verified    TEXT NOT NULL,        -- ISO date, drives the stale badge

    -- Everything else from the frontmatter, as JSON. Card types differ enough
    -- that giving each one its own columns would mean seven sparse tables; the
    -- structured fields that need querying are promoted out of here into their
    -- own tables below.
    meta        TEXT NOT NULL DEFAULT '{}'
) STRICT;

CREATE INDEX IF NOT EXISTS cards_by_type  ON cards(type);
CREATE INDEX IF NOT EXISTS cards_by_track ON cards(track, ord) WHERE track IS NOT NULL;

-- --------------------------------------------------------------------------
-- Lexical index
-- --------------------------------------------------------------------------

-- An external-content FTS5 table: the text lives in `cards`, and this holds
-- only the inverted index pointing back at it. Without `content=`, every card
-- body would be stored a second time inside the index.
--
-- `porter` stems words, so a search for "committing" finds "commit". `unicode61`
-- handles the tokenizing and `remove_diacritics 2` makes "café" and "cafe" match
-- each other, which matters more for a search box than purists like.
--
-- Column weights are applied at query time via bm25(), not here.
CREATE VIRTUAL TABLE IF NOT EXISTS cards_fts USING fts5(
    title,
    answer,
    body,
    keywords,
    content='cards',
    content_rowid='rowid',
    tokenize='porter unicode61 remove_diacritics 2'
);

-- --------------------------------------------------------------------------
-- Chunks and vectors
-- --------------------------------------------------------------------------

-- Cards are split into ~400-token chunks with 60 tokens of overlap for semantic
-- search. A whole card is too big to embed usefully: a 900-word section produces
-- one vector that means "this is about git", which matches everything about git
-- and distinguishes nothing.
CREATE TABLE IF NOT EXISTS chunks (
    id           INTEGER PRIMARY KEY,
    card_id      TEXT NOT NULL REFERENCES cards(id) ON DELETE CASCADE,
    ord          INTEGER NOT NULL,    -- position within the card

    -- The heading trail this chunk sits under, e.g. "How to read an error
    -- message > Full > Exit codes". Prepended to the text before embedding,
    -- because a chunk lifted from the middle of a document otherwise carries no
    -- indication of what it is about. Cheapest retrieval win available.
    heading_path TEXT NOT NULL DEFAULT '',

    text         TEXT NOT NULL
) STRICT;

CREATE INDEX IF NOT EXISTS chunks_by_card ON chunks(card_id, ord);

-- One 384-dimensional vector per chunk, stored as a raw little-endian f32 blob
-- (1536 bytes). No vector index: at roughly 6,000 chunks a brute-force cosine
-- scan is well under a millisecond, while the query encoder that produces the
-- search vector takes 2 to 6 ms. An index would optimize the part that is
-- already free.
CREATE TABLE IF NOT EXISTS chunk_vectors (
    chunk_id INTEGER PRIMARY KEY REFERENCES chunks(id) ON DELETE CASCADE,
    vec      BLOB NOT NULL
) STRICT;

-- --------------------------------------------------------------------------
-- Retrieval helpers
-- --------------------------------------------------------------------------

-- Plain-language goals mapped to the card that answers them. These exist because
-- a beginner does not know the topic word: they know they want to undo
-- something, not that the word is `revert`. Each intent carries many phrasings,
-- including the panicked and ungrammatical ones, and all of them are indexed.
CREATE TABLE IF NOT EXISTS intents (
    id       TEXT PRIMARY KEY,
    goal     TEXT NOT NULL,
    target   TEXT NOT NULL REFERENCES cards(id),
    urgency  TEXT NOT NULL DEFAULT 'calm'   -- calm | stuck | panic
) STRICT;

CREATE VIRTUAL TABLE IF NOT EXISTS intents_fts USING fts5(
    goal,
    phrasings,
    content='',                    -- contentless: phrasings have no base table
    tokenize='porter unicode61 remove_diacritics 2'
);

-- Hand-authored vocabulary bridge, separate from intents because it maps single
-- words rather than whole goals. "oops" -> "undo", "wrecked" -> "reset".
CREATE TABLE IF NOT EXISTS aliases (
    term      TEXT NOT NULL,
    canonical TEXT NOT NULL,
    PRIMARY KEY (term, canonical)
) STRICT;

-- Glossary terms, promoted out of `meta` because the reader looks them up on
-- every hover and the build script uses them to auto-link first occurrences.
CREATE TABLE IF NOT EXISTS glossary (
    id           TEXT PRIMARY KEY REFERENCES cards(id) ON DELETE CASCADE,
    term         TEXT NOT NULL,
    expansion    TEXT,
    short_def    TEXT NOT NULL
) STRICT;

CREATE INDEX IF NOT EXISTS glossary_by_term ON glossary(term COLLATE NOCASE);

-- --------------------------------------------------------------------------
-- Identifier scoring table
-- --------------------------------------------------------------------------

-- Compiled from the `tells` and `rules_out` fields on the language cards, so the
-- cards stay the single source of truth for both the prose the reader sees and the
-- classifier that guesses on their behalf. Editing a tell in the markdown changes the
-- classifier on the next build, which is the property that keeps the two from
-- drifting apart.
CREATE TABLE IF NOT EXISTS language_signals (
    language_id TEXT NOT NULL REFERENCES cards(id) ON DELETE CASCADE,
    pattern     TEXT NOT NULL,
    kind        TEXT NOT NULL,       -- token | regex | operator | sigil | line_start
    weight      REAL NOT NULL,       -- negative for rules_out
    note        TEXT NOT NULL DEFAULT ''
) STRICT;

CREATE INDEX IF NOT EXISTS signals_by_language ON language_signals(language_id);

-- Files that identify a project language on sight. Feeds both the identifier and
-- Project Scan.
CREATE TABLE IF NOT EXISTS project_manifests (
    language_id TEXT NOT NULL REFERENCES cards(id) ON DELETE CASCADE,
    file        TEXT NOT NULL,
    decisive    INTEGER NOT NULL DEFAULT 0,
    note        TEXT NOT NULL DEFAULT ''
) STRICT;

-- Machine-readable form of each card's `settle_it` prose. Consulted only when
-- two languages score within the ambiguity band, where weights alone are not
-- trustworthy and a single decisive token is.
CREATE TABLE IF NOT EXISTS language_tiebreaks (
    language_id TEXT NOT NULL REFERENCES cards(id) ON DELETE CASCADE,
    versus      TEXT NOT NULL,
    pattern     TEXT NOT NULL,
    kind        TEXT NOT NULL,
    favors      TEXT NOT NULL
) STRICT;

-- Regexes that route a pasted error to the card explaining it.
--
-- This is what closes the loop on the identifier. Telling the reader "this is a Python
-- crash" is half an answer; the half they wanted is what the error means and what
-- to try first. Compiled from the `patterns` field on every error card.
CREATE TABLE IF NOT EXISTS error_patterns (
    card_id TEXT NOT NULL REFERENCES cards(id) ON DELETE CASCADE,
    pattern TEXT NOT NULL
) STRICT;

CREATE INDEX IF NOT EXISTS error_patterns_by_card ON error_patterns(card_id);

-- --------------------------------------------------------------------------
-- Build provenance
-- --------------------------------------------------------------------------

-- Written by compile-content, read by the app at startup.
--
-- `embed_model_sha256` is load-bearing. The compiler and the app must use
-- identical model bytes or their vectors live in different spaces and semantic
-- search returns confident nonsense with no error anywhere. The app compares
-- this against the model it loaded and hard-fails semantic search on a mismatch
-- rather than silently mixing them.
CREATE TABLE IF NOT EXISTS build_meta (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
) STRICT;
