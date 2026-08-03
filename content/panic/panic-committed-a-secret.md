---
id: panic-committed-a-secret
title: I committed a password or an API key
type: panic
verified: 2026-08-02
volatility: quarterly
danger: >
  This tree contains history rewriting. `git filter-repo` changes every commit
  hash in the repository and detaches the remote, and the force push that follows
  replaces the branches on GitHub. One route deletes the entire local history
  with `Remove-Item .git -Recurse -Force`, which is permanent. None of that
  matters more than the first step: rotate the credential. Rewriting history
  limits future exposure and does nothing about exposure that already happened.
symptom: >
  A password, an API key, a token, or a `.env` file went into a commit, and it
  may already be on GitHub.
reassurance: >
  This is fixable and it is common enough that GitHub built automatic detection
  for it. One thing has to be said before anything else, because almost everyone
  gets it wrong: deleting the secret in a new commit does not remove it from the
  history. The old commit still holds it and anyone with the repository can read
  it. Treat the credential as compromised, rotate it now, and clean the history
  afterward. Rotation takes five minutes and it is the step that actually removes
  the risk.
backup_first: git branch backup-now
root: rotate-first
nodes:
  rotate-first:
    ask: >
      Before anything else. Has the leaked credential been replaced with a new
      one and switched off at the service it came from?
    how_to_tell: git log -p -1
    branches:
      - label: Yes, the old one is dead
        goto: pushed
      - label: Not yet
        goto: rotate-now
      - label: I am not sure which service it belongs to
        goto: find-it

  find-it:
    ask: Find every checkpoint the secret appears in, so you know what it is and how far back it goes.
    resolve:
      command: git log --oneline -p -S "<the-first-eight-characters-of-the-key>"
      shell: powershell
      does: >
        Searches the whole history for checkpoints where that text was added or
        removed and prints the change each time. The output names the file, the
        checkpoint, and usually the service.
      destroys: Nothing. It only reads.
      verify: >
        You can name the service that issued the credential. Go rotate it, then
        come back.
      if_it_did_not_work: >
        If nothing prints, the text may not match exactly. Try a shorter
        fragment, or look at the file directly with `git log -p -- <path/to/file>`.

  rotate-now:
    ask: >
      Rotate it now. Open the service, create a new credential, put the new value
      somewhere git ignores, and switch the old one off.
    resolve:
      command: git log -p -1
      shell: powershell
      does: >
        Prints exactly what your last checkpoint added, so you can read the leaked
        value and work out which service issued it. Then go to that service and
        replace it.
      destroys: Nothing. It only reads.
      verify: >
        The old credential no longer works. Test it if the service gives you a
        way to.
      if_it_did_not_work: >
        If the secret went in an older checkpoint, `git log -p -S "<fragment>"`
        finds it. Do not skip this step. Everything below limits future exposure
        only, and rotation is the only thing that fixes exposure that already
        happened.

  pushed:
    ask: Has the checkpoint containing the secret been sent to GitHub?
    how_to_tell: git status -sb
    branches:
      - label: No, it is only on my machine
        goto: local-only
      - label: Yes, it is on GitHub
        goto: on-github
      - label: I do not know
        goto: check-pushed

  check-pushed:
    ask: Ask git what GitHub already has.
    resolve:
      command: |
        git fetch
        git status -sb
      shell: powershell
      does: >
        Downloads GitHub's current state without touching your files, then prints
        one summary line. `ahead 1` means one checkpoint has not been sent. No
        mention of ahead means GitHub has everything.
      destroys: Nothing. `fetch` only downloads.
      verify: You have your answer. Go back and answer the previous question.
      if_it_did_not_work: >
        If it reports no upstream, this branch was never pushed. Answer no to the
        previous question.

  local-only:
    ask: Is the checkpoint containing the secret the most recent one?
    how_to_tell: git log --oneline -5
    branches:
      - label: Yes, it is the top line
        goto: fix-last-commit
      - label: No, it is further back
        goto: rewrite-local

  fix-last-commit:
    ask: Undo the checkpoint, take the secret out, and save it again.
    resolve:
      command: git reset --soft HEAD~1
      shell: powershell
      does: >
        Removes the last checkpoint and puts everything it contained back in the
        staging area. Now delete the secret from the file, add the file to
        `.gitignore`, unstage it with `git restore --staged <the-file>`, and
        commit again.
      destroys: Nothing. Every change is kept.
      verify: >
        `git log -p -1` no longer shows the secret anywhere in its output, and
        `git status` no longer lists the secret file at all.
      if_it_did_not_work: >
        If the value also appears in older checkpoints, this is not enough. Treat
        it as the further-back case and rewrite the history instead.

  rewrite-local:
    ask: Remove the file from every checkpoint in the history.
    resolve:
      command: |
        pip install git-filter-repo
        git filter-repo --invert-paths --path <path/to/the/file> --force
      shell: powershell
      does: >
        Installs the history-rewriting tool, then deletes that path from every
        checkpoint in the repository. `--invert-paths` means remove these paths
        rather than keep only these paths.
      destroys: >
        Every checkpoint hash in the repository changes, so any other copy of
        this repository stops matching yours. The tool also removes the `origin`
        remote deliberately, so you have to add it back before you can push. Your
        files on disk are not touched.
      verify: >
        `git log -p -S "<fragment-of-the-secret>"` prints nothing, and `git log
        --oneline` still shows a sensible history.
      if_it_did_not_work: >
        If `git filter-repo` is not recognized after installing, close and reopen
        the terminal so PATH reloads. If Python is not installed, an alternative
        for a short history is starting a fresh repository: see the delete route
        in this tree.

  on-github:
    ask: >
      Is this a private repository that only you use, short enough that losing
      its history would cost you nothing?
    branches:
      - label: Yes, it is mine and private and the history does not matter
        goto: fresh-start
      - label: No, other people or other machines have it
        goto: rewrite-pushed

  fresh-start:
    ask: Throw the entire history away and start a new one from the files as they are now.
    resolve:
      command: |
        Remove-Item .git -Recurse -Force
        git init
        git add .
        git commit -m "chore: fresh start after a credential leak"
      shell: powershell
      does: >
        Deletes the repository's history completely and starts a new one holding
        only your current files. The leaked value is gone because no old
        checkpoint exists any more.
      destroys: >
        Every checkpoint, branch, and tag in this repository, permanently. You
        keep only the files exactly as they are on disk right now. Make sure the
        secret is already out of those files before you run it.
      verify: >
        `git log --oneline` shows exactly one checkpoint, and searching the file
        for the secret finds nothing.
      if_it_did_not_work: >
        Then delete the copy on GitHub too and push this one fresh: `gh repo
        delete <owner>/<repo> --yes` followed by `gh repo create <name> --private
        --source . --push`. Leaving the old repository up leaves the secret
        readable.

  rewrite-pushed:
    ask: Rewrite the history, then replace what GitHub has.
    resolve:
      command: |
        pip install git-filter-repo
        git filter-repo --invert-paths --path <path/to/the/file> --force
        git remote add origin https://github.com/<owner>/<repo>.git
        git push --force --all
      shell: powershell
      does: >
        Removes the file from every checkpoint, reconnects the repository to
        GitHub, and replaces the branches there with the cleaned versions.
      destroys: >
        Every checkpoint hash changes, and the force push overwrites the branches
        on GitHub. Anyone else's copy becomes incompatible and they have to clone
        again. Old checkpoints can stay reachable through GitHub for a while
        afterward, and any fork keeps its own copy, which is why this is the
        second step and rotation is the first.
      verify: >
        Clone the repository into a new folder and search it: the secret should
        not appear. `git log --oneline` on GitHub shows the rewritten history.
      if_it_did_not_work: >
        Ask GitHub Support to expire the cached views of the old checkpoints. Do
        this only after rotating, because it does not help with a credential that
        still works.

see_also:
  - g6-secrets-and-what-never-to-commit
  - d12-gitignore-and-what-not-to-commit
  - g8-what-never-to-paste-into-a-chat
  - g5-environment-variables
  - panic-cant-push
  - e11-what-to-never-let-an-agent-do
keywords:
  - committed a secret
  - api key in git
  - leaked token
  - pushed my env file
  - password in commit
  - remove secret from history
  - push protection blocked
---

The one sentence that matters: a new commit that deletes the secret does not remove it from
the history. Git keeps every version of every file. The old checkpoint is still there, still
readable by anyone with a copy of the repository, and a search takes one command to run.

So the order is fixed. Rotate the credential first, before you touch git at all. Create a
new key at the service, put it somewhere ignored, and switch the old one off. Assume the old
one is compromised from the moment it was pushed, because credential scanners watch public
repositories continuously and they are fast.

Cleaning the history is worth doing and it is second. It stops the value being handed to the
next person who clones, and that is a real gain. It does not retract anything that was
already read.

If GitHub blocked your push with a message about push protection, that is their scanner
catching it before it went public, which is the best version of this problem. Rotate anyway
if the value was ever real, then take it out of the commit. The bypass link they offer is
for false positives only.

`.gitignore` prevents the next one: [d12](#d12-gitignore-and-what-not-to-commit) for the
mechanism, [g6](#g6-secrets-and-what-never-to-commit) for the policy.
