---
slug: use-nsis-lang-russian-and-lang-simpchinese-constants-inside
title: "Use NSIS LANG_RUSSIAN and LANG_SIMPCHINESE constants inside the early installer hook"
type: dead_end
tags: []
task: guided-windows-installer
edges: []
---

Approach: Use NSIS LANG_RUSSIAN and LANG_SIMPCHINESE constants inside the early installer hook
Reason: Tauri includes installer hooks before MUI language declarations, so makensis treats those constants as unknown; use stable Windows LANGID values at runtime instead.
