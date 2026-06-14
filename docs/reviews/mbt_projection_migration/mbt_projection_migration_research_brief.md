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

# Research Brief: MBT Projection Migration

Status: draft for spec authoring

Slug: `mbt_projection_migration`

Repository:

```text
/home/tia/_DEV/MATHILDE/metamorphic-binary-transport
```

## Source Materials

Protocol reads:

```text
AGENTS.md
docs/invariants/core_invariants.md
docs/protocols/lifecycle_protocol.md
docs/protocols/research_protocol.md
docs/protocols/spec_protocol.md
docs/protocols/codegen_protocol.md
docs/protocols/testing_benchmark_protocol.md
```

Current repository reads:

```text
docs/specs/mbt_workspace_architecture_SPEC.md
docs/specs/mbt_schema_core_generation_SPEC.md
proto/mathilde/options.proto
crates/projection/Cargo.toml
crates/projection/src/lib.rs
crates/projection/src/runtime.rs
crates/codegen/src/options.rs
crates/codegen/src/model.rs
crates/codegen/src/descriptor.rs
crates/codegen/src/rust_emit.rs
crates/codegen/src/tests/test_descriptor.rs
crates/codegen/src/tests/mod.rs
crates/schemas/test_compatibility_core/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

Experiment repository reads:

```text
/media/Development/MATHILDE/experiments/docs/reviews/mathilde_binary_transport_projection/mathilde_binary_transport_projection_research_brief.md
/media/Development/MATHILDE/experiments/docs/reviews/mathilde_binary_transport_projection_fast_path/mathilde_binary_transport_projection_fast_path_result_review.md
/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/tests/test_projection.rs
/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/tests/test_trusted_projection.rs
```

## Measured Object

The measured object is MBT-to-MBT projection migration into the split MBT
workspace:

```text
.proto + mathilde.projection definitions
  -> codegen projection model
  -> generated projected schema marker/row/payload/view/API
  -> source MBT bytes projected to projected MBT bytes
  -> projected bytes validated by the projected schema
```

Projection is a transport operation. It remains MBT-to-MBT. JSON, protobuf,
CSV, Arrow, Arrow IPC, Parquet, and transponding are boundary adapter concerns
and are not part of this phase.

## Candidate Approach

Use the existing repository-wide MBT options:

```text
proto/mathilde/options.proto
```

The option file already defines:

```text
mathilde.projection
mathilde.projection_group
```

No new proto option is required for this phase.

Extend the codegen model so a source `SchemaModel` can carry generated
projection models derived from root `mathilde.projection` declarations. The
current generated schema code already has the checked archived accessor and
trusted archived accessor needed for the efficient projection shape:

```text
checked public bytes
  -> source access_archived
  -> generated archived projection loop
  -> projected encode_owned
  -> projected MBT bytes

trusted immutable bytes
  -> source access_archived_trusted_unchecked
  -> generated archived projection loop
  -> projected encode_owned
  -> projected MBT bytes
```

Use the existing all-fields compatibility schema as the primary fixture:

```text
crates/schemas/test_compatibility_core/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto
```

That proto already declares:

```text
projection name = "no_optional"
projection marker = "TestCompatibilityV1NoOptional"

projection name = "numeric_only"
projection marker = "TestCompatibilityV1NumericOnly"
```

It also contains projection groups and all current MBT scalar, bytes, raw
string, dictionary, bitmask, numeric array, and nullable array field kinds.

## Evidence Table

| Evidence type | Observation |
| --- | --- |
| Code-read evidence | `proto/mathilde/options.proto` defines `ProjectionDefinition`, repeated message option `mathilde.projection`, and field option `mathilde.projection_group`. |
| Code-read evidence | `crates/schemas/test_compatibility_core/proto/.../all_fields.proto` already declares two projections: `no_optional` and `numeric_only`. |
| Code-read evidence | `crates/codegen/src/options.rs` loads `mathilde.projection` and `mathilde.projection_group` descriptors. |
| Code-read evidence | `crates/codegen/src/model.rs` stores `projection_group` on `PhysicalField`, but `SchemaModel` does not yet store projection definitions or projection target models. |
| Code-read evidence | `crates/codegen/src/descriptor.rs` currently propagates inherited projection groups, but it binds `let _projection = &extensions.projection;`, so root projection declarations are intentionally ignored by core-only codegen. |
| Code-read evidence | `crates/codegen/src/rust_emit.rs` emits one source schema only and has no projected marker types, projected row/payload structs, projection APIs, or projected schema hashes. |
| Code-read evidence | `crates/codegen/src/tests/test_descriptor.rs` has a current core-only assertion named `alias_and_projection_do_not_affect_core_hash_or_fields`; projection migration must replace the projection-ignored part of that contract. |
| Code-read evidence | `crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs` has checked `access_archived`, unsafe trusted `access_archived_trusted_unchecked`, and generated `inspect`, which are the necessary source access points for projection. |
| Code-read evidence | `crates/projection/src/lib.rs` and `crates/projection/src/runtime.rs` are placeholders. They do not currently own shared projection runtime logic. |
| Prior experiment evidence | The experiment projection result review records public checked projection, crate-private archived projection, trusted projection, wrong-marker rejection, projected output validation, and public/trusted byte equality as validated in the old monolithic crate. |
| Prior benchmark evidence | The experiment fast-path result review records the intended timing boundary: archived projection removes repeated source-byte validation from projection timing while projected output validation remains separate. |
| Run evidence | None in this repository for projection. No projection performance or correctness claim is made for the split workspace yet. |
| Hypothesis | The split workspace can recover the old projection behavior with codegen-only changes and no MBT core wire/runtime changes. This remains unproved until implementation and validation run. |

## Important Existing Split-Workspace Constraint

The workspace architecture separates crates by compile surface:

```text
crates/core
crates/codegen
crates/projection
crates/metamorphose
crates/transponding
crates/adapters/*
crates/benches
```

Projection is generated schema behavior. This phase should not add adapter
dependencies to schema crates and should not pull `metamorphose` or
`transponding` into projection.

`crates/projection` remains the ownership boundary for future shared projection
traits or helpers. The current code-read evidence does not prove a need for a
runtime helper dependency in generated schema crates. The spec should therefore
default to generated projection code in schema modules and keep
`crates/projection` untouched unless peer audit proves a shared API is needed.

## Unknowns

1. The exact generated code size increase for the all-fields schema is unknown
   until projected marker schemas are emitted.
2. The exact compile-time impact is unknown until the generated compatibility
   schema crate is rebuilt with projections.
3. The old experiment projected schema hash algorithm must be ported or
   re-specified precisely before implementation.
4. The old projection naming rules must be re-bound for the split codegen:
   source marker, projected marker, row type, payload type, view type, rows
   iterator type, and archived row type.
5. The projected wide-presence behavior must be explicitly verified against the
   current codegen presence-page implementation.
6. The correct long-term public trait surface in `crates/projection` is not
   proved by current code-read evidence.

## Risks

1. Emitting projection code by hand would violate generated-code invariants.
2. Reusing the source schema hash for projected bytes would allow wrong marker
   access to succeed and would corrupt schema isolation.
3. Preserving source presence-bit numbers in projected rows would create sparse
   projected presence pages and could reintroduce historical optional-field
   limits.
4. Adding format adapter behavior to projection would violate the MBT-to-MBT
   architecture.
5. Adding a generic projection runtime dependency before proving the need would
   enlarge compile surface without evidence.
6. Testing only row counts would miss field-selection, presence-repacking, and
   wrong-marker regressions.
7. Comparing performance before byte-for-byte correctness would violate the
   lifecycle protocol.

## Required Evidence Before Coding

The implementation plan must bind validation that proves:

- projection definitions are parsed from root message options;
- unknown projection fields/groups fail before emission;
- mandatory schema/key fields cannot be excluded;
- projected schema hash differs from source schema hash;
- source bytes are rejected by projected schema access;
- projected bytes are rejected by source schema access;
- checked projection output validates under the projected marker schema;
- trusted projection output equals checked projection output for immutable
  validated bytes;
- projected presence bits are repacked and support current wide presence pages;
- nullable arrays preserve null versus empty-present semantics;
- generated files are reproducible by `mbt_codegen --check`;
- generated files are not hand-edited.

## Recommended Next Phase

Write and peer-audit:

```text
docs/specs/mbt_projection_migration_SPEC.md
```

Implementation must not begin until that spec and a separate implementation
plan are approved.
