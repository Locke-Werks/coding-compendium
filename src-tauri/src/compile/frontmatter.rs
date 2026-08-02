//! Parsing a card file into its frontmatter and its body.
//!
//! A card is YAML between two `---` fences, then markdown:
//!
//! ```text
//! ---
//! id: f1-how-to-read-an-error-message
//! type: section
//! ---
//!
//! ## More
//! ...
//! ```
//!
//! The content linter has already validated this shape against a JSON Schema
//! before anything reaches here, so this parser can be strict and fail loudly
//! rather than guessing at malformed input.

use anyhow::{bail, Context, Result};
use serde::Deserialize;
use std::path::Path;

/// Which kind of card this is. Determines how the app renders it and which
/// schema the linter validated it against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CardKind {
    Section,
    Language,
    Error,
    Command,
    Intent,
    Glossary,
    Panic,
}

impl CardKind {
    pub fn as_str(self) -> &'static str {
        match self {
            CardKind::Section => "section",
            CardKind::Language => "language",
            CardKind::Error => "error",
            CardKind::Command => "command",
            CardKind::Intent => "intent",
            CardKind::Glossary => "glossary",
            CardKind::Panic => "panic",
        }
    }
}

/// The frontmatter fields every card type shares.
///
/// Type-specific fields are left in the raw YAML and carried through as JSON in
/// [`Card::meta_json`]. Modelling all seven card types as one Rust struct would
/// mean thirty optional fields, most of them meaningless for any given card.
#[derive(Debug, Deserialize)]
struct CommonFields {
    id: String,
    title: String,
    #[serde(rename = "type")]
    kind: CardKind,
    volatility: String,
    verified: serde_yaml_ng::Value,
    #[serde(default)]
    track: Option<String>,
    #[serde(default)]
    order: Option<i64>,
    #[serde(default)]
    answer: Option<String>,
    #[serde(default)]
    keywords: Vec<String>,
}

#[derive(Debug)]
pub struct Card {
    pub id: String,
    pub title: String,
    pub kind: CardKind,
    pub track: Option<String>,
    pub order: Option<i64>,
    pub answer: Option<String>,
    pub body: String,
    pub keywords: Vec<String>,
    pub volatility: String,
    pub verified: String,
    /// Everything from the frontmatter, as JSON, including the common fields.
    /// The app reads type-specific data out of here.
    pub meta_json: String,
}

/// Split a card file into its raw YAML frontmatter and its markdown body.
///
/// Returns an error rather than treating a missing fence as an empty header: a
/// card with no frontmatter has no id, and silently accepting it would produce a
/// row the app can never link to.
pub fn split_frontmatter(raw: &str) -> Result<(&str, &str)> {
    // Tolerate a UTF-8 byte order mark. Windows editors add one and it is
    // invisible, so the resulting "expected --- on line 1" is baffling.
    let raw = raw.trim_start_matches('\u{feff}');

    let rest = raw
        .strip_prefix("---\n")
        .or_else(|| raw.strip_prefix("---\r\n"))
        .context("card does not start with a --- frontmatter fence")?;

    // Find the closing fence at the start of a line.
    let end = rest
        .match_indices("\n---")
        .find(|(i, _)| {
            let after = &rest[i + 4..];
            after.is_empty() || after.starts_with('\n') || after.starts_with('\r')
        })
        .map(|(i, _)| i)
        .context("frontmatter is never closed by a --- line")?;

    let yaml = &rest[..end];

    // Step past the closing fence, then trim whatever blank space follows it.
    // A chain of single trim_start_matches calls cannot handle CRLF, where the
    // separator is "\r\n\r\n" and each call strips only the one character it
    // was given.
    let body = rest[end..]
        .trim_start_matches(['\n', '\r'])
        .trim_start_matches("---")
        .trim_start();

    Ok((yaml, body))
}

/// The first real paragraph of a body, for use as a fallback `answer`.
///
/// Skips headings, code fences, blockquotes, lists, and tables, so the result is
/// a sentence rather than a fragment of a table row. Returns `None` when the
/// card opens with something that is not prose, in which case the palette shows
/// the title alone, which is honest.
fn lead_paragraph(body: &str) -> Option<String> {
    let mut para = String::new();
    let mut in_fence = false;

    for line in body.lines() {
        let t = line.trim();

        if t.starts_with("```") {
            in_fence = !in_fence;
            if !para.is_empty() {
                break;
            }
            continue;
        }
        if in_fence {
            continue;
        }

        if t.is_empty() {
            if !para.is_empty() {
                break; // paragraph ended
            }
            continue;
        }
        // Anything that is not running prose.
        if t.starts_with('#') || t.starts_with('>') || t.starts_with('|') || t.starts_with("- ")
            || t.starts_with("* ") || t.starts_with("---")
        {
            if !para.is_empty() {
                break;
            }
            continue;
        }

        if !para.is_empty() {
            para.push(' ');
        }
        para.push_str(t);
    }

    if para.is_empty() {
        return None;
    }

    // Strip the markdown emphasis and code markers that would otherwise show as
    // literal asterisks and backticks in a plain-text result row.
    let cleaned: String = para.chars().filter(|c| !matches!(c, '*' | '`' | '_')).collect();
    let cleaned = cleaned.trim();

    // Keep it to roughly the length of an authored answer. Cutting at a sentence
    // boundary avoids ending mid-clause.
    const MAX: usize = 240;
    if cleaned.len() <= MAX {
        return Some(cleaned.to_string());
    }
    let cut = cleaned[..MAX]
        .rfind(". ")
        .map(|i| i + 1)
        .unwrap_or_else(|| cleaned[..MAX].rfind(' ').unwrap_or(MAX));
    Some(cleaned[..cut].trim().to_string())
}

impl Card {
    pub fn parse(raw: &str, path: &Path) -> Result<Self> {
        let (yaml, body) = split_frontmatter(raw)?;

        let common: CommonFields =
            serde_yaml_ng::from_str(yaml).context("frontmatter is not valid YAML")?;

        // YAML turns an unquoted 2026-08-02 into a date, not a string. Accept
        // both rather than making every author remember to quote it, which is
        // the kind of rule that is followed exactly once.
        let verified = match &common.verified {
            serde_yaml_ng::Value::String(s) => s.clone(),
            other => serde_yaml_ng::to_string(other)
                .context("reading `verified`")?
                .trim()
                .trim_matches('"')
                .to_string(),
        };

        // The filename is the id. Keeping them in lockstep means a card found in
        // search can be found on disk, and it makes a duplicate id impossible to
        // create by copying a file.
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .context("card has no filename")?;
        if stem != common.id {
            bail!("frontmatter id `{}` does not match filename `{stem}.md`", common.id);
        }

        // Round-trip the whole header through JSON so the app can read
        // type-specific fields without this struct having to know about them.
        let value: serde_yaml_ng::Value =
            serde_yaml_ng::from_str(yaml).context("re-reading frontmatter as a value")?;
        let meta_json = serde_json::to_string(&value).context("converting frontmatter to JSON")?;

        // Only prose sections declare an `answer` in frontmatter. Every other
        // card type would show as a bare title in the palette, which throws away
        // the whole answer-first premise, so one is derived from the opening
        // paragraph. Authors already write that paragraph as a one-line
        // definition ("A compiled systems language that refuses to build your
        // program until it can prove the memory handling is safe"), so the
        // derived answer is the sentence they would have written anyway.
        let answer = common.answer.or_else(|| lead_paragraph(body));

        Ok(Card {
            id: common.id,
            title: common.title,
            kind: common.kind,
            track: common.track,
            order: common.order,
            answer,
            body: body.to_string(),
            keywords: common.keywords,
            volatility: common.volatility,
            verified,
            meta_json,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    const MINIMAL: &str = "---\nid: demo\ntitle: Demo\ntype: section\nvolatility: low\nverified: 2026-08-02\n---\n\n## More\n\nBody text.\n";

    fn path(id: &str) -> PathBuf {
        PathBuf::from(format!("content/tracks/A/{id}.md"))
    }

    #[test]
    fn splits_header_from_body() {
        let (yaml, body) = split_frontmatter(MINIMAL).unwrap();
        assert!(yaml.contains("id: demo"));
        assert!(body.starts_with("## More"), "got: {body:?}");
        assert!(!body.contains("---"));
    }

    #[test]
    fn tolerates_a_byte_order_mark() {
        let with_bom = format!("\u{feff}{MINIMAL}");
        let (yaml, _) = split_frontmatter(&with_bom).unwrap();
        assert!(yaml.contains("id: demo"), "a BOM is invisible and must not break parsing");
    }

    #[test]
    fn tolerates_windows_line_endings() {
        let crlf = MINIMAL.replace('\n', "\r\n");
        let (yaml, body) = split_frontmatter(&crlf).unwrap();
        assert!(yaml.contains("id: demo"));
        assert!(body.starts_with("## More"));
    }

    /// A `---` inside the body is a horizontal rule in markdown and must not be
    /// mistaken for the closing fence.
    #[test]
    fn a_horizontal_rule_in_the_body_is_not_the_closing_fence() {
        let doc = "---\nid: demo\ntitle: Demo\ntype: section\nvolatility: low\nverified: 2026-08-02\n---\n\nIntro.\n\n---\n\nAfter the rule.\n";
        let (yaml, body) = split_frontmatter(doc).unwrap();
        assert!(yaml.contains("id: demo"));
        assert!(body.contains("After the rule"), "body was truncated at a horizontal rule");
    }

    #[test]
    fn missing_frontmatter_is_an_error() {
        assert!(split_frontmatter("# Just markdown\n").is_err());
    }

    #[test]
    fn unclosed_frontmatter_is_an_error() {
        assert!(split_frontmatter("---\nid: demo\n").is_err());
    }

    #[test]
    fn parses_a_whole_card() {
        let card = Card::parse(MINIMAL, &path("demo")).unwrap();
        assert_eq!(card.id, "demo");
        assert_eq!(card.kind, CardKind::Section);
        assert_eq!(card.verified, "2026-08-02", "an unquoted YAML date must survive as a string");
    }

    #[test]
    fn id_must_match_filename() {
        let err = Card::parse(MINIMAL, &path("something-else")).unwrap_err();
        assert!(err.to_string().contains("does not match filename"));
    }

    #[test]
    fn an_explicit_answer_wins_over_the_derived_one() {
        let doc = "---\nid: demo\ntitle: Demo\ntype: section\nvolatility: low\nverified: 2026-08-02\nanswer: The authored answer.\n---\n\nThe opening paragraph.\n";
        let card = Card::parse(doc, &path("demo")).unwrap();
        assert_eq!(card.answer.as_deref(), Some("The authored answer."));
    }

    /// Language, error, and command cards carry no `answer` field, and without a
    /// derived one they render in the palette as a bare title. That throws away
    /// the answer-first premise the whole app is built on.
    #[test]
    fn the_opening_paragraph_becomes_the_answer() {
        let doc = "---\nid: demo\ntitle: Demo\ntype: language\nvolatility: low\nverified: 2026-08-02\n---\n\nA compiled systems language that refuses to build your program.\n\n## The shape\n\nMore text.\n";
        let card = Card::parse(doc, &path("demo")).unwrap();
        assert_eq!(
            card.answer.as_deref(),
            Some("A compiled systems language that refuses to build your program.")
        );
    }

    #[test]
    fn the_derived_answer_skips_non_prose() {
        // Opening with a heading and a code fence must not produce "## Heading"
        // or a line of Rust as the answer.
        let body = "## Heading\n\n```rust\nfn main() {}\n```\n\nThe actual sentence.\n";
        assert_eq!(lead_paragraph(body).as_deref(), Some("The actual sentence."));

        assert_eq!(lead_paragraph("| a | b |\n| - | - |\n").as_deref(), None);
        assert_eq!(lead_paragraph("").as_deref(), None);
    }

    #[test]
    fn the_derived_answer_drops_markdown_markers() {
        assert_eq!(
            lead_paragraph("Its packages are called **crates**, which uses `cargo`.").as_deref(),
            Some("Its packages are called crates, which uses cargo.")
        );
    }

    #[test]
    fn a_long_opening_paragraph_is_cut_at_a_sentence() {
        let body = format!("{} And a trailing clause that runs past the limit.", "Sentence one is quite long. ".repeat(12));
        let out = lead_paragraph(&body).unwrap();
        assert!(out.len() <= 240);
        assert!(out.ends_with('.'), "should cut at a sentence boundary, got: {out:?}");
    }

    /// Type-specific fields are not modelled in Rust and must survive into the
    /// JSON blob, because that is how the language cards carry their tells.
    #[test]
    fn unknown_fields_survive_into_meta() {
        let doc = "---\nid: demo\ntitle: Demo\ntype: language\nvolatility: low\nverified: 2026-08-02\nfamily: compiled\ntells:\n  - pattern: fn\n    weight: 9\n---\n\nBody.\n";
        let card = Card::parse(doc, &path("demo")).unwrap();
        assert!(card.meta_json.contains("\"family\":\"compiled\""));
        assert!(card.meta_json.contains("\"pattern\":\"fn\""));
    }
}
