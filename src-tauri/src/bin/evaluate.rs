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
use compendium_lib::search::{Index, CANDIDATE_DEPTH};
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

    let mut hits_at_5 = 0usize;
    let mut hits_at_1 = 0usize;
    let mut reciprocal_total = 0.0f64;
    let mut failures: Vec<(String, Vec<String>)> = Vec::new();

    for case in &cases {
        // `false` for prefix: an eval query is a committed question, not a
        // half-typed one.
        let ranked = index.lexical(&case.query, false, CANDIDATE_DEPTH)?;
        let ids: Vec<String> = ranked.iter().map(|(id, _)| id.clone()).collect();

        // An intent forwards to its target, so a hit on either is correct: what
        // she sees when she opens it is the same card.
        let mut rank = None;
        for (i, id) in ids.iter().enumerate() {
            let resolved = index.card(id)?.map(|c| c.id);
            if case.acceptable.contains(id) || resolved.is_some_and(|r| case.acceptable.contains(&r))
            {
                rank = Some(i + 1);
                break;
            }
        }

        match rank {
            Some(r) => {
                reciprocal_total += 1.0 / r as f64;
                if r <= 5 {
                    hits_at_5 += 1;
                }
                if r == 1 {
                    hits_at_1 += 1;
                }
                if r > 5 {
                    failures.push((case.query.clone(), ids.iter().take(3).cloned().collect()));
                }
            }
            None => failures.push((case.query.clone(), ids.iter().take(3).cloned().collect())),
        }
    }

    let n = cases.len() as f64;
    println!("{} queries over {} cards\n", cases.len(), index.card_count()?);
    println!("  recall@5   {:.1}%   the right answer is on screen without scrolling", hits_at_5 as f64 / n * 100.0);
    println!("  recall@1   {:.1}%   the right answer is the first thing she reads", hits_at_1 as f64 / n * 100.0);
    println!("  MRR        {:.3}    higher means the answer sits nearer the top", reciprocal_total / n);

    if !failures.is_empty() {
        println!("\n{} queries missed the top five:", failures.len());
        for (query, got) in &failures {
            println!("  {query}");
            println!("      got: {}", if got.is_empty() { "nothing".into() } else { got.join(", ") });
        }
    }

    Ok(())
}
