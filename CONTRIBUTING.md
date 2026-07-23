# Contributing

Keep HARBOR scale-aware, evidence-labelled, and explicit about the difference
between analysis and engineering, permitting, leasing, or advocacy.

Useful public contributions include gateway source inventories, channel or dwell
corrections, terminal and hinterland review, carrier or resilience notes, and
safer public language that prevents dredging, leasing, endorsement, or advocacy
drift. For local adaptations, start with
[`docs/adoption/README.md`](docs/adoption/README.md).

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
cargo run -p harbor-cli -- --help
```

Do not commit raw restricted datasets, credentials, local build state, or
uncited public claims.
