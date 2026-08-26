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

//! End-to-end test of the lexical search path.
//!
//! The unit tests cover the escaper and the fusion arithmetic in isolation. This
//! one builds a real database from real card text and searches it, which is the
//! only way to catch the wiring mistakes that unit tests structurally cannot:
//! an FTS5 table whose columns do not line up with its content table, bm25
//! weights applied in the wrong order, or a MATCH expression that parses in
//! theory and is rejected by SQLite in practice.

use compendium_lib::compile;
use compendium_lib::search::Index;
use std::path::PathBuf;

/// Write a set of cards to a temp directory, compile them, and open the result.
fn build_corpus(name: &str, cards: &[(&str, &str)]) -> (Index, PathBuf) {
    let dir = std::env::temp_dir().join(format!("compendium-test-{name}"));
    let content = dir.join("content");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&content).unwrap();

    for (id, body) in cards {
        std::fs::write(content.join(format!("{id}.md")), body).unwrap();
    }

    let loaded = compile::load_cards(&content).unwrap();
    let db = dir.join("content.db");
    let mut conn = compile::create_database(&db).unwrap();
    compile::write_cards(&mut conn, &loaded).unwrap();
    drop(conn);

    (Index::open(&db).unwrap(), dir)
}

fn card(id: &str, title: &str, answer: &str, body: &str) -> String {
    format!(
        "---\nid: {id}\ntitle: {title}\ntype: section\ntrack: D\nvolatility: low\nverified: 2026-08-02\nanswer: >\n  {answer}\n---\n\n## More\n\n{body}\n\n## Full\n\n{body} More detail here about the same subject, at greater length.\n"
    )
}

/// Build the shared fixture corpus in a directory of its own.
///
/// The `name` is per-test and not decoration. Cargo runs integration tests in
/// parallel threads, and on Windows an open SQLite file cannot be deleted, so a
/// shared directory makes every test after the first fail with a file-in-use
/// error that looks nothing like the bug it actually is.
fn corpus(name: &str) -> (Index, PathBuf) {
    let conflicts = card(
        "merge-conflicts",
        "Merge conflicts",
        "A merge conflict means git could not combine two changes automatically.",
        "When two branches change the same line, git stops and asks you to choose. The markers show both versions.",
    );
    let branches = card(
        "branches",
        "Branches",
        "A branch is a parallel line of development.",
        "You branch off main, do work, then merge back. Use git switch to move between them.",
    );
    let reset = card(
        "undoing-things",
        "Undoing things",
        "Use revert to undo safely and reset only when you understand what it discards.",
        "The reflog recovers almost anything you thought you lost.",
    );

    build_corpus(
        name,
        &[
            ("merge-conflicts", &conflicts),
            ("branches", &branches),
            ("undoing-things", &reset),
        ],
    )
}

#[test]
fn finds_a_card_by_its_title() {
    let (index, _dir) = corpus("finds_a_card_by_its_title");
    let hits = index.lexical("merge conflict", false, 10).unwrap();
    assert!(!hits.is_empty(), "expected a hit for 'merge conflict'");
    assert_eq!(hits[0].0, "merge-conflicts");
}

/// The whole reason bm25 gets column weights. Both cards contain the word
/// "branch", but only one is titled with it, and that one is what someone
/// searching for it wants.
#[test]
fn a_title_match_outranks_a_body_match() {
    let (index, _dir) = corpus("a_title_match_outranks_a_body_match");
    let hits = index.lexical("branch", false, 10).unwrap();
    assert_eq!(
        hits[0].0, "branches",
        "the card titled 'Branches' must outrank the one merely mentioning branches"
    );
}

/// Porter stemming, declared in the FTS5 tokenizer. Without it a search for the
/// word someone actually types misses the card that uses a different inflection.
#[test]
fn stemming_matches_across_inflections() {
    let (index, _dir) = corpus("stemming_matches_across_inflections");
    let hits = index.lexical("branching", false, 10).unwrap();
    assert!(!hits.is_empty(), "'branching' should still find the branches card via stemming");
}

/// The prefix behavior that makes as-you-type search feel live. Without it the
/// result list empties out between words.
#[test]
fn a_partial_last_word_still_matches() {
    let (index, _dir) = corpus("a_partial_last_word_still_matches");
    let hits = index.lexical("conflic", true, 10).unwrap();
    assert!(!hits.is_empty(), "a half-typed word must match with prefix search on");
    assert_eq!(hits[0].0, "merge-conflicts");

    let committed = index.lexical("conflic", false, 10).unwrap();
    assert!(committed.is_empty(), "without prefix search, a half-typed word is a literal and matches nothing");
}

/// The failure this whole design exists to prevent. Raw, these are FTS5 syntax
/// and either error or silently change the meaning of the query.
#[test]
fn operator_words_and_punctuation_do_not_break_the_query() {
    let (index, _dir) = corpus("operator_words_and_punctuation_do_not_break_the_query");
    for query in [
        "git reset --hard",
        "NOT branch",
        "a OR b",
        "NEAR(x y)",
        "C++",
        r#"unterminated "quote"#,
        "*",
        "()",
        "-",
    ] {
        // The assertion is that this returns rather than erroring. FTS5 rejects
        // a malformed MATCH when the statement steps, so a regression in the
        // escaper surfaces here as an Err, not as a bad ranking.
        let result = index.lexical(query, true, 10);
        assert!(result.is_ok(), "query {query:?} should not error: {:?}", result.err());
    }
}

#[test]
fn an_empty_query_returns_nothing_rather_than_everything() {
    let (index, _dir) = corpus("an_empty_query_returns_nothing_rather_than_everything");
    assert!(index.lexical("", true, 10).unwrap().is_empty());
    assert!(index.lexical("   ", true, 10).unwrap().is_empty());
}

#[test]
fn hydrate_returns_display_fields_in_rank_order() {
    let (index, _dir) = corpus("hydrate_returns_display_fields_in_rank_order");
    let ranked = index.lexical("merge conflict", false, 10).unwrap();
    let hits = index
        .hydrate(&ranked, |_| compendium_lib::search::Matched::LexicalOnly)
        .unwrap();

    assert_eq!(hits[0].card_id, "merge-conflicts");
    assert_eq!(hits[0].title, "Merge conflicts");
    assert_eq!(hits[0].card_type, "section");
    assert!(
        hits[0].answer.as_deref().unwrap().contains("could not combine"),
        "the palette renders `answer` inline, so it has to survive compilation"
    );
}

/// A ranking naming a card that no longer exists must not fail the whole search.
#[test]
fn hydrate_skips_ids_that_are_gone() {
    let (index, _dir) = corpus("hydrate_skips_ids_that_are_gone");
    let ranked = vec![("branches".to_string(), 1.0), ("deleted-card".to_string(), 0.5)];
    let hits = index
        .hydrate(&ranked, |_| compendium_lib::search::Matched::LexicalOnly)
        .unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].card_id, "branches");
}

#[test]
fn the_corpus_compiles_and_chunks() {
    let (index, _dir) = corpus("the_corpus_compiles_and_chunks");
    assert_eq!(index.card_count().unwrap(), 3);
}

/// An intent has no body. It exists to catch a query in the reader's words and point at
/// the card that answers it, so opening one must land on the answer rather than
/// an empty page.
#[test]
fn opening_an_intent_lands_on_its_target() {
    let branches = card(
        "branches",
        "Branches",
        "A branch is a parallel line of development.",
        "You branch off main, do work, then merge back.",
    );
    let intent = "---\nid: make-a-branch\ntitle: Make a new branch\ntype: intent\nverified: 2026-08-02\nvolatility: low\ngoal: I want to start work without touching main.\nphrasings: [new branch, branch off, start something new, make a branch]\ntarget: branches\n---\n";

    let (index, _dir) = build_corpus(
        "opening_an_intent_lands_on_its_target",
        &[("branches", &branches), ("make-a-branch", intent)],
    );

    let opened = index.card("make-a-branch").unwrap().unwrap();
    assert_eq!(opened.id, "branches", "an intent must forward to its target");
    assert!(opened.body.contains("branch off main"));
}

/// A target that does not resolve must still show something rather than
/// failing, because a broken link is an authoring bug and not a reason to
/// refuse to render.
#[test]
fn an_intent_with_a_dangling_target_still_opens() {
    let intent = "---\nid: orphan\ntitle: Goes nowhere\ntype: intent\nverified: 2026-08-02\nvolatility: low\ngoal: I want a card that does not exist.\nphrasings: [a, b, c, d]\ntarget: no-such-card\n---\n";
    let (index, _dir) = build_corpus("an_intent_with_a_dangling_target_still_opens", &[("orphan", intent)]);

    let opened = index.card("orphan").unwrap().unwrap();
    assert_eq!(opened.id, "orphan");
}

#[test]
fn a_missing_card_is_none_not_an_error() {
    let (index, _dir) = corpus("a_missing_card_is_none_not_an_error");
    assert!(index.card("does-not-exist").unwrap().is_none());
}
