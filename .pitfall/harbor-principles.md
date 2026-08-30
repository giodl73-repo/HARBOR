# HARBOR Principles

These entries summarize durable HARBOR decision rules for maritime-freight
evidence, scale, port-service chains, gap analysis, review, and public-authority
boundaries.

## HARBOR-P-01: Port Service Is A Chain, Not A Berth

**Status:** ACTIVE

**Statement:** Port adequacy depends on channel, terminal, dwell, hinterland,
resilience, governance, and demand basis; no single berth or channel metric is
enough to claim service adequacy.

**Rationale:** Maritime gateways fail through coupled constraints, and a narrow
depth or throughput signal can hide landside, operational, or governance limits.

**Decision rule:** Any promoted finding must name assessed dimensions,
unassessed dimensions, market scale, source labels, and demand/access basis.

**Evidence:** `docs/vtrace/MISSION.md`, `docs/vtrace/REQUIREMENTS.md`,
`docs/vtrace/SPECIFICATION_BASELINE.md`, `.roles/ROLE.md`, and
`docs/findings/2026-06-deep-draft-channel-tail.md`.

## HARBOR-P-02: Scale And Market Bind Every Claim

**Status:** ACTIVE

**Statement:** International, national, regional, and local port claims are
interpreted only within their declared scale and market unless a cross-scale
comparison is explicitly labelled.

**Rationale:** A national gateway adequacy signal and a local terminal or berth
signal can share dimension names while using different denominators and
intervention meanings.

**Decision rule:** Missing scale or market holds corpus promotion, and gap runs
must filter to the requested scale.

**Evidence:** `corpus/SCHEMA.md`, `docs/vtrace/TRACE.md`,
`docs/vtrace/VERIFICATION.md`, `crates/harbor-corpus/src/lib.rs`, and
`crates/harbor-gap/src/lib.rs`.

## HARBOR-P-03: Evidence Labels And Source IDs Stay Visible

**Status:** ACTIVE

**Statement:** Implemented, proxy, source-needed, heuristic, held, and
confidence-limited evidence states must survive corpus parsing, scoring, tier
checks, gap analysis, CLI output, and findings.

**Rationale:** Maritime data often blends official publications, port-authority
reports, AIS proxies, and operational assumptions; upgrading labels silently
creates false certainty.

**Decision rule:** Source ids, evidence labels, scale tags, and measurement
basis cannot be silently dropped or upgraded as data moves across crates.

**Evidence:** `data/sources.md`, `corpus/SCHEMA.md`,
`docs/vtrace/CODE_RIGOR.md`, `crates/harbor-corpus/src/lib.rs`, and
`crates/harbor-cli/src/main.rs`.

## HARBOR-P-04: Tail Gaps Do Not Become Systemic Claims

**Status:** ACTIVE

**Statement:** A concentrated gateway tail can be real without indicting every
gateway in the market.

**Rationale:** The June 2026 DIM-01 replay shows Houston and Savannah below the
channel-depth baseline while the remaining six national gateways are at or
above baseline.

**Decision rule:** Preserve broad alarms and `TailRegion.entry_ids` as separate
artifacts; use the tail set as the actionable localization signal.

**Evidence:** `docs/findings/2026-06-deep-draft-channel-tail.md`,
`docs/vtrace/VERIFICATION.md`, `crates/harbor-gap/src/lib.rs`, and
`crates/harbor-gap` tests.

## HARBOR-P-05: Research Output Is Not Port Authority

**Status:** ACTIVE

**Statement:** HARBOR can support research, review, cited diagnostics, and
conceptual design, but it does not issue dredging designs, terminal lease
advice, NEPA/permitting evidence, funding direction, or agency endorsements.

**Rationale:** A reproducible adequacy signal and a public decision artifact
have different owners, standards, and accountability requirements.

**Decision rule:** Public-facing outputs must retain research-lab framing,
source and scale limits, and no-authority language before reuse or publication.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `docs/vtrace/REVIEW.md`, and
`.roles/ROLE.md`.
