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

//! Turning what the reader types into something FTS5 will accept.
//!
//! This is the part most likely to be wrong, so it lives in its own module with
//! its own tests and touches no database.
//!
//! # Why this is harder than it looks
//!
//! FTS5 has a real query language. `AND`, `OR`, `NOT`, `NEAR`, parentheses,
//! quotes, and a trailing `*` all mean something. A search box does not: someone
//! searching for `git reset --hard` or `C++` or `"unexpected token }"` means
//! those characters literally.
//!
//! Hand those raw to FTS5 and two things happen, both bad:
//!
//! 1. **Syntax errors.** An unbalanced quote or a bare `-` is a parse failure.
//!    SQLite reports it when the statement *steps*, not when it is prepared, so
//!    it surfaces as a runtime error in the middle of a keystroke rather than at
//!    startup.
//! 2. **Silently wrong results.** `NOT` is a real operator, so searching for the
//!    English word "not" quietly excludes things instead of finding them. That is
//!    worse than an error, because nobody notices.
//!
//! The fix is to quote every token, which makes it a literal. FTS5 string
//! literals are double-quoted, and an embedded double quote is escaped by
//! doubling it, the same convention as SQL.
//!
//! # The one deliberate exception
//!
//! DeadLetter's version of this quotes every token, which is correct for a
//! mailbox search that runs when you press Enter. This index runs on every
//! keystroke, so the *last* token is a word the user is still typing. Quoting it
//! means "gi" matches nothing until the "t" lands, and the result list flickers
//! empty between words.
//!
//! So the last token gets a `*` suffix for prefix matching, and every other token
//! is quoted literally. That single difference is what makes the search feel live
//! rather than laggy.

/// English function words, dropped before searching.
///
/// Readers type questions, not keywords: "how do i know if this is python", not
/// "python identification". Every word in that question except the last is a
/// function word that appears in half the corpus, contributes nothing to
/// relevance, and actively hurts when terms are combined.
///
/// Deliberately short. A long stopword list starts eating meaningful terms, and
/// several near-misses were left in on purpose: "not" matters in "not found",
/// "no" matters in "no such file", and "can" matters in "cannot". BM25 already
/// discounts common words through inverse document frequency, so this list only
/// needs to remove the ones that are pure noise.
const STOPWORDS: &[&str] = &[
    "a", "am", "an", "and", "any", "are", "as", "at", "be", "been", "but", "by", "did", "do",
    "does", "for", "from", "had", "has", "have", "how", "i", "if", "in", "into", "is", "it",
    "its", "me", "my", "of", "on", "or", "should", "so", "some", "than", "that", "the", "their",
    "them", "then", "there", "these", "they", "this", "to", "was", "were", "what", "when",
    "where", "which", "who", "why", "will", "with", "would", "you", "your",
];

fn is_stopword(token: &str) -> bool {
    let lower = token.to_ascii_lowercase();
    STOPWORDS.binary_search(&lower.as_str()).is_ok()
}

/// A token lifted out of the user's raw input.
#[derive(Debug, PartialEq, Eq)]
struct Token<'a> {
    text: &'a str,
    /// True when the user wrote it inside double quotes, meaning they want the
    /// exact phrase and no prefix expansion.
    quoted: bool,
}

/// Split raw input into tokens, honoring double-quoted phrases.
///
/// An unterminated quote is treated as running to the end of input rather than
/// as an error. Someone typing `"unexpected token` has not made a mistake, they
/// are mid-sentence, and refusing to search until they close the quote would be
/// hostile.
fn tokenize(raw: &str) -> Vec<Token<'_>> {
    let mut tokens = Vec::new();
    let bytes = raw.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        // Skip whitespace between tokens.
        if bytes[i].is_ascii_whitespace() {
            i += 1;
            continue;
        }

        if bytes[i] == b'"' {
            let start = i + 1;
            let mut end = start;
            while end < bytes.len() && bytes[end] != b'"' {
                end += 1;
            }
            if start < end {
                tokens.push(Token { text: &raw[start..end], quoted: true });
            }
            // Step past the closing quote if there was one.
            i = if end < bytes.len() { end + 1 } else { end };
        } else {
            let start = i;
            while i < bytes.len() && !bytes[i].is_ascii_whitespace() && bytes[i] != b'"' {
                i += 1;
            }
            tokens.push(Token { text: &raw[start..i], quoted: false });
        }
    }

    tokens
}

/// Strip characters FTS5's tokenizer will not index anyway.
///
/// The default `unicode61` tokenizer splits on anything that is not a letter or
/// a digit, so punctuation inside a quoted literal cannot match and only adds
/// ways for the quoting to go wrong. `git reset --hard` becomes the three tokens
/// `git`, `reset`, `hard`, which is what the index actually contains.
///
/// The doubling of `"` is what makes the result safe to embed in an FTS5 string
/// literal. Everything else here is about matching, not safety.
fn sanitize(token: &str) -> String {
    let mut out = String::with_capacity(token.len());
    for ch in token.chars() {
        if ch.is_alphanumeric() {
            out.push(ch);
        } else if ch == '"' {
            // Doubling escapes it inside an FTS5 literal.
            out.push_str("\"\"");
        } else if !out.is_empty() && !out.ends_with(' ') {
            // Collapse runs of punctuation into a single separator so
            // `foo--bar` becomes the phrase "foo bar" rather than "foo  bar".
            out.push(' ');
        }
    }
    out.trim().to_string()
}

/// The meaningful words in a query, lowercased, with function words removed.
///
/// Used for the title boost in [`crate::search::Index::lexical`]. BM25 alone
/// ranks by term statistics and has no notion that a card *is about* the thing
/// named in its title, which is the difference between "python" returning the
/// Python card and returning whichever card happens to mention it most.
pub fn content_tokens(raw: &str) -> Vec<String> {
    tokenize(raw)
        .iter()
        .filter(|t| t.quoted || !is_stopword(t.text))
        .map(|t| sanitize(t.text).to_lowercase())
        .filter(|t| !t.is_empty())
        .collect()
}

/// Build an FTS5 MATCH expression from raw user input.
///
/// Returns `None` when there is nothing searchable, which is a normal state (an
/// empty box, or someone who has typed only punctuation) and not an error. The
/// caller should show no results rather than run a query.
///
/// `prefix_last` should be true for as-you-type search and false when the user
/// has committed to the query, for example by pressing Enter. A committed query
/// wants exact matching; a live one wants the trailing word to expand.
pub fn to_match_expression(raw: &str, prefix_last: bool) -> Option<String> {
    let all = tokenize(raw);
    if all.is_empty() {
        return None;
    }

    // Drop function words, but never everything. Someone searching for the
    // literal word "the" gets to search for it, and dropping the only token
    // would turn a real query into an empty result list.
    let kept: Vec<&Token<'_>> = all.iter().filter(|t| t.quoted || !is_stopword(t.text)).collect();
    let tokens: Vec<&Token<'_>> = if kept.is_empty() { all.iter().collect() } else { kept };

    let mut parts: Vec<String> = Vec::with_capacity(tokens.len());
    let last = tokens.len() - 1;

    for (i, token) in tokens.iter().enumerate() {
        let clean = sanitize(token.text);
        if clean.is_empty() {
            continue;
        }

        // Prefix-expand only the final token, only when asked, and never when
        // the user quoted it: an explicit quote is an explicit request for the
        // exact phrase.
        let wants_prefix = prefix_last && i == last && !token.quoted;

        if wants_prefix {
            // The `*` sits outside the closing quote. FTS5 reads it as a prefix
            // operator on the preceding string; inside the quotes it would be a
            // literal asterisk, which matches nothing.
            parts.push(format!("\"{clean}\"*"));
        } else {
            parts.push(format!("\"{clean}\""));
        }
    }

    if parts.is_empty() {
        return None;
    }

    // OR, not AND.
    //
    // Space-separated terms are an implicit AND in FTS5, and AND is wrong for
    // this audience. A reader types "how do i know if this is python", and requiring
    // every word to appear in one card returns nothing at all, which reads as
    // the app being broken rather than the query being conversational.
    //
    // OR cannot fail that way, and it is not the loose free-for-all it sounds
    // like: BM25 ranks a card matching three query terms above one matching a
    // single term, and its inverse-document-frequency weighting means a rare
    // word like "python" counts for far more than a common one like "file". The
    // result is that the specific words in a question drive the ranking and the
    // filler words quietly stop mattering.
    //
    // Someone who genuinely wants every word can quote the phrase.
    Some(parts.join(" OR "))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `binary_search` returns nonsense on an unsorted slice, and it fails
    /// silently: the list would just stop recognizing some stopwords.
    #[test]
    fn the_stopword_list_is_sorted() {
        let mut sorted = STOPWORDS.to_vec();
        sorted.sort_unstable();
        assert_eq!(STOPWORDS, sorted.as_slice(), "STOPWORDS must stay alphabetical");
    }

    #[test]
    fn plain_words_are_quoted_and_last_is_prefixed() {
        assert_eq!(to_match_expression("git branch", true).unwrap(), r#""git" OR "branch"*"#);
    }

    #[test]
    fn without_prefix_every_token_is_literal() {
        assert_eq!(to_match_expression("git branch", false).unwrap(), r#""git" OR "branch""#);
    }

    /// The failure that made this necessary. Under AND semantics this returned
    /// nothing at all, because no card contains every one of those words. That
    /// reads as the app being broken rather than the question being casual, and
    /// casual questions are the entire audience.
    #[test]
    fn a_natural_language_question_keeps_only_its_content_words() {
        assert_eq!(
            to_match_expression("how do i know if this is python", false).unwrap(),
            r#""know" OR "python""#
        );
    }

    #[test]
    fn stopwords_are_dropped() {
        assert_eq!(to_match_expression("what is a branch", false).unwrap(), r#""branch""#);
        assert_eq!(to_match_expression("how do I undo the last commit", false).unwrap(),
                   r#""undo" OR "last" OR "commit""#);
    }

    /// Dropping every token would turn a real query into an empty result list,
    /// so a query made entirely of function words searches for them literally.
    #[test]
    fn a_query_of_only_stopwords_is_not_erased() {
        assert_eq!(to_match_expression("what is it", false).unwrap(), r#""what" OR "is" OR "it""#);
    }

    /// Words that look like stopwords and are not. "not found", "no such file",
    /// and "cannot" are all real search terms in this corpus.
    #[test]
    fn negations_survive_because_error_messages_need_them() {
        let out = to_match_expression("command not found", false).unwrap();
        assert!(out.contains(r#""not""#), "'not' matters in 'not found': {out}");
        assert!(out.contains(r#""found""#));
    }

    /// The bug this whole module exists to prevent. Unquoted, FTS5 reads these
    /// as operators and either errors or silently changes what the query means.
    #[test]
    fn fts5_operators_are_defused() {
        // Every one of these is a quoted literal in the output, so FTS5 reads
        // them as words to find rather than as instructions.
        assert_eq!(to_match_expression("branch NOT merge", false).unwrap(),
                   r#""branch" OR "NOT" OR "merge""#);
        assert_eq!(to_match_expression("NEAR(a b)", false).unwrap(), r#""NEAR a" OR "b""#);

        // "and" is also a stopword, so it is dropped before it can ever reach
        // FTS5. Defused by removal rather than by quoting, which is equally
        // safe and slightly faster.
        assert_eq!(to_match_expression("x AND y", false).unwrap(), r#""x" OR "y""#);
    }

    /// A real query someone would type while panicking. The dashes would be a
    /// parse error raw, and they carry no meaning to the tokenizer regardless.
    #[test]
    fn punctuation_is_stripped_not_escaped() {
        assert_eq!(to_match_expression("git reset --hard", false).unwrap(),
                   r#""git" OR "reset" OR "hard""#);
    }

    /// A quote in the middle of a word opens a phrase, so this splits into two
    /// tokens rather than producing one with an embedded quote.
    #[test]
    fn a_mid_word_quote_starts_a_phrase() {
        assert_eq!(to_match_expression(r#"say"hi"#, false).unwrap(), r#""say" OR "hi""#);
    }

    /// The doubling in `sanitize` is currently unreachable through
    /// `to_match_expression`, because the tokenizer treats every `"` as a
    /// delimiter and never passes one through. It is kept as a second line of
    /// defense and tested directly, so that a future change to tokenization
    /// cannot silently produce an unescaped quote inside an FTS5 literal.
    #[test]
    fn sanitize_doubles_embedded_quotes() {
        assert_eq!(sanitize(r#"say"hi"#), r#"say""hi"#);
    }

    #[test]
    fn quoted_phrases_stay_together_and_never_prefix() {
        assert_eq!(
            to_match_expression(r#""merge conflict""#, true).unwrap(),
            r#""merge conflict""#,
            "an explicit quote is an explicit request for the exact phrase"
        );
    }

    #[test]
    fn phrase_plus_trailing_word() {
        assert_eq!(
            to_match_expression(r#""detached head" recov"#, true).unwrap(),
            r#""detached head" OR "recov"*"#
        );
    }

    /// Someone mid-sentence has not made a mistake. Refusing to search until the
    /// quote is closed would make the box feel broken.
    #[test]
    fn unterminated_quote_is_tolerated() {
        assert_eq!(
            to_match_expression(r#""unexpected token"#, false).unwrap(),
            r#""unexpected token""#
        );
    }

    #[test]
    fn nothing_searchable_returns_none() {
        assert_eq!(to_match_expression("", true), None);
        assert_eq!(to_match_expression("   ", true), None);
        assert_eq!(to_match_expression("---", true), None, "punctuation alone is not a search");
        assert_eq!(to_match_expression(r#""""#, true), None, "an empty phrase is not a search");
    }

    /// `C++` and `C#` are real things a reader will search for. They must not error,
    /// and they must reduce to something that finds the card.
    #[test]
    fn language_names_with_symbols_survive() {
        assert_eq!(to_match_expression("C++", false).unwrap(), r#""C""#);
        assert_eq!(to_match_expression("C#", false).unwrap(), r#""C""#);
        assert_eq!(to_match_expression(".gitignore", false).unwrap(), r#""gitignore""#);
    }

    #[test]
    fn unicode_is_indexable_not_stripped() {
        // is_alphanumeric is Unicode-aware, so accented text survives. FTS5's
        // unicode61 tokenizer indexes it, so stripping it would lose real words.
        assert_eq!(to_match_expression("café", false).unwrap(), r#""café""#);
    }

    #[test]
    fn runs_of_punctuation_collapse_to_one_separator() {
        assert_eq!(to_match_expression("foo--bar", false).unwrap(), r#""foo bar""#);
        assert_eq!(to_match_expression("a...b", false).unwrap(), r#""a b""#);
    }

    /// A single trailing character is the most common state of a live search
    /// box, and the one where getting prefix matching wrong is most visible.
    #[test]
    fn single_character_prefixes() {
        assert_eq!(to_match_expression("g", true).unwrap(), r#""g"*"#);
    }
}
