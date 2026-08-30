# Verification Plan

## Scope

Repo: HARBOR

VTRACE adoption scope: define verification methods and command levels for HARBOR's requirements.
The initial VTRACE baseline was authored before implementation; the six-crate workspace now exists
and the June 2026 implementation wave has closed. This file records current command evidence while
preserving residual limits: corpus coverage is narrow, many dimensions are intentionally empty, and
public findings remain research outputs rather than port-design, lease, permitting, or agency
authority.

## Verification Matrix

| Req ID | Method | Command / Inspection | Expected Evidence | Result | Evidence Pointer |
|---|---|---|---|---|---|
| REQ-001 | inspection / demonstration | `cargo run -p harbor-cli -- gap --input corpus --scale national` | a documented regeneration path with labels + scale preserved | passed | EVID-001 |
| REQ-002 | inspection / review | inspect evidence labels on corpus quantities and source ids | every material quantity carries an evidence label | passed | EVID-002 |
| REQ-003 | citation audit | inspect corpus source ids against `data/sources.md` | every cited quantity resolves to a registry source or is labelled | passed | EVID-003 |
| REQ-004 | schema check / inspection | validate corpus frontmatter keys against schema | stable port/terminal/lane/network id present; labels are not keys | passed | EVID-004 |
| REQ-005 | gate / data inspection | cargo tests for missing id/source/scale hold or reject paths | such rows held, not promoted | passed | EVID-005 |
| REQ-006 | calibration record | inspect rubric version + calibration rationale | rubric changes are versioned and justified | pass_with_risk | EVID-006 (v0, provisional) |
| REQ-007 | analysis / inspection | inspect typed demand/access basis surfaces and tests | surge-vs-average and tide-restricted-vs-all-tide basis named on each claim | passed | EVID-007 |
| REQ-008 | gap inspection / review | `harbor-cli gap` plus gap tests | null/empty regions recorded; tail gap localized | passed | EVID-008 |
| REQ-009 | review inspection | confirm parliament + editorial gate ran on a promoted claim | review records exist with dispositions | pass_with_risk | EVID-009 (panel exists, not yet exercised on a corpus claim) |
| REQ-010 | role review | confirm throughput/dwell/resilience/channel/hinterland/competition/environment/cost/control lenses represented | stakeholder lenses present in `.roles/` and applied | pass_with_risk | EVID-010 (`.roles/` panel built) |
| REQ-011 | editorial review | inspect public claims for scope boundary | outputs framed as research/tooling/conceptual design | pass_with_risk | EVID-011 (`README`/`PRODUCT_PLAN`/`MISSION` non-goals) |
| REQ-012 | git inspection | `git status --short`; confirm no TRACKER pointer touched | HARBOR changes stay in the child repo | passed | EVID-012 |
| REQ-013 | wave ledger / review | inspect wave ledger + pulses for one-at-a-time discipline | each VTRACE stage settled to a fixed point in sequence | passed | EVID-013 |
| REQ-014 | schema check / inspection | validate `tier` + `sla` frontmatter and tier model | every element classified T1–T4 with declared SLA | passed | EVID-014 |
| REQ-015 | gate / gap inspection | tier-SLA conformance checks and DIM-13 empty-region reporting | tier-SLA shortfalls reported before adequacy claimed | pass_with_risk | EVID-015 (DIM-13 currently empty in first corpus) |
| REQ-016 | schema check / gate | validate `scale`/`market` tags + within-scale gap filter | every element scale-tagged; cross-scale notes explicit | passed | EVID-016 |
| REQ-DOC-001 | doc QA | `proof check .` | markdown QA clean across repo docs | passed | EVID-DOC-001 |

## Commands

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
cargo run -p harbor-cli -- --help
cargo run -p harbor-cli -- gap --input corpus --scale national
git diff --check
```

## Validation Levels

| Level | Purpose | Commands / Evidence | Result |
|---|---|---|---|
| L0 | Fast crate/doc sanity for the active stage. | package tests and `git diff --check` | passed |
| L1 | Full repo confidence before push. | fmt, clippy, workspace tests, CLI help | passed |
| L2 | Readiness proof before a public claim. | scale-filtered gap replay + role review + finding scope review | pass_with_risk |

## Evidence Ledger

| Evidence ID | Type | Path / Command | Covers | Result |
|---|---|---|---|---|
| EVID-DOC-001 | report | `proof check .` (0 errors) | REQ-DOC-001 | passed |
| EVID-012 | command | `git status --short` (standalone child repo) | REQ-012 | passed |
| EVID-013 | review | `context/waves/2026-06-26-vtrace-foundation/` ledger + pulses | REQ-013 | passed |
| EVID-009..011 | review | `.roles/` panel present and applied in stage reviews | REQ-009/010/011 | pass_with_risk |
| EVID-001..008, 014, 015, 016 | command/review | fmt, clippy, tests, CLI help, national gap replay | REQ-001..008/014/015/016 | passed/pass_with_risk |

## Gaps

| Gap | Impact | Disposition |
|---|---|---|
| Corpus coverage is narrow. | The first finding covers eight national-scale US gateways and DIM-01 only. | label as first-slice evidence; do not generalize to all port adequacy |
| DIM-02..DIM-13 are empty in the national replay. | Empty dimensions can be mistaken for zeros or passing scores. | keep `EmptyRegion` artifacts explicit |
| Public finding may be overused. | A research signal could be cited as dredging, lease, NEPA, funding, or agency authority. | keep public scope boundary and role review before promotion |

## Role Review Notes

| Role Lens | Verification Impact | Disposition |
|---|---|---|
| V&V lens | Methods are credible and mapped 1:1 to requirements; unrun checks are `pending`, not faked. | pass |
| Citation Auditor | Evidence pointers are real (commands run) or explicitly future. | pass |
| Numeracy Checker | The one quantity (0 errors) is a real command result. | pass |
| Scope Keeper | Verification stays at method/result level; REQ-016 scale check named. | pass |

Fixed-point note: no actionable finding required a change. The plan honestly separates verified-now
(process/doc) from pending (implementation). No unresolved critical/major finding.
