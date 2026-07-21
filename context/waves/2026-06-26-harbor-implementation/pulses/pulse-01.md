# Pulse 01: WP-001 `harbor-network` port/freight kernel

Status: pending. Executes WP-001 (see `docs/vtrace/WORK_PACKAGES.md`).

## Scope

The port/freight graph kernel — the pipeline primitive every other crate depends on. Implements
the load-bearing identity, connectivity, and typed demand basis (peak-season/average) invariants
required by REQ-007.

## Planned changes

- `Cargo.toml` workspace (member `crates/harbor-network`).
- `crates/harbor-network/Cargo.toml` (deps: `petgraph`, `serde`, `thiserror`).
- `crates/harbor-network/src/lib.rs`: `Port`, `Lane` (with typed `DemandBasis` enum), `Network`,
  `NetworkError`; `add_port`/`add_lane` (identity + validation); `port_count`, `lane_count`,
  `degree`, `is_connected`, `has_diverse_path`, `incident_capacity_teu`.

## Parent IDs

REQ-004/005/007 · SPEC-001/005 · IF-005 · PKG-001 · CR-001..008.

## Exit criteria

- Workspace compiles; `cargo test -p harbor-network` green.
- Tests cover: build network; degree; connectivity vs gap; incident capacity; demand basis
  preserved (peak-season/average); `has_diverse_path` true on a ring/mesh and false on a
  single-path chain; duplicate-port, non-positive capacity, unknown-port typed errors.
- No `unwrap`/`panic!` in lib paths except tests; `clippy -D warnings` clean.

## Validation

```powershell
cargo fmt --check
cargo clippy --workspace -- -D warnings
cargo test -p harbor-network
```

## VTRACE closeout (on completion)

VER-004/005/007 + EVID-CR-001..003 → passed; TRACE REQ-004/005/007 → implemented; WORK_PACKAGES
WP-001 → done; unblock WP-002.

## Status

Completed — the six-crate workspace and validation baseline are implemented.
