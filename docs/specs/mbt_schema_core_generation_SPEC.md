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

# SPEC: MBT Schema Core Generation

## 1. Identification

Slug: `mbt_schema_core_generation`

Repository:

```text
/home/tia/_DEV/MATHILDE/metamorphic-binary-transport
```

Task class: spec authoring.

Research brief:

```text
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_research_brief.md
```

Source experiment schema:

```text
/media/Development/MATHILDE/experiments/crates/schema/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto
```

Target schema crate:

```text
crates/schemas/test_compatibility_core
```

## 2. Status

Status: `DRAFT_AWAITING_PEER_AUDIT_V5`

This spec does not authorize implementation.

Implementation may start only after:

1. this spec passes a separate peer audit;
2. an implementation plan binds exact edits, generated artifacts, tests, and
   validation commands;
3. the implementation plan is explicitly approved.

## 3. Purpose

Prove the new MBT workspace can generate, compile, and validate a real core
schema crate from `.proto + mathilde/options.proto`.

The schema crate must cover every MBT physical field kind currently supported
by core codegen, without pulling in adapter, database, cache, lookup, serving,
or benchmark dependencies.

The intended generation chain is:

```text
MBT-only all-fields proto
  -> metamorphic_binary_transport_codegen --surface core
  -> generated Rust schema module
  -> schema crate compile
  -> deterministic encode/access/inspect tests
```

The partial implementation found that generated Rust was reproducible by
`mbt_codegen --check` but not accepted by workspace `cargo fmt --all --check`
because codegen formats with `rustfmt --edition 2021` while the workspace uses
edition `2024`. This spec now includes the narrow codegen formatting alignment
needed to keep generated files both reproducible and workspace-formatted.

The subsequent validation run proved a second narrow generator issue: archived
`bool` fields are emitted as if they were endian-wrapped archived numeric
fields. In `rkyv`, archived `bool` access is already native `bool`; generated
archived bool checksum and getter code must not call `.to_native()`.

The next validation run proved three generated-code clippy-cleanliness issues:
bitmask masks are emitted with a leading identity `0 |`, payload construction
emits `rows: rows` when shorthand is available, and key-order validation emits
a collapsible nested `if`. These are generated Rust shape issues only; the
approved fixes must not change generated runtime behavior.

## 4. Non-goals

This spec does not:

- generate Bars, Primitives, Regime, Aggregator, MDB, cache, lookup, or MLDB
  schemas;
- import DB/cache/lookup options into `proto/mathilde/options.proto`;
- generate JSON, protobuf, CSV, Arrow, Arrow IPC, or Parquet adapters;
- generate MBT-to-MBT projection implementations;
- generate transponding implementations;
- benchmark runtime throughput;
- claim runtime performance parity with the experiment crate;
- claim compile-time improvement beyond measured build evidence;
- redesign the MBT envelope;
- change trusted-access semantics;
- change core wire bytes;
- add serde, prost runtime, Arrow, Parquet, SQLite, Postgres, or cache
  dependencies.

## 5. Measured object

The measured object is schema-core integration:

```text
crates/schemas/test_compatibility_core/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto
  -> crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
  -> crates/schemas/test_compatibility_core tests
```

Measured by this spec:

- the MBT-only proto imports repository `proto/mathilde/options.proto`;
- `mbt_codegen --inspect` succeeds for the all-fields root message;
- `mbt_codegen --write` produces a generated Rust module;
- `mbt_codegen --check` proves the generated Rust module is reproducible;
- generated Rust formatting uses the workspace Rust edition;
- the schema crate compiles as an independent workspace member;
- the generated schema implements `MbtSchema`;
- checked access validates bytes before archive access;
- trusted access remains a distinct unsafe generated API;
- deterministic encode/access/inspect tests pass for all supported physical
  field kinds.

Not measured by this spec:

- format adapters;
- projection output schemas;
- transponding output;
- storage integration;
- service/API serving behavior;
- large production Primitives compile time.

## 6. Schema source contract

The schema source of truth for this phase is:

```text
crates/schemas/test_compatibility_core/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto
proto/mathilde/options.proto
```

The schema proto is derived from the experiment source:

```text
/media/Development/MATHILDE/experiments/crates/schema/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto
```

The derived proto must preserve:

- package name:

```text
mathilde.binary_transport.test_compatibility.v1
```

- root message:

```text
mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1
```

- schema identity:

```text
schema_id = 40001
schema_version = 1
transport_name = "mathilde.test_compatibility.v1"
payload_root = true
```

- row message:

```text
TestCompatibilityRowV1
```

- repeated payload field:

```text
repeated TestCompatibilityRowV1 rows = 2 [(mathilde.repeated_payload) = true];
```

- field numbers and names from the source compatibility schema;
- MBT dictionary definitions for `tenant`, `entity`, `status`, and `venue`;
- MBT key annotations for `tenant`, `entity`, and `close_ms`;
- MBT presence bits for nullable fields and nullable arrays;
- MBT projection declarations and `projection_group` annotations as parsed
  transport options.

The derived proto must remove every non-MBT option from the experiment source,
including:

```text
mathilde.cache_route
mathilde.cache_table
mathilde.cache_field
mathilde.cache_column
mathilde.table
mathilde.column
```

The fixture must cover these MBT physical kinds:

```text
ConstU16
U16Dictionary
optional U16Dictionary
U64BitmaskDictionary
I64
optional I64
I32
optional I32
U32
optional U32
F64
optional F64
F32
optional F32
Bool
optional Bool
Bytes
optional Bytes
RawString
optional RawString
I64Array
nullable I64Array
I32Array
nullable I32Array
U32Array
nullable U32Array
F64Array
nullable F64Array
F32Array
nullable F32Array
```

Nullable array semantics for this fixture:

- nullable arrays use `repeated` protobuf fields plus `presence_bit`;
- absent/null means the presence bit is not set and the generated owned vector
  is empty;
- present empty array means the presence bit is set and the generated owned
  vector is empty;
- present non-empty array means the presence bit is set and the vector contains
  values.

Unsupported schema shapes remain rejected by codegen. This spec does not relax
existing codegen rejection rules.

## 7. Wire and archive contract

This phase does not change the MBT wire format.

The generated schema must use the existing core envelope contract:

```text
128-byte fixed little-endian transport header
  + rkyv archive payload bytes
```

The generated schema must bind:

- `SCHEMA_ID`;
- `SCHEMA_VERSION`;
- `GENERATED_SCHEMA_HASH`;
- `TRANSPORT_NAME`;
- `SCHEMA_HEADER`.

The generated encoder must:

1. validate owned rows before archive construction;
2. build the rkyv payload archive;
3. compute payload checksum through core `fnv1a64`;
4. write the existing core transport header;
5. enforce `max_response_bytes`.

The generated accessor must:

1. decode the header;
2. validate schema identity and payload length;
3. validate payload checksum on the checked path;
4. access the archive through the generated archive type;
5. validate archived row semantics.

No alternate envelope, schema hash, checksum, encoding kind, or compression
format is allowed in this phase.

## 8. Checked and trusted access contract

The generated schema must expose distinct checked and trusted paths.

Checked path:

```text
TestCompatibilityV1::access(bytes)
```

The checked path must validate the envelope and archived row semantics before
returning a view.

Trusted path:

```text
unsafe TestCompatibilityV1::access_archived_trusted_unchecked(bytes)
```

The trusted path may skip payload checksum and row semantic validation only
under the existing trusted-access contract:

- bytes were previously checked by MBT;
- bytes are immutable after validation;
- schema identity and payload length are still checked by core
  `trusted_payload_for_schema`;
- caller owns the safety obligation.

Tests may call the trusted path only after first producing bytes through the
checked generated encoder or after a checked access has succeeded.

## 9. Codegen contract

The generator command for this schema crate is:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- \
  --write \
  --proto-root crates/schemas/test_compatibility_core/proto \
  --proto-root proto \
  --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto \
  --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 \
  --module test_compatibility_v1 \
  --surface core \
  --out crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

The reproducibility check command is:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- \
  --check \
  --proto-root crates/schemas/test_compatibility_core/proto \
  --proto-root proto \
  --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto \
  --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 \
  --module test_compatibility_v1 \
  --surface core \
  --out crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

The inspect command is:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- \
  --inspect \
  --proto-root crates/schemas/test_compatibility_core/proto \
  --proto-root proto \
  --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto \
  --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 \
  --module test_compatibility_v1 \
  --surface core
```

Generated output requirements:

- file starts with the deterministic generated header;
- file imports `metamorphic_binary_transport_core`;
- file imports `rkyv`;
- file implements `MbtSchema`;
- file emits no adapter, projection, transponding, MDB, cache, lookup, or MLDB
  code;
- file does not require `serde`;
- file does not require `prost`;
- file does not require `PartialEq` for row equality tests.

If current codegen cannot generate this fixture without manual generated-file
edits, implementation must stop and amend the codegen spec before continuing.

Generated formatting contract:

- `crates/codegen/src/emit.rs` must format generated Rust with
  `rustfmt --edition 2024`;
- this formatter edit must not change descriptor loading, schema modeling,
  emitted semantics, emitted APIs, CLI arguments, dependency behavior, or
  output surfaces;
- after the edit, generated output must still pass `mbt_codegen --check` and
  workspace `cargo fmt --all --check`.

Archived bool emission contract:

- for archived `Bool` emission, `crates/codegen/src/rust_emit.rs` may be
  changed only in the two bound emission sites below;
- `emit_archived_checksum_line` must emit archived bool checksum code as:

```rust
checksum = update_bool(checksum, row.required_bool);
```

using the generated field access expression directly, without `.to_native()`;

- `archived_value_access` must emit archived bool getter access as the generated
  field access expression directly, without `.to_native()`;
- all numeric, dictionary, bitmask, and floating archived scalar fields must
  keep `.to_native()`;
- bytes, raw strings, and arrays must keep their existing access paths;
- the edit must not change descriptor loading, schema modeling, emitted
  semantics, emitted APIs, CLI arguments, dependency behavior, or output
  surfaces;
- the edit must not change the MBT wire format, archive layout, schema hash,
  checksum algorithm, validation rules, or trusted-access contract.

Generated clippy-cleanliness contract:

- `emit_bitmask_dictionary` may be changed only to emit `VALID_*_MASK` without
  a leading identity `0 |` term;
- generated valid mask semantics must remain the bitwise OR of all declared
  dictionary bits;
- `emit_order_validation` may be changed only to emit the same key-regression
  condition in clippy-clean Rust without changing comparison operands,
  ordering, or error variant;
- generated key-order failure must still return
  `TransportError::InvalidTimeGrid("key order regression".to_string())`;
- generated payload construction may be changed only to use field-init shorthand
  for the repeated payload field when the local variable name is identical to
  the field name;
- payload type, schema version field, row field name, row vector ownership, and
  validation order must not change.

## 10. Crate boundary contract

This spec creates the first approved workspace schema crate under the reserved
path:

```text
crates/schemas/test_compatibility_core
```

Package name:

```text
metamorphic_binary_transport_schema_test_compatibility
```

Allowed dependency direction:

```text
schema crate -> metamorphic_binary_transport_core
schema crate -> rkyv
```

Forbidden dependency direction:

```text
metamorphic_binary_transport_core -> schema crate
metamorphic_binary_transport_codegen -> schema crate
adapter crate -> schema crate
schema crate -> codegen
schema crate -> adapters
schema crate -> benches
```

The root workspace manifest may add exactly this member:

```text
crates/schemas/test_compatibility_core
```

No other schema crate may be created by this phase.

## 11. Dependency contract

Allowed new schema crate dependencies:

```toml
[dependencies]
metamorphic_binary_transport_core = { path = "../../core" }
rkyv = "=0.8.16"
```

No other runtime dependency is allowed for the schema crate.

The schema crate must not depend on:

```text
metamorphic_binary_transport_codegen
serde
serde_json
prost
prost-reflect
arrow
parquet
zstd
rusqlite
sqlx
heed
```

The implementation plan must record dependency-tree evidence with:

```text
cargo tree -p metamorphic_binary_transport_schema_test_compatibility
cargo tree -p metamorphic_binary_transport_core
```

`Cargo.lock` may change only as a Cargo-generated dependency artifact required
by adding the schema crate and `rkyv`.

## 12. Determinism contract

Determinism requirements:

- repeated `mbt_codegen --write` produces identical bytes when the proto input
  is unchanged;
- `mbt_codegen --check` passes after `--write`;
- encoding the same deterministic row vector twice produces byte-identical MBT
  output;
- `inspect` returns the same row count and checksums across repeated calls on
  identical bytes;
- row iteration order equals input order;
- key-order validation is deterministic;
- tests must not depend on hash map iteration order, filesystem glob order, or
  randomized input.

## 13. Failure contract

Tests must prove explicit failures for:

- invalid schema version field on owned row;
- duplicate or out-of-range dictionary ordinal if a row can be constructed with
  such a value;
- invalid bitmask dictionary mask;
- nonfinite `f32`;
- nonfinite `f64`;
- absent optional field with non-default backing value;
- nullable array absent with non-empty backing vector;
- `max_response_bytes` too small;
- corrupt magic;
- truncated envelope;
- wrong schema ID through header mutation, asserting
  `TransportError::UnknownSchemaId`;
- wrong schema hash through header mutation, asserting
  `TransportError::SchemaHashMismatch`.

Failure assertions must match concrete `TransportError` variants. Tests must
not collapse all failures into generic `is_err` checks unless a specific error
variant is not externally observable.

## 14. Compile-surface budget

This phase must record compile-surface evidence. It does not claim compile-time
improvement.

Required commands:

```text
wc -l crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
cargo check -p metamorphic_binary_transport_codegen
cargo test -p metamorphic_binary_transport_codegen
cargo clippy -p metamorphic_binary_transport_codegen --all-targets -- -D warnings
cargo check -p metamorphic_binary_transport_schema_test_compatibility
/usr/bin/time -v cargo check -p metamorphic_binary_transport_schema_test_compatibility
cargo clippy -p metamorphic_binary_transport_schema_test_compatibility --all-targets -- -D warnings
cargo tree -p metamorphic_binary_transport_schema_test_compatibility
```

If `/usr/bin/time` is unavailable, implementation must record that fact in the
result review and still run plain `cargo check`.

Compile-surface stop gates:

- generated schema crate pulls adapter dependencies;
- generated schema crate pulls codegen as a dependency;
- generated row structs require `Debug`, `PartialEq`, or
  `rkyv::Deserialize`;
- `cargo check` fails;
- clippy warnings are introduced in the schema crate.

## 15. Runtime performance budget

This phase has no runtime throughput claim.

Runtime budget for tests only:

- correctness tests must use deterministic in-memory row fixtures;
- tests must not perform network or database I/O;
- tests must not benchmark adapters or storage;
- no latency, rows/sec, or MB/sec claim may be made from this phase.

Runtime performance benchmarking for this schema requires a later benchmark
spec.

## 16. Correctness oracle

The correctness oracle is field-for-field agreement between deterministic owned
rows and checked archived row views.

The fixture rows must include:

1. one row with every optional scalar present and every nullable array present
   and non-empty;
2. one row with every optional scalar absent and every nullable array absent;
3. one row with nullable arrays present but empty;
4. at least two dictionary/key combinations in deterministic key order.

For each checked view row, tests must compare:

- schema version;
- dictionary ordinals and resolved dictionary strings where helpers exist;
- bitmask values;
- signed and unsigned integers;
- floats, using exact equality for selected finite fixture values;
- booleans;
- raw strings;
- bytes;
- arrays by collected values;
- presence bits or presence words;
- row count;
- semantic checksum;
- minimal projection checksum.

The same bytes must also be inspected through the generated `inspect` API.

## 17. Benchmark methodology

No throughput benchmark is authorized by this spec.

The only measurement-like artifacts are build and size observations:

- generated line count;
- `cargo check` result;
- optional `/usr/bin/time -v cargo check` output;
- dependency tree.

If a later spec adds runtime benchmarking, it must define:

- dataset size;
- row count;
- payload shape;
- build profile;
- baseline;
- warm/cold behavior if storage is involved;
- exact output artifact path.

## 18. Test plan

Required checks:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --inspect ...
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write ...
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check ...
cargo fmt --all --check
cargo check -p metamorphic_binary_transport_codegen
cargo test -p metamorphic_binary_transport_codegen
cargo clippy -p metamorphic_binary_transport_codegen --all-targets -- -D warnings
cargo check -p metamorphic_binary_transport_schema_test_compatibility
cargo test -p metamorphic_binary_transport_schema_test_compatibility
cargo clippy -p metamorphic_binary_transport_schema_test_compatibility --all-targets -- -D warnings
cargo check --workspace
```

Required test files:

```text
crates/schemas/test_compatibility_core/tests/test_roundtrip.rs
crates/schemas/test_compatibility_core/tests/test_failures.rs
crates/schemas/test_compatibility_core/tests/test_determinism.rs
```

Tests must not use `unwrap`, `expect`, or `panic!` in reusable helpers. Test
functions may return `Result<(), Box<dyn std::error::Error>>`.

## 19. Code bindings

Files allowed to be created:

```text
crates/schemas/test_compatibility_core/Cargo.toml
crates/schemas/test_compatibility_core/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto
crates/schemas/test_compatibility_core/src/lib.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
crates/schemas/test_compatibility_core/tests/test_roundtrip.rs
crates/schemas/test_compatibility_core/tests/test_failures.rs
crates/schemas/test_compatibility_core/tests/test_determinism.rs
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_implementation_plan.md
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_result_review.md
```

Files allowed to be edited:

```text
Cargo.toml
Cargo.lock
crates/codegen/src/emit.rs
crates/codegen/src/rust_emit.rs
```

The only allowed edit in `crates/codegen/src/emit.rs` is:

```text
rustfmt --edition 2021
  -> rustfmt --edition 2024
```

The only allowed edits in `crates/codegen/src/rust_emit.rs` are:

```text
emit_archived_checksum_line:
  FieldKind::Bool emits update_bool(checksum, {access})
  not update_bool(checksum, {access}.to_native())

archived_value_access:
  FieldKind::Bool emits {access}
  not {access}.to_native()

emit_bitmask_dictionary:
  VALID_*_MASK emits the OR of dictionary bit constants without leading `0 |`

emit_order_validation:
  key-order validation emits a clippy-clean single condition without changing
  the tuple comparison or InvalidTimeGrid error

encode_owned payload construction:
  use `{rows}` field-init shorthand only when the payload repeated field name
  is exactly `rows`; otherwise keep explicit `{field_name}: rows`
```

No other production source file in these crates may be edited by this phase
unless a peer-audited amendment proves codegen cannot generate the approved
fixture:

```text
crates/core
crates/projection
crates/metamorphose
crates/transponding
crates/adapters
crates/benches
```

If any codegen fix beyond rustfmt edition alignment, the archived bool
generator fix, and the generated clippy-cleanliness fixes above is required,
work must stop and this spec plus the implementation plan must be amended
before touching more codegen code.

## 20. Generated artifact bindings

Generated source:

```text
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

Generated source owner:

```text
metamorphic_binary_transport_codegen --surface core
```

Generated source must not be manually edited.

Generated source reproducibility command:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- \
  --check \
  --proto-root crates/schemas/test_compatibility_core/proto \
  --proto-root proto \
  --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto \
  --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 \
  --module test_compatibility_v1 \
  --surface core \
  --out crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

Temporary codegen artifacts may be written only under:

```text
target/mbt-codegen-check/
target/mbt-schema-core-generation/
```

No generated adapter, projection, transponding, DB, cache, lookup, or serving
artifact is allowed by this spec.

## 21. Review artifact bindings

Required review artifacts:

```text
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_research_brief.md
docs/specs/mbt_schema_core_generation_SPEC.md
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_peer_audit.md
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_implementation_plan.md
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_result_review.md
```

If peer audit blocks the spec, amendments must use versioned audit files:

```text
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_peer_audit_v2.md
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_peer_audit_v3.md
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_peer_audit_v4.md
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_peer_audit_v5.md
```

## 22. Implementation plan requirement

The implementation plan must bind:

- exact proto derivation from the experiment source;
- exact workspace manifest change;
- exact schema crate manifest;
- exact generated source path;
- exact codegen commands;
- exact tests;
- exact dependency-tree commands;
- exact compile-surface commands;
- expected generated line-count artifact;
- result review output path;
- rollback boundary.

The plan must explicitly state that no generated Rust line may be hand-edited.

## 23. Approval checklist

This spec is implementation-ready only when all are true:

- required protocol reads are complete;
- the measured object is schema-core generation only;
- the MBT-only fixture contract is explicit;
- DB/cache/lookup options are excluded;
- codegen command is exact;
- generated artifact path is exact;
- schema crate dependency boundary is exact;
- correctness oracle is field-for-field;
- failure contract is explicit;
- compile-surface budget is explicit;
- peer audit passes;
- implementation plan is written and approved.

Current status:

```text
NOT IMPLEMENTATION_READY
```

Reason:

```text
peer audit and implementation plan are still required
```

## 24. Open questions

No blocking open question is known at spec-authoring time.

Peer audit must specifically challenge:

- whether the derived MBT-only proto preserves the intended physical field
  surface from the experiment source;
- whether committing the generated schema module is acceptable for a schema
  crate proof;
- whether the dependency boundary is narrow enough;
- whether the tests prove nullable array semantics without adding derive
  pressure to generated rows.
