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
//! - [`search`] runs the two engines and merges their results.
//! - [`compile`] builds the database. Used by the `compile-content` binary, and
//!   living in the library so it shares an embedding model with the app. Two
//!   copies of that model would eventually diverge, and vectors from different
//!   models compare as confident nonsense with no error anywhere.
//!
//! # The optionality rule
//!
//! Everything above works with no network, no account, and no language model. A
//! local model is used for one narrow job (reading retrieved cards and writing a
//! short cited answer) and is reached only through a trait with a do-nothing
//! implementation. Deleting the model file degrades exactly that feature and
//! nothing else. No code path outside `synth` may reference it.

pub mod compile;
pub mod search;

use std::path::PathBuf;

/// Where the app looks for its data.
///
/// The corpus ships beside the executable and is read-only. Notes go in the
/// user's roaming app data so they survive an app update, which would otherwise
/// replace the whole install directory.
pub struct Paths {
    pub corpus: PathBuf,
    pub notes: PathBuf,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running the Coding Compendium");
}
