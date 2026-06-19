# MBT Projection Metamorphose Adapter Research Brief

## Status

Status: corrected research brief for full projection-marker adapter coverage.

Repository:

```text
/home/tia/_DEV/MATHILDE/metamorphic-binary-transport
```

Task class: research and spec authoring only.

No code change is authorized by this brief.

This brief supersedes the earlier JSON-only brief for the same slug. The
JSON-only scope was too narrow because MBT projections are transport-level
schemas, and every requested boundary adapter must be able to target a
projected marker exactly as it targets a source marker.

## Source Materials

Required protocol reads completed:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/research_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/protocols/implementation_protocol.md`
- `docs/protocols/codegen_protocol.md`

Relevant prior specs read:

- `docs/specs/mbt_projection_direct_writer_SPEC.md`
- `docs/specs/mbt_metamorphose_migration_SPEC.md`
- `/home/tia/_DEV/MATHILDE/experiments/docs/specs/serving_new_mbt_cache_rebuild_SPEC.md`

Relevant code paths read:

- `crates/codegen/src/rust_emit.rs`
- `crates/codegen/src/emit.rs`
- `crates/codegen/src/config.rs`
- `crates/codegen/src/tests/test_rust_emit_metamorphose.rs`
- `crates/codegen/src/tests/test_rust_emit_projection_direct_writer.rs`
- `crates/codegen/src/tests/mod.rs`
- `crates/schemas/bars_core/src/bars_v1.rs`
- `crates/schemas/bars_core/src/bars_v1_json.rs`
- `crates/schemas/bars_core/src/bars_v1_csv.rs`
- `crates/schemas/bars_core/src/bars_v1_protobuf.rs`
- `crates/schemas/bars_core/src/bars_v1_transponding.rs`
- `crates/schemas/bars_core/src/bars_v1_arrow.rs`
- `crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs`
- `crates/schemas/bars_core/src/bars_v1_parquet.rs`
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs`
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_json.rs`
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_csv.rs`
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_protobuf.rs`
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_transponding.rs`

## Measured Object

The measured object is generated adapter availability and correctness for
proto-declared MBT projection markers.

The required architecture is:

```text
source MBT bytes
  -> generated source projection API
  -> projected MBT bytes
  -> generated adapter API for the projected marker
  -> requested boundary output
```

The adapter is selected by the existing codegen surface:

```text
--surface metamorphose --adapter <adapter>
```

The corrected scope covers every currently supported adapter selector:

```text
json
protobuf
csv
transponding
arrow
arrow-ipc
parquet
```

Transponding remains hidden. It is generated for projected markers only because
Arrow, Arrow IPC, and Parquet need the same schema-local row-to-column kernel
for projected payloads.

## Candidate Approach

Extend `generated_metamorphose_adapter_schema(model, adapter)` so each adapter
emits:

- source marker adapter support exactly as today;
- adapter support for every projection marker in `model.projections`;
- scoped helper symbols for projection sections;
- projection-specific row-format output metadata where row formats need it;
- projection-specific columnar metadata where transponding and columnar
  adapters need it.

The implementation should use the projection `SchemaModel` derived from
`projection_schema_model(source, projection)`, then complete the adapter-facing
metadata that the current projection model intentionally clears:

- `derived_utc_fields`;
- `json_csv_output_fields`;
- `protobuf_messages`.

The adapter must not perform projection. Projection remains MBT-to-MBT before
adapter conversion.

## Evidence Table

| Type | Evidence |
| --- | --- |
| Code-read evidence | `docs/invariants/core_invariants.md` states: `Projection, when present, is MBT-to-MBT before boundary conversion.` |
| Code-read evidence | `docs/specs/mbt_metamorphose_migration_SPEC.md` states: generated adapter modules must work for both source schemas and projected schemas emitted by the projection surface. |
| Code-read evidence | `crates/codegen/src/rust_emit.rs` dispatches every adapter through `generated_metamorphose_adapter_schema(model, adapter)` and currently emits only one source marker section per adapter. |
| Code-read evidence | `crates/codegen/src/rust_emit.rs` already derives projection schema models inside `generated_projection_schema(model)` with `projection_schema_model(model, projection)`. |
| Code-read evidence | `crates/codegen/src/rust_emit.rs` has `EmitScope::projection`, which provides deterministic scoped names for projection constants and helper functions. |
| Code-read evidence | `projection_schema_model` currently clears `derived_utc_fields`, `json_csv_output_fields`, and `protobuf_messages`; row-format projection adapters therefore need explicit projected metadata derivation. |
| Code-read evidence | Current generated Bars adapter files contain source marker impls such as `impl JsonMetamorphoseSchema for BarsV1`, but no projection marker impls such as `impl JsonMetamorphoseSchema for BarsV1NoMetadata`. |
| Code-read evidence | Current generated columnar adapters call `Self::transpond_archived(...)`; projected columnar adapter support therefore requires projected marker transponding support as well. |
| Code-read evidence | `crates/codegen/src/tests/test_rust_emit_metamorphose.rs` proves source adapter generation for all adapter selectors, but has no projection marker adapter matrix test. |
| Code-read evidence | `crates/codegen/src/tests/mod.rs` contains `valid_alias_and_projection_ignored_proto()`, a non-serving fixture with a projection. It can prove generic behavior without Bars hardcoding. |
| Hypothesis | Full projection-marker adapter support can be implemented entirely in `crates/codegen/src/rust_emit.rs` plus tests/generated artifacts, with no MBT core or adapter crate changes. This must be verified by the implementation plan and validation. |

## Unknowns

The implementation plan must verify these before code:

- exact projected protobuf message derivation from the projected physical field
  set;
- exact handling of derived UTC fields when their source physical field is
  removed by a projection;
- exact helper-name scoping for every adapter selector;
- whether the generated test fixtures already contain enough projections to
  prove all adapters without adding a new fixture;
- compile-surface impact of adding projection sections to each requested
  adapter file.

No unknown authorizes code before the corrected spec and implementation plan
are approved.

## Risks

1. Incomplete adapter matrix risk:
   implementing JSON only leaves protobuf, CSV, and columnar adapters with a
   different architecture than projection MBT.

2. Helper collision risk:
   source and projection adapter sections live in one generated file. Projection
   constants and helper functions must use `EmitScope::projection`.

3. Empty output metadata risk:
   current projection schema models clear row-format metadata. The generator
   must rebuild projected JSON/CSV/protobuf metadata instead of emitting empty
   rows or empty protobuf messages.

4. Columnar dependency risk:
   Arrow, Arrow IPC, and Parquet depend on hidden transponding. Projection
   marker columnar support must include generated projected transponding,
   without exposing transponding as public API.

5. Schema-specific hardcoding risk:
   codegen must not contain Bars, no-metadata, serving, or crate-name branches.

6. Compile-surface risk:
   adapter files grow with projection count. Growth must remain inside the
   selected adapter feature and not affect default schema builds.

## Required Decisions Before Spec

The corrected spec must bind:

- projection adapter support applies to every currently supported adapter
  selector;
- the adapter generated file for a selector owns both source marker and
  projection marker sections for that selector;
- projection helper symbols are scoped;
- source helper symbols remain stable where possible;
- projection row-format metadata is derived from the projected schema;
- projection columnar metadata is derived from the projected schema;
- transponding remains hidden;
- no schema-specific MBT artifacts are added to the MBT repo for serving;
- serving remains downstream and owns its local schema generation.

## Evidence Required Before Coding

Before implementation, the spec and implementation plan must bind:

- exact codegen source files to edit;
- exact tests proving projection marker support for all adapter selectors;
- exact generated files to regenerate for Bars and test-compatibility schema
  crates;
- exact codegen write/check commands for every adapter file;
- exact compile-surface checks;
- exact forbidden-source checks proving no serving/Bars hardcoding in codegen;
- exact schema runtime tests for at least Bars no-metadata projection and one
  non-Bars compatibility projection.

## Recommended Next Phase

Amend:

```text
docs/specs/mbt_projection_metamorphose_adapter_SPEC.md
```

The next peer audit should be a separate artifact:

```text
docs/reviews/mbt_projection_metamorphose_adapter/mbt_projection_metamorphose_adapter_peer_audit_v4.md
```
