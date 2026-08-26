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

import type { Format, Identification } from "../api";

/**
 * The answer to "what am I looking at?"
 *
 * The evidence is the feature, not the verdict. Being told "this is Rust"
 * answers the question once. Being shown which tokens said so, and what the
 * neighboring language would have used instead, means the next time they
 * recognizes it without opening anything. A reference tool that makes itself
 * unnecessary is doing its job, so the reasons get more visual weight than the
 * confidence number.
 */

/** Plain-language name for each format, and what it means for the reader's next move. */
const FORMAT_LABEL: Record<Format, { title: string; hint: string }> = {
  source: { title: "Source code", hint: "" },
  stack_trace: {
    title: "A crash report",
    hint: "This is what the program printed when it failed, not the code itself. Read the last line first.",
  },
  error_message: {
    title: "An error message",
    hint: "One line, no stack below it. The type is the part worth searching for.",
  },
  shell_command: {
    title: "A terminal command",
    hint: "Something to type, not something to save in a file.",
  },
  diff: {
    title: "A diff",
    hint: "A record of what changed. Lines starting with + were added, lines with - were removed.",
  },
  config: {
    title: "A configuration file",
    hint: "Settings, not code. Nothing here runs: it describes data for some other program to read.",
  },
  log: { title: "Log output", hint: "A record of what a program did while it ran." },
  file_listing: { title: "A file listing", hint: "The shape of a folder." },
  prose: { title: "Ordinary writing", hint: "No code here. This looks like text from a page." },
};

export default function IdentifyPanel({ result }: { result: Identification }) {
  const label = FORMAT_LABEL[result.format] ?? FORMAT_LABEL.source;
  const top = result.candidates[0];
  const rest = result.candidates.slice(1, 3);

  return (
    <div className="selectable p-4">
      {/* What kind of thing this is, always, before any language guess. */}
      <div className="mb-4 rounded-md border border-ink-700 bg-ink-850 p-3">
        <div className="text-sm font-medium text-paper-100">{label.title}</div>
        <div className="mt-0.5 text-xs text-paper-500">{result.format_because}</div>
        {label.hint && <p className="mt-2 text-sm text-paper-300">{label.hint}</p>}
      </div>

      {/* If we know this exact error, that is the answer they came for. It goes
          above the language guess, because "which language" is trivia once you
          know what actually went wrong. */}
      {result.known_error && (
        <div className="mb-4 rounded-md border border-amber-dim/50 bg-amber-dim/10 p-3">
          <div className="text-xs uppercase tracking-wide text-amber-mark">
            This is a known error
          </div>
          <div className="mt-1 font-medium text-paper-100">{result.known_error.title}</div>
          <p className="mt-1.5 text-sm leading-relaxed text-paper-300">
            {result.known_error.means}
          </p>
        </div>
      )}

      {result.candidates.length === 0 && !result.known_error && (
        <p className="text-sm text-paper-500">
          Not enough here to name a language. Paste a few more lines.
        </p>
      )}

      {top && (
        <>
          {result.ambiguous && (
            // Naming a winner on a coin flip would be worse than saying so. The
            // whole point is that they can trust the answer.
            <p className="mb-3 rounded-md border border-amber-dim/50 bg-amber-dim/10 p-2 text-sm text-paper-300">
              Genuinely close between {top.name} and {rest[0]?.name}. The evidence below is what
              separates them.
            </p>
          )}

          <div className="flex items-baseline gap-3">
            <span className="text-lg font-medium text-amber-mark">{top.name}</span>
            <span className="text-sm text-paper-500">{top.confidence}%</span>
          </div>

          {top.tiebreak && <p className="mt-1 text-sm text-paper-300">{top.tiebreak}</p>}

          {/* The evidence table. Two columns: what matched, and why it matters. */}
          <ul className="mt-3 flex flex-col gap-2">
            {top.evidence.map((e, i) => (
              <li key={i} className="flex gap-3 text-sm">
                <code className="mt-0.5 shrink-0 rounded bg-ink-700 px-1.5 py-0.5 font-mono text-xs text-amber-mark">
                  {e.matched}
                </code>
                <span className="text-paper-300">{e.note}</span>
              </li>
            ))}
          </ul>

          {rest.length > 0 && (
            <div className="mt-4 border-t border-ink-700 pt-3">
              <div className="text-xs uppercase tracking-wide text-paper-500">Also considered</div>
              <ul className="mt-1.5 flex flex-col gap-1">
                {rest.map((c) => (
                  <li key={c.language_id} className="text-sm text-paper-500">
                    {c.name} <span className="text-paper-500">{c.confidence}%</span>
                    {c.evidence[0] && (
                      <span className="ml-2 text-xs">because {c.evidence[0].matched}</span>
                    )}
                  </li>
                ))}
              </ul>
            </div>
          )}
        </>
      )}
    </div>
  );
}
