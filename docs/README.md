# Validation and benchmark evidence

Latest run: `20260913T131540Z-m4-pro`. All 24 recorded commands passed, including
178 workspace tests and checks for all 24 generated modules. Two generation
replays produced identical files. This is local evidence; remote CI and release
readiness are not established.

The machine was an Apple M4 Pro (Mac16,7), 14 logical CPUs, 24 GiB RAM, macOS 26.5,
arm64, running Rust 1.89.0. Runs used the release profile, deterministic in-memory
fixtures and an existing build cache. The working tree was dirty. No other-workload
isolation was asserted.

[Machine and toolchain](evidence/20260913T131540Z-m4-pro/machine.json),
[commands and exit codes](evidence/20260913T131540Z-m4-pro/commands.json),
[source hashes](evidence/20260913T131540Z-m4-pro/source_sha256.json), and
[dependency lockfile](evidence/20260913T131540Z-m4-pro/dependencies.lock)
bind the results to their inputs. Every benchmark JSON includes measured machine
metadata and the invocation in `execution_context`; original measurements remain
unchanged. This supplements the native Linux CPU/RAM collectors on macOS.

| Validation | Evidence |
| --- | --- |
| 178 tests passed, zero failed | [Workspace test log](evidence/20260913T131540Z-m4-pro/tests.log) |
| All 24 generated modules match | [Generation check](evidence/20260913T131540Z-m4-pro/generation.log) |
| Deterministic generation replay | [Replay check](evidence/20260913T131540Z-m4-pro/generation_replay_check.log) |
| All-target, all-feature check | [Check log](evidence/20260913T131540Z-m4-pro/check.log) |
| All-feature build | [Build log](evidence/20260913T131540Z-m4-pro/build.log) |
| Formatting and whitespace | [Formatting](evidence/20260913T131540Z-m4-pro/format.log), [diff](evidence/20260913T131540Z-m4-pro/diff.log) |
| New schema default/all-feature checks and release build | [Commands with profiles and timings](evidence/20260913T131540Z-m4-pro/commands.json) |

The recovery audit maps 21 original tests and 76 assertion bodies onto neutral
schema identifiers: eight schema tests and thirteen benchmark contract tests.
Three additional tests validate the reconstructed owned projection and reference
report rejection. The previous 154 tests remain. See [coverage audit](evidence/20260913T131540Z-m4-pro/coverage.json).

The neutral measurement schema preserves the mapped field types, tags, dictionary
order, presence bits, nesting and projection definitions. Existing 16 generated
modules and core/generator source files are unchanged. Eight new generated modules
are within the specified 10% byte-growth budget. See [structure and size audit](evidence/20260913T131540Z-m4-pro/schema_structure.json).

| Benchmark | Lanes × input sizes × runs | Reports |
| --- | --- | --- |
| Wide measurement regression | 10 × 6 × 3 = 180 records | [Run 1](evidence/20260913T131540Z-m4-pro/measurement_regression/measurement_regression_run_1.json), [2](evidence/20260913T131540Z-m4-pro/measurement_regression/measurement_regression_run_2.json), [3](evidence/20260913T131540Z-m4-pro/measurement_regression/measurement_regression_run_3.json) |
| Measurement and compatibility projection | 12 × 6 × 3 = 216 records | [Run 1](evidence/20260913T131540Z-m4-pro/measurement_projection/projection_run_1.json), [2](evidence/20260913T131540Z-m4-pro/measurement_projection/projection_run_2.json), [3](evidence/20260913T131540Z-m4-pro/measurement_projection/projection_run_3.json) |
| Matching reconstructed owned projections | 12 × 6 × 3 = 216 records | [Run 1](evidence/20260913T131540Z-m4-pro/measurement_projection/owned_projection_run_1.json), [2](evidence/20260913T131540Z-m4-pro/measurement_projection/owned_projection_run_2.json), [3](evidence/20260913T131540Z-m4-pro/measurement_projection/owned_projection_run_3.json) |
| Wide measurement compression, full mode | 3 × 6 × 3 = 54 records | [Run 1](evidence/20260913T131540Z-m4-pro/measurement_compression/compression_run_1.json), [2](evidence/20260913T131540Z-m4-pro/measurement_compression/compression_run_2.json), [3](evidence/20260913T131540Z-m4-pro/measurement_compression/compression_run_3.json) |
| Retained telemetry regression | 9 × 6 × 1 = 54 records | [Report](evidence/20260913T131540Z-m4-pro/regression/telemetry_regression_run_1.json) |
| Retained projection suite | 9 × 6 × 1 = 54 records | [Report](evidence/20260913T131540Z-m4-pro/projection/projection_run_1.json) |
| Retained compression suite, full mode | 2 × 6 × 1 = 12 records | [Report](evidence/20260913T131540Z-m4-pro/compression/compression_run_1.json) |

The six sizes are 1, 100, 500, 1,000, 10,000 and 100,000 rows. Recovered regression
and projection retain one timed sample per lane/size per run. Full compression
retains 50, 50, 50, 50, 10 and 3 iterations respectively. All reports have finite
metrics; compression verifies byte equality and deterministic checksums. Direct
and reconstructed owned projections have matching output sizes and checksums.

Across the three runs, 46 of 60 regression lane/size combinations and 59 of 72
projection combinations exceed a 7% max/min throughput spread. The
[three-run medians and stability flags](evidence/20260913T131540Z-m4-pro/repeat_stability.json)
record this variation. These measurements do not support a stable speedup claim.

The regression serde baseline uses a flat physical-row DTO, whose JSON shape
differs from generated logical JSON. The owned projection reference is reconstructed
using current generated types; its checked wrapper validates and then reopens the
immutable archive. It is not the unavailable external implementation. External
comparison fields remain null; reference path metadata denotes expected inputs,
not available evidence. Historical timings were not reused as current results.

The first attempt exposed a temporary-directory collision between retained and
recovered compression tests. The recovered test now uses its own directory;
assertions were unchanged. The [failed attempt](evidence/20260913T131442Z-m4-pro/tests.log)
and the passing rerun are both retained.

[Validation](evidence/20260913T131540Z-m4-pro/validation.json) and
[final checks](evidence/20260913T131540Z-m4-pro/final_checks.json) summarize artifact
verification. The earlier limited suite remains under
[evidence/20260913T125341Z-m4-pro](evidence/20260913T125341Z-m4-pro/validation.json).
