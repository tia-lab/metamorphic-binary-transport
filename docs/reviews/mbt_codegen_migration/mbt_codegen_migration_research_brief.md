# Research Brief: MBT Codegen Migration

Status: draft for spec authoring

Slug: `mbt_codegen_migration`

Repository: `/home/tia/_DEV/MATHILDE/metamorphic-binary-transport`

Source experiment crate:

```text
/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport
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

Architecture and runtime reads:

```text
docs/architecture/repository_structure.md
docs/specs/mbt_workspace_architecture_SPEC.md
docs/specs/mbt_core_runtime_migration_SPEC.md
docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_result_review.md
```

New repository code-read paths:

```text
crates/core/src/{codec,envelope,error,runtime,lib}.rs
crates/codegen/src/{descriptor,model,options,rust_emit,main,lib}.rs
proto/mathilde/options.proto
Cargo.toml
crates/core/Cargo.toml
crates/codegen/Cargo.toml
```

Experiment code-read paths:

```text
src/codegen/{descriptor,emit,error,model,rust_emit}.rs
src/bin/mbt_codegen.rs
src/generated/bars_v1.rs
src/tests/test_generic_codegen.rs
src/tests/test_array_codegen.rs
src/tests/test_codegen_multi_schema.rs
/media/Development/MATHILDE/experiments/crates/schema/proto/mathilde/options.proto
/media/Development/MATHILDE/experiments/crates/schema/proto/mathilde/binary_transport/v1/bars.proto
/media/Development/MATHILDE/experiments/crates/schema/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto
```

## Measured Object

The measured object is migration of the schema generator boundary into:

```text
crates/codegen
proto/mathilde/options.proto
```

This phase measures and specifies:

- MBT option ownership;
- descriptor loading;
- option validation;
- schema model construction;
- deterministic Rust core-schema emission;
- codegen CLI/check shape;
- crate and dependency isolation.

This phase does not measure runtime transport throughput and does not generate
or commit production schema artifacts.

## Candidate Approach

Migrate the useful parts of the experiment generator into `crates/codegen`,
but narrow the first emitted surface to core MBT binary schema code only.

The experiment generator proves the needed descriptor strategy:

```text
protoc descriptor set
  -> prost_reflect DescriptorPool
  -> custom option descriptors
  -> SchemaModel
  -> deterministic Rust source
```

The production migration must not copy the experiment monolith as-is. The
experiment emitter currently writes all of these surfaces into one generated
module:

```text
core encode/access
MBT projections
JSON/protobuf/CSV writers
transponding column batches
Arrow and Arrow IPC
Parquet
prost DTO sidecar output
```

The first migration should emit only:

```text
owned row structs
rkyv archive structs
schema constants
dictionary constants/helpers
presence constants/helpers
row validation
encode/access/inspect
checked and trusted archive access
MbtSchema implementation against metamorphic_binary_transport_core
```

Projection, metamorphose, transponding, adapter, benchmark, MDB, cache, and
lookup emission remain deferred to later specs.

## Evidence Table

| Evidence type      | Observation                                                                                                                                                                                                                      |
| ------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Code-read evidence | `crates/codegen/src/*.rs` in the new repo are placeholders; `src/main.rs` is empty.                                                                                                                                              |
| Code-read evidence | `crates/core/src/runtime.rs` exposes `MbtSchema`, `encode`, `encode_owned`, `access`, and `inspect`.                                                                                                                             |
| Code-read evidence | `crates/core/src/envelope.rs` owns `SchemaHeaderSpec`, fixed 128-byte header encoding/decoding, checked checksum validation, and trusted payload identity/length validation.                                                     |
| Code-read evidence | Experiment `src/codegen/descriptor.rs` loads descriptor sets with `protoc`, uses `prost_reflect::DescriptorPool`, reads extensions by fully qualified name, builds physical fields, target fields, projections, and schema hash. |
| Code-read evidence | Experiment `src/codegen/model.rs` defines schema model, field kinds, dictionary model, projection model, codegen requests, and canonical schema requests.                                                                        |
| Code-read evidence | Experiment `src/codegen/rust_emit.rs` is 4,155 lines and emits core runtime, projections, boundary writers, transponding, Arrow, Arrow IPC, and Parquet code in one module.                                                      |
| Code-read evidence | Experiment `src/generated/bars_v1.rs` imports Arrow, Parquet, metamorphose, transponding, rkyv, core envelope, and runtime in one generated module.                                                                              |
| Code-read evidence | Experiment `src/bin/mbt_codegen.rs` supports `--write`, `--check`, `--inspect-options`, `--all`, and explicit `--schema/--root/--module`.                                                                                        |
| Code-read evidence | Experiment `crates/schema/proto/mathilde/options.proto` contains MBT transport options plus DB, lookup, MLDB, and cache options. Only MBT transport options are in scope for this repository phase.                              |
| Run evidence       | `wc -l` reported old codegen files at 6,332 total lines and new codegen skeleton/options placeholder at 30 total lines.                                                                                                          |
| Build evidence     | Previous core result review recorded `cargo check --workspace`, core tests, and core clippy passing after core migration.                                                                                                        |
| Hypothesis         | The first generated core-only surface can be substantially smaller than old generated modules because adapter/transponding/projection emission is deferred. This requires implementation evidence later.                         |

## Unknowns

1. Exact generated core-only line count is unknown until the narrowed emitter is
   implemented and run on fixture schemas.
2. Whether the first migration should compile a temporary generated fixture
   crate during tests is an implementation-plan decision, but the spec should
   require at least one compile check before production schema generation.
3. Rustfmt availability must be validated by the implementation plan because
   deterministic output depends on it.
4. `protoc` availability must be validated by the implementation plan because
   descriptor loading uses it.

## Risks

1. Blindly copying the experiment emitter would reintroduce the monolithic
   compile surface this repository was created to remove.
2. Reintroducing canonical hardcoded schema requests would make codegen less
   useful for external application schemas.
3. Migrating DB/cache/lookup options into MBT transport codegen would blur
   repository boundaries.
4. Emitting adapter methods from core schema output would force unrelated
   adapter dependencies into schema users.
5. Wide schemas can still create large generated files; this phase must bind
   line-count and compile-surface evidence even if it does not commit wide
   generated artifacts.

## Required Decisions Before Spec

Decisions made by this brief and carried into the spec:

1. `proto/mathilde/options.proto` will contain MBT transport options only for
   this migration.
2. `crates/codegen` owns descriptor/model/options/emitter/CLI logic.
3. Codegen input is explicit proto roots plus explicit root message and module
   name. No schema intent is inferred from Rust.
4. No production generated schema files are committed by this migration.
5. The first generated output surface is core MBT binary transport only.
6. Adapter, projection, transponding, and prost DTO output are deferred.

## Recommended Next Phase

Write `docs/specs/mbt_codegen_migration_SPEC.md`.

The spec should be peer-audited separately before any implementation plan.
