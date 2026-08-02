//! Splitting a card into pieces small enough to embed usefully.
//!
//! # Why chunk at all
//!
//! An embedding is one vector per input, however long that input is. Embed a
//! 900-word section and you get a single vector meaning roughly "this is about
//! git", which sits near every git question and distinguishes none of them. The
//! card matches everything and answers nothing.
//!
//! Splitting it into pieces gives one vector per idea, so a question about
//! detached HEAD matches the paragraph about detached HEAD rather than the whole
//! git chapter.
//!
//! # Why they overlap
//!
//! A hard split lands in the middle of an explanation about as often as not. The
//! sentence that sets up the answer ends up in chunk 3 and the answer itself
//! starts chunk 4, so neither chunk contains a complete thought and neither
//! matches well. Overlapping by a few sentences means every idea appears whole in
//! at least one chunk.
//!
//! # Why the heading path is prepended
//!
//! This is the cheapest retrieval win available and it is easy to skip. A chunk
//! taken from the middle of a document is just prose: nothing in the words
//! "run it again with the verbosity flag" says it is about reading errors. The
//! model cannot recover context it was never given. Prefixing each chunk with the
//! headings it sits under puts that context back in for the cost of a few tokens.

use super::frontmatter::Card;

/// Target chunk size, in tokens.
///
/// Follows the precedent set in Reliquary. Small enough that a chunk is about one
/// idea, large enough that it is a complete one.
pub const CHUNK_TARGET_TOKENS: usize = 400;

/// How much each chunk repeats from the end of the previous one, in tokens.
pub const CHUNK_OVERLAP_TOKENS: usize = 60;

/// Rough tokens-per-word for English prose.
///
/// Real tokenization happens inside the embedding model. This estimate only
/// decides where to cut, and being off by ten percent moves a boundary rather
/// than breaking anything, so it is not worth pulling in a tokenizer to make the
/// split exact.
const TOKENS_PER_WORD: f32 = 1.3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk {
    /// The heading trail, e.g. "How to read an error message > Full > Exit codes".
    pub heading_path: String,
    /// The text, with the heading path already prepended. This is what gets
    /// embedded and what is stored, so the two can never disagree.
    pub text: String,
}

/// A block of body text and the headings above it.
struct Block {
    heading_path: String,
    text: String,
}

/// Walk the markdown, tracking the current heading trail.
///
/// Fenced code blocks are passed through without being interpreted, so a `#`
/// comment inside a shell example is not mistaken for a heading. That happens
/// constantly in this corpus.
fn split_into_blocks(title: &str, body: &str) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();
    // headings[0] is the card title, so every chunk carries it even before the
    // first `##`.
    let mut headings: Vec<String> = vec![title.to_string()];
    let mut current = String::new();
    let mut in_fence = false;

    let flush = |headings: &[String], current: &mut String, blocks: &mut Vec<Block>| {
        if !current.trim().is_empty() {
            blocks.push(Block {
                // Skip empty levels. A document that jumps from ## straight to
                // #### leaves a gap, and "Demo >  > Deep" helps nobody.
                heading_path: headings
                    .iter()
                    .filter(|h| !h.is_empty())
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(" > "),
                text: current.trim().to_string(),
            });
        }
        current.clear();
    };

    for line in body.lines() {
        let trimmed = line.trim_start();

        if trimmed.starts_with("```") {
            in_fence = !in_fence;
            current.push_str(line);
            current.push('\n');
            continue;
        }

        // A `#` inside a fence is a comment, not a heading.
        if !in_fence && trimmed.starts_with('#') {
            let level = trimmed.chars().take_while(|c| *c == '#').count();
            let text = trimmed[level..].trim();

            flush(&headings, &mut current, &mut blocks);

            // headings[0] is always the card title, so a `##` belongs at index
            // 1, a `###` at index 2, and so on. Keep everything shallower than
            // this heading and drop everything at or below its level.
            //
            // A bare `#` in the body is treated as `##`: the card already has a
            // title and a second top-level heading would replace it, losing the
            // one piece of context every chunk needs.
            let depth = level.max(2) - 1;
            headings.truncate(depth);
            // Pad if the document skipped a level, so index and depth stay
            // aligned. The empty entries are filtered out when joining.
            while headings.len() < depth {
                headings.push(String::new());
            }
            headings.push(text.to_string());
            continue;
        }

        current.push_str(line);
        current.push('\n');
    }

    flush(&headings, &mut current, &mut blocks);
    blocks
}

/// Split one block's text into overlapping windows.
fn window(text: &str, target_words: usize, overlap_words: usize) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.len() <= target_words {
        return vec![text.to_string()];
    }

    let mut out = Vec::new();
    let stride = target_words.saturating_sub(overlap_words).max(1);
    let mut start = 0;

    while start < words.len() {
        let end = (start + target_words).min(words.len());
        out.push(words[start..end].join(" "));
        if end == words.len() {
            break;
        }
        start += stride;
    }
    out
}

/// Split a card into embeddable chunks.
pub fn chunk_card(card: &Card) -> Vec<Chunk> {
    let target_words = (CHUNK_TARGET_TOKENS as f32 / TOKENS_PER_WORD) as usize;
    let overlap_words = (CHUNK_OVERLAP_TOKENS as f32 / TOKENS_PER_WORD) as usize;

    let mut chunks = Vec::new();

    // The one-sentence answer becomes its own chunk. It is the most precisely
    // written text on the card and the thing the palette displays, so it earns a
    // vector of its own rather than being diluted inside a larger window.
    if let Some(answer) = &card.answer {
        if !answer.trim().is_empty() {
            chunks.push(Chunk {
                heading_path: card.title.clone(),
                text: format!("{}\n\n{}", card.title, answer.trim()),
            });
        }
    }

    for block in split_into_blocks(&card.title, &card.body) {
        for piece in window(&block.text, target_words, overlap_words) {
            chunks.push(Chunk {
                text: format!("{}\n\n{}", block.heading_path, piece),
                heading_path: block.heading_path.clone(),
            });
        }
    }

    chunks
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compile::frontmatter::CardKind;

    fn card(body: &str, answer: Option<&str>) -> Card {
        Card {
            id: "demo".into(),
            title: "Demo card".into(),
            kind: CardKind::Section,
            track: None,
            order: None,
            answer: answer.map(str::to_string),
            body: body.into(),
            keywords: vec![],
            volatility: "low".into(),
            verified: "2026-08-02".into(),
            meta_json: "{}".into(),
        }
    }

    #[test]
    fn short_card_is_one_chunk_per_block() {
        let c = card("Just a little text.", None);
        let chunks = chunk_card(&c);
        assert_eq!(chunks.len(), 1);
        assert!(chunks[0].text.contains("Just a little text."));
    }

    #[test]
    fn every_chunk_carries_its_heading_path() {
        let c = card("## More\n\nFirst part.\n\n## Full\n\nSecond part.\n", None);
        let chunks = chunk_card(&c);
        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0].heading_path, "Demo card > More");
        assert_eq!(chunks[1].heading_path, "Demo card > Full");
        // The path is inside the embedded text too, not only in the column.
        assert!(chunks[0].text.starts_with("Demo card > More"));
    }

    #[test]
    fn nested_headings_build_a_trail() {
        let c = card("## Full\n\nIntro.\n\n### Exit codes\n\nDetail.\n", None);
        let chunks = chunk_card(&c);
        assert_eq!(chunks[1].heading_path, "Demo card > Full > Exit codes");
    }

    #[test]
    fn a_deeper_heading_replaces_only_its_own_level() {
        let c = card("## Full\n\nA.\n\n### One\n\nB.\n\n### Two\n\nC.\n", None);
        let chunks = chunk_card(&c);
        let paths: Vec<&str> = chunks.iter().map(|c| c.heading_path.as_str()).collect();
        assert_eq!(
            paths,
            vec!["Demo card > Full", "Demo card > Full > One", "Demo card > Full > Two"],
            "a sibling heading must replace the previous one, not nest under it"
        );
    }

    /// This corpus is full of shell examples whose comments start with `#`.
    /// Treating one as a heading would shatter the chunk and produce a nonsense
    /// heading path.
    #[test]
    fn a_hash_inside_a_code_fence_is_not_a_heading() {
        let c = card("## More\n\n```powershell\n# This is a comment, not a heading\ngit status\n```\n\nAfter.\n", None);
        let chunks = chunk_card(&c);
        assert_eq!(chunks.len(), 1, "the fence should not have split the block");
        assert_eq!(chunks[0].heading_path, "Demo card > More");
        assert!(chunks[0].text.contains("git status"));
    }

    #[test]
    fn the_answer_becomes_its_own_chunk() {
        let c = card("## More\n\nBody.\n", Some("The one-sentence answer."));
        let chunks = chunk_card(&c);
        assert_eq!(chunks.len(), 2);
        assert!(chunks[0].text.contains("The one-sentence answer."));
        assert_eq!(chunks[0].heading_path, "Demo card");
    }

    #[test]
    fn long_text_splits_with_overlap() {
        let long = (0..900).map(|i| format!("word{i}")).collect::<Vec<_>>().join(" ");
        let c = card(&format!("## More\n\n{long}\n"), None);
        let chunks = chunk_card(&c);
        assert!(chunks.len() > 1, "900 words should exceed one chunk");

        // The tail of chunk 0 must reappear at the head of chunk 1, or an idea
        // split across the boundary is lost from both.
        let first_words: Vec<&str> = chunks[0].text.split_whitespace().collect();
        let second_words: Vec<&str> = chunks[1].text.split_whitespace().collect();
        let tail = first_words[first_words.len() - 5..].to_vec();
        assert!(
            second_words.windows(5).any(|w| w == tail.as_slice()),
            "chunks must overlap so an idea spanning the boundary survives"
        );
    }

    #[test]
    fn windowing_terminates_on_pathological_settings() {
        // overlap >= target would make stride zero and loop forever without the
        // saturating_sub().max(1) guard.
        let words = (0..50).map(|i| format!("w{i}")).collect::<Vec<_>>().join(" ");
        let out = window(&words, 10, 20);
        assert!(!out.is_empty());
        assert!(out.len() < 200, "windowing must make progress");
    }
}
