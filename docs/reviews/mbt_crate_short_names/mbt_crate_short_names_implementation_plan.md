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

# MBT Crate Short Names Implementation Plan

## Status

Status: awaiting approval.

This plan does not authorize implementation until the user approves it.

## Inputs

- Spec:
  `docs/specs/mbt_crate_short_names_SPEC.md`
- Passed peer audit:
  `docs/reviews/mbt_crate_short_names/mbt_crate_short_names_peer_audit_v2.md`
- Research brief:
  `docs/reviews/mbt_crate_short_names/mbt_crate_short_names_research_brief.md`

## Goal

Rename the current Rust package/import surface from
`metamorphic_binary_transport_*` to `mbt_*`, while preserving crate paths,
runtime behavior, generated-code ownership, and validation evidence.

## Non-goals

- No wire/archive changes.
- No schema hash changes.
- No algorithm changes.
- No dependency additions.
- No dependency version changes.
- No repository or directory renames.
- No historical evidence rewrite.
- No hand edits to generated schema files.

## Rename Map

```text
metamorphic_binary_transport_core              -> mbt_core
metamorphic_binary_transport_codegen           -> mbt_codegen
metamorphic_binary_transport_metamorphose      -> mbt_metamorphose
metamorphic_binary_transport_transponding      -> mbt_transponding
metamorphic_binary_transport_compression       -> mbt_compression
metamorphic_binary_transport_adapter_json      -> mbt_adapter_json
metamorphic_binary_transport_adapter_protobuf  -> mbt_adapter_protobuf
metamorphic_binary_transport_adapter_csv       -> mbt_adapter_csv
metamorphic_binary_transport_adapter_arrow     -> mbt_adapter_arrow
metamorphic_binary_transport_adapter_arrow_ipc -> mbt_adapter_arrow_ipc
metamorphic_binary_transport_adapter_parquet   -> mbt_adapter_parquet
metamorphic_binary_transport_benches           -> mbt_benches
metamorphic_binary_transport_schema_bars       -> mbt_schema_bars
metamorphic_binary_transport_schema_test_compatibility
                                               -> mbt_schema_test_compatibility
```

The binary remains:

```text
mbt_codegen
```

## Edit Order

The implementation must use this order:

1. Update Cargo package names and dependency keys.
2. Update non-generated source imports.
3. Update codegen crate self-imports and smoke manifest emission.
4. Update `crates/codegen/src/rust_emit.rs` emitted crate names.
5. Run codegen to regenerate generated schema modules.
6. Run matching codegen `--check` commands.
7. Update current README, architecture, and inventory surfaces.
8. Run `cargo make inventory`.
9. Run formatting, tests, compile checks, dependency checks, and static grep.
10. Write the result review.

Generated schema files must not be hand edited. If generated files require a
manual edit, the work stops and the codegen path is fixed instead.

## Files To Edit

### Manifests

```text
Cargo.toml
crates/adapters/arrow/Cargo.toml
crates/adapters/arrow_ipc/Cargo.toml
crates/adapters/csv/Cargo.toml
crates/adapters/json/Cargo.toml
crates/adapters/parquet/Cargo.toml
crates/adapters/protobuf/Cargo.toml
crates/benches/Cargo.toml
crates/codegen/Cargo.toml
crates/compression/Cargo.toml
crates/core/Cargo.toml
crates/metamorphose/Cargo.toml
crates/schemas/bars_core/Cargo.toml
crates/schemas/test_compatibility_core/Cargo.toml
crates/transponding/Cargo.toml
```

Manifest edits:

- change `[package].name`;
- change local dependency keys;
- change schema crate feature `dep:` entries.

### Runtime, Adapter, Codegen, Bench, And Test Source

```text
crates/adapters/arrow/src/lib.rs
crates/adapters/arrow/src/tests/mod.rs
crates/adapters/arrow_ipc/src/arrow_bridge.rs
crates/adapters/arrow_ipc/src/lib.rs
crates/adapters/arrow_ipc/src/tests/mod.rs
crates/adapters/csv/src/lib.rs
crates/adapters/csv/src/tests/mod.rs
crates/adapters/json/src/lib.rs
crates/adapters/json/src/tests/mod.rs
crates/adapters/parquet/src/arrow_bridge.rs
crates/adapters/parquet/src/lib.rs
crates/adapters/parquet/src/tests/mod.rs
crates/adapters/protobuf/src/lib.rs
crates/adapters/protobuf/src/tests/mod.rs
crates/benches/src/bars_regression.rs
crates/benches/src/bin/mbt_bars_regression_bench.rs
crates/benches/src/bin/mbt_compression_bench.rs
crates/benches/src/bin/mbt_projection_bench.rs
crates/benches/src/compression.rs
crates/benches/src/projection.rs
crates/codegen/src/emit.rs
crates/codegen/src/lib.rs
crates/codegen/src/main.rs
crates/codegen/src/rust_emit.rs
crates/codegen/src/tests/test_rust_emit_metamorphose.rs
crates/compression/src/runtime.rs
crates/compression/src/tests/test_runtime.rs
crates/metamorphose/src/runtime.rs
crates/metamorphose/src/tests/test_runtime.rs
crates/schemas/bars_core/tests/test_bars_metamorphose.rs
crates/schemas/bars_core/tests/test_bars_projection.rs
crates/schemas/bars_core/tests/test_bars_shape.rs
crates/schemas/test_compatibility_core/tests/test_determinism.rs
crates/schemas/test_compatibility_core/tests/test_failures.rs
crates/schemas/test_compatibility_core/tests/test_projection.rs
crates/schemas/test_compatibility_core/tests/test_roundtrip.rs
crates/transponding/src/runtime.rs
crates/transponding/src/tests/test_runtime.rs
```

Source edits:

- replace current long imports with `mbt_*` imports;
- update codegen crate self-imports from
  `metamorphic_binary_transport_codegen` to `mbt_codegen`;
- update codegen smoke manifest emission from
  `metamorphic_binary_transport_core` to `mbt_core`;
- update codegen emitted adapter/core/metamorphose/transponding imports;
- update tests that assert emitted long names.

### Generated Schema Outputs

Generated files to be regenerated only by `mbt_codegen`:

```text
crates/schemas/bars_core/src/bars_v1.rs
crates/schemas/bars_core/src/bars_v1_json.rs
crates/schemas/bars_core/src/bars_v1_protobuf.rs
crates/schemas/bars_core/src/bars_v1_csv.rs
crates/schemas/bars_core/src/bars_v1_transponding.rs
crates/schemas/bars_core/src/bars_v1_arrow.rs
crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs
crates/schemas/bars_core/src/bars_v1_parquet.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_json.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_protobuf.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_csv.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_transponding.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow_ipc.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_parquet.rs
```

### Current Documentation And Inventory

```text
README.md
architecture.md
docs/architecture/repository_structure.md
inventory.md
crates/*/docs/inventory.md
crates/adapters/*/docs/inventory.md
crates/schemas/*/docs/inventory.md
```

Rules:

- Update current public docs to use `mbt_*`.
- Preserve historical reviews/specs/evidence unless explicitly bound later.
- Update per-component inventories when source purpose text contains long
  crate names.
- Regenerate root `inventory.md` with `cargo make inventory`.

## Files To Create

```text
docs/reviews/mbt_crate_short_names/mbt_crate_short_names_result_review.md
```

No source files are created.

## Dependency Changes

No dependency versions change.

No new dependency is added.

Only local dependency keys and package names change.

`Cargo.lock` may change locally because it is an ignored Cargo artifact. It is
not a tracked output of this plan.

## Codegen Write Commands

Run Bars writes:

```bash
cargo run -p mbt_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface core --out crates/schemas/bars_core/src/bars_v1.rs
cargo run -p mbt_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface projection --out crates/schemas/bars_core/src/bars_v1.rs
cargo run -p mbt_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter json --out crates/schemas/bars_core/src/bars_v1_json.rs
cargo run -p mbt_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter protobuf --out crates/schemas/bars_core/src/bars_v1_protobuf.rs
cargo run -p mbt_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter csv --out crates/schemas/bars_core/src/bars_v1_csv.rs
cargo run -p mbt_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter transponding --out crates/schemas/bars_core/src/bars_v1_transponding.rs
cargo run -p mbt_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter arrow --out crates/schemas/bars_core/src/bars_v1_arrow.rs
cargo run -p mbt_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter arrow-ipc --out crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs
cargo run -p mbt_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/bars_core/proto --proto-root proto --schema mathilde/binary_transport/v1/bars.proto --root mathilde.binary_transport.v1.MathildeTransportResponseV1 --module bars_v1 --surface metamorphose --adapter parquet --out crates/schemas/bars_core/src/bars_v1_parquet.rs
```

Run test-compatibility writes:

```bash
cargo run -p mbt_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface core --out crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
cargo run -p mbt_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface projection --out crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
cargo run -p mbt_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface metamorphose --adapter json --out crates/schemas/test_compatibility_core/src/test_compatibility_v1_json.rs
cargo run -p mbt_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface metamorphose --adapter protobuf --out crates/schemas/test_compatibility_core/src/test_compatibility_v1_protobuf.rs
cargo run -p mbt_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface metamorphose --adapter csv --out crates/schemas/test_compatibility_core/src/test_compatibility_v1_csv.rs
cargo run -p mbt_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface metamorphose --adapter transponding --out crates/schemas/test_compatibility_core/src/test_compatibility_v1_transponding.rs
cargo run -p mbt_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface metamorphose --adapter arrow --out crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow.rs
cargo run -p mbt_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface metamorphose --adapter arrow-ipc --out crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow_ipc.rs
cargo run -p mbt_codegen --bin mbt_codegen -- --write --proto-root crates/schemas/test_compatibility_core/proto --proto-root proto --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 --module test_compatibility_v1 --surface metamorphose --adapter parquet --out crates/schemas/test_compatibility_core/src/test_compatibility_v1_parquet.rs
```

## Codegen Check Commands

Run matching `--check` commands for every write command above after all writes
complete. The command bodies are identical to the write commands except
`--write` is replaced by `--check`.

If any check command reports a generated diff, stop and fix codegen or
manifest/import setup before continuing.

## Validation Commands

Formatting:

```bash
cargo fmt --check
```

Tests:

```bash
cargo test -p mbt_core --all-targets
cargo test -p mbt_codegen --all-targets
cargo test -p mbt_metamorphose --all-targets
cargo test -p mbt_transponding --all-targets
cargo test -p mbt_compression --all-targets
cargo test -p mbt_adapter_json --all-targets
cargo test -p mbt_adapter_protobuf --all-targets
cargo test -p mbt_adapter_csv --all-targets
cargo test -p mbt_adapter_arrow --all-targets
cargo test -p mbt_adapter_arrow_ipc --all-targets
cargo test -p mbt_adapter_parquet --all-targets
cargo test -p mbt_schema_bars --features json,protobuf,csv,arrow_ipc,parquet --all-targets
cargo test -p mbt_schema_test_compatibility --features json,protobuf,csv,arrow_ipc,parquet --all-targets
cargo test -p mbt_benches --all-targets
```

Compile evidence:

```bash
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p mbt_core
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p mbt_codegen --all-targets
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p mbt_schema_bars --features json,protobuf,csv,arrow_ipc,parquet
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p mbt_schema_test_compatibility --features json,protobuf,csv,arrow_ipc,parquet
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p mbt_benches --all-targets
```

Inventory:

```bash
cargo make inventory
```

Static current-surface checks:

```bash
! rg -n "metamorphic_binary_transport_[a-z0-9_]+" Cargo.toml crates README.md architecture.md inventory.md docs/architecture/repository_structure.md --glob '!target/**' --glob '!crates/**/target/**' --glob '!docs/reviews/**' --glob '!docs/specs/**' --glob '!docs/evidence/**'
! rg -n "name = \"metamorphic_binary_transport|dep:metamorphic_binary_transport|metamorphic_binary_transport_[a-z0-9_]+ =" Cargo.toml crates --glob '*.toml' --glob '!target/**' --glob '!crates/**/target/**'
! rg -n "metamorphic_binary_transport_[a-z0-9_]+" crates/codegen/src/emit.rs crates/codegen/src/rust_emit.rs crates/codegen/src/tests
```

Dependency graph checks:

```bash
cargo tree -p mbt_core
cargo tree -p mbt_schema_bars --features json,protobuf,csv,arrow_ipc,parquet
cargo tree -p mbt_schema_test_compatibility --features json,protobuf,csv,arrow_ipc,parquet
cargo tree -p mbt_compression
```

## Expected Outputs

- All current source imports use `mbt_*`.
- All package names and dependency keys use `mbt_*`.
- `mbt_codegen` emits `mbt_*` crate names.
- Generated schema modules use `mbt_*`.
- README and architecture docs use current `mbt_*` names.
- Root inventory is regenerated.
- Historical reviews/specs/evidence may still contain long names by design.
- Runtime behavior remains unchanged.

## Rollback Boundary

Rollback is all files changed by this plan.

Do not use destructive git commands. If rollback is needed, produce a reverse
patch or ask for explicit approval.

## Known Risks

- Partial rename can break Cargo package resolution.
- Codegen must be updated before generated files are regenerated.
- Historical docs will still contain old names and must not be treated as
  active API surface.
- `Cargo.lock` may change locally but is ignored and not tracked.
- Full schema adapter tests may take time because this touches package names
  across all optional adapter surfaces.

## Stop Conditions

Stop if:

- any generated schema file needs manual edits;
- any codegen check fails;
- any validation command fails;
- static grep still finds a long name in current source surfaces;
- a dependency version changes;
- implementation requires behavior not present in the spec.

## Result Review Requirement

After implementation and validation, write:

```text
docs/reviews/mbt_crate_short_names/mbt_crate_short_names_result_review.md
```

The result review must record:

- changed package names;
- generated artifacts regenerated;
- commands run;
- command outputs or pass/fail summaries;
- any skipped command as a blocker;
- dirty state note;
- statement that historical docs/evidence preserve old names intentionally.
