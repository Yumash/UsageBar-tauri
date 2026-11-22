---
slug: run-cargo-fmt-check-for-the-whole-inherited-rust-tree
title: "Run cargo fmt --check for the whole inherited Rust tree"
type: dead_end
tags: []
task: guided-windows-installer
edges: []
---

Approach: Run cargo fmt --check for the whole inherited Rust tree
Reason: The upstream tree contains pre-existing rustfmt drift across unrelated files; formatting it would create a large out-of-scope diff. Verify the changed lib.rs with skip_children instead.
