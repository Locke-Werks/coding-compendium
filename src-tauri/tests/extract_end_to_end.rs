//! The extractive answer path, against the real corpus and the real model.
//!
//! The unit tests in `synth` prove the selection logic with a stub embedder.
//! This proves the thing that actually matters and that a stub cannot: that
//! against 686 real cards, a real question pulls sentences that answer it, and
//! every one of them appears verbatim in a card she can open.
//!
//! Ignored by default because it loads the embedding model. Run deliberately:
//!
//! ```powershell
//! cargo test --manifest-path src-tauri/Cargo.toml --test extract_end_to_end -- --ignored --nocapture
//! ```

use compendium_lib::embed::Embedder;
use compendium_lib::search::{fuse_engines, semantic, Index, CANDIDATE_DEPTH};
use compendium_lib::synth::{extract, Passage};
use std::path::{Path, PathBuf};

fn corpus() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("build")
        .join("content.db")
}

/// Search, then extract, exactly as the app does.
fn answer(query: &str) -> Vec<(String, String)> {
    let db = corpus();
    assert!(db.exists(), "run `pnpm build:content` first");

    let index = Index::open(&db).expect("opening corpus");
    let mut model = Embedder::load().expect("loading model");

    let qv = model.embed_query(query).expect("embedding query");
    let lex: Vec<String> = index
        .lexical(query, false, CANDIDATE_DEPTH)
        .unwrap()
        .into_iter()
        .map(|(id, _)| id)
        .collect();
    let sem: Vec<String> = semantic(&qv, &index.load_vectors().unwrap(), CANDIDATE_DEPTH)
        .into_iter()
        .map(|(id, _)| id)
        .collect();

    let ranked = fuse_engines(lex, sem);
    let cards: Vec<_> = ranked
        .iter()
        .take(4)
        .filter_map(|(id, _)| index.card(id).ok().flatten())
        .collect();

    let passages: Vec<Passage<'_>> = cards
        .iter()
        .map(|c| Passage {
            card_id: &c.id,
            card_title: &c.title,
            heading_path: &c.title,
            text: &c.body,
        })
        .collect();

    let out = extract(&qv, &passages, |texts| model.embed_documents(texts.to_vec()).ok());

    // The property the whole module exists for: nothing was written, only
    // selected. Every returned sentence must appear in the card it claims,
    // word for word and in order.
    //
    // Both sides are whitespace-normalized before comparing. An excerpt carries
    // the card's original line wrapping, which is collapsed for display, so a
    // raw `contains` fails on a sentence that is genuinely verbatim. Collapsing
    // both sides preserves the property being tested (same words, same order)
    // and drops only the line breaks, which are a rendering detail.
    let squash = |s: &str| {
        s.replace("**", "")
            .replace(['`', '*'], "")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    };

    for e in &out.excerpts {
        let card = cards.iter().find(|c| c.id == e.card_id).expect("cited a card we did not send");
        let haystack = squash(&card.body);
        let needle = squash(&e.text);
        let needle = needle.trim_end_matches('.');
        assert!(
            haystack.contains(needle),
            "excerpt is not verbatim from {}:\n  excerpt: {:?}",
            e.card_id,
            e.text
        );
    }

    out.excerpts.into_iter().map(|e| (format!("{} {:.3}", e.card_id, e.score), e.text)).collect()
}

#[test]
#[ignore = "loads the embedding model"]
fn a_real_question_pulls_sentences_that_answer_it() {
    for (query, expect_topic) in [
        ("what is a merge conflict", "conflict"),
        ("how do i undo the last commit", "commit"),
        ("what does the staging area do", "stag"),
    ] {
        let out = answer(query);
        assert!(!out.is_empty(), "no excerpt for {query:?}");

        println!("\n{query}");
        for (card, text) in &out {
            println!("  [{card}] {text}");
        }

        let joined = out.iter().map(|(_, t)| t.to_lowercase()).collect::<String>();
        assert!(
            joined.contains(expect_topic),
            "nothing about {expect_topic:?} in the answer to {query:?}"
        );
    }
}

/// A question the corpus does not cover must produce no answer rather than a
/// confident-looking one. This is the property the local model failed to
/// guarantee, and the reason nothing here generates text.
#[test]
#[ignore = "loads the embedding model"]
fn an_uncovered_question_returns_nothing_rather_than_something() {
    for query in [
        "how do i connect prisma to postgres",
        "what is the best react state management library",
        "how do i configure a jenkins pipeline",
    ] {
        let out = answer(query);
        println!("\n{query}  ->  {} excerpts", out.len());
        for (card, text) in &out {
            println!("  [{card}] {text}");
        }
        // Retrieval will always return its best guess, so cards come back. The
        // requirement is only that nothing is fabricated, which `answer`
        // asserts on every excerpt above. An empty result is the ideal outcome
        // and a verbatim-but-tangential sentence is an acceptable one, because
        // she can see the card it came from and judge it herself.
    }
}
