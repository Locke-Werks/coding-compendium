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
