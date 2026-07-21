# Specification Baseline

## Scope

Repo: HARBOR

Baseline type: target (provisional)

Baseline date: 2026-06-26

VTRACE adoption scope: define the controlled behavior HARBOR intends to build — the dimension
pool, scoring scale, demand basis, corpus schema, evidence labels, tier model, and the
**multi-scale model** — before architecture, interfaces, or implementation planning. Because
HARBOR is greenfield, every item is `target`, not observed `current`. The dimension pool is
**provisional**: dimensions and their basis are controlled here, but per-dimension anchors and
rubric weights calibrate from the scored corpus (REQ-006) and are not asserted in this file.
Future work packages must cite `SPEC-*` / `DIM-*` IDs instead of making unanchored changes.

## Specification Sources

| Source | Evidence | Status | Notes |
|---|---|---|---|
| `README.md` | HARBOR thesis, hypothesis, multi-scale, pipeline. | target | Public-facing repo intent. |
| `PRODUCT_PLAN.md` | Scope, non-goals, method, waves. | target | Product framing. |
| `CLAUDE.md` | House rules, multi-scale rule, pipeline, quality bar. | target | Operating constraints. |
| `docs/vtrace/MISSION.md` | `NEED-*`, `CON-*`. | current | VTRACE mission source. |
| `docs/vtrace/CONOPS.md` | `OPS-*` scenarios. | current | VTRACE scenario source. |
| `docs/vtrace/REQUIREMENTS.md` | `REQ-001..016`, `DEF-001..005`. | current | VTRACE requirement source. |
| `.roles/ROLE.md` | Parliament + editorial review lenses. | current | Review-lane source. |

## Scale Model (`SCALE-*`) (resolves NEED-008 / REQ-016)

HARBOR runs the same methodology at any scale. Every corpus element declares a scale; scores,
tiers, and gaps are interpreted within scale.

| Scale | Meaning | Example governance |
|---|---|---|
| `international` | Global lanes, gateway/transshipment ports, alliances. | IMO, alliance networks, global terminal operators. |
| `national` | A single nation's ports and policy. | National regulators (MARAD), port systems. |
| `regional` | Multi-jurisdiction within a nation. | Coastal ranges, inland-port systems, corridors. |
| `local` | A single port/terminal/berth. | Port authorities, terminal operators. |

| ID | Rule |
|---|---|
| SCALE-01 | Every corpus element carries a `scale` and a `market`/jurisdiction tag (REQ-016). |
| SCALE-02 | Scores, tiers, and gaps are interpreted within the element's scale; cross-scale comparison/aggregation requires an explicit labelled note (CON-007). |
| SCALE-03 | Scale may nest (a local terminal within a port within a national/international network); nesting representation is provisional (DEF-005). |

## Dimension Pool (`DIM-*`)

The candidate pool HARBOR scores existing port/freight-network elements against. Each dimension
is scored 0–10. Anchors and weights are **calibrated from the corpus** (REQ-006), not fixed here.
"Primary basis" names where the input comes from; "Default label" is the evidence posture a fresh
value carries until upgraded with a cited source (REQ-002, REQ-003).

| DIM ID | Dimension | What it measures | Primary basis | Default label |
|---|---|---|---|---|
| DIM-01 | Throughput / Volume | TEU/tonnage moved at the stated scale. | USACE Waterborne Commerce, UNCTAD | source-needed |
| DIM-02 | Capacity / Throughput | Berth/crane/yard capacity vs demand. | Terminal benchmarks, AIS dwell | heuristic |
| DIM-03 | Dwell / Turn Time | Container dwell and truck/rail turn under load. | BTS Port Performance, AIS | source-needed |
| DIM-04 | Network Connectivity | Lane + hinterland reach and centrality (graph). | UNCTAD LSCI, AIS lanes (computable) | implemented |
| DIM-05 | Resilience | Single-terminal/channel/supply-chain shock exposure. | Infrastructure + disruption history | heuristic |
| DIM-06 | Channel Depth / Access | Draft, air draft, and lock constraints for the design ship. | USACE channel data | source-needed |
| DIM-07 | Competition / Alliance Diversity | Operator and alliance diversity; concentration. | Carrier/alliance shares (computable) | implemented |
| DIM-08 | Intermodal / Hinterland Access | Inland rail/road/barge connection and velocity. | Rail/road inventories, corridors | heuristic |
| DIM-09 | Labor / Automation | Throughput-per-worker, automation posture, labor terms. | Operator/labor data | proxy |
| DIM-10 | Environmental Exposure | Port/drayage emissions, dredging/habitat, community air. | Emissions/dredging inventories | heuristic |
| DIM-11 | Equity / Regional Access Exposure | Who gains vs. who absorbs the corridor/fenceline burden. | Census + EJ + corridor traffic | implemented |
| DIM-12 | Capital-Efficiency / Benefit-Cost | Benefit per unit capital. | Port/corridor B/C studies | heuristic |
| DIM-13 | Tier-SLA Conformance | Degree the element meets its tier's throughput/dwell/connectivity/access SLA (derived; shortfall = tier-SLA gap). | Tier model + DIM-01/02/03/06 | heuristic |

Calibration note (per REQ-006, OPS-002): after the first corpus pass, low-variance or redundant
dimensions are retired and informative ones promoted; the rubric version records each change. The
pool above is the v0 candidate set, not a final rubric.

## Demand Basis (resolves DEF-002 minimum)

| ID | Rule |
|---|---|
| SPEC-DB-01 | Throughput/dwell/capacity dimensions (DIM-01, DIM-02, DIM-03) name the **demand basis** (peak-season surge vs average) and the **access basis** (tide-restricted vs all-tide/full-draft) on every derived claim (REQ-007). |
| SPEC-DB-02 | Nominal capacity and AIS-derived dwell may be used as a labelled proxy when observed terminal data is unavailable; the proxy status must be explicit. Default scope names the basis (DEF-002 remains open for observed-data adoption). |

## System Tier Model (`T1–T4`) (resolves NEED-007 / REQ-014 / REQ-015)

HARBOR classifies each element into a four-tier hierarchy — from international gateway port to
local terminal/berth — with a throughput/dwell/connectivity/access SLA per tier. This is the
Ports 2.0 analog of the portfolio tiering. The tier hierarchy nests within the scale model (a T1
gateway port is typically international-scale). Roles are typical, not strict.

| Tier | Name | Typical role | SLA promise (target) |
|---|---|---|---|
| T1 | International Gateway Port | Deep-water gateway/transshipment hub, megaship calls. | Massive throughput; managed dwell; deep all-tide access; broad lane connectivity; resilient. |
| T2 | National Port | National load-center port. | High throughput; bounded dwell; strong hinterland connectivity; channel adequacy. |
| T3 | Regional / Inland Port | Regional/feeder port or inland intermodal port. | Adequate throughput; reliable hinterland feed; maintained connectivity to a gateway. |
| T4 | Local Terminal / Berth | Local terminal, short-sea, barge. | Maintained access; basic reliability; lifeline connectivity. |

Each tier's SLA is expressed over four contract terms, assessed by DIM-13:

| SLA term | Meaning | Backing dimensions |
|---|---|---|
| Throughput / capacity | Capacity the tier promises vs demand. | DIM-01, DIM-02 |
| Dwell | Dwell/turn time the tier must hold (demand basis named). | DIM-03 |
| Connectivity | Lane + hinterland reach the tier provides. | DIM-04, DIM-08 |
| Access | Channel/affordability access the tier provides. | DIM-06, DIM-11 |

SLA values per tier are **target and provisional** — exact thresholds calibrate with the rubric
(REQ-006) and are not asserted here. A tier-SLA shortfall is a first-class gap (REQ-015, OPS-006).

## Controlled Specification Items

| Spec ID | Parent REQ IDs | Type | C/T/D/U | Specification Statement | Verification Method | Validation Method | Owner | Risk | Status |
|---|---|---|---|---|---|---|---|---|---|
| SPEC-001 | REQ-004 / REQ-005 | architecture | target | Every element is keyed by a stable port/terminal/lane/network identifier; operator, vessel name, and map id are mutable presentation fields, not keys. | schema check / inspection | OPS-001 | HARBOR maintainer | high | accepted |
| SPEC-002 | REQ-001 / REQ-003 / REQ-014 / REQ-016 | product | target | A corpus entry is one markdown file with frontmatter (id, type, scale, market, termini, tier, sla, source rows) and a scored dimension block, regenerable from documented commands. | inspection / command review | OPS-001 | HARBOR maintainer | medium | accepted |
| SPEC-003 | REQ-002 | product | target | Every quantity carries an evidence label from {implemented, heuristic, simulated, proxy, planned, held, source-needed, confidence-limited}. | artifact inspection | OPS-001 / OPS-004 | HARBOR maintainer | medium | accepted |
| SPEC-004 | REQ-006 | product | target | The dimension pool is `DIM-01..DIM-13` scored 0–10; anchors and weights are calibrated from corpus variance and versioned, not fixed in this baseline. | calibration record / version diff | OPS-002 | HARBOR maintainer | high | accepted |
| SPEC-005 | REQ-007 | software | target | Throughput/dwell dimensions name the demand basis (surge vs average) and access basis (tide-restricted vs all-tide) on each claim (SPEC-DB-01). | analysis / inspection | OPS-003 | operations reviewer | high | accepted |
| SPEC-006 | REQ-008 | product | target | An already fluid, deep, well-connected port system is recorded as a labelled null result; scope is not expanded to manufacture a gap. | gap-artifact inspection / review | OPS-003 | HARBOR maintainer | medium | accepted |
| SPEC-007 | REQ-009 / REQ-010 | ops | target | Promotable claims pass the 7-voice parliament and 3-role editorial gate, with throughput, dwell, resilience, channel, hinterland, competition, environment, benefit-cost, and terminal/alliance lenses represented. | review inspection | OPS-004 | review steward | medium | accepted |
| SPEC-008 | REQ-011 | product | target | Outputs carry a scope boundary: research/tooling/conceptual-design only; no construction readiness, capacity/depth validity of record, concession/permit determination, or endorsement. | editorial review | OPS-004 | HARBOR maintainer | medium | accepted |
| SPEC-009 | REQ-003 | data | target | `data/sources.md` is the citation registry; every cited quantity names a registry entry, and proxies/heuristics (incl. nominal/AIS) are labelled rather than cited. | citation audit | OPS-001 | data steward | high | accepted |
| SPEC-010 | REQ-012 / REQ-013 | ops | target | VTRACE deliverables advance one at a time to a `.roles` fixed point; HARBOR changes stay in the child repo until an intentional TRACKER pointer update after intake. | wave ledger / git status | OPS-005 | HARBOR maintainer | low | accepted |
| SPEC-011 | REQ-014 | product | target | Every analyzed element is classified into exactly one tier (T1–T4) per the System Tier Model and carries that tier's declared SLA terms. | schema check / inspection | OPS-006 | HARBOR maintainer | high | accepted |
| SPEC-012 | REQ-015 | software | target | Tier-SLA conformance (DIM-13) is assessed per element against its tier SLA; any shortfall is recorded as a tier-SLA gap and a market is not called adequate while an unaddressed shortfall stands. | analysis / gate / inspection | OPS-003 / OPS-006 | HARBOR maintainer | high | accepted |
| SPEC-013 | REQ-016 | product | target | Every element carries a `scale` and `market`/jurisdiction tag (SCALE-01); analysis runs within a scale and any cross-scale comparison carries an explicit labelled note (SCALE-02). | schema check / gate / review | OPS-007 | HARBOR maintainer | high | accepted |

## Public Contracts

| Contract ID | Spec IDs | Surface | Compatibility Rule | Change-Control Trigger | Verification Evidence |
|---|---|---|---|---|---|
| IF-001 | SPEC-001 / SPEC-002 / SPEC-013 | corpus file (markdown + frontmatter, incl. scale/market) | Frontmatter keys are additive; `id` immutable; `scale` from a fixed enum. | Any key rename/removal, id-semantics, or scale-enum change. | schema check (target) |
| IF-002 | SPEC-009 | `data/sources.md` (registry) | Source entries are append/annotate; ids stable. | Removing or re-pointing a source id. | citation audit (target) |
| IF-003 | SPEC-004 | rubric version record | Dimension set + weights versioned; changes recorded. | Retiring/adding a `DIM-*` or changing weights. | calibration record (target) |
| IF-004 | SPEC-011 / SPEC-012 | tier/SLA record | Tier set (T1–T4) and per-tier SLA terms are versioned; tier reassignment is recorded. | Changing a tier definition, SLA term, or an element's tier. | tier/SLA record (target) |

## Package / Language Allocation

| Spec IDs | Package / Module | Responsibility | Forbidden Responsibility | Validation Profile |
|---|---|---|---|---|
| SPEC-001 / SPEC-004 / SPEC-005 | port/freight kernel (future `harbor-network`) | Graph model, connectivity/centrality metrics (DIM-04), throughput/dwell helpers. | Scoring policy, evidence labels, review logic. | L1 |
| SPEC-002 / SPEC-003 / SPEC-009 / SPEC-013 | corpus + data layer | File schema, scale/market tags, source registry, evidence labels. | Graph math, design proposals. | L0/L1 |
| SPEC-007 / SPEC-008 | review layer (`.roles`) | Parliament/editorial gate, scope boundary. | Computing scores. | L0 |
| SPEC-011 / SPEC-012 | tier/SLA layer | Tier classification, SLA terms, tier-SLA conformance (DIM-13). | Setting calibrated SLA thresholds without rubric. | L1 |

## Nonfunctional Constraints

| Constraint ID | Parent Spec IDs | Constraint | Threshold / Rule | Verification Method | Status |
|---|---|---|---|---|---|
| SPEC-NF-001 | SPEC-002 / SPEC-004 | Reproducibility | Active corpus/score/gap artifacts regenerate from documented commands with labels and scale preserved. | command review | proposed |
| SPEC-NF-002 | SPEC-009 | No raw datasets committed | Raw/cache data is gitignored; only derived, cited artifacts are committed. | inspection | proposed |
| SPEC-NF-003 | SPEC-001 / SPEC-013 | Deterministic identity + scale | Element ids and scale tags are deterministic given source inputs. | inspection / test | proposed |

## Assumptions And Unknowns

| ID | Item | Impact | Disposition | Owner |
|---|---|---|---|---|
| SPEC-UNK-001 | Throughput (DIM-01), dwell (DIM-03), channel (DIM-06) depend on data of varying openness/lag across scales. | May force proxy/source-needed labels on early corpus rows. | discovery → `data/sources.md` | data steward |
| SPEC-UNK-002 | AIS-derived dwell and nominal capacity differ from observed terminal data. | Likely proxy at v0 for data-poor ports. | accept risk (labelled proxy) | operations reviewer |
| SPEC-UNK-003 | Benefit-cost (DIM-12) requires program assumptions. | Heuristic until grounded. | defer to corpus calibration | maritime economist |
| SPEC-UNK-004 | Per-tier SLA thresholds (DIM-13). | Affects conformance scoring. | defer to calibration | HARBOR maintainer |
| SPEC-UNK-005 | Whether scale nests as a hierarchy or stays a flat tag. | Affects schema + cross-scale notes. | defer (DEF-005) | HARBOR maintainer |

## Requirement-To-Spec Coverage

| Requirement ID | Spec IDs | Coverage Status | Notes |
|---|---|---|---|
| REQ-001 | SPEC-002, SPEC-NF-001 | covered | Regeneration path. |
| REQ-002 | SPEC-003 | covered | Evidence labels. |
| REQ-003 | SPEC-009 | covered | Citation registry. |
| REQ-004 | SPEC-001 | covered | Stable identity. |
| REQ-005 | SPEC-001, SPEC-013 | covered | Hold/reject unidentified/untagged rows. |
| REQ-006 | SPEC-004, IF-003 | covered | Calibrated rubric. |
| REQ-007 | SPEC-005, SPEC-DB-01 | covered | Demand + access basis named. |
| REQ-008 | SPEC-006 | covered | Null result. |
| REQ-009 | SPEC-007 | covered | Review gate. |
| REQ-010 | SPEC-007 | covered | Stakeholder lenses. |
| REQ-011 | SPEC-008 | covered | Scope boundary. |
| REQ-012 | SPEC-010 | covered | Child-repo scoping. |
| REQ-013 | SPEC-010 | covered | One-at-a-time VTRACE. |
| REQ-014 | SPEC-011, IF-004 | covered | Tier classification + SLA. |
| REQ-015 | SPEC-012, DIM-13 | covered | Tier-SLA gap gating. |
| REQ-016 | SPEC-013, SCALE-01..03, IF-001 | covered | Multi-scale tagging + within-scale interpretation. |

## Spec-To-Verification Coverage

| Spec ID | Verification IDs / Commands | Expected Result | Evidence Pointer | Status |
|---|---|---|---|---|
| SPEC-001..013 | future `VER-*` in `VERIFICATION.md` | Each spec has a credible check (schema, command, inspection, or review). | future `EVID-*` | planned |

## Role Review Notes

| Role Lens | Spec Impact | Disposition |
|---|---|---|
| Scope Keeper | Baseline defines controlled behavior, a candidate pool, a tier model, and the scale model; it asserts no scored network or design. | pass |
| Citation Auditor | No quantities asserted; "Primary basis" names where inputs come from; DIM default labels enforce citation discipline. | pass |
| Numeracy Checker | Only the 0–10 scale is named; the system `scale` enum is distinct from the score scale; no computed values. | pass |
| Operations & Logistics Officer | Demand basis is controlled (SPEC-DB-01 / SPEC-005); observed-data adoption deferred. | pass |
| Port-Authority & Alliance Realist | Initial draft let DIM-01 read as if throughput were control-free; resolved by keeping alliance diversity (DIM-07) in SPEC-007 and the access SLA term. | resolved |
| Maritime Economist | Benefit-cost (DIM-12) default label set to `heuristic`; SPEC-UNK-003 records the gap. | pass |
| Regional-Hinterland & Environmental advocates | Hinterland access (DIM-08/11) and environment (DIM-10) are in the pool. | pass |

Fixed-point note: one actionable finding (throughput read as control-free) was raised and applied
via the alliance-diversity review lens + SPEC-007. No unresolved critical or major finding
remains. Pool, SLA, and scale-nesting details are explicitly provisional; calibration and DEF-005
deferred.

## Specification Gate

Decision: pass_with_risk

Required before implementation planning:

- [x] Every accepted `REQ-*` maps to one or more `SPEC-*` IDs or a recorded deferral.
- [x] Every implementation work package can name parent `SPEC-*` IDs or discovery status.
- [x] Public contracts have owners and change-control triggers.
- [~] Unknowns are resolved, blocked, deferred, or converted to discovery work (SPEC-UNK-001..005 are discovery/defer/accept-risk).
- [x] Verification and validation methods are credible for the controlled claim.

Rationale: the baseline is coherent enough to drive trace, verification, and the review gate.
Residual risk is concentrated in cross-scale data openness and AIS/nominal proxy ambiguity
(SPEC-UNK-001/002), provisional weights/SLA thresholds, and scale-nesting representation
(DEF-005), all deferred to the corpus calibration wave rather than blocking the minimum slice.
