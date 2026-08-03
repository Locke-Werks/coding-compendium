---
id: panic-cant-push
title: My push is being refused
type: panic
verified: 2026-08-02
volatility: quarterly
danger: >
  Nothing in this tree destroys anything. It is here because the wrong fix does:
  `git push --force` replaces what GitHub holds with what you have and deletes
  any checkpoint you did not have. Almost every refusal on this page is solved by
  pulling first, and none of them is solved by forcing.
symptom: >
  I ran `git push` and it printed a wall of red text instead of sending my work.
reassurance: >
  A refused push has damaged nothing. Your checkpoints are still on your machine
  exactly as they were and GitHub is unchanged, which is the point: git refuses
  rather than guessing when the two sides disagree. Every refusal has a specific
  phrase in it that names the cause, and this tree sorts by that phrase.
backup_first: git branch backup-now
root: which-message
nodes:
  which-message:
    ask: Which of these appears in the text git printed?
    how_to_tell: git push
    branches:
      - label: rejected, fetch first, or non-fast-forward
        goto: behind
      - label: no upstream branch, or set-upstream
        goto: no-upstream
      - label: Permission denied, Authentication failed, or it asked for a password
        goto: auth
      - label: protected branch, or refusing to allow
        goto: protected
      - label: something about a file being too large
        goto: too-big
      - label: secret, token, or push protection
        goto: secret-block
      - label: none of these, or I cannot tell
        goto: read-again

  behind:
    ask: GitHub has checkpoints you do not. Take them, then send yours.
    resolve:
      command: |
        git pull --rebase
        git push
      shell: powershell
      does: >
        Downloads what GitHub has that you are missing, replays your own
        checkpoints on top of them, and sends the result. The refusal means the
        branch moved on GitHub since you last looked.
      destroys: >
        Nothing. The rebase can stop on a conflict, which is normal and is not
        damage.
      verify: >
        `git status -sb` no longer says behind, and the push prints a summary
        line ending in your branch name.
      if_it_did_not_work: >
        If it stops on a conflict, panic-merge-conflict-stuck handles it and `git
        rebase --abort` puts you back. Do not reach for a forced push here: it
        deletes whatever GitHub had that you did not.

  no-upstream:
    ask: This branch has never been sent before. Send it and record the pairing.
    resolve:
      command: git push -u origin <your-branch-name>
      shell: powershell
      does: >
        Creates the branch on GitHub and records which local branch belongs to
        which remote one. `-u` is what makes every later push on this branch just
        `git push` with no arguments.
      destroys: Nothing.
      verify: >
        The command prints a link for opening a pull request, and `git status
        -sb` now names an upstream on its first line.
      if_it_did_not_work: >
        If you do not know your branch name, `git branch --show-current` prints
        it. If it then says the repository does not exist, `git remote -v` shows
        where you are pointed.

  auth:
    ask: GitHub does not believe you are allowed. Check how you are signed in.
    resolve:
      command: gh auth status
      shell: powershell
      does: >
        Reports whether the GitHub command-line tool is signed in and which
        account it is using. If it says you are not logged in, run `gh auth
        login` and choose the same connection type your repository uses.
      destroys: Nothing. It only reads.
      verify: >
        `gh auth status` prints your account name and a success line. Then try
        the push again.
      if_it_did_not_work: >
        Run `git remote -v`. An address starting with `git@github.com` is a key
        problem rather than a login problem, and b5-ssh-vs-https covers both. An
        address pointing at somebody else's account means you have no write
        access there and need your own copy.

  protected:
    ask: >
      That branch refuses direct pushes on purpose. Put the work on a new branch
      instead.
    resolve:
      command: |
        git switch -c fix/<short-name>
        git push -u origin fix/<short-name>
      shell: powershell
      does: >
        Creates a new branch at exactly where you are, with all your work already
        on it, and sends that instead. Protected branches are meant to be reached
        through a pull request.
      destroys: Nothing. The original branch keeps its checkpoints too.
      verify: >
        The push succeeds and prints a link. `gh pr create --fill` opens the pull
        request from that link.
      if_it_did_not_work: >
        If your local copy of the protected branch is now ahead of GitHub and you
        want it back in line, d10-undo-everything covers walking a branch back.

  too-big:
    ask: >
      GitHub refuses any single file over one hundred megabytes. Stop tracking
      the large one.
    resolve:
      command: git rm --cached "<path/to/the/large/file>"
      shell: powershell
      does: >
        Stops git tracking the file and leaves it exactly where it is on disk.
        Then add its name to `.gitignore` and commit both changes together.
      destroys: Nothing on disk. The file itself is untouched.
      verify: >
        `git status` no longer lists it and the push goes through.
      if_it_did_not_work: >
        If the push still fails, the file is in an older checkpoint too, and
        removing it from the newest one is not enough. That needs a history
        rewrite with the same tool panic-committed-a-secret uses. If you
        genuinely need to store files that size, look up Git LFS (Large File
        Storage).

  secret-block:
    ask: GitHub found a credential in what you tried to send. Find it.
    resolve:
      command: git log -p -1
      shell: powershell
      does: >
        Prints exactly what your last checkpoint added, so you can see the line
        their scanner flagged. This block is the system working: the credential
        never reached the internet.
      destroys: Nothing. It only reads.
      verify: >
        You can see the flagged value in the output and name which service issued
        it.
      if_it_did_not_work: >
        Do not use the bypass link unless you are certain it is a false positive.
        If the value was ever a real credential, rotate it first, then follow
        panic-committed-a-secret to get it out of the checkpoint.

  read-again:
    ask: Get the message again, cleanly, along with where you are pushing.
    resolve:
      command: |
        git remote -v
        git push
      shell: powershell
      does: >
        Prints the address you are pushing to, then tries again so the refusal
        arrives fresh. The cause is always in the last three lines.
      destroys: Nothing.
      verify: >
        You have the exact wording. If it names something from the list above, go
        back and pick that branch.
      if_it_did_not_work: >
        Paste the whole message, unedited, with the command you ran, into your
        agent. f5-what-to-paste-and-what-not-to covers the one thing to strip
        first.

see_also:
  - panic-committed-a-secret
  - panic-merge-conflict-stuck
  - b5-ssh-vs-https
  - b4-github-and-gh
  - d8-pull-requests
  - d10-undo-everything
  - d2-repo-remote-clone-origin
keywords:
  - push rejected
  - cannot push
  - fetch first
  - non fast forward
  - permission denied
  - authentication failed
  - updates were rejected
  - failed to push some refs
---

Git refuses a push whenever accepting it would lose something or whenever it cannot prove
who you are. Both are protections, and neither has changed anything yet.

The most common refusal by far is the first one in the tree: someone or something added a
checkpoint on GitHub that you do not have, so sending yours would overwrite it. Pulling
first and pushing second solves it. On a solo project the other party is usually you, from
the website or another machine.

The refusal you should never solve with force is that same one. Forcing tells GitHub to
replace what it has with what you have, which does exactly the thing git was protecting you
from. Reach for it only when you deliberately rewrote history, and even then prefer
`git push --force-with-lease`, which refuses if the remote moved since you last looked.

Read the last three lines of the output rather than the first three. The top of a push
failure is usually a long address and a progress bar, and the reason is at the bottom.
[f1](#f1-how-to-read-an-error-message) covers why that is true of nearly every command.
