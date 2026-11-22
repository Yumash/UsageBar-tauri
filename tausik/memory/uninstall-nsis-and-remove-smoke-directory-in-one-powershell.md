---
slug: uninstall-nsis-and-remove-smoke-directory-in-one-powershell
title: "Uninstall NSIS and remove smoke directory in one PowerShell command"
type: dead_end
tags:
  - cleanup
  - windows
task: verify-cross-platform-release
edges: []
---

Approach: Uninstall NSIS and remove smoke directory in one PowerShell command
Reason: The execution safety policy rejected a combined recursive cleanup command; split uninstall and deletion after the absolute target was verified under src-tauri/target.
