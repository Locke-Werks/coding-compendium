---
id: g7-dependency-risk
title: Dependency risk, in proportion
type: section
track: G
order: 70
verified: 2026-08-02
volatility: quarterly
verify: npm view express version
answer: >
  Three dependency risks are worth your attention: a name one keystroke off a
  real package, a library nobody has maintained in years, and a package your
  agent invented that somebody has since registered. Check the name, the last
  publish date, and the repository link.
owns:
  - supply chain risk
  - typosquatting
  - audit tooling
  - hallucinated packages
see_also:
  - e7-agent-failure-modes
  - g2-package-managers
  - g1-what-a-dependency-is
  - e11-what-to-never-let-an-agent-do
  - g3-lockfiles
keywords:
  - is this package safe
  - typosquatting
  - the package doesnt exist
  - did the ai invent this library
  - npm audit
  - abandoned library
  - slopsquatting
  - is this library maintained
---

## More

You are installing code written by strangers, and the honest framing is that this is
normal, unavoidable, and mostly fine. Three risks are worth actual attention. Everything
else is a threat model for companies shipping to millions of people.

**A typosquatted name.** Somebody publishes `reqeusts` next to `requests`, or `crossenv`
next to `cross-env`, and waits for a typo. The install succeeds, the code runs, and the
package quietly does something extra on the side. The defense is reading the name character
by character against whatever documentation told you to install it.

**An abandoned library.** Last published four years ago, ninety open issues, no replies.
Nobody is going to attack it. It is going to break on the next version of the language and
stay broken, and you will be the one deciding what to do about it at the worst moment.

**A package the agent invented.** Agents produce plausible package names that were never
published ([e7](#e7-agent-failure-modes)). Most of the time you get an error, which is
harmless. The dangerous version is that attackers watch for the names agents commonly
invent and register them, so a package that did not exist last month exists now, has your
agent's favorite name, and is not what anyone intended you to install.

The check before installing something you have not heard of takes about ninety seconds:

1. Open the package's page on the registry.
2. Check the last publish date.
3. Check the weekly download count.
4. Click the repository link and confirm it goes somewhere real.
5. Compare the name, character by character, with the documentation that named it.

A package with millions of weekly downloads, a release this year, and a repository full of
recent activity is as safe as anything you install. That is most of what you will use.

## Full

### Doing the check from the terminal

```powershell
npm view express
```

Prints the current version, the publish date, the homepage, the repository, the license,
and the maintainer list, without installing anything. The date and the repository link are
the two fields that earn their keep.

For Python, `pip index versions <name>` lists published versions, and the package's page on
PyPI shows the rest. For Rust, the crates.io page for the package shows downloads, recent
versions, and the repository.

What you are reading, in order of how much it tells you:

- **The name matches the documentation exactly.** This catches the whole typosquat
  category. Nothing else you check matters if this fails.
- **The repository link works and looks like a real project.** A package with no repository
  link at all deserves a second look.
- **Something was published in the last year or two.** Older is not automatically bad for a
  small stable library, and it is a bad sign for anything touching security or a
  fast-moving framework.
- **Download counts are in the thousands or better.** A brand-new package with forty
  downloads and a name that sounds perfect for your use case is the exact shape of the
  problem.

### The agent-specific one, named

The failure has a name people have started using: slopsquatting. The agent suggests
`import fastapi_auth_helper`, a name that sounds like it should exist. Sometimes it does
not. Sometimes an attacker noticed the same pattern and published it first.

One rule handles it: install a package because documentation named it, not because an agent
named it. When an agent hands you an install command for something unfamiliar, ask which
registry page it comes from and check that page yourself. An agent that invented the name
will happily invent a plausible link too, so follow the link rather than reading it.

The tell that you are in this situation is an install command that appears in generated
prose rather than in a real project's setup instructions.

### npm audit, and how seriously to take it

```powershell
npm audit
```

Compares your installed tree against a public database of reported vulnerabilities and
prints a count by severity. The equivalents are `pip-audit` for Python and `cargo audit`
for Rust. On GitHub, the same information arrives as Dependabot alerts on the repository.

Reading the output honestly:

- `14 vulnerabilities (3 high)` means somebody filed a report about a version somewhere in
  your tree. It does not mean your app is exploitable, and often the affected code path is
  one your project never calls.
- Most findings are in **development dependencies**: build tools, test runners, and linters
  that never run anywhere but your machine. Those matter far less than anything that ships.
- A CVE (Common Vulnerabilities and Exposures) number is a catalog entry, and the number
  itself carries no information about whether it affects you.

```powershell
npm audit fix
```

Applies updates that stay inside your allowed version ranges. Usually uneventful. The
`--force` variant upgrades across major versions, changes behavior, and breaks working
projects, so it is one of the commands not to leave to an agent unsupervised
([e11](#e11-what-to-never-let-an-agent-do)). Run your tests after either one
([h1](#h1-what-a-test-is)).

### The one mechanism worth knowing

Packages in some ecosystems, npm especially, can run a script at install time. That is why
a malicious package is dangerous the moment you install it rather than the moment your code
imports it, and it is why "I installed it but never used it" is not the reassurance it
sounds like.

You do not need to change how you work because of this. It is the reason the ninety-second
check happens before the install rather than after.

### What genuinely is not worth worrying about

- **The size of the tree.** Four hundred transitive packages is ordinary
  ([g1](#g1-what-a-dependency-is)).
- **Unmaintained packages deep in the tree.** You did not choose them and you cannot
  replace them. Your direct dependencies are the ones you control.
- **Reading dependency source code.** Nobody does this and you will not be the exception.
- **Advisories in build tooling with no exploit path.** Note them, fix them when convenient.

### Where the risk actually concentrates

Your direct dependencies, chosen one at a time, usually by an agent, in the middle of doing
something else. That is the moment the decision gets made, and it is the only place a
ninety-second check fits. Adding a dependency to save ten lines of code is where most of
the avoidable risk enters a project, and the alternative is writing the ten lines.
