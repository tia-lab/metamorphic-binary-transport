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

# MBT Crate Short Names Result Review

## Status

Status: implemented and validated.

This review records the implementation of:

- `docs/specs/mbt_crate_short_names_SPEC.md`
- `docs/reviews/mbt_crate_short_names/mbt_crate_short_names_implementation_plan.md`

## Changed Package Names

The current Rust package/import surface was shortened as specified:

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

The codegen binary remains `mbt_codegen`.

## Generated Artifacts

Generated schema files were regenerated through `mbt_codegen`; they were not
hand edited.

Regenerated Bars outputs:

```text
crates/schemas/bars_core/src/bars_v1.rs
crates/schemas/bars_core/src/bars_v1_json.rs
crates/schemas/bars_core/src/bars_v1_protobuf.rs
crates/schemas/bars_core/src/bars_v1_csv.rs
crates/schemas/bars_core/src/bars_v1_transponding.rs
crates/schemas/bars_core/src/bars_v1_arrow.rs
crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs
crates/schemas/bars_core/src/bars_v1_parquet.rs
```

Regenerated test-compatibility outputs:

```text
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_json.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_protobuf.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_csv.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_transponding.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow_ipc.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1_parquet.rs
```

Codegen write/check evidence:

- Bars core write/check passed.
- Bars projection write/check passed.
- Bars JSON, protobuf, CSV, transponding, Arrow, Arrow IPC, and Parquet
  adapter checks passed.
- Test-compatibility core write/check passed.
- Test-compatibility projection write/check passed.
- Test-compatibility JSON, protobuf, CSV, transponding, Arrow, Arrow IPC, and
  Parquet adapter checks passed.

The shared core/projection files must be checked in write/check pair order:
`bars_v1.rs` and `test_compatibility_v1.rs` are intentionally final projection
surface outputs after projection generation. A core-only check after projection
generation reports a diff against the final projection surface and is not a
runtime or codegen failure.

## Inventory

Command:

```bash
cargo make inventory
```

Result:

```text
ok: wrote inventory.md
Build Done in 1.50 seconds.
```

## Static Checks

Command:

```bash
rg -n "metamorphic_binary_transport_[a-z0-9_]+" Cargo.toml crates README.md architecture.md inventory.md docs/architecture/repository_structure.md --glob '!target/**' --glob '!crates/**/target/**' --glob '!docs/reviews/**' --glob '!docs/specs/**' --glob '!docs/evidence/**'
```

Result: no matches.

Command:

```bash
rg -n "name = \"metamorphic_binary_transport|dep:metamorphic_binary_transport|metamorphic_binary_transport_[a-z0-9_]+ =" Cargo.toml crates --glob '*.toml' --glob '!target/**' --glob '!crates/**/target/**'
```

Result: no matches.

Command:

```bash
rg -n "metamorphic_binary_transport_[a-z0-9_]+" crates/codegen/src/emit.rs crates/codegen/src/rust_emit.rs crates/codegen/src/tests
```

Result: no matches.

Historical specs, reviews, and evidence were intentionally excluded from these
current-surface scans. They preserve the old names as historical evidence.

## Formatting

Command:

```bash
cargo fmt --check
```

Initial result: failed with formatting-only drift after import rewrites and
generated output updates.

Command:

```bash
cargo fmt
cargo fmt --check
```

Final result: passed.

## Tests

Commands and results:

```text
cargo test -p mbt_core --all-targets
29 passed

cargo test -p mbt_codegen --all-targets
29 passed

cargo test -p mbt_metamorphose --all-targets
2 passed

cargo test -p mbt_transponding --all-targets
5 passed

cargo test -p mbt_compression --all-targets
9 passed

cargo test -p mbt_adapter_json --all-targets
3 passed

cargo test -p mbt_adapter_protobuf --all-targets
4 passed

cargo test -p mbt_adapter_csv --all-targets
3 passed

cargo test -p mbt_adapter_arrow --all-targets
4 passed

cargo test -p mbt_adapter_arrow_ipc --all-targets
3 passed

cargo test -p mbt_adapter_parquet --all-targets
3 passed

cargo test -p mbt_schema_bars --features json,protobuf,csv,arrow_ipc,parquet --all-targets
7 integration tests passed; lib target had 0 tests

cargo test -p mbt_schema_test_compatibility --features json,protobuf,csv,arrow_ipc,parquet --all-targets
17 integration tests passed; lib target had 0 tests

cargo test -p mbt_benches --all-targets
13 passed; bench binary test harnesses had 0 tests
```

## Compile Evidence

Commands and observed output:

```text
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p mbt_core
elapsed=0.28 user=0.19 sys=0.09 max_rss_kb=118128

/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p mbt_codegen --all-targets
elapsed=0.75 user=0.89 sys=0.23 max_rss_kb=173308

/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p mbt_schema_bars --features json,protobuf,csv,arrow_ipc,parquet
elapsed=0.80 user=0.65 sys=0.15 max_rss_kb=198400

/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p mbt_schema_test_compatibility --features json,protobuf,csv,arrow_ipc,parquet
elapsed=0.91 user=0.82 sys=0.10 max_rss_kb=208968

/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p mbt_benches --all-targets
elapsed=0.11 user=0.06 sys=0.05 max_rss_kb=43956
```

These are warm-cache compile checks from this run. They are evidence that the
renamed package graph compiles; they are not performance benchmarks.

## Dependency Graph Checks

Commands:

```bash
cargo tree -p mbt_core
cargo tree -p mbt_schema_bars --features json,protobuf,csv,arrow_ipc,parquet
cargo tree -p mbt_schema_test_compatibility --features json,protobuf,csv,arrow_ipc,parquet
cargo tree -p mbt_compression
```

Observed containment:

- `mbt_core` depends on `thiserror` only.
- `mbt_compression` depends on `mbt_core` and `zstd`.
- Schema crates depend on `mbt_core`, `rkyv`, and selected `mbt_*` adapter
  crates through enabled features.
- Arrow and Parquet dependency trees remain behind the Arrow IPC and Parquet
  adapter feature surfaces.

## Dirty State Note

The worktree contains the expected modified files from this rename and the new
spec/review artifacts under `docs/reviews/mbt_crate_short_names/` and
`docs/specs/mbt_crate_short_names_SPEC.md`.

An untracked `.github/` directory was present in the worktree during this run.
It was not part of this implementation plan and was not modified by this
change.

## Claims

Proved by this evidence:

- Current package names, dependency keys, and generated imports use the
  approved `mbt_*` names.
- Generated schema files compile and pass the existing generated-schema tests
  with all adapter features enabled.
- Codegen emits the short crate names.
- Core dependency containment is unchanged in kind: core remains separate from
  format adapters.

Not claimed:

- No runtime performance improvement is claimed.
- No wire/archive/schema-hash behavior change is claimed.
- No historical evidence rewrite is claimed.
