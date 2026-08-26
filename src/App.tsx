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

import { useCallback, useEffect, useRef, useState } from "react";
import {
  getCapabilities,
  getCard,
  extract,
  identify,
  inTauri,
  onSummoned,
  search,
  toggleSidecar,
  type Capabilities,
  type CardDetail,
  type Extract,
  type Hit,
  type Identification,
} from "./api";
import CardView from "./components/CardView";
import ExtractPanel from "./components/ExtractPanel";
import IdentifyPanel from "./components/IdentifyPanel";
import PanicView from "./components/PanicView";
import ResultList from "./components/ResultList";

/**
 * The one surface.
 *
 * Everything here serves one claim: this should be faster than opening a browser
 * tab. Four decisions follow from that, and each looks like a detail and is not.
 *
 * 1. **The box is always focused.** No click to start typing.
 * 2. **Results update as you type**, with the final word treated as a prefix, so
 *    the list does not empty out between keystrokes.
 * 3. **The answer is on screen without clicking.** A search engine makes you
 *    click. That round trip is the thing being beaten.
 * 4. **Pasting something switches modes automatically.** The reader should never have
 *    to know they wanted "identify" rather than "search". If it looks like they
 *    pasted a thing rather than asked a question, it gets identified.
 */

/** Wait this long after the last keystroke before searching. */
const DEBOUNCE_MS = 45;

/**
 * Does this look like pasted material rather than a typed question?
 *
 * Anything with a line break is the strong signal: nobody types a newline into a
 * search box. Beyond that, a long single line dense with code punctuation is
 * usually a command or a one-liner, and a short phrase never is.
 */
function looksPasted(text: string): boolean {
  if (text.includes("\n")) return true;
  if (text.length < 12) return false;

  const punct = (text.match(/[{}();=<>[\]$|\\/]/g) ?? []).length;
  const words = text.trim().split(/\s+/).length;
  // Roughly: more than one code character per two words, or an obvious path.
  return punct * 2 >= words || /[A-Za-z]:\\|\.\w{1,4}\b.*[:(]\d+/.test(text);
}

type Mode = "search" | "identify";

/**
 * The sidecar is the same app in a narrow strip, not a separate one.
 *
 * Signalled by a query string rather than a second HTML entry point, so both
 * windows ship one bundle and cannot drift apart. A fix to the reader lands in
 * both by construction.
 */
const IS_SIDECAR =
  typeof window !== "undefined" && window.location.search.includes("sidecar");

export default function App() {
  const [query, setQuery] = useState("");
  const [mode, setMode] = useState<Mode>("search");
  const [hits, setHits] = useState<Hit[]>([]);
  const [ident, setIdent] = useState<Identification | null>(null);
  const [excerpt, setExcerpt] = useState<Extract | null>(null);
  // A stack, not a single card. Authors were told to link rather than
  // re-explain, so the corpus is dense with cross-references and reading it
  // means following them. Without history, one click is a dead end.
  const [trail, setTrail] = useState<CardDetail[]>([]);
  const card = trail[trail.length - 1] ?? null;
  const [selected, setSelected] = useState(0);
  const [caps, setCaps] = useState<Capabilities | null>(null);
  const [error, setError] = useState<string | null>(null);

  const inputRef = useRef<HTMLTextAreaElement>(null);

  // Guards against a slow reply overwriting a newer one. Each request carries a
  // sequence number and a stale reply is dropped. Without this, typing quickly
  // can leave results for a prefix of what is in the box, which reads as the
  // search being wrong rather than late.
  const seqRef = useRef(0);

  useEffect(() => {
    if (!inTauri()) {
      setError(
        "Running outside the app shell, so there is no database to search. Use `pnpm tauri dev`.",
      );
      return;
    }
    getCapabilities()
      .then((c) => {
        setCaps(c);
        if (!c.corpus_ready) setError(c.load_error ?? "The reference database is not loaded.");
      })
      .catch((e) => setError(String(e)));
  }, []);

  // Summoning selects the existing query instead of clearing it. Typing
  // replaces it, which is what they want nearly every time, and the rest of the
  // time the previous question is still there to edit rather than retype.
  useEffect(() => {
    if (!inTauri()) return;
    let unlisten: (() => void) | undefined;

    onSummoned(() => {
      const el = inputRef.current;
      if (!el) return;
      el.focus();
      el.select();
    })
      .then((fn) => {
        unlisten = fn;
      })
      .catch(() => {
        /* No shortcut is a degraded feature, not an error worth showing. */
      });

    return () => unlisten?.();
  }, []);

  useEffect(() => {
    if (!inTauri()) return;

    setTrail([]);

    const text = query.trim();
    if (!text) {
      setHits([]);
      setIdent(null);
      setMode("search");
      return;
    }

    const wantsIdentify = looksPasted(query);
    setMode(wantsIdentify ? "identify" : "search");

    const seq = ++seqRef.current;
    const timer = setTimeout(() => {
      const request = wantsIdentify
        ? identify(query).then((r) => {
            if (seq !== seqRef.current) return;
            setIdent(r);
            setHits([]);
          })
        : search(text, true).then((r) => {
            if (seq !== seqRef.current) return;
            setHits(r);
            setIdent(null);
            setSelected(0);
            setExcerpt(null);

            // Sentence extraction runs after the results are already on screen,
            // never before. The result list is the answer; this is a shortcut
            // into it, and it must never delay what they can already read.
            if (r.length > 0) {
              extract(text, r.slice(0, 4).map((h) => h.card_id))
                .then((x) => seq === seqRef.current && setExcerpt(x))
                .catch(() => {
                  /* No excerpt is a normal outcome, not an error worth showing. */
                });
            }
          });

      request.then(() => seq === seqRef.current && setError(null)).catch((e) => {
        if (seq === seqRef.current) setError(String(e));
      });
    }, DEBOUNCE_MS);

    return () => clearTimeout(timer);
  }, [query]);

  /** Open a card fresh, discarding any reading history. */
  const open = useCallback((id: string) => {
    getCard(id)
      .then((c) => setTrail([c]))
      .catch((e) => setError(String(e)));
  }, []);

  /** Follow a link from inside a card, keeping the way back. */
  const navigate = useCallback((id: string) => {
    getCard(id)
      .then((c) => setTrail((t) => [...t, c]))
      .catch((e) => setError(String(e)));
  }, []);

  /** One step back: to the previous card, or out to the results. */
  const back = useCallback(() => setTrail((t) => t.slice(0, -1)), []);

  const onKeyDown = useCallback(
    (e: React.KeyboardEvent) => {
      // Enter opens the selected card rather than inserting a newline. A
      // textarea is used only so pasted multi-line text survives; it is not a
      // place to compose. Shift+Enter still breaks a line, for the rare case of
      // typing a snippet by hand.
      if (e.key === "Enter" && !e.shiftKey) {
        e.preventDefault();
        const hit = hits[selected];
        if (mode === "search" && hit) open(hit.card_id);
        return;
      }

      if (e.key === "Escape") {
        // Escape backs out one level: card first, then the query. Closing the
        // window would lose everything, and the usual reason to press it is to
        // start over rather than to leave.
        if (card) back();
        else setQuery("");
        return;
      }

      if (mode !== "search" || card) return;

      if (e.key === "ArrowDown") {
        e.preventDefault();
        setSelected((i) => Math.min(i + 1, hits.length - 1));
      } else if (e.key === "ArrowUp") {
        e.preventDefault();
        setSelected((i) => Math.max(i - 1, 0));
      }
    },
    [back, card, hits, mode, open, selected],
  );

  const showHint = !query.trim() && !error;

  return (
    // `fixed inset-0` rather than h-full or h-screen. h-full resolves against
    // the parent chain, so one ancestor without a definite height breaks it.
    // h-screen is 100vh, which WebView2 reports slightly larger than the visible
    // client area, so the footer sat just below the bottom edge. Pinning to the
    // viewport is immune to both.
    <div className="fixed inset-0 flex flex-col overflow-hidden bg-ink-900">
      <header className={`border-b border-ink-700 ${IS_SIDECAR ? "px-3 py-2" : "px-4 py-3"}`}>
        <textarea
          ref={inputRef}
          autoFocus
          rows={query.includes("\n") ? Math.min(query.split("\n").length, 10) : 1}
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          onKeyDown={onKeyDown}
          placeholder={IS_SIDECAR ? "Ask, or paste" : "Ask a question, or paste anything you do not recognize"}
          spellCheck={false}
          aria-label="Search the compendium, or paste something to identify"
          className={`w-full resize-none bg-transparent text-paper-100 caret-amber-mark outline-none placeholder:text-paper-500 ${IS_SIDECAR ? "text-sm" : "text-lg"}`}
        />
      </header>

      <main className="min-h-0 min-w-0 flex-1 overflow-y-auto overflow-x-hidden">
        {error && (
          <div className="selectable m-4 rounded-md border border-danger/40 bg-danger/10 p-3 text-sm text-paper-300">
            {error}
          </div>
        )}

        {/* A panic tree is a walkthrough, not an article. Rendering its raw
            markdown would show an empty body, because the whole tree lives in
            frontmatter. */}
        {!error && card?.card_type === "panic" && <PanicView card={card} onBack={back} />}

        {!error && card && card.card_type !== "panic" && (
          <CardView
            card={card}
            onBack={back}
            onNavigate={navigate}
            depth={trail.length - 1}
          />
        )}

        {!error && !card && mode === "identify" && ident && <IdentifyPanel result={ident} />}

        {!error && !card && mode === "search" && excerpt && (
          <ExtractPanel extract={excerpt} onOpen={open} />
        )}

        {!error && !card && mode === "search" && hits.length > 0 && (
          <ResultList
            hits={hits}
            selected={selected}
            onSelect={setSelected}
            onOpen={(hit) => open(hit.card_id)}
          />
        )}

        {!error && !card && mode === "search" && query.trim() && hits.length === 0 && (
          <p className="p-6 text-sm text-paper-500">
            Nothing matches that yet. The corpus is still being written.
          </p>
        )}

        {!card && showHint && (
          <div className="p-6 text-sm text-paper-500">
            <p>Two things happen here.</p>
            <ul className="mt-3 flex flex-col gap-1.5">
              <li>
                Type a question. Try &ldquo;what is a branch&rdquo; or &ldquo;how do I undo the last
                commit&rdquo;.
              </li>
              <li>
                Or paste anything you do not recognize: code, an error, a command, a config file. It
                will tell you what it is and show you how it knew.
              </li>
            </ul>
          </div>
        )}
      </main>

      <footer className="flex items-center justify-between border-t border-ink-700 px-4 py-2 text-xs text-paper-500">
        <span>
          {IS_SIDECAR ? "" : caps?.corpus_ready ? `${caps.card_count} cards` : "no corpus"}
          {/* Say which engines are running. Without the model, results are
              measurably worse, and an unexplained drop in quality is the kind
              of thing that makes a tool feel unreliable rather than degraded. */}
          {!IS_SIDECAR && caps?.corpus_ready && (
            caps.semantic
              ? " · hybrid search"
              : // The reason rides along as a tooltip. It names a fixable
                // cause instead of leaving "word match only" to look permanent.
                <span title={caps.semantic_error ?? undefined}> · word match only</span>
          )}
          {!IS_SIDECAR && caps?.corpus_ready && " · local, no network"}
          {!IS_SIDECAR && caps?.hotkey && ` · ${caps.hotkey} from anywhere`}
          {IS_SIDECAR && caps?.hotkey && `${caps.hotkey}`}
        </span>
        <span className="flex items-center gap-3">
          {!IS_SIDECAR && (
            // Presence beats findability. A strip left open beside the terminal
            // is read; a window they have to remember to open is not.
            <button
              type="button"
              onClick={() => void toggleSidecar()}
              className="rounded px-1.5 py-0.5 transition-colors hover:bg-ink-700 hover:text-paper-100"
              title="Ctrl+Shift+D. A narrow strip that stays on top, to keep beside your terminal"
            >
              dock a strip
            </button>
          )}
          {IS_SIDECAR ? null : card ? (
            <>
              <kbd className="font-mono">esc</kbd> back to results
            </>
          ) : mode === "search" ? (
            <>
              <kbd className="font-mono">up/down</kbd> move
              <kbd className="font-mono">enter</kbd> open
              <kbd className="font-mono">esc</kbd> clear
            </>
          ) : (
            <span>identifying pasted text</span>
          )}
        </span>
      </footer>
    </div>
  );
}
