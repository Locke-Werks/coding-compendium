---
id: agent-authentication-error
title: "API Error: 401 authentication_error"
type: error
verified: 2026-08-02
volatility: weekly

category: permission

# Prints True or False without revealing the value. If it prints True and you
# sign in with a subscription, that variable is your problem.
verify: '[bool]$env:ANTHROPIC_API_KEY'

sample: |
  PS C:\Users\nyx\dev\site> claude
  API Error: 401 {"type":"error","error":{"type":"authentication_error","message":"invalid x-api-key"}}

patterns:
  - "authentication_error"
  - "invalid x-api-key"
  - "Invalid API key"
  - "401"
  - "Unauthorized"
  - "OAuth token has expired"

means: >
  The service does not accept whoever this request says it is. Either no credential was sent, or
  the one that was sent is expired, revoked, or belongs to a different account. The request never
  reached a model. Nothing about your project, your files, or your command is involved.

fix_ladder:
  - try: Sign in again from inside the tool.
    command: /login
    shell: powershell
    why: >
      Assumes a sign-in that expired, which happens on a schedule and after a password change.
      Type this inside a running Claude Code session and it opens a browser to reauthenticate.
      Codex has its own sign-in command and prompts on startup.

  - try: Check whether a leftover API key is overriding your subscription.
    command: '[bool]$env:ANTHROPIC_API_KEY'
    shell: powershell
    why: >
      Assumes an environment variable is winning. A key set months ago for a script takes
      precedence over your interactive sign-in, so the tool sends a dead key and never uses the
      account you are signed in to. `True` here on a subscription plan is very likely your
      answer.

  - try: Remove the variable for this session and try again.
    command: Remove-Item Env:ANTHROPIC_API_KEY
    shell: powershell
    why: >
      Assumes the previous step printed `True`. This clears it for this terminal only, which is a
      safe test. If the tool works afterward, remove it permanently under Windows settings,
      "Edit environment variables for your account".

  - try: Check the key itself if you sign in with one.
    why: >
      Assumes a real key problem. Sign in to the provider's console and confirm the key exists,
      is not revoked, and belongs to the account with credit. Keys can be deleted by anyone with
      console access, and a deleted key fails exactly this way.

  - try: Look for a proxy or base address override.
    command: 'Get-ChildItem Env: | Where-Object Name -like "*ANTHROPIC*"'
    shell: powershell
    why: >
      Assumes something is redirecting requests. A leftover `ANTHROPIC_BASE_URL` from a tutorial
      sends your requests somewhere that does not know your credential. This lists every related
      variable and shows values, so read it on your own screen and do not paste the output
      anywhere.

if_none_worked: >
  Paste the error with the `type` field intact, say whether you sign in with a subscription or an
  API key, and say whether it worked yesterday. Never paste the key itself, or the output of any
  command that prints it. The `type` field is what people trim, and it separates a bad credential
  from a missing one.

see_also:
  - b6-install-claude-code
  - b7-install-codex
  - g5-environment-variables
  - g8-what-never-to-paste-into-a-chat

keywords:
  - 401 authentication error
  - invalid x-api-key
  - api key not working
  - claude login expired
  - unauthorized agent
---

The environment variable case deserves the top spot, because it produces the most confusing
version of this. You are signed in. The tool says you are signed in. It still fails, because a
variable set for something else quietly outranks the sign-in.

Check for that before reinstalling anything. `[bool]$env:ANTHROPIC_API_KEY` answers it without
printing the key.

While you are here, one rule about the key itself: it is a password. It goes in an environment
variable or a credential store, never in a file inside the repository, never in a commit, and
never pasted into a chat window to prove what it looks like. A key that has been pasted anywhere
should be rotated, which means deleting it in the console and creating a new one.

These sign-in flows change often. Both tools ship updates weekly and the exact command names move
with them, so the tool's own help output is the source of truth over anything written down.
