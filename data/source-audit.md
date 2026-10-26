# Historical gateway source-custody audit

Audit date: 2026-10-26. Scope: all eight `corpus/us-*.md` records.

The repository retains eight channel-depth quantities and eight rounded 2023
TEU quantities. The source registry supplies broad publisher/project descriptions
for two source IDs. It does not supply original source bytes, page/table/row
locators, per-record publication dates or record-level URLs for those sixteen
quantities. All sixteen are now labelled `source-needed`. This audit does not
establish that the values are wrong, and does not independently verify any of them.

Exact copies and SHA-256 hashes of corpus files demonstrate custody of derivative
repository records, not custody or verification of the original measurements.
Scores and fixture bytes remain unchanged so the Rust detector illustration is
reproducible. The historical 50-foot / 7-point bar and depth-to-score transform
are modeling choices, not a demonstrated clearance rule. TEU stays unscored.

## What a reader can inspect

`source-audit.json` in the Pages artifact lists every retained quantity and its
source ID, current evidence label, unresolved primary-source custody, corpus path,
corpus hash and illustrative score. The browser displays the same generated rows
without requiring WASM. It cannot convert absent evidence into clearance.

The historical finding is preserved byte for byte at
`data/history/2026-06-deep-draft-channel-tail-original.md`:
SHA-256 `93f59553afd315aba5ca610794311b95444a015240212aebfc89cd86e4103f72`. That superseded text contains unsupported physical-navigation
and intervention-target conclusions. The corrected finding limits its conclusion
to the behavior of detectors on the retained model distribution.

## Next evidence gate

For each depth: retain the original publisher file with exact citation/locator,
publication and observation dates, channel reach and datum, and distinguish
project/authorized, maintained and surveyed depth. Separately establish the
specific vessel draft, tide and under-keel/operating constraints before any
navigation claim. For TEU: retain the original table and calendar/fiscal window,
loaded/empty coverage, gateway/terminal aggregation and rounding rule. Do not
assume BTS loaded TEU and port-authority total TEU have the same denominator.

Only then review each source-to-quantity-to-score mapping through the maritime
and citation roles. Recheck dates, units and scales independently. Add dwell,
hinterland and other observations on their own evidence basis; twelve unassessed
dimensions remain coverage gaps. No fiscal admission, savings, engineering,
dredging or navigation authority is granted by this audit.
