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

import type { CardDetail } from "../api";

/**
 * The structured half of a language card.
 *
 * Most of a language card's value is in its frontmatter, not its prose: the
 * tells, what rules it out, the manifest that identifies the project on sight,
 * and the settle-it rules against confusable neighbors. Rendering only the
 * markdown body would show the introduction and hide the reference.
 *
 * The ordering is the recognition procedure from j1, most decisive signal first:
 * the manifest in the folder settles it outright, the tells are next, and the
 * confusable pairs are the tiebreak when two answers still look possible.
 */

interface Tell {
  pattern: string;
  note: string;
  weight: number;
}

interface Manifest {
  file: string;
  decisive?: boolean;
  note?: string;
}

interface Confusable {
  language: string;
  settle_it: string;
}

interface LanguageMeta {
  tells?: Tell[];
  rules_out?: Array<{ pattern: string; because?: string }>;
  project_fingerprint?: {
    manifests?: Manifest[];
    lockfiles?: string[];
    build_dirs?: string[];
    entry_points?: string[];
  };
  shape?: Record<string, string>;
  tooling?: Record<string, string>;
  confusable_with?: Confusable[];
  meet_it_when?: string;
  what_agents_get_wrong?: string;
  version_landscape?: string;
  errors_look_like?: { sample?: string; recognize_by?: string };
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <section className="mt-7">
      <h2 className="mb-2 text-lg font-medium text-paper-100">{title}</h2>
      {children}
    </section>
  );
}

function Mono({ children }: { children: React.ReactNode }) {
  return (
    <code className="rounded bg-ink-800 px-1.5 py-0.5 font-mono text-[0.9em] text-amber-mark">
      {children}
    </code>
  );
}

export default function LanguageCard({
  card,
  onNavigate,
}: {
  card: CardDetail;
  onNavigate: (id: string) => void;
}) {
  const m = (card.meta ?? {}) as unknown as LanguageMeta;
  const manifests = m.project_fingerprint?.manifests ?? [];
  const decisive = manifests.filter((x) => x.decisive);

  return (
    <div className="selectable">
      {/* The fastest identification there is, and the only one that works on a
          folder rather than a snippet. It goes first for that reason. */}
      {manifests.length > 0 && (
        <Section title="If you see this file, it is this language">
          <ul className="flex flex-col gap-2">
            {manifests.map((f) => (
              <li key={f.file} className="flex flex-wrap items-baseline gap-2">
                <Mono>{f.file}</Mono>
                {f.decisive && (
                  <span className="rounded bg-amber-dim/25 px-1.5 py-0.5 text-[10px] uppercase tracking-wide text-amber-mark">
                    settles it
                  </span>
                )}
                {f.note && <span className="text-sm text-paper-300">{f.note}</span>}
              </li>
            ))}
          </ul>
          {decisive.length === 0 && manifests.length > 0 && (
            <p className="mt-2 text-sm text-paper-500">
              None of these settles it alone. They name an ecosystem, not a language, so check
              the source folder too.
            </p>
          )}
        </Section>
      )}

      {/* Every note is written as a contrast against a neighbor. That contrast
          is the thing that teaches recognition, and it is also exactly what the
          identifier shows as evidence, so the two surfaces agree. */}
      {m.tells && m.tells.length > 0 && (
        <Section title="The tells">
          <ul className="flex flex-col gap-2.5">
            {[...m.tells]
              .sort((a, b) => b.weight - a.weight)
              .map((t, i) => (
                <li key={i} className="flex gap-3">
                  <span className="mt-0.5 shrink-0">
                    <Mono>{t.pattern}</Mono>
                  </span>
                  <span className="text-sm leading-relaxed text-paper-300">{t.note}</span>
                </li>
              ))}
          </ul>
        </Section>
      )}

      {m.rules_out && m.rules_out.length > 0 && (
        <Section title="What rules it out">
          <p className="mb-2 text-sm text-paper-500">
            Seeing any of these means it is not this language.
          </p>
          <ul className="flex flex-wrap gap-2">
            {m.rules_out.map((r, i) => (
              <li key={i} className="text-sm text-paper-300">
                <Mono>{r.pattern}</Mono>
                {r.because && <span className="ml-1.5 text-paper-500">{r.because}</span>}
              </li>
            ))}
          </ul>
        </Section>
      )}

      {m.confusable_with && m.confusable_with.length > 0 && (
        <Section title="Confusable with">
          <ul className="flex flex-col gap-3">
            {m.confusable_with.map((c) => (
              <li key={c.language}>
                <button
                  type="button"
                  onClick={() => onNavigate(c.language)}
                  className="font-medium text-amber-mark underline decoration-amber-dim underline-offset-2 transition-colors hover:decoration-amber-mark"
                >
                  {c.language}
                </button>
                <p className="mt-0.5 text-sm leading-relaxed text-paper-300">{c.settle_it}</p>
              </li>
            ))}
          </ul>
        </Section>
      )}

      {m.errors_look_like?.sample && (
        <Section title="What its errors look like">
          {m.errors_look_like.recognize_by && (
            <p className="mb-2 text-sm leading-relaxed text-paper-300">
              {m.errors_look_like.recognize_by}
            </p>
          )}
          <pre className="overflow-x-auto rounded-md border border-ink-700 bg-ink-850 p-3">
            <code className="font-mono text-xs text-paper-100">
              {m.errors_look_like.sample}
            </code>
          </pre>
        </Section>
      )}

      {/* The field that makes this deck useful to a vibe coder rather than to a
          student: what an agent produces here that looks fine and is not. */}
      {m.what_agents_get_wrong && (
        <Section title="What agents get wrong here">
          <p className="leading-relaxed text-paper-300">{m.what_agents_get_wrong}</p>
        </Section>
      )}

      {m.tooling && Object.keys(m.tooling).length > 0 && (
        <Section title="Tooling">
          <dl className="grid grid-cols-[auto_1fr] gap-x-4 gap-y-1.5 text-sm">
            {Object.entries(m.tooling).map(([k, v]) => (
              <div key={k} className="contents">
                <dt className="text-paper-500">{k.replace(/_/g, " ")}</dt>
                <dd className="text-paper-300">{v}</dd>
              </div>
            ))}
          </dl>
        </Section>
      )}

      {m.version_landscape && (
        <Section title="Does an old answer still apply?">
          <p className="leading-relaxed text-paper-300">{m.version_landscape}</p>
        </Section>
      )}
    </div>
  );
}
