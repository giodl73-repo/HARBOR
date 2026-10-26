# HARBOR

**Ports 2.0 — multi-scale maritime gateway and intermodal freight analysis.**

**A port is not a berth. It is a synchronized promise from channel to inland market.**

HARBOR scores ports, terminals, lanes, and hinterland connections across
throughput, dwell, channel access, intermodal connectivity, resilience, and
competition. It treats the gateway as a network rather than blaming every delay
on the waterfront.

**Series:** [Applied Systems](https://github.com/giodl73-repo/giodl73-repo/blob/main/series/applied-systems.md)

## Show someone (start here)

| Audience | Path | Time |
|---|---|---|
| **Planner / researcher** | [SHOWCASE.md](SHOWCASE.md) → [deep-draft channel finding](docs/findings/2026-06-deep-draft-channel-tail.md) | 15–25 min |
| **CLI implementer** | [SHOWCASE.md](SHOWCASE.md) → `cargo run -p harbor-cli -- gap --input corpus --scale national` | 10–20 min |

Research lab only — not dredging design, NEPA, terminal lease advice, or agency
endorsement. Optimize the service chain, not the most visible asset.

## Infrastructure 2.0 family

HARBOR is one domain implementation of a shared evidence-first method:

```text
PUBLIC SOURCES → CORPUS → SCORE → SERVICE PROMISE → GAP MAP
                                                     ↓
                                      CONCEPT → REVIEW → DESIGN
```

| Lane | Repositories |
|------|--------------|
| Movement | [ROUTE](https://github.com/giodl73-repo/ROUTE), [GAUGE](https://github.com/giodl73-repo/GAUGE), [TARMAC](https://github.com/giodl73-repo/TARMAC), [HARBOR](https://github.com/giodl73-repo/HARBOR) |
| Lifelines | [PYLON](https://github.com/giodl73-repo/PYLON), [PACKET](https://github.com/giodl73-repo/PACKET), [BASIN](https://github.com/giodl73-repo/BASIN), [DRAIN](https://github.com/giodl73-repo/DRAIN) |
| Public access | [SHIELD](https://github.com/giodl73-repo/SHIELD), [SLATE](https://github.com/giodl73-repo/SLATE) |
| Civic boundaries | [ZONES](https://github.com/giodl73-repo/ZONES) |

The family shares evidence labels, explicit scale and demand bases, T1–T4
service promises where meaningful, adversarial review, and acceptance of a
rigorous null result. Each repository owns its domain semantics and safety
boundary.

> HARBOR is a research and conceptual-design project. It is not an engineering
> study, dredging design, environmental review, terminal lease, or advocacy
> brief, and it claims no MARAD, port-authority, carrier, or IMO endorsement.

## Use HARBOR

[Open the gateway-distribution workbench](https://giodl73-repo.github.io/HARBOR/)
to replay the repository’s June 2026 eight-gateway illustration and change
hypothetical scores. The unchanged native gap detector runs in a worker as
WebAssembly: compare its minimum alert with concentrated-tail/systemic membership,
mark a score missing, and see why clearing the assessed dimension does not clear
twelve unassessed dimensions. This is a historical model replay with 2023 source
context, not current channel clearance or a dredging recommendation. Inputs are
neither uploaded nor stored; results, exact source records and WASM are downloadable.

[Inspect frozen Los Angeles container sources](https://giodl73-repo.github.io/HARBOR/#container-source)
to compare twelve months of 2023 loaded and total TEU, reconcile all seven
quantity columns and see the difference between the publisher's table and its
January 16, 2024 news release. Both totals round to the old 8.6-million context;
that does not establish an exact historical value. The separate partial fiscal
row is excluded from calendar sums. Capture date, unknown table-update date and
unknown discrepancy cause remain visible. This local gateway source task does
not alter the national hypothetical model or recover its original citations.
Original publisher responses, fixture, custody and results are downloadable.
Native source query: `cargo run --locked -p harbor-web --bin harbor-browser -- cargo fixtures/cargo-all.json`.
Offline source verification: `python tools/verify-port-la.py`.
See [WP-WEB-003](docs/vtrace/WP_WEB_003.md) for the bounded contract.

Build: `python tools/build-pages.py` with Rust 1.95.0 and the WASM target.
Verify: locked workspace tests/strict Clippy, `npm ci`,
`npx playwright install chromium`, then `npm test`. Native replay:
`cargo run -p harbor-web --bin harbor-browser -- fixtures/browser-reset.json`.
See [WP-WEB-001](docs/vtrace/WP_WEB_001.md) for scope and role review.

HARBOR is public and open to use as a reference model, historical detector
illustration, diagnostic pattern, review discipline, or local adaptation starting
point.

### Reuse boundary

HARBOR is currently a specialist maritime-gateway analysis product, not a
supported cross-repository library. Its network model, evidence taxonomy,
DIM-01..13 scoring, service tiers, gap policy, and CLI remain product-local; no
portfolio repository pins a `harbor-*` crate or owns compatibility proof.

Infrastructure 2.0 siblings share and adapt an evidence-first method, not a
versioned HARBOR dependency. Family source similarities, findings, worksheets,
and local adaptations are not stable provider contracts. Direct reuse requires
a named downstream consumer, a bounded versioned surface, and consumer-owned
compatibility tests.

If you want to apply it to a gateway, port, terminal district, channel-access
problem, hinterland connection, or maritime service question, start with
[`docs/adoption/README.md`](docs/adoption/README.md). It lays out safe reuse,
first adaptation steps, contribution targets, and claim boundaries.

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

The corrected historical
[gateway-score illustration](docs/findings/2026-06-deep-draft-channel-tail.md)
covers eight US gateways. Its retained scores name Houston and Savannah in the
model tail where a minimum-only detector names all eight. The sixteen physical
quantities lack original-source custody and are labelled `source-needed`; this
is detector behavior, not a demonstrated navigation deficit.
See the [source-custody audit](data/source-audit.md) and inspect it directly in
the [workbench](https://giodl73-repo.github.io/HARBOR/#source-audit), even if WASM
cannot load. The superseded original finding is preserved byte for byte.

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
- [`docs/adoption/`](docs/adoption) — open reuse, local adaptation, and review path.
- [`docs/vtrace/`](docs/vtrace) — VTRACE requirements, architecture, trace, and verification.
- [`context/waves/`](context/waves) — repo-local execution history.
- [`.roles/ROLE.md`](.roles/ROLE.md) — adversarial review panel.

## License

HARBOR uses separate licenses for software and content. Source code,
executable scripts, tests, configuration, and ordinary software
documentation are MIT-licensed (copyright Gio Della-Libera). Original
non-software content is licensed CC BY-NC 4.0 (copyright Gio Della-Libera);
commercial use of that content requires separate written permission.
Third-party material remains under its own terms.
See [LICENSE](./LICENSE) for the complete notice.
