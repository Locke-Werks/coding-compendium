---
id: git-failed-to-push-some-refs
title: "error: failed to push some refs"
type: error
verified: 2026-08-02
volatility: low

category: conflict

# After a successful pull and push, this prints one line with no ahead or
# behind counts on it.
verify: git status -sb

danger: >
  The last step uses `git push --force-with-lease`, which replaces the branch on GitHub
  with your version. Any commit that exists only on GitHub is dropped from that branch. The
  safe path is the first step, `git pull`, which keeps both sides. Only force when you know
  exactly what is on the remote and have decided it should go.

sample: |
  PS C:\Users\nyx\dev\scraper> git push
  To https://github.com/nyxlocke/scraper.git
   ! [rejected]        main -> main (fetch first)
  error: failed to push some refs to 'https://github.com/nyxlocke/scraper.git'
  hint: Updates were rejected because the remote contains work that you do
  hint: not have locally. This is usually caused by another repository pushing
  hint: to the same ref. You may want to first integrate the remote changes
  hint: (e.g., 'git pull ...') before pushing again.
  hint: See the 'Note about fast-forwards' in 'git push --help' for details.

patterns:
  - "failed to push some refs"
  - "Updates were rejected because the remote"
  - "Note about fast-forwards"
  - "non-fast-forward"
  - "fetch first"

means: >
  GitHub has at least one commit that your machine does not. Pushing would replace the
  branch on GitHub with yours and those commits would stop being reachable, so git refuses
  and makes you deal with it. This is a safety refusal, not a failure. Nothing was sent,
  nothing was lost, and your local commits are exactly where you left them.

fix_ladder:
  - try: Bring the remote's commits down, then push again.
    command: git pull
    shell: powershell
    why: >
      Assumes someone or something added commits on GitHub that you do not have. That is
      what "the remote contains work" means. If the pull finishes without complaint, run
      `git push` again and you are done. This is the answer the large majority of the time.

  - try: See what is over there before you merge it.
    command: git fetch; git log HEAD..origin/main --oneline
    shell: powershell
    why: >
      Assumes you want to know what is arriving. `git fetch` downloads without changing any
      of your files, and the two dots mean "on the remote and not on my side". A single
      commit editing `README.md` means you changed a file in the GitHub web interface and
      forgot.

  - try: Check that you are pushing the branch you think you are.
    command: git status -sb
    shell: powershell
    why: >
      Assumes a branch mix-up. The first line names your branch and the remote branch it
      tracks. Pushing a local `main` at a remote `main` that a pull request already merged
      into produces exactly this rejection.

  - try: Finish the merge if the pull stopped with a conflict.
    command: git status
    shell: powershell
    why: >
      Assumes the pull started a merge and could not finish it, because the same lines
      changed on both sides. Nothing will push until the merge is resolved and committed.
      Resolving conflicts has its own card.

  - try: Overwrite the remote branch, only when you are certain.
    command: git push --force-with-lease
    shell: powershell
    why: >
      Assumes you rebased or amended on purpose and the remote's version of history is the
      one that should go. `--force-with-lease` refuses to run if the remote moved since your
      last fetch, which plain `--force` does not check. Never do this on a branch anyone
      else works on.

if_none_worked: >
  Paste the entire push output including every `hint:` line, plus the output of
  `git log --oneline --graph --all -10`. The hint lines are the first thing people cut and
  they name the exact refusal. The graph shows where the two histories split, which is what
  decides between pulling and forcing.

see_also:
  - d6-merge-and-rebase
  - d7-merge-conflicts
  - d2-repo-remote-clone-origin
  - e11-what-to-never-let-an-agent-do

keywords:
  - failed to push
  - rejected non-fast-forward
  - updates were rejected
  - fetch first
  - push rejected
---

The word "refs" means branch and tag names. "Failed to push some refs" means git would not
move the branch pointer on GitHub to where yours points.

The cause is nearly always ordinary. You merged a pull request in the browser, or edited a
file on the GitHub website, or an agent pushed from a second folder. Any of those puts a
commit on GitHub that your machine has never seen.

`git pull` then `git push` is the whole fix in that case. The pull creates a merge commit
joining the two lines of work, which is untidy in a history graph and completely harmless.

Force pushing is the tempting shortcut and the one that actually loses work. It tells
GitHub to forget its version. If the thing it forgets was a merged pull request or a
teammate's commit, it is gone from the branch and finding it again means digging through
the reflog on whichever machine still has it.
