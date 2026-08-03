---
id: i5-shipping-a-desktop-app
title: Shipping something someone can install
type: section
track: I
order: 50
verified: 2026-08-02
volatility: quarterly
answer: >
  You ship a desktop program as an installer file people download and run. An
  unsigned one triggers a blue "Windows protected your PC" box that needs two
  clicks to get past, and the certificate that removes it costs money every year.
owns:
  - installers
  - code signing at a glance
  - SmartScreen
see_also:
  - i3-builds-and-artifacts
  - i4-releases-and-versioning
  - i1-what-deployment-means
  - i2-servers-and-hosting
keywords:
  - smartscreen
  - windows protected your pc
  - code signing certificate
  - msi
  - unsigned installer
  - publisher unknown
  - antivirus false positive
---

## More

A web app is deployed to a machine you rent. A desktop app is deployed to machines you will
never see, one download at a time. What you hand over is an **installer**: a file that
copies your program into place, adds a Start menu entry, and registers itself so Windows can
uninstall it later.

Producing one is the easy half. This app is built with Tauri, and one command turns the
built front end and the compiled Rust binary into an `.msi` package under
`src-tauri\target\release\bundle\`. That file is the artifact
([i3](#i3-builds-and-artifacts)), and the version number stamped into it comes from
`tauri.conf.json` ([i4](#i4-releases-and-versioning)).

The hard half is what happens on the other person's machine. Windows checks whether the
installer carries a digital signature naming a verified publisher. This one does not, so
Nyx, or anyone else you send it to, gets a full-screen blue box:

```text
Windows protected your PC

Microsoft Defender SmartScreen prevented an unrecognized app from
starting. Running this app might put your PC at risk.
```

There is one visible button, "Don't run." The way through is the small **More info** link,
which reveals a **Run anyway** button. Two clicks, and the installer proceeds normally.

Nothing is wrong with the file. The warning means "nobody has vouched for this publisher,"
and vouching is a paid service. A code signing certificate runs a few hundred dollars a
year, requires proving your identity to the issuer, and since 2023 requires the private key
to sit on a physical hardware token or a cloud service that holds it for you.

The honest tradeoff: for something you hand to a handful of people who already trust you,
ship it unsigned and tell them in advance exactly which two clicks to make. Buy the
certificate when strangers start downloading it, or when the person on the other end works
somewhere that blocks unsigned software outright.

## Full

### What an installer actually does

Four things, in this order:

1. Copies the program files, usually into `C:\Program Files\<AppName>\` for everyone or
   `C:\Users\<yourname>\AppData\Local\<AppName>\` for the current user only.
2. Creates the Start menu shortcut, and a desktop shortcut if you asked for one.
3. Writes a registry entry so the app appears in Settings under Installed apps, with a
   working Uninstall button.
4. Records the version number, publisher name, and icon that Windows shows in that list.

Skipping the installer entirely is a real option. A `.zip` holding the `.exe` works, runs
from anywhere, needs no administrator rights, and leaves nothing behind. It is also the
format people are most suspicious of, and it gets no Start menu entry.

### The formats, and which to pick

| Format | Made by | Good for | Cost |
|---|---|---|---|
| `.msi` | Windows Installer, which Tauri produces by default | Anything corporate, because administrators can deploy it centrally | Free |
| `.exe` setup | A tool like NSIS (Nullsoft Scriptable Install System) | More control over the install screens | Free |
| Portable `.zip` | A build script | Testing, and users without admin rights | Free |
| Microsoft Store | Store submission | Reach, and no SmartScreen warning ever | A one-time developer fee, plus review |
| `winget` | A manifest sent to Microsoft's package repository | Users who install from the terminal | Free, needs a signed installer |

The Store and `winget` rows are the two paths where somebody else's trust carries yours.
Both take longer than a weekend and both are worth knowing exist.

### Why signing costs what it costs

A signature is a claim by a Certificate Authority, meaning the company issuing the
certificate, that they checked who you are. Two grades:

- **OV (Organization Validation).** They verify a registered business or, for some issuers,
  an individual. Roughly two to four hundred dollars a year.
- **EV (Extended Validation).** A heavier identity check. Historically it bought an
  immediate good reputation with SmartScreen where the cheaper grade did not. Prices run
  higher and the reputation behavior has changed more than once, so confirm the current
  situation with the issuer rather than trusting a blog post.

Since June 2023 the private key can no longer live in a file on your disk. It has to be held
on a hardware token mailed to you, or in an HSM (Hardware Security Module) run as a cloud
service. That single rule ended the era of cheap certificates and is why older guides
quoting seventy dollars are useless now.

Microsoft also sells a signing service of its own, billed monthly at a price closer to a
streaming subscription than to a certificate, with an option for individual developers who
can prove a few years of verifiable identity history. That is the cheapest legitimate route
onto the signed side at the moment. Check the current terms before planning around it,
because this specific corner of the market has changed every year since 2023.

### Reputation, and why signing may not silence the warning immediately

SmartScreen does not ask "is this signed." It asks "have I seen this publisher's software
succeed on enough machines." A brand new certificate starts with no history, so an OV-signed
installer can still show the blue box until enough people have installed it without
incident. The warning does change: it names your publisher instead of saying "unknown," and
support conversations get shorter.

Antivirus false positives are the same problem wearing a different hat. A freshly compiled
binary that nobody has seen before, especially one written in Rust or Go, gets flagged by
smaller scanners on reputation alone. Every vendor has a false positive submission form,
they generally respond within a few days, and this is a normal part of shipping rather than
evidence that your program does something wrong.

### The updater key is a different key

Tauri and most desktop frameworks can update themselves, and the update mechanism has its
own signing key so the app can refuse an update that did not come from you. That key you
generate yourself, for free, in about ten seconds. Keep the private half out of the
repository ([g6](#g6-secrets-and-what-never-to-commit)).

Two separate systems, both called signing. The free one proves an update came from the same
author as the install. The expensive one proves to Windows who that author is.

### What to write in the download instructions

Tell people what will happen before it happens. Anyone who hits an unexplained security
warning stops, and they are right to.

```text
Windows will show a blue "Windows protected your PC" screen, because this
installer is not signed with a paid certificate. Click "More info", then
"Run anyway". The publisher will show as unknown. That is expected.
```

Publishing the file's checksum next to the download is the honest supplement: it lets a
careful person confirm the file they got is the file you built. It also costs you one line
in the release notes.

### The bottom line for this app

Unsigned, distributed as an `.msi` from a GitHub release, with the warning documented in the
readme. The certificate is a real cost with a real benefit and it is not the first thing to
buy. When the number of people installing this gets past the number you can text, that
calculation flips.
