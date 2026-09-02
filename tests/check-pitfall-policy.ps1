$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot

function Read-RepoFile {
    param([string]$Path)
    $fullPath = Join-Path $repoRoot $Path
    if (-not (Test-Path -LiteralPath $fullPath)) {
        throw "Missing required file: $Path"
    }
    Get-Content -LiteralPath $fullPath -Raw
}

function Assert-Contains {
    param(
        [string]$Content,
        [string]$Needle,
        [string]$Label
    )
    $normalizedContent = ($Content -split '\s+') -join ' '
    $normalizedNeedle = ($Needle -split '\s+') -join ' '
    if (-not $normalizedContent.Contains($normalizedNeedle)) {
        throw "$Label missing expected text: $Needle"
    }
}

$pitfalls = Read-RepoFile ".pitfall/harbor-pitfalls.md"
$invariants = Read-RepoFile ".pitfall/harbor-invariants.md"
$readme = Read-RepoFile "README.md"
$plan = Read-RepoFile "PRODUCT_PLAN.md"
$claude = Read-RepoFile "CLAUDE.md"
$roles = Read-RepoFile ".roles/ROLE.md"
$adoption = Read-RepoFile "docs/adoption/README.md"
$finding = Read-RepoFile "docs/findings/2026-06-deep-draft-channel-tail.md"
$review = Read-RepoFile "docs/vtrace/REVIEW.md"
$boundaries = Read-RepoFile "docs/pitfall-boundaries.v1.json"

Assert-Contains $pitfalls "## HARBOR-PF-05: First Port Finding Becomes Public Authority" "pitfall section"
Assert-Contains $pitfalls "**Status:** MITIGATED" "pitfall status"
Assert-Contains $pitfalls "tests/check-pitfall-policy.ps1" "pitfall retained test"
Assert-Contains $invariants "Public Reuse Requires Boundary Review" "public boundary invariant"
Assert-Contains $invariants "**Status:** MITIGATED" "public boundary invariant status"
Assert-Contains $invariants "DIM-01-only scope" "public boundary invariant enforcement"
Assert-Contains $invariants "declared scale" "public boundary invariant enforcement"

Assert-Contains $readme "Research lab only" "README research boundary"
Assert-Contains $readme "not dredging design, NEPA, terminal lease advice, or agency endorsement" "README research boundary"
Assert-Contains $readme "engineering" "README authority boundary"
Assert-Contains $readme "study" "README authority boundary"
Assert-Contains $readme "dredging design" "README authority boundary"
Assert-Contains $readme "environmental review" "README authority boundary"
Assert-Contains $readme "terminal lease" "README authority boundary"
Assert-Contains $readme "advocacy" "README authority boundary"
Assert-Contains $readme "brief" "README authority boundary"
Assert-Contains $readme "Direct reuse requires a named downstream consumer, a bounded versioned surface, and consumer-owned compatibility tests" "README provider boundary"
Assert-Contains $plan "Additional dwell, hinterland, resilience, and competition runs remain next steps" "product plan future work boundary"
Assert-Contains $plan "No dredging, berth, terminal, environmental, lease, or concession design" "product plan non-goals"
Assert-Contains $claude "No throughput or dwell claim dressed as solved engineering" "CLAUDE engineering boundary"
Assert-Contains $claude "Do not start implementation code until the relevant work package is accepted" "CLAUDE implementation gate"

Assert-Contains $roles "First public port finding" "role public finding PITFALL gate"
Assert-Contains $roles "Port-System Planner" "role planner gate"
Assert-Contains $roles "Port-Authority & Alliance Realist" "role authority gate"
Assert-Contains $roles "Citation Auditor" "role citation gate"
Assert-Contains $roles "Scope Keeper" "role scope gate"
Assert-Contains $roles "Numeracy Checker" "role numeracy gate"

Assert-Contains $adoption "Public use does not create an engineering study, dredging design, environmental review, terminal lease, advocacy brief" "adoption authority boundary"
Assert-Contains $adoption "Reuse alone does not create engineering, dredging, environmental, leasing, funding, procurement, endorsement, or validation claims" "adoption reuse boundary"
Assert-Contains $finding "Scale: national" "finding scale"
Assert-Contains $finding "Dimension assessed: DIM-01 deep-draft channel adequacy (only)" "finding DIM-01-only scope"
Assert-Contains $finding "TEU is recorded as cited context only" "finding context-only note"
Assert-Contains $finding "12 of 13 dimensions are unassessed and reported as explicit" "finding unassessed dimensions"
Assert-Contains $finding "EmptyRegion" "finding unassessed dimensions"
Assert-Contains $review "Role Review Matrix" "review role matrix"
Assert-Contains $review "PITFALL Boundary" "review PITFALL boundary"
Assert-Contains $review "No finding currently has public-authority status" "review public authority boundary"

Assert-Contains $boundaries '"schema": "pitfall-boundaries.v1"' "boundary schema"
Assert-Contains $boundaries "first-port-finding-is-not-public-authority" "boundary rule"
Assert-Contains $boundaries "research-lab evidence only" "boundary research posture"
Assert-Contains $boundaries "They do not authorize dredging design, terminal lease advice, NEPA or permitting evidence, funding direction, port-authority position, agency endorsement, carrier endorsement, or IMO endorsement" "boundary held claims"
Assert-Contains $boundaries "named release artifact, visible unassessed dimensions, cited corpus/source basis, declared scale, reproducible run, and full role review" "boundary promotion prerequisites"

Write-Output "HARBOR PITFALL policy checks passed"
