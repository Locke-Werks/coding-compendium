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

/// Rank cards by meaning, against a pre-loaded set of chunk vectors.
///
/// Cards are scored by their single best chunk rather than an average. A long
/// card has many chunks about many things, and averaging them dilutes the one
/// paragraph that actually answers the question until a short, vaguely-related
/// card outranks it.
pub fn semantic(query_vec: &[f32], vectors: &[(String, Vec<f32>)], limit: usize) -> Vec<(String, f64)> {
    use std::collections::HashMap;

    let mut best: HashMap<&str, f32> = HashMap::new();
    for (card_id, vec) in vectors {
        let score = crate::embed::cosine(query_vec, vec);
        let entry = best.entry(card_id.as_str()).or_insert(f32::MIN);
        if score > *entry {
            *entry = score;
        }
    }

    let mut ranked: Vec<(String, f64)> =
        best.into_iter().map(|(k, v)| (k.to_string(), v as f64)).collect();
    ranked.sort_by(|a, b| {
        b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal).then_with(|| a.0.cmp(&b.0))
    });
    ranked.truncate(limit);
    ranked
}

/// How much each engine's opinion counts in fusion.
///
/// **Not equal, and measured rather than guessed.** Over 60 real queries against
/// 686 cards:
///
/// ```text
///                  recall@5   recall@1      MRR
///   lexical           91.7%      71.7%    0.805
///   semantic          95.0%      81.7%    0.871
///   hybrid, equal     98.3%      73.3%    0.845
///   hybrid, 1:3       98.3%      81.7%    0.878
/// ```
///
/// Equal weight is the obvious thing to reach for and it is wrong here. It won
/// on recall@5 and gave up ten points of recall@1 against semantic alone: it was
/// finding the answer and then burying it, because an equal vote lets the
/// measurably weaker engine outvote the stronger one on what belongs first.
///
/// recall@1 is the metric that matters most, because the palette renders the top
/// result's answer inline. Rank 1 is the difference between reading the answer
/// and choosing from a list.
///
/// A sweep from 0.5 to 10.0 puts the peak at 3.0, where fusion beats **both**
/// engines individually on every metric: it keeps lexical's coverage of exact
/// strings (error text, flags, command names) while letting semantic decide the
/// order. Past 3.0 the numbers flatten as lexical stops mattering, so the lowest
/// weight that reaches the peak is the one to take.
///
/// Re-run `pnpm eval -- --sweep` after any substantial content change. This is a
/// tuned constant, not a law.
pub const LEXICAL_WEIGHT: f64 = 1.0;
pub const SEMANTIC_WEIGHT: f64 = 3.0;

/// Merge ranked lists with weighted Reciprocal Rank Fusion.
///
/// RRF scores each result by `weight / (k + rank)` in every list it appears in,
/// and sums. The important property is that it uses only *position*, never the
/// raw scores.
///
/// That matters because BM25 and cosine similarity are not comparable. BM25 is
/// unbounded, corpus-relative, and returned negated by SQLite; cosine sits in
/// [-1, 1]. Normalizing them against each other means picking a mapping, and
/// every choice of mapping is a thumb on the scale that has to be re-tuned
/// whenever the corpus changes. Ranks need no such choice.
///
/// Each entry is a ranking and how much that engine's opinion is worth. Pass
/// equal weights for plain RRF.
pub fn fuse_weighted(rankings: &[(Ranking, f64)]) -> Vec<(String, f64)> {
    use std::collections::HashMap;

    let mut scores: HashMap<&str, f64> = HashMap::new();
    for (ranking, weight) in rankings {
        for (i, id) in ranking.iter().enumerate() {
            // Rank is 1-based: the top result should score w/(k+1), not w/k.
            let rank = (i + 1) as f64;
            *scores.entry(id.as_str()).or_insert(0.0) += weight / (RRF_K + rank);
        }
    }

    let mut out: Vec<(String, f64)> = scores.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
    // Sort by score descending, then by id so equal scores are stable across
    // runs. Without the tiebreak, HashMap iteration order makes the result list
    // shuffle between identical queries.
    out.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal).then_with(|| a.0.cmp(&b.0)));
    out
}

/// Merge ranked lists with every engine counting equally.
pub fn fuse(rankings: &[Ranking]) -> Vec<(String, f64)> {
    let weighted: Vec<(Ranking, f64)> = rankings.iter().map(|r| (r.clone(), 1.0)).collect();
    fuse_weighted(&weighted)
}

/// Merge the two search engines with the weights measured to work best.
pub fn fuse_engines(lexical: Ranking, semantic: Ranking) -> Vec<(String, f64)> {
    fuse_weighted(&[(lexical, LEXICAL_WEIGHT), (semantic, SEMANTIC_WEIGHT)])
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

/// A whole card, for the reader.
#[derive(Debug, Clone, Serialize)]
pub struct CardDetail {
    pub id: String,
    pub title: String,
    pub card_type: String,
    pub answer: Option<String>,
    /// True when `answer` came from the body's opening paragraph. The reader
    /// hides the callout in that case, because the body already opens with it.
    pub answer_derived: bool,
    /// The markdown body. Rendered by the frontend.
    pub body: String,
    pub volatility: String,
    pub verified: String,
    /// Type-specific frontmatter as JSON, so the reader can render a language
    /// card's tells or an error card's fix ladder without this side needing to
    /// know the shape of all seven card types.
    pub meta: serde_json::Value,
    /// True when the card has passed its own freshness budget.
    pub stale: bool,
}

/// Has this card outlived its own freshness budget?
///
/// The budget is per-card because a blanket date on everything is noise: git's
/// data model has not changed in fifteen years, while the install command for a
/// coding agent can change in a fortnight. A card only goes stale against the
/// volatility its author declared, so the badge means something when it appears.
fn is_stale(volatility: &str, verified: &str) -> bool {
    let days = match volatility {
        "weekly" => 30,
        "quarterly" => 180,
        // "low" still expires, just slowly. Nothing is true forever.
        _ => 730,
    };

    // verified is an ISO date, YYYY-MM-DD. Comparing as text works because the
    // format sorts lexicographically, which avoids pulling in a date library for
    // one subtraction.
    let Some(then) = parse_ymd(verified) else { return false };
    let Some(now) = parse_ymd(&current_date()) else { return false };
    days_between(then, now) > days
}

fn parse_ymd(s: &str) -> Option<(i64, i64, i64)> {
    let mut parts = s.split('-');
    let y = parts.next()?.parse().ok()?;
    let m = parts.next()?.parse().ok()?;
    let d = parts.next()?.parse().ok()?;
    Some((y, m, d))
}

/// Today, as YYYY-MM-DD, from the system clock.
fn current_date() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let days = secs / 86_400;
    let (y, m, d) = civil_from_days(days);
    format!("{y:04}-{m:02}-{d:02}")
}

/// Days since the epoch to a calendar date.
///
/// Howard Hinnant's civil-from-days algorithm. Twenty lines and no dependency,
/// against a date crate pulled in for one conversion.
fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn days_between(a: (i64, i64, i64), b: (i64, i64, i64)) -> i64 {
    days_from_civil(b) - days_from_civil(a)
}

fn days_from_civil((y, m, d): (i64, i64, i64)) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = if m > 2 { m - 3 } else { m + 9 };
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
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

    /// Fetch a whole card for the reader, following intents to their target.
    ///
    /// An intent has no body: it exists only to catch a query in the reader's words
    /// ("oops", "i wrecked it") and point at the card that answers it. Opening
    /// one would show an empty page, so it forwards instead. Resolved here
    /// rather than in the UI so every caller gets the same behavior.
    pub fn card(&self, id: &str) -> Result<Option<CardDetail>> {
        let Some(card) = self.card_raw(id)? else { return Ok(None) };

        if card.card_type == "intent" {
            if let Some(target) = card.meta.get("target").and_then(|t| t.as_str()) {
                // One hop only. An intent pointing at another intent is an
                // authoring mistake, and following a chain would risk a loop.
                if let Some(resolved) = self.card_raw(target)? {
                    return Ok(Some(resolved));
                }
            }
        }
        Ok(Some(card))
    }

    /// Fetch a card exactly as stored, without following intents.
    fn card_raw(&self, id: &str) -> Result<Option<CardDetail>> {
        let mut stmt = self.conn.prepare_cached(
            "SELECT id, title, type, answer, answer_derived, body, volatility, verified, meta
             FROM cards WHERE id = ?1",
        )?;

        let row = stmt.query_row([id], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, Option<String>>(3)?,
                r.get::<_, i64>(4)? != 0,
                r.get::<_, String>(5)?,
                r.get::<_, String>(6)?,
                r.get::<_, String>(7)?,
                r.get::<_, String>(8)?,
            ))
        });

        match row {
            Ok((id, title, card_type, answer, answer_derived, body, volatility, verified, meta)) => {
                let stale = is_stale(&volatility, &verified);
                Ok(Some(CardDetail {
                    id,
                    title,
                    card_type,
                    answer,
                    answer_derived,
                    body,
                    stale,
                    volatility,
                    verified,
                    // A card whose meta failed to parse still renders; it just
                    // loses its type-specific extras.
                    meta: serde_json::from_str(&meta).unwrap_or(serde_json::Value::Null),
                }))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e).context("reading card"),
        }
    }

    /// Load every chunk vector into memory for brute-force scanning.
    ///
    /// No vector index, deliberately. At ~800 chunks a linear scan of 384-dim
    /// vectors is well under a millisecond, while the query encoder that
    /// produces the search vector takes 2 to 6 ms. An index would optimize the
    /// part that is already free and add a storage format to keep working.
    /// Revisit above roughly 100,000 chunks.
    pub fn load_vectors(&self) -> Result<Vec<(String, Vec<f32>)>> {
        let mut stmt = self.conn.prepare(
            "SELECT c.card_id, v.vec FROM chunk_vectors v JOIN chunks c ON c.id = v.chunk_id",
        )?;
        let rows = stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, Vec<u8>>(1)?)))?;

        let mut out = Vec::new();
        for row in rows {
            let (card_id, blob) = row?;
            // A wrong-sized vector means the database was built by a different
            // model. Skipping it is right: mixing vector spaces produces
            // confident nonsense with no error anywhere.
            if let Some(v) = crate::embed::from_blob(&blob) {
                out.push((card_id, v));
            }
        }
        Ok(out)
    }

    /// Whether this corpus has vectors at all.
    ///
    /// False during content authoring, when the text has been rebuilt but the
    /// slow embedding pass has not run. Semantic search is skipped rather than
    /// returning nothing, so the app stays useful mid-build.
    pub fn has_vectors(&self) -> Result<bool> {
        let n: i64 = self.conn.query_row("SELECT count(*) FROM chunk_vectors", [], |r| r.get(0))?;
        Ok(n > 0)
    }

    /// Build the paste identifier from this corpus.
    ///
    /// Loaded once at startup and reused: it compiles a few hundred regexes,
    /// which is cheap once and wasteful on every keystroke.
    pub fn identifier(&self) -> Result<crate::identify::Identifier> {
        crate::identify::Identifier::load(&self.conn)
    }

    /// Read one value out of `build_meta`.
    ///
    /// `None` for a key the compiler did not write, which is the normal state
    /// for a database built before that key existed rather than an error.
    pub fn build_meta(&self, key: &str) -> Result<Option<String>> {
        let mut stmt = self.conn.prepare("SELECT value FROM build_meta WHERE key = ?1")?;
        let mut rows = stmt.query([key])?;
        Ok(match rows.next()? {
            Some(row) => Some(row.get(0)?),
            None => None,
        })
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
    fn date_arithmetic_round_trips() {
        for date in [(2026, 8, 2), (2000, 2, 29), (1970, 1, 1), (2024, 12, 31)] {
            assert_eq!(civil_from_days(days_from_civil(date)), date, "round trip failed for {date:?}");
        }
        assert_eq!(days_between((2026, 1, 1), (2026, 1, 31)), 30);
        // Across a leap day.
        assert_eq!(days_between((2024, 2, 28), (2024, 3, 1)), 2);
    }

    /// The badge only means something if it appears when the card's own budget
    /// is blown, not on a fixed schedule for everything.
    #[test]
    fn staleness_respects_the_cards_own_budget() {
        let recent = current_date();
        assert!(!is_stale("weekly", &recent), "a card verified today is never stale");

        // A two-year-old card: stale at weekly and quarterly, still fine at low.
        let (y, m, d) = parse_ymd(&recent).unwrap();
        let two_years_ago = format!("{:04}-{:02}-{:02}", y - 2, m, d);
        assert!(is_stale("weekly", &two_years_ago));
        assert!(is_stale("quarterly", &two_years_ago));
        assert!(!is_stale("low", &two_years_ago), "git's data model has not moved");
    }

    #[test]
    fn a_malformed_date_is_not_reported_as_stale() {
        // Better to show nothing than a wrong warning.
        assert!(!is_stale("weekly", "not a date"));
        assert!(!is_stale("weekly", ""));
    }

    #[test]
    fn an_unrelated_title_gets_no_boost() {
        assert_eq!(title_boost("Merge conflicts", &terms("python")), 0.0);
        assert_eq!(title_boost("Python", &[]), 0.0);
    }
}
