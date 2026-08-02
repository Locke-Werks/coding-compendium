//! Turning what Nyx types into something FTS5 will accept.
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
    let tokens = tokenize(raw);
    if tokens.is_empty() {
        return None;
    }

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

    // Space-separated terms are an implicit AND in FTS5. Every word has to
    // appear somewhere in the card, which is the behavior people expect from a
    // search box.
    Some(parts.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_words_are_quoted_and_last_is_prefixed() {
        assert_eq!(to_match_expression("git branch", true).unwrap(), r#""git" "branch"*"#);
    }

    #[test]
    fn without_prefix_every_token_is_literal() {
        assert_eq!(to_match_expression("git branch", false).unwrap(), r#""git" "branch""#);
    }

    /// The bug this whole module exists to prevent. Unquoted, FTS5 reads `NOT`
    /// as an operator and silently excludes results instead of finding them.
    #[test]
    fn fts5_operators_are_defused() {
        assert_eq!(to_match_expression("this NOT that", false).unwrap(), r#""this" "NOT" "that""#);
        assert_eq!(to_match_expression("a OR b", false).unwrap(), r#""a" "OR" "b""#);
        assert_eq!(to_match_expression("x AND y", false).unwrap(), r#""x" "AND" "y""#);
        // The parenthesis is stripped as punctuation, so this splits on
        // whitespace into "NEAR a" and "b". Both are quoted literals, which is
        // the property that matters: NEAR is no longer an operator.
        assert_eq!(to_match_expression("NEAR(a b)", false).unwrap(), r#""NEAR a" "b""#);
    }

    /// A real query someone would type while panicking. The dashes would be a
    /// parse error raw, and they carry no meaning to the tokenizer regardless.
    #[test]
    fn punctuation_is_stripped_not_escaped() {
        assert_eq!(to_match_expression("git reset --hard", false).unwrap(), r#""git" "reset" "hard""#);
    }

    /// A quote in the middle of a word opens a phrase, so this splits into two
    /// tokens rather than producing one with an embedded quote.
    #[test]
    fn a_mid_word_quote_starts_a_phrase() {
        assert_eq!(to_match_expression(r#"say"hi"#, false).unwrap(), r#""say" "hi""#);
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
            r#""detached head" "recov"*"#
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

    /// `C++` and `C#` are real things Nyx will search for. They must not error,
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
