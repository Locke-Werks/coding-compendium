//! Extracting the identifier's scoring table out of the language cards.
//!
//! The cards are the single source of truth for both halves of the feature: the
//! prose the reader sees and the weights the classifier scores with. Editing a tell in
//! the markdown changes the classifier on the next build.
//!
//! That property is the whole reason the tells carry a `note` explaining
//! themselves against a neighbor language. The note is not documentation for the
//! card; it is the evidence line the identifier displays. Being told "this is
//! Rust" answers the question once. Being shown "because `fn`, and Go would say
//! `func`" means the next time they recognize it themselves.

use anyhow::{Context, Result};
use rusqlite::Transaction;
use serde::Deserialize;

use super::frontmatter::{Card, CardKind};

/// The language-specific frontmatter, read back out of the JSON meta blob.
///
/// Every field is optional at this layer even though the JSON Schema requires
/// most of them, because the content linter has already rejected a malformed
/// card before the compiler runs. Failing softly here would only turn a caught
/// error into a silently weaker classifier.
#[derive(Debug, Default, Deserialize)]
pub struct LanguageMeta {
    #[serde(default)]
    pub tells: Vec<Tell>,
    #[serde(default)]
    pub rules_out: Vec<RuleOut>,
    #[serde(default)]
    pub project_fingerprint: Option<ProjectFingerprint>,
    #[serde(default)]
    pub confusable_with: Vec<Confusable>,
    #[serde(default)]
    pub errors_look_like: Option<ErrorsLookLike>,
    #[serde(default)]
    pub extensions: Vec<String>,
    // These three mirror fields the language schema requires and the reader
    // renders straight from the card body. The compiler parses them so that a
    // malformed one fails the build rather than surfacing later as a bad card,
    // but nothing downstream of the identifier reads them.
    #[allow(dead_code)]
    #[serde(default)]
    pub aka: Vec<String>,
    #[allow(dead_code)]
    #[serde(default)]
    pub likelihood: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Tell {
    pub pattern: String,
    #[serde(default = "default_kind")]
    pub kind: String,
    pub weight: f64,
    #[serde(default)]
    pub note: String,
}

#[derive(Debug, Deserialize)]
pub struct RuleOut {
    pub pattern: String,
    #[serde(default = "default_kind")]
    pub kind: String,
    #[serde(default)]
    pub because: String,
}

#[derive(Debug, Deserialize)]
pub struct ProjectFingerprint {
    #[serde(default)]
    pub manifests: Vec<Manifest>,
    #[serde(default)]
    pub lockfiles: Vec<String>,
    // Parsed for schema fidelity; the manifests and lockfiles are what the
    // project scan actually keys on.
    #[allow(dead_code)]
    #[serde(default)]
    pub build_dirs: Vec<String>,
    #[serde(default)]
    pub entry_points: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub file: String,
    #[serde(default)]
    pub decisive: bool,
    #[serde(default)]
    pub note: String,
}

#[derive(Debug, Deserialize)]
pub struct Confusable {
    pub language: String,
    // Prose for the reader, rendered from the card body. The identifier settles
    // a confusion with `tiebreak`, which is machine-readable and weighted.
    #[allow(dead_code)]
    #[serde(default)]
    pub settle_it: String,
    #[serde(default)]
    pub tiebreak: Option<Tiebreak>,
}

#[derive(Debug, Deserialize)]
pub struct Tiebreak {
    pub pattern: String,
    #[serde(default = "default_kind")]
    pub kind: String,
    pub favors: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorsLookLike {
    #[serde(default)]
    pub patterns: Vec<String>,
}

fn default_kind() -> String {
    "token".to_string()
}

/// How much a rules_out hit counts against a language.
///
/// Negative evidence is stronger than positive evidence here, and deliberately
/// so. A single `<?php` proves the file is not Rust no matter how many
/// Rust-shaped tokens happen to appear around it, whereas any one positive tell
/// can show up in a comment or a string by accident.
const RULE_OUT_WEIGHT: f64 = -12.0;

/// Write the scoring tables for every language card.
pub fn write_language_signals(tx: &Transaction<'_>, cards: &[Card]) -> Result<LanguageStats> {
    let mut stats = LanguageStats::default();

    let mut signal = tx.prepare(
        "INSERT INTO language_signals (language_id, pattern, kind, weight, note)
         VALUES (?1, ?2, ?3, ?4, ?5)",
    )?;
    let mut manifest = tx.prepare(
        "INSERT INTO project_manifests (language_id, file, decisive, note) VALUES (?1, ?2, ?3, ?4)",
    )?;
    let mut tiebreak = tx.prepare(
        "INSERT INTO language_tiebreaks (language_id, versus, pattern, kind, favors)
         VALUES (?1, ?2, ?3, ?4, ?5)",
    )?;

    for card in cards.iter().filter(|c| c.kind == CardKind::Language) {
        let meta: LanguageMeta = serde_json::from_str(&card.meta_json)
            .with_context(|| format!("reading language fields from {}", card.id))?;

        for tell in &meta.tells {
            signal.execute(rusqlite::params![
                card.id,
                tell.pattern,
                tell.kind,
                tell.weight,
                tell.note
            ])?;
            stats.tells += 1;
        }

        for rule in &meta.rules_out {
            let note = if rule.because.is_empty() {
                String::new()
            } else {
                format!("points at {} instead", rule.because)
            };
            signal.execute(rusqlite::params![
                card.id,
                rule.pattern,
                rule.kind,
                RULE_OUT_WEIGHT,
                note
            ])?;
            stats.rules_out += 1;
        }

        // A file extension is decisive on its own when it belongs to one
        // language, which is why it outweighs any single token. `.rs` settles a
        // file that `fn` alone only suggests.
        for ext in &meta.extensions {
            signal.execute(rusqlite::params![
                card.id,
                ext,
                "extension",
                9.0,
                format!("the {ext} extension")
            ])?;
            stats.tells += 1;
        }

        if let Some(fp) = &meta.project_fingerprint {
            for m in &fp.manifests {
                manifest.execute(rusqlite::params![card.id, m.file, m.decisive as i64, m.note])?;
                stats.manifests += 1;
            }
            for f in fp.lockfiles.iter().chain(&fp.entry_points) {
                manifest.execute(rusqlite::params![card.id, f, 0i64, ""])?;
                stats.manifests += 1;
            }
        }

        for c in &meta.confusable_with {
            if let Some(t) = &c.tiebreak {
                tiebreak.execute(rusqlite::params![
                    card.id,
                    c.language,
                    t.pattern,
                    t.kind,
                    t.favors
                ])?;
                stats.tiebreaks += 1;
            }
        }

        // Error output is often all the reader has: they paste a stack trace, not the
        // code that produced it. These patterns route it to a language.
        if let Some(errs) = &meta.errors_look_like {
            for p in &errs.patterns {
                signal.execute(rusqlite::params![
                    card.id,
                    p,
                    "error_regex",
                    10.0,
                    "matches this language's error format"
                ])?;
                stats.error_patterns += 1;
            }
        }
    }

    // Error cards carry the patterns that route a pasted error to the card
    // explaining it. Same table shape, different source.
    let mut err = tx.prepare("INSERT INTO error_patterns (card_id, pattern) VALUES (?1, ?2)")?;
    for card in cards.iter().filter(|c| c.kind == CardKind::Error) {
        #[derive(Deserialize)]
        struct ErrorMeta {
            #[serde(default)]
            patterns: Vec<String>,
        }
        let meta: ErrorMeta = serde_json::from_str(&card.meta_json)
            .with_context(|| format!("reading error patterns from {}", card.id))?;
        for p in &meta.patterns {
            err.execute(rusqlite::params![card.id, p])?;
            stats.error_cards += 1;
        }
    }

    Ok(stats)
}

#[derive(Debug, Default)]
pub struct LanguageStats {
    pub error_cards: usize,
    pub tells: usize,
    pub rules_out: usize,
    pub manifests: usize,
    pub tiebreaks: usize,
    pub error_patterns: usize,
}
