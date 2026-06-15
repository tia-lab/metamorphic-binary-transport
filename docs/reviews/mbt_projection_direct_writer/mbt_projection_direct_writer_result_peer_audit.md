# MBT Projection Direct Writer Result Peer Audit

Status: `BLOCKED`
Date: 2026-06-15

## Audit Scope

This audit reviews whether the completed result review can be accepted as the
final evidence artifact for `mbt_projection_direct_writer`.

Reviewed artifacts:

- `docs/specs/mbt_projection_direct_writer_SPEC.md`
- `docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_implementation_plan.md`
- `docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_result_review.md`
- `docs/evidence/mbt_projection_direct_writer/projection_run_1.json`
- `docs/evidence/mbt_projection_direct_writer/projection_run_2.json`
- `docs/evidence/mbt_projection_direct_writer/projection_run_3.json`
- `docs/evidence/mbt_projection_direct_writer/compile_surface.md`

Protocol reads:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`
- `docs/protocols/peer_audit_protocol.md`
- `docs/protocols/review_documentation_protocol.md`
- `docs/protocols/testing_benchmark_protocol.md`

## Findings

### 1. Result review does not apply the required stable-run methodology

Severity: blocking

Spec evidence:

- `docs/specs/mbt_projection_direct_writer_SPEC.md` requires the result to
  compare median rows/sec and median MB/sec.
- The same section requires a lane to be marked unstable when max/min rows/sec
  differs by more than 7 percent across the three runs.
- It also says a performance claim from an unstable lane must not be accepted
  without rerunning and explaining the instability.

Result-review evidence:

- `docs/reviews/mbt_projection_direct_writer/mbt_projection_direct_writer_result_review.md`
  reports averages, not medians.
- It does not enumerate unstable lanes with max/min ratios.
- It accepts large-row lanes using average values even where the spec requires
  median-based comparison and explicit instability handling.

Run evidence:

```bash
jq -s -r '[.[].rows[] | select(.schema == "bars") | select((.label|test("archived|public"))) ] | group_by(.label,.row_count)[] | {label:.[0].label,row_count:.[0].row_count,ratio:((map(.rows_per_second)|max)/(map(.rows_per_second)|min)),median_current:(map(.current_owned_row_comparison.ratio_rows_per_second)|sort|.[1]),median_old:(map(.old_crate_comparison.ratio_rows_per_second)|sort|.[1])} | select(.ratio > 1.07) | [.label,.row_count,((.ratio-1)*100),.median_current,.median_old] | @tsv' docs/evidence/mbt_projection_direct_writer/projection_run_1.json docs/evidence/mbt_projection_direct_writer/projection_run_2.json docs/evidence/mbt_projection_direct_writer/projection_run_3.json
```

Observed unstable Bars public/archived lanes include:

| Lane | Rows | Max/min spread | Median vs current owned | Median vs old crate |
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

This does not mean the implementation is wrong. It means the result review does
not yet satisfy the benchmark acceptance protocol.

Required correction:

- Amend the result review to report median rows/sec and median MB/sec.
- Add an unstable-lane table using the 7 percent max/min rule.
- Restrict speed claims to stable lanes, or rerun/explain unstable lanes.

### 2. Result review overstates the large-row acceptance unless instability is separated

Severity: blocking

The result review states that large-row archived projection passed the old-crate
gate and current-owned average gate. The median data supports the large archived
10,000 and 100,000 row lanes, but the result review does not explicitly separate
stable from unstable large-row lanes.

Run evidence:

```bash
jq -s -r '[.[].rows[] | select(.schema == "bars") | select((.label|test("archived|public"))) ] | group_by(.label,.row_count)[] | {label:.[0].label,row_count:.[0].row_count,median_current_ratio:(map(.current_owned_row_comparison.ratio_rows_per_second)|sort|.[1]),median_old_ratio:(map(.old_crate_comparison.ratio_rows_per_second)|sort|.[1])} | select(.row_count >= 10000) | [.label,.row_count,.median_current_ratio,.median_old_ratio] | @tsv' docs/evidence/mbt_projection_direct_writer/projection_run_1.json docs/evidence/mbt_projection_direct_writer/projection_run_2.json docs/evidence/mbt_projection_direct_writer/projection_run_3.json
```

Observed median ratios for large rows:

| Lane | Rows | Median vs current owned | Median vs old crate |
|---|---:|---:|---:|
| no metadata archived | 10,000 | 1.021 | 2.636 |
| no metadata archived | 100,000 | 1.062 | 2.788 |
| no metadata public | 10,000 | 1.073 | 2.163 |
| no metadata public | 100,000 | 1.082 | 2.470 |
| ohlcv archived | 10,000 | 1.081 | 2.972 |
| ohlcv archived | 100,000 | 1.059 | 2.852 |
| ohlcv public | 10,000 | 1.003 | 2.860 |
| ohlcv public | 100,000 | 1.063 | 2.860 |

These median ratios are favorable for the large lanes. However, at least
`no_metadata public` with 100,000 rows had a 56.83 percent max/min spread across
the three runs. The result review must state whether that lane is accepted only
by median, rerun for stability, or excluded from a stable speed claim.

Required correction:

- Separate "median favorable" from "stable accepted".
- Do not use three-run average to close a spec gate that requires medians and
  unstable-lane handling.

### 3. Benchmark artifacts do not record required environment metadata

Severity: blocking for benchmark-protocol completeness

Protocol evidence:

- `docs/protocols/testing_benchmark_protocol.md` requires benchmark artifacts
  to record UTC timestamp, operator, git commit or dirty state, command, build
  profile, CPU, RAM, OS/kernel, Rust toolchain, dataset identity, row count and
  payload size, warm/cold mode when relevant, result summary, and raw output
  path.

Artifact evidence:

```bash
jq 'keys' docs/evidence/mbt_projection_direct_writer/projection_run_1.json
```

Observed top-level keys:

```text
["rows"]
```

The row-level schema is deterministic and useful, but the artifact lacks the
benchmark environment metadata required by the repository protocol. The result
review also does not compensate with a complete environment block.

Required correction:

- Either amend the benchmark writer to include a top-level metadata object and
  rerun, or add a companion evidence file with the exact required metadata for
  the three recorded runs.
- The result review must cite that metadata.

### 4. Dependency evidence is summarized, not fully recorded as required by the plan

Severity: non-blocking for runtime correctness, blocking for plan completeness

Implementation-plan evidence:

- The plan requires dependency containment commands to be appended to
  `docs/evidence/mbt_projection_direct_writer/compile_surface.md`.

Artifact evidence:

- `compile_surface.md` records the dependency commands and a summary of the
  observed surface.
- It does not contain the full command output from `cargo tree`.

This is not evidence of a dependency violation. It is an artifact completeness
gap.

Required correction:

- Append the full `cargo tree` outputs for:
  - `metamorphic_binary_transport_schema_bars`
  - `metamorphic_binary_transport_schema_test_compatibility`
  - `metamorphic_binary_transport_benches`

## Accepted Evidence

The following parts of the result are supported by the reviewed evidence:

- Generated source reproducibility passed for both schema crates.
- Test suites passed for codegen, compatibility schema, Bars schema, and bench
  harness.
- The benchmark checksum oracle reported `0` mismatches across `216` rows.
- Generated schema crates remained isolated to core plus `rkyv`.
- Large-row median ratios are favorable versus the current owned-row baseline
  and old-crate baseline.

These accepted points do not remove the benchmark-reporting blockers above.

## Classification

`BLOCKED`

The implementation may remain in place, but this result review cannot be used
to close the `mbt_projection_direct_writer` surface until the result artifact is
amended or the benchmark is rerun with the required metadata and stable-lane
classification.

## Required Next Step

Amend the result review and evidence chain before moving to the next migration
surface.

Minimum amendment:

1. add median rows/sec and median MB/sec tables;
2. add unstable-lane table using the 7 percent max/min rule;
3. distinguish median-favorable lanes from stable-accepted lanes;
4. add benchmark environment metadata;
5. append full dependency tree command output to compile evidence;
6. rerun only if the amended review wants to claim stability for lanes that are
   currently unstable.
