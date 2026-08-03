//! "What am I looking at?"
//!
//! Paste anything and get back what it is, with the evidence shown.
//!
//! The evidence is the feature. Being told "this is Rust" answers the question
//! once. Being shown
//!
//! ```text
//! Rust   92%
//!   fn        only Rust uses exactly `fn`. Go uses `func`, Kotlin `fun`
//!   let mut   Rust variables cannot change by default
//!   ::        the separator between modules and types
//! ```
//!
//! means that next time she recognizes it herself and does not need the app. A
//! reference tool that makes itself unnecessary is doing its job.
//!
//! Nothing here uses a model. It is pattern matching over weights compiled from
//! the language cards, so it answers instantly and can explain every point it
//! awarded.

pub mod format;

use anyhow::{Context, Result};
use regex::Regex;
use rusqlite::Connection;
use serde::Serialize;
use std::collections::HashMap;

pub use format::{detect as detect_format, Format, FormatVerdict};

/// One piece of evidence for a guess.
#[derive(Debug, Clone, Serialize)]
pub struct Evidence {
    /// What matched, as it appears in the pasted text.
    pub matched: String,
    /// Why it points at this language, stated against a neighbor. This is the
    /// `note` written on the language card.
    pub note: String,
    pub weight: f64,
}

/// One candidate language.
#[derive(Debug, Clone, Serialize)]
pub struct Candidate {
    pub language_id: String,
    pub name: String,
    /// 0 to 100. A share of the total score, not a probability.
    pub confidence: u8,
    pub evidence: Vec<Evidence>,
    /// Present when a confusable pair was settled by a single decisive token.
    pub tiebreak: Option<String>,
}

/// The error card that explains a pasted error, when one matches.
#[derive(Debug, Clone, Serialize)]
pub struct KnownError {
    pub card_id: String,
    pub title: String,
    /// What actually went wrong, in plain language.
    pub means: String,
}

/// The whole answer.
#[derive(Debug, Clone, Serialize)]
pub struct Identification {
    pub format: Format,
    /// Why the format was decided, in one line.
    pub format_because: String,
    pub candidates: Vec<Candidate>,
    /// Set when the top two are close enough that the honest answer is "one of
    /// these two". Saying "Java, 51%" when it is a coin flip is worse than
    /// saying so.
    pub ambiguous: bool,
    /// The card explaining this specific error, when the paste matches one.
    ///
    /// This is what closes the loop. "This is a Python crash" is half an answer.
    /// The half she wanted is what the error means and what to try first.
    pub known_error: Option<KnownError>,
}

/// A compiled signal, loaded once and reused across calls.
struct Signal {
    language_id: String,
    kind: String,
    weight: f64,
    note: String,
    /// The literal text, for display and for token matching.
    pattern: String,
    /// Compiled form, for the kinds that need one.
    regex: Option<Regex>,
}

struct Tiebreak {
    language_id: String,
    versus: String,
    favors: String,
    regex: Option<Regex>,
    pattern: String,
    kind: String,
}

/// The scoring tables, read out of the database once.
pub struct Identifier {
    signals: Vec<Signal>,
    tiebreaks: Vec<Tiebreak>,
    names: HashMap<String, String>,
    errors: Vec<ErrorPattern>,
}

struct ErrorPattern {
    card_id: String,
    title: String,
    means: String,
    regex: Regex,
}

/// Below this the guess is not worth showing. A handful of incidental token
/// matches will score a few points against almost anything.
const MIN_SCORE: f64 = 6.0;

/// When the runner-up is within this fraction of the leader, say so rather than
/// pretending to have decided.
const AMBIGUITY_BAND: f64 = 0.80;

impl Identifier {
    /// Load the scoring tables.
    pub fn load(conn: &Connection) -> Result<Self> {
        let mut signals = Vec::new();
        let mut stmt = conn
            .prepare("SELECT language_id, pattern, kind, weight, note FROM language_signals")
            .context("reading language_signals")?;

        for row in stmt.query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, f64>(3)?,
                r.get::<_, String>(4)?,
            ))
        })? {
            let (language_id, pattern, kind, weight, note) = row?;
            // A card can carry an invalid regex. Skipping one signal is far
            // better than refusing to identify anything, and the content linter
            // is where a bad pattern should be caught.
            let regex = compile(&pattern, &kind);
            signals.push(Signal { language_id, kind, weight, note, pattern, regex });
        }

        let mut tiebreaks = Vec::new();
        let mut stmt = conn
            .prepare("SELECT language_id, versus, pattern, kind, favors FROM language_tiebreaks")
            .context("reading language_tiebreaks")?;
        for row in stmt.query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, String>(3)?,
                r.get::<_, String>(4)?,
            ))
        })? {
            let (language_id, versus, pattern, kind, favors) = row?;
            let regex = compile(&pattern, &kind);
            tiebreaks.push(Tiebreak { language_id, versus, favors, regex, pattern, kind });
        }

        let mut names = HashMap::new();
        let mut stmt = conn.prepare("SELECT id, title FROM cards WHERE type = 'language'")?;
        for row in stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))? {
            let (id, title) = row?;
            names.insert(id, title);
        }

        // Error cards, with their `means` pulled out of the JSON meta blob.
        let mut errors = Vec::new();
        let mut stmt = conn.prepare(
            "SELECT e.card_id, c.title, json_extract(c.meta, '$.means'), e.pattern
             FROM error_patterns e JOIN cards c ON c.id = e.card_id",
        )?;
        for row in stmt.query_map([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, Option<String>>(2)?.unwrap_or_default(),
                r.get::<_, String>(3)?,
            ))
        })? {
            let (card_id, title, means, pattern) = row?;
            // Error text is matched case-insensitively: the same failure is
            // capitalized differently by different tools, and by PowerShell
            // versus Git Bash for the same underlying error.
            if let Ok(regex) = Regex::new(&format!("(?i){pattern}")) {
                errors.push(ErrorPattern { card_id, title, means, regex });
            }
        }

        Ok(Self { signals, tiebreaks, names, errors })
    }

    /// The error card whose patterns match this text, if any.
    ///
    /// First match wins. Patterns are anchored on the stable part of a message
    /// (the error class and fixed wording) rather than on paths or line numbers,
    /// so overlaps are rare and the first hit is the right one.
    fn match_error(&self, text: &str) -> Option<KnownError> {
        self.errors.iter().find(|e| e.regex.is_match(text)).map(|e| KnownError {
            card_id: e.card_id.clone(),
            title: e.title.clone(),
            means: e.means.clone(),
        })
    }

    /// Identify pasted text.
    pub fn identify(&self, text: &str) -> Identification {
        let verdict = format::detect(text);

        // Some shapes are the answer. A diff is a diff whatever is inside it,
        // and prose is someone having pasted the wrong thing. Scoring those
        // against language tells produces a confident answer to a question
        // nobody asked.
        let score_languages = !matches!(verdict.format, Format::Diff | Format::Prose);

        let mut candidates = if score_languages { self.score(text) } else { Vec::new() };

        // A stack trace names its language through the error patterns, but the
        // useful answer is what the error means, so only the top guess is kept.
        if verdict.format == Format::StackTrace {
            candidates.truncate(1);
        }

        let ambiguous = candidates.len() >= 2
            && candidates[1].evidence.iter().map(|e| e.weight).sum::<f64>()
                >= candidates[0].evidence.iter().map(|e| e.weight).sum::<f64>() * AMBIGUITY_BAND;

        // Any paste can contain a known error, not only one the router called an
        // error: a build log and a stack trace both end in one.
        let known_error = self.match_error(text);

        Identification {
            format: verdict.format,
            format_because: verdict.because,
            candidates,
            ambiguous,
            known_error,
        }
    }

    fn score(&self, text: &str) -> Vec<Candidate> {
        let mut totals: HashMap<&str, f64> = HashMap::new();
        let mut evidence: HashMap<&str, Vec<Evidence>> = HashMap::new();

        for sig in &self.signals {
            let Some(hit) = matches(sig, text) else { continue };

            *totals.entry(sig.language_id.as_str()).or_insert(0.0) += sig.weight;

            // Negative evidence still counts against the score but is not shown
            // as a reason. "This is Rust because it is not PHP" is not an
            // explanation anyone wants.
            if sig.weight > 0.0 {
                evidence.entry(sig.language_id.as_str()).or_default().push(Evidence {
                    matched: hit,
                    note: sig.note.clone(),
                    weight: sig.weight,
                });
            }
        }

        let mut ranked: Vec<(&str, f64)> =
            totals.into_iter().filter(|(_, s)| *s >= MIN_SCORE).collect();
        ranked.sort_by(|a, b| {
            b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal).then_with(|| a.0.cmp(b.0))
        });
        ranked.truncate(5);

        // Apply the tiebreaks. Weights are a blunt instrument for a pair like
        // Java and C#, where the cards agree on almost every token; one decisive
        // token settles it and the card already says which.
        let mut tiebreak_note: HashMap<&str, String> = HashMap::new();
        if ranked.len() >= 2 {
            let (top, second) = (ranked[0].0.to_string(), ranked[1].0.to_string());
            let close = ranked[1].1 >= ranked[0].1 * AMBIGUITY_BAND;

            if close {
                for tb in &self.tiebreaks {
                    let pair = (tb.language_id.as_str(), tb.versus.as_str());
                    let relevant = (pair.0 == top && pair.1 == second)
                        || (pair.0 == second && pair.1 == top);
                    if !relevant {
                        continue;
                    }
                    let sig = Signal {
                        language_id: tb.favors.clone(),
                        kind: tb.kind.clone(),
                        weight: 0.0,
                        note: String::new(),
                        pattern: tb.pattern.clone(),
                        regex: tb.regex.clone(),
                    };
                    if let Some(hit) = matches(&sig, text) {
                        if let Some(entry) = ranked.iter_mut().find(|(id, _)| *id == tb.favors) {
                            entry.1 += 15.0;
                            let favors_name =
                                self.names.get(&tb.favors).cloned().unwrap_or(tb.favors.clone());
                            tiebreak_note.insert(
                                entry.0,
                                format!("`{hit}` settles it in favor of {favors_name}"),
                            );
                        }
                    }
                }
                ranked.sort_by(|a, b| {
                    b.1.partial_cmp(&a.1)
                        .unwrap_or(std::cmp::Ordering::Equal)
                        .then_with(|| a.0.cmp(b.0))
                });
            }
        }

        let total: f64 = ranked.iter().map(|(_, s)| s.max(0.0)).sum();

        ranked
            .into_iter()
            .map(|(id, score)| {
                let mut ev = evidence.remove(id).unwrap_or_default();
                // Strongest reasons first, and only the ones worth reading.
                ev.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap_or(std::cmp::Ordering::Equal));
                ev.truncate(6);

                Candidate {
                    name: self.names.get(id).cloned().unwrap_or_else(|| id.to_string()),
                    language_id: id.to_string(),
                    confidence: if total > 0.0 {
                        ((score.max(0.0) / total) * 100.0).round().clamp(0.0, 100.0) as u8
                    } else {
                        0
                    },
                    evidence: ev,
                    tiebreak: tiebreak_note.get(id).cloned(),
                }
            })
            .collect()
    }
}

/// Build the regex for a signal kind, if that kind needs one.
fn compile(pattern: &str, kind: &str) -> Option<Regex> {
    match kind {
        // A token must match on word boundaries, or `fn` matches inside
        // "function" and every JavaScript file scores as Rust.
        "token" => Regex::new(&format!(r"\b{}\b", regex::escape(pattern))).ok(),
        // Operators and sigils are punctuation, so word boundaries would never
        // match. `::` and `$` have no word edges.
        "operator" | "sigil" => Regex::new(&regex::escape(pattern)).ok(),
        "line_start" => Regex::new(&format!(r"(?m)^\s*{}", regex::escape(pattern))).ok(),
        "extension" => Regex::new(&format!(r"{}\b", regex::escape(pattern))).ok(),
        // Written as a regex by the card author.
        "regex" | "error_regex" => Regex::new(pattern).ok(),
        _ => Regex::new(&regex::escape(pattern)).ok(),
    }
}

/// Does this signal appear in the text? Returns what matched, for display.
fn matches(sig: &Signal, text: &str) -> Option<String> {
    let re = sig.regex.as_ref()?;
    let m = re.find(text)?;
    let hit = m.as_str().trim();
    if hit.is_empty() {
        // A zero-width match tells the user nothing, so show the pattern.
        return Some(sig.pattern.clone());
    }
    Some(hit.to_string())
}

impl Clone for Signal {
    fn clone(&self) -> Self {
        Self {
            language_id: self.language_id.clone(),
            kind: self.kind.clone(),
            weight: self.weight,
            note: self.note.clone(),
            pattern: self.pattern.clone(),
            regex: self.regex.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sig(lang: &str, pattern: &str, kind: &str, weight: f64) -> Signal {
        Signal {
            language_id: lang.into(),
            kind: kind.into(),
            weight,
            note: format!("{pattern} points at {lang}"),
            pattern: pattern.into(),
            regex: compile(pattern, kind),
        }
    }

    /// The bug that makes a naive classifier useless: `fn` inside "function"
    /// scores every JavaScript file as Rust.
    #[test]
    fn a_token_does_not_match_inside_a_longer_word() {
        let s = sig("rust", "fn", "token", 9.0);
        assert!(matches(&s, "fn main() {}").is_some());
        assert!(matches(&s, "function main() {}").is_none(), "`fn` must not match inside `function`");
    }

    /// Operators have no word boundaries, so the token rule would never fire.
    #[test]
    fn operators_match_without_word_boundaries() {
        assert!(matches(&sig("rust", "::", "operator", 5.0), "std::io::Write").is_some());
        assert!(matches(&sig("php", "$", "sigil", 5.0), "$name = 1;").is_some());
    }

    #[test]
    fn line_start_anchors_per_line() {
        let s = sig("c", "#include", "line_start", 9.0);
        assert!(matches(&s, "int x;\n#include <stdio.h>").is_some());
        assert!(matches(&s, "// see #include docs").is_none(), "must be at the start of a line");
    }

    /// A card with a broken regex must cost that one signal, not the feature.
    #[test]
    fn an_invalid_regex_is_skipped_not_fatal() {
        let s = sig("bad", "([unclosed", "regex", 5.0);
        assert!(s.regex.is_none());
        assert!(matches(&s, "anything").is_none());
    }

    #[test]
    fn an_empty_paste_identifies_nothing_and_does_not_panic() {
        let id = Identifier { signals: vec![], tiebreaks: vec![], names: HashMap::new(), errors: vec![] };
        let out = id.identify("");
        assert!(out.candidates.is_empty());
        assert_eq!(out.format, Format::Prose);
    }

    #[test]
    fn evidence_is_ordered_strongest_first_and_capped() {
        let signals = (0..10)
            .map(|i| sig("rust", &format!("tok{i}"), "token", i as f64 + 1.0))
            .collect();
        let id = Identifier {
            signals,
            tiebreaks: vec![],
            errors: vec![],
            names: HashMap::from([("rust".to_string(), "Rust".to_string())]),
        };
        let text = (0..10).map(|i| format!("tok{i}")).collect::<Vec<_>>().join(" ");
        let out = id.score(&text);

        assert_eq!(out.len(), 1);
        assert!(out[0].evidence.len() <= 6, "too much evidence is noise");
        let weights: Vec<f64> = out[0].evidence.iter().map(|e| e.weight).collect();
        assert!(weights.windows(2).all(|w| w[0] >= w[1]), "strongest reason first");
    }

    /// Negative evidence must move the score without appearing as a reason.
    /// "This is Rust because it is not PHP" explains nothing.
    #[test]
    fn rules_out_lower_the_score_but_are_not_shown_as_reasons() {
        let id = Identifier {
            signals: vec![
                sig("rust", "fn", "token", 9.0),
                sig("rust", "def", "token", -12.0),
            ],
            tiebreaks: vec![],
            errors: vec![],
            names: HashMap::from([("rust".to_string(), "Rust".to_string())]),
        };

        let clean = id.score("fn main() {}");
        assert_eq!(clean.len(), 1);
        assert_eq!(clean[0].evidence.len(), 1);

        // With the ruling-out token present, the score drops below the floor.
        assert!(id.score("fn def").is_empty(), "negative evidence should sink the guess");
    }

    #[test]
    fn a_weak_incidental_match_is_not_reported() {
        let id = Identifier {
            signals: vec![sig("rust", "fn", "token", 2.0)],
            tiebreaks: vec![],
            errors: vec![],
            names: HashMap::new(),
        };
        assert!(id.score("fn").is_empty(), "2 points is below the floor and means nothing");
    }

    /// A diff of Python is a diff. Language scoring is skipped entirely rather
    /// than answering a question nobody asked.
    #[test]
    fn a_diff_skips_language_scoring() {
        let id = Identifier {
            signals: vec![sig("python", "import", "token", 9.0)],
            tiebreaks: vec![],
            errors: vec![],
            names: HashMap::new(),
        };
        let out = id.identify("diff --git a/x.py b/x.py\n@@ -1 +1 @@\n-import os\n+import sys");
        assert_eq!(out.format, Format::Diff);
        assert!(out.candidates.is_empty());
    }

    #[test]
    fn confidences_are_a_share_of_the_total() {
        let id = Identifier {
            signals: vec![sig("a", "aaa", "token", 20.0), sig("b", "bbb", "token", 10.0)],
            tiebreaks: vec![],
            errors: vec![],
            names: HashMap::new(),
        };
        let out = id.score("aaa bbb");
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].confidence, 67);
        assert_eq!(out[1].confidence, 33);
    }
}
