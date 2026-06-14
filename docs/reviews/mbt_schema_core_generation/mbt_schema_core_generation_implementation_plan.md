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

# Implementation Plan: MBT Schema Core Generation

Status: `DRAFT_AWAITING_APPROVAL`

Slug: `mbt_schema_core_generation`

Spec:

```text
docs/specs/mbt_schema_core_generation_SPEC.md
```

Peer audit:

```text
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_peer_audit_v5.md
```

This plan does not authorize implementation until explicitly approved.

## 1. Required Refresh Before Code

Before implementation, reread:

```text
AGENTS.md
docs/invariants/core_invariants.md
docs/protocols/lifecycle_protocol.md
docs/protocols/spec_protocol.md
docs/protocols/implementation_protocol.md
docs/protocols/code_style_protocol.md
docs/protocols/codegen_protocol.md
docs/protocols/testing_benchmark_protocol.md
docs/specs/mbt_schema_core_generation_SPEC.md
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_peer_audit_v5.md
```

Implementation must stop if any file binding below is insufficient.

## 2. Files To Create

Create exactly:

```text
crates/schemas/test_compatibility_core/Cargo.toml
crates/schemas/test_compatibility_core/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto
crates/schemas/test_compatibility_core/src/lib.rs
crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
crates/schemas/test_compatibility_core/tests/test_roundtrip.rs
crates/schemas/test_compatibility_core/tests/test_failures.rs
crates/schemas/test_compatibility_core/tests/test_determinism.rs
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_result_review.md
```

`src/test_compatibility_v1.rs` is generated only by `mbt_codegen`. It must not
be edited by hand.

No additional test support file is allowed by this plan.

## 3. Files To Edit

Edit exactly:

```text
Cargo.toml
Cargo.lock
crates/codegen/src/emit.rs
crates/codegen/src/rust_emit.rs
```

`Cargo.lock` may change only as Cargo-generated dependency state.

The only allowed edit in `crates/codegen/src/emit.rs` is:

```text
rustfmt --edition 2021
  -> rustfmt --edition 2024
```

The only allowed edits in `crates/codegen/src/rust_emit.rs` are archived bool
emission fixes and generated clippy-cleanliness fixes:

```text
emit_archived_checksum_line:
  FieldKind::Bool emits update_bool(checksum, {access})
  not update_bool(checksum, {access}.to_native())

archived_value_access:
  FieldKind::Bool emits {access}
  not {access}.to_native()

emit_bitmask_dictionary:
  VALID_*_MASK emits the OR of dictionary bit constants without leading `0 |`

emit_order_validation:
  key-order validation emits a clippy-clean single condition without changing
  the tuple comparison or InvalidTimeGrid error

encode_owned payload construction:
  use `{rows}` field-init shorthand only when the payload repeated field name
  is exactly `rows`; otherwise keep explicit `{field_name}: rows`
```

No other `rust_emit.rs` branch, helper, API, schema model, descriptor behavior,
wire behavior, checksum algorithm, schema hash behavior, validation rule, or
trusted-access behavior may change.

## 4. Forbidden Edits

Do not edit:

```text
crates/core
crates/projection
crates/metamorphose
crates/transponding
crates/adapters
crates/benches
proto/mathilde/options.proto
```

Do not edit any `crates/codegen` file except:

```text
crates/codegen/src/emit.rs
crates/codegen/src/rust_emit.rs
```

If the current generator cannot produce the schema crate without codegen changes
beyond rustfmt edition alignment, the archived bool fix, and the generated
clippy-cleanliness fixes above, stop and request a spec amendment.

## 5. Workspace Manifest Change

Add one workspace member to root `Cargo.toml`:

```toml
"crates/schemas/test_compatibility_core",
```

The member must be added without reordering unrelated members.

## 6. Schema Crate Manifest

Create:

```text
crates/schemas/test_compatibility_core/Cargo.toml
```

Required contents:

```toml
[package]
name = "metamorphic_binary_transport_schema_test_compatibility"
version.workspace = true
edition.workspace = true
authors.workspace = true
publish = false

[dependencies]
metamorphic_binary_transport_core = { path = "../../core" }
rkyv = "=0.8.16"
```

No dev-dependencies are allowed unless the spec is amended.

## 7. Schema Crate Library

Create:

```text
crates/schemas/test_compatibility_core/src/lib.rs
```

Required contents:

```rust
pub mod test_compatibility_v1;
```

Do not add fixture builders or test helper APIs to the library.

## 8. MBT-Only Proto Derivation

Create:

```text
crates/schemas/test_compatibility_core/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto
```

Derive it from:

```text
/media/Development/MATHILDE/experiments/crates/schema/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto
```

Keep:

- `syntax = "proto3";`
- package `mathilde.binary_transport.test_compatibility.v1`;
- import `mathilde/options.proto`;
- dictionaries `tenant`, `entity`, `status`, and `venue`;
- `TestCompatibilityResponseV1`;
- `TestCompatibilityRowV1`;
- all source field names and field numbers;
- schema identity options;
- `payload_root`;
- `projection` definitions;
- `repeated_payload`;
- `const_u16`;
- `dictionary`;
- `bitmask_dictionary`;
- `presence_bit`;
- `key_part`;
- `key_order`;
- `raw_string`;
- `projection_group`.

Remove:

- `cache_route`;
- `cache_table`;
- `cache_field`;
- `cache_column`;
- `table`;
- `column`;
- any `CACHE_*` value;
- any `POSTGRES_*` value.

Required row field list after stripping non-MBT options:

```text
1  schema_version          const_u16=1
2  tenant                  dictionary=tenant, key_part=true, key_order=1, projection_group=key
3  entity                  dictionary=entity, key_part=true, key_order=2, projection_group=key
4  close_ms                key_part=true, key_order=3, projection_group=key
5  status                  dictionary=status
6  optional_status         dictionary=status, presence_bit=0, projection_group=optional
7  venues                  bitmask_dictionary=venue
8  required_i64            int64
9  optional_i64            presence_bit=1, projection_group=optional
10 required_i32            int32
11 optional_i32            presence_bit=2, projection_group=optional
12 required_u32            uint32
13 optional_u32            presence_bit=3, projection_group=optional
14 required_f64            double
15 optional_f64            presence_bit=4, projection_group=optional
16 required_f32            float
17 optional_f32            presence_bit=5, projection_group=optional
18 required_bool           bool
19 optional_bool           presence_bit=6, projection_group=optional
20 required_text           raw_string=true
21 optional_text           raw_string=true, presence_bit=7, projection_group=optional
22 required_bytes          bytes
23 optional_bytes          presence_bit=8, projection_group=optional
24 uuid_text               raw_string=true
25 jsonb_text              raw_string=true
26 timestamptz_text        raw_string=true
27 numeric_text            raw_string=true
28 required_i64_array      repeated int64
29 nullable_i64_array      repeated int64, presence_bit=9, projection_group=optional
30 required_i32_array      repeated int32
31 nullable_i32_array      repeated int32, presence_bit=10, projection_group=optional
32 required_u32_array      repeated uint32
33 nullable_u32_array      repeated uint32, presence_bit=11, projection_group=optional
34 required_f64_array      repeated double
35 nullable_f64_array      repeated double, presence_bit=12, projection_group=optional
36 required_f32_array      repeated float
37 nullable_f32_array      repeated float, presence_bit=13, projection_group=optional
```

## 9. Codegen Commands

Before rerunning codegen, align generated Rust formatting with the workspace
edition by changing only:

```text
crates/codegen/src/emit.rs
  rustfmt --edition 2021
  -> rustfmt --edition 2024
```

Also apply only the approved archived bool fix:

```text
crates/codegen/src/rust_emit.rs
  emit_archived_checksum_line Bool branch removes .to_native()
  archived_value_access Bool branch removes .to_native()
```

Also apply only the approved generated clippy-cleanliness fixes:

```text
crates/codegen/src/rust_emit.rs
  emit_bitmask_dictionary removes leading `0 |` from VALID_*_MASK
  emit_order_validation emits one clippy-clean condition
  encode_owned payload construction uses `rows` shorthand only when valid
```

Run inspect first:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- \
  --inspect \
  --proto-root crates/schemas/test_compatibility_core/proto \
  --proto-root proto \
  --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto \
  --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 \
  --module test_compatibility_v1 \
  --surface core
```

Then generate:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- \
  --write \
  --proto-root crates/schemas/test_compatibility_core/proto \
  --proto-root proto \
  --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto \
  --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 \
  --module test_compatibility_v1 \
  --surface core \
  --out crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

Then check reproducibility:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- \
  --check \
  --proto-root crates/schemas/test_compatibility_core/proto \
  --proto-root proto \
  --schema mathilde/binary_transport/test_compatibility/v1/all_fields.proto \
  --root mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1 \
  --module test_compatibility_v1 \
  --surface core \
  --out crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```

Expected inspect output must include:

```text
module=test_compatibility_v1
schema_id=40001
schema_version=1
transport_name=mathilde.test_compatibility.v1
payload_root=true
```

The result review must record the generated line count from inspect and from
`wc -l`.

## 10. Test Implementation Plan

### `tests/test_roundtrip.rs`

Implement deterministic field-for-field roundtrip tests.

Rows required:

1. all optional scalar fields present and nullable arrays present/non-empty;
2. all optional scalar fields absent and nullable arrays absent;
3. nullable arrays present but empty;
4. deterministic key ordering across at least two `tenant/entity/close_ms`
   combinations.

Assertions required:

- `encode_rows` succeeds;
- checked `access` succeeds;
- view length equals row count;
- archived row accessors match expected values field by field;
- dictionary string helpers return expected strings where generated;
- arrays collected from accessors match expected values;
- presence state matches expected rows;
- `inspect_bytes` row count and checksums match generated inspection.

Do not compare generated rows with `PartialEq`.

### `tests/test_failures.rs`

Implement failure tests for:

- invalid owned row schema version;
- invalid dictionary ordinal;
- invalid bitmask dictionary mask;
- nonfinite `f32`;
- nonfinite `f64`;
- absent optional scalar with non-default backing value;
- absent nullable array with non-empty backing vector;
- `max_response_bytes` too small;
- corrupt magic;
- truncated envelope;
- schema ID header mutation asserting `TransportError::UnknownSchemaId`;
- schema hash header mutation asserting `TransportError::SchemaHashMismatch`.

Failure assertions must match concrete `TransportError` variants.

### `tests/test_determinism.rs`

Implement deterministic replay tests:

- encoding identical deterministic rows twice yields byte-identical MBT output;
- `inspect_bytes` is stable across repeated calls;
- `mbt_codegen --check` is validated by the command sequence, not from inside
  the Rust test;
- row iteration order remains the input order.

## 11. Validation Commands

Run in this order:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --inspect ...
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write ...
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check ...
wc -l crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
cargo fmt --all --check
cargo check -p metamorphic_binary_transport_codegen
cargo test -p metamorphic_binary_transport_codegen
cargo clippy -p metamorphic_binary_transport_codegen --all-targets -- -D warnings
cargo check -p metamorphic_binary_transport_schema_test_compatibility
/usr/bin/time -v cargo check -p metamorphic_binary_transport_schema_test_compatibility
cargo test -p metamorphic_binary_transport_schema_test_compatibility
cargo clippy -p metamorphic_binary_transport_schema_test_compatibility --all-targets -- -D warnings
cargo tree -p metamorphic_binary_transport_schema_test_compatibility
cargo tree -p metamorphic_binary_transport_core
cargo check --workspace
```

For the first three commands, use the exact full command bodies from section 9.

If `/usr/bin/time` is unavailable, record that in the result review and run the
plain `cargo check` command.

## 12. Evidence To Record

Write:

```text
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_result_review.md
```

Record:

- generated source line count;
- inspect output summary;
- codegen `--check` result;
- codegen crate check, test, and clippy result;
- schema crate check result;
- schema crate test result;
- schema crate clippy result;
- workspace check result;
- dependency tree summary;
- `/usr/bin/time -v` output or reason it was unavailable;
- any failed command and diagnosis.

No runtime throughput claim is allowed.

## 13. Pre-Test Audit

Before validation, inspect the diff and confirm:

- only approved files were created or edited;
- generated file header says `@generated by mbt_codegen`;
- generated file contains no adapter, projection implementation,
  transponding, DB, cache, lookup, or serving code;
- generated file imports only `rkyv` and `metamorphic_binary_transport_core`
  outside `std`;
- schema crate does not depend on codegen;
- tests do not use `PartialEq` row comparison;
- tests do not use `unwrap`, `expect`, or `panic!` in reusable helpers.

## 14. Rollback Boundary

Rollback for this plan is limited to:

```text
crates/schemas/test_compatibility_core/
Cargo.toml
Cargo.lock
crates/codegen/src/emit.rs
crates/codegen/src/rust_emit.rs
docs/reviews/mbt_schema_core_generation/mbt_schema_core_generation_result_review.md
```

Do not modify unrelated crates to make rollback easier.

## 15. Stop Gates

Stop and report if:

- codegen fails on the MBT-only proto;
- generated output requires hand edits;
- schema crate requires dependencies beyond core and rkyv;
- generated row derives require `Debug`, `PartialEq`, or `rkyv::Deserialize`;
- failure tests cannot assert the required concrete `TransportError` variants;
- any codegen source change beyond `crates/codegen/src/emit.rs` rustfmt edition
  alignment, the two archived bool branches in
  `crates/codegen/src/rust_emit.rs`, and the three generated
  clippy-cleanliness fixes in `crates/codegen/src/rust_emit.rs` appears
  necessary;
- any core source change appears necessary;
- validation commands fail after implementation.

## 16. Known Risks

- The generated file may expose a compile-surface issue not seen in temporary
  codegen fixtures.
- The current generator may reject a stripped field combination that was
  accepted in the experiment crate.
- Header mutation tests require exact byte offsets from the current core header
  contract; if the test cannot avoid magic/length corruption, implementation
  must stop and diagnose rather than weaken the failure expectation.

## 17. Approval State

Current state:

```text
NOT APPROVED_FOR_IMPLEMENTATION
```

Implementation may start only after explicit approval of this plan.
