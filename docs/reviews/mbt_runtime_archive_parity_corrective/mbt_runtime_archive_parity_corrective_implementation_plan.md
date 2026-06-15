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

# Implementation Plan: MBT Runtime Archive Parity Corrective

Slug: `mbt_runtime_archive_parity_corrective`

Status: `PEER_AUDITED_IMPLEMENTATION_AWAITING_APPROVAL`

Spec:

```text
docs/specs/mbt_runtime_archive_parity_corrective_SPEC.md
```

Peer audit:

```text
docs/reviews/mbt_runtime_archive_parity_corrective/mbt_runtime_archive_parity_corrective_peer_audit.md
```

This plan does not authorize implementation until explicitly approved.

## Scope

Fix two proved old-vs-new runtime/archive mismatches:

1. restore old `rkyv` `unaligned` archive layout feature for schema crates;
2. restore old generated trusted access semantics.

## Files To Edit

```text
Cargo.toml
crates/schemas/bars_core/Cargo.toml
crates/schemas/test_compatibility_core/Cargo.toml
crates/codegen/src/rust_emit.rs
crates/codegen/src/tests/test_rust_emit_core.rs
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md
```

No other manually edited files are authorized.

## Generated Files

Regenerate through codegen only:

```text
crates/schemas/bars_core/src/bars_v1.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

No manual generated-file edits are allowed.

## Step 1: Centralize rkyv Dependency

Edit root `Cargo.toml`:

```toml
[workspace.dependencies]
rkyv = { version = "=0.8.16", features = ["unaligned"] }
```

Edit schema crate manifests:

```toml
rkyv = { workspace = true }
```

If `Cargo.lock` changes, stop and report the diff.

## Step 2: Restore Trusted Access Codegen

Edit `crates/codegen/src/rust_emit.rs` so generated
`access_archived_trusted_unchecked` emits:

```rust
let payload = trusted_payload_for_schema(bytes, Self::header_spec())?;
Ok(unsafe { rkyv::access_unchecked::<ArchivedPayload>(payload) })
```

It must not emit:

```rust
let header = decode_header(bytes)?;
validate_archived_payload(archived, header.row_count)?;
```

If `decode_header` becomes unused in generated imports, remove that generated
import in the emitter. Current projection-surface generated files still use
`decode_header` in checked decode helpers, so the import must remain if it is
still used outside trusted access.

Also update archived numeric array checksum helper generation to use
`rkyv::primitive::Archived*` aliases:

```text
rkyv::primitive::ArchivedI64
rkyv::primitive::ArchivedI32
rkyv::primitive::ArchivedU32
rkyv::primitive::ArchivedF64
rkyv::primitive::ArchivedF32
```

Do not emit `rkyv::rend::*_le` or `rkyv::rend::*_ule` in those helper iterator
type bounds.

## Step 3: Update Codegen Unit Test

Edit `crates/codegen/src/tests/test_rust_emit_core.rs` to assert:

- trusted access contains `trusted_payload_for_schema`;
- trusted access contains `rkyv::access_unchecked`;
- trusted access does not contain `validate_archived_payload(archived`;
- trusted access does not contain `let header = decode_header(bytes)?`.

## Step 4: Regenerate Projection-Surface Schema Files

Run:

```text
cargo run -p metamorphic_binary_transport_codegen -- \
  --write \
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
  --write \
  --proto-root proto \
  --proto-root crates/schemas/test_compatibility_core/proto \
  --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto \
  --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 \
  --module test_compatibility_v1 \
  --surface projection \
  --out crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

Then run the same commands with `--check`.

## Step 5: Static Proof Checks

The broad generated files still contain checked access validation by design.
The proof must therefore inspect only the generated
`access_archived_trusted_unchecked` function bodies.

Run:

```text
for file in crates/schemas/bars_core/src/bars_v1.rs crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs; do
  awk '/pub unsafe fn access_archived_trusted_unchecked/{inside=1} inside{print} inside && /^    }$/{inside=0}' "$file" \
    | rg -n "validate_archived_payload|let header = decode_header" && exit 1 || true
done
```

Expected result:

```text
no matches
```

Run:

```text
for file in crates/schemas/bars_core/src/bars_v1.rs crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs; do
  awk '/pub unsafe fn access_archived_trusted_unchecked/{inside=1} inside{print} inside && /^    }$/{inside=0}' "$file" \
    | rg -n "trusted_payload_for_schema|rkyv::access_unchecked"
done
```

Expected result:

```text
matches in generated trusted access functions
```

Run:

```text
cargo tree -p metamorphic_binary_transport_schema_bars --no-default-features -e features | rg "rkyv|unaligned"
cargo tree -p metamorphic_binary_transport_schema_test_compatibility --no-default-features -e features | rg "rkyv|unaligned"
```

Expected result:

```text
rkyv dependency tree includes unaligned feature
```

## Step 6: Validation Commands

Run:

```text
cargo fmt --check
cargo test -p metamorphic_binary_transport_codegen -- --nocapture
cargo check -p metamorphic_binary_transport_schema_bars --no-default-features
cargo check -p metamorphic_binary_transport_schema_test_compatibility --no-default-features
cargo check -p metamorphic_binary_transport_schema_bars --features json,protobuf,csv,arrow_ipc,parquet
cargo test -p metamorphic_binary_transport_schema_bars --features json,protobuf,csv -- --nocapture
cargo test -p metamorphic_binary_transport_schema_test_compatibility --features json,protobuf,csv -- --nocapture
```

If any command fails, stop and diagnose without changing expectations.

## Step 7: Benchmark Evidence

Run the new Bars regression benchmark three times:

```text
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_bars_regression_bench -- --report-dir docs/evidence/mbt_bars_regression_benchmark
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_bars_regression_bench -- --report-dir docs/evidence/mbt_bars_regression_benchmark
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_bars_regression_bench -- --report-dir docs/evidence/mbt_bars_regression_benchmark
```

Do not rerun the old experiments benchmark unless a new approval explicitly
requests it.

## Step 8: Result Review

Update:

```text
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md
```

Required content:

- commands run;
- generated run files;
- `rkyv` `unaligned` proof;
- trusted access grep proof;
- 100k MBT output bytes before and after;
- old reference evidence file used;
- whether runtime parity is proved, failed, or still unproved;
- no speed claim beyond evidence.

## Rollback Boundary

Rollback is limited to:

```text
Cargo.toml
crates/schemas/bars_core/Cargo.toml
crates/schemas/test_compatibility_core/Cargo.toml
crates/codegen/src/rust_emit.rs
crates/codegen/src/tests/test_rust_emit_core.rs
crates/schemas/bars_core/src/bars_v1.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
docs/reviews/mbt_bars_regression_benchmark/mbt_bars_regression_benchmark_corrective_result_review.md
```

No destructive git command is allowed.

## Known Risks

- Restoring old trusted semantics will not help checked access if remaining
  overhead is elsewhere.
- Archive byte size should decrease after `unaligned`, but this remains
  unproved until benchmark evidence exists.
- If Cargo feature resolution does not expose `unaligned`, implementation must
  stop.
