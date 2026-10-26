# Source Registry

Every cited corpus quantity points to a source id in this registry. Entries can
be expanded as the corpus grows; a source id is stable once cited.

| Source ID | Label | Citation | Notes |
|---|---|---|---|
| `SRC-BTS-PORT-PERFORMANCE` | source-needed | Bureau of Transportation Statistics Port Performance Freight Statistics Program | Port throughput, vessel calls, and dwell-adjacent operational indicators. |
| `SRC-USACE-WATERBORNE-COMMERCE` | source-needed | U.S. Army Corps of Engineers Waterborne Commerce Statistics Center | Domestic tonnage and waterborne commerce movements. |
| `SRC-UNCTAD-MARITIME` | source-needed | UNCTAD Review of Maritime Transport / port connectivity data | International maritime connectivity and trade context. |
| `SRC-AIS-PROXY` | proxy | AIS-derived vessel movement datasets | Proxy-only lane, dwell, and call-pattern evidence until observed data is available. |
| `SRC-CENSUS-TRADE` | source-needed | U.S. Census international trade data | Market and corridor trade flows for national/regional analysis. |
| `usace-channel-depth-2023` | source-needed | U.S. Army Corps of Engineers harbor/channel project depths and port-authority published channel depths (MLW), 2022–2024 | Deep-draft channel depths used for DIM-01 adequacy. LA/LB San Pedro main channel ~-53 ft; NY/NJ Harbor Deepening -50 ft (2016); Norfolk/Virginia -50 ft (deepening to -55 ft underway); Oakland -50 ft; NWSA (Seattle ~-50 ft / Tacoma Blair -51 ft); Savannah Harbor Expansion (SHEP) -47 ft (USACE, 2022); Houston Ship Channel ~-45 ft (USACE Project 11). |
| `port-teu-2023` | source-needed | Port-authority calendar-year 2023 container volume reports and BTS port container rankings | Loaded+empty TEU rounded to one decimal (million TEU). Context only; not scored. |

## Source-custody correction — 2026-10-26

The two historical IDs above are broad references, not verified record-level
citations. All sixteen quantities across the eight gateway records are now
`source-needed`; retained values and illustrative scores are unchanged. No
original measurement is verified by a hash of a repository corpus file.
See [source-audit.md](source-audit.md) for the required primary-source gate.

## Frozen Los Angeles publisher captures — separate from the historical corpus

| Source ID | Evidence surface | Exact locator |
|---|---|---|
| `port-la-2023-table-20261026` | Frozen publisher table; local gateway throughput | [2023 table](https://portoflosangeles.org/business/statistics/container-statistics/historical-teu-statistics-2023), table summary `2023 Container Statistics`, twelve month rows, seven quantity columns, calendar total and separate partial fiscal row. |
| `port-la-2024-news-20261026` | Frozen January 16, 2024 news release; whole-TEU precision | [Publisher news](https://portoflosangeles.org/references/2024-news-releases/news_011624_jan_cargo), annual-total paragraph and December-total opening paragraph. |

Exact response bytes, UTC capture times, SHA-256 hashes, source JSON and third-party
notice: `data/port-la-2023/`. Offline verifier: `tools/verify-port-la.py`.
The source table and news differ by 4,815.90 TEU annually; their cause/revision
chronology and table update date remain unknown. Both round to 8.6 million, which
does not recover the historical original. Partial FY2023/24 is not a calendar
observation. These source IDs are not silently substituted into the old corpus.
