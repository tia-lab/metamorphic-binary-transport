# Telemetry migration result review

Status: approved functional migration implemented and locally validated.
Date: 2026-09-09.
No release-readiness, full-sanitization or performance claim.

## Changes

The workspace example is now `mbt_schema_telemetry`, with synthetic device,
timestamp, temperature, optional battery, status and tag fields. Options use
the neutral namespace. Both schemas' eight output surfaces were regenerated
through mbt_codegen. The old bars crate and tracked build fixture were removed
within the approved file boundary. Existing dirty content was captured under
`/tmp/mbt-telemetry-migration-backup` before edits; no affected file had changed
since the planning manifest.

All-fields coverage remains, with fictional bitmask values. Telemetry tests
include field oracles, presence, old identity rejection, deterministic replay,
projection values and adapter parity. All three benchmark programs now consume
local fixtures and do not load old internal baselines. README, affected
architecture/inventory entries and CI generation/test commands were updated.
The core change is the approved build identity string; the envelope layout and
magic are unchanged. The approved CSV amendment changes one emitted writer call and adds nested
JSON/CSV escaping to the CSV adapter. Other generator algorithms are unchanged.

## Evidence

Evidence directory: `docs/evidence/mbt_telemetry_migration`.

| Command/check | Observed result | Artifact |
| --- | --- | --- |
| cargo test -p mbt_adapter_csv -p mbt_codegen | 5 adapter and 45 generator tests pass | csv_correction_tests.txt |
| Wrapper write twice, then check | 16 files; identical hashes; check passes | generation_corrected.txt, generated_sha256.txt |
| cargo test --workspace --all-features | 154 tests pass, including unchanged telemetry CSV oracle | workspace_tests_final.txt |
| cargo test -p mbt_benches after timestamp correction | 9 tests pass | benches_final.txt |
| cargo check --all-targets --all-features | passes before timestamp-only correction | workspace_check_final.txt |
| cargo build --all-features | passes before timestamp-only correction | workspace_build_final.txt |
| cargo fmt --all -- --check; git diff --check | pass after final source correction | validation.txt |
| Generated telemetry line/byte budget | all eight outputs within individual old-output bounds | compile_surface.md |
| Release regression benchmark | 54 records, nine lanes for six row counts | regression_run.txt, regression/telemetry_regression_run_1.json |
| Release projection benchmark | 54 records, nine lanes for six row counts | projection_run.txt, projection/projection_run_1.json |
| Release compression smoke benchmark | 6 records, two lanes for three row counts | compression_retry.txt, compression/compression_run_1.json |
| Independent report/Parquet/hash validation | values, nullability, finite metrics, projection parity and compression determinism pass | final_artifact_checks.txt |

Local rustc is 1.89.0 on macOS arm64; CI remains configured for 1.90.0. No CI
execution is claimed. protoc was missing and installed with owner-approved
Homebrew execution. The initial generator smoke test failed because the sandbox
could not resolve crates.io; its failure is retained in codegen_initial.txt.

Timed local checks: default telemetry dev check 1.34 s wall, all-feature dev
check 2.07 s, all-feature release build 26.34 s. These are warm/mixed existing
target/dependency observations from a dirty working tree, not clean-build or
speed-improvement evidence. Raw timing outputs are check_default.txt,
check_all_features.txt and build_release.txt. Dependency trees are captured in
dependency_trees.txt. No third-party dependency was added to the workspace.

Telemetry source hash is 232196938863914698, read from current generated output;
full identity and projection hashes are in telemetry_inspect.txt. Compatibility
identity is recorded in compatibility_inspect.txt. Schema correctness relies on
generation checks and runtime oracles, not these numbers alone.

## Diagnosed failures

The initial new row-format oracle incorrectly assumed a fixed `.000` UTC suffix
and omitted derived UTC from projected row-format output. Existing core UTC
tests and generator remapping proved that UTC has second precision and survives
when its timestamp source is selected. Only those new oracle expectations were
corrected, with diagnosis retained in validation.txt. No runtime behavior changed.

The full CSV failure came from emit_csv_bitmask_helpers invoking a standalone
CSV string-cell writer inside an already quoted array cell. The owner-approved
csv_correction_amendment.md binds the writer and one generator call correction.
Adapter tests cover embedded quotes, backslashes, every ASCII control, Unicode,
empty strings and cap overflow with independent Python csv/json decoding. The
original telemetry oracle now passes without further expectation changes.

The first compression benchmark failed while collecting its timestamp because
macOS date does not implement GNU `%3N`. The existing metadata contract now uses
standard-library epoch milliseconds, propagating errors. Measurement timers and
compression logic are unchanged. compression_run.txt retains the failure;
compression_retry.txt records the successful release run.

## Final scope and limits

The approved generation, correctness, compile-surface and benchmark evidence
chain is complete locally. Benchmark reports contain new synthetic fixture
measurements; they do not establish a performance improvement over the old
application schema. CPU/RAM metadata is explicitly unavailable on this platform.
Independent review has not been performed; the design audit is single-agent.

Historical specs, reviews, evidence, root identity metadata and Git history remain
outside this functional phase. They have not been declared sanitized. No commit,
publication, remote write or upstream-system edit was performed.
