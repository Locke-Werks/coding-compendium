import { useEffect, useState } from "react";
import type { CardDetail } from "../api";

/**
 * The "I broke it" walkthrough.
 *
 * The design constraint here is different from every other surface, and it comes
 * from the state the reader is in when they reach it: something is broken, they do not
 * know what, and they are not reading carefully. Everything follows from that.
 *
 * **One question on screen at a time.** No wall of options to scan.
 * **Every branch states its cost before they can pick it.** A command that
 * destroys work never appears without saying so first, in the same view as the
 * button, not a paragraph above it.
 * **A backup is offered before the first question**, because nearly every git
 * disaster is recoverable if a snapshot exists before the fix attempt, and
 * nobody thinks to take one while panicking.
 * **Every question has an escape hatch.** "I do not know" is the normal answer
 * when you are lost, so any node that asks something they may not be able to
 * answer carries a command that answers it for them.
 */

interface Branch {
  label: string;
  goto: string;
}

interface Resolve {
  command: string;
  shell?: string;
  does: string;
  destroys?: string;
  verify?: string;
  if_it_did_not_work?: string;
}

interface Node {
  ask: string;
  how_to_tell?: string;
  branches?: Branch[];
  resolve?: Resolve;
}

interface Tree {
  symptom?: string;
  reassurance?: string;
  backup_first?: string;
  root?: string;
  nodes?: Record<string, Node>;
}

function CopyableCommand({ command, label }: { command: string; label?: string }) {
  const [copied, setCopied] = useState(false);
  return (
    <div className="my-2 overflow-hidden rounded-md border border-ink-700">
      {label && (
        <div className="bg-ink-800 px-3 py-1 font-mono text-xs text-paper-500">{label}</div>
      )}
      <div className="flex items-start gap-2 bg-ink-850 p-3">
        <code className="selectable min-w-0 flex-1 break-all font-mono text-sm text-paper-100">
          {command}
        </code>
        <button
          type="button"
          onClick={() => {
            navigator.clipboard.writeText(command).then(
              () => {
                setCopied(true);
                setTimeout(() => setCopied(false), 1400);
              },
              () => {},
            );
          }}
          className="shrink-0 rounded px-2 py-0.5 text-xs text-paper-500 transition-colors hover:bg-ink-700 hover:text-paper-100"
        >
          {copied ? "copied" : "copy"}
        </button>
      </div>
    </div>
  );
}

export default function PanicView({ card, onBack }: { card: CardDetail; onBack: () => void }) {
  const tree = (card.meta ?? {}) as unknown as Tree;
  const nodes = tree.nodes ?? {};

  // The path taken, so they can step back one answer instead of restarting. A
  // wrong turn three questions deep should not cost the whole walk.
  const [path, setPath] = useState<string[]>(tree.root ? [tree.root] : []);
  const [tookBackup, setTookBackup] = useState(false);

  // A different tree means a different walk. Without this the path carries over
  // and points at node keys that do not exist in the new tree.
  useEffect(() => {
    setPath(tree.root ? [tree.root] : []);
    setTookBackup(false);
  }, [card.id, tree.root]);

  const key = path[path.length - 1];
  const node = key ? nodes[key] : undefined;

  return (
    <article className="selectable mx-auto max-w-2xl p-6">
      <button
        type="button"
        onClick={onBack}
        className="mb-4 text-xs text-paper-500 transition-colors hover:text-paper-100"
      >
        &larr; back to results
      </button>

      <h1 className="text-2xl font-medium text-paper-100">{card.title}</h1>
      {tree.symptom && <p className="mt-2 leading-relaxed text-paper-300">{tree.symptom}</p>}

      {/* Reassurance first, and honest rather than soothing. Most git disasters
          really are recoverable, and knowing that changes how carefully they
          proceed through the rest of this. */}
      {tree.reassurance && (
        <p className="mt-4 rounded-md border border-ink-700 bg-ink-850 p-3 leading-relaxed text-paper-300">
          {tree.reassurance}
        </p>
      )}

      {/* Offered before the first question, because it is the one step that
          makes everything below survivable and the one nobody takes unprompted. */}
      {tree.backup_first && path.length <= 1 && (
        <div className="mt-4">
          <div className="flex items-center gap-2">
            <span className="text-sm font-medium text-paper-100">First, take a snapshot</span>
            {tookBackup && <span className="text-xs text-paper-500">done</span>}
          </div>
          <p className="mt-1 text-sm text-paper-500">
            Nothing below can lose work you have copied somewhere safe. Thirty seconds now.
          </p>
          <CopyableCommand command={tree.backup_first} label="run in PowerShell" />
          <button
            type="button"
            onClick={() => setTookBackup(true)}
            className="text-xs text-paper-500 underline transition-colors hover:text-paper-100"
          >
            {tookBackup ? "carry on" : "skip this"}
          </button>
        </div>
      )}

      <div className="mt-6 border-t border-ink-700 pt-5">
        {!node && (
          <p className="text-sm text-paper-500">
            This tree has no starting question yet.
          </p>
        )}

        {node && (
          <>
            <h2 className="text-lg leading-relaxed text-paper-100">{node.ask}</h2>

            {/* The escape hatch. "I do not know which branch I am on" is the
                normal state when you are lost, not a failure. */}
            {node.how_to_tell && (
              <div className="mt-3">
                <p className="text-sm text-paper-500">Not sure? This tells you.</p>
                <CopyableCommand command={node.how_to_tell} />
              </div>
            )}

            {node.branches && (
              <ul className="mt-4 flex flex-col gap-2">
                {node.branches.map((b) => (
                  <li key={b.goto + b.label}>
                    <button
                      type="button"
                      onClick={() => setPath((p) => [...p, b.goto])}
                      className="w-full rounded-md border border-ink-700 bg-ink-850 px-4 py-3 text-left leading-relaxed text-paper-100 transition-colors hover:border-amber-dim hover:bg-ink-800"
                    >
                      {b.label}
                    </button>
                  </li>
                ))}
              </ul>
            )}

            {node.resolve && (
              <div className="mt-4">
                {/* Cost before command, always. They will paste the first thing
                    they see, so what it destroys has to be above it, not below. */}
                {node.resolve.destroys && (
                  <div
                    className={`rounded-md border p-3 ${
                      /^nothing/i.test(node.resolve.destroys.trim())
                        ? "border-ink-700 bg-ink-850"
                        : "border-danger/50 bg-danger/10"
                    }`}
                  >
                    <div className="text-xs uppercase tracking-wide text-paper-500">
                      What this destroys
                    </div>
                    <p className="mt-1 leading-relaxed text-paper-300">{node.resolve.destroys}</p>
                  </div>
                )}

                <CopyableCommand
                  command={node.resolve.command}
                  label={`run in ${node.resolve.shell ?? "PowerShell"}`}
                />

                <p className="mt-2 leading-relaxed text-paper-300">{node.resolve.does}</p>

                {node.resolve.verify && (
                  <div className="mt-4">
                    <div className="text-xs uppercase tracking-wide text-paper-500">
                      How to tell it worked
                    </div>
                    <p className="mt-1 leading-relaxed text-paper-300">{node.resolve.verify}</p>
                  </div>
                )}

                {node.resolve.if_it_did_not_work && (
                  <div className="mt-4 border-t border-ink-700 pt-3">
                    <div className="text-xs uppercase tracking-wide text-paper-500">
                      If that did not do it
                    </div>
                    <p className="mt-1 leading-relaxed text-paper-300">
                      {node.resolve.if_it_did_not_work}
                    </p>
                  </div>
                )}
              </div>
            )}
          </>
        )}
      </div>

      {path.length > 1 && (
        <div className="mt-6 flex gap-4 border-t border-ink-700 pt-4 text-xs text-paper-500">
          <button
            type="button"
            onClick={() => setPath((p) => p.slice(0, -1))}
            className="transition-colors hover:text-paper-100"
          >
            &larr; previous question
          </button>
          <button
            type="button"
            onClick={() => setPath(tree.root ? [tree.root] : [])}
            className="transition-colors hover:text-paper-100"
          >
            start over
          </button>
        </div>
      )}
    </article>
  );
}
