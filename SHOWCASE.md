# HARBOR Showcase — Ports 2.0

**Who this is for:** someone you would hand the repo to for 15–30 minutes —
a **ports / freight planner** thinking channel-to-inland as one promise chain, or
a **CLI implementer** running maritime corpus → gap.

**Posture:** research and conceptual-design lab. **Not** an engineering study,
dredging design, environmental review, terminal lease, or MARAD / port-authority /
carrier / IMO endorsement.

| Audience | Open this first | Time |
|---|---|---|
| Planner / researcher | [Deep-draft channel finding](docs/findings/2026-06-deep-draft-channel-tail.md) | 15–25 min |
| CLI implementer | README crate table + gap command | 10–20 min |
| Local adapter | [Adoption guide](docs/adoption/README.md) | 15–25 min |

## One-minute pitch

**A port is not a berth. It is a synchronized promise from channel to inland market.**

HARBOR scores ports, terminals, lanes, and hinterland links across throughput,
dwell, channel access, intermodal connectivity, resilience, and competition. It
optimizes the **service chain**, not the most visible waterfront asset.

```text
CORPUS -> SCORE -> TIER-SLA -> GAP -> CONCEPT -> REVIEW -> DESIGN
```

## Two doors

### A. Planner / researcher path

**Question HARBOR answers well:** *Where do channel access, dwell, throughput,
hinterland connection, or resilience constrain a gateway—without blaming every
delay on the berth?*

| Step | What to look at | Why |
|---|---|---|
| 1 | README “Why this matters” + boundary | Claim fence |
| 2 | [2026-06 deep-draft channel analysis](docs/findings/2026-06-deep-draft-channel-tail.md) | Eight US gateways; concentrated tail (e.g. Houston, Savannah) vs min-only overgeneralization |
| 3 | [Adoption guide](docs/adoption/README.md) | Local gateway adaptation |
| 4 | Reproduce | `cargo run -p harbor-cli -- gap --input corpus --scale national` |

**Do not say:** dredge this channel, approve this lease, or carrier schedule
commitment.

### B. CLI implementer path

| Crate | Responsibility |
|---|---|
| `harbor-network` | Port, terminal, lane, hinterland contracts |
| `harbor-corpus` | Evidence-labelled maritime corpus |
| `harbor-score` | DIM-01..13 score artifacts |
| `harbor-tier` | Tier-SLA classification |
| `harbor-gap` | Scale-filtered gaps and nulls |
| `harbor-cli` | Corpus / score / tier / gap front door |

```powershell
cargo run -p harbor-cli -- corpus --input corpus --scale national
cargo run -p harbor-cli -- gap --input corpus --scale national
cargo test --workspace
```

## Claim packet (this showcase)

| Field | Value |
|---|---|
| Claim text | HARBOR can be shown as Ports 2.0 gateway-chain scoring with a cited deep-draft channel finding and scale-aware CLI. |
| Audience | Maritime/freight planners; CLI implementers. |
| Evidence | README; 2026-06 finding; adoption docs; CLI path. |
| Validation | Finding-scoped gateway set; not engineering or environmental clearance. |
| Limitations | First finding is channel-tail focused; full multi-DIM national port program is broader than one showcase run. |
| Non-claims | Dredging design, NEPA, terminal lease advice, official endorsements. |

## Where not to start

| Avoid… | Why |
|---|---|
| Berth-only blame story | Misses hinterland and gate dependencies |
| Unscoped “fix every US port” tour | Logistics bury the chain thesis |

## Related

- Family hub: [`../README.md`](../README.md)
- Product plan: [`PRODUCT_PLAN.md`](PRODUCT_PLAN.md)
