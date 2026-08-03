/**
 * The bridge to the Rust side.
 *
 * Tauri's `invoke` calls a function in the Rust process by name and returns
 * whatever it returns, as JSON. It is untyped at the boundary, so every command
 * gets a wrapper here with the types written out. That way a rename in Rust
 * breaks the build in one file rather than silently returning undefined in six
 * components.
 */

import { invoke } from "@tauri-apps/api/core";

/** Which engines found a result. Mirrors `search::Matched` in Rust. */
export type Matched = "hybrid" | "lexical_only" | "semantic_only";

/** One search result. Mirrors `search::Hit` in Rust. */
export interface Hit {
  card_id: string;
  title: string;
  card_type: CardType;
  /** The one-sentence answer, rendered inline. Null on card types without one. */
  answer: string | null;
  score: number;
  matched: Matched;
}

export type CardType =
  | "section"
  | "language"
  | "error"
  | "command"
  | "intent"
  | "glossary"
  | "panic";

export interface Capabilities {
  /** False when content.db could not be opened. */
  corpus_ready: boolean;
  card_count: number;
  /** Why the corpus failed to load, when it did. */
  load_error: string | null;
  /** Whether the local model is available. The UI must work with this false. */
  synthesis: boolean;
}

export async function getCapabilities(): Promise<Capabilities> {
  return invoke<Capabilities>("capabilities");
}

/**
 * Search the corpus.
 *
 * `live` should be true while the user is still typing: it prefix-expands the
 * final word so the result list does not empty out between keystrokes. Pass
 * false once the query is committed, where exact matching is wanted.
 */
export async function search(query: string, live = true): Promise<Hit[]> {
  return invoke<Hit[]>("search", { query, live });
}

/** What kind of thing was pasted. Mirrors `identify::Format` in Rust. */
export type Format =
  | "source"
  | "stack_trace"
  | "error_message"
  | "shell_command"
  | "diff"
  | "config"
  | "log"
  | "file_listing"
  | "prose";

/** One reason behind a guess. The evidence is the whole point of the feature. */
export interface Evidence {
  /** What matched, as it appears in the pasted text. */
  matched: string;
  /** Why it points at this language, stated against a neighbor language. */
  note: string;
  weight: number;
}

export interface Candidate {
  language_id: string;
  name: string;
  /** 0-100, a share of the total score rather than a probability. */
  confidence: number;
  evidence: Evidence[];
  /** Set when a confusable pair was settled by one decisive token. */
  tiebreak: string | null;
}

/** The card explaining a pasted error, when one matches. */
export interface KnownError {
  card_id: string;
  title: string;
  /** What actually went wrong, in plain language. */
  means: string;
}

export interface Identification {
  format: Format;
  /** One line explaining how the format was decided. */
  format_because: string;
  candidates: Candidate[];
  /** True when the top two are close enough that naming a winner would be dishonest. */
  ambiguous: boolean;
  /**
   * The card for this specific error, when the paste matches one.
   * "This is a Python crash" is half an answer; this is the other half.
   */
  known_error: KnownError | null;
}

/**
 * Identify pasted text: code, an error, a command, a config file.
 *
 * Runs locally with no model, so it returns in about a millisecond.
 */
export async function identify(text: string): Promise<Identification> {
  return invoke<Identification>("identify", { text });
}

/**
 * True when running inside the Tauri shell rather than a plain browser tab.
 *
 * `vite dev` can serve the frontend at localhost with no Rust process behind
 * it, where every invoke rejects. Checking lets the UI say so instead of
 * showing an error that looks like a bug in search.
 */
export function inTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}
