# HARBOR Product Plan

## Thesis

Score maritime gateways and freight connections at a declared scale, identify
measurable capacity and connectivity gaps, and design Ports 2.0 interventions
only where evidence supports them.

## Implemented product shape

- Six-crate Rust workspace covering network, corpus, score, tier, gap, and CLI.
- International, national, regional, and local scale contracts.
- DIM-01..13 scoring and tier-SLA shortfall artifacts.
- Tail-versus-systemic gap classification.
- Deterministic tests and machine-readable CLI outputs.

## Current evidence

The first cited channel-adequacy analysis covers eight US gateways and
demonstrates why concentrated deficits must not be reported as a system-wide
failure. Additional dwell, hinterland, resilience, and competition runs remain
next steps.

## Next public work

1. Publish reproducible source manifests for cited gateway runs.
2. Expand dwell, channel, rail/truck access, and resilience evidence.
3. Add sensitivity analysis across gateway and terminal scales.
4. Review the first gap-targeted intervention through the full panel.

## Non-goals

- No dredging, berth, terminal, environmental, lease, or concession design.
- No forecast of what ports, carriers, or agencies will build.
- No uncited throughput, dwell, depth, capacity, or cost claim.
- No aggregation across scales without an explicit comparison basis.

## Validation

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
cargo run -p harbor-cli -- --help
```
