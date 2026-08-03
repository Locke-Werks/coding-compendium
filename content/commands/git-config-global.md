---
id: git-config-global
title: git config --global
type: command
verified: 2026-08-02
volatility: low

tool: git
command: git config --global user.name "<Your Name>"
shell: any

verify: git config --global --list

does: >
  Sets a git setting for every repository on your machine, most often the name and email
  stamped onto each commit you make.

flags:
  - flag: "--global"
    means: >
      Write the setting to your personal config file at `C:\Users\<yourname>\.gitconfig`,
      where it applies to every repository you touch. Without it, git writes to the current
      repository only, and fails outright if you are not inside one.
  - flag: "--local"
    means: >
      Write to this repository only, at `.git\config`. The narrower setting wins, which is
      how you use a different email on one project.
  - flag: "--list"
    means: >
      Print every setting currently in effect. `git config --global --list` shows only your
      personal ones, which is the readable version.
  - flag: "--unset <key>"
    means: Removes a setting you no longer want, as in `git config --global --unset user.email`.

expect: >
  Nothing printed. Read the value back with `git config --global user.name`, which should
  echo exactly what you set.

see_also:
  - git-commit
  - b3-tell-git-who-you-are
  - b9-where-settings-live

keywords:
  - set my name and email
  - please tell me who you are
  - git identity
  - default branch name
---

The four settings worth having on a new machine:

```powershell
git config --global user.name "<Your Name>"
git config --global user.email "<you@example.com>"
git config --global init.defaultBranch main
git config --global pull.rebase false
```

The email is public on every commit you push. GitHub gives you a no-reply address at
Settings, Emails if you would rather not publish a real one.
