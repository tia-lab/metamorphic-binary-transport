# MBT Core Runtime Migration Research Brief

Status: `COMPLETE_FOR_SPEC_DRAFT`

Slug: `mbt_core_runtime_migration`

## 1. Measured Object

The measured object is the first runtime migration slice from the experiment
crate into the new production workspace:

```text
experiment mathilde_binary_transport core modules
  -> crates/metamorphic_binary_transport_core
```

This slice covers only schema-agnostic MBT core runtime ownership:

- envelope constants and header encode/decode;
- schema header identity struct;
- checked envelope validation;
- trusted payload boundary validation;
- response checksum helper;
- transport error/result surface;
- generic `MbtSchema` runtime dispatch trait.

It does not cover generated schema code, rkyv archive access, projections,
metamorphose, transponding, adapters, codegen, or benchmarks.

## 2. Candidate Approach

Migrate the proven experiment core runtime behavior by selective extraction,
not by copying the old monolithic crate.

Required extraction rule:

```text
copy core algorithms and public contracts
remove schema-specific constants and imports
remove adapter/benchmark/codegen errors and dependencies
keep generated-schema hooks schema-agnostic
```

The core crate should expose only the parts generated schema crates need later:

```text
codec::response_checksum
envelope::{SchemaHeaderSpec, TransportHeader, encode_header, decode_header,
           validate_header_for_schema, trusted_payload_for_schema,
           fnv1a64, normalized_proto_hash, default_build_id}
error::{Result, TransportError}
runtime::{BinaryInspection, MbtSchema, encode, encode_owned, access, inspect}
```

## 3. Source Materials

Repository contract and protocols:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/research_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/protocols/code_style_protocol.md`
- `docs/protocols/testing_benchmark_protocol.md`

Architecture artifacts:

- `docs/specs/mbt_workspace_architecture_SPEC.md`
- `docs/reviews/mbt_workspace_architecture/mbt_workspace_architecture_peer_audit_v2.md`
- `docs/reviews/mbt_workspace_architecture/mbt_workspace_architecture_result_review.md`

New skeleton code:

- `crates/metamorphic_binary_transport_core/Cargo.toml`
- `crates/metamorphic_binary_transport_core/src/lib.rs`
- `crates/metamorphic_binary_transport_core/src/codec.rs`
- `crates/metamorphic_binary_transport_core/src/envelope.rs`
- `crates/metamorphic_binary_transport_core/src/error.rs`
- `crates/metamorphic_binary_transport_core/src/runtime.rs`

Experiment source code:

- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/envelope.rs`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/codec.rs`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/error.rs`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/runtime.rs`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/lib.rs`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/bars_v1.rs`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/tests/test_envelope.rs`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/tests/test_codec.rs`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/tests/test_multi_schema_api.rs`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/tests/test_encode_owned.rs`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/docs/bench_results.md`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/docs/test_results.md`

## 4. Evidence Table

| Evidence type      | Observation                                                                                                                                                                                                                                                                                        |
| ------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Code-read evidence | Experiment `src/envelope.rs` owns `MAGIC`, `HEADER_LEN`, transport version, encoding kind, FNV-1a checksum, fixed 128-byte header encode/decode, checked validation, and trusted payload validation.                                                                                               |
| Code-read evidence | Experiment `src/envelope.rs` also contains Bars-specific constants and `schema_hash()` reading `crate::schema::GENERATED_SCHEMA_HASH`; those are not acceptable in schema-agnostic core.                                                                                                           |
| Code-read evidence | Experiment `src/runtime.rs` defines `BinaryInspection`, `MbtSchema`, and generic `encode`, `encode_owned`, `access`, and `inspect` wrappers without adapter dependencies.                                                                                                                          |
| Code-read evidence | Experiment `src/codec.rs` only wraps `fnv1a64` as `response_checksum`.                                                                                                                                                                                                                             |
| Code-read evidence | Experiment `src/error.rs` mixes transport errors with adapter, CLI, report I/O, JSON, protobuf, Arrow, Parquet, and compression errors. Core migration must remove adapter and benchmark errors from core.                                                                                         |
| Code-read evidence | Experiment `src/lib.rs` exposes adapter modules, benches, generated schemas, schema aliases, codegen, and tests from one crate. The new core must not reproduce that module graph.                                                                                                                 |
| Code-read evidence | Generated `bars_v1.rs` uses core for `SchemaHeaderSpec`, `TransportHeader::new_with_schema`, `encode_header`, `decode_header`, `validate_header_for_schema`, `trusted_payload_for_schema`, `fnv1a64`, `Result`, `TransportError`, `BinaryInspection`, and `MbtSchema`.                             |
| Code-read evidence | Generated `bars_v1.rs` owns rkyv serialization/access and row/archive validation. Core does not need a direct `rkyv` dependency for this migration slice.                                                                                                                                          |
| Build evidence     | Workspace skeleton validation passed in `mbt_workspace_architecture_result_review.md`; core currently has no runtime implementation and no dependencies.                                                                                                                                           |
| Benchmark evidence | Existing experiment benchmark evidence records the current MBT baseline, including large-row `mathilde_binary_generated` at 535,720 rows/sec and 165.53 MB/sec for 100,000 synthetic Bars rows. This is a future parity baseline after schema/bench migration, not proof for this core-only slice. |

## 5. Hypotheses

Hypothesis: extracting the envelope/checksum/runtime dispatch code without
schema or adapter dependencies preserves core runtime behavior while lowering
the production compile surface.

Why not proved yet: the code has not been migrated, and the new core has no
generated schema crate to exercise full encode/access behavior.

Hypothesis: core does not need `rkyv`, Arrow, Parquet, prost, serde_json, zstd,
or benchmark dependencies for this migration slice.

Why suspected: code-read evidence shows rkyv calls are in generated schema code
and adapter dependencies are outside envelope/runtime/codec behavior.

## 6. Unknowns

No unknown blocks the spec draft.

Deferred to later specs:

- generated schema crate migration;
- codegen migration;
- adapter error ownership;
- full benchmark parity against the experiment MBT baseline;
- compile-time comparison once generated schemas are migrated.

## 7. Risks

Risk: copying experiment `envelope.rs` directly would retain Bars-specific
schema constants.

Mitigation: the spec must require `TransportHeader::new_with_schema` and
`validate_header_for_schema` as the schema-agnostic path, and must forbid
Bars-specific constants in core.

Risk: copying experiment `error.rs` directly would pull adapter and benchmark
failure surfaces into core.

Mitigation: the spec must list allowed core error variants and forbid JSON,
protobuf, Arrow, Parquet, compression, CLI, and report I/O errors in core.

Risk: full MBT performance cannot be proved from the core-only slice.

Mitigation: this spec must make no runtime throughput claim and must bind later
schema/benchmark parity before accepting full runtime migration.

Risk: changing the header bytes would invalidate existing baselines.

Mitigation: the spec must preserve `MAGIC`, header length, field offsets,
transport version, encoding kind, flags, FNV constants, and current default
build-id input for this migration.

## 8. Required Decisions Before Spec

Decisions resolved by this brief:

1. Core runtime is schema-agnostic.
2. Core runtime does not own rkyv archive access.
3. Core runtime does not own generated schema constants.
4. Core runtime does not own adapter, benchmark, report, or CLI errors.
5. Full performance parity is deferred until schema and benchmark surfaces are
   migrated.
6. Header wire layout must remain byte-compatible with the experiment runtime.

## 9. Recommended Next Phase

Write `docs/specs/mbt_core_runtime_migration_SPEC.md`.

The spec should bind:

- exact files to edit/create;
- exact forbidden dependencies and modules;
- schema-agnostic header API;
- core-only error surface;
- unit tests for envelope/codec/runtime dispatch;
- build/dependency checks;
- no generated artifacts;
- no performance claim beyond successful structural and unit validation.
