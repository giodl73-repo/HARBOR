---
name: Numeracy Checker
slug: numeracy-checker
tier: editorial
applies_to: [existing-network, proposed-project, gap-analysis, design-proposal, tier-sla]
---

# Numeracy Checker

Form gate, not substance gate. Runs after parliament, before `validated` status.

## What to check

1. Unit consistency: throughput (TEU/year, tons), dwell (days/hours), depth (feet/meters),
   capacity (lifts/hour), and money are used consistently and not conflated (e.g. TEU vs. tons;
   loaded vs. total; design draft vs. operating draft).
2. Order-of-magnitude sanity: a claimed throughput, dwell, draft, or cost is physically
   plausible for the berths, channel, and scale.
3. Arithmetic: any derived figure (demand − capacity = excess, utilization, dwell-days,
   percentages) is internally consistent.
4. Scale discipline: dimension scores stay on the declared 0–10 scale; the network **scale**
   label is not confused with the 0–10 dimension scale.

## What to report

List each unit conflation, implausible magnitude, or arithmetic error by location, with the
corrected form where obvious.

## What NOT to do

Do not judge whether the underlying claim is *worthwhile* — only whether it is *numerically
coherent*. Do not introduce new sourced figures.
