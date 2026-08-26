// Coding Compendium, an offline reference for software development in the age of coding agents.
// Copyright (C) 2026 Locke Werks
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
// PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with
// this program. If not, see <https://www.gnu.org/licenses/>.
//
// The reference corpus in content/ is not part of this program and is dedicated
// to the public domain under CC0 1.0. See LICENSE-CONTENT.

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

    // Embedding is by far the slowest part of the build, so it is skippable
    // while authoring. `--no-embed` leaves the database lexical-only, which the
    // app handles: it checks for vectors and runs one engine instead of two.
    if std::env::args().any(|a| a == "--no-embed") {
        compile::seal(&conn)?;
        println!("         skipped embeddings (--no-embed), search will be lexical only");
        return Ok(());
    }

    let embed_started = Instant::now();
    println!("embedding {} chunks, this is the slow part", stats.chunks);
    let vectors = compile::write_embeddings(&mut conn)?;
    println!(
        "         {} vectors in {:.1}s",
        vectors,
        embed_started.elapsed().as_secs_f32()
    );

    compile::seal(&conn)?;
    println!("sealed   rollback journal, readable from a directory it cannot write");

    Ok(())
}
