//! Search over the compiled corpus.
//!
//! Two engines run against the same cards and fail in opposite directions.
//!
//! **Lexical** (this module, plus [`query`]) matches the words that were typed.
//! It is exact and completely literal. Searching for "detached HEAD" finds every
//! card containing those words; searching for "my commits went somewhere weird"
//! finds nothing, because none of those words appear anywhere.
//!
//! **Semantic** (added in [`crate::embed`]) matches meaning, by comparing
//! vectors produced by a small neural network. It finds the detached-HEAD card
//! for that second query despite zero words in common. Its failure is the mirror
//! image: it is fuzzy, so an exact search for `git reset --hard` can drift to
//! merely related cards.
//!
//! Each covers the other's blind spot, so both run and their ranked lists are
//! merged with Reciprocal Rank Fusion in [`fuse`].

pub mod query;

use anyhow::{Context, Result};
use rusqlite::{Connection, OpenFlags};
use serde::Serialize;
use std::path::Path;

/// How many candidates each engine contributes before fusion.
///
/// Fusion only reorders; it cannot recover a result neither engine returned. 50
/// is deep enough that the right card is nearly always somewhere in one of the
/// lists, and shallow enough to stay well inside the latency budget.
pub const CANDIDATE_DEPTH: usize = 50;

/// The `k` constant in Reciprocal Rank Fusion.
///
/// **Not 60.** The value everyone quotes comes from a paper fusing thousand-item
/// web-scale result lists, where flattening the curve is the point. Over a
/// 50-item list, `k = 60` compresses rank 1 and rank 50 into a 1.8x band, which
/// lets a card that both engines rank mediocre beat one that a single engine
/// ranks first. That is exactly backwards for a corpus this size, where an
/// engine ranking something first is strong evidence.
///
/// `k = 10` keeps rank 1 worth about 5.5x rank 50, so a confident single-engine
/// hit still wins while a two-engine agreement still gets its boost.
pub const RRF_K: f64 = 10.0;

/// A single search result, ready to render.
#[derive(Debug, Clone, Serialize)]
pub struct Hit {
    pub card_id: String,
    pub title: String,
    pub card_type: String,
    /// The one-sentence answer, when the card has one. This is what the palette
    /// shows inline, so a hit without it renders as a title only.
    pub answer: Option<String>,
    pub score: f64,
    /// Which engines found it. Useful for debugging retrieval, and shown in the
    /// UI only in developer mode.
    pub matched: Matched,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Matched {
    /// Both engines returned it. The strongest signal available.
    Hybrid,
    /// Only the word matcher. Typical for exact strings: error text, flags,
    /// command names.
    LexicalOnly,
    /// Only the meaning matcher. Typical for the vague plain-language questions
    /// this app exists to answer.
    SemanticOnly,
}

/// A ranked list from one engine, most relevant first.
pub type Ranking = Vec<String>;

/// Merge ranked lists with Reciprocal Rank Fusion.
///
/// RRF scores each result by `1 / (k + rank)` in every list it appears in, and
/// sums. The important property is that it uses only *position*, never the raw
/// scores.
///
/// That matters because BM25 and cosine similarity are not comparable. BM25 is
/// unbounded, corpus-relative, and returned negated by SQLite; cosine sits in
/// [-1, 1]. Normalizing them against each other means picking a mapping, and
/// every choice of mapping is a thumb on the scale that has to be re-tuned
/// whenever the corpus changes. Ranks need no such choice.
pub fn fuse(rankings: &[Ranking]) -> Vec<(String, f64)> {
    use std::collections::HashMap;

    let mut scores: HashMap<&str, f64> = HashMap::new();
    for ranking in rankings {
        for (i, id) in ranking.iter().enumerate() {
            // Rank is 1-based: the top result should score 1/(k+1), not 1/k.
            let rank = (i + 1) as f64;
            *scores.entry(id.as_str()).or_insert(0.0) += 1.0 / (RRF_K + rank);
        }
    }

    let mut out: Vec<(String, f64)> = scores.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
    // Sort by score descending, then by id so equal scores are stable across
    // runs. Without the tiebreak, HashMap iteration order makes the result list
    // shuffle between identical queries.
    out.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal).then_with(|| a.0.cmp(&b.0)));
    out
}

/// Extra score for a card whose title matches a query term.
///
/// BM25 ranks purely on term statistics: how often a word appears here, how rare
/// it is across the corpus, how long the document is. It has no notion that a
/// card *is about* the thing its title names.
///
/// That gap is visible the moment you use it. Searching "how do i know if this
/// is python" returned the C card first, because C's card mentions Python
/// several times while comparing itself to it, and mentions are all BM25 counts.
/// The card actually titled "Python" was nowhere near the top.
///
/// The boost is deliberately large. It is not a nudge among equals: a title
/// match is a categorically different kind of evidence from a body mention, and
/// treating it as merely one more signal is what produced the wrong answer.
fn title_boost(title: &str, terms: &[String]) -> f64 {
    if terms.is_empty() {
        return 0.0;
    }
    let lower = title.to_lowercase();

    // The whole query is the title, e.g. "merge conflicts". Nothing outranks it.
    if terms.len() > 1 && lower == terms.join(" ") {
        return 40.0;
    }

    let title_words: Vec<&str> = lower.split_whitespace().collect();
    let mut boost = 0.0;

    for term in terms {
        if lower == *term {
            // A single-word query naming the card exactly: "python", "rust".
            boost += 30.0;
        } else if title_words.contains(&term.as_str()) {
            // The term is one word of a multi-word title.
            boost += 12.0;
        }
    }
    boost
}

/// A handle to the compiled corpus.
pub struct Index {
    conn: Connection,
}

impl Index {
    /// Open content.db read-only.
    ///
    /// Read-only is deliberate and not just hygiene: it makes "the shipped
    /// content is unmodified" enforced by SQLite rather than by convention.
    /// Anything the user writes goes to a separate notes.db.
    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .with_context(|| format!("opening corpus at {}", path.display()))?;

        Ok(Self { conn })
    }

    /// Rank cards by word match.
    ///
    /// `prefix` should be true while the user is still typing and false once the
    /// query is committed. See [`query::to_match_expression`] for why the last
    /// token is treated differently from the rest.
    pub fn lexical(&self, raw: &str, prefix: bool, limit: usize) -> Result<Vec<(String, f64)>> {
        let Some(expr) = query::to_match_expression(raw, prefix) else {
            // Nothing searchable is a normal state, not an error.
            return Ok(Vec::new());
        };

        // bm25() weights columns left to right, matching the FTS5 declaration
        // order: title, answer, body, keywords. A title match is worth far more
        // than a body match, because a card titled "Merge conflicts" is almost
        // certainly what someone searching "merge conflict" wants.
        //
        // SQLite returns bm25 negated, so more negative is more relevant. The
        // sign is flipped here so callers can treat larger as better throughout.
        let mut stmt = self.conn.prepare_cached(
            r#"
            SELECT c.id, c.title, -bm25(cards_fts, 10.0, 5.0, 1.0, 3.0) AS score
            FROM cards_fts
            JOIN cards c ON c.rowid = cards_fts.rowid
            WHERE cards_fts MATCH ?1
            ORDER BY score DESC
            LIMIT ?2
            "#,
        )?;

        // A malformed MATCH expression fails when the statement steps, not when
        // it is prepared, so the error surfaces here. to_match_expression should
        // make that impossible, and the context string says so to make a
        // regression obvious rather than mysterious.
        let rows = stmt
            .query_map(rusqlite::params![expr, limit as i64], |r| {
                Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?, r.get::<_, f64>(2)?))
            })
            .with_context(|| format!("FTS5 rejected a generated expression: {expr}"))?;

        let mut scored: Vec<(String, f64)> = Vec::new();
        let terms = query::content_tokens(raw);

        for row in rows {
            let (id, title, score) = row.context("reading lexical results")?;
            scored.push((id, score + title_boost(&title, &terms)));
        }

        // The boost changes the order, so re-sort. The id tiebreak keeps the
        // result stable across identical queries.
        scored.sort_by(|a, b| {
            b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal).then_with(|| a.0.cmp(&b.0))
        });
        Ok(scored)
    }

    /// Fetch the display fields for a set of ids, preserving the given order.
    ///
    /// Ranking and hydration are separate steps because fusion works on ids
    /// alone. Fetching card bodies for 100 candidates only to discard 90 of them
    /// would be the most expensive part of the query.
    pub fn hydrate(&self, ranked: &[(String, f64)], matched: impl Fn(&str) -> Matched) -> Result<Vec<Hit>> {
        let mut stmt = self
            .conn
            .prepare_cached("SELECT id, title, type, answer FROM cards WHERE id = ?1")?;

        let mut hits = Vec::with_capacity(ranked.len());
        for (id, score) in ranked {
            let row = stmt.query_row([id], |r| {
                Ok(Hit {
                    card_id: r.get(0)?,
                    title: r.get(1)?,
                    card_type: r.get(2)?,
                    answer: r.get(3)?,
                    score: *score,
                    matched: matched(id),
                })
            });
            match row {
                Ok(hit) => hits.push(hit),
                // A ranking can name a card that no longer exists if an index is
                // stale. Skipping is better than failing the whole search.
                Err(rusqlite::Error::QueryReturnedNoRows) => continue,
                Err(e) => return Err(e).context("hydrating search results"),
            }
        }
        Ok(hits)
    }

    /// Build the paste identifier from this corpus.
    ///
    /// Loaded once at startup and reused: it compiles a few hundred regexes,
    /// which is cheap once and wasteful on every keystroke.
    pub fn identifier(&self) -> Result<crate::identify::Identifier> {
        crate::identify::Identifier::load(&self.conn)
    }

    /// Total cards in the corpus. Used by the startup sanity check.
    pub fn card_count(&self) -> Result<usize> {
        let n: i64 = self.conn.query_row("SELECT count(*) FROM cards", [], |r| r.get(0))?;
        Ok(n as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ids(v: &[(String, f64)]) -> Vec<&str> {
        v.iter().map(|(id, _)| id.as_str()).collect()
    }

    fn ranking(items: &[&str]) -> Ranking {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn a_single_ranking_survives_fusion_unchanged() {
        let fused = fuse(&[ranking(&["a", "b", "c"])]);
        assert_eq!(ids(&fused), vec!["a", "b", "c"]);
    }

    /// The whole point of fusion: something both engines like beats something
    /// only one engine likes.
    #[test]
    fn agreement_between_engines_wins() {
        let lexical = ranking(&["only-lexical", "agreed"]);
        let semantic = ranking(&["only-semantic", "agreed"]);
        let fused = fuse(&[lexical, semantic]);
        assert_eq!(fused[0].0, "agreed", "a card both engines returned should rank first");
    }

    /// The reason k is 10 rather than 60. A card ranked first by one engine and
    /// absent from the other must still beat a card both engines rank near the
    /// bottom. At k = 60 this assertion fails, which is what makes the constant
    /// worth a test rather than a comment.
    #[test]
    fn a_confident_single_hit_beats_two_mediocre_ones() {
        let mut lexical = vec!["filler".to_string(); 40];
        lexical[0] = "confident".to_string();
        lexical[38] = "mediocre".to_string();

        let mut semantic = vec!["other-filler".to_string(); 40];
        semantic[37] = "mediocre".to_string();

        let fused = fuse(&[lexical, semantic]);
        let pos = |id: &str| fused.iter().position(|(k, _)| k == id).unwrap();
        assert!(
            pos("confident") < pos("mediocre"),
            "rank-1-in-one-list should outrank rank-38-and-39; k is too flat"
        );
    }

    #[test]
    fn ties_break_deterministically() {
        // Both ids appear once at the same rank in different lists, so their
        // scores are identical. Without the id tiebreak this order would follow
        // HashMap iteration and shuffle between runs.
        let a = fuse(&[ranking(&["zebra"]), ranking(&["apple"])]);
        let b = fuse(&[ranking(&["apple"]), ranking(&["zebra"])]);
        assert_eq!(ids(&a), ids(&b), "identical scores must produce a stable order");
    }

    #[test]
    fn empty_input_is_not_an_error() {
        assert!(fuse(&[]).is_empty());
        assert!(fuse(&[vec![]]).is_empty());
    }

    fn terms(s: &str) -> Vec<String> {
        query::content_tokens(s)
    }

    /// The exact failure that motivated the boost: "python" ranked the C card
    /// first, because C's card compares itself to Python repeatedly and BM25
    /// counts mentions.
    #[test]
    fn a_card_named_by_the_query_outranks_one_merely_mentioning_it() {
        let t = terms("how do i know if this is python");
        assert!(
            title_boost("Python", &t) > title_boost("C language", &t),
            "the card titled Python must beat one that only discusses it"
        );
    }

    #[test]
    fn the_whole_query_matching_a_title_beats_a_single_word_match() {
        let t = terms("merge conflicts");
        assert!(title_boost("Merge conflicts", &t) > title_boost("Merge and rebase", &t));
    }

    #[test]
    fn title_matching_ignores_case_and_function_words() {
        assert!(title_boost("PowerShell", &terms("what is powershell")) > 0.0);
    }

    #[test]
    fn an_unrelated_title_gets_no_boost() {
        assert_eq!(title_boost("Merge conflicts", &terms("python")), 0.0);
        assert_eq!(title_boost("Python", &[]), 0.0);
    }
}
