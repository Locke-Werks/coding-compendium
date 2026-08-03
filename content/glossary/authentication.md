---
id: "authentication"
title: "Authentication"
type: glossary
verified: "2026-08-02"
volatility: low
term: "Authentication"
aliases: ["authn", "auth", "authenticate", "authenticated", "sign in"]
short_def: "Proving who you are. The login step, which happens before anything asks what you are allowed to do."
not_to_be_confused_with: ["authorization"]
canonical_section: "b5-ssh-vs-https"
---

Authentication answers who are you. Authorization answers what may you do. They fail differently and the error messages say so: a failed login is a 401, a logged-in user reaching for something forbidden is a 403.
