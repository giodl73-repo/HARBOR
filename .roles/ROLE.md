# HARBOR — Role Index

Four tiers of review roles. Read this before opening any role file. Reviews of corpus entries,
gap findings, design proposals, tier/SLA definitions, and VTRACE deliverables run against these
files and record dispositions (`pass` / `finding` / `defer`).

---

## Parliament roles (7 voices)

Adversarial expert voices. They plant incompatible stakes; the argument record is the output,
not consensus. No voice is skipped. A good project survives all seven; a weak one collapses
under one or two, and the collapse is the finding.

| File | Voice | Primary tension |
|---|---|---|
| `parliament/port-system-planner.md` | Port-System Planner | System throughput + resilience vs. single-port framing |
| `parliament/maritime-engineer.md` | Maritime / Civil Engineer | Buildable channel/berth/crane vs. brochure-fantasy throughput |
| `parliament/operations-logistics-officer.md` | Operations & Logistics Officer | Surge fluidity/dwell vs. average-flow optimism |
| `parliament/maritime-economist.md` | Maritime Economist | Benefit-cost + cargo base vs. discretionary-volume inflation |
| `parliament/regional-hinterland-advocate.md` | Regional-Hinterland Advocate | Inland access vs. gateway-concentrated benefit |
| `parliament/environmental-community-advocate.md` | Environmental & Port-Community Advocate | Emissions/dredging/fenceline vs. throughput expansion |
| `parliament/port-authority-alliance-realist.md` | Port-Authority & Alliance Realist | Terminal/alliance/labor control vs. assumed-cooperation assumptions |

---

## Editorial roles (3 voices)

Form gate before `validated` status. Run after parliament, not instead of it.

| File | Role | Checks |
|---|---|---|
| `editorial/citation-auditor.md` | Citation Auditor | Every quantity sourced in `data/sources.md` or labelled |
| `editorial/scope-keeper.md` | Scope Keeper | Artifact stays within its declared type, **scale**, schema, pool, and tier model |
| `editorial/numeracy-checker.md` | Numeracy Checker | Units consistent (TEU/days/feet/$); magnitudes sane; arithmetic and 0–10 scale clean |

---

## Stakeholder roles (cross-cutting lenses)

Not reviewers — lenses for who the network serves, used during corpus scoring, gap analysis,
and tier/SLA assignment.

| File | Stakeholder | Primary concern |
|---|---|---|
| `stakeholders/shipper-bco.md` | Shipper / Beneficial Cargo Owner | Reliable transit, dwell, total landed cost |
| `stakeholders/drayage-trucker.md` | Drayage Trucker | Gate turn time, queueing, appointments |
| `stakeholders/longshore-worker.md` | Longshore Worker | Jobs, safety, automation, conditions |
| `stakeholders/ocean-carrier.md` | Ocean Carrier / Alliance | Berth windows, draft, routing economics |
| `stakeholders/port-adjacent-community.md` | Port-Adjacent Community | Air quality, truck traffic, local benefit |

---

## Panel reviewer roles (illustrative peer panel)

Archetype academic/practitioner peer reviewers for HARBOR research outputs. See
`panel-reviewer/panel.md`. Used for paper-grade methodology review, distinct from parliament and
editorial.

---

## How reviews are recorded

When a `docs/vtrace/` deliverable, corpus entry, gap finding, design proposal, or tier/SLA
definition is being settled, the relevant subset of this panel is applied and dispositions are
recorded in:

- the deliverable's **Role Review Notes** section, and
- the active wave pulse ledger.

A stage reaches its **fixed point** when no unresolved critical or major actionable finding
remains and every deferred item names a later stage or work package.
