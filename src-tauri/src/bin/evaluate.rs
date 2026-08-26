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

//! Measures how good the search actually is.
//!
//! ```powershell
//! pnpm build:content
//! cargo run --manifest-path src-tauri/Cargo.toml --bin evaluate
//! ```
//!
//! Retrieval quality is not something you can eyeball. Every ranking change
//! looks like an improvement on the three queries you happen to try, and the
//! damage shows up on the ones you did not. This runs a fixed set of realistic
//! questions and reports two numbers.
//!
//! **recall@5** is the share of queries whose right answer appears in the top
//! five. It is the number that matters most here, because the palette shows
//! about five results without scrolling. A right answer at rank 6 is a miss.
//!
//! **MRR (Mean Reciprocal Rank)** averages 1/rank of the first correct hit. It
//! rewards being first rather than merely present, which recall@5 cannot see.
//! Moving an answer from rank 4 to rank 1 leaves recall unchanged and moves MRR
//! a long way.
//!
//! The immediate job is deciding whether semantic search earns its place.
//! Measure now with lexical only, add embeddings, measure again. If the numbers
//! do not move, the model is 63 MB of installer buying nothing.

use anyhow::{bail, Context, Result};
use compendium_lib::embed::Embedder;
use compendium_lib::search::{
    fuse, fuse_engines, fuse_weighted, semantic, Index, CANDIDATE_DEPTH, LEXICAL_WEIGHT,
    SEMANTIC_WEIGHT,
};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

struct Case {
    query: String,
    /// Any of these counts as correct. Several cards can legitimately answer the
    /// same question, and demanding one right answer measures the fixture rather
    /// than the search.
    acceptable: HashSet<String>,
}

fn load_cases(path: &Path) -> Result<Vec<Case>> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("reading {}", path.display()))?;

    let mut cases = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let line = line.trim_end();
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split('\t').filter(|p| !p.trim().is_empty());
        let query = parts.next().context(format!("line {} has no query", i + 1))?.trim().to_string();
        let acceptable: HashSet<String> = parts.map(|p| p.trim().to_string()).collect();

        if acceptable.is_empty() {
            bail!("line {} has a query but no expected card ids (use a real tab)", i + 1);
        }
        cases.push(Case { query, acceptable });
    }
    Ok(cases)
}

fn main() -> Result<()> {
    let root = std::env::current_dir()?;
    let root = if root.join("content").is_dir() {
        root
    } else {
        root.parent().map(PathBuf::from).context("cannot find the repo root")?
    };

    let db = root.join("build").join("content.db");
    if !db.exists() {
        bail!("{} not found. Run `pnpm build:content` first.", db.display());
    }
    let index = Index::open(&db)?;
    let cases = load_cases(&root.join("eval").join("queries.tsv"))?;

    // An expected id that does not exist would score as a permanent miss and
    // look like a search failure. Catch it as what it is: a stale fixture.
    let mut missing: Vec<String> = Vec::new();
    for case in &cases {
        for id in &case.acceptable {
            if index.card(id)?.is_none() {
                missing.push(id.clone());
            }
        }
    }
    missing.sort();
    missing.dedup();
    if !missing.is_empty() {
        println!("WARNING: {} expected ids do not exist yet:", missing.len());
        for id in &missing {
            println!("  {id}");
        }
        println!("These count as misses below, so the scores are a floor.\n");
    }

    // Run each engine alone as well as fused. Fusion is only worth its cost if
    // it beats both, and reporting all three is what makes that checkable
    // instead of assumed.
    let has_vectors = index.has_vectors()?;
    let vectors = if has_vectors { index.load_vectors()? } else { Vec::new() };
    let mut embedder = if has_vectors { Some(Embedder::load()?) } else { None };

    if !has_vectors {
        println!("No vectors in this database. Reporting lexical only.");
        println!("Run `pnpm build:content` without --no-embed to include semantic search.\n");
    }

    let mut scores = [Metrics::default(), Metrics::default(), Metrics::default(), Metrics::default()];
    const LEXICAL: usize = 0;
    const SEMANTIC: usize = 1;
    const HYBRID_EQUAL: usize = 2;
    const HYBRID_WEIGHTED: usize = 3;

    let mut failures: Vec<(String, Vec<String>)> = Vec::new();
    // Kept so the sweep can re-fuse without re-running either engine.
    let mut rankings: Vec<(Vec<String>, Vec<String>)> = Vec::new();

    for case in &cases {
        // `false` for prefix: an eval query is a committed question, not a
        // half-typed one.
        let lex = index.lexical(&case.query, false, CANDIDATE_DEPTH)?;
        let lex_ids: Vec<String> = lex.iter().map(|(id, _)| id.clone()).collect();

        let sem_ids: Vec<String> = match embedder.as_mut() {
            Some(e) => {
                let qv = e.embed_query(&case.query)?;
                semantic(&qv, &vectors, CANDIDATE_DEPTH)
                    .into_iter()
                    .map(|(id, _)| id)
                    .collect()
            }
            None => Vec::new(),
        };

        // Both fusions are measured. Equal weight is the obvious thing to
        // reach for and it is not the best thing here, so reporting both keeps
        // that finding visible instead of buried in a constant.
        let (fused, fused_equal): (Vec<String>, Vec<String>) = if sem_ids.is_empty() {
            (lex_ids.clone(), lex_ids.clone())
        } else {
            (
                fuse_engines(lex_ids.clone(), sem_ids.clone())
                    .into_iter().map(|(id, _)| id).collect(),
                fuse(&[lex_ids.clone(), sem_ids.clone()])
                    .into_iter().map(|(id, _)| id).collect(),
            )
        };

        scores[LEXICAL].add(rank_of(&index, &lex_ids, &case.acceptable)?);
        if !sem_ids.is_empty() {
            scores[SEMANTIC].add(rank_of(&index, &sem_ids, &case.acceptable)?);
        }
        scores[HYBRID_EQUAL].add(rank_of(&index, &fused_equal, &case.acceptable)?);
        let hybrid_rank = rank_of(&index, &fused, &case.acceptable)?;
        scores[HYBRID_WEIGHTED].add(hybrid_rank);

        rankings.push((lex_ids.clone(), sem_ids.clone()));

        if hybrid_rank.is_none_or(|r| r > 5) {
            failures.push((case.query.clone(), fused.iter().take(3).cloned().collect()));
        }
    }

    let n = cases.len();
    println!("{} queries over {} cards\n", n, index.card_count()?);
    println!("                 recall@5   recall@1      MRR");
    println!("  lexical        {}", scores[LEXICAL].row(n));
    if has_vectors {
        println!("  semantic       {}", scores[SEMANTIC].row(n));
        println!("  hybrid equal   {}", scores[HYBRID_EQUAL].row(n));
        // Label derived from the constant, not typed in. A hardcoded "1:1.8"
        // survived a change to 3.0 and reported the wrong ratio next to the
        // right numbers, which is worse than no label.
        println!(
            "  hybrid {LEXICAL_WEIGHT:.0}:{SEMANTIC_WEIGHT:.0}     {}   <- shipping",
            scores[HYBRID_WEIGHTED].row(n)
        );
    }

    // `--sweep` tries a range of semantic weights instead of guessing one.
    // Picking a fusion constant by intuition is how a number nobody can defend
    // ends up sitting in the codebase for a year.
    if std::env::args().any(|a| a == "--sweep") && has_vectors {
        println!("\nweight sweep, lexical held at 1.0\n");
        println!("  semantic w   recall@5   recall@1      MRR");
        for w in [0.5, 1.0, 1.5, 1.8, 2.0, 2.5, 3.0, 4.0, 6.0, 10.0] {
            let mut m = Metrics::default();
            for (case, (lex_ids, sem_ids)) in cases.iter().zip(&rankings) {
                let fused: Vec<String> =
                    fuse_weighted(&[(lex_ids.clone(), 1.0), (sem_ids.clone(), w)])
                        .into_iter()
                        .map(|(id, _)| id)
                        .collect();
                m.add(rank_of(&index, &fused, &case.acceptable)?);
            }
            println!("  {w:>9.1}   {}", m.row(n));
        }
    }

    if !failures.is_empty() {
        println!("\n{} queries missed the top five:", failures.len());
        for (query, got) in &failures {
            println!("  {query}");
            println!("      got: {}", if got.is_empty() { "nothing".into() } else { got.join(", ") });
        }
    }

    Ok(())
}

#[derive(Default)]
struct Metrics {
    at_5: usize,
    at_1: usize,
    reciprocal: f64,
}

impl Metrics {
    fn add(&mut self, rank: Option<usize>) {
        let Some(r) = rank else { return };
        self.reciprocal += 1.0 / r as f64;
        if r <= 5 {
            self.at_5 += 1;
        }
        if r == 1 {
            self.at_1 += 1;
        }
    }

    fn row(&self, n: usize) -> String {
        let n = n as f64;
        format!(
            "{:>7.1}%   {:>7.1}%   {:>6.3}",
            self.at_5 as f64 / n * 100.0,
            self.at_1 as f64 / n * 100.0,
            self.reciprocal / n
        )
    }
}

/// Position of the first acceptable answer in a ranking, 1-based.
fn rank_of(index: &Index, ids: &[String], acceptable: &HashSet<String>) -> Result<Option<usize>> {
    for (i, id) in ids.iter().enumerate() {
        // An intent forwards to its target, so a hit on either is correct: what
        // they see on opening it is the same card.
        let resolved = index.card(id)?.map(|c| c.id);
        if acceptable.contains(id) || resolved.is_some_and(|r| acceptable.contains(&r)) {
            return Ok(Some(i + 1));
        }
    }
    Ok(None)
}

