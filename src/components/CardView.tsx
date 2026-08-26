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

import { useState } from "react";
import Markdown from "react-markdown";
import remarkGfm from "remark-gfm";
import type { CardDetail } from "../api";
import LanguageCard from "./LanguageCard";

/**
 * The reader.
 *
 * Two things here do real work beyond rendering markdown.
 *
 * **Copy-ready commands.** Every code block gets a copy button and shows which
 * shell it is for. People paste these without reading them, so the
 * shell label is not decoration: a bash command pasted into PowerShell fails in
 * a way that looks like the instructions were wrong.
 *
 * **The stale badge, only when earned.** It appears when a card has outlived the
 * budget its own author declared, not on a fixed schedule. A date stamped on
 * everything is noise nobody reads; a badge that appears rarely is a warning
 * that means something.
 */

/** Shells whose blocks get a "this is for X" label. */
const SHELL_LABEL: Record<string, string> = {
  powershell: "PowerShell",
  bash: "Git Bash",
  cmd: "Command Prompt",
};

function CodeBlock({ language, code }: { language: string; code: string }) {
  const [copied, setCopied] = useState(false);
  const shell = SHELL_LABEL[language];

  const copy = () => {
    navigator.clipboard.writeText(code).then(
      () => {
        setCopied(true);
        setTimeout(() => setCopied(false), 1400);
      },
      () => {
        /* Clipboard denied. The text is selectable, so this is recoverable. */
      },
    );
  };

  return (
    <div className="my-3 overflow-hidden rounded-md border border-ink-700">
      <div className="flex items-center justify-between bg-ink-800 px-3 py-1.5">
        <span className="font-mono text-xs text-paper-500">
          {shell ? `run in ${shell}` : language || "text"}
        </span>
        <button
          type="button"
          onClick={copy}
          className="rounded px-2 py-0.5 text-xs text-paper-500 transition-colors hover:bg-ink-700 hover:text-paper-100"
        >
          {copied ? "copied" : "copy"}
        </button>
      </div>
      <pre className="selectable overflow-x-auto bg-ink-850 p-3">
        <code className="font-mono text-sm text-paper-100">{code}</code>
      </pre>
    </div>
  );
}

interface Props {
  card: CardDetail;
  /** Back to results, or to the previous card when there is history. */
  onBack: () => void;
  /** Follow a cross-card link. */
  onNavigate: (id: string) => void;
  /** How deep the reading history is, so the back label can say where it goes. */
  depth: number;
}

export default function CardView({ card, onBack, onNavigate, depth }: Props) {
  return (
    <article className="selectable mx-auto max-w-3xl p-4 sm:p-6">
      <button
        type="button"
        onClick={onBack}
        className="mb-4 text-xs text-paper-500 transition-colors hover:text-paper-100"
      >
        &larr; {depth > 0 ? "back" : "back to results"}
      </button>

      <div className="flex items-baseline gap-3">
        <h1 className="text-2xl font-medium text-paper-100">{card.title}</h1>
        {card.stale && (
          <span
            className="rounded bg-amber-dim/25 px-1.5 py-0.5 text-[10px] uppercase tracking-wide text-amber-mark"
            title={`Last checked ${card.verified}. This card is marked "${card.volatility}", so it is due a look.`}
          >
            may be out of date
          </span>
        )}
      </div>

      {card.answer && !card.answer_derived && (
        // Shown only when an author wrote it. A derived answer is the body's
        // first paragraph verbatim, so rendering both prints the same sentence
        // twice, once in a callout and again three lines below it.
        //
        // Rendered as markdown rather than text: answers routinely contain code
        // spans naming a command, and showing the raw backticks is exactly the
        // kind of small wrongness that makes a tool feel untrustworthy.
        <div className="mt-3 border-l-2 border-amber-dim pl-3 text-base leading-relaxed text-paper-300">
          <Markdown
            components={{
              p: ({ children }) => <p>{children}</p>,
              code: ({ children }) => (
                <code className="rounded bg-ink-800 px-1 py-0.5 font-mono text-[0.9em] text-amber-mark">
                  {children}
                </code>
              ),
            }}
          >
            {card.answer}
          </Markdown>
        </div>
      )}

      <div className="prose-compendium mt-5">
        <Markdown
          remarkPlugins={[remarkGfm]}
          components={{
            code({ className, children, ...props }) {
              const text = String(children).replace(/\n$/, "");
              const language = /language-(\w+)/.exec(className ?? "")?.[1];

              // react-markdown gives inline spans and fenced blocks to the same
              // component. A fence always carries a language- class, so that is
              // how the two are told apart.
              if (!language) {
                return (
                  <code
                    className="rounded bg-ink-800 px-1 py-0.5 font-mono text-[0.9em] text-amber-mark"
                    {...props}
                  >
                    {children}
                  </code>
                );
              }
              return <CodeBlock language={language} code={text} />;
            },
            h2: ({ children }) => (
              <h2 className="mt-7 mb-2 text-lg font-medium text-paper-100">{children}</h2>
            ),
            h3: ({ children }) => (
              <h3 className="mt-5 mb-1.5 font-medium text-paper-100">{children}</h3>
            ),
            p: ({ children }) => (
              <p className="my-3 leading-relaxed text-paper-300">{children}</p>
            ),
            ul: ({ children }) => (
              <ul className="my-3 ml-5 list-disc space-y-1.5 text-paper-300">{children}</ul>
            ),
            ol: ({ children }) => (
              <ol className="my-3 ml-5 list-decimal space-y-1.5 text-paper-300">{children}</ol>
            ),
            strong: ({ children }) => (
              <strong className="font-medium text-paper-100">{children}</strong>
            ),
            table: ({ children }) => (
              // Wide tables scroll inside their own box rather than widening the
              // page and forcing the whole article sideways.
              <div className="my-4 overflow-x-auto">
                <table className="w-full border-collapse text-sm">{children}</table>
              </div>
            ),
            th: ({ children }) => (
              <th className="border-b border-ink-600 px-2 py-1.5 text-left font-medium text-paper-100">
                {children}
              </th>
            ),
            td: ({ children }) => (
              <td className="border-b border-ink-700 px-2 py-1.5 align-top text-paper-300">
                {children}
              </td>
            ),
            blockquote: ({ children }) => (
              <blockquote className="my-3 border-l-2 border-ink-600 pl-3 text-paper-500">
                {children}
              </blockquote>
            ),
            a: ({ href, children }) => {
              // Cross-card links are written as [text](#card-id). The corpus is
              // dense with them by design: every author was told to link rather
              // than re-explain, so following them is how most of it is reached.
              const id = href?.startsWith("#") ? href.slice(1) : null;
              if (!id) {
                return (
                  <span className="text-amber-mark underline decoration-amber-dim underline-offset-2">
                    {children}
                  </span>
                );
              }
              return (
                <button
                  type="button"
                  onClick={() => onNavigate(id)}
                  className="text-amber-mark underline decoration-amber-dim underline-offset-2 transition-colors hover:decoration-amber-mark"
                >
                  {children}
                </button>
              );
            },
          }}
        >
          {card.body}
        </Markdown>
      </div>

      {/* A language card's reference material lives in frontmatter, and it is
          most of the card's value. The body is the introduction; this is the
          part they came for. */}
      {card.card_type === "language" && <LanguageCard card={card} onNavigate={onNavigate} />}

      <footer className="mt-8 border-t border-ink-700 pt-3 text-xs text-paper-500">
        Last checked {card.verified}. Changes {card.volatility === "low" ? "rarely" : card.volatility}.
      </footer>
    </article>
  );
}
