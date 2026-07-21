# HARBOR

**Ports 2.0 — multi-scale maritime gateway and intermodal freight analysis.**

**A port is not a berth. It is a synchronized promise from channel to inland market.**

HARBOR scores ports, terminals, lanes, and hinterland connections across
throughput, dwell, channel access, intermodal connectivity, resilience, and
competition. It treats the gateway as a network rather than blaming every delay
on the waterfront.

**Series:** [Applied Systems](https://github.com/giodl73-repo/giodl73-repo/blob/main/series/applied-systems.md)

> HARBOR is a research and conceptual-design project. It is not an engineering
> study, dredging design, environmental review, terminal lease, or advocacy
> brief, and it claims no MARAD, port-authority, carrier, or IMO endorsement.

## Why this matters

Deep water without rail capacity, terminal throughput without gate flow, and
berth capacity without labor or inland resilience do not produce a reliable
gateway. HARBOR keeps those dependencies visible and forces each proposed
intervention to name the measured gap it addresses.

The transferable principle is: **optimize the service chain, not the most
visible asset.**

## What is implemented

| Crate | Responsibility |
|---|---|
| `harbor-network` | Port, terminal, lane, and hinterland network contracts. |
| `harbor-corpus` | Evidence-labelled maritime corpus parsing and validation. |
| `harbor-score` | DIM-01..13 score artifacts. |
| `harbor-tier` | Tier-SLA classification and shortfall reporting. |
| `harbor-gap` | Scale-filtered gap analysis and null-result reporting. |
| `harbor-cli` | CLI front door for corpus, score, tier-SLA, and gap commands. |

## Evidence

The first cited
[deep-draft channel analysis](docs/findings/2026-06-deep-draft-channel-tail.md)
covers eight US gateways. Its concentrated distribution names Houston and
Savannah where a minimum-only detector would overgeneralize; the systemic
classifier keeps that distinction explicit.

## Quick start

```powershell
cargo run -p harbor-cli -- corpus --input corpus --scale national
cargo run -p harbor-cli -- gap --input corpus --scale national
cargo test --workspace
```

## Method

```text
CORPUS -> SCORE -> TIER-SLA -> GAP -> CONCEPT -> REVIEW -> DESIGN
```

Every element carries a scale and market. Throughput and dwell claims retain
their evidence and demand basis.

## Documentation

- [`PRODUCT_PLAN.md`](PRODUCT_PLAN.md) — scope, product shape, and next work.
- [`docs/vtrace/`](docs/vtrace) — VTRACE requirements, architecture, trace, and verification.
- [`context/waves/`](context/waves) — repo-local execution history.
- [`.roles/ROLE.md`](.roles/ROLE.md) — adversarial review panel.

## License

MIT. See [`LICENSE`](LICENSE).
