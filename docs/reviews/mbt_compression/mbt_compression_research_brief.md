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

# MBT Compression Research Brief

Slug: `mbt_compression`

Status: `RESEARCH_COMPLETE_SPEC_DRAFTED`

Repository: `/home/tia/_DEV/MATHILDE/metamorphic-binary-transport`

Task class: research and spec preparation only.

This brief does not authorize implementation.

## Source Materials

Protocol and invariant reads:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/research_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/protocols/implementation_protocol.md`
- `docs/protocols/code_style_protocol.md`
- `docs/protocols/testing_benchmark_protocol.md`

Architecture and local code reads:

- `architecture.md`
- `Cargo.toml`
- `crates/core/src/envelope.rs`
- `crates/core/src/error.rs`
- `crates/core/src/output.rs`
- `crates/core/src/runtime.rs`
- `crates/metamorphose/src/runtime.rs`
- `crates/schemas/bars_core/src/bars_v1.rs`
- `crates/benches/src/bars_regression.rs`
- `crates/benches/src/projection.rs`

Prior MBT compression evidence:

- `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/compression.rs`
- `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/bench_results.md`
- `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/test_results.md`

External dependency docs:

- `https://docs.rs/zstd/0.13.3/zstd/`
- `https://docs.rs/zstd/0.13.3/zstd/stream/`
- `https://docs.rs/zstd/0.13.3/zstd/stream/write/struct.Encoder.html`
- `https://docs.rs/zstd/0.13.3/zstd/stream/fn.encode_all.html`
- `https://crates.io/crates/zstd/0.13.3`

## Measured Object

The measured object is an optional MBT compression crate that compresses and
decompresses completed MBT byte responses, including already projected MBT
responses.

Measured surfaces:

- MBT bytes -> zstd frame bytes;
- zstd frame bytes -> MBT bytes;
- response cap enforcement during compression output;
- response cap enforcement during decompression output;
- byte equality after roundtrip;
- compressed size, compression ratio, compression throughput, decompression
  throughput;
- compile-surface containment of the zstd dependency.

Not measured by this brief:

- schema encode performance;
- projection performance;
- metamorphose JSON/protobuf/CSV/Arrow/Parquet performance;
- source-data finality;
- network transport;
- non-Rust SDK compression.

## Candidate Approach

Create an opt-in crate:

```text
crates/compression
```

Package name:

```text
metamorphic_binary_transport_compression
```

The crate provides small zstd helpers over completed MBT bytes:

```rust
pub const DEFAULT_ZSTD_LEVEL: i32 = 3;

pub struct CompressionConfig {
    pub level: i32,
}

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

`compress_into` and `decompress_into` are the production hot-path APIs because
the caller owns the output buffer. `compress` and `decompress` are explicit
convenience allocation points.

The compressed artifact is a zstd frame containing MBT bytes. It is not a new
MBT wire envelope. A caller must decompress before calling MBT access.

## Evidence Table

| Evidence type | Source | Observation |
| --- | --- | --- |
| Code-read evidence | `docs/invariants/core_invariants.md` | Compression is outside the MBT envelope unless a later approved spec changes that contract. |
| Code-read evidence | `architecture.md` | Core, schemas, adapters, benches, and codegen are separate compile surfaces; adapters do not enter core. |
| Code-read evidence | `Cargo.toml` | Current workspace has no `crates/compression` member and only pins `rkyv` in workspace dependencies. |
| Code-read evidence | `crates/core/src/envelope.rs` | Checked access validates identity, length, and checksum; trusted access validates identity and length only. |
| Code-read evidence | `crates/core/src/error.rs` | `TransportError::ResponseTooLarge` and `TransportError::MalformedArchive` already provide reusable cap and corrupt-byte failure surfaces. |
| Code-read evidence | `crates/core/src/output.rs` | Existing output helpers enforce response caps while writing into owned buffers. Compression should follow this style with a capped writer. |
| Code-read evidence | `crates/metamorphose/src/runtime.rs` | Trusted access is explicit and unsafe at MBT access boundaries. Compression should not hide validation or trusted access. |
| Code-read evidence | `crates/schemas/bars_core/src/bars_v1.rs` | Full and projected MBT paths already produce completed MBT byte vectors; compression can treat them as opaque bytes. |
| Prior benchmark evidence | Old `docs/bench_results.md` | Existing MBT compression evidence measured zstd `0.13.3` level `3` externally over completed response bytes. |
| Prior correctness evidence | Old `docs/test_results.md` | Prior compression benchmark proved byte equality and stable response/compressed checksums for its measured run. |
| External-doc evidence | docs.rs `zstd 0.13.3` crate docs | The crate provides read/write wrappers and common convenience functions for compression and decompression. |
| External-doc evidence | docs.rs `Encoder` docs | Stream encoder writes compressed data into a supplied writer; level `0` maps to zstd default level `3`. |
| External-doc evidence | docs.rs `encode_all` docs | Convenience `encode_all` returns `Vec<u8>` and produces zstd frame-format output. |

## Hypotheses

These are not proved yet:

- `compress_into` and `decompress_into` will reduce MBT-owned allocations
  compared with the old benchmark's `encode_all` and `decode_all` convenience
  calls.
- zstd level `3` remains the best default for the first production helper.
- zstd internal buffering and context setup overhead are acceptable for the
  target MBT payload sizes.
- compression ratios from the old Bars benchmark will remain similar after the
  workspace split.

## Unknowns

- Exact compression and decompression throughput in the new workspace.
- Whether zstd context reuse should be exposed later. It is not included in the
  first candidate because it expands API state and must be measured first.
- Whether dictionaries are useful for small repeated MBT payloads. They are out
  of scope for the first candidate.
- Whether non-Rust SDKs should use zstd through native libraries or through
  language-specific package adapters. That is a downstream SDK decision.

## Risks

- Pulling `zstd` into core would violate the crate boundary invariants.
- Hiding decompression behind MBT trusted access would blur the validation
  contract. Decompression only reconstructs bytes; MBT checked or trusted
  access remains a separate caller decision.
- Convenience APIs allocate; benchmarks must separate them from `*_into` APIs.
- zstd has internal buffers and native-library behavior. MBT can control its own
  output buffers, but cannot claim zero allocations inside zstd without
  allocator instrumentation.
- Compression speed claims require fresh run evidence in this repository.

## Required Decisions Before Spec

Closed by this brief:

- Compression remains outside the MBT envelope.
- Compression is MBT-byte oriented, not schema-specific.
- Default zstd level is explicit `3`.
- The first dependency target is `zstd = "=0.13.3"`.
- The crate is opt-in and separate from core, schemas, adapters, codegen, and
  benches.
- `*_into` APIs are the production hot path; convenience APIs are declared
  allocation points.

## Evidence Required Before Coding

Before implementation:

- spec peer audit must pass;
- implementation plan must bind exact files, commands, and artifacts;
- implementation plan peer audit must pass;
- user approval must be explicit.

Before performance claims:

- correctness tests must prove byte-for-byte roundtrip equality;
- corrupt compressed input must reject;
- response cap overflow must reject for compression and decompression;
- dependency-tree checks must prove `zstd` does not enter core or schema crates;
- benchmark evidence must record command, profile, dataset, row counts,
  compression level, byte sizes, ratios, throughput, and checksums.

## Recommended Next Phase

Write and peer-audit:

```text
docs/specs/mbt_compression_SPEC.md
docs/reviews/mbt_compression/mbt_compression_peer_audit.md
```

Implementation must not start until the spec passes peer audit and an approved
implementation plan exists.
