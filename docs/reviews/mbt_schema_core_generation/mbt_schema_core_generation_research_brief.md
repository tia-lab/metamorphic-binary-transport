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

# Research Brief: MBT Schema Core Generation

Status: draft for spec authoring

Slug: `mbt_schema_core_generation`

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
docs/protocols/peer_audit_protocol.md
```

Existing MBT repository reads:

```text
docs/specs/mbt_workspace_architecture_SPEC.md
docs/specs/mbt_core_runtime_migration_SPEC.md
docs/specs/mbt_codegen_migration_SPEC.md
docs/reviews/mbt_codegen_migration/mbt_codegen_migration_result_review.md
proto/mathilde/options.proto
crates/core/src/runtime.rs
crates/core/src/envelope.rs
crates/core/src/error.rs
crates/codegen/src/config.rs
crates/codegen/src/descriptor.rs
crates/codegen/src/model.rs
crates/codegen/src/rust_emit.rs
crates/codegen/src/emit.rs
Cargo.toml
```

Experiment source read:

```text
/media/Development/MATHILDE/experiments/crates/schema/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto
```

## Measured Object

The measured object is the first committed schema-core generation proof in the
new MBT workspace:

```text
MBT-only all-fields proto
  -> mbt_codegen --surface core
  -> generated Rust schema module
  -> schema crate compile and correctness tests
```

This phase measures whether the current core-only codegen surface can generate
and compile a real schema crate covering the complete MBT physical field set
that was developed in the experiment crate.

This phase does not measure JSON, protobuf, CSV, Arrow, Arrow IPC, Parquet,
projection, transponding, MDB, cache, lookup, or serving behavior.

## Candidate Approach

Create an approved workspace schema crate under the reserved schema path:

```text
crates/schemas/test_compatibility_core/
```

The crate will contain an MBT-only compatibility proto derived from the
experiment `all_fields.proto`. The derived proto must preserve:

- field numbers;
- field names;
- MBT dictionaries;
- MBT physical field annotations;
- key-part annotations;
- presence-bit annotations;
- repeated payload annotation;
- projection declarations and projection groups as parsed MBT options.

The derived proto must remove annotations outside this repository boundary:

- cache route annotations;
- cache table annotations;
- cache column annotations;
- DB table annotations;
- DB column annotations;
- lookup or MLDB annotations.

Codegen then writes a generated Rust module into the schema crate. The module
is committed as a generated artifact only if it is reproducible by the bound
`mbt_codegen --check` command.

## Evidence Table

| Evidence type | Observation |
| --- | --- |
| Code-read evidence | `docs/specs/mbt_workspace_architecture_SPEC.md` reserves `crates/schemas/[schema_name]/` for later approved workspace schema crates. |
| Code-read evidence | `docs/reviews/mbt_codegen_migration/mbt_codegen_migration_result_review.md` records `crates/codegen` as implemented and validated for explicit `--proto-root`, `--schema`, `--root`, `--module`, and `--surface core` inputs. |
| Code-read evidence | `proto/mathilde/options.proto` in this repository contains MBT-only option definitions and does not contain DB/cache/lookup options. |
| Code-read evidence | `crates/codegen/src/config.rs` supports only `Surface::Core`, which matches this phase boundary. |
| Code-read evidence | `crates/codegen/src/descriptor.rs` maps supported protobuf kinds to MBT `FieldKind` values: dictionaries, bitmask dictionaries, `i32`, `u32`, `i64`, `f32`, `f64`, `bool`, `bytes`, raw strings, and numeric arrays. |
| Code-read evidence | `crates/codegen/src/descriptor.rs` rejects unannotated strings, unsupported repeated bool/bytes, nullable bitmask dictionaries, duplicate presence bits, presence gaps, duplicate key orders, and key-order gaps. |
| Code-read evidence | `crates/codegen/src/rust_emit.rs` emits core-only generated code: schema constants, dictionaries, presence constants, row/payload structs, validation, checksums, runtime API, `MbtSchema`, view types, and decode helpers. |
| Code-read evidence | `crates/core/src/runtime.rs` defines the schema contract used by generated modules: `MbtSchema`, `encode`, `encode_owned`, `access`, and `inspect`. |
| Code-read evidence | `crates/core/src/envelope.rs` owns the fixed header, schema identity validation, checksum validation, and trusted payload identity/length validation. |
| Code-read evidence | Experiment `all_fields.proto` includes DB/cache annotations that are outside MBT core scope, but its MBT physical field set covers the broad compatibility surface needed for this schema proof. |
| Build evidence | Existing codegen migration result review records `cargo check --workspace`, `cargo test -p metamorphic_binary_transport_codegen`, and `cargo clippy -p metamorphic_binary_transport_codegen --all-targets -- -D warnings` passing before this schema phase. |
| Benchmark evidence | None. This phase does not claim runtime throughput. |
| Hypothesis | A committed MBT-only all-fields schema crate can compile without adapter dependencies and prove generated core support for all MBT physical field kinds. This remains unproved until the implementation plan is executed. |

## MBT Physical Field Coverage From Source

The experiment `all_fields.proto` provides source coverage for:

- `ConstU16`;
- required and optional `U16Dictionary`;
- `U64BitmaskDictionary`;
- `I64`;
- optional `I64`;
- `I32`;
- optional `I32`;
- `U32`;
- optional `U32`;
- `F64`;
- optional `F64`;
- `F32`;
- optional `F32`;
- `Bool`;
- optional `Bool`;
- `Bytes`;
- optional `Bytes`;
- `RawString`;
- optional `RawString`;
- `I64Array`;
- nullable `I64Array`;
- `I32Array`;
- nullable `I32Array`;
- `U32Array`;
- nullable `U32Array`;
- `F64Array`;
- nullable `F64Array`;
- `F32Array`;
- nullable `F32Array`.

Nullable arrays in this source are represented by `repeated` protobuf fields
plus an MBT `presence_bit`. The presence bit distinguishes absent/null from an
empty-but-present array at the MBT row level.

## Unknowns

1. The exact generated line count for the all-fields schema crate is unknown
   until codegen runs against the MBT-only fixture.
2. The exact `cargo check` wall time and memory for the generated schema crate
   are unknown until the crate is added and measured.
3. The existing codegen unit tests prove fixture compilation through a temporary
   smoke crate, but they do not yet prove a committed workspace schema crate.
4. Wide presence above 64 nullable fields is implemented in the current emitter
   and covered by codegen tests, but the selected all-fields fixture has 14
   nullable fields. This phase proves complete physical field kind coverage, not
   a production-wide schema compile budget.

## Risks

1. Importing the experiment proto unchanged would pull DB/cache options into
   this repository and violate the MBT boundary.
2. Hand-editing generated Rust would violate generated-code invariants.
3. Adding adapter dependencies to the schema crate would recreate the compile
   surface this repository split was designed to avoid.
4. Relying only on codegen unit tests would leave schema-crate integration
   unproved.
5. Using `PartialEq`-based row equality in tests would pressure generated row
   derives that the lean generated-surface invariant intentionally avoids.

## Required Decisions Before Spec

Decisions carried into the spec:

1. The schema proof uses a derived MBT-only compatibility proto.
2. The schema crate path is `crates/schemas/test_compatibility_core/`.
3. The generated Rust module is committed only as reproducible codegen output.
4. The schema crate depends on `metamorphic_binary_transport_core` and `rkyv`
   only.
5. Codegen remains a tool surface and is not a runtime dependency of the schema
   crate.
6. Correctness tests compare fields explicitly, not by requiring generated row
   `PartialEq`.

## Recommended Next Phase

Write:

```text
docs/specs/mbt_schema_core_generation_SPEC.md
```

Then run a separate peer audit before writing any implementation plan.
