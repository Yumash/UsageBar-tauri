---
slug: run-frontend-syntax-and-rustfmt-checks-from-src-tauri-with
title: "Run frontend syntax and rustfmt checks from src-tauri with repository-root paths"
type: dead_end
tags:
  - path
  - verify
task: russian-localization
edges: []
---

Approach: Run frontend syntax and rustfmt checks from src-tauri with repository-root paths
Reason: The paths were resolved relative to src-tauri and did not exist there; cargo tests still ran and passed. Re-run path-sensitive checks from the repository root.
