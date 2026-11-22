---
slug: tausik-standard-scoped-verify-for-providers-rs
title: "TAUSIK standard scoped verify for providers.rs"
type: dead_end
tags:
  - legacy
  - upstream
  - verify
task: stabilize-reset-tests
edges: []
---

Approach: TAUSIK standard scoped verify for providers.rs
Reason: The inherited upstream providers.rs is 3671 lines and fails the framework 500-line filesize gate before tests; splitting the whole provider module is outside this focused fix and would create an unsafe, noisy PR.
