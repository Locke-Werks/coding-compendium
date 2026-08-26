//! Deciding what KIND of thing was pasted, before asking which language it is.
//!
//! This runs first, and it matters more than it sounds. The reader pastes whatever is
//! on their clipboard: a stack trace, a terminal command they were told to run, a
//! config file, a diff, an error line. Scoring all of those against language
//! tells produces confident nonsense, because a Python traceback is full of
//! Python-shaped tokens while not being Python source at all. Telling them "this
//! is Python" when they pasted a crash is answering a question they did not ask.
//!
//! So the router classifies the shape first and the language second, and some
//! shapes short-circuit language detection entirely. A `git status` command is
//! not a language; it is a command, and the useful answer is what it does.

use serde::Serialize;

/// What kind of text this is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Format {
    /// Source code. The only case that goes on to full language scoring.
    Source,
    /// A crash or exception report. Carries language evidence, but the useful
    /// answer is what the error means, not what language it is.
    StackTrace,
    /// A single error line, no stack.
    ErrorMessage,
    /// A command they are being told to run.
    ShellCommand,
    /// Unified diff or patch output.
    Diff,
    /// Structured data rather than code: JSON, YAML, TOML, INI.
    Config,
    /// Log output.
    Log,
    /// A file listing or directory tree.
    FileListing,
    /// Prose. Usually means they pasted the wrong thing.
    Prose,
}

#[derive(Debug, Clone, Serialize)]
pub struct FormatVerdict {
    pub format: Format,
    /// Why, in one line, shown to the user.
    pub because: String,
}

/// Classify pasted text by shape.
///
/// Checks run most-specific first. A diff containing Python is a diff; a
/// traceback mentioning a shell command is still a traceback.
pub fn detect(text: &str) -> FormatVerdict {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return FormatVerdict { format: Format::Prose, because: "nothing to look at".into() };
    }

    let lines: Vec<&str> = trimmed.lines().collect();
    let first = lines.first().copied().unwrap_or("");

    // --- Diff. Checked first: a diff of Python is a diff, not Python. ---
    if first.starts_with("diff --git")
        || (first.starts_with("--- ") && lines.get(1).is_some_and(|l| l.starts_with("+++ ")))
        || lines.iter().any(|l| l.starts_with("@@ ") && l.contains(" @@"))
    {
        return FormatVerdict {
            format: Format::Diff,
            because: "the @@ markers and +/- line prefixes are unified diff format".into(),
        };
    }

    // --- Stack traces. Each language announces itself distinctively. ---
    if let Some(why) = stack_trace_reason(trimmed, &lines) {
        return FormatVerdict { format: Format::StackTrace, because: why };
    }

    // --- Structured data. Checked before source: JSON is not code. ---
    if let Some(why) = config_reason(trimmed, &lines) {
        return FormatVerdict { format: Format::Config, because: why };
    }

    // --- A single error line with no stack. ---
    //
    // This has to run BEFORE the command check, because PowerShell prints errors
    // starting with the command that failed: "git : The term 'git' is not
    // recognized". That opens exactly like a command and is the opposite of one.
    if lines.len() <= 4 {
        const MARKERS: &[&str] = &[
            "error:", "error ", "fatal:", "warning:", "exception", "failed", "cannot find",
            "not recognized", "is not recognized", "permission denied", "no such file",
            "command not found", "npm err!", "panic:", "denied", "cannot access",
        ];
        let lower = trimmed.to_lowercase();
        if let Some(m) = MARKERS.iter().find(|m| lower.contains(**m)) {
            return FormatVerdict {
                format: Format::ErrorMessage,
                because: format!("contains \"{m}\" and has no stack below it"),
            };
        }
    }

    // --- A single command. ---
    if lines.len() <= 3 && !trimmed.contains('{') && !trimmed.contains(';') {
        const TOOLS: &[&str] = &[
            "git", "gh", "npm", "pnpm", "yarn", "npx", "cargo", "rustup", "python", "py", "pip",
            "node", "deno", "docker", "winget", "choco", "code", "claude", "codex", "dotnet",
            "java", "go", "ssh", "curl", "ls", "cd", "mkdir", "cat", "grep", "rm", "cp", "mv",
        ];
        const CMDLETS: &[&str] = &[
            "Get-", "Set-", "New-", "Remove-", "Start-", "Stop-", "Test-", "Invoke-", "Select-",
            "Write-", "Add-", "Copy-", "Move-",
        ];

        let cmd = strip_prompt(first);
        let word = cmd.split_whitespace().next().unwrap_or("");
        let bare = word.rsplit(['\\', '/']).next().unwrap_or(word);

        if TOOLS.contains(&bare) || TOOLS.contains(&bare.trim_end_matches(".exe")) {
            return FormatVerdict {
                format: Format::ShellCommand,
                because: format!("starts with `{bare}`, a command-line tool"),
            };
        }
        if CMDLETS.iter().any(|p| word.starts_with(p)) {
            return FormatVerdict {
                format: Format::ShellCommand,
                because: "Verb-Noun shape is a PowerShell cmdlet".into(),
            };
        }
    }

    // --- Logs: many lines carrying timestamps or levels. ---
    if lines.len() >= 4 {
        let logish = lines
            .iter()
            .filter(|l| {
                let u = l.to_uppercase();
                u.contains("[INFO]") || u.contains("[WARN]") || u.contains("[ERROR]")
                    || u.contains("[DEBUG]")
                    || l.starts_with("20") && l.len() > 19 && l.as_bytes().get(4) == Some(&b'-')
            })
            .count();
        if logish * 2 >= lines.len() {
            return FormatVerdict {
                format: Format::Log,
                because: "most lines carry a timestamp or a log level".into(),
            };
        }
    }

    // --- A directory listing or tree. ---
    if lines.len() >= 3 {
        let treeish = lines.iter().filter(|l| {
            l.contains('\u{251c}') || l.contains('\u{2514}') || l.contains('\u{2502}')
        }).count();
        if treeish >= 2 {
            return FormatVerdict {
                format: Format::FileListing,
                because: "box-drawing characters make this a directory tree".into(),
            };
        }
    }

    // --- Prose: no code punctuation and it reads like sentences. ---
    let code_punct = trimmed
        .chars()
        .filter(|c| matches!(c, '{' | '}' | ';' | '=' | '(' | ')' | '<' | '>' | '#'))
        .count();
    let words = trimmed.split_whitespace().count();
    if words > 12 && code_punct * 40 < words {
        return FormatVerdict {
            format: Format::Prose,
            because: "reads as sentences, with almost no code punctuation".into(),
        };
    }

    FormatVerdict { format: Format::Source, because: "looks like source code".into() }
}

/// Remove a copied shell prompt from the front of a line.
///
/// People paste straight out of a tutorial, prompt and all: `PS C:\Users\you>
/// git status` or `$ npm install`. Without stripping it the first word is the
/// prompt rather than the command, and nothing is ever recognized.
///
/// Only strips when what precedes the marker actually looks like a prompt. A
/// line containing `>` in the middle, such as a redirect or a comparison, is
/// left alone.
fn strip_prompt(line: &str) -> &str {
    let l = line.trim();

    if let Some(i) = l.find("> ") {
        let head = &l[..i];
        let looks_like_a_prompt =
            head.is_empty() || head.starts_with("PS ") || head.contains(":\\") || head.contains(":/");
        if looks_like_a_prompt {
            return l[i + 2..].trim();
        }
    }

    for marker in ["$ ", "# ", "> ", "PS> "] {
        if let Some(rest) = l.strip_prefix(marker) {
            return rest.trim();
        }
    }
    l
}

fn stack_trace_reason(text: &str, lines: &[&str]) -> Option<String> {
    if text.contains("Traceback (most recent call last)") {
        return Some("the words \"Traceback (most recent call last)\" are Python's".into());
    }
    // Node and browser JavaScript: several lines of "    at something".
    if lines.iter().filter(|l| l.trim_start().starts_with("at ")).count() >= 2 {
        return Some("repeated \"    at ...\" lines are a JavaScript stack".into());
    }
    // Java and Kotlin.
    if lines.iter().filter(|l| l.trim_start().starts_with("at ") || l.contains(".java:")).count() >= 2
        && text.contains("Exception")
    {
        return Some("\"at package.Class.method(File.java:12)\" is a Java stack".into());
    }
    if text.contains("panic:") && text.contains("goroutine ") {
        return Some("\"goroutine\" appears only in a Go panic".into());
    }
    if text.contains("thread 'main' panicked at") {
        return Some("\"thread 'main' panicked at\" is Rust's panic format".into());
    }
    if text.contains("Unhandled exception") && text.contains("   at ") {
        return Some("\"Unhandled exception\" with indented at-lines is .NET".into());
    }
    if lines.iter().any(|l| l.contains("Stack trace:")) || text.contains("PHP Fatal error") {
        return Some("\"PHP Fatal error\" and \"Stack trace:\" are PHP's".into());
    }
    None
}

fn config_reason(text: &str, lines: &[&str]) -> Option<String> {
    let t = text.trim();

    // JSON: whole thing is one object or array, with quoted keys.
    let bracketed = (t.starts_with('{') && t.ends_with('}'))
        || (t.starts_with('[') && t.ends_with(']'));
    if bracketed && (t.contains("\":") || t.contains("\" :")) {
        return Some("wrapped in braces with quoted keys, which is JSON".into());
    }

    // TOML: [section] headings and key = value.
    let toml_sections = lines
        .iter()
        .filter(|l| {
            let l = l.trim();
            l.starts_with('[') && l.ends_with(']') && !l.contains(' ')
        })
        .count();
    let equals = lines.iter().filter(|l| {
        let t = l.trim();
        !t.starts_with('#') && t.contains(" = ")
    }).count();
    if toml_sections >= 1 && equals >= 1 {
        return Some("[section] headings over key = value pairs is TOML or INI".into());
    }

    // YAML: bare key: value at consistent indentation, no braces anywhere.
    if !t.contains('{') && !t.contains(';') && lines.len() >= 2 {
        let keyish = lines
            .iter()
            .filter(|l| {
                let t = l.trim();
                if t.is_empty() || t.starts_with('#') {
                    return false;
                }
                // "key:" or "key: value", with an unquoted key.
                t.split_once(':').is_some_and(|(k, _)| {
                    !k.is_empty()
                        && !k.starts_with('"')
                        && k.chars().all(|c| c.is_alphanumeric() || "-_ ".contains(c))
                })
            })
            .count();
        let listish = lines.iter().filter(|l| l.trim_start().starts_with("- ")).count();
        if keyish >= 2 && keyish * 2 >= lines.len() {
            return Some("unquoted key: value lines with no braces is YAML".into());
        }
        if keyish >= 1 && listish >= 2 {
            return Some("key: value plus \"- \" list items is YAML".into());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn f(text: &str) -> Format {
        detect(text).format
    }

    #[test]
    fn a_python_traceback_is_a_stack_trace_not_python_source() {
        let t = "Traceback (most recent call last):\n  File \"main.py\", line 1, in <module>\n    import requests\nModuleNotFoundError: No module named 'requests'";
        assert_eq!(f(t), Format::StackTrace);
    }

    #[test]
    fn node_and_rust_and_go_stacks_are_recognized() {
        assert_eq!(
            f("Error: nope\n    at Server.listen (node:net:1953:7)\n    at Object.<anonymous> (C:\\app\\server.js:14:8)"),
            Format::StackTrace
        );
        assert_eq!(
            f("thread 'main' panicked at src/main.rs:5:20:\nindex out of bounds"),
            Format::StackTrace
        );
        assert_eq!(
            f("panic: runtime error: index out of range\n\ngoroutine 1 [running]:\nmain.main()"),
            Format::StackTrace
        );
    }

    /// A diff of Python is a diff. Answering "this is Python" would be true and
    /// useless.
    #[test]
    fn a_diff_wins_over_the_language_inside_it() {
        let t = "diff --git a/main.py b/main.py\n--- a/main.py\n+++ b/main.py\n@@ -1,3 +1,3 @@\n-import os\n+import sys";
        assert_eq!(f(t), Format::Diff);
    }

    #[test]
    fn commands_are_recognized_with_and_without_a_pasted_prompt() {
        assert_eq!(f("git status"), Format::ShellCommand);
        assert_eq!(f("PS C:\\Users\\ada> git status"), Format::ShellCommand);
        assert_eq!(f("$ npm install"), Format::ShellCommand);
        assert_eq!(f("Get-ChildItem -Recurse"), Format::ShellCommand);
        assert_eq!(f("cargo build --release"), Format::ShellCommand);
    }

    #[test]
    fn config_formats_are_told_apart() {
        assert_eq!(f("{\n  \"name\": \"app\",\n  \"version\": \"1.0.0\"\n}"), Format::Config);
        assert_eq!(f("[package]\nname = \"app\"\nversion = \"0.1.0\""), Format::Config);
        assert_eq!(f("name: app\nversion: 1.0.0\nservices:\n  - web"), Format::Config);
    }

    #[test]
    fn source_code_falls_through_to_language_scoring() {
        assert_eq!(f("fn main() {\n    println!(\"hi\");\n}"), Format::Source);
        assert_eq!(f("def greet(name):\n    return f\"hi {name}\""), Format::Source);
    }

    /// Someone pasting a paragraph of a tutorial should be told that is what it
    /// is, rather than being given a confident language guess.
    #[test]
    fn prose_is_not_mistaken_for_code() {
        let t = "This is a paragraph of ordinary writing about programming that contains no code at all and simply describes what someone might do next.";
        assert_eq!(f(t), Format::Prose);
    }

    #[test]
    fn a_single_error_line_is_not_a_stack_trace() {
        assert_eq!(
            f("git : The term 'git' is not recognized as the name of a cmdlet"),
            Format::ErrorMessage
        );
    }

    #[test]
    fn prompt_stripping_only_fires_on_real_prompts() {
        assert_eq!(strip_prompt("PS C:\\Users\\ada> git status"), "git status");
        assert_eq!(strip_prompt("C:\\dev\\app> npm run dev"), "npm run dev");
        assert_eq!(strip_prompt("$ cargo build"), "cargo build");
        assert_eq!(strip_prompt("git status"), "git status");
        // A redirect is not a prompt, and eating the left side would be wrong.
        assert_eq!(strip_prompt("npm run build > log.txt"), "npm run build > log.txt");
    }

    #[test]
    fn nothing_is_not_a_crash() {
        assert_eq!(f(""), Format::Prose);
        assert_eq!(f("   \n  "), Format::Prose);
    }

    #[test]
    fn a_directory_tree_is_recognized() {
        let t = "project\\\n\u{251c}\u{2500}\u{2500} src\\\n\u{251c}\u{2500}\u{2500} tests\\\n\u{2514}\u{2500}\u{2500} README.md";
        assert_eq!(f(t), Format::FileListing);
    }

    #[test]
    fn every_verdict_explains_itself() {
        for sample in [
            "git status",
            "fn main() {}",
            "{\n  \"a\": 1\n}",
            "Traceback (most recent call last):\n  File \"x.py\", line 1",
        ] {
            assert!(!detect(sample).because.is_empty(), "no reason given for {sample:?}");
        }
    }
}
