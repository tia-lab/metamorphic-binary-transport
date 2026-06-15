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

# Corrective Implementation Plan: MBT Bars Regression Benchmark

Slug: `mbt_bars_regression_benchmark`
Date: 2026-06-15
Status: `AMENDED_PER_SPEC_PEER_AUDIT_V3_AWAITING_IMPLEMENTATION_PLAN_PEER_AUDIT_V3`

Primary repository:

```text
/home/tia/_DEV/MATHILDE/metamorphic-binary-transport
```

Old implementation repository:

```text
/home/tia/_DEV/MATHILDE/experiments
```

This plan does not authorize code changes. It must pass a separate
implementation-plan peer audit and then be explicitly approved before any code
edit starts.

## Source Chain

| Artifact | Path | Status |
|---|---|---|
| Research brief | `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_research_brief.md` | complete |
| Spec | `docs/specs/mbt_bars_regression_benchmark_SPEC.md` | corrective parity-port feature amendment present |
| Corrective peer audit v2 | `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_peer_audit_v2.md` | passed |
| Corrective peer audit v3 | `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_peer_audit_v3.md` | passed |
| Existing implementation plan | `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_implementation_plan.md` | superseded for corrective parity claims |
| Existing corrective implementation plan | this file before this amendment | superseded; prior status was `BLOCKED_BEFORE_CODE` |
| Implementation plan peer audit v2 | `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_implementation_plan_peer_audit_v2.md` | blocked; spec-plan mismatch resolved by corrective peer audit v3 |
| Existing result review | `docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_result_review.md` | non-authoritative for old-vs-new parity |

## Goal

Correct the Bars regression benchmark comparison by adding an old-MBT parity
benchmark that mirrors the current new split benchmark semantics.

The accepted comparison is:

```text
old MBT implementation + new benchmark semantics
vs
new split MBT implementation + same benchmark semantics
```

The plan intentionally does not:

- modify new codegen;
- add generated archived adapter entrypoints;
- hand-edit generated files;
- compare speed against historical `bench_results.md`;
- change production runtime behavior;
- compile old non-Bars generated modules or old non-parity benchmark modules
  for the old parity benchmark command.

## Evidence From Code Read

| Evidence type | Path | Observed behavior |
|---|---|---|
| Code-read evidence | `crates/benches/src/bars_regression.rs` | Current new benchmark still carries historical markdown baseline parsing and fields. |
| Code-read evidence | `crates/benches/src/bin/mbt_bars_regression_bench.rs` | Current new benchmark parses `OLD_BENCH_RESULTS` before measuring and attaches old markdown comparisons. |
| Code-read evidence | `crates/benches/src/projection.rs` | Current Bars fixture is deterministic BTCUSDT-only with all generated Bars presence bits set. |
| Code-read evidence | `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/bars_v1.rs` | Old Bars row field names differ for some ordinal fields: `source`, `process`, and `recomputed_reason` instead of new `*_ordinal` names. |
| Code-read evidence | `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/bars_v1.rs` | Old Bars generated code exposes checked JSON/protobuf/CSV/Arrow IPC/Parquet functions and trusted JSON/protobuf functions. |
| Code-read evidence | `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/bars_v1.rs` | Old Bars generated code exposes `access_archived_trusted_unchecked` and crate-internal archived helpers for CSV, Arrow IPC, and Parquet. |
| Code-read evidence | `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/main.rs` | Old binary dispatches benchmark subcommands from `main.rs`. |
| Code-read evidence | `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/mod.rs` | Old benchmark modules are exported from `src/benches/mod.rs`. |

## Files To Edit

New split repository:

```text
crates/benches/src/bars_regression.rs
crates/benches/src/bin/mbt_bars_regression_bench.rs
crates/benches/src/tests/test_bars_regression_bench_output.rs
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_result_review.md
```

Old experiments repository:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/Cargo.toml
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/lib.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/mod.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/main.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/tests/mod.rs
```

No other existing file is approved.

## Files To Create

Old experiments repository:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/bars_regression_parity.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/tests/test_bars_regression_parity.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/evidences/bars_regression_parity/.gitkeep
```

New split repository after corrective runs:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md
```

No generated file may be created or modified.

## Dependency Changes

No dependency package changes are approved.

The only `Cargo.toml` edit approved is adding an old-crate feature flag with
no dependencies:

```toml
bars-regression-parity-only = []
```

The feature is allowed only to limit old-crate compilation for the parity
benchmark command. It must not change non-featured old-crate behavior.

These files must not change:

```text
/home/tia/_DEV/MATHILDE/metamorphic-binary-transport/Cargo.toml
/home/tia/_DEV/MATHILDE/metamorphic-binary-transport/Cargo.lock
/home/tia/_DEV/MATHILDE/experiments/Cargo.toml
/home/tia/_DEV/MATHILDE/experiments/Cargo.lock
```

If implementation proves any dependency edit, lockfile edit, or workspace
manifest edit is required, work must stop and the spec and plan must be
amended and audited again.

## Implementation Steps

### 0. Add old parity-only compile feature

Edit:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/Cargo.toml
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/lib.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/mod.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/tests/mod.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/main.rs
```

Required changes:

- add only this old-crate feature:

```toml
bars-regression-parity-only = []
```

- under `bars-regression-parity-only`, expose only the old Bars generated
  modules required by the parity benchmark:

```rust
pub mod generated {
    pub mod bars_v1;
    pub mod bars_v1_proto;
}
```

- keep the existing generated module graph unchanged when the feature is not
  enabled;
- under `bars-regression-parity-only`, compile only:

```text
src/benches/bars_regression_parity.rs
src/tests/test_bars_regression_parity.rs
the `bench-bars-regression-parity` binary dispatch path
```

- keep all existing old benchmark modules, test modules, and binary dispatches
  unchanged when the feature is not enabled;
- do not edit `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/mod.rs`;
- do not edit any generated file.

### 1. Remove historical markdown speed comparison from the new benchmark

Edit:

```text
crates/benches/src/bars_regression.rs
crates/benches/src/bin/mbt_bars_regression_bench.rs
crates/benches/src/tests/test_bars_regression_bench_output.rs
```

Required changes:

- remove use of `OLD_BENCH_RESULTS` as a speed baseline;
- remove the required historical markdown parser from the measured run path;
- remove mandatory calls to `parse_old_bars_baselines` and
  `verify_required_old_baselines` from the benchmark binary;
- keep the current deterministic Bars fixture unchanged:

```rust
pub fn bars_rows(row_count: usize) -> Vec<MathildeBarRowV1> {
    crate::projection::bars_rows(row_count)
}
```

- keep current labels exactly:

```text
bars_mbt_full_encode_inspect_checked
bars_metamorphose_json_checked
bars_metamorphose_protobuf_checked
bars_metamorphose_csv_checked
bars_metamorphose_json_trusted
bars_metamorphose_protobuf_trusted
bars_metamorphose_csv_trusted
bars_metamorphose_arrow_ipc_trusted
bars_metamorphose_parquet_trusted
bars_serde_json_baseline
```

- keep current row counts:

```text
1, 100, 500, 1_000, 10_000, 100_000
```

- keep current single-pass timing boundary unless the same change is applied
  to the old parity port in the same implementation.

The new benchmark may keep optional comparison fields as `null`, but it must
not populate them from historical `bench_results.md`.

### 2. Create old parity fixture in the old crate

Create:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/bars_regression_parity.rs
```

The old parity fixture must mirror the current new fixture from:

```text
/home/tia/_DEV/MATHILDE/metamorphic-binary-transport/crates/benches/src/projection.rs
```

Required old row field mapping:

| New fixture field | Old row field |
|---|---|
| `schema_version` | `schema_version` |
| `pair_ordinal = PAIR_BTCUSDT` | `pair_ordinal = PAIR_BTCUSDT` |
| `tf_ordinal = TIMEFRAME_1M` | `tf_ordinal = TF_1M` |
| `open_ms` | `open_ms` |
| `close_ms` | `close_ms` |
| `o` | `o` |
| `h` | `h` |
| `l` | `l` |
| `c` | `c` |
| `v` | `v` |
| `quote_v` | `quote_v` |
| `taker_known_v` | `taker_known_v` |
| `taker_signed_v` | `taker_signed_v` |
| `taker_known_quote_v` | `taker_known_quote_v` |
| `taker_signed_quote_v` | `taker_signed_quote_v` |
| `taker_known_n` | `taker_known_n` |
| `taker_signed_n` | `taker_signed_n` |
| `vw` | `vw` |
| `n` | `n` |
| `source_ordinal = SOURCE_FRONTIER` | `source = SOURCE_FRONTIER` |
| `process_ordinal = PROCESS_DERIVED` | `process = PROCESS_DERIVED` |
| `recomputed_reason_ordinal = RECOMPUTED_REASON_CANONICAL_REPAIR` | `recomputed_reason = RECOMPUTE_CANONICAL_REPAIR` |
| `presence_bits = PRESENCE_ALLOWED_MASK` | `presence_bits = PRESENCE_ALLOWED_MASK` |

All numeric value formulas must match the current new fixture exactly:

```text
close_ms = 1_700_000_000_000 + idx * 60_000
base = 100.0 + (idx % 10_000) * 0.01
o = base
h = base + 1.0
l = base - 1.0
c = base + 0.25
v = 1_000.0 + idx
quote_v = 100_000.0 + idx
taker_known_v = 500.0 + idx
taker_signed_v = -10.0 + (idx % 20)
taker_known_quote_v = 50_000.0 + idx
taker_signed_quote_v = -1_000.0 + idx
taker_known_n = idx
taker_signed_n = idx - 10
```

The fixture must call old generated row validation before returning rows:

```rust
crate::generated::bars_v1::validate_rows(&rows)?;
```

### 3. Implement old parity benchmark lanes

Create old benchmark support in:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/bars_regression_parity.rs
```

Required public entrypoint:

```rust
pub fn run_from_env() -> Result<()>;
```

Required CLI:

```text
mathilde_binary_transport bench-bars-regression-parity --report-dir <path>
```

Required labels, in order:

```text
bars_mbt_full_encode_inspect_checked
bars_metamorphose_json_checked
bars_metamorphose_protobuf_checked
bars_metamorphose_csv_checked
bars_metamorphose_json_trusted
bars_metamorphose_protobuf_trusted
bars_metamorphose_csv_trusted
bars_metamorphose_arrow_ipc_trusted
bars_metamorphose_parquet_trusted
bars_serde_json_baseline
```

Required lane boundaries:

| Label | Old implementation boundary |
|---|---|
| `bars_mbt_full_encode_inspect_checked` | time `BarsV1::encode(rows, cap)`, `BarsV1::inspect(&bytes)`, and `response_checksum(&bytes)` |
| `bars_metamorphose_json_checked` | encode outside timing, then time `BarsV1::metamorphose_json(&encoded, cap)` and checksum |
| `bars_metamorphose_protobuf_checked` | encode outside timing, then time `BarsV1::metamorphose_protobuf(&encoded, cap)` and checksum |
| `bars_metamorphose_csv_checked` | encode outside timing, then time `BarsV1::metamorphose_csv(&encoded, cap)` and checksum |
| `bars_metamorphose_json_trusted` | encode outside timing, then time old `unsafe BarsV1::metamorphose_json_trusted_unchecked(&encoded, cap)` and checksum |
| `bars_metamorphose_protobuf_trusted` | encode outside timing, then time old `unsafe BarsV1::metamorphose_protobuf_trusted_unchecked(&encoded, cap)` and checksum |
| `bars_metamorphose_csv_trusted` | encode outside timing, then time trusted archived access plus `BarsV1::metamorphose_csv_archived(archived, cap)` and checksum |
| `bars_metamorphose_arrow_ipc_trusted` | encode outside timing, then time trusted archived access plus `BarsV1::metamorphose_arrow_ipc_archived(archived, cap)` and checksum |
| `bars_metamorphose_parquet_trusted` | encode outside timing, then time trusted archived access plus `BarsV1::metamorphose_parquet_archived(archived, cap)` and checksum |
| `bars_serde_json_baseline` | build DTO outside timing, then time `serde_json::to_vec(&dto_rows)` and checksum |

For old trusted CSV, Arrow IPC, and Parquet, trusted archived access must be
inside the measured closure because the current new public trusted functions
also include trusted access inside the call.

Required output path pattern:

```text
crates/mathilde-binary-transport/docs/evidences/bars_regression_parity/bars_regression_parity_run_N.json
```

The writer must refuse to overwrite an existing run file.

### 4. Wire old benchmark dispatch

Edit:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/mod.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/main.rs
```

Required `mod.rs` export:

```rust
pub mod bars_regression_parity;
```

Under `bars-regression-parity-only`, `src/benches/mod.rs` must compile only
`bars_regression_parity`. Without the feature, all existing old benchmark
module exports must remain unchanged.

Required `main.rs` dispatch:

```rust
Some("bench-bars-regression-parity") => bars_regression_parity::run_from_env(),
```

Under `bars-regression-parity-only`, `src/main.rs` must accept only the
`bench-bars-regression-parity` command. Without the feature, all existing
subcommands must remain unchanged.

### 5. Add old parity tests

Create:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/tests/test_bars_regression_parity.rs
```

Edit:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/tests/mod.rs
```

Required test coverage:

- required labels match the exact spec list;
- required row counts are `1, 100, 500, 1_000, 10_000, 100_000`;
- selected fixture rows match the current new fixture formulas for row `0`,
  row `1`, and row `10_000` when applicable;
- writer refuses overwrite;
- sample JSON report contains metadata and required row fields;
- old fixture validation succeeds.

Tests must not use `unwrap`, `expect`, or `panic!`.

Under `bars-regression-parity-only`, `src/tests/mod.rs` must compile only
`test_bars_regression_parity`. Without the feature, all existing old test
module exports must remain unchanged.

### 6. Update new benchmark tests and metadata

Edit:

```text
crates/benches/src/tests/test_bars_regression_bench_output.rs
crates/benches/src/bars_regression.rs
```

Required changes:

- remove tests that require historical old markdown baselines;
- keep tests for current labels, row counts, report overwrite refusal, and
  serde DTO conversion;
- update metadata from historical baseline path to old parity evidence glob:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/evidences/bars_regression_parity/bars_regression_parity_run_*.json
```

### 7. Update result review status

Edit:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_result_review.md
```

Before new benchmark runs, set status to:

```text
RESULT_REVIEW_SUPERSEDED_BY_CORRECTIVE_PARITY_PORT_SPEC
```

The note must state:

- historical markdown comparisons are no longer the corrective baseline;
- the accepted baseline is the old-MBT parity-port benchmark;
- prior `bars_regression_run_1..3.json` artifacts are not old-vs-new parity
  evidence until paired with matching old parity-port reports.

### 8. Corrective result review after runs

Create only after validation and benchmark runs:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md
```

Required comparison method:

- load three old parity reports;
- load three new split reports;
- match rows by `label` and `row_count`;
- require full MBT semantic checksum equality before speed ratios;
- compute ratio:

```text
new_rows_per_second / old_parity_rows_per_second
new_mb_per_second / old_parity_mb_per_second
```

- report per-run ratios and spread;
- do not claim stability from one run;
- state any byte-output checksum mismatch as evidence, not failure, unless the
  implementation proves writer byte determinism first.

## Validation Commands

### New split repository checks

Run from:

```text
cd /home/tia/_DEV/MATHILDE/metamorphic-binary-transport
```

Formatting:

```text
cargo fmt --all --check
```

Forbidden-pattern scan:

```text
rg -n "unwrap\\(|expect\\(|panic!|todo!|unreachable!" crates/benches/src/bars_regression.rs crates/benches/src/bin/mbt_bars_regression_bench.rs crates/benches/src/tests/test_bars_regression_bench_output.rs
```

Codegen checks:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface projection --out crates/schemas/bars_core/src/bars_v1.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter json --out crates/schemas/bars_core/src/bars_v1_json.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter protobuf --out crates/schemas/bars_core/src/bars_v1_protobuf.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter csv --out crates/schemas/bars_core/src/bars_v1_csv.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter transponding --out crates/schemas/bars_core/src/bars_v1_transponding.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter arrow-ipc --out crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter parquet --out crates/schemas/bars_core/src/bars_v1_parquet.rs
```

Tests and compile checks:

```text
cargo test -p metamorphic_binary_transport_benches --lib
/usr/bin/time -v cargo check -p metamorphic_binary_transport_benches --all-targets
/usr/bin/time -v cargo check -p metamorphic_binary_transport_schema_bars --features json,protobuf,csv,arrow_ipc,parquet
/usr/bin/time -v cargo check -p metamorphic_binary_transport_core
cargo tree -p metamorphic_binary_transport_benches
```

### Old experiments repository checks

Run from:

```text
cd /home/tia/_DEV/MATHILDE/experiments
```

Formatting:

```text
rustfmt --check crates/mathilde-binary-transport/src/lib.rs crates/mathilde-binary-transport/src/benches/bars_regression_parity.rs crates/mathilde-binary-transport/src/benches/mod.rs crates/mathilde-binary-transport/src/main.rs crates/mathilde-binary-transport/src/tests/test_bars_regression_parity.rs crates/mathilde-binary-transport/src/tests/mod.rs
```

`cargo fmt --all --check` may be run as a diagnostic, but if it fails on
pre-existing unrelated formatting drift outside the bound files, the result
must be recorded and must not authorize reformatting unapproved files.

Forbidden-pattern scan:

```text
rg -n "unwrap\\(|expect\\(|panic!|todo!|unreachable!" crates/mathilde-binary-transport/src/lib.rs crates/mathilde-binary-transport/src/benches/bars_regression_parity.rs crates/mathilde-binary-transport/src/benches/mod.rs crates/mathilde-binary-transport/src/main.rs crates/mathilde-binary-transport/src/tests/test_bars_regression_parity.rs crates/mathilde-binary-transport/src/tests/mod.rs
```

Tests and compile checks:

```text
cargo test -p mathilde_binary_transport --features bars-regression-parity-only test_bars_regression_parity
/usr/bin/time -v cargo check -p mathilde_binary_transport --all-targets --features bars-regression-parity-only
```

The unfeatured old-crate check may be run as a diagnostic only. If it fails on
pre-existing non-parity code, the failure must be recorded and must not block
the feature-gated parity validation.

## Benchmark Commands

Run three old parity runs:

```text
cd /home/tia/_DEV/MATHILDE/experiments
cargo run --release -p mathilde_binary_transport --features bars-regression-parity-only -- bench-bars-regression-parity --report-dir crates/mathilde-binary-transport/docs/evidences/bars_regression_parity
cargo run --release -p mathilde_binary_transport --features bars-regression-parity-only -- bench-bars-regression-parity --report-dir crates/mathilde-binary-transport/docs/evidences/bars_regression_parity
cargo run --release -p mathilde_binary_transport --features bars-regression-parity-only -- bench-bars-regression-parity --report-dir crates/mathilde-binary-transport/docs/evidences/bars_regression_parity
```

Run three new split runs:

```text
cd /home/tia/_DEV/MATHILDE/metamorphic-binary-transport
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_bars_regression_bench -- --report-dir docs/evidence/mbt_bars_regression_benchmark
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_bars_regression_bench -- --report-dir docs/evidence/mbt_bars_regression_benchmark
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_bars_regression_bench -- --report-dir docs/evidence/mbt_bars_regression_benchmark
```

Companion projection run:

```text
cd /home/tia/_DEV/MATHILDE/metamorphic-binary-transport
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_projection_bench -- --report-dir docs/evidence/mbt_projection_direct_writer
```

## Expected Outputs

The implementation is accepted only if:

- no generated files change;
- no dependency package changes are made;
- no lockfile changes are made;
- `/home/tia/_DEV/MATHILDE/experiments/Cargo.lock` does not change;
- `/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/Cargo.toml`
  changes only by adding the empty `bars-regression-parity-only` feature;
- old `bars-regression-parity-only` builds compile only Bars generated modules
  and the parity benchmark/test modules;
- old non-featured module exports and command dispatches remain unchanged by
  inspection;
- old parity report contains all required labels and row counts;
- new split report contains all required labels and row counts;
- full MBT semantic checksum matches between old and new for every row count;
- old and new reports use the same row fixture semantics;
- prior historical markdown ratios are not used as corrective speed evidence;
- result review separates:
  - raw old parity run evidence;
  - raw new split run evidence;
  - old-vs-new ratios;
  - instability or residual limitations.

## Rollback Boundary

Rollback is limited to:

```text
crates/benches/src/bars_regression.rs
crates/benches/src/bin/mbt_bars_regression_bench.rs
crates/benches/src/tests/test_bars_regression_bench_output.rs
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_result_review.md
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/Cargo.toml
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/lib.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/bars_regression_parity.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/mod.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/main.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/tests/test_bars_regression_parity.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/tests/mod.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/evidences/bars_regression_parity/.gitkeep
```

No generated-file rollback is expected because generated files are not touched.

## Known Risks

1. Current new benchmark timing is single-pass. The old parity port must mirror
   that exactly. Stability claims require three runs and cannot be made from
   one run.
2. Arrow IPC and Parquet byte outputs may differ between old and new dependency
   surfaces. The result review must not require byte equality unless writer
   determinism is proved first.
3. Old parity code lives in the experiments repository, so validation commands
   must be run from both repository roots.
4. If the old parity benchmark needs dependency changes, this plan is invalid
   and work must return to spec/plan amendment.
5. If a feature gate misses one old non-Bars module, old release compilation
   can still include wide/primitives generated code and reproduce the blocked
   benchmark build time. The implementation must verify the feature-gated
   generated module surface by code read.
6. If the feature gate changes old non-featured behavior, the corrective
   comparison is invalid because the old crate has been modified outside the
   parity-only benchmark boundary.

## Required Next Step

Write an implementation-plan peer audit for this amended plan:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_implementation_plan_peer_audit_v3.md
```

No code may be changed before that audit passes and this plan is explicitly
approved for implementation.
