# MBT Projection Direct Writer Compile Surface

## Initial Dirty State

```text
 M crates/codegen/src/config.rs
 M crates/codegen/src/descriptor.rs
 M crates/codegen/src/emit.rs
 M crates/codegen/src/lib.rs
 M crates/codegen/src/model.rs
 M crates/codegen/src/rust_emit.rs
 M crates/codegen/src/tests/mod.rs
 M crates/codegen/src/tests/test_cli.rs
 M crates/codegen/src/tests/test_descriptor.rs
 M crates/codegen/src/tests/test_rust_emit_core.rs
 M crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
?? crates/schemas/test_compatibility_core/tests/test_projection.rs
?? docs/evidence/mbt_projection_direct_writer/
?? docs/reviews/mbt_projection_direct_writer/
?? docs/reviews/mbt_projection_migration/mbt_projection_migration_implementation_plan.md
?? docs/reviews/mbt_projection_migration/mbt_projection_migration_implementation_plan_peer_audit.md
?? docs/reviews/mbt_projection_migration/mbt_projection_migration_implementation_plan_peer_audit_v2.md
?? docs/specs/mbt_projection_direct_writer_SPEC.md
```

## Phase 1 Baseline Commands

### cargo check core

```text
```

### cargo check codegen all targets

```text
```

### cargo check test compatibility schema all targets

```text
```

### test compatibility generated line count

```text
2082 crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
```


## current-owned-row pre-direct-writer compile baseline

```bash
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_schema_bars --all-targets
```
    Checking metamorphic_binary_transport_schema_bars v0.1.0 (/media/Development/MATHILDE/metamorphic-binary-transport/crates/schemas/bars_core)
warning: function `validate_finite_f32` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:299:4
    |
299 | fn validate_finite_f32(field: &'static str, value: f32) -> Result<()> {
    |    ^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` on by default

warning: function `update_bytes` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:428:4
    |
428 | fn update_bytes(mut checksum: u64, bytes: &[u8]) -> u64 {
    |    ^^^^^^^^^^^^

warning: function `update_bool` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:451:4
    |
451 | fn update_bool(checksum: u64, value: bool) -> u64 {
    |    ^^^^^^^^^^^

warning: function `update_i32` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:459:4
    |
459 | fn update_i32(checksum: u64, value: i32) -> u64 {
    |    ^^^^^^^^^^

warning: function `update_u32` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:463:4
    |
463 | fn update_u32(checksum: u64, value: u32) -> u64 {
    |    ^^^^^^^^^^

warning: function `update_f32` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:475:4
    |
475 | fn update_f32(checksum: u64, value: f32) -> u64 {
    |    ^^^^^^^^^^

warning: function `update_i64_array` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:483:4
    |
483 | fn update_i64_array(mut checksum: u64, values: &[i64]) -> u64 {
    |    ^^^^^^^^^^^^^^^^

warning: function `update_i32_array` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:491:4
    |
491 | fn update_i32_array(mut checksum: u64, values: &[i32]) -> u64 {
    |    ^^^^^^^^^^^^^^^^

warning: function `update_u32_array` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:499:4
    |
499 | fn update_u32_array(mut checksum: u64, values: &[u32]) -> u64 {
    |    ^^^^^^^^^^^^^^^^

warning: function `update_f64_array` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:507:4
    |
507 | fn update_f64_array(mut checksum: u64, values: &[f64]) -> u64 {
    |    ^^^^^^^^^^^^^^^^

warning: function `update_f32_array` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:515:4
    |
515 | fn update_f32_array(mut checksum: u64, values: &[f32]) -> u64 {
    |    ^^^^^^^^^^^^^^^^

warning: function `update_archived_i64_array` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:523:4
    |
523 | fn update_archived_i64_array<'a, I>(mut checksum: u64, values: I) -> u64
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `update_archived_i32_array` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:533:4
    |
533 | fn update_archived_i32_array<'a, I>(mut checksum: u64, values: I) -> u64
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `update_archived_u32_array` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:543:4
    |
543 | fn update_archived_u32_array<'a, I>(mut checksum: u64, values: I) -> u64
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `update_archived_f64_array` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:553:4
    |
553 | fn update_archived_f64_array<'a, I>(mut checksum: u64, values: I) -> u64
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `update_archived_f32_array` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:563:4
    |
563 | fn update_archived_f32_array<'a, I>(mut checksum: u64, values: I) -> u64
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `metamorphic_binary_transport_schema_bars` (lib) generated 16 warnings
warning: `metamorphic_binary_transport_schema_bars` (lib test) generated 16 warnings (16 duplicates)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.37s
elapsed=0.44 user=0.35 sys=0.09 max_rss_kb=161664

```bash
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_benches --all-targets
```
warning: function `validate_finite_f32` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:299:4
    |
299 | fn validate_finite_f32(field: &'static str, value: f32) -> Result<()> {
    |    ^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` on by default

warning: function `update_bytes` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:428:4
    |
428 | fn update_bytes(mut checksum: u64, bytes: &[u8]) -> u64 {
    |    ^^^^^^^^^^^^

warning: function `update_bool` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:451:4
    |
451 | fn update_bool(checksum: u64, value: bool) -> u64 {
    |    ^^^^^^^^^^^

warning: function `update_i32` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:459:4
    |
459 | fn update_i32(checksum: u64, value: i32) -> u64 {
    |    ^^^^^^^^^^

warning: function `update_u32` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:463:4
    |
463 | fn update_u32(checksum: u64, value: u32) -> u64 {
    |    ^^^^^^^^^^

warning: function `update_f32` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:475:4
    |
475 | fn update_f32(checksum: u64, value: f32) -> u64 {
    |    ^^^^^^^^^^

warning: function `update_i64_array` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:483:4
    |
483 | fn update_i64_array(mut checksum: u64, values: &[i64]) -> u64 {
    |    ^^^^^^^^^^^^^^^^

warning: function `update_i32_array` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:491:4
    |
491 | fn update_i32_array(mut checksum: u64, values: &[i32]) -> u64 {
    |    ^^^^^^^^^^^^^^^^

warning: function `update_u32_array` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:499:4
    |
499 | fn update_u32_array(mut checksum: u64, values: &[u32]) -> u64 {
    |    ^^^^^^^^^^^^^^^^

warning: function `update_f64_array` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:507:4
    |
507 | fn update_f64_array(mut checksum: u64, values: &[f64]) -> u64 {
    |    ^^^^^^^^^^^^^^^^

warning: function `update_f32_array` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:515:4
    |
515 | fn update_f32_array(mut checksum: u64, values: &[f32]) -> u64 {
    |    ^^^^^^^^^^^^^^^^

warning: function `update_archived_i64_array` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:523:4
    |
523 | fn update_archived_i64_array<'a, I>(mut checksum: u64, values: I) -> u64
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `update_archived_i32_array` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:533:4
    |
533 | fn update_archived_i32_array<'a, I>(mut checksum: u64, values: I) -> u64
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `update_archived_u32_array` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:543:4
    |
543 | fn update_archived_u32_array<'a, I>(mut checksum: u64, values: I) -> u64
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `update_archived_f64_array` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:553:4
    |
553 | fn update_archived_f64_array<'a, I>(mut checksum: u64, values: I) -> u64
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `update_archived_f32_array` is never used
   --> crates/schemas/bars_core/src/bars_v1.rs:563:4
    |
563 | fn update_archived_f32_array<'a, I>(mut checksum: u64, values: I) -> u64
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `metamorphic_binary_transport_schema_bars` (lib) generated 16 warnings
    Checking metamorphic_binary_transport_benches v0.1.0 (/media/Development/MATHILDE/metamorphic-binary-transport/crates/benches)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
elapsed=0.28 user=0.28 sys=0.18 max_rss_kb=125076

```bash
wc -l crates/schemas/bars_core/src/bars_v1.rs
```
1942 crates/schemas/bars_core/src/bars_v1.rs

## Final Direct-Writer Checkpoint

This checkpoint was recorded after direct projection helper cleanup, schema
regeneration, and formatting.

```bash
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_core
```

```text
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
elapsed=0.07 user=0.05 sys=0.02 max_rss_kb=28752
```

```bash
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_codegen --all-targets
```

```text
Checking metamorphic_binary_transport_codegen v0.1.0 (/media/Development/MATHILDE/metamorphic-binary-transport/crates/codegen)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
elapsed=0.34 user=0.42 sys=0.19 max_rss_kb=142668
```

```bash
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_schema_test_compatibility --all-targets
```

```text
Checking metamorphic_binary_transport_schema_test_compatibility v0.1.0 (/media/Development/MATHILDE/metamorphic-binary-transport/crates/schemas/test_compatibility_core)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.56s
elapsed=0.64 user=0.94 sys=0.31 max_rss_kb=170368
```

```bash
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_schema_bars --all-targets
```

```text
Checking metamorphic_binary_transport_schema_bars v0.1.0 (/media/Development/MATHILDE/metamorphic-binary-transport/crates/schemas/bars_core)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
elapsed=0.51 user=0.67 sys=0.20 max_rss_kb=159836
```

```bash
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_benches --all-targets
```

```text
Checking metamorphic_binary_transport_benches v0.1.0 (/media/Development/MATHILDE/metamorphic-binary-transport/crates/benches)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
elapsed=0.31 user=0.36 sys=0.17 max_rss_kb=129900
```

```bash
wc -l crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs crates/schemas/bars_core/src/bars_v1.rs
```

```text
2394 crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
2032 crates/schemas/bars_core/src/bars_v1.rs
4426 total
```

Dependency containment:

```bash
cargo tree -p metamorphic_binary_transport_schema_bars
cargo tree -p metamorphic_binary_transport_schema_test_compatibility
cargo tree -p metamorphic_binary_transport_benches
```

Observed dependency surface:

- schema crates depend on `metamorphic_binary_transport_core` and `rkyv`.
- benchmark crate depends on `metamorphic_binary_transport_core`,
  `metamorphic_binary_transport_schema_bars`, and
  `metamorphic_binary_transport_schema_test_compatibility`.
- no Arrow, Parquet, CSV, JSON adapter, or protobuf adapter dependency is pulled
  into either generated schema crate.

Full raw dependency tree output is recorded in:

```text
docs/evidence/mbt_projection_direct_writer/dependency_trees.md
```

## Final Post-Iterator-Size-Hint Checkpoint

This checkpoint was recorded after adding generated iterator `size_hint`
delegation for direct projection row iterators, regenerating schemas, and
rerunning tests.

```bash
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_core
```

```text
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
elapsed=0.08 user=0.06 sys=0.02 max_rss_kb=28528
```

```bash
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_codegen --all-targets
```

```text
Checking metamorphic_binary_transport_codegen v0.1.0 (/media/Development/MATHILDE/metamorphic-binary-transport/crates/codegen)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
elapsed=0.30 user=0.33 sys=0.17 max_rss_kb=129876
```

```bash
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_schema_test_compatibility --all-targets
```

```text
Checking metamorphic_binary_transport_schema_test_compatibility v0.1.0 (/media/Development/MATHILDE/metamorphic-binary-transport/crates/schemas/test_compatibility_core)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.93s
elapsed=1.00 user=0.98 sys=0.29 max_rss_kb=171908
```

```bash
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_schema_bars --all-targets
```

```text
Checking metamorphic_binary_transport_schema_bars v0.1.0 (/media/Development/MATHILDE/metamorphic-binary-transport/crates/schemas/bars_core)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.40s
elapsed=0.47 user=0.62 sys=0.20 max_rss_kb=162200
```

```bash
/usr/bin/time -f 'elapsed=%e user=%U sys=%S max_rss_kb=%M' cargo check -p metamorphic_binary_transport_benches --all-targets
```

```text
Checking metamorphic_binary_transport_benches v0.1.0 (/media/Development/MATHILDE/metamorphic-binary-transport/crates/benches)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.06s
elapsed=1.12 user=0.22 sys=0.17 max_rss_kb=110536
```

```bash
wc -l crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs crates/schemas/bars_core/src/bars_v1.rs
```

```text
2400 crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs
2038 crates/schemas/bars_core/src/bars_v1.rs
4438 total
```
