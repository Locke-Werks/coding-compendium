import type { CardType, Hit } from "../api";

/**
 * Search results, answer first.
 *
 * The design decision that matters here: the top result renders its answer
 * inline rather than making you click through. Clicking is what a search engine
 * makes you do, and the whole claim of this app is being faster than that. If
 * the one-sentence answer is on screen the moment you stop typing, you are done.
 */

/** Short label and color per card type, so the kind of answer is legible at a glance. */
const TYPE_STYLE: Record<CardType, { label: string; className: string }> = {
  section: { label: "guide", className: "bg-ink-700 text-paper-300" },
  language: { label: "language", className: "bg-amber-dim/30 text-amber-mark" },
  error: { label: "error", className: "bg-danger/15 text-danger" },
  command: { label: "command", className: "bg-ink-700 text-paper-300" },
  intent: { label: "task", className: "bg-ink-700 text-paper-300" },
  glossary: { label: "term", className: "bg-ink-700 text-paper-500" },
  panic: { label: "panic", className: "bg-danger/25 text-danger" },
};

function TypeBadge({ type }: { type: CardType }) {
  const style = TYPE_STYLE[type] ?? TYPE_STYLE.section;
  return (
    <span
      className={`shrink-0 rounded px-1.5 py-0.5 text-[10px] font-medium uppercase tracking-wide ${style.className}`}
    >
      {style.label}
    </span>
  );
}

interface Props {
  hits: Hit[];
  selected: number;
  onSelect: (index: number) => void;
  onOpen: (hit: Hit) => void;
}

export default function ResultList({ hits, selected, onSelect, onOpen }: Props) {
  return (
    <ul className="flex flex-col gap-1 p-2" role="listbox" aria-label="Search results">
      {hits.map((hit, i) => {
        const active = i === selected;
        return (
          <li key={hit.card_id}>
            <button
              type="button"
              role="option"
              aria-selected={active}
              // Selection follows the mouse as well as the keyboard, so the two
              // never disagree about which row is current.
              onMouseMove={() => !active && onSelect(i)}
              onClick={() => onOpen(hit)}
              className={`w-full rounded-md px-3 py-2.5 text-left transition-colors ${
                active ? "bg-ink-700" : "hover:bg-ink-800"
              }`}
            >
              <div className="flex min-w-0 items-baseline gap-2">
                <span className="truncate font-medium text-paper-100">{hit.title}</span>
                <TypeBadge type={hit.card_type} />
              </div>

              {hit.answer && (
                // Capped at 90 characters per line. Long measures are genuinely
                // harder to read, and this is prose someone is reading under
                // stress. `break-words` stops a long path or URL from forcing
                // the whole row wider than the window.
                <p
                  className={`mt-1 max-w-[90ch] break-words text-sm leading-relaxed ${
                    active ? "text-paper-300" : "text-paper-500"
                  }`}
                >
                  {hit.answer}
                </p>
              )}
            </button>
          </li>
        );
      })}
    </ul>
  );
}
