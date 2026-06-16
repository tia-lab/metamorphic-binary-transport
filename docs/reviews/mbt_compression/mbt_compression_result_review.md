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

# MBT Compression Result Review

Slug: `mbt_compression`

Spec:

```text
docs/specs/mbt_compression_SPEC.md
```

Implementation plan:

```text
docs/reviews/mbt_compression/mbt_compression_implementation_plan.md
```

Implementation plan audit:

```text
docs/reviews/mbt_compression/mbt_compression_implementation_plan_peer_audit_v2.md
```

Status: `IMPLEMENTED_WITH_EVIDENCE`

## What Changed

Added one opt-in production crate:

```text
crates/compression
```

The crate exposes zstd level-3 compression and decompression for completed MBT
bytes. It does not inspect schema descriptors, MBT headers, projections,
metamorphose outputs, or transponded columns.

Added one benchmark surface:

```text
crates/benches/src/compression.rs
crates/benches/src/bin/mbt_compression_bench.rs
crates/benches/src/tests/test_compression_bench_output.rs
```

The benchmark measures completed MBT bytes only:

- `mbt_full`;
- `mbt_no_metadata`;
- `mbt_ohlcv_only`.

Source byte construction is outside the timed compression/decompression loop.

## Files Changed

Edited:

- `Cargo.toml`
- `crates/benches/Cargo.toml`
- `crates/benches/docs/inventory.md`
- `crates/benches/src/lib.rs`
- `crates/benches/src/tests/mod.rs`
- `inventory.md`

Created:

- `crates/compression/Cargo.toml`
- `crates/compression/src/lib.rs`
- `crates/compression/src/runtime.rs`
- `crates/compression/src/tests/mod.rs`
- `crates/compression/src/tests/test_runtime.rs`
- `crates/compression/docs/inventory.md`
- `crates/benches/src/compression.rs`
- `crates/benches/src/bin/mbt_compression_bench.rs`
- `crates/benches/src/tests/test_compression_bench_output.rs`
- `docs/evidence/mbt_compression/.gitkeep`
- `docs/evidence/mbt_compression/compression_run_1.json`
- `docs/evidence/mbt_compression/compression_run_2.json`
- `docs/evidence/mbt_compression/compression_run_3.json`
- `docs/evidence/mbt_compression/compression_summary.md`
- `docs/evidence/mbt_compression/benchmark_environment.md`

Review/spec artifacts for this task were also created under:

```text
docs/specs/mbt_compression_SPEC.md
docs/reviews/mbt_compression/
```

Forbidden edit check:

- no core source file changed;
- no codegen source file changed;
- no generated schema source file changed;
- no metamorphose source file changed;
- no transponding source file changed;
- no adapter source file changed;
- no proto file changed.

## Dependency Evidence

New dependency:

```toml
zstd = "=0.13.3"
```

Dependency containment commands:

```bash
! cargo tree -p metamorphic_binary_transport_core | rg -n "zstd"
! cargo tree -p metamorphic_binary_transport_schema_bars | rg -n "zstd"
! cargo tree -p metamorphic_binary_transport_schema_test_compatibility | rg -n "zstd"
cargo tree -p metamorphic_binary_transport_compression | rg -n "zstd v0.13.3"
rg -n 'name = "zstd"|version = "0.13.3"' Cargo.lock
```

Observed result:

- core dependency tree: no zstd match;
- Bars schema dependency tree: no zstd match;
- test-compatibility schema dependency tree: no zstd match;
- compression dependency tree contains `zstd v0.13.3`;
- local `Cargo.lock` contains the compression crate, `name = "zstd"`, and
  `version = "0.13.3"`.

`Cargo.lock` is ignored in this repository (`git status --short --ignored
Cargo.lock` reports `!! Cargo.lock`), so the local lockfile is dependency
evidence but not a tracked source diff in this run.

## Validation Evidence

Initial implementation check:

```bash
cargo test -p metamorphic_binary_transport_compression --all-targets
```

First observed result:

- failed with `E0308` because `zstd::stream::copy_encode` and
  `zstd::stream::copy_decode` return `io::Result<()>` in `zstd 0.13.3`, not
  `io::Result<u64>`.

Fix:

- changed the internal stream-result mapper to accept `io::Result<()>`.

Final validation commands:

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

Observed final results:

- `cargo fmt --check`: passed.
- compression crate tests: 9 passed, 0 failed.
- benchmark crate tests: 13 passed, 0 failed.
- core `cargo check`: passed.
- compression `cargo check`: passed.
- dependency containment checks: passed.
- forbidden-call static check: passed with no matches.
- smoke benchmark: passed and wrote
  `docs/evidence/mbt_compression/compression_run_1.json`.
- full benchmark: passed and wrote
  `docs/evidence/mbt_compression/compression_run_2.json`.
- post-structure full benchmark: passed and wrote
  `docs/evidence/mbt_compression/compression_run_3.json`.
- inventory generation: passed and wrote `inventory.md`.

## Benchmark Evidence

Full benchmark artifact:

```text
docs/evidence/mbt_compression/compression_run_3.json
```

Summary artifact:

```text
docs/evidence/mbt_compression/compression_summary.md
```

Environment artifact:

```text
docs/evidence/mbt_compression/benchmark_environment.md
```

Environment recorded:

- command: `target/release/mbt_compression_bench --report-dir docs/evidence/mbt_compression`
- build profile: `release`
- rustc: `rustc 1.90.0 (1159e78c4 2025-09-14)`
- os: `Linux 5.15.0-156-generic`
- git dirty: `dirty`
- zstd version: `0.13.3`
- zstd level: `3`

Full benchmark deterministic result:

```json
{
  "all_byte_equal": true,
  "source_checksum_stable": true,
  "compressed_checksum_stable": true,
  "decompressed_checksum_stable": true
}
```

Full benchmark summary:

| Label | Lane | Rows | Uncompressed bytes | Compressed bytes | Ratio | Reduction % | Compress MB/s | Decompress MB/s |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| one | mbt_full | 1 | 462 | 206 | 0.445887 | 55.411255 | 4.541098 | 84.699998 |
| one | mbt_no_metadata | 1 | 288 | 160 | 0.555556 | 44.444444 | 5.882433 | 136.431383 |
| one | mbt_ohlcv_only | 1 | 200 | 114 | 0.570000 | 43.000000 | 4.228035 | 110.620955 |
| small | mbt_full | 100 | 32538 | 5233 | 0.160827 | 83.917266 | 238.181262 | 648.484983 |
| small | mbt_no_metadata | 100 | 15138 | 3777 | 0.249505 | 75.049544 | 202.726957 | 596.939609 |
| small | mbt_ohlcv_only | 100 | 6338 | 1770 | 0.279268 | 72.073209 | 165.970440 | 701.973834 |
| page_500 | mbt_full | 500 | 162138 | 24848 | 0.153252 | 84.674783 | 414.359358 | 1127.373134 |
| page_500 | mbt_no_metadata | 500 | 75138 | 17692 | 0.235460 | 76.453991 | 297.656862 | 833.364511 |
| page_500 | mbt_ohlcv_only | 500 | 31138 | 7266 | 0.233348 | 76.665168 | 265.228004 | 710.789279 |
| page_1000 | mbt_full | 1000 | 324138 | 49275 | 0.152019 | 84.798142 | 404.681186 | 1147.437039 |
| page_1000 | mbt_no_metadata | 1000 | 150138 | 35311 | 0.235190 | 76.480971 | 299.218112 | 833.227642 |
| page_1000 | mbt_ohlcv_only | 1000 | 62138 | 14159 | 0.227864 | 77.213621 | 291.030184 | 768.855101 |
| medium | mbt_full | 10000 | 3240138 | 515683 | 0.159155 | 84.084536 | 335.132688 | 1137.355684 |
| medium | mbt_no_metadata | 10000 | 1500138 | 372364 | 0.248220 | 75.178017 | 255.214367 | 828.339810 |
| medium | mbt_ohlcv_only | 10000 | 620138 | 135534 | 0.218555 | 78.144542 | 268.289735 | 782.145065 |
| large | mbt_full | 100000 | 32400138 | 5415168 | 0.167134 | 83.286590 | 290.388576 | 1031.673879 |
| large | mbt_no_metadata | 100000 | 15000138 | 3939701 | 0.262644 | 73.735568 | 173.768741 | 750.633507 |
| large | mbt_ohlcv_only | 100000 | 6200138 | 1455666 | 0.234780 | 76.522039 | 196.932443 | 723.468055 |

## Proved By This Run

- Compression is opt-in and outside the MBT envelope.
- The new compression crate does not add zstd to core or schema dependency
  trees.
- `compress_into` and `decompress_into` roundtrip completed MBT bytes under the
  tested caps.
- Invalid and truncated zstd frames return `MalformedArchive`.
- Output cap overflows return `ResponseTooLarge`.
- Full and projected Bars MBT bytes roundtrip through zstd level `3` in the
  recorded benchmark.
- The benchmark records byte equality and checksum stability for the measured
  lanes.

## Not Proved

- zstd internal allocation behavior is not proved.
- Compatibility-schema compression is not measured in this phase.
- Non-MBT adapter output compression is not implemented or measured.
- The recorded speed and ratio claims apply only to this machine, build, zstd
  version, row fixture, and benchmark command.

## Conclusion

The approved compression scope is implemented and validated.

No production claim beyond completed MBT byte compression/decompression is made
by this result review.
