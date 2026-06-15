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

# SPEC: MBT Bars Regression Benchmark

## 1. Identification

Slug: `mbt_bars_regression_benchmark`

Primary repository:

```text
/home/tia/_DEV/MATHILDE/metamorphic-binary-transport
```

Old implementation repository used only for the parity-port benchmark:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport
```

Research brief:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_research_brief.md
```

Historical old benchmark evidence, retained as context only:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/bench_results.md
```

## 2. Status

Status: `DRAFT_CORRECTIVE_PARITY_PORT_FEATURE_AMENDMENT_AWAITING_PEER_AUDIT_V3`

This spec does not authorize code changes. Implementation may start only after:

1. this spec passes a separate peer audit;
2. the corrective implementation plan is amended to this spec;
3. the amended implementation plan passes its own peer audit;
4. the amended implementation plan is explicitly approved.

Corrective amendment:

The prior corrective path tried to reproduce the historical old archived timing
boundary in the new split workspace. That would require generated archived
adapter entrypoints or a different public API surface. That path is no longer
approved.

The accepted corrective path is to leave the new codegen and runtime API
unchanged, then add a benchmark to the old MBT crate that mirrors the current
new split benchmark semantics. The only valid old-vs-new performance
comparison for this pass is:

```text
old MBT implementation + new benchmark semantics
vs
new split MBT implementation + same benchmark semantics
```

Historical rows from `bench_results.md` must not be used as the speed baseline
for this corrective comparison.

Feature-gated old parity amendment:

The old experiments crate may add one empty benchmark-only feature to avoid
compiling old non-Bars generated modules and old non-parity benchmark/test
modules for the parity benchmark command:

```toml
bars-regression-parity-only = []
```

This feature is not a dependency feature and must not change old non-featured
crate behavior.

## 3. Purpose

Create an apple-to-apple Bars regression benchmark by measuring the old
implementation and the new split workspace under the same benchmark semantics.

The benchmark must answer:

- whether the new split workspace regressed against the old implementation
  when both use the same fixture, labels, output caps, trusted-access policy,
  timing boundary, and evidence format;
- whether the current generated trusted metamorphose paths are comparable
  without changing codegen;
- whether current generated metamorphose JSON is faster or slower than a
  current bench-only Rust DTO to serde JSON baseline for the same logical rows;
- whether existing projection direct-writer evidence remains separately owned
  by the projection benchmark.

## 4. Non-goals

This spec does not:

- change core, schema, codegen, projection, metamorphose, transponding, or
  adapter runtime behavior in the new split workspace;
- add generated archived adapter entrypoints;
- hand-edit generated files;
- reproduce historical old archived timing boundaries;
- compare against historical `bench_results.md` as a speed baseline for this
  corrective pass;
- add compression benchmarks;
- add wide-schema benchmarks;
- add storage, cache, MDB, MLDB, service, SDK, or network benchmarks;
- add direct prost DTO protobuf baseline generation in the new workspace;
- claim performance parity before at least three old-parity and three
  new-split release runs are recorded.

## 5. Measured object

The new split benchmark remains:

```text
deterministic current Bars rows
  -> BarsV1::encode
  -> BarsV1::inspect
```

```text
deterministic current Bars rows
  -> BarsV1::encode
  -> BarsV1::{metamorphose_json, metamorphose_protobuf, metamorphose_csv}
```

```text
deterministic current Bars rows
  -> BarsV1::encode
  -> unsafe BarsV1::{
       metamorphose_json_trusted_unchecked,
       metamorphose_protobuf_trusted_unchecked,
       metamorphose_csv_trusted_unchecked,
       metamorphose_arrow_ipc_trusted_unchecked,
       metamorphose_parquet_trusted_unchecked
     }
```

```text
deterministic current Bars rows
  -> bench-only Rust DTO view
  -> serde_json::to_vec
```

The old parity-port benchmark must measure the same logical lanes and timing
boundaries using the old implementation:

```text
deterministic current Bars rows ported to old row type
  -> old BarsV1 encode/access/metamorphose APIs
  -> same labels and same report fields
```

The existing projection benchmark remains the only measured object for
MBT-to-MBT projection:

```text
crates/benches/src/bin/mbt_projection_bench.rs
```

## 6. Schema source contract

The new split schema source is:

```text
/home/tia/_DEV/MATHILDE/metamorphic-binary-transport/crates/schemas/bars_core
```

The old parity-port schema source is:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/bars_v1.rs
```

Neither benchmark may define a new schema. Neither benchmark may hand-edit
generated schema files.

Required new generated modules:

```text
crates/schemas/bars_core/src/bars_v1.rs
crates/schemas/bars_core/src/bars_v1_json.rs
crates/schemas/bars_core/src/bars_v1_protobuf.rs
crates/schemas/bars_core/src/bars_v1_csv.rs
crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs
crates/schemas/bars_core/src/bars_v1_parquet.rs
crates/schemas/bars_core/src/bars_v1_transponding.rs
```

Required new schema features:

```text
json,protobuf,csv,arrow_ipc,parquet
```

The benchmark must not require the new schema `arrow` feature unless a later
spec adds an Arrow RecordBatch lane.

## 7. Wire and archive contract

The new full MBT lane output is:

```rust
BarsV1::encode(&rows, MAX_RESPONSE_BYTES)
```

The old parity full MBT lane output is the equivalent old generated Bars
encoder output for the same logical rows.

Metamorphose lanes consume the MBT byte vector produced for that implementation
and row count.

Checked lanes use checked public access.

Trusted lanes use trusted access only after bytes were produced by the same
schema encoder in the same benchmark setup and are not mutated before use.

The old parity port may use old-crate internal `pub(crate)` archived helpers
only to mirror the current new split trusted byte-boundary semantics. It must
not revive the historical old archived setup boundary as the comparison method.

Every report row must record:

- output byte length;
- response checksum for byte outputs;
- semantic checksum where `inspect` can provide it;
- old parity rows/sec and MB/sec when comparing a new run against an old parity
  run.

## 8. Checked and trusted access contract

Checked full MBT lane:

```text
encode rows
inspect encoded bytes
```

Checked metamorphose lanes:

```text
encode rows
checked metamorphose function
```

Trusted metamorphose lanes:

```text
encode rows
unsafe trusted access path for the same MBT bytes
format conversion
```

The benchmark must label checked and trusted lanes separately. It must not mix
checked and trusted times into one metric.

Trusted lanes are valid only for this benchmark because bytes are generated by
the schema encoder before the measured conversion and remain immutable.

## 9. Codegen contract

This benchmark does not change codegen.

Generated artifacts are inputs only. New split reproducibility validation must
include:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface projection --out crates/schemas/bars_core/src/bars_v1.rs

cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter json --out crates/schemas/bars_core/src/bars_v1_json.rs

cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter protobuf --out crates/schemas/bars_core/src/bars_v1_protobuf.rs

cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter csv --out crates/schemas/bars_core/src/bars_v1_csv.rs

cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter transponding --out crates/schemas/bars_core/src/bars_v1_transponding.rs

cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter arrow-ipc --out crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs

cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter parquet --out crates/schemas/bars_core/src/bars_v1_parquet.rs
```

The old parity port does not run or modify old codegen.

## 10. Crate boundary contract

New split benchmark code lives only in:

```text
/home/tia/_DEV/MATHILDE/metamorphic-binary-transport/crates/benches
```

Old parity-port code lives only in:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/Cargo.toml
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/lib.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/tests
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/main.rs
```

New production crates remain unchanged:

```text
crates/core
crates/codegen
crates/metamorphose
crates/transponding
crates/adapters/*
crates/schemas/*
```

The old parity port is an experiment benchmark surface only. It must not change
old runtime behavior or generated code.

Old parity-only feature contract:

```text
feature enabled:
  - old generated module surface exposes only bars_v1 and bars_v1_proto;
  - old benchmark module graph compiles only bars_regression_parity;
  - old test module graph compiles only test_bars_regression_parity;
  - old binary dispatch accepts only bench-bars-regression-parity.

feature disabled:
  - old generated module graph remains unchanged;
  - old benchmark module graph remains unchanged;
  - old test module graph remains unchanged;
  - old binary dispatch remains unchanged.
```

The feature must not edit:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/mod.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/*.rs
```

## 11. Dependency contract

No new dependency is approved by this corrective amendment.

Already approved new benchmark dependency surface remains limited to:

```text
/home/tia/_DEV/MATHILDE/metamorphic-binary-transport/crates/benches/Cargo.toml
```

The old parity port must use dependencies already present in:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/Cargo.toml
```

Only this old-crate `Cargo.toml` edit is approved:

```toml
bars-regression-parity-only = []
```

No dependency package may be added or changed. No lockfile may change.

These files must not change:

```text
/home/tia/_DEV/MATHILDE/metamorphic-binary-transport/Cargo.toml
/home/tia/_DEV/MATHILDE/metamorphic-binary-transport/Cargo.lock
/home/tia/_DEV/MATHILDE/experiments/Cargo.toml
/home/tia/_DEV/MATHILDE/experiments/Cargo.lock
```

Any dependency edit, workspace manifest edit, or lockfile edit outside the
approved empty old-crate feature is out of scope.

## 12. Determinism contract

Both old and new benchmarks must use the current new split Bars regression
fixture semantics.

Source fixture to mirror:

```text
/home/tia/_DEV/MATHILDE/metamorphic-binary-transport/crates/benches/src/projection.rs
```

Required fixture identity:

| Property | Required value |
|---|---|
| function semantics | current `projection::bars_rows(row_count)` |
| first close timestamp | `1_700_000_000_000` |
| time step | `60_000` ms |
| entity | BTCUSDT only |
| timeframe | `1m` |
| source | `frontier` |
| process | `derived` |
| presence | all Bars presence bits set through the generated allowed mask |
| row values | deterministic arithmetic values from current `projection::bars_rows` |

The old parity port must not use the old `generate_rows(row_count,
DEFAULT_SEED)` fixture for this corrective comparison.

Required row counts:

```text
1, 100, 500, 1_000, 10_000, 100_000
```

Required max response bytes:

```text
1_073_741_824
```

Each run must record:

- UTC timestamp;
- command;
- dirty state;
- rustc;
- OS/kernel;
- CPU;
- row counts;
- enabled schema features where applicable;
- fixture source identity;
- max response bytes;
- output report path.

## 13. Failure contract

The benchmark must fail if:

- `--report-dir` is missing;
- a report path would overwrite an existing run file;
- required labels are missing;
- required row counts are missing;
- a measured output exceeds `MAX_RESPONSE_BYTES`;
- any checked MBT inspect fails;
- any metamorphose function fails;
- a JSON artifact cannot be written;
- a row reports non-finite timing or throughput;
- output bytes are zero for a byte-output lane.

The benchmark must not silently skip a lane.

## 14. Compile-surface budget

The new benchmark is allowed to increase compile surface only for:

```text
metamorphic_binary_transport_benches
```

The old parity port is allowed to increase compile surface only for:

```text
mathilde_binary_transport
```

Required new compile checks:

```text
cargo check -p metamorphic_binary_transport_benches --all-targets
cargo check -p metamorphic_binary_transport_schema_bars --features json,protobuf,csv,arrow_ipc,parquet
cargo check -p metamorphic_binary_transport_core
cargo tree -p metamorphic_binary_transport_benches
```

Required old compile checks:

```text
cd /home/tia/_DEV/MATHILDE/experiments
cargo check -p mathilde_binary_transport --all-targets --features bars-regression-parity-only
```

The unfeatured old-crate check may be run as a diagnostic only. If it fails on
pre-existing non-parity code, the result review must record it separately and
must not treat it as parity benchmark evidence.

The result review must record elapsed time and max RSS if `/usr/bin/time` is
available.

## 15. Runtime performance budget

This pass is an old-vs-new regression check. It is not compared against the
historical old markdown table.

Required current/new labels and old parity labels are identical:

| Label | Measured surface |
|---|---|
| `bars_mbt_full_encode_inspect_checked` | encode plus inspect |
| `bars_metamorphose_json_checked` | checked JSON metamorphose |
| `bars_metamorphose_protobuf_checked` | checked protobuf metamorphose |
| `bars_metamorphose_csv_checked` | checked CSV metamorphose |
| `bars_metamorphose_json_trusted` | trusted JSON metamorphose |
| `bars_metamorphose_protobuf_trusted` | trusted protobuf metamorphose |
| `bars_metamorphose_csv_trusted` | trusted CSV metamorphose |
| `bars_metamorphose_arrow_ipc_trusted` | trusted Arrow IPC metamorphose |
| `bars_metamorphose_parquet_trusted` | trusted Parquet metamorphose |
| `bars_serde_json_baseline` | bench-only Rust DTO rows through `serde_json::to_vec` |

Required comparisons:

- every new label must compare against the same label from the old parity-port
  report for the same row count;
- `bars_metamorphose_json_checked` must also compare against
  `bars_serde_json_baseline` from the same new run;
- missing old parity labels or row counts are result-review blockers;
- at least three old parity runs and three new split runs are required before
  stability claims.

Throughput is computed from exactly the measured timing boundary in each report
row. If the current new benchmark remains single-pass, the old parity port must
also be single-pass. If a later approved plan changes the new benchmark to
accumulated iterations, the old parity port must change in the same plan.

## 16. Correctness oracle

For MBT full lanes:

```text
BarsV1::inspect(bytes).semantic_checksum
```

The old parity port and new split benchmark must report semantic checksum for
the full MBT lane. The result review must compare old-vs-new semantic checksum
for the same row count before speed comparison.

For JSON/protobuf/CSV/Arrow IPC/Parquet byte outputs:

```text
response checksum = fnv1a64(output bytes)
output length > 0
```

The result review must record response checksums for old and new outputs.
Byte-for-byte equality is required only when the implementation plan proves the
format writer is deterministic across the old and new dependency surfaces. If
that is not proved, semantic payload identity is established by the shared
fixture and full MBT semantic checksum, and the limitation must be stated.

For current serde JSON baseline:

```text
response checksum = fnv1a64(output bytes)
serde row count equals source row count
output length > 0
```

Projection correctness remains owned by:

```text
docs/evidence/mbt_projection_direct_writer/projection_run_*.json
```

Projection rows must have:

```text
semantic_checksum == minimal_projection_checksum
```

## 17. Benchmark methodology

New split benchmark command:

```text
cd /home/tia/_DEV/MATHILDE/metamorphic-binary-transport
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_bars_regression_bench -- --report-dir docs/evidence/mbt_bars_regression_benchmark
```

Old parity-port benchmark command:

```text
cd /home/tia/_DEV/MATHILDE/experiments
cargo run --release -p mathilde_binary_transport --features bars-regression-parity-only -- bench-bars-regression-parity --report-dir crates/mathilde-binary-transport/docs/evidences/bars_regression_parity
```

Required companion projection command:

```text
cd /home/tia/_DEV/MATHILDE/metamorphic-binary-transport
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_projection_bench -- --report-dir docs/evidence/mbt_projection_direct_writer
```

New output path pattern:

```text
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_N.json
```

Old parity output path pattern:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/evidences/bars_regression_parity/bars_regression_parity_run_N.json
```

The benchmark must measure setup outside the measured loop:

- row generation is outside measured lane timing;
- MBT encode is measured for the full MBT lane;
- checked metamorphose lanes time the checked public conversion from already
  encoded MBT bytes;
- trusted metamorphose lanes time the trusted conversion from already encoded
  MBT bytes;
- serde JSON baseline measures serde JSON output over prebuilt bench-only DTO
  rows.

Old parity timing boundary:

The old parity port must match the current new benchmark boundary for each
label. It must not use the old `bench.rs` archived setup timing boundary as the
comparison baseline.

Existing new artifacts produced before this corrective amendment:

```text
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_1.json
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_2.json
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_3.json
```

are valid only as evidence of the first benchmark implementation behavior. They
must not be used to claim old-vs-new parity until paired with an old parity-port
artifact that satisfies this methodology.

## 18. Test plan

Required new tests:

```text
crates/benches/src/tests/test_bars_regression_bench_output.rs
```

Required old parity tests:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/tests/test_bars_regression_parity.rs
```

Required old parity test command:

```text
cd /home/tia/_DEV/MATHILDE/experiments
cargo test -p mathilde_binary_transport --features bars-regression-parity-only test_bars_regression_parity
```

The tests must prove:

- old and new required label lists are identical;
- old and new row counts are identical;
- old parity fixture values match the current new `projection::bars_rows`
  contract for selected rows;
- report writers refuse overwrite;
- serde JSON baseline row conversion preserves row count and selected field
  values;
- benchmark JSON output contains command, environment, row count, label, bytes,
  checksum, timing, rows/sec, MB/sec, and optional comparison fields;
- no test depends on historical `bench_results.md` for speed claims.

Existing projection tests remain:

```text
crates/benches/src/tests/test_projection_bench_output.rs
```

## 19. Code bindings

Files to edit in the new split repository after implementation plan approval:

```text
crates/benches/src/bars_regression.rs
crates/benches/src/bin/mbt_bars_regression_bench.rs
crates/benches/src/tests/test_bars_regression_bench_output.rs
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_result_review.md
```

Files to create in the old experiments repository after implementation plan
approval:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/bars_regression_parity.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/tests/test_bars_regression_parity.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/evidences/bars_regression_parity/.gitkeep
```

Files to edit in the old experiments repository after implementation plan
approval:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/Cargo.toml
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/lib.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/mod.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/main.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/tests/mod.rs
```

Files to create in the new split repository after corrective runs:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md
```

No production crate source file may be edited in the new split repository.
No generated file may be edited in either repository.

## 20. Generated artifact bindings

This benchmark does not create generated schema artifacts.

Generated files consumed by the new benchmark:

```text
crates/schemas/bars_core/src/bars_v1.rs
crates/schemas/bars_core/src/bars_v1_json.rs
crates/schemas/bars_core/src/bars_v1_protobuf.rs
crates/schemas/bars_core/src/bars_v1_csv.rs
crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs
crates/schemas/bars_core/src/bars_v1_parquet.rs
crates/schemas/bars_core/src/bars_v1_transponding.rs
```

Generated files consumed by the old parity port:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/bars_v1.rs
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/bars_v1_proto.rs
```

Generated files remain owned by their existing codegen commands. This spec
does not authorize generated-file writes.

The old parity-only feature may replace the old generated module root only at
the old `src/lib.rs` module boundary when the feature is enabled. It must not
modify the old generated module file itself.

## 21. Review artifact bindings

Required artifacts:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_research_brief.md
docs/specs/mbt_bars_regression_benchmark_SPEC.md
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_peer_audit.md
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_implementation_plan.md
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_result_review.md
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_peer_audit.md
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_peer_audit_v2.md
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_peer_audit_v3.md
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_implementation_plan.md
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md
```

Evidence artifacts:

```text
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_N.json
docs/evidence/mbt_projection_direct_writer/projection_run_N.json
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/evidences/bars_regression_parity/bars_regression_parity_run_N.json
```

## 22. Implementation plan requirement

The corrective implementation plan must be amended to bind:

- exact old parity-port fixture mapping from current new `projection::bars_rows`;
- exact old parity labels;
- exact old parity timing boundaries matching the current new benchmark;
- exact old parity report fields;
- exact old parity-only feature contract;
- exact old feature-gated parity command and output path;
- exact old feature-gated validation commands;
- exact old and new comparison method;
- validation commands for both repositories;
- three old parity runs and three new split runs;
- result review update.

The prior corrective implementation plan status `BLOCKED_BEFORE_CODE` is
superseded by this spec once an amended plan is written and audited.

No code may be changed until the amended implementation plan is approved.

## 23. Approval checklist

Pre-audit closure checklist:

- Mandatory section order matches `docs/protocols/spec_protocol.md`.
- Prior approved specs searched:
  - `mbt_projection_direct_writer_SPEC.md` remains projection owner;
  - `mbt_metamorphose_migration_SPEC.md` remains adapter architecture owner.
- Command surfaces are exact for new and old benchmark commands.
- Old parity benchmark command uses `--features bars-regression-parity-only`.
- Generated artifacts have one owner.
- No generated artifact is bound to two incompatible command surfaces.
- Old `src/generated/mod.rs` and all old generated files remain forbidden to
  edit.
- Old `Cargo.lock` remains forbidden to edit.
- Old `Cargo.toml` edit is limited to the empty `bars-regression-parity-only`
  feature.
- Runtime dispatch paths to benchmark are listed.
- Existing projection tests are preserved.
- No design decision is deferred to the implementation plan.
- Exact benchmark labels are bound in section 15.
- Compile-surface evidence commands are defined for both repositories.
- Corrective amendment removes generated archived entrypoint work from scope.
- Corrective amendment binds the old-MBT benchmark parity port.
- Corrective amendment states historical `bench_results.md` is context only for
  this pass.
- Existing single-pass artifacts are marked non-authoritative until paired with
  matching old parity-port artifacts.

Implementation approval checklist:

- peer audit passed;
- amended implementation plan peer audit passed;
- amended implementation plan approved;
- new benchmark code remains only in `crates/benches`;
- old parity changes remain only in old `Cargo.toml`, old `src/lib.rs`, old
  `src/benches`, old `src/tests`, and old `src/main.rs` dispatch;
- old `Cargo.toml` changes only by adding the empty
  `bars-regression-parity-only` feature;
- old feature-gated validation and benchmark commands use
  `--features bars-regression-parity-only`;
- no generated files are edited;
- no dependency package changes or lockfile changes are introduced.

## 24. Open questions

None for the follow-up peer audit.
