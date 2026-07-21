# HARBOR Corpus Schema

Each corpus entry is a Markdown file with YAML-like frontmatter and line-oriented
quantity and score rows. The stable `id` and typed `scale` are required for safe
joins and scale-filtered analysis.

Required frontmatter keys:

| Key | Values | Notes |
|---|---|---|
| `id` | stable string | Immutable port, terminal, lane, or network identifier. |
| `type` | string | Product element type, e.g. `port`, `lane`, `terminal`. |
| `scale` | `international`, `national`, `regional`, `local` | Required scale tag. |
| `market` | string | Jurisdiction, trade lane, range, or local market. |
| `termini` | `[id, ...]` | Port/network node ids used by graph-backed checks. |
| `tier` | `T1`, `T2`, `T3`, `T4` | Tier model assignment. |
| `sla` | string | SLA record or rationale label. |
| `cross_scale` | `true` / `false` | Optional explicit marker for cross-scale inclusion. |

Quantity rows use:

```text
quantity: <value> | <unit> | <evidence-label> | <source-id-or-> | <demand-basis-or->
```

Evidence labels are `implemented`, `heuristic`, `simulated`, `proxy`, `planned`,
`held`, `source-needed`, and `confidence-limited`. Demand basis values are
`peak-season` or `average`. Score rows use `score: DIM-01 | 0..10` through
`DIM-13`.
