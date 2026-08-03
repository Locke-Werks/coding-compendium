/**
 * Self-test for the linter.
 *
 * A linter is only worth running if it can fail. This feeds it a fixture with
 * every rule broken on purpose and asserts each rule fires, then asserts the
 * rules stay quiet on the things they must not flag.
 *
 * That second half matters more than it looks. Three separate bugs showed up
 * within an hour of writing the rules, and all three were false behavior rather
 * than missing behavior:
 *
 *   - `colou?rs?` made the u optional, so the correct American spelling was
 *     reported as British.
 *   - `\Z` is Perl, not JavaScript, where it is a literal Z. The tier-depth
 *     check silently captured nothing and reported every section as too short.
 *   - The fence rule ran on text that had already had its fences blanked out,
 *     so it could never fire at all.
 *
 * Run with:  pnpm lint:selftest
 */

import { readFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { lintCorpus } from "./index.js";

const FIXTURES = join(fileURLToPath(new URL(".", import.meta.url)), "fixtures");

/** Rules that must fire on fixtures/violations.md, with why the line is there. */
const MUST_FIRE: Array<[rule: string, because: string]> = [
  ["no-dashes", "an em dash and an en dash are both present"],
  ["banned-words", "simply, delve, robust, seamless, comprehensive, and more"],
  ["us-spelling", "behaviour, colours, favourite, organisation, licence"],
  ["no-not-just-but", "'not just wrong, but very wrong'"],
  ["acronym-contract", "API and SDK appear with no expansion"],
  ["tagged-code-fences", "one fence has no language tag"],
  ["danger-annotation", "git reset --hard with no danger: in frontmatter"],
  ["link-resolves", "see_also points at an id that does not exist"],
];

/**
 * Rules that must NOT fire, and the trap each one guards.
 *
 * Every entry here is a false positive that actually happened or that the rule
 * design makes likely.
 */
const MUST_NOT_FIRE: Array<[rule: string, trap: string]> = [
  ["schema", "the fixture's frontmatter is valid; only its prose is broken"],
  ["duplicate-id", "the fixture id is unique"],
  ["id-matches-filename", "violations.md declares id: violations"],
  [
    "tier-depth",
    "tier-fence.md has a heading inside a code fence; a regex splitter would truncate Full and report it as too short",
  ],
  ["tier-structure", "tier-fence.md has both a More and a Full tier"],
];

function main(): void {
  // The fixture links to a deliberately missing id, so link-resolves fires. It
  // also needs its own id known, which lintCorpus handles by collecting ids in
  // pass one.
  const reports = lintCorpus(FIXTURES);

  if (reports.length === 0) {
    console.error("FAIL: the linter reported the violations fixture as clean.");
    process.exit(1);
  }

  const fired = new Set(reports.flatMap((r) => r.findings.map((f) => f.rule)));
  let failures = 0;
  // Counted rather than computed. An earlier hardcoded total silently drifted
  // out of step with the checks the moment one was added.
  let checks = 0;

  for (const [rule, because] of MUST_FIRE) {
    checks++;
    if (fired.has(rule)) {
      console.log(`  ok    ${rule.padEnd(22)} fired (${because})`);
    } else {
      console.error(`  FAIL  ${rule.padEnd(22)} did NOT fire, but ${because}`);
      failures++;
    }
  }

  // The prose rules must reach the frontmatter, not just the body.
  //
  // Error, command, intent, and glossary cards keep nearly all their prose in
  // the YAML header. Checking only bodies left most of the corpus unguarded, and
  // the gap was invisible precisely because a clean report looks identical
  // whether the rules ran or not.
  checks++;
  const frontmatterFindings = reports
    .flatMap((r) => r.findings)
    .filter((f) => f.message.includes("(in frontmatter)"));

  if (frontmatterFindings.length >= 2) {
    console.log(
      `  ok    frontmatter-prose      ${frontmatterFindings.length} findings in YAML header ` +
        `(${[...new Set(frontmatterFindings.map((f) => f.rule))].join(", ")})`,
    );
  } else {
    console.error(
      "  FAIL  frontmatter-prose      the fixture has a banned word and an em dash in its " +
        "frontmatter; prose rules are not reaching it",
    );
    failures++;
  }

  for (const [rule, trap] of MUST_NOT_FIRE) {
    checks++;
    if (!fired.has(rule)) {
      console.log(`  ok    ${rule.padEnd(22)} stayed quiet (${trap})`);
    } else {
      console.error(`  FAIL  ${rule.padEnd(22)} fired but should not: ${trap}`);
      failures++;
    }
  }

  // Prove code is excluded from prose checks.
  //
  // The fixture deliberately puts banned words, a British spelling, and an em
  // dash inside fenced blocks. None of them may be reported.
  //
  // The fence ranges are computed from the file rather than hardcoded as line
  // numbers. An earlier version compared against a fixed line, which silently
  // stopped testing anything the moment the fixture grew.
  checks++;
  const fixture = readFileSync(join(FIXTURES, "violations.md"), "utf8");
  const fenceRanges: Array<[number, number]> = [];
  {
    let open: number | null = null;
    fixture.split("\n").forEach((line, i) => {
      if (!line.trimStart().startsWith("```")) return;
      const lineNo = i + 1;
      if (open === null) open = lineNo;
      else {
        fenceRanges.push([open, lineNo]);
        open = null;
      }
    });
  }

  const inFence = (line: number) => fenceRanges.some(([a, b]) => line > a && line < b);
  const leaked = reports
    .flatMap((r) => r.findings)
    .filter((f) => f.rule !== "tagged-code-fences" && inFence(f.line));

  if (fenceRanges.length < 2) {
    console.error("  FAIL  code-is-excluded      fixture has fewer than two fenced blocks to test with");
    failures++;
  } else if (leaked.length > 0) {
    console.error(
      `  FAIL  code-is-excluded      ${leaked.length} finding(s) landed inside a fenced block: ` +
        leaked.map((f) => `${f.rule}@${f.line}`).join(", "),
    );
    failures++;
  } else {
    console.log(
      `  ok    code-is-excluded      no prose findings inside ${fenceRanges.length} fenced blocks`,
    );
  }

  console.log(
    `\n${checks} checks, ${failures} failure${failures === 1 ? "" : "s"}.`,
  );
  process.exit(failures > 0 ? 1 : 0);
}

main();
