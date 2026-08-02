import { useCallback, useEffect, useRef, useState } from "react";
import { getCapabilities, inTauri, search, type Capabilities, type Hit } from "./api";
import ResultList from "./components/ResultList";

/**
 * The search surface.
 *
 * Everything here serves one claim: this should be faster than opening a browser
 * tab. Three decisions follow from that and are worth stating, because each one
 * looks like a detail and is not.
 *
 * 1. **The box is always focused.** No click to start typing.
 * 2. **Results update as you type**, with the final word treated as a prefix, so
 *    the list does not empty out between keystrokes.
 * 3. **The answer is on screen without clicking.** A search engine makes you
 *    click. That round trip is the thing being beaten.
 */

/** Wait this long after the last keystroke before searching. */
const DEBOUNCE_MS = 45;

export default function App() {
  const [query, setQuery] = useState("");
  const [hits, setHits] = useState<Hit[]>([]);
  const [selected, setSelected] = useState(0);
  const [caps, setCaps] = useState<Capabilities | null>(null);
  const [error, setError] = useState<string | null>(null);

  const inputRef = useRef<HTMLInputElement>(null);

  // Guards against a slow search overwriting a newer one. Each request carries a
  // sequence number and a stale reply is dropped. Without this, typing quickly
  // can leave results for a prefix of what is in the box, which reads as the
  // search being wrong rather than late.
  const requestSeq = useRef(0);

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

  useEffect(() => {
    if (!inTauri()) return;

    const trimmed = query.trim();
    if (!trimmed) {
      setHits([]);
      return;
    }

    const seq = ++requestSeq.current;
    const timer = setTimeout(() => {
      search(trimmed, true)
        .then((results) => {
          if (seq !== requestSeq.current) return; // a newer query has been sent
          setHits(results);
          setSelected(0);
          setError(null);
        })
        .catch((e) => {
          if (seq !== requestSeq.current) return;
          setError(String(e));
        });
    }, DEBOUNCE_MS);

    return () => clearTimeout(timer);
  }, [query]);

  const onKeyDown = useCallback(
    (e: React.KeyboardEvent) => {
      if (e.key === "ArrowDown") {
        e.preventDefault();
        setSelected((i) => Math.min(i + 1, hits.length - 1));
      } else if (e.key === "ArrowUp") {
        e.preventDefault();
        setSelected((i) => Math.max(i - 1, 0));
      } else if (e.key === "Escape") {
        // Escape clears rather than closing. Closing would lose the query, and
        // the most common reason to press it is to start over.
        setQuery("");
        setHits([]);
      }
    },
    [hits.length],
  );

  return (
    // `fixed inset-0` rather than h-full or h-screen.
    //
    // h-full is height:100% and resolves against the parent chain, so one
    // ancestor without a definite height breaks it. h-screen is 100vh, which
    // WebView2 reports slightly larger than the visible client area, so the
    // footer sat just below the bottom edge and the results list overflowed the
    // window instead of scrolling inside it. Pinning to the viewport is immune
    // to both.
    <div className="fixed inset-0 flex flex-col overflow-hidden bg-ink-900">
      <header className="border-b border-ink-700 px-4 py-3">
        <input
          ref={inputRef}
          autoFocus
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          onKeyDown={onKeyDown}
          placeholder="What do you need?"
          spellCheck={false}
          aria-label="Search the compendium"
          className="w-full bg-transparent text-lg text-paper-100 caret-amber-mark outline-none placeholder:text-paper-500"
        />
      </header>

      <main className="min-h-0 min-w-0 flex-1 overflow-y-auto overflow-x-hidden">
        {error && (
          <div className="m-4 rounded-md border border-danger/40 bg-danger/10 p-3 text-sm text-paper-300 selectable">
            {error}
          </div>
        )}

        {!error && query.trim() && hits.length === 0 && (
          <p className="p-6 text-sm text-paper-500">
            Nothing matches that yet. The corpus is still being written.
          </p>
        )}

        {hits.length > 0 && (
          <ResultList
            hits={hits}
            selected={selected}
            onSelect={setSelected}
            onOpen={() => {
              /* The reader pane lands next. */
            }}
          />
        )}

        {!query.trim() && !error && (
          <div className="p-6 text-sm text-paper-500">
            <p>Start typing. Try &ldquo;merge conflict&rdquo;, &ldquo;what is Rust&rdquo;, or an error message.</p>
          </div>
        )}
      </main>

      <footer className="flex items-center justify-between border-t border-ink-700 px-4 py-2 text-xs text-paper-500">
        <span>
          {caps?.corpus_ready ? `${caps.card_count} cards` : "no corpus"}
          {caps && !caps.synthesis && " · local search only"}
        </span>
        <span className="flex gap-3">
          <kbd className="font-mono">up/down</kbd> move
          <kbd className="font-mono">esc</kbd> clear
        </span>
      </footer>
    </div>
  );
}
