import { useCallback, useEffect, useRef, useState } from "react";
import {
  getCapabilities,
  getCard,
  identify,
  inTauri,
  search,
  type Capabilities,
  type CardDetail,
  type Hit,
  type Identification,
} from "./api";
import CardView from "./components/CardView";
import IdentifyPanel from "./components/IdentifyPanel";
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
 * 4. **Pasting something switches modes automatically.** Nyx should never have
 *    to know she wanted "identify" rather than "search". If it looks like she
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

export default function App() {
  const [query, setQuery] = useState("");
  const [mode, setMode] = useState<Mode>("search");
  const [hits, setHits] = useState<Hit[]>([]);
  const [ident, setIdent] = useState<Identification | null>(null);
  const [card, setCard] = useState<CardDetail | null>(null);
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

  useEffect(() => {
    if (!inTauri()) return;

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
          });

      request.then(() => seq === seqRef.current && setError(null)).catch((e) => {
        if (seq === seqRef.current) setError(String(e));
      });
    }, DEBOUNCE_MS);

    return () => clearTimeout(timer);
  }, [query]);

  const open = useCallback((id: string) => {
    getCard(id)
      .then(setCard)
      .catch((e) => setError(String(e)));
  }, []);

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
        if (card) setCard(null);
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
    [card, hits, mode, open, selected],
  );

  const showHint = !query.trim() && !error;

  return (
    // `fixed inset-0` rather than h-full or h-screen. h-full resolves against
    // the parent chain, so one ancestor without a definite height breaks it.
    // h-screen is 100vh, which WebView2 reports slightly larger than the visible
    // client area, so the footer sat just below the bottom edge. Pinning to the
    // viewport is immune to both.
    <div className="fixed inset-0 flex flex-col overflow-hidden bg-ink-900">
      <header className="border-b border-ink-700 px-4 py-3">
        <textarea
          ref={inputRef}
          autoFocus
          rows={query.includes("\n") ? Math.min(query.split("\n").length, 10) : 1}
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          onKeyDown={onKeyDown}
          placeholder="Ask a question, or paste anything you do not recognize"
          spellCheck={false}
          aria-label="Search the compendium, or paste something to identify"
          className="w-full resize-none bg-transparent text-lg text-paper-100 caret-amber-mark outline-none placeholder:text-paper-500"
        />
      </header>

      <main className="min-h-0 min-w-0 flex-1 overflow-y-auto overflow-x-hidden">
        {error && (
          <div className="selectable m-4 rounded-md border border-danger/40 bg-danger/10 p-3 text-sm text-paper-300">
            {error}
          </div>
        )}

        {!error && card && <CardView card={card} onBack={() => setCard(null)} />}

        {!error && !card && mode === "identify" && ident && <IdentifyPanel result={ident} />}

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
          {caps?.corpus_ready ? `${caps.card_count} cards` : "no corpus"}
          {caps && !caps.synthesis && " · local, no network"}
        </span>
        <span className="flex gap-3">
          {card ? (
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
