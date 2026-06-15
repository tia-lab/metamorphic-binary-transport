# MBT Projection Direct Writer SPEC

## 1. Identification

Slug: `mbt_projection_direct_writer`
Status: draft amended after peer audit
Task class: corrective spec authoring

Research brief:

```text
docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_research_brief.md
```

Blocking peer audit addressed by this amendment:

```text
docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_peer_audit.md
```

This spec does not authorize implementation.

## 2. Status

This spec is ready for a second peer audit only.

Code changes remain forbidden until:

1. this spec passes peer audit v2;
2. an implementation plan is written and audited if required;
3. the implementation plan is explicitly approved.

## 3. Purpose

Replace generated MBT-to-MBT projection that materializes owned projected rows
with generated schema-specific direct projection writing.

The intended flow is:

```text
source MBT bytes
  -> checked or trusted source archived payload
  -> generated schema-specific projected archive writer
  -> projected MBT bytes
```

The behavioral oracle is the old `mathilde-binary-transport` projection API and
benchmark shape. The implementation target is stricter than the old version:
the old version was schema-specific but still built `Vec<ProjectedRow>` before
calling `ProjectedSchema::encode_owned`.

## 4. Non-goals

This spec does not:

- add JSON, protobuf, CSV, Arrow, Arrow IPC, Parquet, or other boundary-format
  projection;
- change the MBT envelope format;
- change rkyv as the archive format;
- add a schema-independent projection runtime engine;
- add handwritten schema-specific branches to codegen;
- hand-edit generated schema artifacts;
- keep a production slow fallback projection path;
- claim runtime or compile-time improvement before evidence.

## 5. Measured Object

The measured object is the generated archived projection helper plus the public
projection methods that call it.

Checked path:

```text
S::project_p(bytes, max_response_bytes)
  -> S::access_archived(bytes)
  -> project_p_archived_direct(archived, max_response_bytes)
  -> projected MBT bytes
```

Trusted path:

```text
S::project_p_trusted_unchecked(bytes, max_response_bytes)
  -> S::access_archived_trusted_unchecked(bytes)
  -> project_p_archived_direct(archived, max_response_bytes)
  -> projected MBT bytes
```

Archived benchmark path:

```text
already accessed source archive
  -> project_p_archived_direct(archived, max_response_bytes)
  -> projected MBT bytes
```

The archived benchmark lane must isolate projection work after source archive
access, matching the old benchmark lane shape.

## 6. Schema Source Contract

The source of truth remains:

```text
.proto files
proto/mathilde/options.proto
```

Projection definitions are proto-derived:

```proto
option (mathilde.projection) = {
  name: "..."
  rust_marker: "..."
  include_group: "..."
  exclude_group: "..."
  include_field: "..."
  exclude_field: "..."
};
```

Projection selection rules:

- the `const_u16` schema-version field is always retained;
- every key field is always retained;
- included groups add fields;
- excluded groups remove fields after mandatory fields are retained;
- included fields add fields;
- excluded fields remove fields after mandatory fields are retained;
- codegen must fail if a projection removes any key field;
- codegen must fail if a projection refers to an unknown field or group.

The Bars benchmark schema must be added at:

```text
crates/schemas/bars_core/proto/mathilde/binary_transport/v1/bars.proto
```

It must be derived from:

```text
/media/Development/MATHILDE/experiments/crates/schema/proto/mathilde/binary_transport/v1/bars.proto
```

Because this repository owns only MBT options, the Bars benchmark proto must
remove only options not defined in `proto/mathilde/options.proto`, including
cache and DB annotations. It must preserve all MBT-owned semantics:

- package and message names;
- field names and field numbers;
- dictionary values and aliases;
- schema id;
- schema version;
- transport name;
- payload root;
- repeated payload field;
- `const_u16`;
- dictionary and bitmask dictionary annotations;
- key fields and key order;
- ignored UTC fields and `derived_utc_from`;
- nested metadata message shape;
- metadata projection group;
- presence-bit numbers;
- `no_metadata` projection;
- `ohlcv_only` projection.

The Bars physical-shape oracle must prove that the stripped proto did not
change MBT physical semantics. The implementation plan must create this exact
fixture:

```text
crates/schemas/bars_core/tests/expected_bars_projection_inspect.txt
```

The fixture is an expected `mbt_codegen --inspect --surface projection` output
for the Bars benchmark schema. It must be reviewed against the old Bars proto
and old generated Bars field shape before code is accepted.

## 7. Wire and Archive Contract

Projection output is a complete projected MBT payload:

```text
128-byte MBT envelope
projected rkyv payload bytes
```

The projected envelope must use the projected schema header:

- projected schema id;
- projected schema version;
- projected schema hash;
- projected row count;
- projected payload byte length;
- projected payload checksum.

The source and projected schemas must be distinct when projection changes the
physical field list.

Required validation behavior:

- projected output must validate with projected schema checked access;
- projected output must fail under source schema checked access;
- source output must fail under projected schema checked access;
- checked and trusted projection output for the same immutable input must be
  byte-for-byte equal.

Response-cap semantics match current generated `encode_owned`:

```text
total_len = HEADER_LEN + projected_payload_len
```

If `total_len > max_response_bytes`, projection must return
`TransportError::ResponseTooLarge { observed: total_len, cap: max_response_bytes }`.
If `HEADER_LEN + projected_payload_len` overflows `usize`, projection must return
`TransportError::ResponseTooLarge { observed: usize::MAX, cap: max_response_bytes }`.

## 8. Checked and Trusted Access Contract

Generated projection methods must remain distinct:

```rust
impl SourceMarker {
    pub fn project_<projection>(
        bytes: &[u8],
        max_response_bytes: usize,
    ) -> Result<Vec<u8>>;

    pub unsafe fn project_<projection>_trusted_unchecked(
        bytes: &[u8],
        max_response_bytes: usize,
    ) -> Result<Vec<u8>>;
}
```

Checked projection:

- calls the existing checked source archived accessor;
- validates source header identity, source payload length, source payload
  checksum, and source archive shape before projection.

Trusted projection:

- calls the existing trusted source archived accessor;
- may skip source payload checksum validation only under the existing trusted
  access safety contract;
- still validates source header identity and source payload length through the
  existing trusted payload path.

The generated unsafe method must keep a Rust safety doc comment stating that
the caller guarantees the bytes were previously accepted by checked MBT access
for the source schema and then stored or transported without mutation.

## 9. Codegen Contract

The codegen source files allowed for implementation planning are:

```text
crates/codegen/src/model.rs
crates/codegen/src/descriptor.rs
crates/codegen/src/rust_emit.rs
crates/codegen/src/tests/mod.rs
crates/codegen/src/tests/test_cli.rs
crates/codegen/src/tests/test_descriptor.rs
crates/codegen/src/tests/test_rust_emit_core.rs
crates/codegen/src/tests/test_rust_emit_projection_direct_writer.rs
```

`Cargo.toml` may be edited only to add `crates/schemas/bars_core` to the
workspace and to wire benchmark crate dependencies already approved in this
spec.

Codegen inputs for the compatibility schema:

```text
crates/schemas/test_compatibility_core/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto
proto/mathilde/options.proto
root = mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1
module = test_compatibility_v1
surface = projection
```

Codegen inputs for the Bars benchmark schema:

```text
crates/schemas/bars_core/proto/mathilde/binary_transport/v1/bars.proto
proto/mathilde/options.proto
root = mathilde.binary_transport.v1.MathildeTransportResponseV1
module = bars_v1
surface = projection
```

Generated output may only be regenerated by these write commands:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface projection --out crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface projection --out crates/schemas/bars_core/src/bars_v1.rs
```

Generated output checks:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface projection --out crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface projection --out crates/schemas/bars_core/src/bars_v1.rs
```

Inspect commands:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --inspect --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface projection
```

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --inspect --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface projection
```

Generated direct-writer API naming is deterministic. For a projection whose
function stem is `<projection>`, codegen must emit:

```rust
impl SourceMarker {
    pub fn project_<projection>(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>>;

    pub unsafe fn project_<projection>_trusted_unchecked(
        bytes: &[u8],
        max_response_bytes: usize,
    ) -> Result<Vec<u8>>;
}

fn project_<projection>_archived_direct(
    archived: &ArchivedSourcePayload,
    max_response_bytes: usize,
) -> Result<Vec<u8>>;
```

The generated helper body must use schema-specific borrowed writer types. Their
names must be based on the projected marker type:

```text
<ProjectionMarker>DirectPayloadRef<'a>
<ProjectionMarker>DirectRowsRef<'a>
<ProjectionMarker>DirectRowsIter<'a>
<ProjectionMarker>DirectRowRef<'a>
```

The generated borrowed writer layer must implement or use rkyv serialization so
the archived output type is the same projected archived payload type produced by
`ProjectedMarker::encode_owned` for the same projected logical rows.

Generated projected row collection serialization must use:

```text
rkyv::vec::ArchivedVec::serialize_from_iter
```

over generated projected row references. Local rkyv 0.8.16 source shows this
method uses `SerVec::with_capacity` for resolver storage. That resolver storage
is allowed only as serializer scratch, not as projected field storage, and must
be recorded in result review if it affects performance.

Variable-width field strategy:

- raw strings serialize from source archived string slices into the final
  projected archive;
- bytes serialize from source archived byte slices into the final projected
  archive;
- numeric arrays serialize from source archived array values into the final
  projected archive without collecting into an owned `Vec`;
- nullable arrays with absent presence bit serialize as empty arrays with the
  projected presence bit clear;
- optional scalar/string/bytes fields preserve projected presence-bit semantics.

Unsupported schema combinations:

- none for currently supported MBT field kinds listed in Section 16;
- any future field kind without direct-writer support must fail at codegen, not
  fall back to owned projection.

Rejected generated hot-path code shapes inside direct projection helpers:

```text
Vec::with_capacity(archived
rows.push(
encode_owned(rows
.to_string()
.to_vec()
.collect()
```

These shapes may still appear in core owned encode, fixtures, or tests, but not
inside generated direct projection helper bodies.

## 10. Crate Boundary Contract

Generated schema crates must not depend on `metamorphic_binary_transport_codegen`.

The Bars schema crate is a benchmark schema crate only:

```text
crates/schemas/bars_core
```

It may depend only on:

```text
metamorphic_binary_transport_core = { path = "../../core" }
rkyv = "=0.8.16"
```

The compatibility schema crate remains:

```text
crates/schemas/test_compatibility_core
```

It may keep only:

```text
metamorphic_binary_transport_core = { path = "../../core" }
rkyv = "=0.8.16"
```

The projection benchmark crate is:

```text
crates/benches
```

It may depend on:

```text
metamorphic_binary_transport_core
metamorphic_binary_transport_schema_bars
metamorphic_binary_transport_schema_test_compatibility
```

No adapter crate dependency is allowed in generated schema crates or projection
benchmarks for this work.

`crates/projection` remains an ownership-boundary crate. This spec does not
authorize moving the hot path into `crates/projection`; the hot path is emitted
into schema-specific generated modules.

## 11. Dependency Contract

No new external dependency is approved.

The only external dependency used by generated schema code remains:

```text
rkyv = "=0.8.16"
```

The only workspace dependency additions approved by this spec are:

```text
crates/schemas/bars_core -> crates/core
crates/schemas/bars_core -> rkyv = "=0.8.16"
crates/benches -> crates/core
crates/benches -> crates/schemas/bars_core
crates/benches -> crates/schemas/test_compatibility_core
```

Dependency validation commands:

```text
cargo tree -p metamorphic_binary_transport_schema_bars
cargo tree -p metamorphic_binary_transport_schema_test_compatibility
cargo tree -p metamorphic_binary_transport_benches
```

The result review must show no JSON, protobuf, CSV, Arrow, Arrow IPC, Parquet,
or metamorphose adapter dependency in these trees.

## 12. Determinism Contract

Generated code must be deterministic:

- same proto input and command produce identical generated source;
- `--check` must fail on generated source drift;
- generated Rust formatting uses repository rustfmt behavior;
- projected field order is deterministic and matches projected schema field
  order;
- projected presence bits are densely remapped in projected schema order;
- row order is preserved exactly from the source archived row order;
- checksum outputs are deterministic for the same input bytes.

The direct writer must not use reflection, dynamic field lookup, trait-object
dispatch, hash-map iteration, or schema-independent runtime projection dispatch
in the projection hot path.

## 13. Failure Contract

Projection must return existing `TransportError` variants for:

- truncated source bytes;
- invalid source header;
- source schema id mismatch;
- source schema version mismatch;
- source schema hash mismatch;
- source payload length mismatch;
- source payload checksum mismatch on checked path;
- source archive validation failure on checked path;
- projected response larger than `max_response_bytes`;
- projected row count mismatch if generated serialization observes mismatch;
- malformed projected archive during inspect validation.

Trusted projection may skip source payload checksum validation only through the
existing trusted access contract. It must still reject wrong source schema
identity and wrong source length.

Codegen must fail before runtime for:

- projection removes a key field;
- projection refers to an unknown field;
- projection refers to an unknown group;
- any currently supported MBT field kind cannot be emitted by the direct writer;
- unsupported future field kind appears in a projection.

## 14. Compile-surface Budget

Generated-code compile surface must be measured before performance claims.

Required build commands:

```text
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_core
```

```text
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_codegen --all-targets
```

```text
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_schema_test_compatibility --all-targets
```

```text
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_schema_bars --all-targets
```

```text
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_benches --all-targets
```

Required generated line-count commands:

```text
wc -l crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

```text
wc -l crates/schemas/bars_core/src/bars_v1.rs
```

Compile-surface acceptance:

- core must not gain adapter dependencies;
- generated schema crates must not gain codegen or adapter dependencies;
- direct projection generation must not cause compile failure under
  `cargo check --all-targets` for the schema crates;
- generated line count and build timings must be recorded in:

```text
docs/evidence/mbt_projection_direct_writer/compile_surface.md
```

If a generated schema crate build time regresses by more than 15 percent against
a same-session pre-implementation baseline recorded by the implementation plan,
the result review must mark compile-surface acceptance failed unless the peer
review accepts a measured reason.

## 15. Runtime Performance Budget

Runtime acceptance is measured on Bars projection lanes because Bars is the old
accepted projection benchmark shape.

Acceptance gates:

- Bars archived projection must be same or faster than the old crate archived
  projection for the same projection shape within a 3.5 percent jitter band;
- Bars direct writer must be same or faster than this repository's current
  generated owned-row projection within a 3.5 percent jitter band;
- public projection must not regress by more than 3.5 percent unless archived
  projection improves and source access dominates the public lane;
- compatibility-schema projection is a coverage gate for all field kinds, not
  the primary old-crate performance gate.

The result review must reject the implementation if archived direct projection
is repeatedly slower than both the old crate and the current owned-row generated
projection by more than 3.5 percent.

The current owned-row generated projection baseline must be captured before
replacing `emit_source_projection_api`. The implementation plan must stage this
explicitly and write:

```text
docs/evidence/mbt_projection_direct_writer/current_owned_row_projection_baseline.json
```

The baseline may be produced by a temporary plan-bound benchmark pass before the
direct-writer codegen edit, or by a plan-bound test-only reference lane that
executes the pre-rewrite generated projection output. It must not remain as a
production fallback after direct writer implementation.

No speed claim is valid without benchmark evidence under Section 17.

## 16. Correctness Oracle

Correctness must be proved before speed.

Schema correctness oracle:

- `mbt_codegen --inspect --surface projection` for Bars must match
  `crates/schemas/bars_core/tests/expected_bars_projection_inspect.txt`;
- the Bars inspect fixture must include schema id, schema version, transport
  name, payload root, schema hash, every dictionary, every physical field, every
  presence bit, every key part, and every projection marker;
- a mismatch in field order, field kind, presence bit, dictionary, key order,
  projection name, projected marker, schema id, schema version, transport name,
  or schema hash fails the test.

Projection bytes oracle:

- checked projection output validates under projected schema access;
- trusted projection output equals checked projection output byte-for-byte for
  the same input;
- projected output rejects under source schema access;
- source output rejects under projected schema access;
- row count is preserved;
- key order is preserved;
- projected schema hash appears in the projected envelope;
- projected payload checksum equals `fnv1a64(projected_payload_bytes)`;
- response cap uses `HEADER_LEN + projected_payload_len`;
- cap-boundary tests cover exactly-at-cap and one-byte-over-cap.

Owned-reference oracle:

- tests may construct expected projected owned rows as fixtures and encode them
  with `ProjectedMarker::encode_owned`;
- this is allowed only in tests as an oracle;
- production generated projection must not retain owned-row projection fallback.

Field-kind oracle:

Generated direct projection must support and test every current MBT field kind:

- `ConstU16`
- `U16Dictionary`
- optional `U16Dictionary`
- `U64BitmaskDictionary`
- `I32`
- optional `I32`
- `U32`
- optional `U32`
- `I64`
- optional `I64`
- `F32`
- optional `F32`
- `F64`
- optional `F64`
- `Bool`
- optional `Bool`
- `RawString`
- optional `RawString`
- `Bytes`
- optional `Bytes`
- `I64Array`
- nullable `I64Array`
- `I32Array`
- nullable `I32Array`
- `U32Array`
- nullable `U32Array`
- `F64Array`
- nullable `F64Array`
- `F32Array`
- nullable `F32Array`

Generated-output oracle:

- tests must scan generated direct projection helper bodies and reject:
  - `Vec::with_capacity(archived`
  - `rows.push(`
  - `encode_owned(rows`
  - `.to_string()`
  - `.to_vec()`
  - `.collect()`

The scan must be scoped to direct projection helper bodies so normal owned
encode and test fixtures do not create false failures.

## 17. Benchmark Methodology

Benchmark crate:

```text
crates/benches
```

Benchmark source files:

```text
crates/benches/src/lib.rs
crates/benches/src/projection.rs
crates/benches/src/bin/mbt_projection_bench.rs
```

Benchmark command:

```text
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_projection_bench -- --report-dir docs/evidence/mbt_projection_direct_writer
```

Required Bars lanes:

```text
mathilde_binary_project_no_metadata_public
mathilde_binary_project_no_metadata_archived
mathilde_binary_project_no_metadata_inspect
mathilde_binary_project_ohlcv_only_public
mathilde_binary_project_ohlcv_only_archived
mathilde_binary_project_ohlcv_only_inspect
```

Required compatibility-schema lanes:

```text
mbt_project_no_optional_public
mbt_project_no_optional_archived
mbt_project_no_optional_inspect
mbt_project_numeric_only_public
mbt_project_numeric_only_archived
mbt_project_numeric_only_inspect
```

Required row counts:

```text
1
100
500
1000
10000
100000
```

Every result row must include:

- label;
- row count;
- output bytes;
- total milliseconds;
- rows/sec;
- MB/sec;
- source access milliseconds;
- projection milliseconds;
- projected inspect milliseconds;
- response checksum;
- semantic checksum when inspected;
- minimal projection checksum when inspected;
- old-crate comparison row for Bars lanes;
- current-owned-row comparison row for same-repository lanes.

Stable run definition:

- run the benchmark command three sequential times in release mode;
- record all three outputs;
- compare median rows/sec and median MB/sec;
- mark a lane unstable if max/min rows/sec differs by more than 7 percent across
  the three runs;
- do not accept a performance claim from an unstable lane without rerunning and
  explaining the instability in the result review.

Baseline policy:

- old-crate comparison values come from
  `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/docs/bench_results.md`;
- if the old crate is rerun, the result review must record both old published
  values and fresh old-run values;
- disagreement between old published and fresh old-run values is not hidden and
  must be reported as benchmark instability or environment drift.

Benchmark artifacts:

```text
docs/evidence/mbt_projection_direct_writer/projection_run_1.json
docs/evidence/mbt_projection_direct_writer/projection_run_2.json
docs/evidence/mbt_projection_direct_writer/projection_run_3.json
docs/evidence/mbt_projection_direct_writer/projection_summary.md
docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_result_review.md
```

Temporary benchmark data directory:

```text
target/mbt_projection_direct_writer/
```

## 18. Test Plan

Codegen tests:

```text
cargo test -p metamorphic_binary_transport_codegen --all-targets
```

Schema tests:

```text
cargo test -p metamorphic_binary_transport_schema_test_compatibility --all-targets
```

```text
cargo test -p metamorphic_binary_transport_schema_bars --all-targets
```

Bench harness tests:

```text
cargo test -p metamorphic_binary_transport_benches --all-targets
```

Required exact test files:

```text
crates/codegen/src/tests/test_rust_emit_projection_direct_writer.rs
crates/schemas/test_compatibility_core/tests/test_projection.rs
crates/schemas/bars_core/tests/test_bars_shape.rs
crates/schemas/bars_core/tests/test_bars_projection.rs
crates/schemas/bars_core/tests/expected_bars_projection_inspect.txt
crates/benches/src/tests/test_projection_bench_output.rs
```

The tests must cover:

- Bars inspect fixture equality;
- generated-output forbidden pattern checks;
- checked projection validation;
- trusted projection equality;
- source/projected schema rejection both directions;
- response cap boundary;
- optional/presence-bit remapping;
- raw string, bytes, arrays, nullable arrays;
- deterministic benchmark output schema.

## 19. Code Bindings

Implementation may edit only these files unless the implementation plan names a
protocol-approved reason:

```text
Cargo.toml
crates/codegen/src/model.rs
crates/codegen/src/descriptor.rs
crates/codegen/src/rust_emit.rs
crates/codegen/src/tests/mod.rs
crates/codegen/src/tests/test_cli.rs
crates/codegen/src/tests/test_descriptor.rs
crates/codegen/src/tests/test_rust_emit_core.rs
crates/codegen/src/tests/test_rust_emit_projection_direct_writer.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
crates/schemas/test_compatibility_core/tests/test_projection.rs
crates/schemas/bars_core/Cargo.toml
crates/schemas/bars_core/proto/mathilde/binary_transport/v1/bars.proto
crates/schemas/bars_core/src/lib.rs
crates/schemas/bars_core/src/bars_v1.rs
crates/schemas/bars_core/tests/test_bars_shape.rs
crates/schemas/bars_core/tests/test_bars_projection.rs
crates/schemas/bars_core/tests/expected_bars_projection_inspect.txt
crates/benches/Cargo.toml
crates/benches/src/lib.rs
crates/benches/src/projection.rs
crates/benches/src/bin/mbt_projection_bench.rs
crates/benches/src/tests/test_projection_bench_output.rs
```

Rejected implementation shapes:

- production generated slow fallback;
- generic projection runtime engine in the hot path;
- adapter crates in projection benchmark dependencies;
- hand-edited generated schema source;
- unbound generated artifacts;
- changes to MBT envelope semantics.

## 20. Generated Artifact Bindings

Generated source artifacts:

```text
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
crates/schemas/bars_core/src/bars_v1.rs
```

Only these commands own those artifacts:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface projection --out crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface projection --out crates/schemas/bars_core/src/bars_v1.rs
```

Generated check commands are the matching `--check` commands from Section 9.

Generated inspect artifacts:

```text
docs/evidence/mbt_projection_direct_writer/test_compatibility_projection_inspect.txt
docs/evidence/mbt_projection_direct_writer/bars_projection_inspect.txt
```

These artifacts are produced by redirecting the Section 9 `--inspect` commands.

No generated artifact may have two owners.

## 21. Review Artifact Bindings

Research brief:

```text
docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_research_brief.md
```

Peer audits:

```text
docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_peer_audit.md
docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_peer_audit_v2.md
```

Implementation plan:

```text
docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_implementation_plan.md
```

Evidence:

```text
docs/evidence/mbt_projection_direct_writer/compile_surface.md
docs/evidence/mbt_projection_direct_writer/test_compatibility_projection_inspect.txt
docs/evidence/mbt_projection_direct_writer/bars_projection_inspect.txt
docs/evidence/mbt_projection_direct_writer/current_owned_row_projection_baseline.json
docs/evidence/mbt_projection_direct_writer/projection_run_1.json
docs/evidence/mbt_projection_direct_writer/projection_run_2.json
docs/evidence/mbt_projection_direct_writer/projection_run_3.json
docs/evidence/mbt_projection_direct_writer/projection_summary.md
```

Result review:

```text
docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_result_review.md
```

## 22. Implementation Plan Requirement

Before any code change, the implementation plan must bind:

- exact file edits;
- exact generated files;
- exact commands;
- exact expected outputs;
- exact test order;
- exact benchmark order;
- exact compile-surface evidence collection;
- exact rollback boundary.

The implementation plan must also include the same-session pre-implementation
baseline commands for generated line counts and compile timings so the
compile-surface budget can be evaluated.

The implementation plan must also capture
`docs/evidence/mbt_projection_direct_writer/current_owned_row_projection_baseline.json`
before replacing the generated owned-row projection emission.

No implementation plan may keep production owned-row projection fallback.

## 23. Approval Checklist

Pre-audit closure checklist:

- mandatory section order matches `docs/protocols/spec_protocol.md`;
- prior projection specs were searched; this spec supersedes
  `mbt_projection_migration` only for generated direct-writer projection hot
  path and benchmark acceptance;
- command surfaces are exact for inspect, write, check, tests, and benchmarks;
- generated artifacts have one owner and one reproducibility command;
- runtime/codegen dispatch paths are listed in code bindings;
- tests that encode old behavior are either preserved as behavior or migrated
  to direct-writer behavior;
- code paths needed for the change are bound;
- compile-surface evidence commands are defined;
- no implementation detail required for correctness is deferred to the
  implementation plan.

Implementation readiness checklist:

- peer audit v2 passes;
- implementation plan is written;
- implementation plan is explicitly approved;
- no code has been changed under this spec before plan approval.

## 24. Open Questions

None.

If implementation proves that rkyv 0.8.16 resolver scratch makes direct writer
slower than the old owned-row path, the result review must reject the
implementation or require a new spec for a lower-level archive writer. That is a
future measured result, not an open spec decision.
