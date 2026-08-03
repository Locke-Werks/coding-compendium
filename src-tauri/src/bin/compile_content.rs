//! Builds `content.db` from `content/*.md`.
//!
//! ```powershell
//! pnpm build:content
//! ```
//!
//! Run it from the repo root. The database is a build artifact: git-ignored,
//! never edited by hand, and safe to delete because this rebuilds it in seconds.

use anyhow::{Context, Result};
use compendium_lib::compile;
use std::path::PathBuf;
use std::time::Instant;

fn main() -> Result<()> {
    let started = Instant::now();

    // Cargo runs this with the manifest directory as the working directory when
    // invoked through --manifest-path, so paths are resolved relative to the
    // repo root explicitly rather than relying on where it was launched from.
    let root = std::env::current_dir()?;
    let root = if root.join("content").is_dir() {
        root
    } else {
        root.parent().map(PathBuf::from).context("cannot find the repo root")?
    };

    let content_dir = root.join("content");
    let out = root.join("build").join("content.db");

    println!("reading  {}", content_dir.display());
    let cards = compile::load_cards(&content_dir)?;

    let mut conn = compile::create_database(&out)?;
    let stats = compile::write_cards(&mut conn, &cards)?;

    // Vectors are added in a later pass, once the embedding model is wired up.
    // Until then the database supports lexical search only, which is exactly
    // what the build order calls for: prove the word matcher feels instant
    // before adding the part that could hide latency behind it.
    println!(
        "wrote    {} ({} cards, {} chunks) in {:.1}s",
        out.display(),
        stats.cards,
        stats.chunks,
        started.elapsed().as_secs_f32()
    );
    let l = &stats.languages;
    println!(
        "         identifier: {} tells, {} rules-out, {} manifests, {} tiebreaks, {} error patterns",
        l.tells, l.rules_out, l.manifests, l.tiebreaks, l.error_patterns
    );

    Ok(())
}
