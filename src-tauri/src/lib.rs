//! Coding Compendium: an offline reference for software development in the age
//! of coding agents.
//!
//! # How the pieces fit
//!
//! ```text
//!   content/*.md  --[compile-content]-->  content.db  --[search]-->  the app
//! ```
//!
//! The markdown in `content/` is the source of truth. `compile-content` turns it
//! into a SQLite database holding the text, a full-text index, and one vector per
//! chunk. The app opens that database read-only and never writes to it.
//!
//! # Module map
//!
//! - [`search`] runs the engines and merges their results.
//! - [`compile`] builds the database. Used by the `compile-content` binary, and
//!   living in the library so it shares an embedding model with the app. Two
//!   copies of that model would eventually diverge, and vectors from different
//!   models compare as confident nonsense with no error anywhere.
//!
//! # The optionality rule
//!
//! Everything here works with no network, no account, and no language model. A
//! local model is used for one narrow job (reading retrieved cards and writing a
//! short cited answer) and is reached only through a trait with a do-nothing
//! implementation. Deleting the model file degrades exactly that feature and
//! nothing else. No code path outside `synth` may reference it.

pub mod compile;
pub mod identify;
pub mod search;

use search::{Index, Matched};
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Manager;

/// Everything the app needs at runtime.
///
/// The corpus is behind a Mutex because rusqlite's `Connection` is not `Sync`.
/// That is not a bottleneck here: a query takes well under a millisecond and
/// there is exactly one user, so contention would require her to type faster
/// than the database can answer.
struct AppState {
    corpus: Mutex<Option<Index>>,
    /// The paste identifier, built once at startup because it compiles a few
    /// hundred regexes. It holds no database handle, so it needs no lock.
    identifier: Option<identify::Identifier>,
    /// Why the corpus is missing, when it is. Shown in the UI instead of an
    /// empty result list, because "no results" and "the database did not load"
    /// look identical to someone searching and are entirely different problems.
    load_error: Option<String>,
}

/// What the frontend learns about the running app.
#[derive(Serialize)]
struct Capabilities {
    /// False when content.db could not be opened. The UI shows the reason rather
    /// than pretending the corpus is empty.
    corpus_ready: bool,
    card_count: usize,
    load_error: Option<String>,
    /// Reserved for the local model. Always false until the synthesis layer
    /// lands, and the UI is built to work with it false, which is the property
    /// that keeps that feature genuinely optional.
    synthesis: bool,
}

/// Find content.db.
///
/// In a release build it is bundled beside the executable. In development it is
/// the build artifact at the repo root, which means `pnpm tauri dev` picks up a
/// rebuild without anything being copied around.
fn locate_corpus(app: &tauri::AppHandle) -> Option<PathBuf> {
    if let Ok(dir) = app.path().resource_dir() {
        let bundled = dir.join("content.db");
        if bundled.exists() {
            return Some(bundled);
        }
    }

    // Development: walk up from the executable to the repo root. The binary
    // lives in src-tauri/target/debug, so build/content.db is four levels up.
    let exe = std::env::current_exe().ok()?;
    let mut dir = exe.parent()?.to_path_buf();
    for _ in 0..5 {
        let candidate = dir.join("build").join("content.db");
        if candidate.exists() {
            return Some(candidate);
        }
        dir = dir.parent()?.to_path_buf();
    }
    None
}

#[tauri::command]
fn capabilities(state: tauri::State<'_, AppState>) -> Capabilities {
    let guard = state.corpus.lock().expect("corpus mutex poisoned");
    let count = guard.as_ref().and_then(|i| i.card_count().ok()).unwrap_or(0);
    Capabilities {
        corpus_ready: guard.is_some(),
        card_count: count,
        load_error: state.load_error.clone(),
        synthesis: false,
    }
}

/// Search the corpus.
///
/// `live` should be true while the user is still typing, which prefix-expands
/// the final word so the result list does not empty out between keystrokes. Pass
/// false once the query is committed, where exact matching is wanted.
#[tauri::command]
fn search(
    state: tauri::State<'_, AppState>,
    query: String,
    live: bool,
) -> Result<Vec<search::Hit>, String> {
    let guard = state.corpus.lock().expect("corpus mutex poisoned");
    let Some(index) = guard.as_ref() else {
        return Err("The reference database is not loaded.".into());
    };

    let ranked = index
        .lexical(&query, live, search::CANDIDATE_DEPTH)
        .map_err(|e| format!("{e:#}"))?;

    // Only the word matcher runs today. Once the semantic engine lands, both
    // rankings go through search::fuse and this becomes a real classification
    // rather than a constant.
    index
        .hydrate(&ranked, |_| Matched::LexicalOnly)
        .map_err(|e| format!("{e:#}"))
}

#[tauri::command]
fn get_card(state: tauri::State<'_, AppState>, id: String) -> Result<search::CardDetail, String> {
    let guard = state.corpus.lock().expect("corpus mutex poisoned");
    let Some(index) = guard.as_ref() else {
        return Err("The reference database is not loaded.".into());
    };
    index.card(&id).map_err(|e| format!("{e:#}")).and_then(|c| {
        c.ok_or_else(|| format!("No card called {id}."))
    })
}

/// Identify pasted text: what kind of thing it is, and what language.
///
/// Runs locally with no model, so it answers instantly and can explain every
/// point it awarded.
#[tauri::command]
fn identify(
    state: tauri::State<'_, AppState>,
    text: String,
) -> Result<identify::Identification, String> {
    let Some(id) = state.identifier.as_ref() else {
        return Err("The reference database is not loaded.".into());
    };
    Ok(id.identify(&text))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // A missing or unreadable corpus is reported through `capabilities`
            // rather than panicking. The window still opens and explains itself,
            // which is a far better failure than an app that will not start.
            let (corpus, load_error) = match locate_corpus(app.handle()) {
                Some(path) => match Index::open(&path) {
                    Ok(index) => (Some(index), None),
                    Err(e) => (None, Some(format!("{e:#}"))),
                },
                None => (
                    None,
                    Some("content.db was not found. Run `pnpm build:content`.".to_string()),
                ),
            };

            // A corpus that loads but has no language signals is a real state
            // during content authoring, so a failure here disables the
            // identifier rather than the app.
            let identifier = corpus.as_ref().and_then(|c| c.identifier().ok());

            app.manage(AppState {
                corpus: Mutex::new(corpus),
                identifier,
                load_error,
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![capabilities, search, identify, get_card])
        .run(tauri::generate_context!())
        .expect("error while running the Coding Compendium");
}
