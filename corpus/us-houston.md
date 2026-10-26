---
id: port:houston
type: port
scale: national
market: us-gulf
tier: T2
---

# Houston container gateway

DIM-01 is deep-draft channel adequacy: channel depth versus the ~50 ft all-tide
draft a fully-laden ultra-large container vessel (ULCV) needs. Score transform is
`clamp(depth_ft - 43, 0, 10)` so 50 ft maps to the 7.0 baseline. TEU is recorded
as cited context only and is **not** scored, to avoid conflating port size with
deficiency.

score: DIM-01 | 2
quantity: 45 | feet-channel-depth-mlw | source-needed | usace-channel-depth-2023 | average
quantity: 3.8 | million-teu-2023 | source-needed | port-teu-2023 | average
Source-custody audit (2026-10-26): these retained quantities lack original
source bytes and record-level locators. Values are unverified historical context;
DIM-01 remains an illustrative score, not an observed navigation finding.
See `data/source-audit.md`.
