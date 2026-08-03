//! Compiling `content/*.md` into the SQLite database the app searches.
//!
//! The markdown is the source of truth. This database is a build artifact: it is
//! never edited by hand, never committed, and is thrown away and rebuilt whenever
//! the content changes.
//!
//! Two things happen here that are worth understanding, because both are places
//! where a subtle mistake produces no error and quietly worse search.
//!
//! **Chunking.** A card is split into overlapping pieces before embedding,
//! because one vector for a 900-word section means "this is about git", which
//! matches every git question and distinguishes none of them.
//!
//! **Heading paths.** Each chunk is prefixed with the trail of headings it sits
//! under. A chunk lifted from the middle of a document otherwise carries no
//! indication of its subject, and the model has no way to recover it.

mod chunk;
mod frontmatter;
mod languages;

pub use chunk::{chunk_card, Chunk, CHUNK_OVERLAP_TOKENS, CHUNK_TARGET_TOKENS};
pub use frontmatter::{split_frontmatter, Card, CardKind};
pub use languages::{write_language_signals, LanguageStats};

use anyhow::{bail, Context, Result};
use rusqlite::Connection;
use std::path::Path;
use walkdir::WalkDir;

/// Read every card under `content_dir`.
///
/// `_meta` is skipped: it holds YAML control files (the frozen id lists, the
/// assumed-acronym list) rather than cards.
pub fn load_cards(content_dir: &Path) -> Result<Vec<Card>> {
    let mut cards = Vec::new();

    for entry in WalkDir::new(content_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() || path.extension().is_none_or(|e| e != "md") {
            continue;
        }
        if path.components().any(|c| c.as_os_str() == "_meta") {
            continue;
        }

        let raw = std::fs::read_to_string(path)
            .with_context(|| format!("reading {}", path.display()))?;
        let card = Card::parse(&raw, path)
            .with_context(|| format!("parsing {}", path.display()))?;
        cards.push(card);
    }

    if cards.is_empty() {
        bail!(
            "no cards found under {}. Run this from the repo root.",
            content_dir.display()
        );
    }

    cards.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(cards)
}

/// Create the database and apply the schema.
///
/// Any existing file is replaced. A partial rebuild is worse than a slow one:
/// the corpus is small enough that a full rebuild takes seconds, and stale rows
/// from a previous build are invisible until they surface as a search result for
/// a card that no longer exists.
pub fn create_database(path: &Path) -> Result<Connection> {
    if path.exists() {
        std::fs::remove_file(path).map_err(|e| {
            // Windows refuses to delete a file another process has open, and the
            // other process is nearly always the app itself still running from
            // `pnpm tauri dev`. The raw "os error 32" gives no hint of that.
            if e.kind() == std::io::ErrorKind::PermissionDenied
                || e.raw_os_error() == Some(32)
            {
                anyhow::anyhow!(
                    "{} is open in another process.\n\
                     The app holds the database while it is running. Close the Coding \
                     Compendium window (or stop `pnpm tauri dev`) and run this again.",
                    path.display()
                )
            } else {
                anyhow::Error::new(e)
                    .context(format!("removing previous build at {}", path.display()))
            }
        })?;
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let conn = Connection::open(path)
        .with_context(|| format!("creating {}", path.display()))?;

    conn.execute_batch(include_str!("../search/schema.sql"))
        .context("applying schema.sql")?;

    Ok(conn)
}

/// Insert cards, their chunks, and the derived tables.
pub fn write_cards(conn: &mut Connection, cards: &[Card]) -> Result<Stats> {
    let tx = conn.transaction()?;
    let mut stats = Stats::default();

    {
        let mut insert_card = tx.prepare(
            "INSERT INTO cards (id, type, title, track, ord, answer, body, keywords, volatility, verified, meta)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        )?;
        let mut insert_chunk = tx.prepare(
            "INSERT INTO chunks (card_id, ord, heading_path, text) VALUES (?1, ?2, ?3, ?4)",
        )?;

        for card in cards {
            insert_card.execute(rusqlite::params![
                card.id,
                card.kind.as_str(),
                card.title,
                card.track,
                card.order,
                card.answer,
                card.body,
                card.keywords.join(" "),
                card.volatility,
                card.verified,
                card.meta_json,
            ])?;
            stats.cards += 1;

            for (i, piece) in chunk_card(card).iter().enumerate() {
                insert_chunk.execute(rusqlite::params![
                    card.id,
                    i as i64,
                    piece.heading_path,
                    piece.text
                ])?;
                stats.chunks += 1;
            }
        }
    }

    // Populate the full-text index from the cards table.
    //
    // This is an external-content FTS5 table, so it does not hold the text
    // itself, only the inverted index pointing back at `cards`. The 'rebuild'
    // command tells it to read the base table and build that index in one pass,
    // which is both faster and less error-prone than inserting row by row.
    //
    // The `keywords` column is pulled out of the JSON meta blob so that search
    // terms an author added deliberately (plurals, misspellings, the
    // wrong-but-common name for a thing) are indexed alongside the prose.
    tx.execute_batch("INSERT INTO cards_fts(cards_fts) VALUES('rebuild')")
        .context("building the full-text index")?;

    // The identifier's scoring table, compiled out of the language cards so the
    // cards stay the one source of truth for both the prose and the classifier.
    stats.languages = write_language_signals(&tx, cards)?;

    tx.commit()?;
    Ok(stats)
}

/// Embed every chunk and store the vectors.
///
/// Runs as a second pass after the text is written, because embedding is by far
/// the slowest part of the build and keeping it separate means a content-only
/// change can skip it during authoring.
///
/// The model identity is recorded in `build_meta`. The compiler and the app must
/// use identical model bytes or their vectors live in different spaces, and the
/// symptom is not an error: it is search that returns confident nonsense. The
/// app checks this at startup and disables semantic search on a mismatch rather
/// than silently mixing them.
pub fn write_embeddings(conn: &mut Connection) -> Result<usize> {
    use crate::embed::{to_blob, Embedder, DIMS};

    let chunks: Vec<(i64, String)> = {
        let mut stmt = conn.prepare("SELECT id, text FROM chunks ORDER BY id")?;
        let rows = stmt.query_map([], |r| Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?)))?;
        rows.collect::<rusqlite::Result<Vec<_>>>()?
    };

    if chunks.is_empty() {
        return Ok(0);
    }

    let mut embedder = Embedder::load()?;
    let mut written = 0usize;

    // Batched, because a forward pass has fixed overhead per call and 700 calls
    // of one chunk each is several times slower than 3 calls of 256.
    const BATCH: usize = 256;
    for batch in chunks.chunks(BATCH) {
        let texts: Vec<String> = batch.iter().map(|(_, t)| t.clone()).collect();
        let vectors = embedder.embed_documents(texts)?;

        let tx = conn.transaction()?;
        {
            let mut insert = tx.prepare(
                "INSERT OR REPLACE INTO chunk_vectors (chunk_id, vec) VALUES (?1, ?2)",
            )?;
            for ((id, _), vector) in batch.iter().zip(&vectors) {
                if vector.len() != DIMS {
                    bail!(
                        "the model returned {} dimensions, expected {DIMS}. \
                         The wrong model is loaded and its vectors would be meaningless.",
                        vector.len()
                    );
                }
                insert.execute(rusqlite::params![id, to_blob(vector)])?;
                written += 1;
            }
        }
        tx.commit()?;
    }

    conn.execute(
        "INSERT OR REPLACE INTO build_meta (key, value) VALUES ('embed_model', ?1)",
        ["bge-small-en-v1.5-q"],
    )?;
    conn.execute(
        "INSERT OR REPLACE INTO build_meta (key, value) VALUES ('embed_dims', ?1)",
        [DIMS.to_string()],
    )?;

    Ok(written)
}

#[derive(Debug, Default)]
pub struct Stats {
    pub cards: usize,
    pub chunks: usize,
    pub languages: LanguageStats,
}
