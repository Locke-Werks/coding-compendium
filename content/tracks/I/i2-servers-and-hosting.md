---
id: i2-servers-and-hosting
title: Servers, hosting, and domains
type: section
track: I
order: 20
verified: 2026-08-02
volatility: quarterly
answer: >
  A server is a computer that stays on and answers requests, hosting is renting
  one, and a domain is the name people type. DNS (Domain Name System) is the
  lookup that turns that name into the address of the machine.
owns:
  - server
  - hosting
  - domain
  - DNS at a glance
see_also:
  - i1-what-deployment-means
  - j6-web-basics
  - c6-ports-and-localhost
  - i5-shipping-a-desktop-app
keywords:
  - web host
  - vps
  - domain name
  - nameservers
  - static hosting
  - serverless
  - where do i put my website
---

## More

Three words get used interchangeably and mean different things.

A **server** is a computer that stays powered on and answers requests from other computers.
The same word also means the program doing the answering, so "the server crashed" might be a
machine or a process. Context settles it, and the process is the usual meaning.

**Hosting** is renting server capacity from a company that keeps the machine running,
patched, and connected. You are renting uptime and someone else's electricity bill.

A **domain** is the name people type, like `example.com`. You rent it from a registrar,
yearly. The name has to be translated into the numeric address of a machine, and DNS (Domain
Name System) is the worldwide lookup table that does the translating. Domain and hosting are
separate purchases from separate companies, which surprises everyone once.

Hosting comes in a ladder, from least work to most:

- **Static hosting.** You upload finished files and the host serves them. No program of
  yours runs. Cheapest and hardest to break.
- **Platform hosting.** You connect a repository, the platform builds and runs your program
  for you. This is where most web apps live now.
- **A virtual machine.** You rent a full Linux machine and everything on it is your job:
  the runtime, the web server, the updates, the firewall.
- **Serverless functions.** Your code runs only when a request arrives and you pay per
  request. There are still servers. You are not renting one full time.

Pick the highest rung that does what you need. Every rung down adds a job that is yours
forever, and none of those jobs are the thing you set out to build.

This app rents nothing. It is a desktop program that runs on the machine it was installed
on, with no server, no domain, and no hosting bill ([i5](#i5-shipping-a-desktop-app)).

## Full

### What you are actually buying at each rung

| Rung | You provide | They provide | Typical use |
|---|---|---|---|
| Static host | Built files | Storage, delivery, certificates | Documentation, marketing pages, a built front end |
| Platform host | A repository | Build, run, restart, logs, certificates | An app with a back end |
| Virtual machine | Everything above the hardware | A machine and a network connection | Anything unusual, or cost control at scale |
| Serverless | One function at a time | Everything else, on demand | Small jobs, webhooks, scheduled tasks |
| Managed database | A schema | Backups, upgrades, replication | Any real data |

Names move around and pricing changes, so treat any specific brand you read about as an
example rather than a recommendation. The rung matters. The logo does not.

### Domains, in the order you meet them

1. **Register the name.** A registrar rents you `example.com` for a year at a time. Renew it
   or you lose it, and a lapsed domain is bought within minutes by someone who will sell it
   back for more.
2. **Point it somewhere.** DNS records are the instructions. An `A` record points a name at
   a numeric address. A `CNAME` record points a name at another name, which is what almost
   every host asks you to create. Your host tells you exactly what to enter.
3. **Wait.** DNS answers are cached all over the internet, and the cache lifetime is set by
   a number called the time-to-live. A change can take minutes or a day. "It works on my
   phone but not my laptop" during that window is normal and needs no fix.

### The certificate, and why the browser cares

HTTPS (Hypertext Transfer Protocol Secure) is the plain protocol with encryption wrapped
around it, using TLS (Transport Layer Security). The wrapping needs a certificate, which is a
file proving your site is allowed to answer for that domain.

Certificates are free now, issued by Let's Encrypt, and every host on the top three rungs
gets and renews yours automatically. Two failures you will meet anyway:

- **Expired certificate.** The browser shows a full-page warning. Certificates last ninety
  days and renew on their own, so an expired one means the automation broke, not that you
  did something.
- **Name mismatch.** The certificate covers `example.com` and someone visited
  `www.example.com`. Add the second name at the host and it reissues.

Note this is unrelated to the certificate that signs a Windows installer. Same word, two
different systems, and [i5](#i5-shipping-a-desktop-app) covers the other one.

### The words you will see and can safely skim past

- **CDN (Content Delivery Network).** Copies of your files kept in many cities so the one
  nearest the visitor answers. Static hosts include one by default.
- **Load balancer.** Splits incoming requests across several machines. You need one when
  one machine is not enough, and not before.
- **Container.** Your program plus its runtime, packaged so it runs identically anywhere. A
  `Dockerfile` is the recipe ([Dockerfile](#dockerfile)).
- **Reverse proxy.** A program sitting in front of yours, handling encryption and passing
  requests along. `nginx` and Caddy are the two you will see named.
- **Uptime.** The share of time the thing answers. Marketed in nines. Nobody with one
  machine has four of them.

### What it costs, in shape rather than in dollars

Static hosting for a small site is free at every major provider. Platform hosting has a free
tier that sleeps when idle and a paid tier starting at a few dollars a month. A small virtual
machine is a similar price with more work attached. A domain is roughly the price of a
sandwich per year unless the name is desirable, and a managed database is usually the
largest line on the bill.

Prices and free tiers change often enough that any number written here would be wrong within
a year. The provider's own pricing page is the source of truth, and the number that actually
matters is the one for the tier above free, because that is what you pay the day something
succeeds.

### When someone says "just spin up a server"

They mean the third rung, and they are describing about four hours of work: create the
machine, connect with SSH (Secure Shell), install the runtime, install a web server,
configure it, get a certificate, open the right ports, set up a service so your program
restarts after a reboot, and arrange backups. Every one of those is learnable and none of
them is your project. Start higher up the ladder and come down only when something forces
you to.
