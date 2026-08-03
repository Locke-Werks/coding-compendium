---
id: "force-push"
title: "Force push"
type: glossary
verified: "2026-08-02"
volatility: low
term: "Force push"
aliases: ["force-push", "force pushing", "forced push", "push force"]
short_def: "Overwriting the remote's history with your own, discarding commits on the remote that you do not have. The one push that can destroy work."
not_to_be_confused_with: ["push"]
canonical_section: "e11-what-to-never-let-an-agent-do"
danger: "A force push replaces the remote branch with yours and deletes any commits on it that you do not have locally. Use the with-lease variant, which refuses when the remote moved since you last looked."
---
