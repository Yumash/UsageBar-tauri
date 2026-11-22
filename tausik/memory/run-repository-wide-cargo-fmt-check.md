---
slug: run-repository-wide-cargo-fmt-check
title: "Run repository-wide cargo fmt check"
type: dead_end
tags:
  - rustfmt
  - upstream
task: expand-tray-controls
edges: []
---

Approach: Run repository-wide cargo fmt check
Reason: The imported upstream baseline contains unrelated formatting drift outside this task; verify the two edited Rust files directly instead of rewriting unrelated files.
