```
MATHILDE PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 MATHILDE. All Rights Reserved.

This document contains trade secrets and confidential information owned
exclusively by MATHILDE, protected under Swiss law (URG, UWG, Art. 162 StGB).

PROHIBITED: Reproduction, copying, distribution, disclosure, or derivative
works without prior written authorization from MATHILDE.

ACCESS REQUIREMENT: Executed NDA with MATHILDE required. Unauthorized access
or possession violates Swiss law. Violations subject to civil remedies,
injunctive relief, damages, and criminal prosecution.

Legal Contact: massimo.nicora@wnlegal.ch
```

# PROTOCOL: MBT Research

Version: 1.0
Status: active
Scope: research-only work

## Purpose

Research builds enough evidence to decide whether a design deserves a spec. It
does not authorize implementation.

## Required Reads

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- this protocol
- relevant existing specs and reviews
- relevant source code
- upstream dependency docs when dependency behavior matters

## Research Output

Write:

- `docs/reviews/[slug]/[slug]_research_brief.md`

The brief must include:

- status,
- source materials,
- measured object,
- candidate approach,
- evidence table,
- hypotheses,
- unknowns,
- risks,
- required decisions before spec,
- recommended next phase.

## Evidence Table

Every brief must separate:

- code-read evidence,
- run evidence,
- build evidence,
- benchmark evidence,
- schema evidence,
- external-doc evidence,
- hypotheses.

## MBT Research Minimum

For MBT work, research must identify:

- schema source and owner,
- root message and proto imports,
- wire/archive boundary,
- validation strategy,
- trusted-access strategy,
- generated-code surface,
- crate boundary impact,
- dependency impact,
- compile-surface risk,
- runtime benchmark baseline,
- correctness oracle,
- expected failure cases.

## Stop Gates

Stop research escalation if:

- schema ownership is unknown,
- wire/archive contract cannot be stated,
- correctness oracle is missing,
- compile-surface risk is ignored,
- dependency behavior is assumed but not read,
- performance baseline cannot be defined.
