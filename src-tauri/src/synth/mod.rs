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

//! Answering a question from the cards, without generating anything.
//!
//! # Why this is extractive rather than generated
//!
//! The plan was for a small local model to read the retrieved cards and write a
//! short cited answer. It was gated on a measurement, the measurement was taken,
//! and the answer was no. `docs/PHASE0-LLM-GATE.md` has the numbers; the short
//! version is that the quantization small enough for a target GPU broke the
//! machine-readable abstention contract, the one that held it needed 2.7 GB of
//! VRAM or twenty seconds of CPU, and the single failure in fifteen answered a
//! question about database connection strings out of the card that exists to warn
//! against pasting database connection strings.
//!
//! So this selects sentences instead of writing them.
//!
//! # Why that is not a consolation prize
//!
//! An extracted sentence cannot be wrong about the corpus, because it *is* the
//! corpus. It cannot invert a warning into advice, cannot cite a card it did not
//! read, and cannot invent a flag. The worst case is that it picks a less
//! relevant sentence than a human would, which they can see immediately because
//! the card it came from is named right there.
//!
//! It also returns in about a millisecond rather than twenty seconds, which for
//! a tool whose entire claim is being faster than opening a browser tab is not a
//! side benefit.

use crate::embed::cosine;
use serde::Serialize;

/// One sentence lifted from a card, with its source attached.
#[derive(Debug, Clone, Serialize)]
pub struct Excerpt {
    pub card_id: String,
    pub card_title: String,
    /// The sentence, verbatim. Never rewritten, never summarized.
    pub text: String,
    /// The heading it sits under, so they can find it in the card.
    pub heading_path: String,
    pub score: f64,
}

/// What the app shows above the result list when a question looks answerable.
#[derive(Debug, Clone, Serialize)]
pub struct Extract {
    pub excerpts: Vec<Excerpt>,
    /// True when nothing scored well enough to be worth showing. The UI shows
    /// the plain result list in that case, which is the honest outcome: the
    /// cards are there, and we do not claim one of them answers the question.
    pub weak: bool,
}

/// A candidate chunk to pull sentences from.
pub struct Passage<'a> {
    pub card_id: &'a str,
    pub card_title: &'a str,
    pub heading_path: &'a str,
    pub text: &'a str,
}

/// How good the BEST sentence has to be before any answer is shown at all.
///
/// Two thresholds rather than one, because a single cutoff cannot separate
/// "the corpus answers this" from "the corpus contains adjacent words".
/// Measured over the real corpus:
///
/// ```text
///   question                              best sentence
///   what does the staging area do              0.821    covered
///   how do i undo the last commit              0.811    covered
///   what is a merge conflict                   0.721    covered
///   what is the best react state library       0.644    NOT covered
///   how do i configure a jenkins pipeline      0.601    NOT covered
/// ```
///
/// The first question is whether there is an answer here at all, and that is
/// what the leader answers. If the best sentence in the whole corpus only
/// reaches 0.64, nothing is shown, because a quote block headed "from the guide"
/// implies an answer and three tangential sentences is a worse outcome than an
/// honest empty space with the result list underneath it.
const LEAD_SCORE: f64 = 0.66;

/// Once a leader clears the bar, supporting sentences only have to be relevant.
/// They are read as context for an answer that already exists, not as the answer.
const MIN_SENTENCE_SCORE: f64 = 0.60;

/// How many sentences to show. Three is the point where it stops reading as an
/// answer and starts reading as a wall.
const MAX_EXCERPTS: usize = 3;

/// Strip fenced code blocks and headings, keeping only running prose.
///
/// This matters more than it sounds. A first version extracted from the raw
/// body and produced excerpts like
///
/// ```text
/// On screen it looks like this:
///
/// text
/// Auto-merging src/config.js
/// CONFLICT (content): Merge conflict in src/config.js
/// ```
///
/// which is a sentence, a stray fence tag, and four lines of terminal output
/// presented as an answer. The card renders that beautifully; a quote block
/// does not.
fn prose_only(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut in_fence = false;

    for line in text.lines() {
        let t = line.trim_start();
        if t.starts_with("```") {
            in_fence = !in_fence;
            // A fence is a hard boundary: a sentence never runs across one.
            out.push('\n');
            continue;
        }
        if in_fence || t.starts_with('#') || t.starts_with('|') {
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

/// Split prose into sentences.
///
/// Aware of the two things that break naive splitting in this corpus: a period
/// inside a filename or version (`Cargo.toml`, `v1.0.0`) and a period ending an
/// acronym. Splitting on those produces fragments that read as broken English
/// when quoted.
pub fn sentences(text: &str) -> Vec<String> {
    let cleaned = prose_only(text);
    let bytes = cleaned.as_bytes();
    let mut out = Vec::new();
    let mut start = 0;

    for i in 0..bytes.len() {
        // A blank line ends a sentence even without punctuation, which is what
        // stops a heading line fusing onto the paragraph below it.
        let hard_break = bytes[i] == b'\n'
            && bytes.get(i + 1).is_some_and(|b| *b == b'\n' || *b == b'\r');

        let terminator = matches!(bytes[i], b'.' | b'!' | b'?')
            && matches!(bytes.get(i + 1), Some(b' ') | Some(b'\n') | None)
            // A single capital before the dot is usually an initial or acronym.
            && !(i > 0 && bytes[i - 1].is_ascii_uppercase());

        if !hard_break && !terminator {
            continue;
        }

        let piece = cleaned[start..=i].trim();
        if piece.len() > 20 {
            out.push(piece.to_string());
        }
        start = i + 1;
    }

    let tail = cleaned[start..].trim();
    if tail.len() > 20 {
        out.push(tail.to_string());
    }
    out
}

/// Strip markdown and collapse whitespace.
///
/// A sentence lifted out of a body carries its original line wrapping, so
/// without this it renders with hard breaks in the middle of a clause.
fn clean(sentence: &str) -> String {
    let stripped = sentence
        .trim()
        .trim_start_matches(['-', '*', '>', '#', ' '])
        .replace("**", "")
        .replace('`', "");

    // Collapse every run of whitespace, newlines included, to one space.
    stripped.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Pick the sentences that best answer the query.
///
/// Each candidate sentence is embedded and compared against the query vector.
/// This is the same machinery as semantic search, one level finer: search finds
/// the card, this finds the line.
///
/// `embed` is passed in rather than owned so the caller can share one loaded
/// model, and so this function stays testable without one.
pub fn extract<F>(query_vec: &[f32], passages: &[Passage<'_>], mut embed: F) -> Extract
where
    F: FnMut(&[String]) -> Option<Vec<Vec<f32>>>,
{
    // Gather candidates first, so the model is called once rather than per
    // sentence. A forward pass has fixed overhead that dominates at this size.
    let mut candidates: Vec<(usize, String)> = Vec::new();
    for (i, p) in passages.iter().enumerate() {
        for s in sentences(p.text) {
            let c = clean(&s);
            // Very short lines are headings, list markers, or fragments. Very
            // long ones are usually a run-on that quotes badly.
            if c.len() >= 40 && c.len() <= 400 {
                candidates.push((i, c));
            }
        }
    }

    if candidates.is_empty() {
        return Extract { excerpts: Vec::new(), weak: true };
    }

    let texts: Vec<String> = candidates.iter().map(|(_, s)| s.clone()).collect();
    let Some(vectors) = embed(&texts) else {
        return Extract { excerpts: Vec::new(), weak: true };
    };

    let mut scored: Vec<Excerpt> = candidates
        .iter()
        .zip(&vectors)
        .map(|((pi, text), v)| {
            let p = &passages[*pi];
            Excerpt {
                card_id: p.card_id.to_string(),
                card_title: p.card_title.to_string(),
                text: text.clone(),
                heading_path: p.heading_path.to_string(),
                score: cosine(query_vec, v) as f64,
            }
        })
        .filter(|e| e.score >= MIN_SENTENCE_SCORE)
        .collect();

    scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    // No leader, no answer. Showing the runners-up on their own would put three
    // tangential sentences under a heading that promises an answer.
    if scored.first().is_none_or(|e| e.score < LEAD_SCORE) {
        return Extract { excerpts: Vec::new(), weak: true };
    }

    // At most one sentence per card. Three lines from the same paragraph is a
    // worse answer than three lines from three cards, and it also hides how
    // narrow the evidence really is.
    let mut seen = std::collections::HashSet::new();
    scored.retain(|e| seen.insert(e.card_id.clone()));
    scored.truncate(MAX_EXCERPTS);

    Extract { weak: scored.is_empty(), excerpts: scored }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_on_sentence_ends() {
        let out = sentences(
            "A merge conflict means git could not combine two changes. The markers show both versions and you choose.",
        );
        assert_eq!(out.len(), 2);
        assert!(out[1].starts_with("The markers"));
    }

    /// The failure that makes naive splitting unusable in this corpus: a quoted
    /// fragment reading "Put a Cargo" is worse than no answer at all.
    #[test]
    fn a_period_inside_a_filename_does_not_split() {
        let out = sentences(
            "Put a Cargo.toml at the root and the project is Rust. Nothing else is needed here.",
        );
        assert_eq!(out.len(), 2, "got: {out:?}");
        assert!(out[0].contains("Cargo.toml"));
    }

    #[test]
    fn version_numbers_do_not_split() {
        let out = sentences("The tag v1.0.0 marks a release point in the history of the project.");
        assert_eq!(out.len(), 1, "got: {out:?}");
    }

    /// The defect that made the first version unusable: a fenced block quoted
    /// as prose produced "On screen it looks like this: text Auto-merging
    /// src/config.js CONFLICT..." presented as an answer.
    #[test]
    fn fenced_code_never_becomes_a_sentence() {
        let body = "Here is what you will see on your screen when it happens.

```text
Auto-merging src/config.js
CONFLICT (content): Merge conflict in src/config.js
```

Pick the version you want and commit the result.";
        let out = sentences(body);
        assert!(
            out.iter().all(|s| !s.contains("Auto-merging") && !s.contains("CONFLICT")),
            "terminal output leaked into a quote: {out:?}"
        );
        assert!(out.iter().any(|s| s.contains("Pick the version")));
    }

    #[test]
    fn headings_do_not_fuse_onto_the_paragraph_below() {
        let out = sentences("## Where the tests live

Most projects keep them in a tests folder at the root.");
        assert!(
            out.iter().all(|s| !s.contains("Where the tests live")),
            "a heading was quoted as prose: {out:?}"
        );
    }

    /// A sentence carries its original line wrapping, so without collapsing it
    /// renders with a hard break in the middle of a clause.
    #[test]
    fn line_wrapping_is_collapsed() {
        assert_eq!(
            clean("This is where you find out the
thing needs Postgres before you start."),
            "This is where you find out the thing needs Postgres before you start."
        );
    }

    #[test]
    fn markdown_is_stripped_from_a_quote() {
        assert_eq!(clean("- **Never** commit a `.env` file"), "Never commit a .env file");
        assert_eq!(clean("> A quoted line"), "A quoted line");
    }

    fn passages<'a>() -> Vec<Passage<'a>> {
        vec![
            Passage {
                card_id: "d7",
                card_title: "Merge conflicts",
                heading_path: "Merge conflicts > More",
                text: "A merge conflict means git could not combine two changes automatically. The markers show you both versions so that you can choose between them.",
            },
            Passage {
                card_id: "rust",
                card_title: "Rust",
                heading_path: "Rust",
                text: "Rust is a compiled systems language that refuses to build until memory is safe. Its packages are called crates, which sounds wrong for a week.",
            },
        ]
    }

    /// Nothing is generated, so every returned string must appear in a passage.
    /// This is the property the whole module exists for.
    #[test]
    fn every_excerpt_is_verbatim_from_a_passage() {
        let ps = passages();
        // Score everything highly so nothing is filtered on relevance.
        let out = extract(&[1.0, 0.0], &ps, |texts| {
            Some(texts.iter().map(|_| vec![1.0, 0.0]).collect())
        });

        assert!(!out.excerpts.is_empty());
        for e in &out.excerpts {
            let source = ps.iter().find(|p| p.card_id == e.card_id).unwrap();
            let stripped = source.text.replace("**", "").replace('`', "");
            assert!(
                stripped.contains(e.text.trim_end_matches('.')),
                "excerpt {:?} is not verbatim from {}",
                e.text,
                e.card_id
            );
        }
    }

    /// The two-tier threshold. A pile of tangential sentences with no strong
    /// leader is the exact shape of a question the corpus does not answer, and
    /// showing it under a heading that promises an answer is the failure this
    /// module exists to avoid.
    #[test]
    fn a_pile_of_mediocre_sentences_is_not_an_answer() {
        // Every sentence scores 0.62: above the supporting bar, below the lead.
        let v = |c: f32| vec![c, (1.0 - c * c).sqrt()];
        let out = extract(&[1.0, 0.0], &passages(), |texts| {
            Some(texts.iter().map(|_| v(0.62)).collect())
        });
        assert!(out.weak, "no leader means no answer");
        assert!(out.excerpts.is_empty());
    }

    #[test]
    fn a_strong_leader_carries_its_supporting_sentences() {
        // First candidate strong, the rest merely relevant.
        let mut first = true;
        let out = extract(&[1.0, 0.0], &passages(), |texts| {
            Some(
                texts
                    .iter()
                    .map(|_| {
                        let c: f32 = if first { 0.95 } else { 0.62 };
                        first = false;
                        vec![c, (1.0 - c * c).sqrt()]
                    })
                    .collect(),
            )
        });
        assert!(!out.weak);
        assert!(!out.excerpts.is_empty());
    }

    #[test]
    fn weak_matches_are_reported_as_weak_rather_than_shown() {
        // Orthogonal vectors score zero, well below the floor.
        let out = extract(&[1.0, 0.0], &passages(), |texts| {
            Some(texts.iter().map(|_| vec![0.0, 1.0]).collect())
        });
        assert!(out.weak);
        assert!(out.excerpts.is_empty(), "a weak match must not be dressed up as an answer");
    }

    /// Three lines from one paragraph hides how narrow the evidence is.
    #[test]
    fn at_most_one_sentence_per_card() {
        let out = extract(&[1.0, 0.0], &passages(), |texts| {
            Some(texts.iter().map(|_| vec![1.0, 0.0]).collect())
        });
        let mut ids: Vec<&str> = out.excerpts.iter().map(|e| e.card_id.as_str()).collect();
        ids.sort_unstable();
        let before = ids.len();
        ids.dedup();
        assert_eq!(before, ids.len(), "one card contributed twice");
    }

    #[test]
    fn no_passages_and_no_model_are_both_survivable() {
        let empty = extract(&[1.0, 0.0], &[], |_| None);
        assert!(empty.weak);

        let no_model = extract(&[1.0, 0.0], &passages(), |_| None);
        assert!(no_model.weak, "a missing model degrades to no answer, never to a wrong one");
    }
}
