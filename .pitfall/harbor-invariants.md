# HARBOR Invariants

These entries summarize properties that must remain true for HARBOR corpus
entries, score artifacts, tier/SLA checks, gap runs, findings, review gates, and
future reuse.

## HARBOR-I-01: Scale And Source Tags Are Non-Optional For Promotion

**Status:** VERIFIED

**Claim:** Every promoted corpus entry has stable identity, `scale`, `market`,
and source-labelled quantities.

**Why it matters:** Unscaled or uncited port data can make local, regional,
national, or international gateway claims look comparable when they are not.

**Enforcement:** Corpus validation preserves scale/source fields, rejects or
holds unsafe rows, and gap analysis filters by requested scale.

**Evidence:** `corpus/SCHEMA.md`, `data/sources.md`,
`crates/harbor-corpus/src/lib.rs`, `crates/harbor-gap/src/lib.rs`, and
`docs/vtrace/VERIFICATION.md`.

## HARBOR-I-02: Empty Dimensions Stay Empty

**Status:** VERIFIED

**Claim:** Empty dimensions are emitted as `EmptyRegion`, not coerced to zero or
hidden as success.

**Why it matters:** Missing DIM-02..DIM-13 evidence in the first corpus could be
misread as low scores or as adequacy if the output collapsed those dimensions.

**Enforcement:** The national gap replay emits explicit empty regions for
unassessed dimensions.

**Evidence:** `cargo run -p harbor-cli -- gap --input corpus --scale national`,
`docs/findings/2026-06-deep-draft-channel-tail.md`,
`crates/harbor-gap/src/lib.rs`, and `docs/vtrace/VERIFICATION.md`.

## HARBOR-I-03: Tail Gaps Stay Separate From Broad Alarms

**Status:** VERIFIED

**Claim:** `TailRegion` and broad `UnderServedRegion` outputs remain separate.

**Why it matters:** A broad min-based alarm can catch a real shortfall while
misattributing it to every scored gateway.

**Enforcement:** The DIM-01 replay localizes the tail to Houston and Savannah
while preserving the broader alarm as a separate output.

**Evidence:** `docs/findings/2026-06-deep-draft-channel-tail.md`,
`crates/harbor-gap/src/lib.rs`, `docs/vtrace/VERIFICATION.md`, and
`cargo test --workspace --locked`.

## HARBOR-I-04: Port Dimensions Stay Separate

**Status:** VERIFIED

**Claim:** Channel depth, throughput, dwell, hinterland, resilience,
competition, environment, and cost dimensions remain separately scored.

**Why it matters:** A channel-depth finding should not imply throughput,
terminal, landside, environmental, or cost adequacy.

**Enforcement:** `harbor-score` owns DIM-01..13 as distinct dimensions, and the
first finding explicitly limits itself to DIM-01.

**Evidence:** `docs/vtrace/SPECIFICATION_BASELINE.md`,
`docs/findings/2026-06-deep-draft-channel-tail.md`,
`crates/harbor-score/src/lib.rs`, and `docs/vtrace/TRACE.md`.

## HARBOR-I-05: Public Reuse Requires Boundary Review

**Status:** PARTIAL

**Claim:** Public or downstream reuse requires scope boundary language and role
review before HARBOR output is treated as a decision artifact.

**Why it matters:** A useful research diagnostic can be mistaken for dredging,
lease, permitting, funding, or agency authority.

**Enforcement:** README/product boundaries, VTRACE review, role panel, and
PITFALL tracking keep public reuse bounded.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `.roles/ROLE.md`,
`docs/vtrace/REVIEW.md`, and `.pitfall/harbor-pitfalls.md`.
