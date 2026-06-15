# MBT Projection Direct Writer Result Peer Audit v2

Status: `PEER_AUDIT_PASSED`
Date: 2026-06-15

## Audit Scope

This audit verifies whether the amended result review resolves the blockers
from:

```text
docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_result_peer_audit.md
```

Reviewed artifacts:

- `docs/specs/mbt_projection_direct_writer_SPEC.md`
- `docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_result_review.md`
- `docs/evidence/mbt_projection_direct_writer/projection_run_1.json`
- `docs/evidence/mbt_projection_direct_writer/projection_run_2.json`
- `docs/evidence/mbt_projection_direct_writer/projection_run_3.json`
- `docs/evidence/mbt_projection_direct_writer/benchmark_environment.md`
- `docs/evidence/mbt_projection_direct_writer/dependency_trees.md`
- `docs/evidence/mbt_projection_direct_writer/compile_surface.md`

Protocols checked:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/protocols/review_documentation_protocol.md`
- `docs/protocols/testing_benchmark_protocol.md`

## Findings

No blocking findings remain.

### 1. Stable-run methodology is now applied

Status: resolved

The amended result review now reports median rows/sec and median MB/sec for
large Bars public/archived lanes. It also reports max/min spread and marks
stability using the required 7 percent rule.

Evidence:

- `docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_result_review.md`
  contains a `Median Performance` table.
- The table includes `Spread` and `Stable` columns.
- The review explicitly refuses stable speed claims for unstable lanes.

Large-row median evidence from raw benchmark artifacts:

| Lane | Rows | Median rows/s | Median MB/s | Spread | Median vs current owned | Median vs old crate |
|---|---:|---:|---:|---:|---:|---:|
| no metadata archived | 10,000 | 1,843,410 | 267 | 6.00% | 1.021 | 2.636 |
| no metadata archived | 100,000 | 1,766,430 | 256 | 2.20% | 1.062 | 2.788 |
| no metadata public | 10,000 | 1,525,830 | 221 | 2.34% | 1.073 | 2.163 |
| no metadata public | 100,000 | 1,558,628 | 225 | 56.83% | 1.082 | 2.470 |
| ohlcv archived | 10,000 | 2,473,812 | 151 | 3.33% | 1.081 | 2.972 |
| ohlcv archived | 100,000 | 2,273,654 | 138 | 1.08% | 1.059 | 2.852 |
| ohlcv public | 10,000 | 2,382,134 | 145 | 5.95% | 1.003 | 2.860 |
| ohlcv public | 100,000 | 2,282,179 | 139 | 1.86% | 1.063 | 2.860 |

Audit conclusion:

- large archived projection speed is accepted;
- seven of eight large public/archived lanes are stable and median-faster than
  both baselines;
- `no_metadata public` at 100,000 rows is median-faster but unstable and is not
  accepted as stable speed evidence.

### 2. Result claims are now scoped to what evidence proves

Status: resolved

The amended review no longer claims blanket same-or-faster behavior. It
separates accepted claims from non-accepted claims.

Accepted in the result review:

- correctness;
- generated source reproducibility;
- dependency containment;
- compile surface for this schema size;
- large archived projection speed.

Not accepted in the result review:

- universal speed claim across all row counts;
- stable speed claim for small-row lanes;
- stable speed claim for `no_metadata public` at 100,000 rows;
- current-owned comparison for small lanes.

This satisfies the evidence discipline requirement: no public claim exceeds the
recorded run evidence.

### 3. Benchmark metadata gap is closed by companion evidence

Status: resolved with caveat

The benchmark JSON artifacts still contain only row measurements. The amended
evidence chain now adds:

```text
docs/evidence/mbt_projection_direct_writer/benchmark_environment.md
```

That companion file records:

- benchmark artifact timestamps;
- operator;
- git commit and dirty state;
- benchmark command and release profile;
- dataset/schema identity and row counts;
- OS/kernel;
- CPU and RAM;
- Rust toolchain;
- raw output paths.

This satisfies the v1 audit requirement because the v1 audit allowed a
companion evidence file. It remains weaker than embedding metadata into each
JSON artifact. That weakness is explicitly recorded in the result review and
does not block closure of this surface.

### 4. Dependency tree evidence is now complete

Status: resolved

Full dependency tree output is now recorded at:

```text
docs/evidence/mbt_projection_direct_writer/dependency_trees.md
```

The compile evidence links to that artifact.

Reviewed dependency evidence shows:

- generated schema crates depend on `metamorphic_binary_transport_core` and
  `rkyv`;
- the benchmark crate depends on core and the two schema crates;
- no JSON, protobuf, CSV, Arrow, Arrow IPC, Parquet, or metamorphose adapter
  dependency is pulled into the generated schema crates.

## Accepted Evidence

The following evidence is accepted:

- codegen, schema, Bars, and benchmark tests passed as recorded in the result
  review;
- generated source reproducibility checks passed;
- benchmark correctness oracle reported 216 rows and 0 checksum mismatches;
- dependency containment passed;
- compile surface was measured and recorded;
- large archived projection lanes are stable and median-faster than both
  baselines.

## Remaining Caveats

These caveats are accepted because they are explicitly stated and no stronger
claim is made:

- small-row projection lanes remain unstable;
- `no_metadata public` at 100,000 rows remains unstable;
- benchmark metadata is stored in a companion evidence file rather than inside
  each benchmark JSON file;
- current-owned baseline remains a single pre-rewrite run, so it is weak
  evidence for unstable lanes.

## Classification

`PEER_AUDIT_PASSED`

The result review is now acceptable for closing the projection direct-writer
surface with the scoped claims stated above.

## Next Step

The next migration surface may proceed only under a new spec and implementation
plan. The projection direct-writer result should be treated as:

- accepted for correctness;
- accepted for generated schema-specific direct projection;
- accepted for large archived projection speed;
- not accepted as universal speed proof for every row count and public lane.
