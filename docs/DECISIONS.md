# Decisions

The non-obvious calls, and why. Every one of these looks like a mistake from the
outside, which is the reason the file exists: a later reader with good instincts
will otherwise helpfully undo them.

Each entry says what was decided, what the obvious alternative was, and what
would justify changing it. If you are here because something looks wrong, read
the "what would change this" line before you touch it.

---

## The local model does not ship

Answers are extracted from retrieved cards, never generated.

A 1.7B model trained specifically to abstain was benchmarked against 50
questions, 15 of which the corpus does not answer. It abstained on 14 or 15 of
them depending on the compute backend. That is the number the gate was written
around and it passed. It did not ship anyway, for three reasons the abstention
rate does not show: the quantization that fits the target laptop's GPU is the
one that broke the machine-readable status contract, CPU latency was 20.6 s per
question on hardware faster than hers, and the single failure inverted a
security warning into instruction.

The full measurement is `PHASE0-LLM-GATE.md`. The trait boundary in
`src-tauri/src/synth/` is built so a synthesizer can be switched on later
without touching anything else, and `capabilities.synthesis` is hardcoded false.

**What would change this:** a measurement on the actual target machine, an
abstention set of 50+ uncoverable questions rather than 15, and a tuned server
with a 2048-token context.

---

## RRF k = 10, not 60

`search::RRF_K`. The published Reciprocal Rank Fusion constant is 60, and every
implementation you will find uses it.

60 was tuned for TREC runs over thousands of documents. Over a 50-item list it
compresses rank 1 and rank 50 into a 1.8x band, which throws away most of what
rank means. At k = 10, rank 1 is worth about 5.5x rank 50, so an engine that is
confident about its top hit can still win against an engine that is lukewarm
about forty things.

There is a test that fails at k = 60. It is there to make the constant
defensible rather than folklore.

---

## Fusion is weighted 1:3, not equal

`search::SEMANTIC_WEIGHT = 3.0`. Equal weight is the obvious choice and it is
measurably wrong here.

Measured over 60 queries and 686 cards, re-run 2026-08-20:

| | recall@5 | recall@1 | MRR |
|---|---|---|---|
| lexical | 90.0% | 71.7% | 0.804 |
| semantic | 95.0% | 81.7% | 0.870 |
| hybrid, equal | 98.3% | 73.3% | 0.846 |
| hybrid, 1:3 | 98.3% | 81.7% | 0.877 |

Equal weight won on recall@5 and gave up ten points of recall@1 against semantic
alone: it found the answer and then buried it, because an equal vote lets the
weaker engine outvote the stronger one on what belongs first. recall@1 is the
metric that matters, because the palette renders the top result's answer inline.

A sweep from 0.5 to 10.0 puts the peak at 3.0. `pnpm eval -- --sweep`
reproduces it.

**What would change this:** a materially different corpus. Re-run the sweep
rather than reasoning about it.

---

## No vector index

1,227 chunk vectors are held in memory and scanned linearly on every query.

An HNSW or IVF index is the reflex, and at this size it is slower: a brute-force
scan of 1,227 vectors finishes in well under a millisecond, which is faster than
the query encode that precedes it. An index would add a dependency, a build
step, and an approximation, to optimize something that is not the bottleneck.

**What would change this:** roughly 100x more content. Measure the scan before
assuming it.

---

## CLS pooling with an explicit query prefix

`bge-small` was trained with CLS pooling, taking the first token's vector rather
than the mean of all of them, and it is asymmetric: documents are embedded raw,
queries behind the instruction "Represent this sentence for searching relevant
passages: ".

Both mistakes are silent. Mean pooling produces plausible numbers that are
slightly wrong; a missing prefix lands queries in a different region than the
documents they should match. Neither errors, and the only symptom is retrieval
quietly getting worse.

`embed::QUERY_PREFIX` is applied explicitly rather than trusting a helper to
remember, and `embedding_canary` asserts a query lands within 0.70 cosine of
text that answers it. The canary is `#[ignore]`d because it downloads weights;
run it deliberately after touching anything in `embed/`.

---

## The heading path is prepended before embedding

A chunk that reads "Use `git switch` instead" means nothing on its own. The
headings above it carry the subject, so the trail is prepended to the text
before it is embedded, at a cost of a few tokens per chunk.

---

## Only the last query token is prefix-matched

While the user is still typing, `to_match_expression` appends `*` to the final
token only. Prefix-expanding every token makes short queries match almost
everything and the result list churn on each keystroke.

---

## content.db is sealed out of WAL mode before it ships

`compile::seal`. The build runs in WAL, which is right for 1,227 vector inserts.
The shipped file is a rollback journal.

This is the one that already reached a built installer once. A WAL database
creates a `-shm` file beside itself before it can be read, and that is true of a
connection opened read-only, which is the counterintuitive part. The app
installs to Program Files, where a standard user has read and execute and
nothing else. The write fails, the open fails with it, and every
non-administrator gets "content.db was not found" and a corpus of zero cards.

`tests/ships_readable.rs` asserts both the header mode and that opening the
corpus leaves no sidecar files. Do not remove the seal to "fix" a slow build.

---

## The embedding weights ship, and the app names their directory

`embed::Embedder::load_from`, called with a path under the app's resource
directory.

fastembed resolves its cache against the process working directory. An installed
app cannot count on what that is: the Start Menu shortcut sets it to the install
directory, a terminal launch does not. A miss is not an error anyone sees, it is
a 66 MB download from HuggingFace, which is the one thing this app promises
never to do.

`scripts/stage-payload.ps1` flattens the HuggingFace cache before packaging.
fastembed reads `refs/<revision>` for a commit hash and then
`snapshots/<hash>/<name>`, and never touches `blobs/`. In a real cache the
snapshot entries are symlinks into `blobs/`, which would cost 66 MB twice
because the packager follows them.

---

## The app refuses a model that did not build its vectors

`build_meta.embed_model_sha256`, written by the compiler and re-checked at
startup.

The model name and the vector width both survive a model swap. The digest does
not. Two models produce vectors in different spaces and comparing across them
does not error: it returns confident nonsense. Semantic search turns off and
says why rather than running wrong.

---

## Two databases, one of them not built yet

`content.db` is opened read-only. Anything the user writes is designed to go to
a separate `notes.db`, ATTACHed at runtime.

Two files rather than one so an app update can replace the corpus wholesale
without touching her notes, and so "the shipped content is unmodified" is a
property of the filesystem rather than a promise. The notes feature does not
exist yet; the split is reserved so that adding it later is not a migration.

---

## The release build needs `--features custom-protocol`

Tauri chooses between loading the frontend from the dev server and serving the
assets compiled into the binary on a cargo feature, not on the build profile.
Its build script is literally `let dev = !custom_protocol`.

So `cargo build --release` on its own produces an optimized, stripped,
signable binary that opens to "localhost refused to connect". It is
indistinguishable from a working build by every signal short of running it,
which is how one got forged, signed and installed here before anyone noticed.

`scripts/stage-payload.ps1` refuses a binary that does not carry the hashed
asset names from `dist/index.html`. It checks for the assets rather than for the
dev URL: Tauri serialises the whole config into the image either way, so the dev
URL is present in both builds and proves nothing. Checking the asset hashes also
catches a frontend rebuilt after the binary it was supposed to go into.

---

## Forge, not the MSI bundler

Packaging is `installer/installer.toml` plus `scripts/stage-payload.ps1`, built
with `lwforge`. `tauri.conf.json` still declares an `msi` bundle target; it is
unused.

---

## The installer requires elevation up front

`install.elevation = "required"`.

The alternative, `on-demand`, stamps `asInvoker` so the license page is read
before any UAC prompt. It is the wrong trade here, because the Forge stub has no
self-elevation path: a machine-scope install stamped `asInvoker` dies at the
first write to Program Files unless the person knew to right-click and run as
administrator. She will double-click it.

`required` puts UAC before the wizard. The license is still read before anything
is installed.
