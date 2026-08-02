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

pub use chunk::{chunk_card, Chunk, CHUNK_OVERLAP_TOKENS, CHUNK_TARGET_TOKENS};
pub use frontmatter::{split_frontmatter, Card, CardKind};

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

    tx.commit()?;
    Ok(stats)
}

#[derive(Debug, Default)]
pub struct Stats {
    pub cards: usize,
    pub chunks: usize,
}
