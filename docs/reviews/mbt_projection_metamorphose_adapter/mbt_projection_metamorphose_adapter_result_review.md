# MBT Projection Metamorphose Adapter Result Review

Slug: `mbt_projection_metamorphose_adapter`

Status: `IMPLEMENTED_VALIDATED`

## Scope Implemented

Implemented projection-marker metamorphose adapter generation for:

- `json`
- `protobuf`
- `csv`
- `transponding`
- `arrow`
- `arrow-ipc`
- `parquet`

Runtime shape remains:

```text
source MBT -> generated MBT projection -> projected MBT -> adapter output
```

Adapters do not project. Projection remains MBT-to-MBT before boundary
conversion.

## Files Changed

Codegen:

- `crates/codegen/src/rust_emit.rs`
- `crates/codegen/src/tests/test_rust_emit_metamorphose.rs`

Runtime tests:

- `crates/schemas/bars_core/tests/test_projection_metamorphose.rs`
- `crates/schemas/test_compatibility_core/tests/test_projection_metamorphose.rs`

Generated artifacts:

- `crates/schemas/bars_core/src/bars_v1_json.rs`
- `crates/schemas/bars_core/src/bars_v1_protobuf.rs`
- `crates/schemas/bars_core/src/bars_v1_csv.rs`
- `crates/schemas/bars_core/src/bars_v1_transponding.rs`
- `crates/schemas/bars_core/src/bars_v1_arrow.rs`
- `crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs`
- `crates/schemas/bars_core/src/bars_v1_parquet.rs`
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_json.rs`
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_protobuf.rs`
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_csv.rs`
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_transponding.rs`
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow.rs`
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow_ipc.rs`
- `crates/schemas/test_compatibility_core/src/test_compatibility_v1_parquet.rs`

No runtime adapter crate, core crate, proto, Cargo manifest, or lockfile edits
were made.

## Codegen Commands

All generated artifacts were written with this exact shape:

```bash
cargo run -p mbt_codegen --bin mbt_codegen -- \
  --write \
  --proto-root <schema_proto_root> \
  --proto-root proto \
  --schema <schema> \
  --root <root> \
  --module <module> \
  --surface metamorphose \
  --adapter <adapter> \
  --out <out>
```

Bars command bindings:

```text
proto-root: crates/schemas/bars_core/proto
schema: mathilde/binary_transport/v1/bars.proto
root: mathilde.binary_transport.v1.MathildeTransportResponseV1
module: bars_v1
adapters/out:
json -> crates/schemas/bars_core/src/bars_v1_json.rs
protobuf -> crates/schemas/bars_core/src/bars_v1_protobuf.rs
csv -> crates/schemas/bars_core/src/bars_v1_csv.rs
transponding -> crates/schemas/bars_core/src/bars_v1_transponding.rs
arrow -> crates/schemas/bars_core/src/bars_v1_arrow.rs
arrow-ipc -> crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs
parquet -> crates/schemas/bars_core/src/bars_v1_parquet.rs
```

Test-compatibility command bindings:

```text
proto-root: crates/schemas/test_compatibility_core/proto
schema: mathilde/binary_transport/test_compatibility/v1/all_fields.proto
root: mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1
module: test_compatibility_v1
adapters/out:
json -> crates/schemas/test_compatibility_core/src/test_compatibility_v1_json.rs
protobuf -> crates/schemas/test_compatibility_core/src/test_compatibility_v1_protobuf.rs
csv -> crates/schemas/test_compatibility_core/src/test_compatibility_v1_csv.rs
transponding -> crates/schemas/test_compatibility_core/src/test_compatibility_v1_transponding.rs
arrow -> crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow.rs
arrow-ipc -> crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow_ipc.rs
parquet -> crates/schemas/test_compatibility_core/src/test_compatibility_v1_parquet.rs
```

The same 14 bindings were validated with `--check`; all checks passed.

## Validation Evidence

Format:

```bash
cargo fmt -p mbt_codegen -p mbt_schema_bars -p mbt_schema_test_compatibility
```

Passed with no output.

Codegen tests:

```bash
cargo test -p mbt_codegen --all-targets
```

Observed:

```text
37 passed; 0 failed
0 binary tests
```

Bars schema tests:

```bash
cargo test -p mbt_schema_bars --all-targets --features json,protobuf,csv,arrow,arrow_ipc,parquet
```

Observed:

```text
test_bars_metamorphose: 2 passed
test_bars_projection: 3 passed
test_bars_shape: 2 passed
test_projection_metamorphose: 1 passed
```

Test-compatibility schema tests:

```bash
cargo test -p mbt_schema_test_compatibility --all-targets --features json,protobuf,csv,arrow,arrow_ipc,parquet
```

Observed:

```text
test_determinism: 1 passed
test_failures: 12 passed
test_projection: 3 passed
test_projection_metamorphose: 2 passed
test_roundtrip: 1 passed
```

No-default compile checks:

```bash
cargo check -p mbt_schema_bars --no-default-features
cargo check -p mbt_schema_test_compatibility --no-default-features
```

Observed:

```text
both finished successfully
```

Timed codegen check:

```bash
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p mbt_codegen --all-targets
```

Observed:

```text
elapsed=0.51 user=0.76 sys=0.19 max_rss_kb=157220
```

Forbidden source scan:

```bash
rg -n 'crates/serving|BarsV1|BarsV1NoMetadata|NoMetadata|OhlcvOnly' crates/codegen/src --glob '!tests/**'
```

Observed:

```text
no matches
```

## What Is Proved

- Projection marker adapter sections are generated for every supported adapter.
- Generated projection adapters compile for Bars and test-compatibility schemas.
- Checked and trusted projection marker adapter outputs match for JSON,
  protobuf, CSV, Arrow IPC, and Parquet in the runtime tests.
- Arrow projection marker outputs expose retained projected fields and omit
  removed fields in runtime schema inspection.
- Row-format projection outputs omit removed metadata or optional fields in the
  tested projections.
- Generated files are reproducible through the matching `--check` commands.
- No schema-specific Bars/serving names were introduced into non-test codegen
  source.

## Remaining Limits

- Parquet readback schema inspection is not exposed by the MBT Parquet adapter;
  runtime tests assert deterministic non-empty Parquet bytes with `PAR1`
  framing.
- This review does not benchmark adapter throughput. It proves generation,
  compile, and runtime correctness for the covered projection adapter paths.
