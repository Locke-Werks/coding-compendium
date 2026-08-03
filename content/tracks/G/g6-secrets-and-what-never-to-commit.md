---
id: g6-secrets-and-what-never-to-commit
title: Secrets, and what must never reach GitHub
type: section
track: G
order: 60
verified: 2026-08-02
volatility: low
verify: git diff --staged
danger: >
  Cleaning a secret out of git history rewrites every commit, changes every
  hash, and requires replacing what GitHub already has, which breaks every other
  copy of the repository. It is also the second step, never the first. Rotating
  the credential is the step that removes the risk, and it destroys nothing.
  The full procedure with its costs is in the panic tree linked below.
answer: >
  A secret is any value that grants access, and deleting one in a later commit
  does not remove it from git history. Rotate the credential immediately and
  assume it is compromised from the moment it was pushed.
owns:
  - API keys
  - credential hygiene
  - what to do after leaking one
see_also:
  - d12-gitignore-and-what-not-to-commit
  - g8-what-never-to-paste-into-a-chat
  - g5-environment-variables
  - panic-committed-a-secret
  - d1-what-git-actually-stores
  - b5-ssh-vs-https
keywords:
  - i committed my api key
  - secret in git history
  - remove a password from a commit
  - leaked token
  - rotate a credential
  - push protection
  - my env file got committed
  - is my key still in the repo
---

## More

A **secret** is any value that grants access to something: an API (Application Programming
Interface) key, a password, a token, a database connection string, a private key file. If
possessing the string is enough to act as you, it is a secret.

They never go in the code, never go in a commit, and never go in a chat window
([g8](#g8-what-never-to-paste-into-a-chat)). They go in environment variables or a `.env`
file that git ignores, which is [g5](#g5-environment-variables) for the mechanism and
[d12](#d12-gitignore-and-what-not-to-commit) for the ignore rules.

Now the part almost everyone gets wrong, and it is the most expensive mistake in this
track.

**Deleting a committed secret in a new commit does not remove it.** Git stores every
version of every file ([d1](#d1-what-git-actually-stores)). The old commit still holds the
key, anyone with a copy of the repository can print it with one command, and if it reached
GitHub then it reached everyone who cared to look.

So the order is fixed, and step one is not a git command:

1. **Rotate the credential.** Open the service, issue a new key, switch the old one off.
   Before you touch git.
2. Put the new value somewhere ignored and confirm the project still works.
3. Take the old value out of the working file and make sure an ignore rule now covers it.
4. Clean the history, if the repository is public or shared. Real work, real cost, and it
   is step four for a reason. [The panic tree](#panic-committed-a-secret) walks it.
5. Assume the old value was read by somebody.

Step 5 is not paranoia. Bots watch public repositories continuously and pull credentials
within minutes of a push. Cleaning history limits future exposure and does nothing about
exposure that already happened. Rotation is the step that ends the risk, and it usually
takes five minutes.

## Full

### What counts as a secret

| Kind | Looks like | What it costs you |
|---|---|---|
| API key | `sk-...`, `AIza...`, a long random string | somebody else's usage on your bill |
| Access token | `ghp_...`, a long signed string in three dotted parts | actions taken as you |
| Connection string | `postgres://user:password@host/db` | your database, read and written |
| Private key file | `.pem`, `.p12`, `.pfx`, `id_rsa` | your identity, until you replace it |
| Service account file | a `.json` file from a cloud provider | often the whole cloud account |
| Session cookie | a long value copied from a browser | somebody logged in as you |
| Webhook address | a normal-looking address with a token in the path | messages sent as your app |

Two that get missed constantly: a private key has no file extension convention you can rely
on, and a webhook address looks like an ordinary link, so neither trips the instinct that a
password would.

### How they get committed anyway

Nobody types a key into a file on purpose. Four routes cover nearly every case.

- **`git add .` swept it up.** The `.env` was not ignored yet, or it was named
  `.env.local.backup` and the rule did not match.
- **It was hardcoded during debugging.** Pasted into the source to check something, then
  forgotten under twenty other changes.
- **An agent wrote a config file** with your real value in it, taken from a place it had
  already read.
- **A log or test fixture captured it.** The value was in the environment, the program
  printed it, and the output got committed as a sample.

The check that catches all four takes five seconds and belongs in the habit right before
committing:

```powershell
git diff --staged
```

That is exactly what is about to be recorded ([d9](#d9-reading-a-diff)). Read the file list
first: an unexpected `.env`, `.pem`, or `credentials.json` in that list is the whole
warning you need.

### After a leak, in order

**Rotate first.** Every service has a page for this, usually called API keys, credentials,
or tokens. Create the new one, update your `.env`, confirm the project works, then revoke
the old one. If the service has no revoke button, that is your answer about how much to
trust it.

**Then decide about history.** Removing the value from every past commit means rewriting
those commits, which changes every hash in the repository and forces you to replace what
GitHub holds. Worth doing for a public repository. Rarely worth doing for a private one you
alone use, once the credential is already dead.
[The panic tree](#panic-committed-a-secret) has the routes with their costs stated.

**Know what cleaning cannot reach.** Anyone who already cloned the repository keeps the old
commits. Any fork keeps its own copy, and rewriting yours does not touch theirs. GitHub can
keep a rewritten commit reachable by its hash for a while afterward, and you have to ask
their support to expire it. All three are reasons the first step is rotation.

### Push protection, and what it does not cover

GitHub scans pushes for recognized credential formats and blocks the push when it finds
one. If you have seen a message about push protection, that is the best possible version of
this problem: the value never became public. Rotate anyway if it was ever a real
credential, then take it out of the commit.

The scanner only knows patterns that services registered with it. Your own application's
signing key, a customer's password, or a connection string to a database you run yourself
will sail straight through. Push protection is a backstop, not a review.

### Rating your own leak

Not every leak is an emergency, and knowing which is which keeps you from panicking at the
wrong moment.

- **Read-only key for a free public service.** Rotate it this week.
- **Paid API key.** Rotate now. Usage is billed to you and stolen keys get resold.
- **Database connection string.** Rotate now, then look at the data. Somebody may have read
  it, and depending on what is in there, that may be something you have to tell people
  about.
- **Cloud provider credentials.** Rotate now and check the account's activity log. This is
  the one with a bill attached that makes the news.
- **Signing key or private key.** Rotate and reissue. Anything signed with it in the
  meantime is suspect.

### Preventing the next one

Three habits, in order of how much they buy you.

Keep every secret in one ignored file per project, always named `.env`, so a single ignore
rule covers all of them and there is no second place to check. Commit a `.env.example`
alongside it so the shape of the file stays visible without the values
([g5](#g5-environment-variables)).

Read `git diff --staged` before you commit. It takes seconds and it catches the case where
the ignore rule was wrong.

Tell your agent the rule in the instruction file it reads every session: never open `.env`,
never print a credential, never inline a value from the environment into a source file
([e4](#e4-claude-md-and-agents-md)). Agents follow standing instructions like this
reliably, and it removes the most common route in.
