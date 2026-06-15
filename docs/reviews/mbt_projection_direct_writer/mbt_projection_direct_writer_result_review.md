# MBT Projection Direct Writer Result Review

Status: completed with benchmark-scope caveat  
Date: 2026-06-15

## Scope

Implemented the approved direct MBT-to-MBT projection writer for generated
projection schemas. The implemented path serializes projected rows from archived
source rows through generated borrowed rkyv wrapper types. It does not construct
an owned projected row `Vec` and does not call the projected schema
`encode_owned` path.

## Code Evidence

Direct writer code is generated from:

- `crates/codegen/src/rust_emit.rs`

Generated schema evidence:

- `crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs`
- `crates/schemas/bars_core/src/bars_v1.rs`

New runtime benchmark support:

- `crates/benches/src/projection.rs`
- `crates/benches/src/bin/mbt_projection_bench.rs`
- `crates/benches/src/tests/test_projection_bench_output.rs`

The generated direct helper bodies were checked by tests to reject:

- `Vec::with_capacity(archived.`
- `.to_string()`
- `.to_vec()`
- `.collect()`
- `rows.push(`
- `encode_owned(rows`

The final generated direct row iterator also delegates `size_hint` to the source
archived row iterator.

## Validation Commands

All commands exited successfully.

```bash
cargo test -p metamorphic_binary_transport_codegen --all-targets
cargo test -p metamorphic_binary_transport_schema_test_compatibility --all-targets
cargo test -p metamorphic_binary_transport_schema_bars --all-targets
cargo test -p metamorphic_binary_transport_benches --all-targets
```

Observed test counts:

- codegen: 21 passed
- test compatibility schema: 17 passed across integration tests
- Bars schema: 5 passed across integration tests
- benches crate: 4 passed

Generated source reproducibility:

```bash
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface projection --out crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs

cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface projection --out crates/schemas/bars_core/src/bars_v1.rs
```

Both checks passed.

## Correctness Evidence

Artifacts:

- `docs/evidence/mbt_projection_direct_writer/test_compatibility_projection_inspect.txt`
- `docs/evidence/mbt_projection_direct_writer/bars_projection_inspect.txt`
- `crates/schemas/bars_core/tests/expected_bars_projection_inspect.txt`

Benchmark correctness oracle:

```bash
jq -s '[.[].rows[]] | length' docs/evidence/mbt_projection_direct_writer/projection_run_1.json docs/evidence/mbt_projection_direct_writer/projection_run_2.json docs/evidence/mbt_projection_direct_writer/projection_run_3.json
```

Observed: `216`

```bash
jq -s '[.[].rows[] | select(.semantic_checksum != null and .minimal_projection_checksum != null and .semantic_checksum != .minimal_projection_checksum)] | length' docs/evidence/mbt_projection_direct_writer/projection_run_1.json docs/evidence/mbt_projection_direct_writer/projection_run_2.json docs/evidence/mbt_projection_direct_writer/projection_run_3.json
```

Observed: `0`

This proves the benchmarked projected outputs matched their deterministic
semantic/minimal projection checksum oracle.

## Compile Evidence

Compile surface evidence is recorded in:

- `docs/evidence/mbt_projection_direct_writer/compile_surface.md`
- `docs/evidence/mbt_projection_direct_writer/dependency_trees.md`

Final post-size-hint checkpoint:

| Package | Command | Elapsed | Max RSS |
|---|---:|---:|---:|
| core | `cargo check -p metamorphic_binary_transport_core` | 0.08s | 28,528 KB |
| codegen | `cargo check -p metamorphic_binary_transport_codegen --all-targets` | 0.30s | 129,876 KB |
| test compatibility schema | `cargo check -p metamorphic_binary_transport_schema_test_compatibility --all-targets` | 1.00s | 171,908 KB |
| Bars schema | `cargo check -p metamorphic_binary_transport_schema_bars --all-targets` | 0.47s | 162,200 KB |
| benches | `cargo check -p metamorphic_binary_transport_benches --all-targets` | 1.12s | 110,536 KB |

Generated source line counts:

| File | Lines |
|---|---:|
| `crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs` | 2,400 |
| `crates/schemas/bars_core/src/bars_v1.rs` | 2,038 |

Dependency containment check:

- schema crates depend on `metamorphic_binary_transport_core` and `rkyv`;
- bench crate depends on core plus the two generated schema crates;
- no Arrow, Parquet, CSV, JSON adapter, or protobuf adapter dependency is pulled
  into either generated schema crate.

## Performance Evidence

Final benchmark artifacts:

- `docs/evidence/mbt_projection_direct_writer/projection_run_1.json`
- `docs/evidence/mbt_projection_direct_writer/projection_run_2.json`
- `docs/evidence/mbt_projection_direct_writer/projection_run_3.json`
- `docs/evidence/mbt_projection_direct_writer/projection_summary.md`
- `docs/evidence/mbt_projection_direct_writer/benchmark_environment.md`

The previous pre-size-hint probe was preserved under:

- `docs/evidence/mbt_projection_direct_writer/pre_size_hint_probe/`

Benchmark environment evidence:

| Field | Value |
|---|---|
| operator | `tia` |
| git commit | `d035f90bc3a6b312293dc7bba8096e78487ab83b` |
| worktree | dirty implementation/evidence branch |
| OS/kernel | Linux `5.15.0-156-generic` |
| CPU | Intel Xeon W-2295, 18 cores, 36 logical CPUs |
| RAM | 503 GiB total |
| rustc | `1.90.0` |
| cargo | `1.90.0` |
| benchmark profile | `release` |
| command | `cargo run --release -p metamorphic_binary_transport_benches --bin mbt_projection_bench -- --report-dir docs/evidence/mbt_projection_direct_writer` |

The benchmark JSON files do not embed this metadata. The companion environment
file records it after the final run artifacts were written. This is weaker than
embedding metadata directly in each benchmark JSON file, but it closes the
current review evidence gap without changing benchmark code.

### Median Performance

The spec requires median comparison across the three final runs. The table below
uses median rows/sec and median MB/sec. `Spread` is max/min rows/sec minus one
across the three runs. A lane is stable only when spread is at or below 7
percent.

| Lane | Rows | Median rows/s | Median MB/s | Spread | Median vs current owned | Median vs old crate | Stable |
|---|---:|---:|---:|---:|---:|---:|---|
| no metadata archived | 10,000 | 1,843,410 | 267 | 6.00% | 1.021 | 2.636 | yes |
| no metadata archived | 100,000 | 1,766,430 | 256 | 2.20% | 1.062 | 2.788 | yes |
| no metadata public | 10,000 | 1,525,830 | 221 | 2.34% | 1.073 | 2.163 | yes |
| no metadata public | 100,000 | 1,558,628 | 225 | 56.83% | 1.082 | 2.470 | no |
| ohlcv archived | 10,000 | 2,473,812 | 151 | 3.33% | 1.081 | 2.972 | yes |
| ohlcv archived | 100,000 | 2,273,654 | 138 | 1.08% | 1.059 | 2.852 | yes |
| ohlcv public | 10,000 | 2,382,134 | 145 | 5.95% | 1.003 | 2.860 | yes |
| ohlcv public | 100,000 | 2,282,179 | 139 | 1.86% | 1.063 | 2.860 | yes |

Interpretation:

- Seven of eight large Bars public/archived lanes are stable and median-faster
  than both current owned-row projection and the old crate baseline.
- `no_metadata public` at 100,000 rows is median-faster, but it is unstable
  under the spec's 7 percent max/min rule. It cannot be used for a stable speed
  claim without rerun or root-cause explanation.
- Large archived lanes are stable and median-faster than both baselines.

### Unstable Lanes

The following Bars public/archived lanes exceed the 7 percent spread rule:

| Lane | Rows | Spread | Median vs current owned | Median vs old crate |
|---|---:|---:|---:|---:|
| no metadata archived | 1 | 198.86% | 2.909 | 0.952 |
| no metadata archived | 100 | 216.73% | 2.623 | 1.674 |
| no metadata archived | 500 | 126.63% | 2.199 | 2.040 |
| no metadata archived | 1,000 | 204.82% | 1.145 | 2.342 |
| no metadata public | 1 | 139.42% | 2.401 | 0.527 |
| no metadata public | 100 | 216.76% | 3.215 | 1.951 |
| no metadata public | 500 | 125.80% | 2.381 | 2.067 |
| no metadata public | 1,000 | 256.79% | 0.941 | 2.051 |
| no metadata public | 100,000 | 56.83% | 1.082 | 2.470 |
| ohlcv archived | 1 | 211.36% | 3.394 | 1.186 |
| ohlcv archived | 100 | 224.58% | 3.237 | 2.021 |
| ohlcv archived | 500 | 100.02% | 1.405 | 2.018 |
| ohlcv archived | 1,000 | 142.83% | 1.108 | 2.194 |
| ohlcv public | 1 | 153.21% | 3.293 | 0.733 |
| ohlcv public | 100 | 217.33% | 3.248 | 2.034 |
| ohlcv public | 500 | 100.16% | 2.168 | 2.069 |
| ohlcv public | 1,000 | 142.66% | 1.014 | 2.129 |

No stable speed claim is made for the unstable lanes. For small row counts this
is consistent with benchmark overhead dominating the measured object. For
`no_metadata public` at 100,000 rows, the instability is real evidence and must
not be hidden behind averages.

### Benchmark Acceptance

Accepted:

- correctness: accepted;
- generated source reproducibility: accepted;
- dependency containment: accepted;
- compile surface: accepted for this schema size;
- large archived projection speed: accepted.

Not accepted:

- universal speed claim across all row counts;
- stable speed claim for small-row lanes;
- stable speed claim for `no_metadata public` at 100,000 rows;
- current-owned comparison for small lanes, because the current-owned baseline
  is one pre-rewrite run and the measured lanes are unstable.

The result does not trigger the spec rejection rule because archived direct
projection is not repeatedly slower than both baselines by more than 3.5
percent. The result is narrower than a blanket performance approval.

## Conclusion

Correctness is proved for generated Bars projections and the all-field
compatibility schema. The direct projection writer removes the owned projected
row vector from the generated projection path and preserves schema-specific
generation.

Large-row archived Bars projection is accepted: it is stable and median-faster
than both the old accepted crate and the current owned-row generated baseline.
Most large public lanes are also stable and median-faster. The exception is
`no_metadata public` at 100,000 rows, which is median-faster but unstable across
the three recorded runs.

Remaining bounded risk:

- tiny, 100, 500, and 1,000 row projection benchmark lanes are unstable and do
  not support a speed claim;
- `no_metadata public` at 100,000 rows is unstable and requires rerun or
  root-cause analysis before it can support a stable speed claim;
- benchmark metadata is recorded in a companion file, not embedded in each
  benchmark JSON artifact;
- the current-owned baseline is a single pre-rewrite run file, so direct-vs-
  current comparisons below stable large-row payloads remain weak evidence.
