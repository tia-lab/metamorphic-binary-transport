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
Repository:

```text
/home/tia/_DEV/MATHILDE/metamorphic-binary-transport
```

Research brief:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_research_brief.md
```

Old benchmark baseline:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/bench_results.md
```

## 2. Status

Status: `DRAFT_CORRECTIVE_AMENDED_AWAITING_PEER_AUDIT`

This spec does not authorize code changes. Implementation may start only after:

1. this spec passes a separate peer audit;
2. an implementation plan binds exact edits and validation commands;
3. the implementation plan is explicitly approved.

Corrective amendment:

The first implementation used the current projection benchmark Bars fixture and
single-pass timing. That does not prove old-baseline parity. This amended spec
supersedes the prior benchmark method for old-baseline comparisons and requires
exact old fixture parity, old iteration-count parity, and old timing-boundary
parity before any performance ratio against the old crate can be accepted.

## 3. Purpose

Create the minimal dedicated benchmark surface needed to compare the new split
MBT workspace against the old tracked Bars benchmark evidence.

The benchmark must answer:

- whether full Bars MBT encode plus checked access/inspect stays close to the
  old tracked `mathilde_binary_generated` lane;
- whether generated metamorphose JSON, protobuf, CSV, Arrow IPC, and Parquet
  stay close to the old tracked metamorphose lanes;
- whether current generated metamorphose JSON is faster or slower than a
  current bench-only Rust DTO to serde JSON baseline for the same logical rows;
- whether existing projection direct-writer evidence remains valid and included
  in the final result view.

## 4. Non-goals

This spec does not:

- add benchmark code to production crates;
- add benchmark code to schema crates;
- change core, schema, codegen, projection, metamorphose, transponding, or
  adapter runtime behavior;
- duplicate the existing projection benchmark;
- add compression benchmarks;
- add wide-schema benchmarks;
- add storage, cache, MDB, MLDB, service, SDK, or network benchmarks;
- add direct prost DTO protobuf baseline generation in the new workspace;
- claim performance parity before at least three release runs are recorded.

## 5. Measured object

The new benchmark binary measures only Bars full-schema runtime surfaces:

```text
deterministic Bars rows
  -> BarsV1::encode
  -> BarsV1::inspect
```

```text
deterministic Bars rows
  -> BarsV1::encode
  -> BarsV1::{metamorphose_json, metamorphose_protobuf, metamorphose_csv}
```

```text
deterministic Bars rows
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
deterministic Bars rows
  -> bench-only Rust DTO view
  -> serde_json::to_vec
```

The existing projection benchmark remains the measured object for MBT-to-MBT
projection:

```text
crates/benches/src/bin/mbt_projection_bench.rs
```

## 6. Schema source contract

The schema source is the existing generated Bars schema crate:

```text
crates/schemas/bars_core
```

The benchmark must not define a new schema and must not hand-edit generated
schema files.

Required generated modules:

```text
crates/schemas/bars_core/src/bars_v1.rs
crates/schemas/bars_core/src/bars_v1_json.rs
crates/schemas/bars_core/src/bars_v1_protobuf.rs
crates/schemas/bars_core/src/bars_v1_csv.rs
crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs
crates/schemas/bars_core/src/bars_v1_parquet.rs
crates/schemas/bars_core/src/bars_v1_transponding.rs
```

The benchmark must compile the Bars schema crate with these features:

```text
json,protobuf,csv,arrow_ipc,parquet
```

The benchmark must not require the `arrow` feature unless a later spec adds an
Arrow RecordBatch lane.

## 7. Wire and archive contract

The full MBT lane output is the encoded MBT byte vector produced by:

```rust
BarsV1::encode(&rows, MAX_RESPONSE_BYTES)
```

Metamorphose lanes consume the same MBT byte vector.

Checked lanes use checked public access.

Trusted lanes may use generated unsafe trusted calls only after the same
benchmark iteration has produced the MBT bytes through `BarsV1::encode`; encoded
bytes are trusted because they were produced by the schema encoder in the same
process and are not mutated before use.

The benchmark must record:

- output byte length;
- response checksum for byte outputs;
- semantic checksum where `BarsV1::inspect` can provide it;
- old baseline rows/sec and MB/sec where available.

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
unsafe trusted metamorphose function
```

The benchmark must label checked and trusted lanes separately. It must not mix
checked and trusted times into one metric.

## 9. Codegen contract

This benchmark does not change codegen.

Generated artifacts are inputs only. Reproducibility validation must include:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface projection --out crates/schemas/bars_core/src/bars_v1.rs

cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter json --out crates/schemas/bars_core/src/bars_v1_json.rs

cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter protobuf --out crates/schemas/bars_core/src/bars_v1_protobuf.rs

cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter csv --out crates/schemas/bars_core/src/bars_v1_csv.rs

cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter transponding --out crates/schemas/bars_core/src/bars_v1_transponding.rs

cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter arrow-ipc --out crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs

cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter parquet --out crates/schemas/bars_core/src/bars_v1_parquet.rs
```

## 10. Crate boundary contract

Benchmark code lives only in:

```text
crates/benches
```

Production crates remain unchanged:

```text
crates/core
crates/codegen
crates/metamorphose
crates/transponding
crates/adapters/*
crates/schemas/*
```

The benchmark crate is `publish = false` and is not part of downstream runtime
dependency surfaces.

## 11. Dependency contract

Allowed hand-edited dependency file:

```text
crates/benches/Cargo.toml
```

Allowed Cargo-generated dependency artifact:

```text
Cargo.lock
```

`Cargo.lock` may change only as Cargo's dependency-resolution effect from the
approved `crates/benches/Cargo.toml` edits. Manual edits to `Cargo.lock` are
not allowed. The result review must record the lockfile/dependency graph
validation outcome.

The existing Bars schema dependency must be feature-activated for the measured
adapter surfaces:

```toml
# crates/benches/Cargo.toml only
metamorphic_binary_transport_schema_bars = {
    path = "../schemas/bars_core",
    features = ["json", "protobuf", "csv", "arrow_ipc", "parquet"]
}
```

The benchmark crate may add bench-only serde dependencies:

```toml
# crates/benches/Cargo.toml only
serde = { version = "=1.0.228", features = ["derive"] }
serde_json = "=1.0.145"
```

Reason:

- the Bars schema feature activation is required to compile the generated
  metamorphose functions under measurement;
- needed only to measure the current normal Rust DTO to JSON baseline;
- all edits are isolated to the benchmark crate dependency surface;
- does not enter MBT core, generated schemas, adapters, metamorphose, or
  transponding.

No other dependency change is allowed by this spec.

## 12. Determinism contract

Rows must be generated by a bench-only port of the old Bars fixture:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/src/benches/fixtures.rs
```

Required fixture identity:

| Property | Required value |
|---|---|
| function semantics | old `generate_rows(row_count, DEFAULT_SEED)` |
| seed | `0x4d415448494c4445` |
| first close timestamp | `1_546_300_860_000` |
| RNG | old `Lcg` transition with multiplier `6364136223846793005` and increment `1442695040888963407` |
| entity ordering | `idx % PAIR_COUNT` exactly as old fixture |
| time ordering | `FIRST_CLOSE_MS + (idx / PAIR_COUNT) * BAR_MS` exactly as old fixture |
| optional presence | same base presence bits and same `idx % 3`, `7`, `11`, `13`, `17` branches as old fixture |
| validation | the port must run the equivalent generated Bars row validation before returning rows |

The existing projection fixture:

```rust
metamorphic_binary_transport_benches::projection::bars_rows(row_count)
```

must not be used for old-baseline regression comparisons. It may remain owned
by projection benchmarks only.

Required row counts:

```text
1, 100, 500, 1_000, 10_000, 100_000
```

Required old iteration counts:

| Label | Rows | Iterations |
|---|---:|---:|
| `one` | 1 | 50 |
| `small` | 100 | 50 |
| `page_500` | 500 | 50 |
| `page_1000` | 1,000 | 50 |
| `medium` | 10,000 | 10 |
| `large` | 100,000 | 3 |

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
- enabled schema features;
- old baseline file path.

Each run must also record:

- fixture source identity;
- seed;
- row label;
- iteration count;
- accumulated output bytes across iterations;
- max single-iteration output bytes;
- accumulated total milliseconds across iterations.

## 13. Failure contract

The benchmark must fail if:

- `--report-dir` is missing;
- a report path would overwrite an existing run file;
- required old baseline labels are missing for required row counts;
- a measured output exceeds `MAX_RESPONSE_BYTES`;
- any checked MBT inspect fails;
- any metamorphose function fails;
- checksum equality checks fail for full logical payload lanes where checksums
  are defined;
- a JSON artifact cannot be written.

The benchmark must not silently skip a lane.

## 14. Compile-surface budget

The new benchmark is allowed to increase compile surface only for
`metamorphic_binary_transport_benches`.

Required compile checks:

```text
cargo check -p metamorphic_binary_transport_benches --all-targets
cargo check -p metamorphic_binary_transport_schema_bars --features json,protobuf,csv,arrow_ipc,parquet
cargo check -p metamorphic_binary_transport_core
cargo tree -p metamorphic_binary_transport_benches
```

The result review must record elapsed time and max RSS if `/usr/bin/time` is
available.

## 15. Runtime performance budget

The first budget is observational, not a pass/fail production gate.

Required current benchmark labels:

| Current label | Measured surface | Old baseline label |
|---|---|---|
| `bars_mbt_full_encode_inspect_checked` | `BarsV1::encode` plus `BarsV1::inspect` | `mathilde_binary_generated` |
| `bars_metamorphose_json_checked` | `BarsV1::metamorphose_json` | `metamorphose_json` |
| `bars_metamorphose_protobuf_checked` | `BarsV1::metamorphose_protobuf` | `metamorphose_protobuf` |
| `bars_metamorphose_csv_checked` | `BarsV1::metamorphose_csv` | none; current checked CSV evidence only |
| `bars_metamorphose_json_trusted` | `BarsV1::metamorphose_json_trusted_unchecked` | none; current trusted JSON evidence only |
| `bars_metamorphose_protobuf_trusted` | `BarsV1::metamorphose_protobuf_trusted_unchecked` | none; current trusted protobuf evidence only |
| `bars_metamorphose_csv_trusted` | `BarsV1::metamorphose_csv_trusted_unchecked` | `metamorphose_csv_full_archived` |
| `bars_metamorphose_arrow_ipc_trusted` | `BarsV1::metamorphose_arrow_ipc_trusted_unchecked` | `metamorphose_arrow_ipc_full_archived` |
| `bars_metamorphose_parquet_trusted` | `BarsV1::metamorphose_parquet_trusted_unchecked` | `metamorphose_parquet_full_archived` |
| `bars_serde_json_baseline` | bench-only Rust DTO rows through `serde_json::to_vec` | none; current baseline only |

Required old baseline labels:

```text
mathilde_binary_generated
metamorphose_json
metamorphose_protobuf
metamorphose_csv_full_archived
metamorphose_arrow_ipc_full_archived
metamorphose_parquet_full_archived
```

Required comparisons:

- every current label with an old baseline label must compare 100k rows against
  that old label;
- `bars_metamorphose_json_checked` must compare 100k rows against
  `bars_serde_json_baseline`;
- missing old labels for the required row counts are benchmark failures;
- CSV is mandatory because the old baseline contains
  `metamorphose_csv_full_archived`.

The result review must not claim parity from a single run. At least three runs
are required before stability claims.

## 16. Correctness oracle

For MBT full:

```text
BarsV1::inspect(bytes).semantic_checksum
```

The semantic checksum for `bars_mbt_full_encode_inspect_checked` must match the
old `mathilde_binary_generated` semantic checksum for each row count parsed
from:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/bench_results.md
```

If the old semantic checksum cannot be parsed for a row count, old-baseline
performance comparison for that row count is blocked.

For JSON/protobuf/CSV/Arrow IPC/Parquet byte outputs:

```text
response checksum = fnv1a64(output bytes)
output length > 0
```

For current serde JSON baseline:

```text
response checksum = fnv1a64(output bytes)
serde row count equals source row count
output length > 0
```

Cross-format semantic equality is limited in this pass because the new split
workspace does not implement direct JSON/protobuf decode baselines. The result
review must state this limitation.

Projection correctness remains owned by:

```text
docs/evidence/mbt_projection_direct_writer/projection_run_*.json
```

Projection rows must have:

```text
semantic_checksum == minimal_projection_checksum
```

## 17. Benchmark methodology

The new benchmark command:

```text
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_bars_regression_bench -- --report-dir docs/evidence/mbt_bars_regression_benchmark
```

Required companion projection command:

```text
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_projection_bench -- --report-dir docs/evidence/mbt_projection_direct_writer
```

The new benchmark output path pattern:

```text
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_N.json
```

The benchmark must measure setup outside the measured loop:

- row generation is outside measured lane timing;
- MBT encode is measured for full MBT lane;
- checked metamorphose lanes measure the same setup and timed boundary as the
  corresponding old tracked lane where an old lane exists;
- trusted archived metamorphose lanes measure conversion over MBT bytes after
  encode and checked archive access are complete, matching the old archived
  timing boundary for Arrow IPC, Parquet, and CSV archived comparisons;
- serde JSON baseline measures serde JSON output over prebuilt bench-only DTO
  rows.

Old-baseline timing parity requirements:

| Current label | Old label | Required measured boundary |
|---|---|---|
| `bars_mbt_full_encode_inspect_checked` | `mathilde_binary_generated` | measure encode, inspect, semantic/minimal checksum extraction, and response checksum inside each timed iteration, matching old `run_iteration` total timing |
| `bars_metamorphose_json_checked` | `metamorphose_json` | measure the current checked public JSON metamorphose path with the old fixture and old iteration count |
| `bars_metamorphose_protobuf_checked` | `metamorphose_protobuf` | measure the current checked public protobuf metamorphose path with the old fixture and old iteration count |
| `bars_metamorphose_csv_trusted` | `metamorphose_csv_full_archived` | measure archived/trusted CSV conversion after encode/access setup, using the old fixture and old iteration count |
| `bars_metamorphose_arrow_ipc_trusted` | `metamorphose_arrow_ipc_full_archived` | measure archived/trusted Arrow IPC conversion after encode/access setup, using the old fixture and old iteration count |
| `bars_metamorphose_parquet_trusted` | `metamorphose_parquet_full_archived` | measure archived/trusted Parquet conversion after encode/access setup, using the old fixture and old iteration count |

For every old-baseline comparison, throughput must be computed as:

```text
rows_per_second = (row_count * iterations) / accumulated_total_seconds
mb_per_second = accumulated_output_bytes / 1024 / 1024 / accumulated_total_seconds
```

Single-pass timings must not be compared against old multi-iteration baselines.

Existing artifacts produced before this corrective amendment:

```text
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_1.json
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_2.json
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_3.json
```

are valid only as evidence of the first benchmark implementation behavior. They
must not be used to claim old-baseline parity or regression until replaced by
corrective runs that satisfy this methodology.

## 18. Test plan

Required tests:

```text
crates/benches/src/tests/test_bars_regression_bench_output.rs
```

The tests must prove:

- required old baseline parser finds all required labels;
- required old baseline parser finds `mathilde_binary_generated` semantic
  checksums for all required row counts;
- report writer refuses overwrite;
- required labels list includes only the approved benchmark lanes;
- the old fixture port produces deterministic rows and matches the old
  `mathilde_binary_generated` semantic checksum for each required row count;
- serde JSON baseline row conversion preserves row count and selected field
  values for the old fixture rows;
- benchmark JSON output contains command, environment, row count, label, bytes,
  checksum, iterations, accumulated timing, rows/sec, MB/sec, and optional old
  comparison.

Existing projection tests remain:

```text
crates/benches/src/tests/test_projection_bench_output.rs
```

## 19. Code bindings

Files to edit after implementation plan approval:

```text
crates/benches/Cargo.toml
crates/benches/src/lib.rs
crates/benches/src/bars_regression.rs
crates/benches/src/bin/mbt_bars_regression_bench.rs
crates/benches/src/tests/test_bars_regression_bench_output.rs
crates/benches/src/tests/mod.rs
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_result_review.md
```

Cargo-generated artifact that may change after implementation plan approval:

```text
Cargo.lock
```

`Cargo.lock` may change only through Cargo resolution of the approved
`crates/benches/Cargo.toml` dependency edits.

Files to create after implementation plan approval:

```text
docs/evidence/mbt_bars_regression_benchmark/.gitkeep
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md
```

No other code files are approved by this spec.

## 20. Generated artifact bindings

This benchmark does not create generated schema artifacts.

Generated files consumed by the benchmark:

```text
crates/schemas/bars_core/src/bars_v1.rs
crates/schemas/bars_core/src/bars_v1_json.rs
crates/schemas/bars_core/src/bars_v1_protobuf.rs
crates/schemas/bars_core/src/bars_v1_csv.rs
crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs
crates/schemas/bars_core/src/bars_v1_parquet.rs
crates/schemas/bars_core/src/bars_v1_transponding.rs
```

Generated files remain owned by `mbt_codegen --check/--write` commands listed
in section 9.

## 21. Review artifact bindings

Required artifacts:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_research_brief.md
docs/specs/mbt_bars_regression_benchmark_SPEC.md
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_peer_audit.md
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_implementation_plan.md
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_result_review.md
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_peer_audit.md
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_implementation_plan.md
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md
```

Evidence artifacts:

```text
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_N.json
docs/evidence/mbt_projection_direct_writer/projection_run_N.json
```

## 22. Implementation plan requirement

The implementation plan must bind:

- exact JSON report fields;
- exact old fixture port fields and constants;
- exact row-count iteration table;
- exact old-baseline semantic checksum parsing;
- exact timing boundaries for every old-baseline comparison lane;
- dependency edits;
- Cargo-generated lockfile behavior;
- validation commands;
- three-run benchmark command sequence;
- result review update.

The existing implementation plan is superseded for old-baseline parity claims.
A corrective implementation plan must be written at:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_implementation_plan.md
```

and approved before any code change to the benchmark.

No code may be changed until the implementation plan is approved.

## 23. Approval checklist

Pre-audit closure checklist:

- Mandatory section order matches `docs/protocols/spec_protocol.md`.
- Prior approved specs searched:
  - `mbt_projection_direct_writer_SPEC.md` remains projection owner;
  - `mbt_metamorphose_migration_SPEC.md` remains adapter architecture owner.
- Command surfaces are exact.
- Generated artifacts have one owner.
- No generated artifact is bound to two incompatible command surfaces.
- Runtime dispatch paths to benchmark are listed.
- Existing projection tests are preserved.
- No design decision is deferred to the implementation plan.
- Exact benchmark labels and old baseline labels are bound in section 15.
- Cargo-generated lockfile behavior is bound in sections 11 and 19.
- Compile-surface evidence commands are defined.
- Corrective amendment binds exact old fixture parity.
- Corrective amendment binds old iteration-count parity.
- Corrective amendment binds timing-boundary parity.
- Existing single-pass `projection::bars_rows` benchmark artifacts are marked
  non-authoritative for old-baseline parity claims.

Implementation approval checklist:

- peer audit passed;
- implementation plan approved;
- benchmark code remains only in `crates/benches`;
- Bars adapter feature activation remains only on the `crates/benches`
  dependency edge;
- serde dependencies remain only in `crates/benches`;
- `Cargo.lock`, if changed, changes only through Cargo resolution of the
  approved benchmark dependency edits;
- no production crate code changes are introduced.

## 24. Open questions

None for the follow-up peer audit.
