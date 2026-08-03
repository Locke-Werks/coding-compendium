---
id: e11-what-to-never-let-an-agent-do
title: What to never let an agent do unsupervised
type: section
track: E
order: 110
verified: 2026-08-02
volatility: low
danger: >
  This card names commands that destroy work permanently. A force push replaces
  the branch on GitHub and anything there that is not in your local history is
  gone. `git branch -D` deletes unmerged work. A recursive delete removes
  untracked files that git never had a copy of. Every one of them is listed here
  as something you run yourself, deliberately, after reading what it will do.
  The safe alternative to each is in the table under Full.
answer: >
  Let an agent do anything you could undo in five minutes, and keep force push,
  history rewrites, branch deletion, production changes, credential rotation,
  bulk deletes, and mass dependency upgrades on your side of the keyboard.
owns:
  - the unsupervised blocklist
see_also:
  - d10-undo-everything
  - g7-dependency-risk
  - g6-secrets-and-what-never-to-commit
  - e9-mcp
keywords:
  - force push
  - dont let it
  - dangerous commands
  - auto accept
  - yolo mode
  - it deleted my branch
  - unattended agent
---

## More

One rule sits under the whole list. If the worst case is "I lose ten minutes," let the
agent run. If the worst case is "I cannot get it back," you type it yourself.

Seven things stay on the manual side, each for a specific reason.

**Force push.** `git push --force` replaces the branch on GitHub with your local version.
Anything on the remote that is not in your local history disappears, including work you
pushed from another machine. The reflog that saves you from most git accidents is local, so
it cannot help here ([d11](#d11-when-you-lose-work)).

**Rewriting history that has already been pushed.** Rebase, amend, squash, filter. All of
them mint new commit identifiers, so every reference to the old ones breaks: open pull
requests, links in your issue tracker, anyone else's clone. And the one thing people expect
it to fix, a leaked secret, it does not fix. The old commit stays on GitHub's servers
([g6](#g6-secrets-and-what-never-to-commit)).

**Deleting branches.** `git branch -D` deletes a branch even when its work was never
merged. The lowercase `-d` refuses in exactly that case, which is the check you want. An
agent tidying up old branches is deleting the only copy of something.

**Anything touching production.** Deploying, running a migration, restarting a live
service, changing a domain record. There is no undo, and the blast radius includes people
who are not you.

**Rotating or creating credentials.** A new key silently breaks everything that used the
old one, including the things you forgot depended on it, and the new secret ends up sitting
in a chat transcript ([g8](#g8-what-never-to-paste-into-a-chat)).

**Bulk deletion.** `rm -rf`, `Remove-Item -Recurse -Force`, "clean up the files nobody
uses." It cannot know what is unused, and anything untracked was never in git at all, so
there is nothing to recover ([d10](#d10-undo-everything)).

**Whole-project dependency upgrades.** One command, hundreds of transitive version changes,
a lockfile diff nobody can read, and a build that either works or fails for reasons that
take a day to trace ([g7](#g7-dependency-risk)).

None of this is distrust of the tool. Every item is a place where being wrong is not a
five-minute problem. The agent can propose all seven. You run them.

## Full

### The safe alternative for each

| Keep off the agent | What you do instead |
|---|---|
| `git push --force` | `git push --force-with-lease`, which refuses if the remote moved since you last looked, and only after you have read the log |
| rebase or amend on pushed commits | a new commit that fixes the problem, or `git revert` ([d10](#d10-undo-everything)) |
| `git branch -D` | `git branch -d`, which refuses to delete unmerged work |
| deploy, or run a migration | the agent writes it, you read it, you run it |
| rotate a key | rotate it yourself in the provider's console, and hand the agent nothing |
| delete a folder | move it to a `_trash/` folder, run the build, delete it next week |
| upgrade everything | one dependency at a time, with the tests run after each |

The pattern in the right-hand column: every one of them keeps the irreversible step in your
hands while leaving all the tedious work in the agent's.

### Making it stick, rather than remembering it

Three layers, weakest first:

1. **The instruction file.** A "Never" list in `CLAUDE.md` or `AGENTS.md`
   ([e4](#e4-claude-md-and-agents-md)). It is a suggestion the model usually follows, which
   is worth something and is not a guarantee.
2. **Permissions and sandboxes.** Claude Code's settings can deny specific commands
   outright ([b9](#b9-where-settings-live)). Codex's `sandbox_mode` limits what it can
   touch and `approval_policy` controls when it stops to ask
   ([b7](#b7-install-codex)). This is the layer that actually holds, because software
   enforces it.
3. **Being in the room.** No configuration substitutes for a human who reads the command
   before pressing yes.

Set layer two before you ever need layer three. It takes ten minutes once.

### Auto-accept, stated plainly

Auto-accept mode is genuinely good. Edit, test, edit, test, twenty times without a prompt,
which is exactly the loop you want when the work is going well.

It is also the mode in which every item on this list becomes possible with nobody watching.
The workable split: auto-accept file edits inside the project folder, keep a prompt on
shell commands, and deny the destructive ones by rule so the question never reaches you at
eleven at night when you are approving everything by reflex.

If you run an agent fully unattended, do the deny rules first. Not afterward, and not
"once I have seen how it behaves."

### The two-question test for anything not on the list

This list is not complete and cannot be. For anything new, ask:

1. **If this goes wrong, what is the recovery, and how long does it take?**
2. **Does anyone other than me feel it?**

An action with a known recovery under five minutes that affects nobody else is safe to
delegate. Anything else moves to your side of the keyboard. That test will still be correct
when the models are twice as good, because it is a rule about recovery rather than about
capability.

### If it already happened

In order, calmly:

1. **Stop.** Do not run another command hoping it helps. The most common way a recoverable
   mistake becomes permanent is the next three commands.
2. **Check the reflog.** Almost anything that was ever committed is recoverable, and
   [d11](#d11-when-you-lose-work) is the card that walks you through finding it.
3. **If a secret was pushed, rotate it now** and assume it is compromised. Deleting it from
   history does not remove it from GitHub
   ([g6](#g6-secrets-and-what-never-to-commit)).
4. **If files were deleted and had never been committed, they are gone.** Recover from a
   backup or accept the loss. This is the whole argument for committing early and often
   ([d4](#d4-commit-well)).
