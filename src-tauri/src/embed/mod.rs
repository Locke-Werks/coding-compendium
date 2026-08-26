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

//! Turning text into vectors, so search can match meaning rather than words.
//!
//! # What an embedding is
//!
//! A small neural network reads a piece of text and produces a fixed list of
//! numbers, here 384 of them. Text that means similar things produces similar
//! numbers. That is the whole trick: once meaning is arithmetic, finding the
//! closest card is a dot product rather than a word match.
//!
//! It is what lets "it keeps making up functions that dont exist" find the card
//! about hallucinated APIs, which shares not one word with the query. BM25
//! cannot do that at any weighting, because there is nothing to match.
//!
//! # Two silent killers
//!
//! Both of these produce no error and quietly worse search, which is the worst
//! failure mode there is. Both have a test.
//!
//! **Pooling.** The model emits one vector per token; they have to be collapsed
//! into one vector per input. `bge-small` was trained with CLS pooling, taking
//! the first token's vector, not the mean of all of them. Use the mean and every
//! number is plausible and slightly wrong.
//!
//! **The query prefix.** `bge` is asymmetric: it was trained with documents
//! embedded raw and queries embedded behind the instruction
//! "Represent this sentence for searching relevant passages: ". Drop the prefix
//! and queries land in a slightly different region of the space than the
//! documents they are meant to match.
//!
//! Using the library's built-in model handles both. This module exists to make
//! that non-negotiable and to keep the compiler and the app on identical bytes:
//! vectors from two different models compare as confident nonsense.

use anyhow::{Context, Result};
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

/// Vector width for bge-small-en-v1.5.
pub const DIMS: usize = 384;

/// The instruction bge was trained to expect in front of a query.
///
/// Documents do not get it. Queries always do. Applied explicitly here rather
/// than relying on a helper to remember, because a missing prefix is invisible
/// until retrieval quality is measured.
pub const QUERY_PREFIX: &str = "Represent this sentence for searching relevant passages: ";

pub struct Embedder {
    model: TextEmbedding,
}

/// fastembed's own default: `.fastembed_cache` under the working directory.
///
/// Mirrored here rather than left implicit, because every caller now says where
/// the weights are and the digest check below has to look in the same place.
pub fn default_cache_dir() -> PathBuf {
    std::env::var("FASTEMBED_CACHE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(".fastembed_cache"))
}

impl Embedder {
    /// Load the model from the default cache.
    ///
    /// For the compiler and the eval harness, which both run from the repo root
    /// and may download on first use. The app uses [`Embedder::load_from`].
    pub fn load() -> Result<Self> {
        Self::load_from(&default_cache_dir())
    }

    /// Load the model from an explicit cache directory.
    ///
    /// Takes a second or two warm, which is why the app loads it once at
    /// startup rather than per query.
    ///
    /// The directory is explicit because fastembed resolves its default against
    /// the process working directory, and an installed app cannot count on what
    /// that is: the Start Menu shortcut sets it to the install directory, a
    /// terminal launch does not. A miss is not an error anyone sees either. It
    /// is a 66 MB download, which is the one thing this app promises never to
    /// do.
    pub fn load_from(cache_dir: &Path) -> Result<Self> {
        let model = TextEmbedding::try_new(
            InitOptions::new(EmbeddingModel::BGESmallENV15Q)
                .with_show_download_progress(false)
                .with_cache_dir(cache_dir.to_path_buf()),
        )
        .with_context(|| {
            format!("loading the bge-small embedding model from {}", cache_dir.display())
        })?;
        Ok(Self { model })
    }

    /// Embed card text. No prefix: documents are embedded raw.
    pub fn embed_documents(&mut self, texts: Vec<String>) -> Result<Vec<Vec<f32>>> {
        self.model.embed(texts, None).context("embedding documents")
    }

    /// Embed a search query, with the instruction prefix bge expects.
    pub fn embed_query(&mut self, query: &str) -> Result<Vec<f32>> {
        let prefixed = format!("{QUERY_PREFIX}{query}");
        let mut out = self.model.embed(vec![prefixed], None).context("embedding query")?;
        out.pop().context("the embedder returned nothing")
    }
}

/// SHA-256 of the ONNX weights inside a fastembed cache directory.
///
/// The compiler records this in `build_meta`; the app re-checks it before
/// loading. It is the only thing that catches a swapped model, because nothing
/// else about the two builds differs: same dimensions, same model name, same
/// code. The vectors simply live in a different space, and comparing across
/// them returns confident nonsense with no error anywhere.
///
/// The file is streamed rather than read whole. It is 66 MB, and there is no
/// reason to hold all of it to hash it.
pub fn model_digest(cache_dir: &Path) -> Result<String> {
    let onnx = walkdir::WalkDir::new(cache_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.into_path())
        .find(|p| p.extension().is_some_and(|x| x == "onnx"))
        .with_context(|| format!("no .onnx weights under {}", cache_dir.display()))?;

    let mut file = std::fs::File::open(&onnx)
        .with_context(|| format!("reading {}", onnx.display()))?;
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher)
        .with_context(|| format!("hashing {}", onnx.display()))?;

    Ok(format!("{:x}", hasher.finalize()))
}

/// Pack a vector into bytes for storage.
///
/// Little-endian f32, 1536 bytes per vector. A blob rather than a table of
/// floats: 771 chunks is 1.2 MB, and reading it is one allocation instead of
/// 300,000 row reads.
pub fn to_blob(vec: &[f32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(vec.len() * 4);
    for f in vec {
        out.extend_from_slice(&f.to_le_bytes());
    }
    out
}

/// Unpack a stored vector. Returns `None` on a wrong-sized blob, which means the
/// database was built by a different model and its vectors are meaningless here.
pub fn from_blob(blob: &[u8]) -> Option<Vec<f32>> {
    if blob.len() != DIMS * 4 {
        return None;
    }
    Some(
        blob.as_chunks::<4>()
            .0
            .iter()
            .map(|b| f32::from_le_bytes(*b))
            .collect(),
    )
}

/// Cosine similarity between two vectors, in -1 to 1.
///
/// fastembed returns normalized vectors, so this is really a dot product. The
/// magnitudes are computed anyway: it costs two multiplies per dimension and
/// removes a silent dependency on a library guarantee that could change.
pub fn cosine(a: &[f32], b: &[f32]) -> f32 {
    let mut dot = 0.0f32;
    let mut norm_a = 0.0f32;
    let mut norm_b = 0.0f32;
    for i in 0..a.len().min(b.len()) {
        dot += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }
    let denom = (norm_a.sqrt()) * (norm_b.sqrt());
    if denom == 0.0 {
        return 0.0;
    }
    dot / denom
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blobs_round_trip() {
        let v: Vec<f32> = (0..DIMS).map(|i| i as f32 * 0.001).collect();
        let back = from_blob(&to_blob(&v)).expect("should round trip");
        assert_eq!(v.len(), back.len());
        for (a, b) in v.iter().zip(&back) {
            assert!((a - b).abs() < f32::EPSILON);
        }
    }

    /// A wrong-sized blob means the database was built by a different model.
    /// Returning garbage would be far worse than returning nothing.
    #[test]
    fn a_wrong_sized_blob_is_rejected() {
        assert!(from_blob(&[0u8; 100]).is_none());
        assert!(from_blob(&[]).is_none());
        assert!(from_blob(&vec![0u8; DIMS * 4 - 4]).is_none());
    }

    #[test]
    fn cosine_behaves() {
        let a = vec![1.0, 0.0, 0.0];
        assert!((cosine(&a, &a) - 1.0).abs() < 1e-6, "a vector matches itself exactly");
        assert!(cosine(&a, &[0.0, 1.0, 0.0]).abs() < 1e-6, "orthogonal vectors score zero");
        assert!((cosine(&a, &[-1.0, 0.0, 0.0]) + 1.0).abs() < 1e-6, "opposites score -1");
        assert_eq!(cosine(&a, &[0.0, 0.0, 0.0]), 0.0, "a zero vector must not divide by zero");
    }

    /// The canary.
    ///
    /// Wrong pooling or a missing query prefix does not error. It produces
    /// plausible numbers that are slightly wrong, and the only symptom is
    /// retrieval quietly getting worse. This asserts that a query lands near
    /// text that answers it and far from text that does not.
    ///
    /// Ignored by default because it downloads a 63 MB model. Run deliberately:
    ///   cargo test --lib -- --ignored embedding_canary
    #[test]
    #[ignore = "downloads the model"]
    fn embedding_canary() {
        let mut e = Embedder::load().expect("model should load");

        let docs = e
            .embed_documents(vec![
                "A merge conflict means git could not combine two changes automatically. \
                 The markers show both versions and you choose."
                    .to_string(),
                "Rust is a compiled systems language that refuses to build your program \
                 until it can prove the memory handling is safe."
                    .to_string(),
            ])
            .expect("documents should embed");

        let q = e
            .embed_query("git stopped and asked me to pick between two versions of a file")
            .expect("query should embed");

        let on_topic = cosine(&q, &docs[0]);
        let off_topic = cosine(&q, &docs[1]);

        assert!(
            on_topic > 0.70,
            "a query should sit close to text that answers it, got {on_topic:.3}. \
             Below 0.70 usually means mean pooling instead of CLS, or a missing query prefix."
        );
        assert!(
            on_topic > off_topic + 0.15,
            "on-topic {on_topic:.3} should clearly beat off-topic {off_topic:.3}"
        );
    }

    /// The prefix is load-bearing and easy to lose in a refactor.
    #[test]
    fn the_query_prefix_is_the_one_bge_expects() {
        assert_eq!(QUERY_PREFIX, "Represent this sentence for searching relevant passages: ");
        assert!(QUERY_PREFIX.ends_with(' '), "the trailing space is part of the trained prompt");
    }
}
