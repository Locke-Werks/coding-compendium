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

import type { Extract } from "../api";

/**
 * The answer, assembled from sentences the corpus already contains.
 *
 * Nothing here was generated. Every line is verbatim from a card, and the card
 * is named beside it, which is the entire safety argument: they can click through
 * and read the sentence in its original context in one move. A generated answer
 * cannot offer that, because there is nothing to click through to.
 *
 * The framing matters as much as the content. This is deliberately presented as
 * "here is what the guide says", not "here is the answer". The first is a claim
 * the app can keep.
 */
export default function ExtractPanel({
  extract,
  onOpen,
}: {
  extract: Extract;
  onOpen: (id: string) => void;
}) {
  if (extract.weak || extract.excerpts.length === 0) return null;

  return (
    <section className="selectable mx-2 mt-2 rounded-md border border-ink-700 bg-ink-850 p-3">
      <div className="text-[10px] uppercase tracking-wide text-paper-500">
        From the guide, word for word
      </div>

      <ul className="mt-2 flex flex-col gap-3">
        {extract.excerpts.map((e, i) => (
          <li key={i}>
            <p className="leading-relaxed text-paper-100">{e.text}</p>
            <button
              type="button"
              onClick={() => onOpen(e.card_id)}
              className="mt-0.5 text-xs text-paper-500 underline decoration-ink-600 underline-offset-2 transition-colors hover:text-amber-mark hover:decoration-amber-dim"
            >
              {e.card_title}
            </button>
          </li>
        ))}
      </ul>
    </section>
  );
}
