---
id: port:long-beach
type: port
scale: national
market: us-pacific-san-pedro
tier: T1
---

# Long Beach container gateway

DIM-01 is deep-draft channel adequacy: channel depth versus the ~50 ft all-tide
draft a fully-laden ultra-large container vessel (ULCV) needs. Score transform is
`clamp(depth_ft - 43, 0, 10)` so 50 ft maps to the 7.0 baseline. TEU is recorded
as cited context only and is **not** scored, to avoid conflating port size with
deficiency.

score: DIM-01 | 10
quantity: 53 | feet-channel-depth-mlw | implemented | usace-channel-depth-2023 | average
quantity: 8 | million-teu-2023 | implemented | port-teu-2023 | average