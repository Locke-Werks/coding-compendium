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

/**
 * The prose rules.
 *
 * Voice consistency across forty parallel authors comes from automation, not from
 * good intentions. Everything in AUTHORING.md that can be checked mechanically is
 * checked here, and the check runs as a merge gate rather than a suggestion.
 *
 * Each rule reports a `Finding` with a line and column so the output is clickable
 * in a terminal. Rules never rewrite text: a linter that silently edits prose
 * teaches authors to ignore it.
 */

export type Severity = "error" | "warn";

export interface Finding {
  rule: string;
  severity: Severity;
  line: number;
  column: number;
  message: string;
  /** The offending text, for display. Trimmed to something readable. */
  excerpt?: string;
}

/** A rule sees the raw file body (frontmatter stripped) and returns findings. */
export interface ProseRule {
  name: string;
  severity: Severity;
  run(text: string, ctx: RuleContext): Finding[];
}

export interface RuleContext {
  /** Path relative to the repo root, for messages. */
  path: string;
  /** Parsed frontmatter, so rules can be type-aware. */
  frontmatter: Record<string, unknown>;
  /** Every card id in the corpus, for link resolution. */
  knownIds: Set<string>;
  /** Acronyms that never need expanding. */
  assumedKnown: Set<string>;
  /**
   * The body before code was blanked out.
   *
   * Rules receive the blanked text by default, because otherwise every prose
   * check fires on legitimate code. A rule that is *about* code, like the fence
   * tagging check, has to reach past that and use this instead.
   */
  raw: string;
}

// --------------------------------------------------------------------------
// Helpers
// --------------------------------------------------------------------------

/**
 * Convert a string offset into a 1-indexed line and column.
 *
 * Doing this on demand per finding is cheaper than pre-computing a line table,
 * because a clean file produces zero findings and pays nothing.
 */
function positionAt(text: string, offset: number): { line: number; column: number } {
  let line = 1;
  let lastNewline = -1;
  for (let i = 0; i < offset; i++) {
    if (text[i] === "\n") {
      line++;
      lastNewline = i;
    }
  }
  return { line, column: offset - lastNewline };
}

/**
 * Strip fenced code blocks and inline code, replacing them with equal-length
 * runs of spaces.
 *
 * Offsets are preserved so findings still point at the right line. This matters
 * because nearly every prose rule would otherwise fire on legitimate code: a
 * shell heredoc contains `--`, Rust doc comments contain `///`, and a YAML
 * sample can contain any banned word as a literal value.
 */
function blankOutCode(text: string): string {
  const blank = (m: string) => m.replace(/[^\n]/g, " ");
  return text
    .replace(/```[\s\S]*?```/g, blank) // fenced blocks, newlines preserved
    .replace(/`[^`\n]*`/g, blank); // inline spans
}

/** Scan for a regex and turn every match into a finding. */
function scan(
  text: string,
  pattern: RegExp,
  rule: string,
  severity: Severity,
  message: (m: RegExpExecArray) => string,
): Finding[] {
  const findings: Finding[] = [];
  // Clone with /g so a caller's regex literal cannot leak lastIndex between files.
  const re = new RegExp(pattern.source, pattern.flags.includes("g") ? pattern.flags : pattern.flags + "g");
  let m: RegExpExecArray | null;
  while ((m = re.exec(text)) !== null) {
    const { line, column } = positionAt(text, m.index);
    findings.push({
      rule,
      severity,
      line,
      column,
      message: message(m),
      excerpt: m[0].slice(0, 60),
    });
    if (m[0].length === 0) re.lastIndex++; // guard against zero-width infinite loops
  }
  return findings;
}

// --------------------------------------------------------------------------
// Rules
// --------------------------------------------------------------------------

/**
 * Em dashes and en dashes, banned outright.
 *
 * Matched by exact codepoint. An earlier shell-based check used a bracket
 * expression and false-positived on every directory tree in the repo, because
 * box-drawing characters (U+2500 and friends) sit near the dashes in byte space
 * under some locales. Naming the two codepoints is the only reliable form.
 */
export const noDashes: ProseRule = {
  name: "no-dashes",
  severity: "error",
  run: (text) =>
    scan(text, /[—–]/, "no-dashes", "error", (m) =>
      `${m[0] === "—" ? "Em dash" : "En dash"} is banned. Use a colon, a comma, a period, or restructure.`,
    ),
};

/** Emoji, banned in content. Covers the pictographic ranges, not every symbol. */
export const noEmoji: ProseRule = {
  name: "no-emoji",
  severity: "error",
  run: (text) =>
    scan(
      text,
      /[\u{1F300}-\u{1FAFF}\u{2600}-\u{27BF}\u{FE0F}\u{1F1E6}-\u{1F1FF}]/u,
      "no-emoji",
      "error",
      () => "Emoji are banned in content.",
    ),
};

/**
 * The banned vocabulary from AUTHORING.md section 3.
 *
 * `leverage` is only banned as a verb, but distinguishing that reliably needs a
 * parser. The corpus has no legitimate use of the noun either, so it is banned
 * outright and the message says why.
 */
const BANNED_WORDS: Array<[RegExp, string]> = [
  [/\bdelve[sd]?\b/i, "Say 'go into' or just make the point."],
  [/\bleverag(e|es|ed|ing)\b/i, "Say 'use'."],
  [/\brobust\b/i, "Say what it actually withstands."],
  [/\bseamless(ly)?\b/i, "Say what does not break."],
  [/\bcomprehensive(ly)?\b/i, "Say the scope instead of claiming completeness."],
  [/\bsimply\b/i, "Never tell the reader it is easy. Say the step."],
  [/\bit'?s worth noting\b/i, "If it is worth noting, note it."],
  [/\bthat said\b/i, "Start the sentence."],
  [/\bin today'?s landscape\b/i, "Cut it."],
  [/\bkey takeaway\b/i, "State the thing."],
  [/\bmoving forward\b/i, "Cut it, or say 'from now on'."],
  [/\bdiv(e|ing) into\b/i, "Say 'look at' or 'read'."],
  [/\bunpack(ing)?\b/i, "Say 'explain' or 'break down'."],
  [/\bin order to\b/i, "Say 'to'."],
  [/\bplease note\b/i, "Cut it."],
];

export const noBannedWords: ProseRule = {
  name: "banned-words",
  severity: "error",
  run: (text) =>
    BANNED_WORDS.flatMap(([re, advice]) =>
      scan(text, re, "banned-words", "error", (m) => `"${m[0]}" is banned. ${advice}`),
    ),
};

/**
 * British spellings.
 *
 * The corpus is Windows-first and American. A card that drifts into British
 * spelling reads as though a different person wrote it, which is exactly the
 * failure the authoring guide exists to prevent.
 */
// The `u` is mandatory in every pattern here. An earlier version wrote `colou?rs?`,
// which made the u optional and flagged the correct American spelling as British.
const BRITISH: Array<[RegExp, string]> = [
  [/\bbehaviours?\b/i, "behavior"],
  [/\bfavour(s|ed|ing|ite)?\b/i, "favor"],
  [/\bcolours?\b/i, "color"],
  [/\b\w+isation\b/i, "-ization"],

  // The -ise verbs are listed explicitly rather than matched by suffix.
  //
  // A generic /\w+ise\b/ rule looks obviously right and is not: it fires on
  // raise, surprise, otherwise, precise, promise, advise, expertise, and
  // exercise, none of which have a -ize form. That rule shipped, and authors
  // dutifully rewrote correct English to get past it before anyone noticed the
  // linter was the thing that was wrong. An explicit list cannot do that.
  [
    /\b(?:organis|realis|recognis|apologis|authoris|categoris|customis|emphasis(?=e)|final is|finalis|initialis|minimis|maximis|normalis|optimis|prioritis|randomis|serialis|specialis|standardis|summaris|synchronis|utilis|visualis|memoris|modularis|parameteris|sanitis|tokenis)(?:e|es|ed|ing)\b/i,
    "the -ize spelling",
  ],
  [/\banalys(?:e|es|ed|ing)\b/i, "analyze"],
  [/\bcancelled\b/i, "canceled"],
  [/\blicence\b/i, "license"],
  [/\bdefence\b/i, "defense"],
  [/\bcatalogue\b/i, "catalog"],
];

export const usSpelling: ProseRule = {
  name: "us-spelling",
  severity: "error",
  run: (text) =>
    BRITISH.flatMap(([re, fix]) =>
      scan(text, re, "us-spelling", "error", (m) => `"${m[0]}" is British. Use ${fix}.`),
    ),
};

/** The "not just X, but Y" construction, in any form. */
export const noNotJustBut: ProseRule = {
  name: "no-not-just-but",
  severity: "error",
  run: (text) =>
    scan(
      text,
      /\bnot (just|only|merely|simply)\b[^.!?\n]{0,60}?\bbut\b/i,
      "no-not-just-but",
      "error",
      () => "The 'not just X, but Y' construction is banned. Make the positive claim on its own.",
    ),
};

/**
 * A rhetorical question answered by the next sentence.
 *
 * Heuristic and deliberately narrow: a question ending a paragraph is usually a
 * real prompt to the reader, while one followed immediately by more prose in the
 * same paragraph is nearly always rhetorical.
 */
export const noRhetoricalQuestions: ProseRule = {
  name: "no-rhetorical-questions",
  severity: "warn",
  run: (text) =>
    scan(
      text,
      /\?\s+[A-Z][^\n]{20,}/,
      "no-rhetorical-questions",
      "warn",
      () => "Looks like a rhetorical question answered by the next sentence. Make the statement directly.",
    ),
};

/** Any AI attribution, anywhere, in any file. Non-negotiable. */
export const noAiAttribution: ProseRule = {
  name: "no-ai-attribution",
  severity: "error",
  run: (text) =>
    scan(
      text,
      /\b(co-authored-by:\s*(claude|codex|copilot)|generated with \[?claude|written by (an )?ai|as an ai\b)/i,
      "no-ai-attribution",
      "error",
      (m) => `"${m[0]}" is AI attribution. It never appears in this repo.`,
    ),
};

/**
 * Fenced code blocks must declare a shell or language.
 *
 * An untagged block loses syntax highlighting, but the real reason is that the
 * authoring guide requires every command to say which shell it needs. PowerShell
 * and Git Bash disagree about enough syntax that an untagged command is a trap.
 */
export const taggedCodeFences: ProseRule = {
  name: "tagged-code-fences",
  severity: "error",
  // A line scanner rather than a regex. Telling an opening fence from a closing
  // one requires knowing whether you are currently inside a block, which is
  // state a single regex cannot carry.
  run: (_text, ctx) => {
    const findings: Finding[] = [];
    let inside = false;
    const lines = ctx.raw.split("\n");

    for (let i = 0; i < lines.length; i++) {
      const line = lines[i] ?? "";
      const fence = /^\s*```(.*)$/.exec(line);
      if (!fence) continue;

      if (inside) {
        inside = false; // closing fence, no tag wanted
        continue;
      }
      inside = true;
      if ((fence[1] ?? "").trim() === "") {
        findings.push({
          rule: "tagged-code-fences",
          severity: "error",
          line: i + 1,
          column: 1,
          message:
            "Untagged code fence. Tag it: powershell, bash, cmd, rust, python, json, yaml, toml, or text.",
        });
      }
    }
    return findings;
  },
};

/**
 * Average sentence length.
 *
 * A warning rather than an error, because one long sentence is fine and a whole
 * card of them is not. The seed doc averages around 17 words.
 */
export const sentenceLength: ProseRule = {
  name: "sentence-length",
  severity: "warn",
  run: (text, ctx) => {
    const sentences = text
      .split(/(?<=[.!?])\s+/)
      .map((s) => s.trim())
      .filter((s) => s.length > 0 && !s.startsWith("#") && !s.startsWith("|"));
    if (sentences.length < 5) return [];
    const words = sentences.reduce((n, s) => n + s.split(/\s+/).length, 0);
    const avg = words / sentences.length;
    if (avg <= 22) return [];
    return [
      {
        rule: "sentence-length",
        severity: "warn" as const,
        line: 1,
        column: 1,
        message: `Average sentence length is ${avg.toFixed(1)} words (target 22 or under). ${ctx.path} reads long.`,
      },
    ];
  },
};

export const PROSE_RULES: ProseRule[] = [
  noDashes,
  noEmoji,
  noBannedWords,
  usSpelling,
  noNotJustBut,
  noRhetoricalQuestions,
  noAiAttribution,
  taggedCodeFences,
  sentenceLength,
];

export { blankOutCode, positionAt, scan };
