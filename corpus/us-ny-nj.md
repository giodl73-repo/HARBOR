---
id: port:ny-nj
type: port
scale: national
market: us-atlantic-north
tier: T1
---

# New York / New Jersey container gateway

DIM-01 is deep-draft channel adequacy: channel depth versus the ~50 ft all-tide
draft a fully-laden ultra-large container vessel (ULCV) needs. Score transform is
`clamp(depth_ft - 43, 0, 10)` so 50 ft maps to the 7.0 baseline. TEU is recorded
as cited context only and is **not** scored, to avoid conflating port size with
deficiency.

score: DIM-01 | 7
quantity: 50 | feet-channel-depth-mlw | implemented | usace-channel-depth-2023 | average
quantity: 7.8 | million-teu-2023 | implemented | port-teu-2023 | average