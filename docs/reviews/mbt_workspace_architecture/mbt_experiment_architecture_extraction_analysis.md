# MBT Experiment Architecture Extraction Analysis

Status: complete for workspace architecture spec draft

Slug: `mbt_workspace_architecture`

Source experiment crate:

```text
/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport
```

Target repository:

```text
/home/tia/_DEV/MATHILDE/metamorphic-binary-transport
```

## Purpose

Extract the architecture that must be preserved from the experiment MBT crate
before any production workspace split is implemented.

This document is evidence for the workspace architecture spec. It is not an
implementation plan and does not authorize code changes.

## Extraction Rule

The new repository must preserve the proven MBT architecture and split compile
surfaces. It must not redesign runtime semantics while performing the split.

Preserved architecture means:

- `.proto + MBT options` remains the source of truth;
- envelope/header semantics remain centralized;
- schema-specific Rust is generated, not handwritten;
- checked access and trusted access stay distinct;
- trusted access remains an explicit safety contract for immutable validated
  bytes;
- projection remains MBT-to-MBT before boundary conversion;
- metamorphose remains boundary conversion from MBT to requested formats;
- transponding remains the columnar transformation mechanism for columnar
  adapters;
- benchmarks and tests remain outside production runtime crates.

## Batch 1: Core Runtime, Envelope, Checked Access, Trusted Access

### Files Read

Code-read evidence:

```text
crates/mathilde-binary-transport/src/runtime.rs
crates/mathilde-binary-transport/src/envelope.rs
crates/mathilde-binary-transport/src/error.rs
crates/mathilde-binary-transport/src/codec.rs
crates/mathilde-binary-transport/src/lib.rs
crates/mathilde-binary-transport/src/generated/bars_v1.rs
```

### Pass 1: Runtime Topology

Code-read evidence from `src/runtime.rs`:

- `MbtSchema` is a schema-marker trait.
- The trait exposes `Row`, `View`, `encode_rows`, `encode_owned_rows`,
  `access_view`, and `inspect_bytes`.
- Free functions `encode`, `encode_owned`, `access`, and `inspect` dispatch to
  the schema marker.
- `BinaryInspection` contains row count plus semantic and minimal projection
  checksums.

Preservation requirement:

- The production core crate must keep a small runtime trait surface with schema
  marker dispatch.
- The runtime core must not require protobuf DTOs, JSON, Arrow, Parquet, CSV,
  or benchmark modules to compile this trait surface.

### Pass 2: Envelope and Header Contract

Code-read evidence from `src/envelope.rs`:

- Envelope magic is currently `MATBT001`.
- Header length is fixed at 128 bytes.
- The header stores transport version, encoding kind, flags, schema id, schema
  version, logical proto schema hash, build id, row count, payload length, and
  payload checksum.
- `TransportHeader::new_with_schema` binds a `SchemaHeaderSpec` to row count,
  payload length, and payload checksum.
- `encode_header` writes a fixed little-endian header.
- `decode_header` rejects truncated payloads, wrong magic, and non-zero
  reserved header bytes.
- `validate_header_for_schema` checks transport version, header length,
  encoding kind, flags, schema id, schema version, schema hash, payload length,
  and payload checksum.
- `trusted_payload_for_schema` validates envelope identity and payload length
  but does not recompute payload checksum.

Preservation requirement:

- Core owns the envelope/header and validation contract.
- Checked access must keep payload checksum validation.
- Trusted access must keep the explicit reduced validation contract and must be
  restricted to bytes already validated before immutable storage or handoff.
- Adapter crates and schema crates must not invent their own envelope
  semantics.

### Pass 3: Generated Schema Access Shape

Code-read evidence from `src/generated/bars_v1.rs`:

- `BarsV1` is a generated schema marker.
- The marker exposes schema id, schema version, schema hash, and transport
  name constants.
- `BarsV1::encode` delegates to `encode_owned`.
- `encode_owned` validates rows, archives a `MathildeTransportPayloadV1` with
  rkyv, checks response cap, writes a core envelope header, and returns the
  header plus payload bytes.
- `BarsV1::access` returns `BarsV1View` after checked archive access.
- `access_archived` decodes and validates the envelope, performs checked rkyv
  archive access, and validates archived payload row count.
- `access_archived_trusted_unchecked` calls `trusted_payload_for_schema` and
  then `rkyv::access_unchecked`.
- The trusted method documents the safety contract: the caller must guarantee
  bytes came from immutable trusted cache values previously accepted by the
  checked `encode` and `access` path.
- `BarsV1` implements `MbtSchema`.
- `BarsV1View` exposes row iteration without materializing owned rows.
- `ArchivedBarsV1Row` exposes typed field accessors over archived rows.

Preservation requirement:

- Generated schema crates must expose the same conceptual marker/view/archive
  accessor shape.
- Checked archive access and trusted unchecked archive access must remain
  separate.
- The production split must not move adapter methods into core schema output
  unless an adapter crate explicitly requests them.

### Observed Compile-Surface Problem

Code-read evidence from `src/lib.rs`:

- The experiment crate currently exposes or compiles `arrow`, `arrow_ipc`,
  `metamorphose`, `parquet`, `transponding`, `benches`, `generated`, and core
  modules from one library crate.

This is a compile-surface problem, not a reason to redesign the runtime
architecture.

Preservation requirement:

- The new workspace must move adapter and benchmark surfaces outside the core
  crate while preserving the core runtime and generated access semantics.

## Batch 1 Spec Amendments Required

The workspace architecture spec must state explicitly:

1. the work is an extraction and crate-surface split, not a runtime redesign;
2. the core crate owns the envelope, errors, checksum helpers, runtime trait,
   checked validation, and trusted payload validation;
3. generated schema crates own schema markers, row/archive/view accessors, and
   schema-specific checked/trusted wrappers;
4. adapter methods must not be generated into the core schema surface by
   default;
5. the current monolithic `lib.rs` shape is evidence of what must be split,
   not a contract to preserve as a single crate.

## Batch 2: Codegen, Descriptor Model, Generated Output Surface

### Files Read

Code-read evidence:

```text
crates/mathilde-binary-transport/src/codegen/model.rs
crates/mathilde-binary-transport/src/codegen/descriptor.rs
crates/mathilde-binary-transport/src/codegen/emit.rs
crates/mathilde-binary-transport/src/codegen/rust_emit.rs
crates/mathilde-binary-transport/src/codegen/error.rs
crates/mathilde-binary-transport/src/generated/mod.rs
crates/schema/proto/mathilde/options.proto
crates/schema/proto/mathilde/binary_transport/v1/bars.proto
```

### Pass 1: Descriptor and Model Ownership

Code-read evidence from `src/codegen/model.rs` and
`src/codegen/descriptor.rs`:

- `CodegenRequest` binds proto path, root message, and Rust module name.
- `canonical_requests()` currently lists Bars, Primitives, test compatibility,
  and wide presence as one canonical set.
- `load_schema_model` loads a descriptor set through `prost_reflect`, finds
  the configured root message, reads MBT options, validates dictionaries, and
  builds a `SchemaModel`.
- `SchemaModel` joins module name, proto/root metadata, payload type, row type,
  schema identity, dictionaries, physical fields, target fields, key parts,
  schema hash, and projections.
- `PhysicalField` describes the rkyv archive shape.
- `TargetField` describes boundary writer shape and may include target-only
  fields such as derived UTC values.
- `FieldKind` currently covers MBT physical primitives including dictionary
  ordinals, bitmask dictionaries, signed/unsigned integers, floats, bool,
  bytes, raw string, and numeric arrays.
- `TargetFieldKind` adds `DerivedUtc` for target-only boundary output.
- Projection definitions are read from root message options and produce
  generated projection schema models.

Preservation requirement:

- Codegen must remain descriptor-driven.
- Schema-specific behavior must come from proto/options, not handwritten Rust
  branches.
- The new codegen crate may split output targets, but it must not reduce
  schema-model validation quality.

### Pass 2: Proto Options and Schema Source

Code-read evidence from `crates/schema/proto/mathilde/options.proto`:

- The current shared options file contains MBT transport options, DB options,
  lookup options, MLDB options, and cache options.
- MBT transport options include schema id, schema version, transport name,
  payload root, projections, dictionary, bitmask dictionary, presence bit, key
  parts, const value, repeated payload, raw string, ignored fields, projection
  group, and derived UTC source.

Code-read evidence from
`crates/schema/proto/mathilde/binary_transport/v1/bars.proto`:

- Bars schema imports `mathilde/options.proto`.
- The payload root declares schema identity, transport name, payload root,
  projections, and downstream DB/cache options.
- Row fields use MBT options for dictionaries, key parts, physical storage,
  ignored target-only UTC fields, and projection groups.

Preservation requirement:

- MetaBT must own the MBT transport option contract required for MBT codegen.
- Application schemas may import those options from outside the MBT repo.
- Downstream DB/cache/lookup options must not enter MBT core by accident.
- If MetaBT keeps non-transport options in a shared options file later, that
  decision requires its own spec because it changes ownership boundaries.

### Pass 3: Emission Surface and Compile Coupling

Code-read evidence from `src/codegen/rust_emit.rs`:

- `generated_schema` currently emits imports, schema constants, dictionaries,
  presence constants, structs, validation, checksums, dictionary helpers, and
  runtime API.
- `emit_runtime_api` emits schema marker, encode, checked access, trusted
  access, inspect, projection methods, metamorphose methods, runtime trait
  implementation, metamorphose trait implementations, view types, decode
  helpers, transponding helpers, and metamorphose helpers in one generated
  module.
- `emit_projection_methods` correctly preserves the architectural rule that
  projection produces MBT bytes first.
- The same projection emission also emits projection-to-JSON and
  projection-to-protobuf convenience methods in the same schema module.
- `emit_transponding_helpers` emits column batch helpers plus Arrow, Arrow IPC,
  and Parquet marker methods in the same schema module.
- `emit_metamorphose_methods` emits MBT, JSON, protobuf, and CSV boundary
  methods in the same schema module.
- `emit_metamorphose_trait` implements JSON, protobuf, CSV, Arrow, Arrow IPC,
  and Parquet adapter traits for the marker from the same generated module.

Code-read evidence from `src/generated/mod.rs`:

- The generated module list currently imports Bars, Primitives, test
  compatibility, wide presence, and their prost DTO modules from one generated
  module tree.

Line-count evidence from `wc -l`:

```text
crates/mathilde-binary-transport/src/generated/bars_v1.rs              7088
crates/mathilde-binary-transport/src/generated/primitives_v1.rs        34469
crates/mathilde-binary-transport/src/generated/test_compatibility_v1.rs 8029
crates/mathilde-binary-transport/src/generated/wide_presence_v1.rs     26995
generated files total                                                  78064
codegen files total                                                     6184
```

This proves a structural compile-surface issue: broad generated schema modules
include runtime, projection, all boundary adapters, transponding, and generated
protobuf DTO modules together.

Preservation requirement:

- The generated core schema output must preserve marker/view/access/validation
  semantics.
- Adapter output must be generated only for selected adapters.
- Prost DTO output must not be generated into the core schema crate unless a
  protobuf adapter spec requests it.
- Schema crates must not compile unrelated schemas.
- Codegen must support explicit output modes instead of one all-surfaces
  generated module.

## Batch 2 Spec Amendments Required

The workspace architecture spec must add:

1. a codegen output surface split;
2. a rule that core schema output includes only transport-core semantics;
3. a rule that JSON/protobuf/CSV/Arrow/Arrow IPC/Parquet output is adapter
   output;
4. a rule that generated prost DTO modules are not core schema output;
5. a rule that canonical multi-schema requests belong to benchmark/check
   surfaces, not production schema crates;
6. a rule that MBT transport options are owned by MetaBT, while downstream
   DB/cache/lookup options require a separate ownership spec.

## Batch 3: Adapters, Transponding, Tests, Benches, Evidence

### Files Read

Code-read evidence:

```text
crates/mathilde-binary-transport/src/metamorphose/mod.rs
crates/mathilde-binary-transport/src/metamorphose/runtime.rs
crates/mathilde-binary-transport/src/metamorphose/json.rs
crates/mathilde-binary-transport/src/metamorphose/protobuf.rs
crates/mathilde-binary-transport/src/metamorphose/csv.rs
crates/mathilde-binary-transport/src/transponding.rs
crates/mathilde-binary-transport/src/arrow.rs
crates/mathilde-binary-transport/src/arrow_ipc.rs
crates/mathilde-binary-transport/src/parquet.rs
crates/mathilde-binary-transport/src/benches/mod.rs
crates/mathilde-binary-transport/Cargo.toml
crates/mathilde-binary-transport/docs/architecture.md
crates/mathilde-binary-transport/docs/bench_results.md
crates/mathilde-binary-transport/docs/test_results.md
```

### Pass 1: Metamorphose Boundary API

Code-read evidence from `src/metamorphose/mod.rs`:

- `MetamorphoseFormat` selects MBT, JSON, protobuf, or CSV.
- `MetamorphoseOutput` keeps MBT output borrowed and boundary formats owned.
- `MetamorphoseSchema` exposes MBT, JSON, protobuf, and CSV conversion.
- Separate traits exist for Arrow, Arrow IPC, Parquet, and CSV.
- Public helper functions call the generated schema trait methods.

Code-read evidence from `src/metamorphose/runtime.rs`,
`json.rs`, `protobuf.rs`, and `csv.rs`:

- `CheckedBytes` enforces response caps.
- JSON and CSV writers stream into capped byte buffers without serde DTOs.
- Protobuf writer emits protobuf wire bytes directly without constructing
  prost DTOs.
- UTC formatting writes directly to byte buffers.
- Bytes are base64 encoded only at text boundaries.

Preservation requirement:

- Metamorphose remains the boundary conversion concept.
- MBT output may remain borrowed.
- JSON/protobuf/CSV output are declared allocation points because they produce
  owned response bytes.
- JSON/protobuf/CSV writer helpers must live in adapter crates or an
  adapter-owned helper crate, not in MBT core.

### Pass 2: Transponding and Columnar Adapters

Code-read evidence from `src/transponding.rs`:

- Transponding owns row-to-columnar intermediate builders.
- Optional columns use validity bitmaps.
- List columns use offsets, values, and optional row validity to preserve
  NULL versus empty.

Code-read evidence from `src/arrow.rs`:

- Arrow adapter consumes generated transponded columns.
- Arrow schema metadata carries MBT transport name, schema id, schema version,
  and schema hash.
- Arrow field metadata carries physical name and dictionary metadata.

Code-read evidence from `src/arrow_ipc.rs` and `src/parquet.rs`:

- Arrow IPC and Parquet are boundary encodings over Arrow `RecordBatch`.
- Both use checked writers to enforce response caps.
- Parquet writes uncompressed output; compression remains outside the MBT
  envelope.

Preservation requirement:

- Transponding stays as the columnar transformation mechanism.
- Arrow, Arrow IPC, and Parquet must be opt-in adapter surfaces.
- Arrow and Parquet dependencies must not enter core.
- Columnar metadata behavior must be preserved when the adapter crates are
  extracted.

### Pass 3: Bench/Test/Evidence and Dependency Shape

Code-read evidence from `Cargo.toml`:

- The experiment crate has unconditional dependencies on Arrow, Arrow IPC,
  Parquet, prost, serde, serde_json, thiserror, and zstd.
- Codegen dependencies `prost-build` and `prost-reflect` are feature-gated, but
  generated runtime, adapters, benches, and schemas are still in one package.

Code-read evidence from `src/benches/mod.rs`:

- Benchmark modules include main benchmark, baseline, compatibility,
  compression, and fixtures.

Code-read evidence from `docs/bench_results.md`:

- Tracked benchmark evidence exists for MBT, JSON, protobuf, CSV, Arrow IPC,
  Parquet, projections, and compression.
- Compression is explicitly measured outside the MBT envelope.

Code-read evidence from `docs/test_results.md`:

- The experiment recorded tests for codegen, metamorphose, Arrow IPC,
  compression, shared schema relocation, and production cleanup.
- The documented source audits include checks that benchmark modules are no
  longer public library API and that generated code is synchronized with
  codegen.

Preservation requirement:

- Benchmark and compatibility fixtures must move to evidence/bench crates.
- Compression benchmark code must remain outside core and outside the transport
  envelope.
- The new repo must preserve benchmark methodology, but not compile benchmark
  modules into production crates.
- Adapter dependencies must be owned by their adapter crates.

## Batch 3 Spec Amendments Required

The workspace architecture spec must add:

1. JSON, protobuf, and CSV writer helpers are adapter-owned;
2. Arrow, Arrow IPC, Parquet, and transponding output are adapter-owned;
3. transponding as a concept is preserved but does not force Arrow/Parquet into
   core;
4. compression remains external to MBT runtime and belongs to benchmark or
   downstream transport configuration;
5. benchmark fixtures and compatibility baselines are not production runtime;
6. production core dependency checks must reject Arrow, Arrow IPC, Parquet,
   prost DTO, serde_json, zstd, and benchmark dependencies unless a later spec
   explicitly moves one into core.

## Extraction Analysis Conclusion

The current experiment crate proves the architecture to preserve:

```text
.proto + options
  -> descriptor model
  -> generated schema marker and rkyv archive access
  -> checked / trusted split
  -> MBT-to-MBT projection
  -> metamorphose boundary writers
  -> transponding for columnar adapters
  -> benchmark/test evidence
```

The production issue is compile-surface ownership, not the runtime model.

The MetaBT workspace spec must therefore split crates and generated output
surfaces while preserving the runtime, projection, metamorphose, and
transponding semantics.
