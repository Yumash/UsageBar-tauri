# TAUSIK project configuration

`gates.json` contains exact-path exemptions for source files that already
exceeded TAUSIK's 500-line limit in the imported UsageBar v0.0.28 baseline.
The list is intentionally file-specific: it does not exempt directories or new
files, and it does not permit additional files to cross the limit.

Feature work should avoid making these modules larger where practical. Splits
belong in dedicated refactoring changes so upstream contributions do not mix
mechanical movement with behavioral changes.
