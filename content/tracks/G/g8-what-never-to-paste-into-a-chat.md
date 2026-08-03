---
id: g8-what-never-to-paste-into-a-chat
title: What never to paste into any AI chat
type: section
track: G
order: 80
verified: 2026-08-02
volatility: low
answer: >
  Never paste keys, tokens, passwords, connection strings, private key files,
  the contents of .env, or other people's personal data. Replace the value with
  a placeholder and keep everything around it; the help you get is identical.
owns:
  - redaction before sending
  - the paste blocklist
see_also:
  - f5-what-to-paste-and-what-not-to
  - g6-secrets-and-what-never-to-commit
  - g5-environment-variables
  - e9-mcp
  - e5-prompting-that-works
keywords:
  - what shouldnt i paste
  - is it safe to paste my env file
  - can they see my api key
  - redact before sending
  - privacy when pasting code
  - i pasted my key into chatgpt
  - screenshot leaked a token
---

## More

The blocklist. Nothing on it goes into a chat with an AI (Artificial Intelligence) tool,
yours or anyone else's, in any window, ever.

- API (Application Programming Interface) keys, access tokens, and passwords, including
  your own
- Database connection strings, because the password is inside them
- Private key files: `.pem`, `.p12`, `.pfx`, `id_rsa`, and anything called a certificate
- The contents of `.env` ([g5](#g5-environment-variables))
- Other people's data: names, email addresses, order records, anything medical or financial
- Session cookies and authorization headers copied out of a browser
- Anything covered by an agreement you signed about somebody else's information

The reason is mechanical rather than moral. Text you paste leaves your machine. It reaches
a server, it may be retained, it is usually written to a session transcript on your own
disk, and the agent can then copy it into a file or a commit while doing something else you
asked for. A secret you controlled becomes a secret in four more places.

Redaction is easy and costs you nothing, because the agent needs the shape of the value,
not the value:

```text
DATABASE_URL=postgres://appuser:<redacted>@db.internal:5432/orders
Authorization: Bearer <redacted-token>
```

Keep the key name, the format, the host, the port, the length if it matters. Replace the
part that grants access. You will get exactly the same answer you would have gotten with
the real string, because nothing in the diagnosis depended on it.

Two leaks that happen without a deliberate paste: an error message with a connection string
inside it, and a screenshot of a terminal whose scrollback holds the key you exported ten
minutes ago.

If you have already pasted one, treat it like a commit. Rotate it now
([g6](#g6-secrets-and-what-never-to-commit)).

## Full

### Redacting a real one, start to finish

You have an error and it contains the whole credential:

```text
psycopg2.OperationalError: connection to server at "db.internal" (10.0.4.12),
port 5432 failed: FATAL: password authentication failed for user "appuser"
  connection string: postgres://appuser:hunter2ButLonger@db.internal:5432/orders
```

Paste this instead:

```text
psycopg2.OperationalError: connection to server at "db.internal" (10.0.4.12),
port 5432 failed: FATAL: password authentication failed for user "appuser"
  connection string: postgres://appuser:<redacted>@db.internal:5432/orders
```

Everything diagnostic survives: the library, the error class, the host, the port, the user,
and the fact that authentication is what failed. One field is gone and it was the field that
could not help.

The same move works for a stack trace with a token in a header, a config file with three
real values, and a log line with a customer's email in it. Replace values, keep structure,
mark what you replaced so the agent does not treat `<redacted>` as the actual password and
send you off debugging a string that is not there.

Angle brackets are the convention, and they are worth using because they are obviously not
a real value. `xxxx` and `1234` have both been read as literal by agents that then wrote
them into a config file.

### What is completely fine to paste

Your source code, in almost every case. The error message, whole and unedited, after the
one redaction pass. Config files with values replaced. A database schema, which is
structure rather than data. A file listing. Command output, once you have looked at it.

[f5](#f5-what-to-paste-and-what-not-to) is the card on getting this right in the other
direction, because the more common failure by far is pasting too little and getting a guess
back. Redact the credential, then paste everything.

### Where the text actually goes

Three places, and the third one surprises people.

**A server.** Every hosted agent sends your prompt somewhere to be processed. Whether it is
retained, for how long, and whether a human may look at it depends on the plan you are on.
Business and enterprise plans usually promise more than consumer ones.

**A transcript on your disk.** Claude Code and Codex both keep session history in your user
folder so they can resume conversations. Anything you pasted is sitting in a file on your
machine, in plain text, indefinitely, and it will be picked up by anything that reads that
folder.

**Wherever the agent puts it next.** This is the one worth internalizing. An agent that has
your key in its context will use it: writing it into a config file to test something,
including it in a commit, printing it in a summary. It is being helpful with the material
you gave it.

Connected tools widen the last one further, because a tool with access to your files or
services can carry the value somewhere you did not have in mind. [e9](#e9-mcp) covers what
granting that access means.

### Other people's data

The rules here are not about your risk. If your project holds customer records, user
emails, or anything medical or financial, you probably do not have permission to send it to
a third party, whatever your own comfort level. This is the one item on the blocklist that
can create a legal problem rather than a security problem.

Use fabricated rows. Three invented customers with obviously fake names debug a query
exactly as well as three real ones, and you can paste them without a second thought.

### Screenshots and terminal scrollback

A screenshot of an error is convenient and it captures the whole window. The window often
contains a prompt with a project path, an earlier command where you set a variable, or a
tab showing a config file. Crop tightly, or copy the text instead, which is more useful to
the agent anyway.

The same applies to pasting a long scrollback. Scan it before it goes in.

### After a paste you regret

Same procedure as a committed secret, and the same order.

1. Rotate the credential at the service. Now, not later.
2. Confirm the new one works.
3. Delete the conversation if the tool lets you, understanding that this reduces exposure
   rather than ending it.

Deleting the chat is the equivalent of deleting the file in a new commit: sensible
housekeeping, not a fix. The credential is the thing that has to change
([g6](#g6-secrets-and-what-never-to-commit)).
