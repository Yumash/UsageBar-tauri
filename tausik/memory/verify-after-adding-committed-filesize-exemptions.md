---
slug: verify-after-adding-committed-filesize-exemptions
title: "Verify after adding committed filesize exemptions"
type: dead_end
tags:
  - config
  - verify
task: stabilize-reset-tests
edges: []
---

Approach: Verify after adding committed filesize exemptions
Reason: Presence of tausik/gates.json activates the test-dedupe ratchet, which refuses without an explicit measured baseline. The verifier measured zero duplicate groups and zero covered tests.
