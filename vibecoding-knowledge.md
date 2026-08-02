# Vibe Coding with Claude Code and OpenAI Codex (Windows Edition)

A complete, no-guesswork guide for Nyx.

This is written for someone who has touched code before but has not lived inside git, the terminal, or AI (Artificial Intelligence) coding agents day to day. It assumes nothing about those three things and builds up from the floor. Everything here targets Windows specifically. There are no Mac or Linux detours to wade through.

Read it top to bottom the first time. After that, use the glossary in Part 9 as your dictionary and the workflow sections as your recipe cards.

Everything here was checked against the current state of both tools as of August 2026. These tools ship changes weekly, so if a command behaves differently, the official docs linked in each section are the source of truth.

Acronyms are spelled out in parentheses the first time they appear. If you land on one cold, check the glossary.

---

## Part 0: What "vibe coding" actually is, and the honest version of it

"Vibe coding" is a term coined by Andrej Karpathy in early 2025. It describes a way of working where you describe what you want in plain language and let an AI (Artificial Intelligence) agent write, run, and revise the code, while you focus on the outcome and the feel of the result rather than reading and hand-writing every line yourself.

That is the fun version. Here is the version that keeps you out of trouble.

The AI is fast and confident and wrong often enough that you cannot trust it blindly. The thing that makes vibe coding safe instead of reckless is not the AI, it is the scaffolding around it: version control so you can undo anything, tests so you can tell whether it actually works, and small reviewable steps so a mistake is caught in five minutes instead of five days. If you take one idea from this whole guide, take that one. The workflow sections below are built around it.

So the real loop is: describe, generate, review the change, test it, save a checkpoint, repeat. You will still not read every line. You will read enough, and you will lean on tests and git to catch what you miss.

---

## Part 1: The mental model, before any installing

### The three pieces

1. **The agent.** Claude Code and Codex are agents. An agent is an AI that does not just answer questions, it takes actions on your machine: it reads your files, writes new files, edits existing ones, runs commands in your terminal, and can talk to GitHub. You give it a goal, it makes a plan, and it executes, checking in with you along the way.

2. **The place the code lives.** Your code sits in folders on your computer. A folder that git is tracking is called a **repository**, or **repo**. Everything the agent builds goes into a repo.

3. **The backup and collaboration layer.** GitHub is a website that stores copies of your repos in the cloud. It is where your work is backed up, where a history of every change lives, and where, if you ever work with someone, you combine your changes with theirs. Your GitHub account is `nyxlocke`.

### Each tool has three "surfaces"

Both Claude Code and Codex come in three flavors. Know the words so the docs make sense:

- **CLI (Command-Line Interface):** the tool runs in your terminal. You type, it responds in text. This is the most capable surface for both tools and the one this guide focuses on.
- **IDE (Integrated Development Environment) extension:** the tool runs inside a code editor like VS Code (Visual Studio Code), which is an editor with build and debug tooling baked in. Same engine, friendlier window.
- **Cloud / web:** the tool runs on the vendor's servers, kicked off from a browser, useful for long jobs you do not want tying up your laptop. Claude has this in the web app and Claude Desktop. OpenAI has Codex Web at chatgpt.com/codex.

You will start with the CLI. It sounds intimidating and is not.

### Terminal, shell, and command line: same neighborhood, different words

- **Terminal:** the window where you type commands. On Windows, the modern one is **Windows Terminal**, and inside it you will usually be running **PowerShell**.
- **PowerShell:** Windows' built-in command shell, the program that interprets what you type.
- **Command Prompt (CMD):** the older Windows shell. A couple of install commands behave differently here, so this guide tells you which shell to use each time. You can tell them apart by the prompt: PowerShell shows `PS C:\...` and Command Prompt shows just `C:\...`.
- **Git Bash:** a bash shell (a different command interpreter, the kind used on Linux and Mac) that gets installed alongside Git for Windows. You will want it for one or two tasks, and Claude Code prefers it.
- **Command line / CLI (Command-Line Interface):** the general idea of controlling a program by typing commands instead of clicking buttons.

When someone says "run this in your terminal," they mean: open Windows Terminal, make sure you are in the right shell (usually PowerShell), paste the command, press Enter.

---

## Part 2: Prerequisites, installed once

Do these in order. Each has a way to confirm it worked, so you are never guessing.

### 2.1 Install Git for Windows

Git is the version-control program. It is separate from GitHub, the website. Git runs on your machine; GitHub stores copies online. You need Git installed before anything else.

Download "Git for Windows" from https://git-scm.com and run the installer. The defaults are fine, accept them all the way through. This installs three things you care about: git itself, Git Bash (the bash shell mentioned above), and the plumbing that lets Claude Code use a proper bash shell.

Open a new Windows Terminal window and confirm:
```
git --version
```
If it prints a version number, you are done.

### 2.2 Tell Git who you are

Every saved change (a commit) is stamped with a name and email. Set yours once, globally, so it applies to all projects. Use the email tied to your GitHub account.
```
git config --global user.name "Nyx Locke"
git config --global user.email "you@example.com"
```
Confirm it took:
```
git config --global user.name
git config --global user.email
```
Set the default branch name to `main` (the modern convention, replacing the older `master`) and make git's pull behavior predictable:
```
git config --global init.defaultBranch main
git config --global pull.rebase false
```

### 2.3 Confirm your GitHub account

You have `nyxlocke`. Log in at https://github.com and make sure you can see your account. That is all you need from the website side for now. Everything else you will drive from the terminal.

### 2.4 Install the GitHub CLI (Command-Line Interface), the `gh` tool

`gh` is GitHub's official command-line tool. It is the cleanest way to connect your machine to your GitHub account, and it is what the AI agents use to open pull requests for you.

In PowerShell:
```
winget install GitHub.cli
```
`winget` is the Windows Package Manager, a tool built into Windows that installs other software from the command line. After it finishes, close and reopen your terminal, then confirm:
```
gh --version
```

### 2.5 Authenticate `gh` to the `nyxlocke` account

This is the step that links your computer to GitHub so you can push code and open pull requests without typing a password every time.
```
gh auth login
```
Answer the prompts like this:
- **Account:** GitHub.com
- **Protocol:** SSH (Secure Shell), recommended and explained below. HTTPS (Hypertext Transfer Protocol Secure) also works.
- **Authenticate:** "Login with a web browser." It shows you a one-time code, opens GitHub in your browser, you paste the code, done.

When it finishes, `gh` has stored credentials for `nyxlocke` and, if you chose SSH, uploaded an SSH key to your GitHub account automatically. Confirm:
```
gh auth status
```
It should say you are logged in as `nyxlocke`.

**SSH (Secure Shell) vs HTTPS (Hypertext Transfer Protocol Secure), in one breath:** these are two ways your machine proves to GitHub that it is allowed to push. HTTPS uses a token, which is a long password-like string. SSH uses a key pair: a private key that stays on your machine and a public key GitHub knows about. SSH means you set it up once and never think about it again, which is why it is the default recommendation. If `gh auth login` set up SSH for you, you are done. If you ever need to make an SSH key by hand, run this in PowerShell:
```
ssh-keygen -t ed25519 -C "you@example.com"
```
Press Enter through the prompts. Then `gh` can upload it, or you paste the contents of `C:\Users\<yourname>\.ssh\id_ed25519.pub` into GitHub under Settings, then SSH and GPG keys.

At this point your foundation is done: Git installed, identity set, GitHub connected. Now the agents.

---

## Part 3: Install and configure Claude Code

Full docs: https://code.claude.com/docs

### 3.1 What you need first

- One of these paid plans: Claude Pro, Max, Team, or Enterprise, or an Anthropic Console account with API (Application Programming Interface) credits. The free Claude.ai plan does not include Claude Code. If you are on Archon's setup, you already have access.
- Git for Windows installed (done in Part 2).

### 3.2 Install

Anthropic now recommends the **native installer**, a self-contained program that needs no extra runtime and updates itself in the background. Use this unless you have a specific reason not to.

Open **PowerShell** (not Command Prompt) and run:
```
irm https://claude.ai/install.ps1 | iex
```
Then close and reopen your terminal so it can find the new command, and confirm:
```
claude --version
```

Troubleshooting the two most common snags:
- If you see `'irm' is not recognized`, you are in Command Prompt, not PowerShell. Open PowerShell (its prompt starts with `PS`) and run it there.
- If PowerShell blocks the script with an execution-policy error, run this once, then rerun the installer:
  ```
  Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
  ```
- If `claude` is "not found" after installing, your terminal has not picked up the new location on its PATH yet. PATH is the list of folders your terminal searches for commands. Just close and reopen the terminal.

Alternate install if you prefer a package manager: `winget install Anthropic.ClaudeCode`. One tradeoff: the winget version does not auto-update, so you would update it yourself with `winget upgrade Anthropic.ClaudeCode`. The native PowerShell installer above auto-updates, which is why it is the recommendation.

There is also an old npm (Node Package Manager) method, which installs through the JavaScript tooling stack and now requires Node.js 22 or newer. Skip it for a fresh Windows setup.

Because you installed Git for Windows in Part 2, Claude Code can use Git Bash as its shell, which is smoother than PowerShell for the commands it runs. Good, nothing extra to do.

### 3.3 First launch and sign-in

Move into a project folder (or make one) and start it:
```
cd ~/dev/my-first-project
claude
```
In PowerShell, `~` is shorthand for your home folder, `C:\Users\<yourname>`. The first run opens your browser to log in. Approve it, come back to the terminal, and you are in an interactive session. You type requests in plain English. Type `/help` any time to see commands. Slash commands, the ones starting with `/`, control the tool itself, for example `/login`, `/model`, `/init`, `/clear`.

### 3.4 Point it at your project with `/init`

Inside a session, run:
```
/init
```
Claude scans the folder and writes a `CLAUDE.md` file. This file is Claude's standing instructions for that project: what the project is, how it is structured, conventions to follow, commands to run. Claude reads it at the start of every session. Think of it as the project's rulebook that you and Claude both edit over time. You will add to it as the project grows.

### 3.5 Connect Claude Code to GitHub

Two different connections, do not confuse them:

1. **Local push and pull, which you already have.** Because `gh` is authenticated to `nyxlocke`, Claude Code can run git and `gh` commands on your behalf: it can commit, push, and open pull requests using your existing credentials. Nothing more to set up. Just ask it, for example, "commit this and open a pull request."

2. **The GitHub App, optional, for automation.** If you want to mention `@claude` inside a GitHub pull request or issue and have it respond on GitHub's servers, run this inside a Claude session:
```
/install-github-app
```
It walks you through installing Anthropic's GitHub App on your `nyxlocke` repos. This is a convenience for later, not required to code locally. Skip it until you want it.

### 3.6 Where Claude Code keeps its settings

Claude Code reads settings from JSON (JavaScript Object Notation) files. JSON is a simple text format for structured data, made of `"key": value` pairs inside braces. Understanding the three levels saves confusion later:

- `C:\Users\<yourname>\.claude\settings.json` : **user level.** Applies to every project on your machine. In PowerShell you can also write this path as `~/.claude/settings.json`.
- `.claude\settings.json` inside a project : **project level.** Gets committed to git and shared with anyone on the repo.
- `.claude\settings.local.json` inside a project : **local level.** Git-ignored, so it is yours alone and never shared.

Narrower wins: local overrides project, project overrides user. You will use these in Part 6 to turn off attribution.

### 3.7 Plan mode (important, use it constantly)

Claude Code has a **plan mode** where it thinks through the whole approach and shows you the plan before touching a single file. You approve or redirect, then it executes. This is the single best habit for vibe coding, because it catches bad ideas before they become bad code. Enter it by pressing **Shift+Tab** to cycle modes, or ask directly: "Make a plan first, do not write code yet." More on this in Part 7.

---

## Part 4: Install and configure OpenAI Codex

Full docs: https://developers.openai.com/codex

Codex is OpenAI's coding agent. It runs in your terminal, and by default it sandboxes what it can do, meaning it cannot touch the network or write outside your project folder unless you allow it. It is a genuinely different engine from Claude Code with different strengths, and running both is common. A widely repeated split is: plan and brainstorm with one, review and execute with the other. You will find your own preference.

### 4.1 What you need first

- A ChatGPT plan (Plus, Pro, Business, or higher) for sign-in access to the models, or an OpenAI API (Application Programming Interface) key for pay-as-you-go and automation use. Signing in with ChatGPT gets you the newest models soonest; API-key access lags on new models but is better for automation.

### 4.2 Install

In **PowerShell**:
```
powershell -ExecutionPolicy ByPass -c "irm https://chatgpt.com/codex/install.ps1 | iex"
```
Confirm:
```
codex --version
```
Alternate via npm (Node Package Manager) if you already have Node.js: `npm i -g @openai/codex`.

One honest note for Windows: Codex's sandbox runs natively in PowerShell, and that is fine for normal work. If you ever hit sandbox errors that will not clear, the fallback is to run Codex inside WSL (Windows Subsystem for Linux), which is a full Linux environment running inside Windows. You do not need WSL to start, and this guide stays on native Windows. File it away only as a break-glass option.

### 4.3 First launch and sign-in

Inside a project folder:
```
cd ~/dev/my-first-project
codex
```
It prompts you to sign in. Choose "Sign in with ChatGPT" and approve in the browser. That is the simplest path and the one to use.

If you would rather use an API key, set it in PowerShell for the current session:
```
$env:OPENAI_API_KEY="sk-..."
```
To make a key stick permanently across new terminals, use:
```
setx OPENAI_API_KEY "sk-..."
```
`setx` saves it for future terminals but does not change the one you are in, so open a fresh terminal afterward. If you ever need to clear a stuck login, run `codex logout`.

### 4.4 Point it at your project with AGENTS.md

Codex's equivalent of `CLAUDE.md` is **`AGENTS.md`**. Same idea: a plain-text file of standing instructions Codex reads for the project. Usefully, `AGENTS.md` is an open convention that several AI tools respect, not just Codex, so it is worth keeping current. Create one at the project root and describe the project, conventions, and any "never do this" rules.

### 4.5 Connect Codex to GitHub

Same as Claude Code: because `gh` is already authenticated to `nyxlocke`, Codex can run git and `gh` commands to commit, push, and open pull requests using your credentials. For the cloud version, Codex Web, there is extra setup to give the remote environment GitHub access, but for local terminal work you are already connected.

### 4.6 Codex's settings file: `config.toml`

Codex is configured through `C:\Users\<yourname>\.codex\config.toml` (also writable as `~/.codex/config.toml` in PowerShell). TOML (Tom's Obvious, Minimal Language) is a simple settings-file format: `name = value` lines, grouped under `[headings]`. The knobs you care about early:

```toml
# C:\Users\<yourname>\.codex\config.toml
model = "gpt-5.6-codex"           # model names change often; pick from the in-app list
approval_policy = "on-request"    # when Codex must ask before acting
sandbox_mode = "workspace-write"  # what Codex is allowed to touch
```

Two controls that matter and are easy to mix up:
- **`sandbox_mode`** controls what Codex is technically able to do. `read-only` (look but do not change), `workspace-write` (change files inside the project, no network), or a fully unlocked bypass mode (avoid unless you know why).
- **`approval_policy`** controls when it stops to ask you first, independent of the sandbox. Common values: `untrusted` (ask a lot), `on-request` (a sensible middle), `never` (do not ask, for automation).

Model names in the Codex world move fast and get retired on a schedule (the GPT (Generative Pre-trained Transformer) 5.x family as of this writing). Do not hard-code a name you read in an old blog post. When you launch `codex`, use its model picker, or check https://developers.openai.com/codex/models for what is current. Setting `model` in `config.toml` just makes your choice stick across sessions.

Approval and sandbox docs: https://developers.openai.com/codex/config-advanced

---

## Part 5: How to arrange your files on disk

A tidy layout makes both agents dramatically more effective, because they navigate the same folders you do. Here is a conventional setup that scales.

### 5.1 One home for all your code

Make a single top-level folder for projects. Common names are `dev`, `code`, or `projects`. Pick one and stick with it. In PowerShell:
```
mkdir ~/dev
```
That creates `C:\Users\<yourname>\dev`. Every project gets its own subfolder inside it. Never nest one project inside another. Never let a project sprawl across random locations. One folder, one project, always.

### 5.2 Anatomy of a single project folder

A healthy project looks roughly like this:
```
C:\Users\<yourname>\dev\my-project\
├── .git\                  (git's hidden database, created by "git init", never touch by hand)
├── .gitignore             (list of files git should ignore, e.g. secrets, build junk)
├── README.md              (what this project is, how to run it; the front door)
├── CLAUDE.md              (standing instructions for Claude Code)
├── AGENTS.md              (standing instructions for Codex and other agents)
├── .claude\
│   ├── settings.json          (project-level Claude settings, shared)
│   └── settings.local.json    (your private Claude settings, git-ignored)
├── src\                   (the actual source code)
├── tests\                 (automated tests)
└── docs\                  (design notes, specs, planning docs)
```

You do not create all of this by hand. `git init` makes `.git\`. `/init` in Claude makes `CLAUDE.md`. The agents create `src\`, `tests\`, and code as they build. Your job is to start the folder and know what each piece is.

### 5.3 The `.gitignore` file, and why it matters for safety

`.gitignore` lists things git should never track: secret keys, passwords, `.env` files, downloaded dependencies, build output. This is not optional housekeeping, it is how you avoid publishing a secret to GitHub by accident. A minimal one:
```
# secrets
.env
*.key
# dependencies and build junk
node_modules/
dist/
build/
# operating system clutter
Thumbs.db
desktop.ini
```
When you ask an agent to set up a project, tell it to create a proper `.gitignore` for the language you are using. Both tools do this well.

### 5.4 Starting a fresh project from zero

The clean sequence, which you can do by hand in PowerShell or hand to an agent:
```
mkdir ~/dev/my-project
cd ~/dev/my-project
git init
gh repo create nyxlocke/my-project --private --source=. --remote=origin
```
That last command creates the repo on GitHub under `nyxlocke`, marks it private, and links your local folder to it. The link is called a **remote**, and it is named `origin` by convention. Now local and GitHub are connected. Launch `claude` or `codex` and start building.

---

## Part 6: Turning off AI attribution (your specific requirement)

By default, some of these tools stamp their name onto your git history: a `Co-Authored-By` line on commits, a "Generated with Claude Code" footer on pull requests. You want none of that. Here is exactly how to silence it in each tool. Set these once and forget them.

### 6.1 Claude Code

The setting changed in late 2025 and into 2026. The old `includeCoAuthoredBy` key is **deprecated**, meaning retired and no longer the right way. The current mechanism is an `attribution` block, and it is what the official docs use. The `attribution` block takes precedence over the old key.

Open (or create) your user-level settings file at `C:\Users\<yourname>\.claude\settings.json` and set the attribution fields to empty strings, plus turn off the session link:

```json
{
  "attribution": {
    "commit": "",
    "pr": "",
    "sessionUrl": false
  }
}
```

- `commit`: text appended to commit messages. Empty string means nothing is added.
- `pr`: text appended to pull request descriptions. Empty string means nothing is added.
- `sessionUrl`: set to `false` so no URL (Uniform Resource Locator), the web link back to the Claude session, is added.

Putting this in the user-level file applies it to every project on your machine. If you ever want it enforced per-project and shared, put the same block in that project's `.claude\settings.json` instead.

Official reference: https://code.claude.com/docs/en/settings (see the attribution section).

**Belt-and-suspenders, optional but recommended:** also add a line to each project's `CLAUDE.md`, because a plain-language instruction is a second layer that survives even if a settings file goes missing:
```
Never add Co-Authored-By lines, "Generated with Claude" footers, or any AI attribution to commits, pull requests, or git metadata.
```

### 6.2 OpenAI Codex

Codex historically added no attribution at all. Newer versions introduced a single config key, `commit_attribution`, in `config.toml`, which inserts a trailer string on commits. Its default, when active, is `Co-authored-by: Codex <noreply@openai.com>`. To guarantee nothing is ever added, set it to an empty string:

```toml
# C:\Users\<yourname>\.codex\config.toml
commit_attribution = ""
```

Because Codex's commit behavior has shifted across versions and can be tied to a git-commit feature flag, add the same plain-language guardrail to your `AGENTS.md` as a second layer:
```
Never add Co-authored-by lines or any AI attribution to commits, pull requests, or git metadata.
```

That combination, empty `commit_attribution` plus the AGENTS.md instruction, covers you regardless of version.

### 6.3 Claude Desktop

Claude Desktop is the app, and it has two modes that matter here:

1. **Its coding capability, the Code tab and agentic coding inside Desktop, runs the same Claude Code engine and reads the same `C:\Users\<yourname>\.claude\settings.json`.** So the `attribution` block from 6.1 already covers Desktop's coding work. Set it once, it applies everywhere Claude Code runs, terminal or Desktop.

2. **Plain chat and document work in Desktop** does not make git commits, so there is no commit trailer to worry about. What can happen is Claude adding a "made with Claude" style note to content it writes. Suppress that with a standing instruction. In Desktop, open Settings and put this in your profile **Preferences**, the box for how Claude should always behave:
```
Never sign, credit, or attribute your output to Claude or any AI. Do not add "generated with" notes, AI disclaimers, or co-authorship lines to anything you produce.
```
Preferences apply across all your chats, so this is a set-once fix.

### 6.4 The nuclear option: a git hook that works no matter what

If you want a guarantee that sits below every tool, add a git hook to a project. A hook is a script git runs automatically at certain moments. This one strips any `Co-Authored-By` line from every commit message in that repo, regardless of which tool or which version created it.

The cleanest way on Windows is to let an agent create the file, or to run the following in **Git Bash** (installed with Git for Windows), since this uses bash syntax that PowerShell does not understand:
```
cat > .git/hooks/commit-msg << 'EOF'
#!/bin/sh
sed -i.bak '/^Co-Authored-By:/d' "$1"
rm -f "$1.bak"
EOF
chmod +x .git/hooks/commit-msg
```
Git on Windows runs hooks through its own bundled shell, so this script works even though the rest of your work is in PowerShell. Hooks live in `.git\hooks\` and are per-project and not shared through git, so you would add this to each repo where you want the hard guarantee. For most people the settings from 6.1 and 6.2 are enough, and this is only for the truly paranoid.

---

## Part 7: How to design, plan, and execute a project

This is the part that separates "I made a mess with an AI" from "I shipped something." The workflow below is the conventional one, adapted for agents.

### 7.1 Design: write down what you are building, before you build

Spend fifteen minutes on a short specification, a "spec," before touching code. Put it in `docs\SPEC.md`. It does not need to be formal. Answer:
- What is this? One paragraph.
- Who uses it and what do they do with it?
- What are the core features, listed simplest to hardest?
- What is explicitly out of scope for version one?
- What does "done" look like?

This document is not busywork. It is the thing you hand to the agent so it builds what you meant instead of what it guessed. You will also feed pieces of it into `CLAUDE.md` and `AGENTS.md`.

### 7.2 Plan: let the agent turn the spec into a step list

Open the agent in your project and use plan mode. In Claude Code, press Shift+Tab into plan mode or say "read docs/SPEC.md and make a plan, do not write code yet." In Codex, ask for a plan first the same way. The agent produces an ordered list of steps. Read it. This is your cheapest chance to catch a wrong assumption. Push back, ask it to reorder, tell it what it got wrong. Only when the plan looks right do you let it build.

### 7.3 Execute: one small piece at a time, checkpoint after each

The rhythm, repeated for every feature:

1. **Branch.** Make a fresh branch for the piece of work so your stable `main` stays clean:
   ```
   git switch -c feature/login
   ```
   Or just ask the agent to do it.
2. **Build.** Have the agent implement that one piece. Keep the scope small. One feature, not five.
3. **Review the diff.** Look at what actually changed before saving it. A **diff** is the line-by-line view of what changed. Ask the agent "show me the diff" or run `git diff`. You are checking for anything obviously wrong, not auditing every character.
4. **Test.** Run the tests, or the app, and confirm it does the thing. If there are no tests, ask the agent to add some. Working software you cannot verify is a rumor, not a result.
5. **Commit.** Save a checkpoint with a clear message (format below):
   ```
   git commit -m "feat: add email and password login"
   ```
6. **Push.** Send it to GitHub:
   ```
   git push -u origin feature/login
   ```
7. **Open a pull request (PR).** Even solo, PRs give you a clean before-and-after view and a place for notes:
   ```
   gh pr create --fill
   ```
8. **Merge.** When you are happy, merge the branch into `main`, then delete the branch. `gh pr merge --squash --delete-branch` does it in one shot.

Then start the next piece from a fresh branch. If something goes badly wrong at any point, git lets you throw away the branch and you have lost nothing but a little time. That safety net is the entire point.

### 7.4 Commit messages: the conventional format

Use "conventional commits." The message starts with a type, then a short summary in the present tense:
- `feat:` a new feature
- `fix:` a bug fix
- `docs:` documentation only
- `refactor:` restructuring without changing behavior
- `test:` adding or fixing tests
- `chore:` maintenance, dependencies, config

Keep the summary under about 72 characters. Example: `fix: stop logout on session timeout`. Put this convention in your `CLAUDE.md` and `AGENTS.md` and the agents will follow it automatically.

### 7.5 Using both agents together

A common and effective pattern: use one tool to plan and draft, then the other to review. For example, build a feature with Claude Code, then open Codex and say "review the changes in the last commit against main and flag anything risky," or the reverse. A second engine catches things the first one is blind to. You are not required to use both, but the option is there and it is genuinely useful.

---

## Part 8: The everyday git and GitHub vocabulary

You will hear these words constantly. Here is the plain-language version of each, in roughly the order you meet them. The full alphabetical glossary is in Part 9; this section is the "learn the neighborhood" version.

Your code lives in a **repo** (repository). The cloud copy on GitHub is called a **remote**, and the default remote is nicknamed **origin**. When you first copy a repo from GitHub to your machine you **clone** it. If you copy someone else's repo into your own account to work on independently, that is a **fork**.

Work happens on a **branch**, a parallel line of development. The main stable line is **main**. You make a branch, do work, then combine it back. Combining is a **merge**. Sometimes instead of merging you **rebase**, which replays your changes on top of the latest main for a cleaner history. When two changes touch the same line and git cannot auto-combine them, you get a **merge conflict**, which you resolve by choosing what the final version should say.

Saving a change is a two-step move. First you **stage** the files you want to include (git calls the staging area the **index**). Then you **commit**, which records a permanent snapshot with a message. To send your commits to GitHub you **push**. To bring down changes from GitHub you **pull**, which is really a **fetch**, meaning download, followed by a merge.

A **diff** is the view of exactly what changed, line by line. A chunk of a diff is a **hunk**. **HEAD** is git's word for "where you are right now" in the history. To move to a different branch you **switch** (older command: **checkout**). To temporarily shelve work you are not ready to commit, you **stash** it. To undo, you can **revert** (make a new commit that cancels an old one, safe) or **reset** (move the history pointer, powerful and sharp, use with care).

When your branch is ready to go into main, you open a **pull request (PR)**, also called a **merge request (MR)** on some platforms. A PR is a proposal: "here are my changes, here is what they do, please merge them." Even alone, PRs give you a review surface and a record. Reviewers leave comments, you address them, then someone merges. Merging with **squash** collapses all the little commits on the branch into one tidy commit on main.

Supporting cast: a **tag** marks a specific point, usually a version like `v1.0.0`. A **release** is a packaged, announced version built from a tag. **CI/CD (Continuous Integration / Continuous Delivery)** is automation that runs your tests and deployments when you push. **`.gitignore`** keeps junk and secrets out of the repo. **`gh`** is the GitHub command-line tool you set up in Part 2. A **PAT (Personal Access Token)** is a password-like string for authenticating to GitHub when you are not using SSH.

---

## Part 9: Full glossary, alphabetical, acronyms expanded

**Agent.** An AI (Artificial Intelligence) that takes actions (reads and writes files, runs commands, calls services) toward a goal, not just one that answers questions.

**AGENTS.md.** A plain-text instructions file for AI coding agents, read at the start of work. Codex's primary instruction file; also an open convention respected by other tools.

**AI (Artificial Intelligence).** Software that performs tasks normally needing human judgment. Here, specifically the models behind Claude and Codex.

**API (Application Programming Interface).** A defined way for one program to talk to another. An "API key" is a secret string that identifies and bills you when your code calls a paid service like OpenAI or Anthropic.

**Approval policy (Codex).** The setting controlling when Codex pauses to ask permission before acting. Independent from the sandbox.

**Attribution.** Text a tool adds to your commits or pull requests crediting the AI. Turned off in Part 6.

**Bash.** A command shell (the program that interprets terminal commands) common on Linux and Mac. On Windows you get it as Git Bash, installed with Git for Windows.

**Branch.** A parallel line of development within a repo. You branch off `main`, work, then merge back.

**CI/CD (Continuous Integration / Continuous Delivery or Deployment).** Automation that runs tests and ships code automatically when changes are pushed.

**CLAUDE.md.** Claude Code's project instructions file, read at the start of every session. Created by `/init`.

**CLI (Command-Line Interface).** Controlling a program by typing commands in a terminal rather than clicking.

**Clone.** Downloading a full copy of a repo from GitHub to your machine (`git clone`).

**CMD (Command Prompt).** The older Windows command shell. Prompt starts with `C:\`. A couple of installers behave differently here versus PowerShell.

**Commit.** A recorded snapshot of your staged changes, with a message. The fundamental unit of git history.

**config.toml.** Codex's settings file, at `C:\Users\<yourname>\.codex\config.toml`.

**Context window.** How much text (your files, the conversation, instructions) an AI model can consider at once, measured in tokens. A bigger window means it can hold more of your project in mind.

**Conventional commits.** A commit-message convention using prefixes like `feat:`, `fix:`, `docs:` (see Part 7.4).

**Detached HEAD.** A state where you are viewing a specific commit rather than the tip of a branch. Usually a "how did I get here" moment; switching back to a branch fixes it.

**Diff.** The line-by-line view of what changed between two versions.

**Fetch.** Downloading changes from a remote without merging them into your work yet.

**Fork.** Your own copy of someone else's repo, under your account, so you can work independently and later propose changes back.

**gh.** GitHub's official command-line tool. Handles authentication, pull requests, repo creation.

**Git.** The version-control program that runs on your machine and tracks every change.

**Git Bash.** A bash shell bundled with Git for Windows. Useful for a few bash-only tasks and preferred by Claude Code as its shell.

**GitHub.** The website that hosts repos in the cloud, backs up your work, and enables collaboration. Your account: `nyxlocke`. Not the same thing as git.

**.gitignore.** A file listing paths git should never track (secrets, build output, dependencies).

**GPT (Generative Pre-trained Transformer).** The family of models behind ChatGPT and Codex, for example `gpt-5.6-codex`.

**HEAD.** Git's pointer to your current position in history, normally the latest commit on your current branch.

**Hook (git hook).** A script git runs automatically at set moments, for example before or after a commit. Lives in `.git\hooks\`.

**Hunk.** A contiguous chunk of a diff.

**HTTPS (Hypertext Transfer Protocol Secure).** One of two ways to authenticate to GitHub, using a token. The alternative is SSH.

**IDE (Integrated Development Environment).** A code editor with build, run, and debug tooling built in, such as VS Code (Visual Studio Code) or JetBrains products.

**Index (staging area).** The in-between zone where you place changes before committing them.

**JSON (JavaScript Object Notation).** A simple text format for structured data, made of `"key": value` pairs. Claude Code's settings files use it.

**LLM (Large Language Model).** The kind of AI that powers Claude and Codex: a model trained on huge amounts of text to generate and reason over language and code.

**Main.** The default primary branch name, holding your stable code. Replaced the older name `master`.

**MCP (Model Context Protocol).** An open standard letting AI agents connect to external tools and data sources (calendars, databases, other services) through a common interface. Both tools support it.

**Merge.** Combining the changes from one branch into another.

**Merge conflict.** When git cannot automatically combine two changes because they touch the same lines; you resolve it by editing the final result.

**MR (Merge Request).** GitLab's name for what GitHub calls a pull request.

**Native installer.** The recommended way to install Claude Code on Windows: a self-contained program that needs no extra runtime and auto-updates.

**Node.js / npm (Node Package Manager).** Node.js is a runtime for running JavaScript outside a browser; npm installs JavaScript packages. Relevant only if you install the tools the npm way or build JavaScript projects.

**Origin.** The default nickname for your main remote, the GitHub copy of your repo.

**PAT (Personal Access Token).** A password-like string used to authenticate to GitHub over HTTPS. SSH keys are the alternative and usually preferred.

**PATH.** The list of folders your terminal searches when you type a command. When a freshly installed command is "not found," it usually just means the terminal has not reloaded its PATH yet; reopen the terminal.

**Plan mode.** A mode where the agent proposes its full approach before writing code, so you can approve or correct it first. Use it constantly.

**PowerShell.** Windows' built-in command shell, the default in Windows Terminal. Prompt starts with `PS C:\`.

**Prompt.** What you type to the AI: your request and any context you give it. Also, confusingly, the little `PS C:\>` marker in your terminal.

**Pull.** Downloading changes from a remote and merging them into your current branch (a `fetch` plus a `merge`).

**Pull request (PR).** A proposal to merge one branch into another, with a description and a place for review. The unit of collaboration on GitHub.

**Push.** Uploading your local commits to the remote (GitHub).

**Rebase.** Replaying your commits on top of another branch's latest state, producing a linear, cleaner history instead of a merge.

**Remote.** A cloud copy of your repo that your local repo is linked to. Default remote: `origin`.

**Release.** A packaged, published version of your software, usually built from a tag.

**Repo (repository).** A project folder that git is tracking, containing your code and its full history.

**Reset.** Moving the branch pointer to a different commit. Powerful and capable of discarding work; use carefully.

**Revert.** Creating a new commit that undoes a previous one. The safe way to back out a change because it does not rewrite history.

**Sandbox (Codex).** The restriction on what Codex can technically do: read-only, write-within-project, or fully open. Separate from the approval policy.

**settings.json (Claude Code).** Claude Code's settings file. Exists at user level (`C:\Users\<yourname>\.claude\settings.json`), project level (`.claude\settings.json`), and local level (`.claude\settings.local.json`), with narrower levels overriding broader ones.

**Shell.** The program inside a terminal that interprets your commands (PowerShell, Command Prompt, or Git Bash on Windows).

**Squash.** Collapsing multiple commits into one, often when merging a PR, for a tidy history.

**SSH (Secure Shell).** A protocol for secure connections. An SSH key pair lets your machine authenticate to GitHub without passwords. The recommended authentication method.

**Stage.** Marking specific changes to be included in the next commit (`git add`).

**Stash.** Temporarily shelving uncommitted changes so you can switch context, retrievable later (`git stash`).

**Switch / checkout.** Moving to a different branch. `git switch` is the modern command; `git checkout` is the older one that does the same thing and more.

**Surface.** One of the ways a tool is delivered: CLI, IDE extension, or cloud/web.

**Tag.** A label marking a specific commit, usually a version number like `v1.0.0`.

**Terminal.** The window where you type commands. On Windows, Windows Terminal.

**Token.** The unit of text an LLM processes, roughly a word-piece. Context windows and pricing are measured in tokens.

**TOML (Tom's Obvious, Minimal Language).** The simple configuration file format Codex uses for `config.toml`.

**TUI (Text User Interface).** A more interactive terminal interface with menus and panels, as opposed to a bare command line. Both agents present a TUI when running.

**URL (Uniform Resource Locator).** A web address. The `sessionUrl` setting in Part 6 controls whether Claude adds a link back to its session.

**Version control.** The practice, and the tools (git), of tracking every change to your code over time so nothing is ever truly lost and any state can be recovered.

**Vibe coding.** Describing what you want in natural language and letting an AI agent generate and revise the code, focusing on outcomes rather than hand-writing every line. Made safe by version control, tests, and small reviewable steps.

**Windows Terminal.** The modern Windows terminal app that hosts PowerShell, Command Prompt, and other shells in tabs.

**winget (Windows Package Manager).** Windows' built-in tool for installing software from the command line.

**WSL (Windows Subsystem for Linux).** A full Linux environment running inside Windows. Optional here, and only a fallback if Codex's native Windows sandbox gives you trouble.

---

## Part 10: A first-week path

If it helps to have a concrete on-ramp rather than the whole map at once:

1. Do Part 2 in full: Git for Windows, identity, GitHub CLI, authenticated as `nyxlocke`. Confirm each step with the check command given.
2. Install Claude Code (Part 3) and Codex (Part 4). Run `claude --version` and `codex --version` to confirm.
3. Set the attribution settings from Part 6 for both tools and Claude Desktop, right away, so you never generate an attributed commit.
4. Make one throwaway project (`~/dev/sandbox`), run `git init`, then `gh repo create nyxlocke/sandbox --private --source=. --remote=origin`, and ask Claude to build something tiny end to end: a plan, a branch, a commit, a push, a pull request. Do it once with Claude, once with Codex, so the loop is in your hands.
5. Only then start the real thing, with a `docs\SPEC.md` and plan mode.

The tools are powerful and the safety net is real. Move in small steps, read the diffs, keep the tests honest, and git will forgive almost anything you do wrong along the way.