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

# Peer Audit: MBT Schema Core Generation

Status: `BLOCKED`

Slug: `mbt_schema_core_generation`

Target spec:

```text
docs/specs/mbt_schema_core_generation_SPEC.md
```

Research brief:

```text
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_research_brief.md
```

## Required Reads

Completed reads:

```text
AGENTS.md
docs/invariants/core_invariants.md
docs/protocols/lifecycle_protocol.md
docs/protocols/spec_protocol.md
docs/protocols/peer_audit_protocol.md
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_research_brief.md
docs/specs/mbt_schema_core_generation_SPEC.md
docs/reviews/mbt_codegen_migration/mbt_codegen_migration_result_review.md
proto/mathilde/options.proto
crates/core/src/runtime.rs
crates/core/src/envelope.rs
crates/core/src/error.rs
crates/codegen/src/config.rs
crates/codegen/src/descriptor.rs
crates/codegen/src/model.rs
crates/codegen/src/rust_emit.rs
```

## Findings

### BLOCKER 1: Wrong Schema ID And Schema Hash Tests Are Optional

Spec section:

```text
13. Failure contract
```

Problem:

The spec requires:

```text
wrong schema ID or schema hash if practical through header mutation
```

The phrase `if practical` weakens a core failure contract. This repository
requires explicit failure behavior for wrong schema and old/corrupt payload
cases. Header mutation is practical because `crates/core/src/envelope.rs`
defines a fixed 128-byte header, and `crates/core/src/error.rs` exposes
concrete variants:

```text
TransportError::UnknownSchemaId(u32)
TransportError::SchemaHashMismatch { observed, expected }
```

The implementation must not be allowed to skip these tests.

Required amendment:

Replace the optional requirement with mandatory tests:

- mutate the schema ID bytes in an otherwise valid MBT response and assert
  `TransportError::UnknownSchemaId`;
- mutate the schema hash bytes in an otherwise valid MBT response and assert
  `TransportError::SchemaHashMismatch`.

The amendment should keep these tests in:

```text
crates/schemas/test_compatibility_core/tests/test_failures.rs
```

## Non-Blocking Observations

### Observation 1: MBT-Only Fixture Boundary Is Correct

The spec correctly excludes non-MBT annotations from the experiment source:

```text
mathilde.cache_route
mathilde.cache_table
mathilde.cache_field
mathilde.cache_column
mathilde.table
mathilde.column
```

This matches the repository boundary: MBT owns transport options, not DB/cache
schema semantics.

### Observation 2: Schema Crate Path Matches The Reserved Architecture

The spec uses:

```text
crates/schemas/test_compatibility_core
```

That matches the reserved future schema path pattern in the workspace
architecture spec.

### Observation 3: Dependency Boundary Is Narrow

The schema crate dependency contract allows only:

```text
metamorphic_binary_transport_core
rkyv = "=0.8.16"
```

It also forbids codegen, adapters, serde, prost, Arrow, Parquet, SQLite,
Postgres, and Heed. This is consistent with the core/schemas/adapters split.

### Observation 4: Correctness Oracle Avoids Derive Pressure

The spec requires field-for-field comparisons and does not require generated
rows to implement `PartialEq`. That is compatible with the lean generated-code
surface.

## Audit Lens Results

| Lens | Result |
| --- | --- |
| Measured object clarity | Passed |
| Schema source ownership | Passed |
| Wire/archive validation | Blocked by optional schema identity mutation tests |
| Trusted-access safety | Passed |
| Codegen determinism | Passed |
| Generated-code compile surface | Passed |
| Crate boundary isolation | Passed |
| Dependency containment | Passed |
| Correctness oracle | Passed except the blocked failure-contract gap |
| Benchmark isolation | Passed |
| Runtime performance budget | Passed, no throughput claim is made |
| Failure behavior | Blocked |
| Code binding completeness | Passed |
| Generated artifact binding completeness | Passed |
| Client/operator interpretation safety | Passed |

## Required Amendment

Amend:

```text
docs/specs/mbt_schema_core_generation_SPEC.md
```

The amendment must make wrong schema ID and wrong schema hash failure tests
mandatory and bind their expected `TransportError` variants.

After amendment, write:

```text
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_peer_audit_v2.md
```

## Classification

```text
BLOCKED
```
