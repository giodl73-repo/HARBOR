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
| `usace-channel-depth-2023` | implemented | U.S. Army Corps of Engineers harbor/channel project depths and port-authority published channel depths (MLW), 2022–2024 | Deep-draft channel depths used for DIM-01 adequacy. LA/LB San Pedro main channel ~-53 ft; NY/NJ Harbor Deepening -50 ft (2016); Norfolk/Virginia -50 ft (deepening to -55 ft underway); Oakland -50 ft; NWSA (Seattle ~-50 ft / Tacoma Blair -51 ft); Savannah Harbor Expansion (SHEP) -47 ft (USACE, 2022); Houston Ship Channel ~-45 ft (USACE Project 11). |
| `port-teu-2023` | implemented | Port-authority calendar-year 2023 container volume reports and BTS port container rankings | Loaded+empty TEU rounded to one decimal (million TEU). Context only; not scored. |
