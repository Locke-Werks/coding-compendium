---
id: h5-ci-cd
title: CI, and what the green check means
type: section
track: H
order: 50
verified: 2026-08-02
volatility: quarterly
verify: gh run list
answer: >
  Continuous integration is automation that runs your build and your tests on a
  server every time you push. The green check means every step the workflow was
  told to run finished with exit code zero, which is not proof the code is
  correct.
owns:
  - CI
  - CD
  - GitHub Actions
  - reading a failed run
see_also:
  - f3-exit-codes-and-streams
  - d8-pull-requests
  - h1-what-a-test-is
  - g3-lockfiles
  - g6-secrets-and-what-never-to-commit
  - c8-line-endings-and-encoding
keywords:
  - ci failed
  - github actions failed
  - red x on my pull request
  - the checks didnt pass
  - what is ci
  - workflow failed
  - the build failed on github
  - passes locally fails on github
---

## More

CI (Continuous Integration) is automation that builds your project and runs your tests on
somebody else's computer every time you push. GitHub's version is called Actions. The check
mark you see on a pull request is its verdict.

That verdict is narrower than it looks. Green means every step the workflow was told to run
finished with an exit code of zero ([f3](#f3-exit-codes-and-streams)). It does not mean the
code is correct, and a workflow that runs no tests at all goes green forever. What it
genuinely proves is that your project installs and runs on a clean machine that has none of
your local setup, which is worth a lot on its own.

CD (Continuous Delivery) is the step after: taking code that passed and shipping it, either
all the way to users or up to a one-click button. Written together they are CI/CD. You will
meet CI early and may never set up CD.

The configuration lives in your repository at `.github/workflows/`, as one or more files
ending `.yml`.

When the check goes red, in order:

1. Click the red mark on the pull request, then **Details**.
2. Find the job with the red mark and open it. The failed step is expanded already.
3. Read upward from the bottom of that step, past `Process completed with exit code 1`,
   which is the result rather than the reason.
4. If the log is long, search it for `Error`, `FAIL`, or `npm ERR!`.

From the terminal, which is faster:

```powershell
gh run view --log-failed
```

Prints the log of only the steps that failed, for the most recent run. `gh run list` shows
recent runs if you need a different one.

Most red checks on a project that works locally come from five causes, and the first is the
most common: a file you never committed. The server clones your repository and nothing else
exists there.

## Full

### Reading a workflow file

```yaml
name: tests
on: [push]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 22
      - run: npm ci
      - run: npm test
```

The format is YAML (YAML Ain't Markup Language), where indentation carries the structure
and a stray space is a syntax error ([j2](#j2-the-config-formats-nobody-explains)).

- `on: [push]` is the trigger. Every push to any branch runs this.
- `runs-on: ubuntu-latest` is a fresh Linux machine, created for this run and destroyed
  afterward. Not Windows, which matters below.
- `uses:` pulls in a prewritten action. `checkout` copies your repository onto the machine,
  and `setup-node` installs Node.
- `run:` is a command in a terminal, exactly as you would type it.

The last two lines are the entire test. Everything above exists to get a machine into a
state where those two commands can run.

That structure explains the whole "works on my machine" category. The runner starts from
nothing, clones what is committed, and installs from your lockfile. Anything living only on
your disk does not exist there, which is why `npm ci` rather than `npm install` is the right
command in that file ([g3](#g3-lockfiles)).

### The five causes of "passes locally, fails on the server"

1. **A file you did not commit.** Your import works because the file is on your disk. Run
   `git status` and look for anything untracked that your code needs.
2. **An environment variable that only exists in your shell.**
   ([g5](#g5-environment-variables)). Real credentials go in the repository's secrets, never
   in the workflow file.
3. **Filename capitalization.** Windows treats `Button.tsx` and `button.tsx` as the same
   file and the Linux runner does not, which is the most common Windows-specific cause by a
   wide margin ([i1](#i1-what-deployment-means) has the full list of machine differences).
4. **A stale lockfile.** The manifest and lockfile disagree, so `npm ci` refuses outright.
   Run the install locally and commit the result.
5. **Line endings.** A test comparing exact file contents can fail when the checkout
   converts them ([c8](#c8-line-endings-and-encoding)).

Notice that none of these are bugs in your code. That is normal for red checks in the first
weeks of a project.

### Finding the real error inside a long log

The last line of a failed job is almost never the reason. `Process completed with exit code
1` means the step failed, which you already knew.

Work like this: find the first step marked red, not the last. Inside it, read from the
bottom up for the first line that says something specific
([f1](#f1-how-to-read-an-error-message) applies unchanged here). If the log is thousands of
lines, use the search box on the page or pipe the terminal output:

```powershell
gh run view --log-failed | Select-String -Pattern "Error|FAIL|error:"
```

`Select-String` is PowerShell's text search. Case matters less than you would think, and
casting a wide net is right here, because you are looking for where to start reading.

### Secrets in a workflow

A workflow needs credentials for anything real: deploying, calling a paid service, pushing
a package. Those go in the repository's settings under secrets, and the workflow refers to
them by name. GitHub masks the value in the log, showing `***` where it appears.

The masking is a courtesy, not a guarantee. A command that prints a secret in a different
encoding defeats it, and logs on a public repository are public. Never echo a secret to
check whether it is set; check whether the command that needs it succeeded
([g6](#g6-secrets-and-what-never-to-commit)).

### What green actually proves

Read the workflow file once, and you know exactly what the check mark is promising. If the
steps are install and test, green means your tests passed on a clean machine. If the only
step is a build, green means it compiled, and every test could be failing.

This is worth checking on a project an agent set up for you, because a workflow that looks
thorough and runs nothing is easy to produce and reads as reassuring
([h6](#h6-when-tests-lie)).

### Cost, time, and the honest limits

Public repositories run free. Private ones get a monthly allowance of minutes, and a slow
suite on every push can exhaust it. Runs take a minute or two to start, so the feedback is
not instant, and a suite that takes fifteen minutes will change how often you push.

Actions changes its syntax and its action versions regularly, so treat the workflow file
you have as the working example and GitHub's documentation as the source of truth when
something behaves differently.

### Continuous delivery, briefly

Once the tests pass, a further job can deploy: push a container, upload a build, publish a
release. The one rule worth stating now is that the deploy job must depend on the test job,
so nothing ships from a red run. [i1](#i1-what-deployment-means) covers what deploying
means, and until you have something deployed, CI on its own is the part that pays.
