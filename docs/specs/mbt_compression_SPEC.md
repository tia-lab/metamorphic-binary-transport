# SPEC: MBT Compression

## 1. Identification

Slug: `mbt_compression`

Repository:

```text
/home/tia/_DEV/MATHILDE/metamorphic-binary-transport
```

Task class: spec authoring.

Research brief:

```text
docs/reviews/mbt_compression/mbt_compression_research_brief.md
```

## 2. Status

Status: `AMENDED_PER_IMPLEMENTATION_PLAN_PEER_AUDIT_AWAITING_IMPLEMENTATION_PLAN_PEER_AUDIT_V2`

This spec does not authorize implementation.

Implementation may start only after:

1. this spec passes peer audit;
2. an implementation plan is written;
3. the implementation plan passes peer audit;
4. the user explicitly approves implementation.

## 3. Purpose

Add an opt-in MBT compression crate for completed MBT byte responses.

The first supported compression algorithm is zstd level `3`.

Compression is an outer policy:

```text
MBT bytes -> zstd frame bytes
zstd frame bytes -> MBT bytes
```

The compressed artifact is not an MBT envelope. Callers must decompress before
calling checked or trusted MBT access.

The crate must support full MBT bytes and projected MBT bytes without schema
specific branches or codegen changes.

## 4. Non-goals

This spec does not:

- change the MBT envelope;
- add a compressed MBT wire encoding kind;
- add compression flags to `TransportHeader`;
- compress individual rows inside an archive;
- add dictionaries;
- add async compression;
- add network transport integration;
- add schema-specific generated compression functions;
- add compression to core, schema crates, metamorphose, transponding, or
  adapter crates;
- compress JSON, protobuf, CSV, Arrow, Arrow IPC, or Parquet adapter output in
  the first production API;
- benchmark the all-fields compatibility schema in this first compression pass;
- change projection, metamorphose, or transponding behavior;
- claim compression speed or ratio before benchmark evidence exists.

## 5. Measured Object

Measured object:

```text
completed MBT bytes <-> zstd frame bytes
```

Measured lanes:

- `mbt_full`;
- `mbt_no_metadata`;
- `mbt_ohlcv_only`.

Measured metrics:

- uncompressed bytes;
- compressed bytes;
- compression ratio;
- reduction percent;
- compression elapsed time;
- decompression elapsed time;
- compression MB/sec;
- decompression MB/sec;
- source checksum;
- compressed checksum;
- decompressed checksum;
- byte equality.

Not measured:

- source row generation;
- MBT encode throughput;
- projection throughput;
- adapter output throughput;
- network throughput;
- storage throughput.

## 6. Schema Source Contract

Compression is schema-agnostic.

The input is an already produced MBT byte slice. The slice may represent:

- a full schema response;
- a projected schema response;
- a schema owned outside this repository that used MBT codegen.

Compression must not inspect protobuf descriptors.

Compression must not depend on generated schema modules.

Compression must not change schema hash, schema ID, schema version, row count,
payload checksum, or any MBT header byte.

## 7. Wire and Archive Contract

The compressed output is a zstd frame whose uncompressed bytes are exactly the
input MBT bytes.

The compressed output is not:

- `MAGIC = MATBT001`;
- `ENCODING_MBT_RKYV`;
- an MBT `TransportHeader`;
- an rkyv archive payload.

Allowed data movement:

```text
MBT bytes
  -> compress_into
  -> zstd frame bytes
  -> decompress_into
  -> identical MBT bytes
```

Forbidden data movement:

```text
zstd frame bytes -> MBT checked/trusted access
```

The caller must decompress first.

The implementation must not add any custom wrapper header around the zstd
frame in this phase. Schema identity stays in the inner MBT bytes.

## 8. Checked and Trusted Access Contract

Compression and decompression are byte transforms. They do not validate MBT
archive semantics.

Checked path:

```text
compressed bytes
  -> decompress_into
  -> S::access / S::inspect / generated checked helper
```

Trusted path:

```text
compressed bytes from an immutable trusted boundary
  -> decompress_into
  -> unsafe generated trusted helper
```

The safety contract for trusted access remains owned by the generated schema
trusted helper. Compression must not expose a safe API that implies trusted MBT
access.

The compression crate must not define a `TrustedUnchecked` token. That token
belongs to MBT access/metamorphose surfaces, not byte decompression.

## 9. Codegen Contract

No codegen change is authorized by this spec.

Generated schema files must not be edited.

Compression must work for any generated schema because it sees only completed
MBT bytes.

Codegen-check commands remain out of scope for this spec unless implementation
accidentally changes generated files, in which case the implementation must
stop and diagnose.

## 10. Crate Boundary Contract

Create one opt-in crate:

```text
crates/compression
```

Package name:

```text
metamorphic_binary_transport_compression
```

Allowed dependencies:

- `metamorphic_binary_transport_core`;
- `zstd = "=0.13.3"`.

Forbidden dependencies:

- generated schema crates;
- `metamorphic_binary_transport_codegen`;
- `metamorphic_binary_transport_metamorphose`;
- `metamorphic_binary_transport_transponding`;
- adapter crates;
- `serde`;
- `serde_json`;
- `prost`;
- Arrow crates;
- Parquet crates.

Root workspace membership must add only:

```text
"crates/compression"
```

Core, schema, adapter, codegen, and benchmark crates must not acquire a
dependency on `metamorphic_binary_transport_compression`.

The bench crate may depend on `metamorphic_binary_transport_compression` for
measurement.

## 11. Dependency Contract

The selected zstd dependency is:

```toml
zstd = "=0.13.3"
```

External-doc evidence:

- docs.rs `zstd 0.13.3` documents read/write wrappers and convenience
  compression/decompression functions:
  `https://docs.rs/zstd/0.13.3/zstd/`
- docs.rs `zstd::stream` documents zstd streams as read/write interfaces:
  `https://docs.rs/zstd/0.13.3/zstd/stream/`
- docs.rs `zstd::stream::write::Encoder` documents an encoder that writes
  compressed data into a supplied writer and states level `0` uses the zstd
  default, currently `3`:
  `https://docs.rs/zstd/0.13.3/zstd/stream/write/struct.Encoder.html`
- docs.rs `zstd::stream::encode_all` documents convenience output as a zstd
  frame `Vec<u8>`:
  `https://docs.rs/zstd/0.13.3/zstd/stream/fn.encode_all.html`
- crates.io identifies `zstd 0.13.3` as the registry package version:
  `https://crates.io/crates/zstd/0.13.3`

MBT must not rely on zstd level `0` to mean default. MBT default is explicit:

```rust
pub const DEFAULT_ZSTD_LEVEL: i32 = 3;
```

The first implementation must use zstd stream write/read APIs for `*_into`
paths so the caller-provided output buffer is the declared MBT-owned output
copy point.

The spec does not claim zero allocations inside zstd. zstd may allocate or use
internal buffers. MBT controls its own public output buffers and measures the
complete compression/decompression path.

## 12. Determinism Contract

For identical input bytes, algorithm version, level, and implementation, the
benchmark must record whether compressed checksums are stable across iterations.

Required deterministic checks:

- decompressed bytes equal source bytes;
- source checksum equals decompressed checksum;
- compressed checksum is recorded for every run;
- compressed checksum stability is reported, not assumed.

The implementation must not use wall-clock time, random data, or external state
inside compression helpers.

Benchmark metadata may include timestamps, but timestamps are not part of the
compression correctness oracle.

## 13. Failure Contract

Required failures:

| Case                            | Required behavior                                            |
| ------------------------------- | ------------------------------------------------------------ |
| compressed output exceeds cap   | return `TransportError::ResponseTooLarge`                    |
| decompressed output exceeds cap | return `TransportError::ResponseTooLarge`                    |
| invalid zstd frame              | return `TransportError::MalformedArchive`                    |
| truncated zstd frame            | return `TransportError::MalformedArchive`                    |
| zstd encoder error              | return `TransportError::MalformedArchive`                    |
| zstd decoder error              | return `TransportError::MalformedArchive`                    |
| empty input compression         | allowed if zstd accepts it; roundtrip must equal empty bytes |
| empty input decompression       | reject unless zstd accepts it as a valid frame               |

The implementation must not use `unwrap`, `expect`, or `panic!` in library code.

The implementation must not silently truncate output.

`compress_into` and `decompress_into` must clear the output buffer before
writing. On error, the buffer contents are unspecified and must not be used by
callers. This keeps the hot API simple and deterministic on success.

## 14. Compile-surface Budget

Adding compression must not change core compile surface.

Required compile-surface checks after implementation:

```bash
cargo check -p metamorphic_binary_transport_core
cargo check -p metamorphic_binary_transport_compression
! cargo tree -p metamorphic_binary_transport_core | rg -n "zstd"
! cargo tree -p metamorphic_binary_transport_schema_bars | rg -n "zstd"
! cargo tree -p metamorphic_binary_transport_schema_test_compatibility | rg -n "zstd"
cargo tree -p metamorphic_binary_transport_compression | rg -n "zstd v0.13.3"
rg -n 'name = "zstd"|version = "0.13.3"' Cargo.lock
```

Expected output:

- the first two `cargo check` commands pass;
- the three negative core/schema dependency checks pass by producing no matches
  and returning success through the negated pipeline;
- the compression crate dependency tree contains `zstd v0.13.3`.
- `Cargo.lock` contains the locked `zstd` package at version `0.13.3`.

No compile-time improvement claim is authorized by this spec.

## 15. Runtime Performance Budget

Runtime claims require benchmark evidence.

Required budget shape:

- no MBT checked validation inside measured `compress_into`;
- no MBT checked validation inside measured `decompress_into`;
- no schema lookup inside compression helpers;
- no projection or metamorphose work inside compression helpers;
- caller-provided output buffer is the only MBT-owned hot-path output buffer
  for `*_into` APIs;
- convenience APIs allocate and must be measured separately or excluded from
  hot-path claims.

No numeric speed threshold is approved before the first new benchmark run.

The result review must compare new evidence with the old MBT compression
boundary only as context, not as a parity requirement, because the new API
separates `*_into` from old `encode_all` and `decode_all` convenience calls.

## 16. Correctness Oracle

Primary oracle:

```text
decompress(compress(mbt_bytes)) == mbt_bytes
```

Required checksum oracle:

```text
checksum(source_mbt_bytes) == checksum(decompressed_mbt_bytes)
```

Required cap oracle:

```text
compressed_len > max_compressed_bytes -> ResponseTooLarge
decompressed_len > max_decompressed_bytes -> ResponseTooLarge
```

Required corrupt-input oracle:

```text
invalid_or_truncated_zstd_frame -> MalformedArchive
```

For benchmark lanes that use generated schemas, MBT byte production must be
outside the measured compression/decompression loop unless the benchmark labels
the lane as end-to-end.

## 17. Benchmark Methodology

Create one focused benchmark binary:

```text
crates/benches/src/bin/mbt_compression_bench.rs
```

Benchmark implementation module:

```text
crates/benches/src/compression.rs
```

Report directory:

```text
docs/evidence/mbt_compression
```

Required command:

```bash
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_compression_bench -- --report-dir docs/evidence/mbt_compression
```

Optional smoke command:

```bash
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_compression_bench -- --smoke --report-dir docs/evidence/mbt_compression
```

Required row-count shape:

| Label     |    Rows | Full-run iterations |
| --------- | ------: | ------------------: |
| one       |       1 |                  50 |
| small     |     100 |                  50 |
| page_500  |     500 |                  50 |
| page_1000 |   1,000 |                  50 |
| medium    |  10,000 |                  10 |
| large     | 100,000 |                   3 |

Required fixture identity:

| Field                   | Value                                                                  |
| ----------------------- | ---------------------------------------------------------------------- |
| fixture source          | `crates/benches/src/projection.rs::bars_rows`                          |
| fixture shape           | deterministic synthetic Bars rows derived from row index               |
| RNG seed                | not applicable; no RNG is used                                         |
| max response bytes      | `crates/benches/src/projection.rs::MAX_RESPONSE_BYTES = 1_073_741_824` |
| full schema hash        | `BarsV1::SCHEMA_HASH = 6061383958499356843`                            |
| no-metadata schema hash | `BarsV1NoMetadata::SCHEMA_HASH = 2810655320728765998`                  |
| OHLCV-only schema hash  | `BarsV1OhlcvOnly::SCHEMA_HASH = 792323260406987672`                    |

Required benchmark lanes:

- `mbt_full`;
- `mbt_no_metadata`;
- `mbt_ohlcv_only`.

Required source byte construction:

| Lane              | Source byte construction                                                          |
| ----------------- | --------------------------------------------------------------------------------- |
| `mbt_full`        | `BarsV1::encode(&bars_rows(row_count), MAX_RESPONSE_BYTES)`                       |
| `mbt_no_metadata` | full MBT bytes, then `BarsV1::project_no_metadata(&full_mbt, MAX_RESPONSE_BYTES)` |
| `mbt_ohlcv_only`  | full MBT bytes, then `BarsV1::project_ohlcv_only(&full_mbt, MAX_RESPONSE_BYTES)`  |

The source byte construction happens outside the measured
compression/decompression loop.

Required report fields:

- `experiment`;
- `status`;
- `utc_ms`;
- `command`;
- `build_profile`;
- `rustc`;
- `os`;
- `cpu`;
- `git_dirty`;
- `dataset_identity`;
- `fixture_source`;
- `fixture_seed`;
- `schema_hashes`;
- `zstd_version`;
- `zstd_level`;
- `max_response_bytes`;
- `row_counts`;
- `metrics`;
- `determinism`.

Required metric fields:

- `label`;
- `lane`;
- `rows`;
- `iterations`;
- `uncompressed_bytes`;
- `compressed_bytes`;
- `compression_ratio`;
- `reduction_percent`;
- `compress_us`;
- `decompress_us`;
- `compress_mb_per_sec`;
- `decompress_mb_per_sec`;
- `p50_compress_us`;
- `p95_compress_us`;
- `p99_compress_us`;
- `p50_decompress_us`;
- `p95_decompress_us`;
- `p99_decompress_us`;
- `source_checksum`;
- `compressed_checksum`;
- `decompressed_checksum`;
- `source_schema_hash`;
- `byte_equal`.

Required artifact names:

```text
docs/evidence/mbt_compression/compression_run_1.json
docs/evidence/mbt_compression/compression_summary.md
docs/evidence/mbt_compression/benchmark_environment.md
```

The benchmark must not update README tables directly. Result review owns
interpretation.

## 18. Test Plan

Compression crate tests:

```bash
cargo test -p metamorphic_binary_transport_compression --all-targets
```

Required tests:

- default config uses level `3`;
- `compress` then `decompress` preserves bytes;
- `compress_into` then `decompress_into` preserves bytes;
- `compress_into` reuses caller buffer and clears prior contents on success;
- `decompress_into` reuses caller buffer and clears prior contents on success;
- compressed cap overflow returns `ResponseTooLarge`;
- decompressed cap overflow returns `ResponseTooLarge`;
- invalid zstd bytes return `MalformedArchive`;
- truncated zstd bytes return `MalformedArchive`.

Bench crate tests:

```bash
cargo test -p metamorphic_binary_transport_benches --all-targets
```

Required tests:

- smoke benchmark writes a JSON report;
- report includes all required MBT lanes;
- report records zstd version `0.13.3`;
- report records zstd level `3`;
- every smoke metric has `byte_equal=true`.

Static checks:

```bash
! rg -n "unwrap\\(|expect\\(|panic!|todo!|unreachable!" crates/compression/src crates/benches/src/compression.rs crates/benches/src/bin/mbt_compression_bench.rs
```

Expected result: command succeeds by finding no matches in runtime or reusable
benchmark support code, except test-only panic wrappers if the implementation
plan explicitly binds them.

## 19. Code Bindings

Workspace files to edit:

```text
Cargo.toml
Cargo.lock
```

`Cargo.lock` is an allowed Cargo-generated dependency artifact for this spec.
It must not be edited manually. It may change only because Cargo resolves the
new exact dependency:

```toml
zstd = "=0.13.3"
```

Unrelated lockfile package upgrades, removals, or dependency drift are rejected
and require rollback or spec amendment.

Production files to create:

```text
crates/compression/Cargo.toml
crates/compression/src/lib.rs
crates/compression/src/runtime.rs
crates/compression/src/tests/mod.rs
crates/compression/src/tests/test_runtime.rs
crates/compression/docs/inventory.md
```

Benchmark files to edit:

```text
crates/benches/Cargo.toml
crates/benches/src/lib.rs
crates/benches/src/tests/mod.rs
```

Benchmark files to create:

```text
crates/benches/src/compression.rs
crates/benches/src/bin/mbt_compression_bench.rs
crates/benches/src/tests/test_compression_bench_output.rs
```

Evidence paths to create or update:

```text
docs/evidence/mbt_compression/.gitkeep
docs/evidence/mbt_compression/compression_run_1.json
docs/evidence/mbt_compression/compression_summary.md
docs/evidence/mbt_compression/benchmark_environment.md
```

Documentation paths to create after implementation:

```text
docs/reviews/mbt_compression/mbt_compression_result_review.md
```

Inventory paths to update after implementation:

```text
inventory.md
crates/benches/docs/inventory.md
crates/compression/docs/inventory.md
```

Forbidden edits:

```text
crates/core/src/*
crates/codegen/src/*
crates/schemas/*/src/*.rs
crates/metamorphose/src/*
crates/transponding/src/*
crates/adapters/*/src/*
proto/*
```

If any forbidden edit appears necessary, implementation must stop and return to
spec amendment.

## 20. Generated Artifact Bindings

No generated source artifact is owned by this spec.

Allowed Cargo-generated dependency artifact:

```text
Cargo.lock
```

The lockfile may be updated only by Cargo dependency resolution for the
approved `zstd = "=0.13.3"` dependency. It is not hand-authored generated code,
but it is a repository artifact whose ownership is bound by this spec.

Generated schema files are inputs only:

```text
crates/schemas/bars_core/src/bars_v1.rs
```

They must not be modified.

No codegen command is authorized by this spec.

If a later implementation needs generated changes, this spec is invalid for
that implementation and must be amended before code changes continue.

## 21. Review Artifact Bindings

Research brief:

```text
docs/reviews/mbt_compression/mbt_compression_research_brief.md
```

Spec:

```text
docs/specs/mbt_compression_SPEC.md
```

Required peer audit:

```text
docs/reviews/mbt_compression/mbt_compression_peer_audit.md
```

Required implementation plan:

```text
docs/reviews/mbt_compression/mbt_compression_implementation_plan.md
```

Required implementation plan peer audit:

```text
docs/reviews/mbt_compression/mbt_compression_implementation_plan_peer_audit.md
```

Required result review after implementation:

```text
docs/reviews/mbt_compression/mbt_compression_result_review.md
```

## 22. Implementation Plan Requirement

The implementation plan must bind:

- exact `Cargo.toml` dependency edits;
- exact `Cargo.lock` generated dependency-artifact policy;
- exact public API signatures;
- exact capped writer implementation shape;
- exact zstd stream API calls;
- exact tests;
- exact benchmark report schema;
- exact evidence paths;
- exact validation commands;
- exact rollback boundary.

The plan must also state:

- no core/schema/codegen/metamorphose/transponding/adapter edits;
- no generated file edits;
- no compression flags in the MBT envelope;
- no hidden schema access inside compression helpers.

## 23. Approval Checklist

Required reads:

- [x] `AGENTS.md`
- [x] `docs/invariants/core_invariants.md`
- [x] lifecycle protocol
- [x] research protocol
- [x] spec protocol
- [x] peer audit protocol
- [x] implementation protocol
- [x] code style protocol
- [x] testing benchmark protocol
- [x] relevant source code
- [x] relevant old MBT compression evidence
- [x] external zstd dependency docs

Pre-audit closure checklist:

- [x] mandatory section order matches `docs/protocols/spec_protocol.md`;
- [x] prior specs searched for `compression`, `zstd`, `compressed`, and
      `decompress`;
- [x] prior invariant that compression is outside the MBT envelope is preserved;
- [x] command surfaces are exact for checks, tests, and benchmarks, including
      executable negative dependency checks;
- [x] generated artifact ownership is closed: no generated source artifacts are
      owned; `Cargo.lock` is bound as a Cargo-generated dependency artifact for
      `zstd = "=0.13.3"`;
- [x] runtime dispatch paths are closed: compression is a separate crate and
      does not enter core/metamorphose dispatch;
- [x] test migration is closed: old convenience compression benchmark behavior
      is not copied as production API; `*_into` is the new hot path;
- [x] benchmark source bytes are bound to deterministic Bars fixture,
      deterministic construction paths, and schema hashes;
- [x] optional compatibility-schema benchmark lane is removed from this phase;
- [x] exact code paths are bound;
- [x] no design decision is deferred to the implementation plan;
- [x] compile-surface evidence commands are defined, including lockfile version
      evidence.

Implementation readiness:

- [ ] peer audit passed;
- [ ] implementation plan written;
- [ ] implementation plan peer audit passed;
- [ ] user approved implementation.

## 24. Open Questions

No implementation-blocking open question remains for the first compression
crate.

Deferred questions:

- whether zstd context reuse is worth exposing;
- whether dictionaries help small MBT payloads;
- whether downstream SDKs should expose compression helpers;
- whether non-MBT adapter outputs should receive their own compression bench
  later.
