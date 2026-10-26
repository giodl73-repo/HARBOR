# Historical gateway-score illustration: a concentrated tail, three detectors

Original model date: 2026-06
Source-custody correction: 2026-10-26
Evidence: illustrative model; sixteen physical quantities need primary sources
Scale: national
Rubric: HARBOR v0 (`UNDER_SERVED_THRESHOLD = 7.0`)
Corpus: `corpus/us-*.md` (8 US container gateways)
Dimension assessed: DIM-01 illustrative channel-adequacy score (only)
Reproduce: `cargo run -p harbor-cli -- --scale national gap --input corpus`

## Question

Does HARBOR's gap detector behave correctly on a *concentrated* adequacy
distribution, and does the PACKET tail/dispersion signal generalize from a
spread-out distribution (US broadband adoption) to a clustered one (US deep-draft
channel depth)?

## Data

The retained corpus assigns illustrative DIM-01 scores using the historical
transform `clamp(depth_ft - 43, 0, 10)` and a model bar of 7.0. The eight depth
values and eight TEU values below are **unverified historical repository context**.
The broad registry descriptions do not retain original source files or
record-level locators. All sixteen quantities are now `source-needed`; the
scores are preserved to replay detector behavior. TEU is not scored.
Neither this transform nor a 50-foot value establishes vessel/all-tide clearance.
See [the source-custody audit](https://giodl73-repo.github.io/HARBOR/evidence/source-audit.md).

| Port | Retained depth claim (ft, labelled MLW) | DIM-01 | Retained 2023 TEU claim (M) |
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

## Result: the same model distribution, seen three different ways

The run emits 12 `EmptyRegion` gaps (DIM-02..DIM-13 are unassessed — honest
coverage gaps, not deficits) and two competing reads of DIM-01:

| Detector | Logic | Fires? | Names | Verdict |
|---|---|---|---|---|
| Mean @ 5.0 (PACKET-old) | corpus mean < bar | No (6.75 ≥ 5.0) | — | Misses sub-bar illustrative scores |
| Min @ 7.0 (HARBOR `UnderServedRegion`) | lowest < bar | Yes (2.0) | **all 8 ports** | Names all eight scored entries, including the six above the model bar |
| Tail @ 7.0 (`TailRegion`, new) | bottom-quartile mean < bar | Yes (3.0) | **Houston, Savannah** | Names the two sub-bar illustrative scores |

In this model, Houston and Savannah carry the two scores below 7.0. The tail
detector names those two entries while the coarse minimum region names all eight.
This establishes detector behavior on these inputs. It does not establish a real
port deficit, vessel restriction, or an intervention target.

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
of the three that is both sensitive (detects the illustrative low-score tail) and
specific (names the illustrative tail instead of the whole scored set).

## Recommendation

Compare native minimum, tail and systemic outputs as diagnostic model signals.
Do not promote their membership to an operational target set. A future cited
gateway run must retain and independently review original observations and their
score mapping before recommending any domain intervention or cross-repo adoption.

## Honesty notes

- TEU is **not** scored. Low throughput is not treated as a deficiency; depth
  values remain unverified context; the only result is detector behavior.
- 12 of 13 dimensions are unassessed and reported as explicit `EmptyRegion`
  coverage gaps, not as passing scores.
- The 50 ft → 7.0 baseline is a documented modeling choice; scores at exactly the
  baseline are treated as above the model gap trigger, not as cleared ports.

## Correction custody

The [superseded original](https://giodl73-repo.github.io/HARBOR/evidence/original-finding.md)
is retained byte for byte for audit. Its navigation and physical-deficit claims
are withdrawn; the source audit explains the unresolved evidence. Exact hashes
of derivative repository files do not independently verify publisher data.
