# SPEC: MBT Runtime Archive Parity Corrective

## 1. Identification

Slug: `mbt_runtime_archive_parity_corrective`

Primary repository:

```text
/home/tia/_DEV/MATHILDE/metamorphic-binary-transport
```

Old parity reference repository:

```text
/home/tia/_DEV/MATHILDE/experiments
```

Research brief:

```text
docs/reviews/mbt_runtime_archive_parity_corrective/mbt_runtime_archive_parity_corrective_research_brief.md
```

Related benchmark spec:

```text
docs/specs/mbt_bars_regression_benchmark_SPEC.md
```

## 2. Status

Status: `PEER_AUDITED_IMPLEMENTATION_AWAITING_APPROVAL`

This spec does not authorize implementation. Code may change only after:

1. this spec passes a separate peer audit;
2. an implementation plan exists;
3. the implementation plan passes a separate peer audit;
4. the implementation plan is explicitly approved.

## 3. Purpose

Restore archive layout and generated trusted access parity with the old MBT
implementation so the Bars regression benchmark measures the split-workspace
architecture rather than accidental runtime/codegen differences.

## 4. Non-goals

This spec does not:

- redesign projections;
- redesign metamorphose;
- redesign transponding;
- add a new benchmark surface;
- add a new adapter;
- hand-edit generated files;
- change checked access validation;
- change MBT envelope format;
- change schema hashes;
- change old experiments code;
- claim performance parity before rerun evidence exists.

## 5. Measured object

The measured object is:

```text
generated schema archive layout
generated trusted access path
Bars regression benchmark after corrective regeneration
```

The correction applies to all generated schema crates currently in the new
workspace, not only Bars:

```text
crates/schemas/bars_core
crates/schemas/test_compatibility_core
```

## 6. Schema source contract

Schema sources remain:

```text
crates/schemas/bars_core/proto/mathilde/binary_transport/v1/bars.proto
crates/schemas/test_compatibility_core/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto
```

The spec does not add, remove, or modify proto schemas.

## 7. Wire and archive contract

The MBT envelope remains unchanged.

The rkyv archive layout must match the old MBT layout policy by enabling
`rkyv` `unaligned` for generated schema crates.

The dependency source of truth must be workspace-level:

```toml
[workspace.dependencies]
rkyv = { version = "=0.8.16", features = ["unaligned"] }
```

Schema crates must depend on workspace `rkyv`, not duplicate local feature
choices.

## 8. Checked and trusted access contract

Checked access remains defensive:

```text
decode payload
validate envelope/header/schema
rkyv checked access
validate archived payload shape
return checked view
```

Trusted access is only for immutable bytes already accepted by checked MBT
access for the same schema and then stored or transported without mutation.

Trusted access must match the old MBT hot path:

```text
trusted_payload_for_schema(bytes, schema)
rkyv::access_unchecked(payload)
return archived payload
```

Trusted access must not call:

```text
decode_header
validate_archived_payload
```

inside the generated `access_archived_trusted_unchecked` function.

## 9. Codegen contract

The codegen emitter owns trusted access generation. Generated schema files must
not be manually edited.

The emitter must generate the old trusted access shape for every generated
core schema.

If a generated import becomes unused after removing `decode_header` from
trusted access, the emitter must stop emitting that unused import unless another
generated helper still uses it. Current projection-surface schema files still
use `decode_header` in checked decode helpers, so this spec does not require
removing that import when it remains used outside trusted access.

Archive checksum helper generation must use stable `rkyv::primitive::Archived*`
aliases for archived numeric arrays:

```text
rkyv::primitive::ArchivedI64
rkyv::primitive::ArchivedI32
rkyv::primitive::ArchivedU32
rkyv::primitive::ArchivedF64
rkyv::primitive::ArchivedF32
```

The emitter must not hardcode `rkyv::rend::*_le` or `rkyv::rend::*_ule` for
array checksum helper iterator types. The `Archived*` aliases are the correct
compile-time bridge across aligned and unaligned archive layouts.

Codegen CLI surfaces are existing behavior and must not change:

```text
--surface core
--surface projection
--surface metamorphose --adapter <adapter>
```

The committed Bars and test-compatibility generated schema files are
projection-surface outputs because they include projection markers and
projection APIs. This corrective pass must therefore regenerate those files
with:

```text
--surface projection
```

The core-surface no-projection behavior remains tested in codegen unit tests,
but it is not the committed generated artifact surface for these two schema
files.

## 10. Crate boundary contract

Allowed crate changes:

- root workspace dependency table;
- schema crate dependency declarations;
- codegen emitter;
- generated core schema files through codegen only;
- tests and result review artifacts listed in this spec.

Forbidden crate changes:

- MBT core runtime logic;
- adapter runtime logic;
- benchmark measured object;
- old experiments crate source.

## 11. Dependency contract

No new dependency is allowed.

Allowed dependency change:

- centralize existing `rkyv` `=0.8.16` dependency in workspace dependencies;
- enable existing `rkyv` `unaligned` feature to match the old MBT archive
  layout policy.

`Cargo.lock` is not expected to change because the package version is unchanged.
If `Cargo.lock` changes during implementation, the implementation must stop and
explain the diff before proceeding.

## 12. Determinism contract

Codegen output must be deterministic.

Benchmark reruns must write timestamped or incremented evidence files through
the existing Bars benchmark output path:

```text
docs/evidence/mbt_bars_regression_benchmark/bars_regression_run_N.json
```

The corrective result review must identify the exact run file used.

## 13. Failure contract

Implementation must stop if:

- generated files differ after a `--check` command;
- `rkyv` `unaligned` is not visible in schema dependency resolution;
- generated trusted access still calls archived payload validation;
- checked access validation is weakened;
- Bars benchmark does not run;
- old-vs-new parity remains unproved but is accidentally claimed.

## 14. Compile-surface budget

The change must not add schema crates, adapter crates, or benchmark crates.

Compile-surface evidence must include:

```text
cargo check -p metamorphic_binary_transport_schema_bars --no-default-features
cargo check -p metamorphic_binary_transport_schema_test_compatibility --no-default-features
cargo check -p metamorphic_binary_transport_schema_bars --features json,protobuf,csv,arrow_ipc,parquet
```

## 15. Runtime performance budget

The target is old-MBT parity within the benchmark tolerance already used for
Bars regression work: no worse than `3.5%` for comparable hot-path lanes after
three reruns, unless the result review proves remaining differences are outside
this corrective scope.

Priority lanes:

- `bars_mbt_full_encode_inspect_checked`;
- trusted JSON;
- trusted protobuf;
- trusted CSV;
- trusted Arrow IPC;
- trusted Parquet.

## 16. Correctness oracle

Correctness is proved by:

1. codegen checks for generated files;
2. schema tests for Bars and test compatibility;
3. generated trusted access grep proving old trusted shape;
4. benchmark run files proving the benchmark completed;
5. result review separating proved facts from unproved parity.

Old and new semantic checksum fields must not be treated as comparable unless
the result review proves their checksum algorithms are the same.

## 17. Benchmark methodology

Use the existing Bars regression benchmark command:

```text
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_bars_regression_bench -- --report-dir docs/evidence/mbt_bars_regression_benchmark
```

The implementation should run it three times after correctness checks pass.

The old parity evidence remains read-only reference evidence:

```text
/home/tia/_DEV/MATHILDE/experiments/crates/mathilde-binary-transport/docs/evidences/bars_regression_parity/
```

## 18. Test plan

Required tests and checks:

```text
cargo fmt --check
cargo test -p metamorphic_binary_transport_codegen -- --nocapture
cargo test -p metamorphic_binary_transport_schema_bars --features json,protobuf,csv -- --nocapture
cargo test -p metamorphic_binary_transport_schema_test_compatibility --features json,protobuf,csv -- --nocapture
```

Required dependency proof:

```text
cargo tree -p metamorphic_binary_transport_schema_bars --no-default-features -e features
cargo tree -p metamorphic_binary_transport_schema_test_compatibility --no-default-features -e features
```

The implementation report must state whether `rkyv` `unaligned` appears in the
feature tree.

## 19. Code bindings

Allowed implementation edits:

```text
Cargo.toml
crates/schemas/bars_core/Cargo.toml
crates/schemas/test_compatibility_core/Cargo.toml
crates/codegen/src/rust_emit.rs
crates/codegen/src/tests/test_rust_emit_core.rs
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md
```

No other production code file is authorized by this spec.

## 20. Generated artifact bindings

Generated files that may change only through codegen:

```text
crates/schemas/bars_core/src/bars_v1.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

These files are projection-surface generated artifacts, not core-only generated
artifacts.

Codegen check commands:

```text
cargo run -p metamorphic_binary_transport_codegen -- \
  --check \
  --proto-root proto \
  --proto-root crates/schemas/bars_core/proto \
  --schema mathilde/binary_transport/v1/bars.proto \
  --root mathilde.binary_transport.v1.MathildeTransportResponseV1 \
  --module bars_v1 \
  --surface projection \
  --out crates/schemas/bars_core/src/bars_v1.rs
```

```text
cargo run -p metamorphic_binary_transport_codegen -- \
  --check \
  --proto-root proto \
  --proto-root crates/schemas/test_compatibility_core/proto \
  --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto \
  --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 \
  --module test_compatibility_v1 \
  --surface projection \
  --out crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

Implementation may use the same commands with `--write` before running
`--check`.

## 21. Review artifact bindings

Required review artifacts:

```text
docs/reviews/mbt_runtime_archive_parity_corrective/mbt_runtime_archive_parity_corrective_research_brief.md
docs/specs/mbt_runtime_archive_parity_corrective_SPEC.md
docs/reviews/mbt_runtime_archive_parity_corrective/mbt_runtime_archive_parity_corrective_peer_audit.md
docs/reviews/mbt_runtime_archive_parity_corrective/mbt_runtime_archive_parity_corrective_implementation_plan.md
docs/reviews/mbt_runtime_archive_parity_corrective/mbt_runtime_archive_parity_corrective_implementation_plan_peer_audit.md
```

Result artifact to update after implementation:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md
```

Evidence directory:

```text
docs/evidence/mbt_runtime_archive_parity_corrective/
```

Benchmark output directory:

```text
docs/evidence/mbt_bars_regression_benchmark/
```

## 22. Implementation plan requirement

An implementation plan is mandatory and must bind:

- exact files to edit;
- exact generated files to regenerate;
- exact validation commands;
- exact grep checks proving trusted access shape;
- exact benchmark commands;
- rollback boundary;
- result review update.

Implementation cannot start until that plan is approved.

## 23. Approval checklist

Pre-audit closure checklist:

- mandatory section order matches `docs/protocols/spec_protocol.md`;
- prior Bars regression benchmark spec is cited and preserved;
- codegen command surfaces are exact;
- generated artifacts have one owner: codegen;
- runtime dispatch path to change is bound to `crates/codegen/src/rust_emit.rs`;
- affected tests are bound;
- dependency change is bound;
- compile-surface evidence commands are defined;
- benchmark methodology is defined;
- correctness oracle is defined;
- no design decision is deferred to implementation.

Spec approval checklist:

- required reads complete;
- measured object precise;
- correctness oracle defined;
- benchmark method defined;
- code bindings exact;
- generated artifact bindings exact;
- peer audit passed;
- implementation plan still required.

## 24. Open questions

None.
