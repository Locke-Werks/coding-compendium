---
id: agent-rate-limit-exceeded
title: "API Error: 429 rate limit exceeded"
type: error
verified: 2026-08-02
volatility: weekly

category: network

sample: |
  PS C:\Users\you\dev\site> claude
  API Error: 429 {"type":"error","error":{"type":"rate_limit_error","message":"This request would exceed your organization's rate limit of 40,000 input tokens per minute. For details, refer to: https://docs.anthropic.com/en/api/rate-limits."}}

patterns:
  - "rate_limit_error"
  - "429"
  - "Too Many Requests"
  - "usage limit reached"
  - "exceeded your current quota"
  - "insufficient_quota"

means: >
  The service refused the request because of how much you have used, not because of anything
  wrong with the request. Three different limits produce similar messages: a per-minute cap on
  how fast you can send, a longer allowance for your plan that refills on a schedule, and running
  out of paid credit entirely. Nothing on your machine is broken and your code is untouched.

fix_ladder:
  - try: Wait a minute and send the same message again.
    why: >
      Assumes a per-minute cap, which is the most common of the three and refills continuously. If
      the message mentions "per minute" or "tokens per minute", waiting genuinely fixes it. A
      long session that suddenly hits this after working fine is usually here.

  - try: Read which of the three limits it is.
    why: >
      Assumes the wording will tell you, which it does. "rate limit" with a per-minute number
      means slow down. "usage limit reached" with a reset time means your plan's allowance is
      spent until that time. "insufficient_quota" or anything mentioning billing means the account
      is out of credit, and waiting will never help.

  - try: Make the next request smaller.
    why: >
      Assumes you are hitting a tokens-per-minute cap with a large context. Everything in the
      session counts on every message, so a conversation carrying twenty files re-sends them each
      turn. In Claude Code, `/compact` shrinks the conversation. Attaching fewer files does the
      same thing more directly.

  - try: Switch to a smaller model for a while.
    why: >
      Assumes limits are tracked per model, which they usually are. A smaller model has its own
      allowance and often a larger one. This is a good trade for mechanical work such as renaming
      or writing tests, and a poor one for design decisions.

  - try: Check the account rather than the request.
    why: >
      Assumes credit or billing. Sign in to the provider's console and look at the usage page. An
      expired card or a spend cap you set months ago produces this error permanently, and no
      amount of waiting or compacting changes it.

if_none_worked: >
  Paste the whole error including the JSON (JavaScript Object Notation) block, especially the
  `type` field inside it and any reset time. People trim the block because it looks like machine
  noise, and it is the only thing that distinguishes "wait sixty seconds" from "your card
  expired". Never paste your API key alongside it.

see_also:
  - e8-tokens-and-cost
  - e2-context-windows
  - e6-when-to-reset-context
  - g8-what-never-to-paste-into-a-chat

keywords:
  - "429"
  - rate limit
  - usage limit reached
  - quota exceeded
  - too many requests
---

Three different problems wearing similar clothes. Telling them apart takes five seconds and
decides whether you wait, change what you send, or open a billing page.

A per-minute rate limit is about speed. You sent too much too quickly, often because the agent
made several large requests back to back while reading files. It clears on its own.

A plan usage limit is about the period. Subscription tools give you an allowance per window and
tell you when it resets. Nothing you change in the request will help before then.

Running out of credit is about money and looks nothing like the other two once you read the
message. It says quota or billing rather than rate.

There is a fourth message worth recognizing in the same family. A 529 or "Overloaded" means the
service itself is busy rather than you. Retrying in a minute is the whole response, and the
provider's status page will say if it is widespread.

These limits and their wording change often. The provider's own rate limit documentation is the
source of truth, and it is linked in the error message itself.
