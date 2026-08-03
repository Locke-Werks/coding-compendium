---
id: a3-the-three-pieces
title: The agent, the repo, and the cloud copy
type: section
track: A
order: 30
verified: 2026-08-02
volatility: low
answer: >
  Three things are moving: an agent running on your machine that edits files and
  runs commands, a project folder that git tracks, and a copy of that folder on
  GitHub. Nothing reaches GitHub until a command sends it.
owns:
  - the three-piece mental model
see_also:
  - e1-what-an-agent-is
  - d1-what-git-actually-stores
  - d2-repo-remote-clone-origin
  - d3-the-three-places
keywords:
  - mental model
  - how does this fit together
  - where does my code live
  - agent vs model
  - what is a repo
  - local vs cloud
---

## More

Three things are in play, and most confusion in the first month comes from mixing up which
one you are talking about.

**1. The agent.** A program running on your machine, in your terminal. It reads your files,
writes new ones, edits existing ones, and runs commands. That is the whole difference between
it and a chat window, and it is the whole risk. Claude Code and Codex are both this.
[e1](#e1-what-an-agent-is) covers what it can reach.

**2. The folder.** Your code is files in a folder, for example
`C:\Users\<yourname>\dev\my-project`. Once git is tracking that folder it is called a
**repository**, or repo, and git keeps every recorded version in a hidden `.git` folder
inside it. The agent works here, on the real files, the same ones you can open in Notepad.
[d1](#d1-what-git-actually-stores) covers what git puts in there.

**3. The copy on GitHub.** GitHub is a website holding a copy of that folder and its history.
It is your backup, the address other machines can reach, and where pull requests live. Your
account is `nyxlocke`. [d2](#d2-repo-remote-clone-origin) covers the vocabulary.

The connections matter more than the pieces:

- **Agent to folder: immediate.** When the agent edits a file, the file on your disk changed
  at that moment. There is no save step and no undo button. Git is the undo button.
- **Folder to GitHub: only when told.** `push` sends up, `pull` and `fetch` bring down.
  Between those commands, work in your folder exists on exactly one computer.

There is a fourth thing that is deliberately not one of the three. The model itself runs on
Anthropic's or OpenAI's servers. The agent on your machine sends it text and gets text back.
The model never touches your disk. The agent does. Whenever a file changed, the agent changed
it, and the agent is the thing you granted permission to.

## Full

### Drawn once

```text
  your machine                                     the internet
  ------------                                     ------------

  agent  ---->  project folder  ---git push--->    github.com/nyxlocke/my-project
    |           (.git holds the history)                    |
    |                                                       |
    |                                              <--git pull---
    |
    +---->  the model, on the vendor's servers
            (text goes out, text comes back, no files)
```

Three arrows, and each one is a different kind of thing. The agent edits files directly. Git
moves versions of the folder to and from GitHub when you run a command. The model exchanges
text with the agent and reaches nothing else.

### Which piece is the problem

Most "something is wrong" moments resolve by naming the piece first.

- **The agent changed something and now the app is broken.** That is the folder. Look at
  what actually changed with `git status` and `git diff`, covered in
  [d3](#d3-the-three-places).
- **My change is not on GitHub.** That is the folder-to-GitHub link, and it is nearly always
  one of two things: committed but never pushed, or never committed.
  [d3](#d3-the-three-places) tells you which.
- **GitHub has a file my folder does not.** The copies drifted apart and yours is behind.
  Pull. [d2](#d2-repo-remote-clone-origin).
- **The agent cannot see a file I know exists.** It is looking outside the folder you started
  it in. Close it, `cd` into the project root, start it again.
- **The agent says it made a change and the file looks the same.** Two copies of the project
  on disk, and it edited the other one. Run `pwd` in the terminal where the agent is running
  to see the folder it is actually sitting in.

### Two complete copies, not a client and a server

The GitHub copy is not the master and your folder is not a cache of it. Each side holds the
entire history and either can rebuild the other, which is why deleting one leaves the other
untouched. [d1](#d1-what-git-actually-stores) makes that case in detail.

The practical consequence is the one worth internalizing now: a folder you have never pushed
is backed up by nothing.

### What the agent can actually reach

Your project folder is where it works, and it is not a wall: the agent runs as you and can
reach anything your Windows account can reach ([e1](#e1-what-an-agent-is)).

Two consequences:

- Start the agent from the project root, every time. It anchors everything it does.
- The things worth never approving without looking are listed in
  [e11](#e11-what-to-never-let-an-agent-do). Force pushes and bulk deletes are on it.

### Inside the folder there are three more places

Once git is tracking a folder, a file you edited can be in one of three states, and "did I
save it" has three different answers depending on which. That is its own model and
[d3](#d3-the-three-places) owns it. Read that one early, because every undo command in git
makes sense only once you know which of the three it moves.

### The cloud versions, which you do not need yet

Both tools have a version that runs the agent on the vendor's machine against a copy of your
repo pulled from GitHub. Same three pieces. The agent has moved off your laptop, so it sees
what GitHub has rather than what your folder has, which is a real difference the first time
you use one. Local terminal work is where to start.
