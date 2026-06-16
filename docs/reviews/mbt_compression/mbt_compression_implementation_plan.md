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

# MBT Compression Implementation Plan

Slug: `mbt_compression`

Status: `AMENDED_PER_IMPLEMENTATION_PLAN_PEER_AUDIT_AWAITING_IMPLEMENTATION_PLAN_PEER_AUDIT_V2`

Spec:

```text
docs/specs/mbt_compression_SPEC.md
```

Spec peer audit:

```text
docs/reviews/mbt_compression/mbt_compression_peer_audit_v3.md
```

This plan does not authorize implementation. Code changes may start only after
this plan passes peer audit and the user explicitly approves implementation.

## Goal

Add a small opt-in compression crate that transforms completed MBT bytes to and
from zstd frame bytes without changing MBT core, generated schema code, codegen,
projection, metamorphose, transponding, adapters, or proto files.

The production API shape is:

```text
MBT bytes -> compress_into -> zstd frame bytes
zstd frame bytes -> decompress_into -> MBT bytes
```

Convenience APIs may allocate. The `*_into` APIs are the production hot-path
surface because callers provide the output buffer.

## Required Reads Before Code

Before editing, re-read:

1. `AGENTS.md`
2. `docs/invariants/core_invariants.md`
3. `docs/protocols/lifecycle_protocol.md`
4. `docs/protocols/implementation_protocol.md`
5. `docs/protocols/code_style_protocol.md`
6. `docs/protocols/testing_benchmark_protocol.md`
7. `docs/specs/mbt_compression_SPEC.md`
8. `docs/reviews/mbt_compression/mbt_compression_peer_audit_v3.md`
9. `crates/core/src/error.rs`
10. `crates/benches/src/projection.rs`
11. `crates/schemas/bars_core/src/bars_v1.rs`

If any read invalidates this plan, stop and amend the spec or plan before code.

## Files To Edit

Edit only these existing files:

1. `Cargo.toml`
2. `Cargo.lock`
3. `crates/benches/Cargo.toml`
4. `crates/benches/src/lib.rs`
5. `crates/benches/src/tests/mod.rs`
6. `crates/benches/docs/inventory.md`
7. `inventory.md`

`Cargo.lock` must be changed only by Cargo dependency resolution for
`zstd = "=0.13.3"`. Do not edit it manually.

## Files To Create

Create production crate files:

1. `crates/compression/Cargo.toml`
2. `crates/compression/src/lib.rs`
3. `crates/compression/src/runtime.rs`
4. `crates/compression/src/tests/mod.rs`
5. `crates/compression/src/tests/test_runtime.rs`
6. `crates/compression/docs/inventory.md`

Create benchmark files:

1. `crates/benches/src/compression.rs`
2. `crates/benches/src/bin/mbt_compression_bench.rs`
3. `crates/benches/src/tests/test_compression_bench_output.rs`

Create evidence and review files during validation:

1. `docs/evidence/mbt_compression/.gitkeep`
2. `docs/evidence/mbt_compression/compression_run_1.json`
3. `docs/evidence/mbt_compression/compression_summary.md`
4. `docs/evidence/mbt_compression/benchmark_environment.md`
5. `docs/reviews/mbt_compression/mbt_compression_result_review.md`

## Forbidden Edits

Do not edit:

```text
crates/core/src/*
crates/codegen/src/*
crates/schemas/*/src/*.rs
crates/metamorphose/src/*
crates/transponding/src/*
crates/adapters/*/src/*
proto/*
```

If any forbidden edit appears necessary, stop and return to spec amendment.

## Dependency Changes

Root `Cargo.toml`:

- add workspace member:

```toml
"crates/compression"
```

`crates/compression/Cargo.toml`:

```toml
[package]
name = "metamorphic_binary_transport_compression"
version.workspace = true
edition.workspace = true
authors.workspace = true
publish = false

[dependencies]
metamorphic_binary_transport_core = { path = "../core" }
zstd = "=0.13.3"
```

`crates/benches/Cargo.toml`:

- add local dependency:

```toml
metamorphic_binary_transport_compression = { path = "../compression" }
```

No other dependency is approved.

## Production API

Expose from `crates/compression/src/lib.rs`:

```rust
#![forbid(unsafe_code)]

pub mod runtime;

pub use runtime::{
    CompressionConfig, DEFAULT_ZSTD_LEVEL, compress, compress_into, compress_with_config,
    decompress, decompress_into,
};
```

Implement in `crates/compression/src/runtime.rs`:

```rust
#![forbid(unsafe_code)]

pub const DEFAULT_ZSTD_LEVEL: i32 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompressionConfig {
    pub level: i32,
}

impl CompressionConfig {
    pub const fn new(level: i32) -> Self;
}

impl Default for CompressionConfig;

pub fn compress(bytes: &[u8], max_compressed_bytes: usize) -> Result<Vec<u8>>;

pub fn compress_with_config(
    bytes: &[u8],
    max_compressed_bytes: usize,
    config: CompressionConfig,
) -> Result<Vec<u8>>;

pub fn compress_into(
    bytes: &[u8],
    out: &mut Vec<u8>,
    max_compressed_bytes: usize,
    config: CompressionConfig,
) -> Result<()>;

pub fn decompress(compressed: &[u8], max_decompressed_bytes: usize) -> Result<Vec<u8>>;

pub fn decompress_into(
    compressed: &[u8],
    out: &mut Vec<u8>,
    max_decompressed_bytes: usize,
) -> Result<()>;
```

`Result` is:

```rust
metamorphic_binary_transport_core::error::Result
```

## Production Implementation Shape

Implement one private capped writer:

```rust
struct CappedVecWriter<'a> {
    out: &'a mut Vec<u8>,
    cap: usize,
    overflow: Option<usize>,
}
```

`CappedVecWriter` implements `std::io::Write`.

Write behavior:

1. `write` computes `observed = out.len() + input.len()` with checked addition.
2. If `observed > cap`, store `overflow = Some(observed)` and return an
   `std::io::Error`.
3. Otherwise append the bytes to `out` and return `Ok(input.len())`.
4. `flush` returns `Ok(())`.

Error mapping:

1. If `overflow` is set, return:

```rust
TransportError::ResponseTooLarge { observed, cap }
```

2. Otherwise map zstd I/O errors to:

```rust
TransportError::MalformedArchive(err.to_string())
```

Compression path:

```rust
out.clear();
let source = std::io::Cursor::new(bytes);
let mut writer = CappedVecWriter::new(out, max_compressed_bytes);
zstd::stream::copy_encode(source, &mut writer, config.level)
```

Decompression path:

```rust
out.clear();
let source = std::io::Cursor::new(compressed);
let mut writer = CappedVecWriter::new(out, max_decompressed_bytes);
zstd::stream::copy_decode(source, &mut writer)
```

Convenience APIs:

- `compress` creates a new `Vec<u8>` and delegates to
  `compress_with_config` with `CompressionConfig::default()`.
- `compress_with_config` creates a new `Vec<u8>` and delegates to
  `compress_into`.
- `decompress` creates a new `Vec<u8>` and delegates to `decompress_into`.

No schema access, header parsing, projection, metamorphose, or transponding is
allowed inside this crate.

## Production Tests

Create `crates/compression/src/tests/mod.rs` with only:

```rust
mod test_runtime;
```

Create `crates/compression/src/tests/test_runtime.rs`.

Required tests:

1. `default_config_uses_level_three`
2. `compress_and_decompress_preserve_bytes`
3. `compress_into_and_decompress_into_preserve_bytes`
4. `compress_into_reuses_preallocated_buffer_on_success`
5. `decompress_into_reuses_preallocated_buffer_on_success`
6. `compress_into_rejects_compressed_cap_overflow`
7. `decompress_into_rejects_decompressed_cap_overflow`
8. `decompress_rejects_invalid_zstd_bytes`
9. `decompress_rejects_truncated_zstd_bytes`

Test rules:

- tests return `Result<()>` or `std::result::Result<(), Box<dyn Error>>`;
- no `unwrap`, `expect`, `panic!`, `todo!`, or `unreachable!`;
- use pattern matching for `TransportError::MalformedArchive` because the
  zstd error string is dependency-owned.

## Benchmark Implementation

Update `crates/benches/src/lib.rs`:

```rust
pub mod compression;
```

Update `crates/benches/src/tests/mod.rs`:

```rust
mod test_compression_bench_output;
```

Create `crates/benches/src/compression.rs` with:

- row-count model;
- report model;
- metric model;
- deterministic source byte construction;
- compression/decompression timing;
- report writing;
- environment writing;
- summary writing.

Use existing deterministic fixture:

```rust
crate::projection::bars_rows(row_count)
```

Use existing max response cap:

```rust
crate::projection::MAX_RESPONSE_BYTES
```

Required source byte construction:

```rust
let rows = bars_rows(row_count);
let full = BarsV1::encode(&rows, MAX_RESPONSE_BYTES)?;
let no_metadata = BarsV1::project_no_metadata(&full, MAX_RESPONSE_BYTES)?;
let ohlcv_only = BarsV1::project_ohlcv_only(&full, MAX_RESPONSE_BYTES)?;
```

This construction must happen before the measured compression/decompression
loop.

Required schema hashes:

```rust
BarsV1::SCHEMA_HASH
BarsV1NoMetadata::SCHEMA_HASH
BarsV1OhlcvOnly::SCHEMA_HASH
```

Required lanes:

1. `mbt_full`
2. `mbt_no_metadata`
3. `mbt_ohlcv_only`

Required full-run row counts and iterations:

| Label | Rows | Iterations |
| --- | ---: | ---: |
| one | 1 | 50 |
| small | 100 | 50 |
| page_500 | 500 | 50 |
| page_1000 | 1,000 | 50 |
| medium | 10,000 | 10 |
| large | 100,000 | 3 |

Required smoke-run row counts and iterations:

| Label | Rows | Iterations |
| --- | ---: | ---: |
| one | 1 | 3 |
| small | 100 | 3 |
| page_1000 | 1,000 | 3 |

Metric semantics:

- `compress_us`: total compression microseconds across all iterations;
- `decompress_us`: total decompression microseconds across all iterations;
- percentile fields: computed from per-iteration timings sorted ascending;
- MB/sec fields: `uncompressed_bytes * iterations / total_seconds`;
- `source_checksum`: `metamorphic_binary_transport_core::codec::response_checksum(source)`;
- `compressed_checksum`: checksum of the first compressed output, with
  determinism checks across later iterations;
- `decompressed_checksum`: checksum of the decompressed bytes;
- `byte_equal`: decompressed bytes equal source bytes for every iteration.

Determinism report:

- `all_byte_equal`;
- `source_checksum_stable`;
- `compressed_checksum_stable`;
- `decompressed_checksum_stable`.

Report writing:

- use `serde` and `serde_json`, already present in the bench crate, for the
  benchmark artifact only;
- write pretty JSON;
- refuse to overwrite an existing `compression_run_N.json`;
- write `compression_summary.md` from the just-written report;
- write `benchmark_environment.md` with command, build profile, rustc, OS,
  CPU, git dirty state, zstd version, zstd level, and report path.

## Benchmark Binary

Create:

```text
crates/benches/src/bin/mbt_compression_bench.rs
```

CLI:

```text
mbt_compression_bench [--smoke] --report-dir <path>
```

Behavior:

- parse only `--smoke` and `--report-dir <path>`;
- reject unknown flags;
- call the benchmark module;
- print the report path on success;
- print the error and exit `1` on failure.

## Benchmark Output Tests

Create:

```text
crates/benches/src/tests/test_compression_bench_output.rs
```

Required tests:

1. `compression_row_counts_are_bound`
2. `compression_lanes_are_bound`
3. `compression_report_uses_required_fields`
4. `next_compression_run_path_refuses_overwrite`
5. `sample_compression_report_records_zstd_level_three`

The report-field test must check for:

- top-level report fields from the spec;
- metric fields from the spec;
- deterministic field ordering as emitted by the report structs.

## Inventory Updates

Create:

```text
crates/compression/docs/inventory.md
```

Update:

```text
crates/benches/docs/inventory.md
inventory.md
```

Run:

```bash
cargo make inventory
```

The global inventory must include `crate::compression` and the new bench files.

## Validation Commands

Run after implementation:

```bash
cargo fmt --check
cargo test -p metamorphic_binary_transport_compression --all-targets
cargo test -p metamorphic_binary_transport_benches --all-targets
cargo check -p metamorphic_binary_transport_core
cargo check -p metamorphic_binary_transport_compression
! cargo tree -p metamorphic_binary_transport_core | rg -n "zstd"
! cargo tree -p metamorphic_binary_transport_schema_bars | rg -n "zstd"
! cargo tree -p metamorphic_binary_transport_schema_test_compatibility | rg -n "zstd"
cargo tree -p metamorphic_binary_transport_compression | rg -n "zstd v0.13.3"
rg -n 'name = "zstd"|version = "0.13.3"' Cargo.lock
! rg -n "unwrap\\(|expect\\(|panic!|todo!|unreachable!" crates/compression/src crates/benches/src/compression.rs crates/benches/src/bin/mbt_compression_bench.rs
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_compression_bench -- --smoke --report-dir docs/evidence/mbt_compression
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_compression_bench -- --report-dir docs/evidence/mbt_compression
cargo make inventory
```

The static `rg` check is written in negated executable form to enforce the spec
intent of no matches.

Expected outputs:

- all build and test commands pass;
- core and schema dependency-tree checks produce no zstd matches;
- compression crate dependency tree contains `zstd v0.13.3`;
- lockfile check finds `zstd` at `0.13.3`;
- static forbidden-call check produces no matches;
- smoke benchmark writes a report;
- full benchmark writes the next available `compression_run_N.json`;
- summary and environment artifacts exist;
- inventory generation succeeds.

## Result Review

After validation, write:

```text
docs/reviews/mbt_compression/mbt_compression_result_review.md
```

The result review must include:

- files changed;
- dependency and lockfile evidence;
- test command results;
- benchmark command results;
- path to benchmark artifacts;
- compression ratios and throughput summary;
- determinism result;
- explicit statement that speed and ratio claims are limited to the recorded
  Bars benchmark;
- explicit statement that zstd internal allocation behavior is not proved.

## Rollback Boundary

If implementation fails or is rejected, rollback all files listed in:

- Files To Edit
- Files To Create
- evidence files created during validation
- result review file if created

Do not rollback unrelated user changes.

`Cargo.lock` rollback must remove only dependency-resolution changes caused by
`zstd = "=0.13.3"` and its transitive dependencies.

## Known Risks

- zstd internal allocation behavior is dependency-owned and not proved zero.
- `*_into` can control MBT-owned output buffers but cannot prevent zstd
  internal buffers.
- The first benchmark covers Bars full and projected MBT only.
- The benchmark uses the bench crate's existing `serde`/`serde_json` surface
  for evidence writing, not production runtime.
- The benchmark uses the first Bars compression lane only; compatibility-schema
  compression is not measured in this phase.

## Approval Gate

Implementation is blocked until:

1. this plan passes implementation-plan peer audit;
2. the user explicitly approves implementation.
