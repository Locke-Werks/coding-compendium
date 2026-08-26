# Phase 0 gate: local grounded answering

## Decision: NOT SHIPPING the generated answer. Extractive only.

Recorded 2026-08-02, after reading the measurements below.

The gate report recommends GO conditional on six things. Three of them cannot be
met, and the reason is a squeeze the abstention number alone does not show:

**The quantization that fits the target GPU is the one that fails the safety gate.**
Q4_K_M needs 2,017 MiB of VRAM and emitted a third status value, `ANSWER_SOURCE`,
six times, breaking the machine-readable contract the entire design rests on.
Q8_0 holds the contract and needs 2,686 MiB. Her card is a 2060-class laptop
part with 6GB, possibly 3GB. At 3GB, Q8_0 is the whole card while the display is
also using it, and Q4 is the option that does not work.

**Twenty seconds is not a reference tool.** 20.6 s median on CPU, measured on a
16-core Ryzen 9 7950X. Her machine is a 4-core or 6-core laptop, so expect worse.
A beginner watching a spinner for twenty seconds concludes the app is broken, and
they would be right to: the extractive path returns in milliseconds.

**The one failure is the wrong kind of failure.** Asked about connecting Prisma
to Postgres, which the corpus does not cover, it answered from
`g8-what-never-to-paste-into-a-chat`, a card whose entire purpose is to warn
against pasting connection strings. It lifted the redacted example out of the
warning and presented it as instruction. Well-formed, correctly cited, literally
quoted, and it inverted a security warning into advice. That is precisely the
failure mode this gate existed to catch, and it happened on 1 of 15.

Fourteen of fifteen is also not the confidence it sounds like. The 95% Wilson
interval on 14/15 runs from 70% to 99%, so the sample does not exclude failing
the 80% bar it appears to clear. And the abstention rate does not transfer
between machines: greedy decoding turned out not to be backend-deterministic,
with one of fifty statuses flipping between CPU and Vulkan, and it was the
confabulation that flipped.

A correct status is also not a correct answer. Three of the 35 answerable
responses were meaningfully degraded against the cards they came from. The worst
told the reader how to kill a process while dropping the card's "try Ctrl+C first" step
and inverting its ordering. The gate measured whether the model knows when to
stay quiet. It does. It did not establish that what it says when it speaks is as
good as the card it is paraphrasing, and on this evidence it is not.

None of this is a criticism of the model, which did what it claims and did it
better than a general instruct model of four times the size would have. It is a
statement about this machine, this audience, and a 380MB to 1.8GB download bought
for a feature that answers more slowly than reading.

**What ships instead:** the extractive path. Highlight the sentences within
retrieved cards that match the question, and pull the two or three most relevant
verbatim sentences into an attributed quote block. Zero hallucination risk by
construction, because nothing is generated. It cannot invert a warning into
advice, because it can only show the reader what a card already says, with the card's
name attached.

**What would reopen this:** a measurement on a real target laptop, a widened
abstention set of 50+ uncoverable questions, and a tuned `--parallel 1` server
with a 2048-token context, which the report projects would cut both the KV
allocation and the latency substantially. The trait boundary in
`src-tauri/src/synth/` is built so this can be switched on later without touching
anything else.

The rest of this document is the measurement it rests on, unedited.

---

Date: 2026-08-02
Scope: does the Compendium ship a small local model that reads retrieved cards
and writes a short cited answer when search finds relevant cards but no card
directly answers the question?

## Recommendation: GO, conditional

The gate threshold was 80% correct abstention on 15 questions the corpus does not
answer. The measured result:

| Build | Backend | Abstention on the 15 uncoverable | Correct on the 35 answerable | Unparseable status |
|---|---|---|---|---|
| OCC-RAG-1.7B **Q8_0** | CPU | **15/15 = 100%** | 35/35 = 100% | 0/50 |
| OCC-RAG-1.7B **Q8_0** | Vulkan | **14/15 = 93.3%** | 35/35 = 100% | 0/50 |
| OCC-RAG-1.7B Q4_K_M | Vulkan | 13/15 = 86.7% | 28/35 = 80% | 7/50 |

The recommendation rests on **14/15 at Q8_0**, the worse of the two Q8_0 runs.
Across both backends, 29 of 30 uncoverable trials abstained. The two runs
disagree on exactly one question, which is discussed below and is the only
confabulation observed at Q8_0 anywhere.

It is conditional on six things, all of which came out of the measurement rather
than out of design preference:

1. **Ship Q8_0, not Q4_K_M.** Q4_K_M breaks the machine-readable status contract
   that the whole design depends on. Measured, not theorized. See "The Q4
   disqualification".
2. **Fail closed on any status that is not exactly `ANSWERABLE` or
   `UNANSWERABLE`.** Q4 produced a third value, `ANSWER_SOURCE`, 6 times. Treat
   anything else, including a generation that hits the token cap without emitting
   a status, as "no answer".
3. **Ship the extractive fallback as the primary surface anyway.** The generated
   answer is an addition to highlighted source sentences, not a replacement for
   them.
4. **Widen the abstention set before shipping.** 15 questions cannot establish
   80% with confidence. The 95% Wilson interval on 14/15 is 70% to 99%. The point
   estimate clears the bar; the interval does not exclude failing it.
5. **Budget for 20 seconds, not 2.** CPU-only median end-to-end was 20.6 s per
   question on a 16-core desktop. The 1.7 s Vulkan figure is an RTX 4090 number
   and does not transfer. See "Latency is the real constraint".
6. **Re-measure on a real target machine.** Everything here was measured on a
   Ryzen 9 7950X with an RTX 4090, which is not the machine described in the
   brief. See "Hardware, and why these numbers are optimistic".

The honest one-line summary: the specialized model does what it claims, the
abstention behavior is real and not a phrase we are hoping for, one question in
15 sits close enough to the decision boundary that the backend decides it, and
on CPU the whole thing takes twenty seconds.

## The one that got through

This is the failure the gate existed to find. It is also the only question in the
whole set whose verdict changed with the compute backend.

Question: `how do i connect my app to a postgres database using prisma`
Retrieved: `j5-databases-at-a-glance`, `g8-what-never-to-paste-into-a-chat`,
`j4-reading-a-repo-you-did-not-write`, `e9-mcp`
Status emitted: `ANSWERABLE`
Answer: "Use the connection string format
`postgres://<username>:<password>@<host>:<port>/<database>`."

Everything about this output is well-formed. The cited sources are real, the
quotes are literal, the reasoning trace is coherent. It is still wrong twice
over. It does not answer the question asked, which is about wiring up Prisma
(a `schema.prisma` datasource block, `prisma generate`), none of which the corpus
covers. And it derives its answer from `g8-what-never-to-paste-into-a-chat`, a
card whose entire purpose is to warn against pasting connection strings, by
lifting the redacted example out of the warning and presenting it as
configuration advice.

The model's own reasoning contains the tell: "While the text doesn't explicitly
say 'connect to PostgreSQL using Prisma', the context of the question implies the
user wants to know how to connect". It noticed the gap and answered anyway.

At Q4_K_M the same question produced a worse version, handing back the literal
placeholder host from the warning card:
`postgres://appuser:<redacted>@db.internal:5432/orders`.

### The same prompt, on CPU, refused

Identical model file, identical prompt, identical greedy decoding
(`temperature 0`, `top_k 1`). Only the backend differed:

> The provided sources discuss PostgreSQL and Prisma in passing (source 1 mentions
> Prisma as an ORM), but none of the sources contain the specific instruction on
> how to connect an application to a PostgreSQL database using Prisma. [...]
> Therefore, the specific steps to connect an app to a PostgreSQL database using
> Prisma are not present in the text.
>
> `<|status_start|>` UNANSWERABLE `<|status_end|>`

49 of 50 questions produced identical statuses on CPU and Vulkan. This one did
not. Greedy decoding is deterministic given identical arithmetic, and the Vulkan
and CPU kernels do not produce identical arithmetic, so a question sitting close
to the decision boundary can land on either side.

Two consequences worth stating plainly. **The abstention rate is not a single
number that transfers across machines**: a user with GPU offload and a user
without can get different verdicts on the same question. And **testing once does
not certify the behavior**; whatever number a wider evaluation produces, it is a
number for that backend.

It also means the honest reading of the prisma case is not "the model confabulates
here" but "the model is not confident either way here", which is worse in one
respect. A confident failure can be found and fixed. A coin-flip cannot.

## The one it got right, for contrast

Question: `how do i deploy this to kubernetes`
Retrieved: `i1-what-deployment-means`, `yaml`, `h5-ci-cd`,
`j2-the-config-formats-nobody-explains`

Two of the four sources name Kubernetes directly. The model still refused:

> None of the provided sources contain specific instructions on how to deploy to
> Kubernetes. While source 2 mentions that Kubernetes configuration is in YAML, it
> does not explain the deployment process.
>
> `<|status_start|>` UNANSWERABLE `<|status_end|>`
> `<|answer_start|>` Not enough information `<|answer_end|>`

This is the behavior the specialized model was picked for. A general instruct
model handed those four cards writes a Kubernetes tutorial.

## 1. Model verification

All of the following was checked independently against the Hugging Face API and
arXiv, not taken from prior research.

| Claim | Verdict | Evidence |
|---|---|---|
| `occ-ai/OCC-RAG-1.7B` exists | Confirmed | 1720.6M params, `qwen3` architecture, updated 28 Jun 2026, 2.1K downloads, 13 likes |
| `occ-ai/OCC-RAG-0.6B` exists | Confirmed | 596.0M params, same architecture and date |
| MIT licensed | Confirmed with a caveat | `license: mit` in repo metadata and card frontmatter. **No LICENSE file exists in any of the four repos.** |
| Official GGUF quantizations | Confirmed | `occ-ai/OCC-RAG-1.7B-GGUF` and `occ-ai/OCC-RAG-0.6B-GGUF`, same org, published 9 Jun 2026 |
| arXiv 2606.00683 exists | Confirmed | Submitted 30 May 2026 |
| Paper matches the model card | Confirmed | Title and all ten authors match the card's BibTeX exactly |
| Base model is Qwen3 | Confirmed on HF, absent from the paper abstract | Card and HF tags say `Qwen/Qwen3-1.7B-Base`; the abstract does not name it |
| `ANSWERABLE`/`UNANSWERABLE` status token | Confirmed empirically | Verified by generation, not by claim. See section 4. |

The paper abstract independently supports the load-bearing claims: an OCC family,
an OCC-RAG variant for "faithful question answering grounded in the provided
context", both 0.6B and 1.7B released, a synthetic corpus of "over three million
examples targeting multi-hop reasoning, strict context faithfulness, and
calibrated abstention", and "structured reasoning traces with source citations
grounded in literal quotes from the context". Benchmarks named are HotpotQA,
MuSiQue, TAT-QA, ConFiQA and MuSiQue-Un, matching the card's evaluation table.

The abstract does not contain the per-benchmark numbers in the card's table, and
does not name the `ANSWERABLE`/`UNANSWERABLE` tokens. Those are card-only claims.
The token claim matters and was verified directly instead.

### License flags

Two things are worth raising before this ships in an installer.

There is **no LICENSE file** in `occ-ai/OCC-RAG-1.7B`, `occ-ai/OCC-RAG-0.6B`, or
either GGUF repo. The MIT grant exists only as a metadata field and a sentence in
the model card.

The GGUF card says the model is "Released under the MIT License, **inherited from
the base model**". That provenance is wrong: `Qwen/Qwen3-1.7B-Base` is
**Apache-2.0**, not MIT. Both are permissive and either is fine for shipping, so
the practical risk is low, but the publisher's stated reasoning for the license
does not hold up, and there is no LICENSE file to fall back on.

### Verified GGUF file sizes

Byte counts from the Hugging Face repo. The two downloaded locally matched
exactly.

| File | Bytes | MB | GiB |
|---|---|---|---|
| `OCC-RAG-1.7B-Q4_K_M.gguf` | 1,107,405,952 | 1056.10 | 1.03 |
| `OCC-RAG-1.7B-Q5_K_M.gguf` | 1,257,876,608 | 1199.60 | 1.17 |
| `OCC-RAG-1.7B-Q6_K.gguf` | 1,417,751,680 | 1352.07 | 1.32 |
| `OCC-RAG-1.7B-Q8_0.gguf` | 1,834,423,424 | 1749.44 | 1.71 |
| `OCC-RAG-1.7B-F16.gguf` | 3,447,346,304 | 3287.65 | 3.21 |
| `OCC-RAG-0.6B-Q4_K_M.gguf` | 396,701,824 | 378.32 | 0.37 |
| `OCC-RAG-0.6B-Q8_0.gguf` | 639,444,096 | 609.82 | 0.60 |

`BF16` and `F16` are byte-identical in size (3,447,346,304 each).

## 2. llama.cpp

Release `b10235`, published 2 Aug 2026.

| Asset | Download | Expanded | Files |
|---|---|---|---|
| `llama-b10235-bin-win-cpu-x64.zip` | 18,376,611 B (17.53 MB) | 47,031,136 B (44.85 MB) | 51 |
| `llama-b10235-bin-win-vulkan-x64.zip` | 34,114,173 B (32.53 MB) | 99,363,168 B (94.76 MB) | 52 |

The Vulkan build is the CPU build plus exactly one extra file:
`ggml-vulkan.dll`, 52,332,032 B (49.91 MB). Every other file is byte-identical
between the two archives, so shipping both backends costs 49.91 MB over shipping
CPU alone, not a second full copy.

CUDA, for the record on why it was excluded:

| Asset | Bytes |
|---|---|
| `llama-b10235-bin-win-cuda-12.4-x64.zip` | 250,473,605 |
| `cudart-llama-bin-win-cuda-12.4-x64.zip` | 391,443,627 |
| `llama-b10235-bin-win-cuda-13.3-x64.zip` | 146,533,711 |
| `cudart-llama-bin-win-cuda-13.3-x64.zip` | 390,970,417 |

CUDA 12.4 is 642 MB of downloads, CUDA 13.3 is 537 MB. The brief's figure of
"over 600 MB" holds for the 12.4 pair and overstates the 13.3 pair by about
70 MB. The conclusion is unaffected: both are an order of magnitude past Vulkan's
32.53 MB, and the newer CUDA 13.3 runtime is still 16x the Vulkan download for a
feature this size. Excluding CUDA was the right call.

### What is actually needed

The 51-file archive is CLI tools, multimodal binaries and a TTS binary. Running
`llama-server` as a sidecar needs a subset:

| Set | Bytes | MB |
|---|---|---|
| Core runtime (`llama-server.exe`, `llama-server-impl.dll`, `llama-common.dll`, `llama.dll`, `ggml-base.dll`, `ggml.dll`, `libomp140.x86_64.dll`) | 22,248,288 | 21.22 |
| All 14 `ggml-cpu-*.dll` microarchitecture variants | 17,300,992 | 16.50 |
| 4 variants covering any x86-64 CPU (`x64`, `sse42`, `haswell`, `alderlake`) | 4,111,360 | 3.92 |
| `ggml-vulkan.dll` | 52,332,032 | 49.91 |

Minimum useful CPU-only runtime: **26,359,648 B (25.14 MB)**.
CPU plus Vulkan: **78,691,680 B (75.05 MB)**.

## 3. Benchmarks

`llama-bench`, `OCC-RAG-1.7B-Q4_K_M.gguf`, build `221f0f635 (b10235)`.

| Backend | pp512 t/s | pp2048 t/s | tg128 t/s |
|---|---|---|---|
| CPU, 16 threads | 634.99 ± 13.30 | 536.41 ± 19.25 | 38.20 ± 0.43 |
| Vulkan, RTX 4090, `-ngl 99` | 20362.11 ± 7259.73 | 26247.49 ± 453.77 | 354.13 ± 9.79 |

### End-to-end on the real workload

Measured across the 50 gate prompts, which is more useful than synthetic
throughput. Prompt length is the four retrieved cards plus the question.

| | Q8_0 CPU | Q8_0 Vulkan | Q4_K_M Vulkan |
|---|---|---|---|
| Prompt tokens, median | 1486 (range 990 to 1786) | 1486 | 1486 |
| Generated tokens, median | 339 (range 218 to 530) | 333 (range 220 to 599) | 362 (range 230 to 1536) |
| Prompt throughput, median | 313 t/s | 21,360 t/s | 22,396 t/s |
| Generation throughput, median | 21.1 t/s | 221.5 t/s | 294.6 t/s |
| **End-to-end, median** | **20.6 s** | **1.7 s** | 1.3 s |
| End-to-end, worst | 106.3 s | 8.4 s | 8.0 s |

The Q4 worst case of 8.0 s is the runaway generation that hit the 1536-token cap
without ever emitting a status. The CPU worst case of 106.3 s is an outlier with
an unremarkable prompt (1358 tokens) and generation (312 tokens), so it is
scheduling contention rather than a property of the workload; the median is the
number to plan against.

### Latency is the real constraint

20.6 s median on CPU is the most consequential number in this document, more so
than the abstention rate.

The tested `llama-server` configuration had `n_slots = 4`, which splits the
thread pool and explains part of the gap against `llama-bench` (313 versus 536
t/s prompt, 21.1 versus 38.2 t/s generation). A single-slot configuration should
land closer to `1486 / 536 + 339 / 38.2`, about **11.5 s**. That is a projection
from the bench figures, not a measurement.

Either way the range is 11 to 21 seconds on a 16-core Zen 4 desktop for one
answer. On the 4-core or 6-core laptop the brief describes, expect meaningfully
worse. A beginner who has just typed a question and is watching a spinner for
twenty seconds will conclude the app is broken, and the extractive fallback in
section 9 returns in milliseconds.

This is a strong argument for the ordering in condition 3: show the highlighted
source sentences immediately, and let the generated answer arrive afterwards if
it arrives at all.

### Memory while loaded

| Configuration | Host working set | VRAM |
|---|---|---|
| Q4_K_M, Vulkan, `-ngl 99`, `-c 8192` | 1,312 MB | 2,017 MiB |
| Q8_0, Vulkan, `-ngl 99`, `-c 8192` | 1,942 MB | 2,686 MiB |
| Q8_0, CPU only, `-c 8192`, at load | 2,714 MB | n/a |
| Q8_0, CPU only, `-c 8192`, under load | 8,335 MB | n/a |

VRAM is measured as the delta against a 3,394 MiB idle baseline on the same GPU.

One measurement caveat that matters for a low-VRAM machine: `llama-server`
defaulted to `n_slots = 4` with `n_ctx_slot = 8192`, so it allocated four
independent 8192-token KV caches. That is also why the CPU-only figure climbs to
8.3 GB under load, which would be fatal on a 16 GB laptop already running a
browser and an editor. A single-user desktop app should pass `--parallel 1` and a
context sized to the actual prompt (2048 to 4096 is ample for four cards), which
cuts the KV allocation by roughly 4x to 8x. The figures above are an upper bound,
not a floor, but the tuned configuration was not measured and should be before
anyone relies on a memory budget.

## 4. The gate

### Method

50 questions, all written against the real corpus after reading cards across
`content/tracks/`, `content/errors/`, `content/languages/`, `content/glossary/`,
`content/commands/` and `content/panic/`.

Retrieval was not approximated. The gate uses a line-for-line replica of the
app's hybrid retriever, reading the shipped `build/content.db`:

- lexical: FTS5 `bm25(cards_fts, 10.0, 5.0, 1.0, 3.0)` negated, plus the same
  `title_boost` from `src-tauri/src/search/mod.rs`
- semantic: `bge-small-en-v1.5` with the `Represent this sentence for searching
  relevant passages: ` prefix, cosine against the stored `chunk_vectors`, best
  chunk per card
- fusion: weighted RRF, `k = 10`, lexical 1.0, semantic 3.0

Sanity check on the replica: recall@4 on the 35 answerable questions was
**35/35**. Every answerable failure below is a model failure, not a retrieval
failure.

Each question was given the top 4 fused cards as sources. Source text is the
card title, its one-line `answer`, and its two chunks closest to the query,
capped at 1800 characters. Feeding whole cards was rejected as unrealistic:
track cards run to thousands of tokens each.

Prompts used the documented structural format assembled directly rather than
through a chat template, so nothing depends on llama.cpp's Jinja handling:

```
<|im_start|>user
<|query_start|>{question}<|query_end|>
<|source_start|><|source_id|>1 {source 1}<|source_end|>
...
<|im_end|>
<|im_start|>assistant
<think>

</think>

<|query_analysis_start|>
```

Decoding was greedy (`temperature 0`, `top_k 1`), the documented default, with
`cache_prompt: false` so no run contaminates the next. Status was read from the
raw token stream via `/detokenize`, not from the rendered text, because
`llama-server` strips special tokens from `content`.

The full 50 were run three times: Q4_K_M on Vulkan, Q8_0 on Vulkan, and Q8_0 on
CPU. Same prompts, same decoding, byte-identical prompt files.

### The status token is real

This was the load-bearing claim and it holds. The raw output ends:

```
<|status_start|>
UNANSWERABLE<|status_end|>
<|answer_start|>
Not enough information<|answer_end|>
```

Rust can branch on
`<\|status_start\|>\s*(\w+)\s*<\|status_end\|>` and get a token, not a phrase it
has to pattern-match out of prose. That is a genuine improvement over hoping a
refusal sentence appears.

### Results, Q8_0

| Set | n | Vulkan | CPU |
|---|---|---|---|
| Uncoverable, abstained (`UNANSWERABLE`) | 15 | **14** | **15** |
| Uncoverable, confabulated (`ANSWERABLE`) | 15 | 1 | 0 |
| Uncoverable, no parseable status | 15 | 0 | 0 |
| Answerable, answered (`ANSWERABLE`) | 35 | 35 | 35 |
| Answerable, wrongly refused | 35 | 0 | 0 |

**Abstention: 14/15 = 93.3% on Vulkan** (95% Wilson interval 70% to 99%),
**15/15 = 100% on CPU** (interval 80% to 100%). Taking the worse of the two as
the reported figure, the gate passes at 93.3% against a threshold of 80%.

The interval is the caveat to hold onto. At n = 15 even a perfect score only just
reaches an 80% lower bound. A 93.3% point estimate carries a lower bound of 70%,
which does not exclude the model failing the threshold in truth. This is the
single biggest reason to widen the set before shipping rather than to treat the
gate as settled.

The correct abstentions cover deploying to Kubernetes, React state libraries,
Jenkins pipelines, Go compiler flags, writing a Dockerfile, Tailwind versus
Bootstrap, ESLint and Prettier, OAuth login, Redis default port, deploying to
Vercel, WebSocket servers, GitHub Actions matrix builds, diagnosing a slow
Postgres query, and publishing to npm. Several of these had strongly on-topic
retrieval; the Redis question retrieved `c6-ports-and-localhost`, which contains
a table of default ports, and the model still refused because Redis is not in it.

Worth noting against the 56% baseline that motivated this gate: not one of the 15
was answered from parametric knowledge. The model knows perfectly well what port
Redis uses and what Jenkins is. It declined to say so because the sources did not.
That is the specific behavior the specialized model was chosen for, and it is
real.

### The Q4 disqualification

Q4_K_M clears the 80% abstention threshold on its own (13/15 = 86.7%) but fails
the engineering premise of the design. 7 of 50 generations produced no usable
status. In 6 of them the status slot emitted a value that is not in the contract:

```
<|status_start|>
ANSWER_SOURCE
finish
<|answer_start|>
git reset --soft HEAD~1<|answer_end|>
```

`ANSWER_SOURCE` is not a documented status. Variants observed included
`ANSWER_SOURCE\nfinish` and `ANSWER_SOURCE\nvalue\n<|source_id|>1, <|source_id|>2`.
In all 6 the answer itself was correct; the machine-readable verdict was not. The
7th generation ran to the 1536-token cap deliberating with itself and never
emitted a status at all. That one was on an uncoverable question, so under a
fail-closed rule it would at least have shown nothing rather than confabulated,
but it cost 1536 tokens and 8 seconds to get there.

Q8_0 produced zero such failures across the same 50 prompts and lifted answerable
accuracy from 28/35 to 35/35. The off-spec status is a quantization artifact, and
1.11 GB versus 1.83 GB is the price of the contract being reliable.

## 5. Answer quality on the 35 answerable questions

Citations first, because this is where a grounded model usually cheats:

- **0 invented source ids** across 35 answers. Every `<|source_id|>N` reference
  was within the range of sources actually supplied.
- The reasoning section names **1.86 of 4 sources on average**, so it does
  discriminate. The source analysis section walks all 4 by design, so a UI that
  counts every mention will over-credit; parse citations from the reasoning
  section, not the whole trace.
- **0 answers leaked raw `<|source_id|>` markup** into the answer text.

On usefulness to a beginner, roughly 27 of 35 are clean and would help. The
remainder split into two kinds.

**Terse to the point of being unhelpful.** "what does exit code 1 actually mean"
returned "generic failure". "should i use SSH or HTTPS for github" returned
"SSH". Both are correct and both throw away the card's explanation. The model is
tuned for extractive short-form QA, not for teaching.

**Degraded against the card it came from.** Three cases are worth naming because
they show that a correct `ANSWERABLE` status does not mean a correct answer.

`what does npm ERESOLVE unable to resolve dependency tree mean`
Card: "Two packages disagree about which version of a third package they need."
Model: "It means a peer dependency is required, but the package requesting it
must be installed because both packages need to use the same copy."
The version disagreement, which is the entire meaning of the error, is gone.

`how do i kill a process that will not stop`
Card: "press Ctrl+C in the window that started it, and only look up its number
and stop it by force when Ctrl+C gets no response."
Model: "Look up the process number and stop it by force using Ctrl+Shift+Esc to
open Task Manager, then End task."
The model dropped the first and safest step and inverted the card's explicit
ordering, then welded the process-id route onto the Task Manager route into one
incoherent instruction.

`what is a stack trace and which line do i read first`
Model: "Python starts at the bottom, JavaScript and Java start at the top, Java
starts at the top, C# starts at the top, and Rust starts at the top." Java is
listed twice. Content correct, presentation degraded.

This is the quieter risk. The gate was built around confabulation on
out-of-corpus questions, and the model largely passes that. It is weaker at
faithfully compressing a card it did have, and the reader cannot check that either.
It is a strong argument for showing the source sentences next to the generated
answer rather than instead of them.

## 6. Installer arithmetic

The current app has no bundled model of its own. `src-tauri/src/lib.rs` calls
`embed::Embedder::load()` at startup and `fastembed` downloads `bge-small-en-v1.5`
on first use, so the app already performs a one-time network fetch on first run
and is not fully offline today. That precedent matters for the choice below.

| Option | Runtime | Model | Total added |
|---|---|---|---|
| Q8_0, CPU only | 25.14 MB | 1749.44 MB | **1774.58 MB (1.86 GB)** |
| Q8_0, CPU + Vulkan | 75.05 MB | 1749.44 MB | **1824.49 MB (1.91 GB)** |
| Q4_K_M, CPU + Vulkan | 75.05 MB | 1056.10 MB | 1131.15 MB (1.19 GB) |
| 0.6B Q8_0, CPU + Vulkan | 75.05 MB | 609.82 MB | 684.87 MB (0.72 GB) |

Q4_K_M is disqualified on measurement, so the real choice is **1.91 GB in the
installer** versus **75 MB in the installer and a 1.75 GB download on first use
of the feature**.

Recommendation: **download on first use.** The reasoning is not primarily size.

- The MSI goes from a few tens of MB to roughly 2 GB, which changes it from
  something you can hand someone to something you have to plan around.
- The app already downloads a model on first run, so the offline promise is
  already qualified and this does not newly break it.
- The feature is the one part of the app that should be easy to turn off. A
  model that is only fetched when the feature is first used means someone who
  never triggers it never pays for it, in bytes or in trust.
- If the gate is re-run on more questions and the number moves the wrong way, an
  un-downloaded model is a feature flag. A model already in the installer is a
  removal.

Ship the 75 MB runtime in the installer so the feature can be enabled without a
second binary download path, and fetch the 1.75 GB GGUF on first use with a
checksum check.

The 0.6B model at 610 MB is the obvious hedge if the download is unacceptable.
It was not benchmarked here. Its published refusal accuracy (86.9 versus 87.2 on
MuSiQue-Un) is close to the 1.7B, but publisher benchmarks are not this corpus,
and given that Q4 versus Q8 of the *same* model changed the status contract, a
different model size must be gated separately before anyone relies on it.

## 7. Hardware, and why these numbers are optimistic

The brief described the target as "likely a 2060-class laptop card, possibly only
3 GB VRAM". The machine these measurements ran on is not that:

| | Brief | Actual |
|---|---|---|
| GPU | 2060-class laptop, ~3 GB | **RTX 4090, 23,028 MiB VRAM** |
| CPU | not stated | **Ryzen 9 7950X, 16 cores / 32 threads** |
| RAM | not stated | **191 GB** |

This is a flag, not a footnote. Nothing here establishes that the feature is
usable on a constrained laptop.

- The Vulkan numbers (26,247 t/s prompt, 354 t/s generation, 1.7 s end-to-end) are
  4090 numbers and should be ignored for planning.
- The CPU numbers are from a 16-core Zen 4 desktop part, and they are already
  marginal at a 20.6 s median. A 4-core or 6-core mobile CPU should be expected to
  land several times slower on prompt processing and roughly 2 to 3 times slower
  on generation. That is extrapolation, clearly labeled as such, and not a
  measurement, but it puts a plausible laptop somewhere around a minute per
  answer.
- The 2,686 MiB VRAM measured for Q8_0 at `-c 8192` would not fit a 3 GB card
  alongside a desktop compositor. With `--parallel 1` and `-c 4096` it likely
  would, but that is untested.

If the real target machine is a 3 GB laptop, re-run sections 3 and 4 on it before
committing. Latency and whether Vulkan offload is viable at all will certainly
not transfer. The abstention result mostly should, since it is a property of the
model and the quantization, but the CPU-versus-Vulkan disagreement on the prisma
question shows that even that is not fully hardware-independent at the margin.

## 8. What could not be verified

- **No LICENSE file** in any of the four OCC-RAG repos. MIT rests on a metadata
  field and card prose.
- The paper's **per-benchmark numbers** were not checked against the full PDF,
  only the abstract. The card's evaluation table is unverified.
- The **0.6B model was not run**. No claim here covers it.
- The abstention set is **15 questions**. That is the number the gate specified
  and it is too few to bound the true rate usefully. See the Wilson intervals in
  section 4.
- **Nothing was measured on the intended target hardware.** This is the largest
  gap in the document.
- The **tuned single-slot server configuration** (`--parallel 1`, smaller context)
  was not measured. Both the memory ceiling and the 11.5 s latency projection for
  that configuration are inferences from other measurements.
- **Q4_K_M was not run on CPU**, so it is not known whether the `ANSWER_SOURCE`
  status defect is Vulkan-specific or intrinsic to the quantization. Given that
  the backend flipped one Q8_0 verdict, this is a real possibility rather than a
  formality. It does not change the recommendation, since Q8_0 is clean on both
  backends and is what should ship.
- **Semantic retrieval used the Python `fastembed` build** of `bge-small-en-v1.5`
  rather than the Rust `BGESmallENV15Q` the app loads. Same model and same query
  prefix, potentially different ONNX quantization. Recall@4 was 35/35, so this
  did not distort the gate.

## 9. If the number moves: the fallback

This should be built regardless, as the primary surface, with the generated
answer layered on top. It is also the entire feature if a wider abstention set
drops the rate below 80%.

**Highlight matching sentences inside retrieved cards.** The chunk vectors that
already exist in `content.db` give per-chunk similarity for free. Scoring
sentences within the top chunks against the query needs no new model and no new
storage.

**Extractive quote block.** Select the 2 or 3 most query-relevant verbatim
sentences across the retrieved cards and render them as an attributed quote,
each line carrying its card id, linked.

The property that matters: **zero hallucination risk by construction**. Nothing
is generated, so nothing can be invented. The worst failure is an irrelevant
quote, which the reader can see is irrelevant. That is categorically different from a
fluent wrong answer, which they cannot.

It is also strictly better than the model on the three degraded answers in
section 5. Quoting the ERESOLVE card verbatim tells the reader two packages disagree
about a version. The model's paraphrase does not.

---

## Reproducing

Everything below ran outside the repo, in the session scratchpad. Nothing was
added to `content/`, `src/` or `src-tauri/`.

- llama.cpp `b10235`, Windows x64 CPU and Vulkan archives from the GitHub release
- `occ-ai/OCC-RAG-1.7B-GGUF` at `Q4_K_M` and `Q8_0`, byte counts confirmed against
  the Hugging Face repo after download
- Retrieval replica reading a copy of `build/content.db`, `fastembed` 0.8.0 for
  the query encoder. The repo was being edited during this session and
  `content.db` was rebuilt afterwards; the snapshot used and the rebuilt file both
  hold 686 cards and 1228 chunk vectors, so the retrieval results stand.
- `llama-server` at `temperature 0`, `top_k 1`, `-c 8192`, prompts assembled
  manually, status parsed from `/detokenize` output
- Three full runs: Q4_K_M Vulkan, Q8_0 Vulkan, Q8_0 CPU

Measured hardware: Ryzen 9 7950X (16C/32T), 191 GB RAM, RTX 4090 (23,028 MiB),
Windows 11 Pro 26200.
