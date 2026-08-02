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

  for (const [rule, because] of MUST_FIRE) {
    if (fired.has(rule)) {
      console.log(`  ok    ${rule.padEnd(22)} fired (${because})`);
    } else {
      console.error(`  FAIL  ${rule.padEnd(22)} did NOT fire, but ${because}`);
      failures++;
    }
  }

  for (const [rule, trap] of MUST_NOT_FIRE) {
    if (!fired.has(rule)) {
      console.log(`  ok    ${rule.padEnd(22)} stayed quiet (${trap})`);
    } else {
      console.error(`  FAIL  ${rule.padEnd(22)} fired but should not: ${trap}`);
      failures++;
    }
  }

  // The tagged fence and the code spans in the fixture exist to prove code is
  // excluded from prose checks. If any finding lands on a line inside the
  // trailing ```text block, the blanking is broken.
  const findings = reports.flatMap((r) => r.findings);
  const codeBlockLines = findings.filter((f) => f.rule !== "tagged-code-fences" && f.line >= 45);
  if (codeBlockLines.length > 0) {
    console.error(
      `  FAIL  code-is-excluded      ${codeBlockLines.length} finding(s) landed inside a fenced block: ` +
        codeBlockLines.map((f) => `${f.rule}@${f.line}`).join(", "),
    );
    failures++;
  } else {
    console.log("  ok    code-is-excluded      no prose findings inside fenced code");
  }

  console.log(
    `\n${MUST_FIRE.length + MUST_NOT_FIRE.length + 1} checks, ${failures} failure${failures === 1 ? "" : "s"}.`,
  );
  process.exit(failures > 0 ? 1 : 0);
}

main();
