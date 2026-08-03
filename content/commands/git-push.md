---
id: git-push
title: git push
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git push
shell: any

does: >
  Uploads the commits on your current branch to GitHub, so the copy stored there matches
  the copy on your machine.

flags:
  - flag: "origin <branch-name>"
    means: >
      Names the remote and the branch explicitly, as in `git push origin main`. `origin` is
      the default nickname for the repository you cloned from. Needed only when this branch
      has no upstream recorded yet.
  - flag: "--tags"
    means: >
      Also uploads your tags. Tags are not pushed by an ordinary `git push`, which is why a
      version tag you made never appears on GitHub.
  - flag: "--dry-run"
    means: Shows what would be uploaded without uploading anything. Costs a second, answers the question.

expect: >
  Counting and compressing lines, then a summary such as
  `To github.com:nyxlocke/myproject.git` and `3f2a1b9..8c4d2e1  main -> main`.
  `Everything up-to-date` means you had nothing new to send.

see_also:
  - git-push-set-upstream
  - git-push-force-with-lease
  - git-pull
  - git-fetch

keywords:
  - upload my commits
  - send code to github
  - push changes
  - everything up to date
---

If it says `fatal: The current branch has no upstream branch`, this branch has never been
pushed. The message tells you the exact command, which is
`git push -u origin <branch-name>`.

If it says `Updates were rejected because the remote contains work that you do not have
locally`, someone pushed before you. Run `git pull` first, then push again.
