---
name: HARBOR Open Adoption Guide
slug: harbor-open-adoption-guide
type: adoption
status: draft
rubric_version: v1.0
author: codex
created: 2026-07-22
updated: 2026-07-22
sources:
  - README.md
  - docs/findings/2026-06-deep-draft-channel-tail.md
---

# HARBOR Open Adoption Guide

## Purpose

HARBOR is public and open to use. Use it as a reference model for evidence-gated
port and maritime-gateway analysis, as a cited finding, or as a pattern for a
bounded gateway, terminal, channel-access, hinterland, resilience, or intermodal
adaptation.

Public use does not create an engineering study, dredging design,
environmental review, terminal lease, advocacy brief, MARAD endorsement,
port-authority endorsement, carrier endorsement, or IMO endorsement.

## Fast Paths

| If You Are | Start With | What You Can Do |
|---|---|---|
| Public reader | [`README.md`](../../README.md) | Understand the port service-chain model. |
| Researcher | [`docs/findings/2026-06-deep-draft-channel-tail.md`](../findings/2026-06-deep-draft-channel-tail.md) | Inspect the cited channel-access finding. |
| Port, freight, or planner reviewer | [`local-adaptation-worksheet.md`](local-adaptation-worksheet.md) | Scope a gateway or terminal-service question without overclaiming. |
| Builder or contributor | [`docs/vtrace/`](../vtrace) | Work from requirements, traceability, and evidence labels. |

## First Local Adaptation

1. Pick a bounded gateway, terminal district, channel, hinterland corridor, or
   intermodal connection.
2. Name the promise: channel access, dwell, throughput, intermodal connection,
   resilience, competition, or market reach.
3. List source surfaces: port statistics, channel depth, terminal gate, rail,
   truck, dwell, vessel call, resilience, or project-plan evidence.
4. Mark every claim as source-backed, heuristic, held, source-needed, or
   confidence-limited.
5. Produce a short readout: service promise, current gap, evidence holds, and
   next source asks.

## Contribution Targets

- source inventories for gateways, terminals, or hinterland links;
- corrections to channel, dwell, throughput, or intermodal claims;
- carrier, port, terminal, resilience, labor, environment, or finance review;
- safer public wording that avoids berth-as-port or map-as-proof drift.

Use GitHub issue templates for local adaptations and source/claim corrections.
Pull requests should use `.github/PULL_REQUEST_TEMPLATE.md`.

## Gate

Decision: **open_for_reference_review_and_adaptation**

Rationale: HARBOR can be reused as an inspectable maritime-gateway analysis
pattern. Reuse alone does not create engineering, dredging, environmental,
leasing, funding, procurement, endorsement, or validation claims.
