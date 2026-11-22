---
slug: pipeline-a-powershell-foreach-block-directly-into-sort
title: "Pipeline a PowerShell foreach block directly into Sort-Object"
type: dead_end
tags:
  - audit
  - powershell
task: stabilize-reset-tests
edges: []
---

Approach: Pipeline a PowerShell foreach block directly into Sort-Object
Reason: This PowerShell parser rejects a pipe immediately after the foreach statement; collect the loop output first.
