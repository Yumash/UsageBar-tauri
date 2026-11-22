---
slug: verify-tray-task-before-registering-inherited-lib-rs
title: "Verify tray task before registering inherited lib.rs exemption"
type: dead_end
tags:
  - filesize
  - tausik
  - upstream
task: expand-tray-controls
edges: []
---

Approach: Verify tray task before registering inherited lib.rs exemption
Reason: The exact-path TAUSIK filesize exemption list omitted src-tauri/src/lib.rs even though the imported upstream module already exceeded 500 lines.
