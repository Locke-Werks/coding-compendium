//! Accuracy of the paste identifier, measured against the real language cards.
//!
//! This compiles the actual `content/languages/` directory rather than fixtures,
//! so it tests the tells and weights that ship. When an author changes a tell,
//! this is what catches the change breaking a neighbor: the cards are one
//! interconnected scoring table, and raising Go's `func` weight is exactly the
//! kind of edit that quietly costs Swift a point it needed.
//!
//! Failures here are usually content bugs, not code bugs. The fix is normally a
//! weight or a rules_out entry on a card, not a change to the classifier.

use compendium_lib::compile;
use compendium_lib::identify::{Format, Identifier};
use compendium_lib::search::Index;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

fn repo_root() -> PathBuf {
    // CARGO_MANIFEST_DIR is src-tauri; the content lives one level up.
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf()
}

/// Compile the real language cards once and share the result across tests.
fn identifier() -> &'static Identifier {
    static ONCE: OnceLock<Identifier> = OnceLock::new();
    ONCE.get_or_init(|| {
        let content = repo_root().join("content");
        let cards = compile::load_cards(&content).expect("loading real content");

        let db = std::env::temp_dir().join("compendium-identify-accuracy.db");
        let mut conn = compile::create_database(&db).expect("creating test database");
        compile::write_cards(&mut conn, &cards).expect("writing cards");
        drop(conn);

        Index::open(&db).expect("opening").identifier().expect("loading identifier")
    })
}

/// Assert the top guess, with the evidence printed on failure so the reason is
/// visible without a second run.
#[track_caller]
fn assert_identifies(snippet: &str, expected: &str) {
    let out = identifier().identify(snippet);
    let top = out.candidates.first();

    let got = top.map(|c| c.language_id.as_str()).unwrap_or("<nothing>");
    if got != expected {
        let detail: Vec<String> = out
            .candidates
            .iter()
            .take(3)
            .map(|c| {
                let ev: Vec<&str> = c.evidence.iter().map(|e| e.matched.as_str()).collect();
                format!("  {} {}%  [{}]", c.language_id, c.confidence, ev.join(", "))
            })
            .collect();
        panic!(
            "expected {expected}, got {got}\nformat: {:?}\ncandidates:\n{}\nsnippet:\n{snippet}",
            out.format,
            detail.join("\n")
        );
    }
}

// --------------------------------------------------------------------------
// The languages Nyx is most likely to meet
// --------------------------------------------------------------------------

#[test]
fn identifies_python() {
    assert_identifies(
        "def greet(name):\n    if name:\n        return f\"hello {name}\"\n    return None\n\nif __name__ == \"__main__\":\n    print(greet(\"nyx\"))",
        "python",
    );
}

#[test]
fn identifies_rust() {
    assert_identifies(
        "use std::collections::HashMap;\n\nfn main() {\n    let mut scores: HashMap<String, i32> = HashMap::new();\n    scores.insert(\"nyx\".to_string(), 10);\n    println!(\"{:?}\", scores);\n}",
        "rust",
    );
}

#[test]
fn identifies_go() {
    assert_identifies(
        "package main\n\nimport \"fmt\"\n\nfunc main() {\n\tcount := 3\n\tif count > 0 {\n\t\tfmt.Println(\"hi\")\n\t}\n}",
        "go",
    );
}

#[test]
fn identifies_typescript() {
    assert_identifies(
        "export interface User {\n  id: number;\n  name: string;\n}\n\nexport function greet(user: User): string {\n  return `hello ${user.name}`;\n}",
        "typescript",
    );
}

#[test]
fn identifies_javascript() {
    assert_identifies(
        "const express = require('express');\nconst app = express();\n\napp.get('/', (req, res) => {\n  res.send('hello');\n});\n\nmodule.exports = app;",
        "javascript",
    );
}

#[test]
fn identifies_powershell() {
    assert_identifies(
        "$files = Get-ChildItem -Path $env:USERPROFILE -Recurse\nforeach ($f in $files) {\n    if ($f.Length -gt 1024) {\n        Write-Host $f.Name\n    }\n}",
        "powershell",
    );
}

#[test]
fn identifies_sql() {
    assert_identifies(
        "SELECT u.name, count(o.id) AS orders\nFROM users u\nJOIN orders o ON o.user_id = u.id\nWHERE u.created_at > '2026-01-01'\nGROUP BY u.name\nORDER BY orders DESC;",
        "sql",
    );
}

// --------------------------------------------------------------------------
// The confusable pairs, which is where a classifier actually earns its keep
// --------------------------------------------------------------------------

#[test]
fn tells_java_from_csharp() {
    assert_identifies(
        "package com.example.app;\n\npublic class Main {\n    public static void main(String[] args) {\n        System.out.println(\"hello\");\n    }\n}",
        "java",
    );
    assert_identifies(
        "namespace Example.App;\n\npublic class Program {\n    static void Main(string[] args) {\n        Console.WriteLine(\"hello\");\n    }\n}",
        "csharp",
    );
}

#[test]
fn tells_c_from_cpp() {
    assert_identifies(
        "#include <stdio.h>\n#include <stdlib.h>\n\nint main(void) {\n    char *buf = malloc(64);\n    printf(\"%s\\n\", buf);\n    free(buf);\n    return 0;\n}",
        "c",
    );
    assert_identifies(
        "#include <iostream>\n#include <vector>\n\nint main() {\n    std::vector<int> v{1, 2, 3};\n    for (auto& x : v) {\n        std::cout << x << std::endl;\n    }\n}",
        "cpp",
    );
}

#[test]
fn tells_python_from_ruby() {
    assert_identifies(
        "class Greeter\n  def initialize(name)\n    @name = name\n  end\n\n  def greet\n    puts \"hello #{@name}\"\n  end\nend",
        "ruby",
    );
}

#[test]
fn tells_bash_from_powershell() {
    assert_identifies(
        "#!/bin/bash\nset -euo pipefail\n\nfor f in \"$@\"; do\n  if [ -f \"$f\" ]; then\n    echo \"$f\"\n  fi\ndone",
        "bash",
    );
}

// --------------------------------------------------------------------------
// The formats that are not code, which nobody tells her about
// --------------------------------------------------------------------------

#[test]
fn config_formats_are_told_apart() {
    let json = identifier().identify("{\n  \"name\": \"app\",\n  \"version\": \"1.0.0\",\n  \"scripts\": { \"dev\": \"vite\" }\n}");
    assert_eq!(json.format, Format::Config);

    let toml = identifier().identify("[package]\nname = \"app\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nserde = \"1\"");
    assert_eq!(toml.format, Format::Config);

    let yaml = identifier().identify("services:\n  web:\n    image: nginx\n    ports:\n      - \"80:80\"");
    assert_eq!(yaml.format, Format::Config);
}

// --------------------------------------------------------------------------
// Shapes that are not source at all
// --------------------------------------------------------------------------

#[test]
fn a_traceback_is_reported_as_a_crash_and_still_names_its_language() {
    let out = identifier().identify(
        "Traceback (most recent call last):\n  File \"C:\\dev\\app\\main.py\", line 12, in <module>\n    import requests\nModuleNotFoundError: No module named 'requests'",
    );
    assert_eq!(out.format, Format::StackTrace, "she pasted a crash, not source");
    assert_eq!(
        out.candidates.first().map(|c| c.language_id.as_str()),
        Some("python"),
        "the crash still identifies its language"
    );
}

#[test]
fn a_command_is_a_command_not_a_language() {
    for cmd in ["git status", "PS C:\\dev\\app> npm run dev", "cargo build --release"] {
        assert_eq!(identifier().identify(cmd).format, Format::ShellCommand, "for {cmd:?}");
    }
}

#[test]
fn a_diff_is_a_diff_whatever_is_inside_it() {
    let out = identifier().identify(
        "diff --git a/main.py b/main.py\nindex 83db48f..bf269f4 100644\n--- a/main.py\n+++ b/main.py\n@@ -1,3 +1,3 @@\n-import os\n+import sys",
    );
    assert_eq!(out.format, Format::Diff);
    assert!(out.candidates.is_empty(), "answering 'this is Python' would be true and useless");
}

// --------------------------------------------------------------------------
// Behavior under uncertainty
// --------------------------------------------------------------------------

/// Every guess must be explainable. A confidence number with no reasons behind
/// it is the thing this feature exists to avoid.
#[test]
fn every_candidate_carries_evidence() {
    let out = identifier().identify("fn main() {\n    let mut x = 0;\n    println!(\"{}\", x);\n}");
    for c in &out.candidates {
        assert!(!c.evidence.is_empty(), "{} has no evidence", c.language_id);
        for e in &c.evidence {
            assert!(!e.note.is_empty(), "{} evidence {:?} has no note", c.language_id, e.matched);
            assert!(!e.matched.is_empty());
        }
    }
}

/// A snippet too short to distinguish anything should say so rather than
/// picking a winner. Confident nonsense is worse than an honest shrug.
#[test]
fn an_ambiguous_snippet_is_marked_ambiguous_or_returns_nothing() {
    let out = identifier().identify("x = 1");
    assert!(
        out.candidates.is_empty() || out.ambiguous || out.candidates[0].confidence < 60,
        "one assignment is not enough to name a language, got {:?}",
        out.candidates.iter().map(|c| (&c.language_id, c.confidence)).collect::<Vec<_>>()
    );
}

#[test]
fn empty_and_junk_input_do_not_panic() {
    for junk in ["", "   ", "\n\n\n", "!!!!", "\u{1F600}\u{1F600}"] {
        let _ = identifier().identify(junk);
    }
}
