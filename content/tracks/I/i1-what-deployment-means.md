---
id: i1-what-deployment-means
title: What deployment actually means
type: section
track: I
order: 10
verified: 2026-08-02
volatility: low
answer: >
  Deployment is putting a working copy of your program on a machine other people
  can reach and starting it there. "It works locally" proves nothing, because
  your machine has a runtime, a database, and settings the target does not.
owns:
  - deployment
  - environments
  - local vs production
see_also:
  - i2-servers-and-hosting
  - i3-builds-and-artifacts
  - i5-shipping-a-desktop-app
  - h5-ci-cd
  - g5-environment-variables
keywords:
  - deploy
  - push to production
  - works on my machine
  - staging
  - go live
  - ship it
  - prod
---

## More

Where the copy goes depends on the shape of the project: a rented Linux machine for a web
app, a file host for a static site, an installer people download for a desktop program
([i5](#i5-shipping-a-desktop-app)).

The word covers three steps that always happen in this order.

1. **Build.** Turn the files you edit into the files that run. What comes out is the
   artifact, and [i3](#i3-builds-and-artifacts) owns that step.
2. **Place.** Copy the artifact onto the target machine, or hand it to a service that does
   the copying for you.
3. **Start and check.** Run it there, then confirm from outside that it answers.

**Environments** are named copies of the same program. *Local* is your machine. *Production*
is the copy real people use. *Staging* is a rehearsal copy that is supposed to match
production and never quite does. These are conventions rather than technology. Production is
production because breaking it has consequences, not because the hardware is special.

Now the part that costs people their first weekend. "It works locally" is not evidence that
it works anywhere else. Your machine carries a state nobody else has: the Node version you
installed months ago, environment variables you set once and forgot
([g5](#g5-environment-variables)), a database holding your test data, and a Windows
filesystem that treats `Button.tsx` and `button.tsx` as the same file. Linux does not. That
last one alone breaks more first deploys than any other single cause.

Before you deploy anything for the first time, decide how you would undo it. Knowing the
way back turns a deploy from a commitment into an experiment, and experiments are the only
kind of change worth making at speed.

## Full

### The five things that differ between your machine and the target

Work down this list before the first deploy. Almost every "it worked locally" failure is
one of them.

1. **Runtime version.** You have Node 22 and the host defaults to Node 18, or you have
   Python 3.13 and the container ships 3.9. Pin the version in the project rather than
   hoping. Most hosts read it from `package.json`, a `.nvmrc`, or `pyproject.toml`.
2. **Environment variables and secrets.** Your `.env` file is on your disk and is not
   committed, correctly ([g6](#g6-secrets-and-what-never-to-commit)). The target has no
   copy, so every key has to be entered again in the host's settings panel. A missing one
   usually shows up as `undefined` in a connection string rather than as a clear error.
3. **The database.** Yours has the rows you typed in while testing. Production starts
   empty, and its schema only matches yours if a migration ran
   ([j5](#j5-databases-at-a-glance)).
4. **The filesystem.** Windows is case-insensitive, Linux is not. `import Button from
   './components/button'` works on your machine and fails on the server with
   `Cannot find module`. Paths use forward slashes there, always
   ([c7](#c7-files-folders-and-paths)).
5. **The network.** Locally everything talks to `localhost` and no firewall is involved. On
   the target, the port has to be the one the host expects, usually supplied to your program
   as an environment variable named `PORT` ([c6](#c6-ports-and-localhost)).

### What a deploy actually consists of

For a web app, most hosting services collapse the whole thing into "push to the `main`
branch." Underneath, five things happen, and knowing them is how you read the log when one
fails:

```text
1. clone the repository at the commit you pushed
2. install dependencies from the lockfile
3. run the build command
4. start the process with the start command
5. wait for it to answer a request, then switch traffic over
```

Step 2 uses the lockfile, not the manifest, which is exactly why the lockfile is committed
([g3](#g3-lockfiles)). Step 5 is why a broken deploy often leaves the old version running:
the new one never answered, so nothing switched.

### Rolling back

Every deploy needs a way back before it needs anything else. The three that exist:

- **Redeploy the previous commit.** Works everywhere, costs one build.
- **The host's rollback button.** Most services keep the last several builds and can switch
  back in seconds. Find this button before you need it.
- **A revert commit.** `git revert` makes a new commit that undoes the old one and then
  deploys normally. Safe, and it leaves a record ([d10](#d10-undo-everything)).

The one thing that does not roll back is a database migration that deleted a column. Code
is reversible. Data is not.

### "Deploy" means different things per project

| Project shape | Deploying means | Who runs it |
|---|---|---|
| Static site | Copying HTML (Hypertext Markup Language), styles, and images to a file host | The host serves files |
| Web app with a server | Running your program as a process on a rented machine | A host or your own server |
| Command-line tool | Publishing to a registry so people can install it | Whoever installs it |
| Desktop app | Producing an installer people download and run | Your user's own machine |
| Library | Publishing a version to npm, crates.io, or PyPI | Whoever depends on it |

This app is the fourth row. Nothing about it is deployed to a server, because there is no
server: it runs entirely on the machine it was installed on. That is a deliberate choice
and [i5](#i5-shipping-a-desktop-app) covers what shipping it involves instead.

### The first-deploy checklist

- The project builds from a clean clone, not from your folder with its accumulated state.
- Every environment variable the code reads is set on the target.
- Secrets are in the host's settings panel and not in the repository.
- You know the exact command the host runs to start the program.
- You know where the logs are and how to read them ([f4](#f4-logs)).
- You have rolled back once on purpose, while nothing was wrong, so you know it works.

Do the last one. A rollback you have never tested is a plan, not a safety net.
