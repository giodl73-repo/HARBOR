# Deep-draft channel adequacy: a concentrated tail, three detectors

Date: 2026-06
Scale: national
Rubric: HARBOR v0 (`UNDER_SERVED_THRESHOLD = 7.0`)
Corpus: `corpus/us-*.md` (8 US container gateways)
Dimension assessed: DIM-01 deep-draft channel adequacy (only)
Reproduce: `cargo run -p harbor-cli -- --scale national gap --input corpus`

## Question

Does HARBOR's gap detector behave correctly on a *concentrated* adequacy
distribution, and does the PACKET tail/dispersion signal generalize from a
spread-out distribution (US broadband adoption) to a clustered one (US deep-draft
channel depth)?

## Data

DIM-01 scores channel depth versus the ~50 ft all-tide draft a fully-laden
ultra-large container vessel (ULCV) needs. Transform: `clamp(depth_ft - 43, 0, 10)`,
so 50 ft maps to the 7.0 baseline. All depths are cited
(`usace-channel-depth-2023`); TEU is recorded as cited context only
(`port-teu-2023`) and is **not** scored, to avoid conflating port size with
deficiency.

| Port | Channel depth (ft, MLW) | DIM-01 | 2023 TEU (M, context) |
|---|---|---|---|
| Los Angeles | 53 | 10.0 | 8.6 |
| Long Beach | 53 | 10.0 | 8.0 |
| New York / New Jersey | 50 | 7.0 | 7.8 |
| Virginia (Norfolk) | 50 | 7.0 | 3.7 |
| Oakland | 50 | 7.0 | 2.1 |
| Northwest Seaport Alliance | 50 | 7.0 | 3.1 |
| Savannah | 47 | 4.0 | 4.9 |
| Houston | 45 | 2.0 | 3.8 |

DIM-01: mean 6.75, min 2.0, bottom-quartile (2 lowest) mean 3.0.

## Result: the same deficit, seen three different ways

The run emits 12 `EmptyRegion` gaps (DIM-02..DIM-13 are unassessed — honest
coverage gaps, not deficits) and two competing reads of DIM-01:

| Detector | Logic | Fires? | Names | Verdict |
|---|---|---|---|---|
| Mean @ 5.0 (PACKET-old) | corpus mean < bar | No (6.75 ≥ 5.0) | — | **Misses** a real deficit |
| Min @ 7.0 (HARBOR `UnderServedRegion`) | lowest < bar | Yes (2.0) | **all 8 ports** | Fires, but **mis-attributes** — blames LA/LB (depth 10) for Houston's shortfall |
| Tail @ 7.0 (`TailRegion`, new) | bottom-quartile mean < bar | Yes (3.0) | **Houston, Savannah** | Fires **and localizes** to the genuinely sub-baseline ports |

The actual deficit is real and narrow: Houston (45 ft) and Savannah (47 ft)
cannot take a fully-laden ULCV at all tides; the other six gateways are at or
above the 50 ft baseline. Only the tail detector both fires and points at the
right ports.

## Cross-repo finding: the portfolio's detectors disagree by construction

PACKET shipped a **mean-based** detector and HARBOR a **min-based** one. On this
corpus they disagree:

- PACKET's mean test (bar 5.0) would call this region null — the broadband
  finding already showed mean tests are blind to a deficient minority.
- HARBOR's min test fires but cannot distinguish a one-port outlier from a
  systemic shortfall; its `entry_ids` list every scored entry, so the "region"
  is operationally meaningless for targeting.

The dispersion/tail signal validated on PACKET's spread-out broadband data
reproduces cleanly on HARBOR's clustered channel-depth data: it is the only one
of the three that is both sensitive (catches the deficit a mean test misses) and
specific (names the tail a min test smears across the whole range).

## Recommendation

Standardize portfolio gap detection on the tail/dispersion signal (keep the
min-based region as a coarse alarm, but treat `TailRegion.entry_ids` as the
actionable target set). Propagate the tail detector to the remaining repos that
still ship only the older single-statistic detectors.

## Honesty notes

- TEU is **not** scored. Low throughput is not treated as a deficiency; channel
  depth is the only adequacy claim, and every depth is cited.
- 12 of 13 dimensions are unassessed and reported as explicit `EmptyRegion`
  coverage gaps, not as passing scores.
- The 50 ft → 7.0 baseline is a documented modeling choice; ports at exactly the
  baseline are treated as adequate, not deficient.
