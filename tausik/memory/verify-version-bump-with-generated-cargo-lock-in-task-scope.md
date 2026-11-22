---
slug: verify-version-bump-with-generated-cargo-lock-in-task-scope
title: "Verify version bump with generated Cargo.lock in task scope"
type: dead_end
tags:
  - filesize
  - generated
  - tausik
task: repair-zai-identity
edges: []
---

Approach: Verify version bump with generated Cargo.lock in task scope
Reason: The filesize gate treated the generated dependency lockfile as authored source; add only src-tauri/Cargo.lock to the exact-path inherited/generated exemption list.
