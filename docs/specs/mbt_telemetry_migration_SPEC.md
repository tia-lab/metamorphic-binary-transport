# SPEC: MBT telemetry example migration

Approved amendment (2026-09-09): [CSV string-array correction](../reviews/mbt_telemetry_migration/csv_correction_amendment.md). It permits one CSV writer method and one generator call-site correction; all other algorithm restrictions remain.

## 1. Identification

Slug: `mbt_telemetry_migration`. Date: 2026-09-09.
Research: `docs/reviews/mbt_neutral_mirror/mbt_neutral_mirror_research_brief.md`.
Scope: replace the application example and its executable dependencies in this
mirror. This is the functional phase of the neutral-mirror work.

## 2. Status

Approved by the repository owner in conversation on 2026-09-09.
Implementation is authorized together with the approved implementation plan.

## 3. Purpose

Replace the bars example with the proposed synthetic telemetry schema. Use
neutral MBT option names and regenerate all affected artifacts. Make examples
and benchmarks runnable without internal files. Keep transport algorithms intact.

Prior-spec disposition: all 15 existing specs were searched for schema, codegen,
projection, compression and build-identity bindings. This spec locally supersedes
their bars crate/type/fixture/path bindings, old benchmark baselines, and old
option namespace wherever executable consumers migrate below. In particular:

- Workspace architecture and crate-short-names: replace bars with telemetry.
- Core runtime: change only the build identity string defined in section 7.
- Codegen migration and explicit dictionary composition: preserve generation,
  resolution and hashing algorithms; change option qualification only.
- Schema core generation: retain all-fields coverage with the neutral identity
  and fictional bitmask values defined in section 6.
- Projection migration and direct writer: retain generic projection behavior;
  substitute the telemetry projection for the two bars projections.
- Metamorphose migration, projection adapters and derived UTC: retain generic
  adapter rules and nested-message tests; replace application example bindings.
- Runtime archive parity corrective: retain runtime validation and access rules.
- Bars regression and compression: retire bars dataset/baseline claims; use the
  newly identified dataset and benchmark lanes below.
- Production commenting: retain its general rules.

Other prior requirements remain in force. Historical artifacts are not evidence
for telemetry. Their removal or redaction is a separate neutral-mirror phase.

## 4. Non-goals

No generator redesign, new options, new runtime validation algorithms, historical
performance equivalence claim, backwards-compatible bars API, remote operation,
history rewrite, publication, or complete repository sanitization in this phase.
Do not delete or rewrite historical specs, reviews or evidence in this phase.

## 5. Measured object

Source-to-generated-output reproducibility; telemetry encode/access/projection
correctness; adapter semantics; build/dependency surface; runnable local benches.
No upstream data or storage service is required.

## 6. Schema source contract

Install exactly the TelemetryResponseV1 / TelemetryRowV1 proto proposed in the
research brief's "Concrete placeholder schema proposal" section. That block is
the single schema definition for this draft; no duplicate proto is maintained.

- Source: `crates/schemas/telemetry_core/proto/mbt/example/telemetry/v1/telemetry.proto`.
- Import: `mbt/options.proto`, owned by `proto/mbt/options.proto`.
- Root: `mbt.example.telemetry.v1.TelemetryResponseV1`.
- Module: `telemetry_v1`; crate: `mbt_schema_telemetry`.
- ID: 50001; version: 1; transport: `mbt.example.telemetry.v1`.
- Keys: device dictionary ordinal, recorded_at_ms, in that order.
- Presence: battery_percent is bit 0; absence requires canonical zero storage.
- Projection: temperature_only, marker TelemetryV1TemperatureOnly, retaining
  schema version and keys through existing projection rules.

Move the all-fields proto to
`crates/schemas/test_compatibility_core/proto/mbt/test_compatibility/v1/all_fields.proto`.
Its package becomes `mbt.test_compatibility.v1`; transport name matches that
package; ID 40001, version 1, module, messages, field tags, field names, presence
bits and projections remain unchanged. Replace its venue dictionary values in
declaration order with `site_a`, `site_b`, `site_c`; migrate corresponding generated
constants and expected values in its tests and benchmark fixture. Venue is merely
a generic fixture field, not an application venue registry.

The options file retains every extension tag and option message field. Only its
path and package change. Remove the old source paths after their replacements
and consumers are complete. No compatibility aliases for the old namespace.

## 7. Wire and archive contract

Preserve the 128-byte envelope, `MATBT001` magic, encoding kind, version, flags,
field offsets, checksum algorithms and rkyv configuration. Change BUILD_ID_INPUT
to `mbt:v1:synthetic_example` in `crates/core/src/envelope.rs`. This changes the
default emitted build ID; it does not change the header layout or establish
schema compatibility. Do not infer new validation rules from that value.

Generate new schema hashes from the unchanged normalized-hash function. Old bars
ID 1 must fail telemetry schema access. Old all-fields hashes are not accepted
as the new identity. Do not manufacture new hashes or update old evidence.
Derived UTC remains a boundary string, absent from the physical archive.

## 8. Checked and trusted access contract

Use existing checked encode/access/inspect APIs. Trusted calls require checked
validation of the same immutable buffer first; document that at each new unsafe
call. Preserve current archive validation and direct projection emission. No
new zero-copy or allocation claim is made. No sorting is added to the runtime.

## 9. Codegen contract

Only namespace lookups and source fixtures change in codegen; descriptor,
normalization, dispatch, formatting and Rust emission algorithms are preserved.
Existing codegen tests remain, including nested-message/derived-UTC tests.

For each schema tuple below, the exact base command is:

```text
cargo run -p mbt_codegen --bin mbt_codegen -- ACTION --proto-root CRATE/proto --proto-root proto --schema SCHEMA --root ROOT --module MODULE --surface SURFACE
```

The uppercase words are finite substitutions defined here, not optional user
configuration. Schema tuples (CRATE; SCHEMA; ROOT; MODULE):

```text
crates/schemas/telemetry_core; mbt/example/telemetry/v1/telemetry.proto; mbt.example.telemetry.v1.TelemetryResponseV1; telemetry_v1
crates/schemas/test_compatibility_core; mbt/test_compatibility/v1/all_fields.proto; mbt.test_compatibility.v1.TestCompatibilityResponseV1; test_compatibility_v1
```

For the main module, SURFACE is `projection`, never `core`: it owns both the
base schema and declared projection APIs in one file. ACTION is `--write`, then
`--check`, each with `--out CRATE/src/MODULE.rs`. No second generator owns that
path. For inspection, ACTION is `--inspect`, with no `--out`.

For each adapter below, SURFACE is `metamorphose`; append `--adapter ADAPTER` and
`--out CRATE/src/MODULE_SUFFIX.rs` to each write/check invocation:

| ADAPTER | SUFFIX |
| --- | --- |
| json | json |
| protobuf | protobuf |
| csv | csv |
| transponding | transponding |
| arrow | arrow |
| arrow-ipc | arrow_ipc |
| parquet | parquet |

`scripts/schema_codegen.py` shall expand exactly this matrix for actions
`--write`, `--check`, `--inspect`; accept exactly one action; exit nonzero on the
first failing subprocess. It must invoke the existing CLI, not generate Rust
itself. Inspect output goes to stdout. No new environment-variable interface.
Two writes followed by check must preserve all 16 generated files byte-for-byte.

## 10. Crate boundary contract

Replace the bars workspace member with telemetry. Telemetry has the same
optional adapter feature names and path dependencies as bars. Default features
remain empty. Compatibility retains its crate and feature boundaries. Bench
fixtures and reporting stay in mbt_benches; no benchmark helper enters core or
the production telemetry crate.

## 11. Dependency contract

No new third-party crate, version change or runtime dependency. Replace the
benches' bars path dependency with telemetry and the same enabled features.
The generation wrapper uses only Python's standard library, invoking Cargo and
the existing generator. CI must provide protoc, Python and the pinned Rust toolchain.

## 12. Determinism contract

Dictionary order is fixed by the proto. Benchmark fixture `telemetry_rows(n)`
is the sole benchmark fixture producer and uses, for row index i:

- device sensor_a; recorded_at_ms = 1_700_000_000_000 + i * 1_000;
- temperature_c = 20 + (i modulo 16) / 4, using finite binary-exact values;
- battery_percent = 50 with presence bit 0 for even i; absent/zero for odd i;
- status cycles through declared dictionary values;
- tags alternate indoor+test and outdoor+test.

Support only the existing bounded benchmark row counts, so index/timestamp
conversion cannot overflow; conversions remain checked. No randomness or clock
value enters fixture generation. Version is 1. Dataset identity is
`synthetic_telemetry_v1_formula_1`. Independent small test rows are correctness
oracles, not a second benchmark fixture implementation.

## 13. Failure contract

Preserve existing typed errors for invalid ordinals, masks, presence, nonfinite
numbers, key regression, schema/version/hash mismatch, corruption, truncation
and response caps. Absence must differ from present zero. Battery 0..100 is a
fixture convention, not a new runtime rule. Equal keys follow existing runtime
semantics; generated benchmark fixtures have unique keys. File failures and
invalid CLI arguments fail explicitly; reports must not overwrite prior runs.
No fallback to old schema bytes, internal paths or stale benchmark baselines.

## 14. Compile-surface budget

No dependency increase; no new runtime generic machinery; exactly eight generated
Rust files per schema. Each telemetry output and their aggregate line/byte count
must not exceed its corresponding current bars output. If exceeded, stop and
review the measured reason before changing the budget. Record counts before
removal. No compile-time speed threshold is asserted.

Record timed dev checks for telemetry with default and all features and an
all-feature release build, using `/usr/bin/time -p cargo ...`; record the current
dirty state, toolchain, OS, CPU, RAM and whether dependencies/target are warm.
Capture `cargo tree -p mbt_schema_telemetry` and its all-feature variant. These
are observed build costs, not clean-build or improvement claims. Macro expansion
size is not a gate because no macro definitions or derivation strategy change.

## 15. Runtime performance budget

No throughput threshold or bars performance comparison. Preserve direct writers
and checked/trusted dispatch. Only finite positive-duration rates are valid;
zero-duration samples fail rather than manufacture throughput. Response cap is
the existing benchmark value 1_073_741_824 bytes. Correctness precedes timing.

## 16. Correctness oracle

Tests compare every physical telemetry field, keys and optional presence after
encode/access. Projection matches independently assembled selected-field values.
Checked/trusted projection and adapter outputs match byte-for-byte. Decode JSON,
CSV and protobuf sufficiently to compare field values, nullability and derived
UTC, rather than only checking nonempty output. Compare Arrow columns and IPC /
Parquet decoded batches, including omission of derived UTC columns under current
columnar rules. Retain all-fields roundtrip and nullable-array tests.

## 17. Benchmark methodology

Rename the bars regression module/binary/report prefix to telemetry regression.
Keep row counts 1, 100, 500, 1000, 10000, 100000 and existing timing boundaries
for full encode+inspect and checked/trusted adapters. Remove old baseline path,
loader and comparison fields rather than populate them with invented values.
Remove the serde ratio/DTO lane in this migration: a new independently validated
logical JSON baseline is outside scope. This intentionally reduces regression
lanes from ten to nine; label tests must assert the exact new set.

Projection keeps public, archived and inspect timing lanes for telemetry's one
temperature projection and compatibility's two existing projections (nine lanes).
Remove both external old-crate and stored owned-row baseline comparisons/loaders.
Compression keeps full and temperature-only lanes (two instead of three), its
existing iteration schedule, cap and compression configuration. Compression
must decode to exactly the source bytes before accepting a sample.

Use new report directories in section 21, never old paths. Every run records
timestamp, command, operator, dirty state, profile, compiler, OS, CPU, RAM,
dataset identity, row count, schema hash, output size and checksums. Mark cache
mode `in_memory`; no storage comparison. Missing optional machine metadata is
explicitly `unavailable`, not an inferred value. Timing uses a monotonic clock.

Commands:

```text
cargo run --release -p mbt_benches --bin mbt_telemetry_regression_bench -- --report-dir docs/evidence/mbt_telemetry_migration/regression
cargo run --release -p mbt_benches --bin mbt_projection_bench -- --report-dir docs/evidence/mbt_telemetry_migration/projection
cargo run --release -p mbt_benches --bin mbt_compression_bench -- --smoke --report-dir docs/evidence/mbt_telemetry_migration/compression
```

## 18. Test plan

Old test migration, all under `crates/schemas/bars_core/tests`:

| Old file | New file under telemetry_core/tests | Contract |
| --- | --- | --- |
| test_bars_shape.rs | test_telemetry_shape.rs | Dictionary order, keys, projection shape; compare fresh inspect output |
| test_bars_projection.rs | test_telemetry_projection.rs | One projection checked/trusted parity, selected values, direct-writer guard |
| test_bars_metamorphose.rs | test_telemetry_metamorphose.rs | UTC and row-format values; nested-message coverage remains in codegen tests |
| test_projection_metamorphose.rs | test_projection_metamorphose.rs | Telemetry projection through all enabled adapters |
| expected_bars_projection_inspect.txt | expected_telemetry_projection_inspect.txt | Fresh generator stdout; never mechanically rename old hashes |

Add `test_telemetry_roundtrip.rs` and `test_telemetry_failures.rs` for the oracles
in sections 13 and 16, empty payloads, deterministic replay and old ID/hash
rejection. One `tests/common/mod.rs` owns the small independent oracle rows.
Feature-gate adapter tests so default-feature test compilation succeeds.

Preserve all five compatibility test files and their cases, updating only
dictionary constants/values and identity expectations. Preserve codegen fixture
tests, updating namespace/imports. Rename the compression sample bytes to neutral
text; retain all compression failure cases. Update benchmark report tests for
the exact new lane sets, absent old-baseline fields, dataset identity, output
ordering and refusal to overwrite. No tolerance relaxation.

Validation sequence:

```text
cargo test -p mbt_codegen
python3 scripts/schema_codegen.py --write
python3 scripts/schema_codegen.py --write
python3 scripts/schema_codegen.py --check
cargo test -p mbt_schema_telemetry
cargo test -p mbt_schema_telemetry --all-features
cargo test -p mbt_schema_test_compatibility --all-features
cargo test -p mbt_core -p mbt_compression -p mbt_benches
cargo test --workspace --all-features
cargo check --all-targets --all-features
cargo build --all-features
cargo fmt --all -- --check
git diff --check
```

The two write invocations must have identical SHA-256 output manifests. Run
section 14 checks/build and section 17 commands after correctness succeeds.

## 19. Code bindings

`docs/reviews/mbt_telemetry_migration/file_bindings.md` enumerates the 60 existing
read/edit/replace/remove paths with their observed working-tree hashes. Together
with the new-path matrix below it is part of this spec's exact file binding.

Edit `Cargo.toml`; `crates/codegen/src/options.rs`;
`crates/codegen/src/tests/mod.rs`, `test_descriptor.rs`,
`test_rust_emit_metamorphose.rs`; `crates/core/src/envelope.rs`;
`crates/compression/src/tests/test_runtime.rs`.

Create the options and telemetry proto paths in section 6. Move compatibility
proto as specified there. Replace `crates/schemas/bars_core/Cargo.toml` and
`src/lib.rs` with telemetry equivalents; remove its old proto/generated/test
files after migration. Its inventory moves to telemetry and is refreshed.
Compatibility tests are those enumerated in section 18; its manifest and lib
module dispatch remain unchanged.

Edit `crates/benches/Cargo.toml`, `src/lib.rs`, `src/projection.rs`,
`src/bin/mbt_projection_bench.rs`, `src/compression.rs`,
`src/tests/mod.rs`, `src/tests/test_projection_bench_output.rs`, and
`src/tests/test_compression_bench_output.rs`. Rename `src/bars_regression.rs`,
`src/bin/mbt_bars_regression_bench.rs`, and
`src/tests/test_bars_regression_bench_output.rs` to corresponding telemetry names
and migrate their behavior as specified above. The compression binary CLI stays.

Create `scripts/schema_codegen.py`. Edit `.github/workflows/ci.yml` to install
protoc and run generation check and workspace all-feature tests before its
existing checks. Edit `.gitignore` to ignore nested target directories and
remove the tracked codegen target fixture, not arbitrary ignored local files.

Edit `README.md` examples/commands and `architecture.md`,
`docs/architecture/repository_structure.md`, `inventory.md`,
`crates/benches/docs/inventory.md` to describe telemetry and current paths.
Refresh only affected inventory entries. Preserve unrelated in-progress edits.
Historical-document sanitization and root author/package metadata are explicitly
outside this functional phase; do not claim zero remaining company references.

## 20. Generated artifact bindings

The 16 paths and sole reproduction commands are the finite matrix in section 9.
`src/lib.rs` files are handwritten module dispatch, not generated output.
Generate telemetry inspection snapshot with its section 9 projection inspect
command, redirecting stdout to the section 18 snapshot path. Codegen owns that
snapshot; its test compares fresh inspection with the checked-in snapshot.
Keep temporary generator files under existing target directories; no new temp
location or environment switch in generator code.

## 21. Review artifact bindings

Under `docs/reviews/mbt_telemetry_migration/`: peer audit, implementation plan,
result review, and `file_bindings.md`. Evidence under
`docs/evidence/mbt_telemetry_migration/`: `validation.txt`, `environment.txt`,
`compile_surface.md`, `dependency_trees.txt`, `generated_sha256.txt`,
`telemetry_inspect.txt`, `compatibility_inspect.txt`, and three benchmark
subdirectories from section 17. Preserve failed-run logs. Migration rollback
snapshot is local `/tmp/mbt-telemetry-migration-backup`; never publish it.

## 22. Implementation plan requirement

After a separate falsification audit, write a plan binding this spec's exact
files, generated commands, dependency changes, test order, expected outputs,
risks and rollback. Obtain explicit approval before source changes. Snapshot
only the affected working-tree files, including existing dirty content; never
reset the repository to HEAD to roll back this migration.

## 23. Approval checklist

Author closure: required section order present; prior-spec local supersession
stated; CLI matrix and sole generated owners defined; option/feature/benchmark
dispatch paths listed; old test migration and new oracle bindings specified;
compile and run commands bound. Historical cleanup is a separate explicit scope,
not an undeclared implementation decision.

Separate single-agent audit: `PEER_AUDIT_PASSED`, with limitations stated in its
artifact. Spec and implementation plan approved by the owner. Pending:
implementation; generation, correctness, build and benchmark evidence. A source
inspection is not proof that the new schema compiles or runs.

## 24. Open questions

No unresolved payload-design choice. Historical-document removal/redaction is
pending in the broader neutral-mirror work and does not alter this functional
spec. No full sanitization or release approval follows from this migration.

## Metadata portability binding

Validation on macOS proved that `date -u +%s%3N` produces a literal `3N` suffix,
causing compression report construction to fail. Within the approved metadata
work in section 17, `compression.rs::utc_millis` uses
`SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis()` instead. UTC epoch
milliseconds, checked failure, dependencies and monotonic measurement timers
remain unchanged. This implements the existing timestamp contract.
