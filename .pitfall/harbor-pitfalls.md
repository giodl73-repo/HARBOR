# HARBOR Pitfalls

These entries capture recurring maritime-freight evidence failure classes and
map them to HARBOR controls or open repo-local risks.

## HARBOR-PF-01: Berth Or Channel Depth Becomes Gateway Adequacy

**Status:** MITIGATED

**Pattern:** A single DIM-01 channel-depth score is treated as complete port or
gateway adequacy.

**Domain:** Corpus entries, DIM-01..13 scoring, deep-draft finding, public
summaries, and future customer distribution.

**Detection difficulty:** Channel depth is concrete, official-looking, and easy
to compare, while landside and operational service constraints need more
evidence.

**Structural solution:** Keep DIM-01 separate and emit DIM-02..DIM-13 as
`EmptyRegion` until sourced.

**Evidence:** `docs/findings/2026-06-deep-draft-channel-tail.md`,
`docs/vtrace/VERIFICATION.md`, `crates/harbor-score/src/lib.rs`, and
`crates/harbor-gap/src/lib.rs`.

## HARBOR-PF-02: Concentrated Tail Becomes Systemic Port Failure

**Status:** MITIGATED

**Pattern:** A min-based alarm catches a real shortfall but blames every scored
gateway for Houston and Savannah's channel-depth tail.

**Domain:** Gap analysis, national DIM-01 finding, CLI output, public narrative,
and cross-repo detector standardization.

**Detection difficulty:** The broad alarm is not false, but it is not localized
enough to support targeted action.

**Structural solution:** Preserve `TailRegion.entry_ids` as the actionable
target set and report it separately from broad alarms.

**Evidence:** `docs/findings/2026-06-deep-draft-channel-tail.md`,
`docs/vtrace/TRACE.md`, `docs/vtrace/VERIFICATION.md`, and
`cargo run -p harbor-cli -- gap --input corpus --scale national`.

## HARBOR-PF-03: Empty Dimensions Become Zero Or Hidden Evidence

**Status:** MITIGATED

**Pattern:** Missing DIM-02..DIM-13 evidence is mistaken for zeros, adequacy, or
absence of issue.

**Domain:** CLI gap output, findings, VTRACE verification, role review, and
portfolio readiness scoring.

**Detection difficulty:** Empty evidence can disappear from summaries unless
the artifact carries explicit coverage gaps.

**Structural solution:** Emit explicit `EmptyRegion` gaps and keep the first
finding limited to DIM-01.

**Evidence:** `docs/findings/2026-06-deep-draft-channel-tail.md`,
`docs/vtrace/VERIFICATION.md`, `crates/harbor-gap/src/lib.rs`, and
`cargo test --workspace --locked`.

## HARBOR-PF-04: Historical Governance Text Lags Implementation

**Status:** MITIGATED

**Pattern:** Foundation VTRACE docs continue to describe HARBOR as greenfield,
pending, or code-free after the Rust workspace, corpus, CLI, and first finding
exist.

**Domain:** VTRACE trace/review/verification, architecture, interfaces, code
rigor, implementation wave, portfolio status scoring, and research packet.

**Detection difficulty:** Historical VTRACE docs are internally coherent, but
they can be mistaken for current status after implementation lands.

**Structural solution:** Add current-state implementation updates to VTRACE
trace/review/verification, architecture, interfaces, and code-rigor docs without
rewriting the historical foundation record.

**Evidence:** `docs/vtrace/VERIFICATION.md`, `docs/vtrace/TRACE.md`,
`docs/vtrace/REVIEW.md`, `docs/vtrace/ARCHITECTURE.md`,
`docs/vtrace/INTERFACES.md`, and `docs/vtrace/CODE_RIGOR.md`.

## HARBOR-PF-05: First Port Finding Becomes Public Authority

**Status:** MITIGATED

**Pattern:** The cited deep-draft channel finding, CLI gap output, or portfolio
summary is treated as dredging design, terminal lease advice, NEPA/permitting
evidence, funding direction, port-authority position, or agency endorsement.

**Domain:** README, first finding, customer distribution, research paper,
partner reuse, and downstream portfolio summaries.

**Detection difficulty:** The finding is reproducible and useful, and it names
real gateways, which can invite authority claims beyond its labels.

**Structural solution:** Keep research-lab and no-authority language visible,
preserve DIM-01-only posture and unassessed dimensions, and require explicit
full-panel release review before treating the finding as a public decision
artifact.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`,
`docs/findings/2026-06-deep-draft-channel-tail.md`,
`docs/adoption/README.md`, `docs/vtrace/REVIEW.md`, `.roles/ROLE.md`,
`docs/pitfall-boundaries.v1.json`, and `tests/check-pitfall-policy.ps1`.
