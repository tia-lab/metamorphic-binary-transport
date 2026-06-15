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

# MBT Metamorphose Migration Research Brief

Status: draft  
Date: 2026-06-15  
Slug: `mbt_metamorphose_migration`

## Source Materials

Protocol reads:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/research_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/peer_audit_protocol.md`

Current repository reads:

- `docs/specs/mbt_workspace_architecture_SPEC.md`
- `docs/specs/mbt_codegen_migration_SPEC.md`
- `docs/specs/mbt_schema_core_generation_SPEC.md`
- `docs/specs/mbt_projection_migration_SPEC.md`
- `docs/specs/mbt_projection_direct_writer_SPEC.md`
- `docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_result_review.md`
- `docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_result_peer_audit_v2.md`
- `crates/metamorphose/src/lib.rs`
- `crates/metamorphose/src/runtime.rs`
- `crates/transponding/src/lib.rs`
- `crates/transponding/src/runtime.rs`
- `crates/adapters/*/src/lib.rs`
- `crates/codegen/src/config.rs`
- `crates/codegen/src/emit.rs`
- `crates/codegen/src/rust_emit.rs`
- `crates/codegen/src/model.rs`
- `crates/schemas/bars_core/src/bars_v1.rs`
- `crates/benches/src/projection.rs`
- `crates/benches/src/bin/mbt_projection_bench.rs`

Old experiment reads:

- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/README.md`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/metamorphose/mod.rs`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/bars_v1.rs`
- `/media/Development/MATHILDE/experiments/docs/specs/mathilde_binary_transport_metamorphose_SPEC.md`
- `/media/Development/MATHILDE/experiments/docs/specs/mathilde_binary_transport_transponding_SPEC.md`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/docs/bench_results.md`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/docs/test_results.md`

No external documentation was required for this research pass.

## Measured Object

The measured object for the next spec is the migration of the old MBT
metamorphose and hidden transponding behavior into the new split workspace:

```text
validated MBT bytes
  -> generated schema-specific adapter writer
  -> requested boundary format
```

For columnar boundary formats:

```text
validated MBT bytes
  -> hidden generated transponding kernel
  -> adapter-specific columnar writer
  -> requested boundary format
```

The public user concept remains `metamorphose::`. Transponding is not a public
user API.

## Candidate Approach

Preserve the old public behavior while isolating compile surfaces:

- `crates/metamorphose` owns public traits, enums, helper functions, and shared
  checked-output helpers.
- `crates/adapters/json`, `protobuf`, `csv`, `arrow`, `arrow_ipc`, and
  `parquet` own concrete format writers and format dependencies.
- `crates/transponding` owns shared columnar buffer contracts only.
- schema crates stay core/projection-only by default.
- codegen emits optional schema-local adapter modules behind explicit schema
  crate features.
- row-oriented adapters use generated direct writers.
- columnar adapters call hidden generated transponding kernels automatically.

This preserves the old ergonomic shape:

```rust
metamorphose::json::<BarsV1>(&bytes, max_response_bytes)?;
metamorphose::arrow::<BarsV1>(&bytes, max_response_bytes)?;
metamorphose::parquet::<BarsV1>(&bytes, max_response_bytes)?;
```

without making core schema builds compile every adapter.

## Evidence Table

| Evidence type | Observation |
| --- | --- |
| Code-read evidence | Old `src/metamorphose/mod.rs` exposes `metamorphose::mbt/json/protobuf/csv/arrow/arrow_ipc/parquet` functions and schema traits. |
| Code-read evidence | Old generated `bars_v1.rs` defines `pub(crate) fn transpond`, `transpond_archived`, and `transpond_rows_direct`; the public Arrow, Arrow IPC, and Parquet methods call `transpond_archived` internally. |
| Code-read evidence | Old README states transponding is internal and that the public boundary concept remains `metamorphose::`. |
| Code-read evidence | Current new repo `crates/metamorphose`, `crates/transponding`, and `crates/adapters/*` are placeholders only. |
| Code-read evidence | Current codegen `Surface` supports only `Core` and `Projection`; no adapter or transponding generation exists yet. |
| Code-read evidence | Current generated schema crates expose `pub(crate) access_archived` and public unsafe trusted bytes access, which allows schema-local generated adapter modules to call archived access without exposing archived references publicly. |
| Run evidence | None produced in this research pass. |
| Build evidence | Projection direct writer result review records current core/schema/bench compile evidence after the previous migration slice. |
| Benchmark evidence | Old MBT `bench_results.md` records metamorphose, transponding, Arrow, Arrow IPC, Parquet, and CSV lanes. |
| Schema evidence | Current Bars and test compatibility schemas are generated by the new codegen and already cover core/projection surfaces. |
| Hypothesis | Optional feature-gated schema-local adapter modules can preserve old public API while avoiding default compile-surface growth. This must be proved by build evidence. |
| Hypothesis | Old direct writer patterns can be preserved or improved by forbidding DTO materialization, serde/prost DTOs, public transponding, extra row `Vec`s, and static field-name allocation. This must be proved by source tests and benchmarks. |

## Unknowns

- Whether the first implementation should migrate all boundary formats in one
  plan or split row formats and columnar formats into separate implementation
  plans.
- Whether Arrow, Arrow IPC, and Parquet dependencies should be pinned to the
  exact old versions or refreshed under a separate dependency audit.
- Whether generated adapter modules should be one file per format or one file
  per group. The spec should prefer one file per format for compile-surface and
  review clarity.

## Risks

- Emitting adapter code into core generated schema files would recreate the
  compile-surface problem this repository was created to avoid.
- Generic reflection-style adapter writers would violate schema-specific hot
  path invariants and risk performance regression.
- Exposing transponding as a user API would drift from the old proven
  architecture.
- Benchmarks may accidentally measure source validation, projection, or setup
  work instead of the intended adapter writer path.
- Columnar targets necessarily allocate output/column buffers; the spec must
  distinguish declared copy points from hidden overhead.

## Required Decisions Before Spec

Resolved by this research pass:

- Public API remains `metamorphose::`.
- Transponding remains hidden and selected automatically by columnar adapters.
- Schema crates remain adapter-free by default.
- Adapter-specific generated schema modules are feature-gated.
- Projection remains MBT-to-MBT before metamorphose.

Still deferred to implementation planning:

- exact dependency versions for Arrow, Arrow IPC, Parquet, and any protobuf
  writer dependency if a dependency is needed;
- exact benchmark run count and stability threshold, except that old-result
  parity and a spread rule must be required.

## Recommended Next Phase

Write `docs/specs/mbt_metamorphose_migration_SPEC.md`.

The spec must bind:

- old API preservation;
- hidden transponding contract;
- row and columnar adapter boundaries;
- feature-gated generated adapter module strategy;
- no generated adapter code in default schema builds;
- exact codegen surface extensions;
- benchmark parity against old MBT result labels;
- compile-surface checks proving selected adapters do not pull unrelated
  adapters.
