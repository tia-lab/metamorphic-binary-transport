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

# Peer Audit V2: MBT Projection Migration

Status: `BLOCKED`

Slug: `mbt_projection_migration`

Target research brief:

```text
docs/reviews/mbt_projection_migration/mbt_projection_migration_research_brief.md
```

Target spec:

```text
docs/specs/mbt_projection_migration_SPEC.md
```

Previous audit:

```text
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit.md
```

## Required Reads

Completed reads:

```text
AGENTS.md
docs/invariants/core_invariants.md
docs/protocols/lifecycle_protocol.md
docs/protocols/spec_protocol.md
docs/protocols/peer_audit_protocol.md
docs/reviews/mbt_projection_migration/mbt_projection_migration_research_brief.md
docs/reviews/mbt_projection_migration/mbt_projection_migration_peer_audit.md
docs/specs/mbt_projection_migration_SPEC.md
docs/specs/mbt_codegen_migration_SPEC.md
docs/specs/mbt_schema_core_generation_SPEC.md
proto/mathilde/options.proto
crates/codegen/src/options.rs
crates/codegen/src/model.rs
crates/codegen/src/descriptor.rs
crates/codegen/src/rust_emit.rs
crates/schemas/test_compatibility_core/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

## Findings

### 1. The Spec Conflicts With Earlier Approved `--surface core` Contracts

Severity: blocking

The amended projection spec now follows the required section order and defines
the exact codegen-check command. However, it still binds projection generation
to:

```text
--surface core
```

Earlier approved specs explicitly define the opposite behavior.

`docs/specs/mbt_codegen_migration_SPEC.md` states:

```text
--surface core must parse these extensions only enough to recognize them
--surface core must not construct projection output schemas
projection definitions and projection groups do not affect generated core output
projection emission is deferred to a later projection codegen spec
```

`docs/specs/mbt_schema_core_generation_SPEC.md` states that its generated file:

```text
emits no adapter, projection, transponding, MDB, cache, lookup, or MLDB code
```

The projection migration spec is that later projection codegen spec, but it
does not explicitly say whether it:

1. supersedes the earlier `--surface core` no-projection clauses for declared
   projections; or
2. introduces a new codegen surface such as `--surface projection`; or
3. keeps `--surface core` source-only and uses a separate command/surface for
   projection output.

Without that decision, implementation would either violate the new projection
spec or violate the already approved codegen/schema-core specs.

Required amendment:

- add an explicit supersession section or subsection under the codegen
  contract;
- state exactly which clauses from `mbt_codegen_migration_SPEC.md` and
  `mbt_schema_core_generation_SPEC.md` are superseded by this projection
  migration;
- choose and bind one command model:
  - `--surface core` now emits declared MBT-to-MBT projections; or
  - a new `--surface projection` is introduced; or
  - another explicit two-step command model;
- update the exact codegen-check command to match that choice;
- update code bindings if the choice requires `crates/codegen/src/config.rs`
  or other CLI files.

### 2. The Spec Does Not Bind Test Changes For Existing Projection-Ignored Assertions

Severity: blocking

Code-read evidence shows an existing codegen test contract:

```text
crates/codegen/src/tests/test_descriptor.rs
alias_and_projection_do_not_affect_core_hash_or_fields
```

That test was correct for the core-only phase. It currently asserts projection
annotations do not affect source core hash or fields. The amended spec says
core source hash remains unchanged, but projected schemas are emitted and have
their own hashes.

The spec binds `test_descriptor.rs` as an allowed file, but it does not state
the required test-contract change:

```text
old: projection declarations are ignored by core output
new: projection declarations do not affect source schema hash, but do produce
     projection models/output under the approved projection command surface
```

Required amendment:

- add this exact existing-test migration to the test plan;
- require a test proving projection declarations do not affect source hash;
- require a separate test proving projection declarations produce projected
  models/output under the approved command surface.

## Resolved Previous Audit Blockers

The previous peer audit blockers are resolved except for the new conflict
above:

| Previous blocker | V2 result |
| --- | --- |
| Mandatory section order missing | Resolved |
| Exact codegen-check command deferred | Resolved, but command surface conflicts with older specs |
| Projected schema hash under-specified | Resolved directionally |
| Compile-surface budget not measurable | Resolved |
| Correctness oracle and test plan combined | Resolved |

## Audit Lens Results

| Lens | Result |
| --- | --- |
| Measured object clarity | Passed |
| Schema source ownership | Passed |
| Wire/archive validation | Passed |
| Trusted-access safety | Passed |
| Codegen determinism | Blocked by unresolved codegen surface conflict |
| Generated-code compile surface | Passed directionally |
| Crate boundary isolation | Passed directionally |
| Dependency containment | Passed |
| Correctness oracle | Passed |
| Benchmark isolation | Passed |
| Runtime performance budget | Passed |
| Failure behavior | Passed |
| Code binding completeness | Blocked because CLI/config binding depends on the unresolved surface choice |
| Generated artifact binding completeness | Passed |
| Client/operator interpretation safety | Blocked by conflicting `--surface core` interpretation |

## Required Amendment Summary

Amend:

```text
docs/specs/mbt_projection_migration_SPEC.md
```

Required changes:

1. Define whether projection generation supersedes the prior `--surface core`
   no-projection contract or uses a new codegen surface.
2. If it supersedes, explicitly list the superseded clauses from:

```text
docs/specs/mbt_codegen_migration_SPEC.md
docs/specs/mbt_schema_core_generation_SPEC.md
```

3. Update exact codegen-check command and code bindings if needed.
4. Add the existing projection-ignored test migration to the test plan.

No code implementation is authorized by this audit.

## Classification

```text
BLOCKED
```
