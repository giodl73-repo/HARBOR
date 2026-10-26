# WP-WEB-003 — Frozen Los Angeles container-source reconciliation

Status: accepted for implementation, 2026-10-26.

## Reader task and source perimeter

Inspect the Port of Los Angeles's 2023 monthly container table and compare its
calendar-year total with the separate January 16, 2024 publisher news release.
Retain exact public response bodies, capture dates, source URLs, hashes, table
rows/columns and the publisher disclaimer. Serve captured HTML as plain text,
never execute it or copy external scripts/images into the app. Source facts are
third-party, excluded from HARBOR's original-content/software license grants.

Use a shared native Rust module in harbor-web to sum integer centi-TEU values,
reconcile all seven reported quantity columns, show loaded versus total TEU,
filter all months/one month/no month, and expose signed source differences.
Both source totals round to the old 8.6-million context. Rounded agreement is not
proof of an exact value or recovery of the historical corpus's source vintage.
The discrepancy's cause and revision chronology are unknown. The table's partial
FY2023/24 row is separately retained and excluded from calendar-year sums.
News whole-TEU precision stays explicit; no causal revision attribution follows.

The existing national eight-gateway score model, physical quantities and source-
needed labels, thresholds, twelve unassessed dimensions and canonical outputs
are unchanged. Add a separate local-gateway source task; do not infer national
coverage, capacity, dwell, vessel clearance, operational feasibility or fiscal
benefits from throughput. Keep explicit null observations and false authorities.
No new runtime dependencies. TAXLANE updates only its transportation provider
description/caveat and regression assertions, then validates its browser route. Other satellites, ICELINES and BISECT are no-op for this package.

## Implementation and acceptance

Before Rust implementation, this contract reaches the implementing-agent role
fixed point below. Build-time standard-library Python independently extracts
all table cells from captured HTML with Decimal conversion, matches the frozen
JSON byte-semantics and original hashes, rejects source/row/column drift and
checks news figures and publisher dates. Cargo runs offline from retained data.
The original 2026-06 finding remains exactly preserved. No live browser fetch to
the publisher is required for the task. Raw HTML downloads remain optional.

Native fixtures all/loaded/December/empty/rejected input must prove fixed-source
arithmetic, unit/denominator separation, source differences and null/authority
boundaries. Actual WASM must match native canonical bytes. Browser tests cover
source edits, stale-result clearing, reset/download, empty selection, strict
input validation, loading/failure, independent score replay, keyboard/mobile,
source links/hashes and project prefix. Keep all prior cases passing. Run fmt,
locked workspace tests/check, strict native and WASM Clippy, docs/CLI examples;
artifact <5 MB. PR gates precede master-only deployment; exact live artifact,
source bytes, browser behavior and TAXLANE roundtrip precede scoped root snapshot.

## Role review notes — preimplementation fixed point

Implementing-agent lenses, not an independent panel or publisher endorsement:

- Port-system planner: pass; one gateway's calendar-year source context cannot
  establish national adequacy or change the national model scores.
- Maritime engineer: pass; TEU is throughput, not vessel count, draft, berth,
  channel, yard or crane capacity. Defer those to a cited operational package.
- Operations/logistics officer: pass; monthly totals do not establish dwell,
  surge performance or reliable delivery. Defer these to a cited gateway run.
- Maritime economist: pass; no volume-to-benefit/cost/savings inference; loaded
  and total units stay separate. Defer economics to a cited intervention package.
- Regional-hinterland advocate: pass; no inland connectivity or distributional
  conclusion. Defer access evidence to a cited regional/local service package.
- Environmental/community advocate: pass; no inferred emissions, community gain
  or dredging need. Defer environmental evidence to a cited intervention package.
- Port-authority/alliance realist: pass; no assumed operator/labor/carrier action.
  Defer control and delivery evidence to a cited operational package.
- Citation auditor: resolved: retain both publisher responses and exact locators;
  distinguish present capture date, news publication date and unknown table
  update date. Preserve the difference rather than silently choosing a total.
- Scope keeper: pass; separate local source task, no original-corpus promotion,
  engineering, navigation, fiscal admission or rate authority.
- Numeracy checker: resolved: integer centi-TEU arithmetic, seven columns with
  row/component/sum residuals, explicit news whole-TEU precision, partial fiscal
  row excluded; 8.6-million rounding is not exact agreement.

Shipper, drayage, longshore, carrier and community lenses retain their separate
cost, reliability, labor, access and environmental unknowns with the deferrals
above. No unresolved critical/major actionable finding remains in the contract.
Repeat all ten lenses at exit. Rollback: revert this task and redeploy the prior
artifact; retain source capture/correction evidence in the release record.

## Implementation exit — 2026-10-26

All 47 pinned Rust 1.95.0 workspace tests pass; formatting, locked all-target
check and strict native/WASM Clippy pass. Fifteen local actual-WASM browser cases
pass: six new source cases and nine prior cases, with four source and three
legacy native canonical-byte comparisons. After replacing the new worker-failure
test's fixed delay with an explicit loading/abort handshake, that case passes
again. The test waits for the loading state before releasing the intercepted
worker request; no product assertion is relaxed. Desktop and mobile source-task
screenshots were inspected. The full local artifact is about 881 KB, <5 MB;
original publisher payloads are 362,027 bytes and remain optional downloads.

All seven Parliament and three editorial implementing-agent lenses repeat pass.
Resolved citation/numeracy findings: preserve both publisher surfaces, compare
full precision despite equal rounding, distinguish capture/news/unknown-update
dates, separate loaded/total and calendar/partial-fiscal denominators, retain
explicit original-citation holds. The response-byte hashes, all 84 monthly
quantity cells, seven annual values, seven partial-fiscal values, table labels
and two news values verify offline. Original HTML is served as text; attributes
preserve bytes and avoid treating third-party whitespace as authored source.
Eight service observations remain null and seven authority flags false. All
operational, economic, geographic and environmental deferrals above remain.
No independent panel, original-corpus promotion or fiscal result is claimed.

TAXLANE's five browser cases pass locally after its bounded provider-note update.
Hosted PR checks, exact clean default-branch deployment, live browser/source-byte
checks and the transportation roundtrip remain publication/snapshot exit gates.
