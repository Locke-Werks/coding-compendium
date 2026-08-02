/**
 * The content linter.
 *
 * Runs as a merge gate over everything in content/. Two passes, because half the
 * checks need to know about the whole corpus: a link cannot be validated until
 * every id has been collected, and a duplicate id is invisible from inside one
 * file.
 *
 *   pnpm lint:content
 *
 * Exit code 0 when there are no errors, 1 when there are. Warnings never fail the
 * build, which keeps the gate credible: an author who has seen the linter cry
 * wolf once stops reading its output.
 */

import { readFileSync, readdirSync, statSync, existsSync } from "node:fs";
import { join, relative, basename, extname, sep } from "node:path";
import { fileURLToPath } from "node:url";
import matter from "gray-matter";
import { parse as parseYaml } from "yaml";
// The default `ajv` export only understands Draft-07. Our schemas declare
// Draft 2020-12, which needs this build.
import Ajv2020 from "ajv/dist/2020.js";
import type { ValidateFunction } from "ajv";
import addFormats from "ajv-formats";
import { PROSE_RULES, blankOutCode, positionAt, scan, type Finding } from "./rules.js";

const ROOT = join(fileURLToPath(new URL(".", import.meta.url)), "..", "..");
const CONTENT = join(ROOT, "content");
const SCHEMA = join(ROOT, "schema");

// --------------------------------------------------------------------------
// Loading
// --------------------------------------------------------------------------

interface Card {
  path: string; // relative to repo root, for messages
  id: string;
  type: string;
  frontmatter: Record<string, any>;
  body: string;
  /** Line offset of the body within the file, so findings point at the real line. */
  bodyLine: number;
}

/** Every .md under content/, excluding _meta which holds yml control files. */
function walk(dir: string, out: string[] = []): string[] {
  for (const entry of readdirSync(dir)) {
    const full = join(dir, entry);
    if (statSync(full).isDirectory()) {
      if (entry !== "_meta") walk(full, out);
    } else if (extname(entry) === ".md") {
      out.push(full);
    }
  }
  return out;
}

function loadCard(absPath: string): { card?: Card; findings: Finding[] } {
  const path = relative(ROOT, absPath).split(sep).join("/");
  const raw = readFileSync(absPath, "utf8");

  let parsed: matter.GrayMatterFile<string>;
  try {
    parsed = matter(raw);
  } catch (err) {
    return {
      findings: [
        {
          rule: "frontmatter-parse",
          severity: "error",
          line: 1,
          column: 1,
          message: `Frontmatter is not valid YAML: ${(err as Error).message}`,
        },
      ],
    };
  }

  const fm = parsed.data as Record<string, any>;
  const findings: Finding[] = [];

  // YAML parses an unquoted 2026-08-02 into a Date, so a schema expecting a
  // string rejects it. Normalize rather than making every author remember to
  // quote dates, which is the kind of rule nobody follows twice.
  for (const key of Object.keys(fm)) {
    if (fm[key] instanceof Date) fm[key] = fm[key].toISOString().slice(0, 10);
  }

  if (!fm.id) {
    findings.push({ rule: "frontmatter-required", severity: "error", line: 1, column: 1, message: "Missing `id`." });
  }
  if (!fm.type) {
    findings.push({ rule: "frontmatter-required", severity: "error", line: 1, column: 1, message: "Missing `type`." });
  }
  if (findings.length > 0) return { findings };

  // The filename is the id. Keeping them in lockstep means a reader who finds a
  // card in search can find the file, and it makes a duplicate id impossible to
  // create accidentally by copying a file.
  const expected = basename(absPath, ".md");
  if (fm.id !== expected) {
    findings.push({
      rule: "id-matches-filename",
      severity: "error",
      line: 1,
      column: 1,
      message: `Frontmatter id "${fm.id}" does not match filename "${expected}.md".`,
    });
  }

  // gray-matter strips the frontmatter block; count its lines so body findings
  // report the line number the author actually sees in their editor.
  const bodyLine = raw.slice(0, raw.length - parsed.content.length).split("\n").length - 1;

  return { card: { path, id: fm.id, type: fm.type, frontmatter: fm, body: parsed.content, bodyLine }, findings };
}

// --------------------------------------------------------------------------
// Schema validation
// --------------------------------------------------------------------------

function buildValidators(): Map<string, ValidateFunction> {
  const ajv = new Ajv2020({ allErrors: true, strict: false, allowUnionTypes: true });
  addFormats(ajv as never);

  // common.schema.json is referenced by $ref from every other schema, so it has
  // to be registered before the ones that depend on it compile.
  ajv.addSchema(JSON.parse(readFileSync(join(SCHEMA, "common.schema.json"), "utf8")), "common.schema.json");

  const validators = new Map<string, ValidateFunction>();
  for (const type of ["section", "language", "error", "command", "intent", "glossary", "panic"]) {
    const file = join(SCHEMA, `${type}.schema.json`);
    if (!existsSync(file)) continue;
    validators.set(type, ajv.compile(JSON.parse(readFileSync(file, "utf8"))));
  }
  return validators;
}

// --------------------------------------------------------------------------
// Structural rules
// --------------------------------------------------------------------------

/**
 * `answer` must be one sentence under 45 words.
 *
 * Checked on words rather than characters because the limit is about how much a
 * reader absorbs from a search result, not about layout.
 */
function checkAnswerLength(card: Card): Finding[] {
  if (card.type !== "section" || typeof card.frontmatter.answer !== "string") return [];
  const words = card.frontmatter.answer.trim().split(/\s+/).length;
  if (words <= 45) return [];
  return [
    {
      rule: "answer-length",
      severity: "error",
      line: 1,
      column: 1,
      message: `\`answer\` is ${words} words (limit 45). It renders in the palette, so it has to land in one read.`,
    },
  ];
}

/**
 * The acronym contract: every acronym gets a parenthetical expansion at its first
 * occurrence in *this card*, not the first occurrence in the corpus.
 *
 * In a searchable app there is no "first". Most people arrive at a card from a
 * search box, so every card has to stand alone.
 */
function checkAcronyms(card: Card, assumedKnown: Set<string>): Finding[] {
  const text = blankOutCode(card.body);
  const seen = new Set<string>();
  const findings: Finding[] = [];

  for (const m of text.matchAll(/\b[A-Z]{2,6}\b/g)) {
    const acronym = m[0];
    if (assumedKnown.has(acronym) || seen.has(acronym)) continue;
    seen.add(acronym);

    // An expansion is a parenthetical immediately after the acronym, or the
    // acronym immediately after a parenthetical-free spelled-out form.
    const after = text.slice(m.index!, m.index! + acronym.length + 120);
    const expanded = /^[A-Z]{2,6}\s*\([A-Z][^)]{4,}\)/.test(after);
    if (expanded) continue;

    const { line, column } = positionAt(text, m.index!);
    findings.push({
      rule: "acronym-contract",
      severity: "error",
      line: line + card.bodyLine,
      column,
      message: `"${acronym}" needs a parenthetical expansion at its first use in this card, e.g. "${acronym} (Spelled Out Form)". Add it to content/_meta/assumed.yml if it truly never needs expanding.`,
      excerpt: acronym,
    });
  }
  return findings;
}

/** Every see_also and inline [text](#card-id) link has to resolve. */
function checkLinks(card: Card, knownIds: Set<string>): Finding[] {
  const findings: Finding[] = [];

  for (const ref of (card.frontmatter.see_also as string[] | undefined) ?? []) {
    if (!knownIds.has(ref)) {
      findings.push({
        rule: "link-resolves",
        severity: "error",
        line: 1,
        column: 1,
        message: `see_also points at "${ref}", which does not exist.`,
      });
    }
  }

  for (const m of card.body.matchAll(/\]\(#([a-z0-9-]+)\)/g)) {
    const target = m[1];
    // The capture group is non-optional in the pattern, so this only satisfies
    // noUncheckedIndexedAccess. It cannot actually be undefined here.
    if (target === undefined) continue;
    if (!knownIds.has(target)) {
      const { line, column } = positionAt(card.body, m.index!);
      findings.push({
        rule: "link-resolves",
        severity: "error",
        line: line + card.bodyLine,
        column,
        message: `Link points at "#${target}", which does not exist.`,
      });
    }
  }
  return findings;
}

/**
 * Anything that can lose work has to say so, in the card, before she runs it.
 *
 * The command schema has a `destructive` flag, but a destructive command can also
 * appear inside a prose section or a panic tree, so the check is textual and
 * applies to every card type.
 */
const DESTRUCTIVE = [
  /git\s+reset\s+--hard/,
  /git\s+push\s+.*--force(?!-with-lease)/,
  /git\s+push\s+.*\s-f\b/,
  /git\s+clean\s+-[a-z]*[fd]/,
  /git\s+branch\s+-D/,
  /git\s+checkout\s+--\s/,
  /rm\s+-rf/,
  /Remove-Item\s+.*-Recurse.*-Force/i,
  /DROP\s+(TABLE|DATABASE)/i,
];

function checkDangerAnnotation(card: Card): Finding[] {
  const hasDanger =
    typeof card.frontmatter.danger === "string" ||
    card.frontmatter.destructive === true ||
    typeof card.frontmatter.destroys === "string";
  if (hasDanger) return [];

  const findings: Finding[] = [];
  for (const re of DESTRUCTIVE) {
    findings.push(
      ...scan(card.body, re, "danger-annotation", "error", (m) =>
        `\`${m[0]}\` can lose work and this card has no \`danger:\` annotation. State what it destroys and name the safe alternative first.`,
      ).map((f) => ({ ...f, line: f.line + card.bodyLine })),
    );
  }
  return findings;
}

/** A card with an install command should give her a way to confirm it worked. */
function checkVerifyPresent(card: Card): Finding[] {
  if (typeof card.frontmatter.verify === "string") return [];
  const installs = /\b(winget install|npm i(nstall)? -g|pnpm add -g|cargo install|pip install|irm .*\| ?iex)\b/;
  if (!installs.test(card.body)) return [];
  return [
    {
      rule: "verify-present",
      severity: "warn",
      line: 1,
      column: 1,
      message:
        "Card contains an install command but has no `verify:`. Every instruction gets a check step, per AUTHORING.md rule 4.",
    },
  ];
}

/** `full` should go deeper than `more`, or it is not a third tier. */
function checkTierDepth(card: Card): Finding[] {
  if (card.type !== "section") return [];
  // `\Z` is Perl and Ruby, not JavaScript, where it is just a literal Z. The
  // end-of-input assertion here is `$(?![\s\S])`. Getting this wrong made the
  // final tier capture nothing and report every section as too short.
  const more = card.body.match(/^##\s+More\s*$([\s\S]*?)(?=^##\s|$(?![\s\S]))/m)?.[1] ?? "";
  const full = card.body.match(/^##\s+Full\s*$([\s\S]*?)(?=^##\s|$(?![\s\S]))/m)?.[1] ?? "";

  if (!more.trim()) {
    return [{ rule: "tier-structure", severity: "error", line: 1, column: 1, message: "Section is missing a `## More` tier." }];
  }
  if (!full.trim()) {
    return [{ rule: "tier-structure", severity: "error", line: 1, column: 1, message: "Section is missing a `## Full` tier." }];
  }
  if (full.length < more.length) {
    return [
      {
        rule: "tier-depth",
        severity: "warn",
        line: 1,
        column: 1,
        message: "`full` is shorter than `more`. The third tier should go deeper, not restate.",
      },
    ];
  }
  return [];
}

// --------------------------------------------------------------------------
// Main
// --------------------------------------------------------------------------

/**
 * Read a control file from content/_meta.
 *
 * These are hand-maintained YAML, so a malformed one is a normal authoring
 * mistake rather than an internal error. Report it the way every other finding is
 * reported instead of dying with a stack trace: a beginner who sees a Node crash
 * dump learns nothing about the missing quote mark that caused it.
 */
function readMeta<T>(name: string): T | undefined {
  const file = join(CONTENT, "_meta", name);
  if (!existsSync(file)) return undefined;
  try {
    return parseYaml(readFileSync(file, "utf8")) as T;
  } catch (err) {
    const e = err as { linePos?: Array<{ line: number; col: number }>; message?: string };
    const at = e.linePos?.[0] ? ` at line ${e.linePos[0].line}, column ${e.linePos[0].col}` : "";
    console.error(`\ncontent/_meta/${name}`);
    console.error(`  error  ${"".padStart(4)}     yaml${" ".repeat(19)}${e.message?.split("\n")[0] ?? "invalid YAML"}${at}`);
    console.error(
      `\nHint: a value containing ": " must be quoted, e.g. title: "The loop: describe, review".\n`,
    );
    process.exit(1);
  }
}

/**
 * Every acronym the linter will not demand an expansion for.
 *
 * assumed.yml groups its entries under headings for human readers. Flattening
 * here means a new group needs no code change and, more usefully, that no entry
 * has to be listed twice.
 */
function loadAssumedKnown(): Set<string> {
  const doc = readMeta<Record<string, unknown>>("assumed.yml") ?? {};
  const all = Object.values(doc)
    .filter((v): v is string[] => Array.isArray(v))
    .flat();
  return new Set(all);
}

export interface FileReport {
  path: string;
  findings: Finding[];
}

/**
 * Lint a directory of cards and return findings per file.
 *
 * Separated from the CLI so the self-test can call it directly. A linter that
 * only reports zero on clean input proves nothing, so tools/lint/selftest.ts
 * runs this against fixtures full of deliberate violations and asserts each one
 * is caught. Three real regressions were found that way within an hour of
 * writing the rules.
 */
export function lintCorpus(contentDir: string, extraKnownIds: Set<string> = new Set()): FileReport[] {
  const files = walk(contentDir);
  const assumedKnown = loadAssumedKnown();
  const validators = buildValidators();

  // Pass 1: load everything and collect ids. Link resolution and duplicate
  // detection are impossible from inside a single file.
  const cards: Card[] = [];
  const perFile = new Map<string, Finding[]>();
  const knownIds = new Set<string>();
  const idOwner = new Map<string, string>();

  for (const file of files) {
    const { card, findings } = loadCard(file);
    const path = relative(ROOT, file).split(sep).join("/");
    if (findings.length) perFile.set(path, [...(perFile.get(path) ?? []), ...findings]);
    if (!card) continue;

    if (idOwner.has(card.id)) {
      perFile.set(path, [
        ...(perFile.get(path) ?? []),
        {
          rule: "duplicate-id",
          severity: "error",
          line: 1,
          column: 1,
          message: `id "${card.id}" is already used by ${idOwner.get(card.id)}.`,
        },
      ]);
    } else {
      idOwner.set(card.id, card.path);
      knownIds.add(card.id);
    }
    cards.push(card);
  }

  // Section ids are also declared in _meta/sections.yml. Anything listed there
  // counts as known, so an author can link to a section that is assigned but not
  // yet written. Without this, the very first prose card cannot link to anything.
  const sections = readMeta<{ sections?: Array<{ id: string }> }>("sections.yml");
  for (const s of sections?.sections ?? []) knownIds.add(s.id);

  // Same for language cards. Every tell on a language card is stated as a
  // contrast against a neighbor, so a card is close to useless until it can
  // link to languages nobody has written yet.
  const langs = readMeta<{
    full?: Array<{ id: string }>;
    short?: Array<{ id: string }>;
    retired?: Array<{ id: string }>;
  }>("languages.yml");
  for (const group of [langs?.full, langs?.short, langs?.retired]) {
    for (const l of group ?? []) knownIds.add(l.id);
  }

  for (const id of extraKnownIds) knownIds.add(id);

  // Pass 2: validate each card against its schema and run every rule.
  for (const card of cards) {
    const findings = perFile.get(card.path) ?? [];

    const validate = validators.get(card.type);
    if (!validate) {
      findings.push({
        rule: "unknown-type",
        severity: "error",
        line: 1,
        column: 1,
        message: `Unknown card type "${card.type}". No schema for it.`,
      });
    } else if (!validate(card.frontmatter)) {
      for (const err of validate.errors ?? []) {
        findings.push({
          rule: "schema",
          severity: "error",
          line: 1,
          column: 1,
          message: `frontmatter${err.instancePath} ${err.message}`,
        });
      }
    }

    const proseText = blankOutCode(card.body);
    const ctx = { path: card.path, frontmatter: card.frontmatter, knownIds, assumedKnown, raw: card.body };
    for (const rule of PROSE_RULES) {
      findings.push(...rule.run(proseText, ctx).map((f) => ({ ...f, line: f.line + card.bodyLine })));
    }

    findings.push(
      ...checkAnswerLength(card),
      ...checkAcronyms(card, assumedKnown),
      ...checkLinks(card, knownIds),
      ...checkDangerAnnotation(card),
      ...checkVerifyPresent(card),
      ...checkTierDepth(card),
    );

    if (findings.length) perFile.set(card.path, findings);
    else perFile.delete(card.path);
  }

  return [...perFile]
    .sort(([a], [b]) => a.localeCompare(b))
    .map(([path, findings]) => ({
      path,
      // Errors before warnings, then by line, so the first thing on screen is
      // the thing blocking the merge.
      findings: findings.sort((a, b) =>
        a.severity === b.severity ? a.line - b.line : a.severity === "error" ? -1 : 1,
      ),
    }));
}

/** Count cards without linting, for the summary line. */
function countCards(dir: string): number {
  return walk(dir).length;
}

function main(): void {
  if (!existsSync(CONTENT)) {
    console.error(`No content directory at ${CONTENT}`);
    process.exit(1);
  }

  const reports = lintCorpus(CONTENT);
  let errors = 0;
  let warnings = 0;

  for (const { path, findings } of reports) {
    console.log(`\n${path}`);
    for (const f of findings) {
      const tag = f.severity === "error" ? "error" : " warn";
      console.log(`  ${tag}  ${String(f.line).padStart(4)}:${String(f.column).padEnd(3)} ${f.rule.padEnd(22)} ${f.message}`);
      if (f.severity === "error") errors++;
      else warnings++;
    }
  }

  const n = countCards(CONTENT);
  console.log(
    `\n${n} card${n === 1 ? "" : "s"} checked. ${errors} error${errors === 1 ? "" : "s"}, ${warnings} warning${warnings === 1 ? "" : "s"}.`,
  );
  process.exit(errors > 0 ? 1 : 0);
}

// Only run the CLI when invoked directly, so selftest.ts can import lintCorpus
// without the linter running itself and calling process.exit.
if (process.argv[1] && import.meta.url.endsWith(basename(process.argv[1]))) {
  main();
}
