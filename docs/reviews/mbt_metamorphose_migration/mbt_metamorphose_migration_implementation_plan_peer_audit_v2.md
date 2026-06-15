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

# Implementation Plan Peer Audit V2: MBT Metamorphose Migration

Status: `PEER_AUDIT_PASSED`
Date: 2026-06-15
Slug: `mbt_metamorphose_migration`

Audited plan:

```text
docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md
```

Prior implementation plan audit:

```text
docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan_peer_audit.md
```

Approved spec:

```text
docs/specs/mbt_metamorphose_migration_SPEC.md
```

## Required Reads

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/protocols/implementation_protocol.md`
- `docs/protocols/code_style_protocol.md`
- `docs/protocols/codegen_protocol.md`
- `docs/protocols/testing_benchmark_protocol.md`
- `docs/specs/mbt_metamorphose_migration_SPEC.md`
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_peer_audit.md`
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan_peer_audit.md`
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md`

## Audit Scope

This audit checks the amended implementation plan after the first
implementation-plan peer audit. It does not authorize code by itself. Code may
start only after owner approval of the audited plan.

Audit lenses:

- prior blocker closure;
- dependency containment;
- crate boundary isolation;
- failure contract closure;
- generated artifact ownership;
- validation command completeness;
- rollback boundary.

## Prior Blocker Closure

### 1. Arrow IPC and Parquet no longer depend on the Arrow adapter crate

The first audit blocked the plan because Arrow IPC and Parquet depended on
`metamorphic_binary_transport_adapter_arrow`.

The amended dependency table no longer contains that dependency for either
adapter.

Code-read evidence:

- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:200-207`

The plan now explicitly states:

```text
Arrow IPC and Parquet must not depend on
metamorphic_binary_transport_adapter_arrow.
```

Code-read evidence:

- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:210-212`

The selected feature gate also pulls only the selected adapter crate plus
`transponding` for columnar outputs.

Code-read evidence:

- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:707-722`

Decision: resolved.

### 2. Adapter failure mapping is now bound without core edits

The first audit blocked the plan because adapter failure mapping was left open
while core edits were forbidden.

The amended plan now says no edit to `crates/core/src/error.rs` is authorized
and binds all adapter failures to existing `TransportError` variants or
compile/codegen-time failures.

Code-read evidence:

- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:230-261`

The plan also states that if a dependency API cannot use the table without a
new `TransportError` variant, that adapter is blocked for this implementation
plan.

Code-read evidence:

- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:260-261`

Decision: resolved.

## Additional Audit Checks

### Dependency containment

The amended plan keeps:

- `crates/core` dependency-free from adapters;
- `crates/projection` untouched;
- default schema builds core-only;
- adapter dependencies behind selected schema features.

Code-read evidence:

- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:78-96`
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:182-228`
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:665-733`

No blocker found.

### Generated artifact ownership

The plan binds one `mbt_codegen --write` command per generated adapter file for
Bars and test compatibility, and requires matching `--check` commands by
replacement.

Code-read evidence:

- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:736-764`
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:970-971`

No blocker found.

### Transponding remains hidden

The plan keeps transponding as generated schema-local support and blocks public
transponding API exposure.

Code-read evidence:

- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:93-105`
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:616-679`
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:789-832`

No blocker found.

### Tests and benchmark evidence are bound

The plan binds:

- narrow adapter tests;
- codegen tests;
- generated schema tests;
- compile-surface checks;
- dependency-tree checks;
- three release benchmark runs;
- evidence files under `docs/evidence/mbt_metamorphose_migration`.

Code-read evidence:

- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:817-859`
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:886-1000`

No blocker found.

## Non-Blocking Observations

### A. RecordBatch bridge code is duplicated per selected columnar adapter by design

The plan allows schema-local RecordBatch bridge statements in `*_arrow.rs`,
`*_arrow_ipc.rs`, and `*_parquet.rs`, while still forbidding duplicated
transponding logic.

Code-read evidence:

- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:214-225`
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:672-679`

This is acceptable under the approved spec because the duplication is
adapter-local bridge code, not a second row-to-column pass.

### B. Adapter error detail may be coarse under current core boundary

The plan maps Arrow, IPC, and Parquet construction/writer failures to
`TransportError::MalformedArchive(message)`.

Code-read evidence:

- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:247-249`
- `docs/reviews/mbt_metamorphose_migration/mbt_metamorphose_migration_implementation_plan.md:1051-1053`

This is acceptable for this implementation plan because core error changes are
explicitly forbidden and the mapping is now bound. If later result review shows
operator ambiguity, that should be handled by a separate audited spec.

## Decision

`PEER_AUDIT_PASSED`

The amended implementation plan is closed enough to proceed after explicit
owner approval. This audit does not waive the implementation protocol: code
must still stay inside the approved plan, generated files must come only from
codegen, and performance claims require the bound evidence.

## Next Required Gate

Implementation requires explicit owner approval of the audited implementation
plan.
