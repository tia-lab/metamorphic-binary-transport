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

# Result Review: MBT Core Runtime Migration

Status: `PASSED_FOR_CORE_RUNTIME_SCOPE`

Slug: `mbt_core_runtime_migration`

## Source Artifacts

Spec:

```text
docs/specs/mbt_core_runtime_migration_SPEC.md
```

Peer audit:

```text
docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_peer_audit_v5.md
```

Implementation plan:

```text
docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_implementation_plan.md
```

## Implemented Scope

Implemented the schema-agnostic MBT core runtime in:

```text
crates/core
```

Changed files:

```text
crates/core/Cargo.toml
crates/core/src/lib.rs
crates/core/src/codec.rs
crates/core/src/envelope.rs
crates/core/src/error.rs
crates/core/src/runtime.rs
crates/core/src/tests/mod.rs
crates/core/src/tests/test_codec.rs
crates/core/src/tests/test_envelope.rs
crates/core/src/tests/test_runtime.rs
Cargo.lock
```

Implemented core surfaces:

- FNV-1a checksum and response checksum helper;
- fixed 128-byte envelope encode/decode;
- schema-agnostic `SchemaHeaderSpec`;
- checked header validation with payload checksum;
- trusted payload validation without checksum recomputation;
- typed `TransportError` surface;
- generic `MbtSchema` runtime dispatch helpers;
- core unit tests for checksum, envelope, validation, trusted payload, and
  runtime dispatch.

Not implemented:

- generated schema crates;
- codegen;
- protobuf option parsing;
- rkyv archive validation;
- projections;
- metamorphose;
- transponding;
- adapters;
- benchmarks.

## Commands Run

```bash
cargo fmt --check
cargo fmt
cargo check --workspace
cargo check -p metamorphic_binary_transport_core
cargo test -p metamorphic_binary_transport_core
cargo clippy -p metamorphic_binary_transport_core --all-targets -- -D warnings
cargo tree -p metamorphic_binary_transport_core
cargo tree -p metamorphic_binary_transport_core | rg -n "thiserror v2\\.0\\.17"
cargo tree -p metamorphic_binary_transport_core | rg -n "thiserror-impl v2\\.0\\.17"
! rg -n "unwrap\\(|expect\\(|panic!|todo!|unreachable!" crates/core/src
! cargo tree -p metamorphic_binary_transport_core | rg -n "rkyv|serde|serde_json|prost|prost-build|prost-reflect|arrow-array|arrow-buffer|arrow-ipc|arrow-schema|parquet|zstd|itoa"
! rg -n "rkyv::|use .*rkyv|extern crate rkyv|serde::|serde_json::|prost::|arrow::|arrow_|parquet::|zstd::" crates/core/src crates/core/Cargo.toml
! rg -n "pub mod (generated|metamorphose|transponding|benches)|mod (generated|metamorphose|transponding|benches)" crates/core/src
git diff --check
```

## Results

`cargo fmt --check` initially failed with rustfmt ordering and line-wrap
differences in newly edited files. `cargo fmt` was run, then
`cargo fmt --check` passed.

`cargo check --workspace` passed after implementation and lockfile generation.
The final rerun after formatting passed:

```text
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
```

`cargo check -p metamorphic_binary_transport_core` passed:

```text
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
```

`cargo test -p metamorphic_binary_transport_core` passed:

```text
running 24 tests
test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

`cargo clippy -p metamorphic_binary_transport_core --all-targets -- -D warnings`
passed:

```text
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.40s
```

Dependency tree:

```text
metamorphic_binary_transport_core v0.1.0
└── thiserror v2.0.17
    └── thiserror-impl v2.0.17 (proc-macro)
        ├── proc-macro2 v1.0.106
        │   └── unicode-ident v1.0.24
        ├── quote v1.0.45
        │   └── proc-macro2 v1.0.106 (*)
        └── syn v2.0.117
            ├── proc-macro2 v1.0.106 (*)
            ├── quote v1.0.45 (*)
            └── unicode-ident v1.0.24
```

Exact dependency pin checks passed:

```text
2:└── thiserror v2.0.17
3:    └── thiserror-impl v2.0.17 (proc-macro)
```

Forbidden runtime/source checks passed with no matches:

- no `unwrap`, `expect`, `panic!`, `todo!`, or `unreachable!` in
  `crates/core/src`;
- no forbidden runtime dependencies in the core dependency tree;
- no forbidden imports for rkyv, serde, prost, Arrow, Parquet, or zstd in core;
- no generated, metamorphose, transponding, or benches modules exposed by core.

`git diff --check` passed.

## Correctness Result

Passed for the core-only correctness oracle:

- FNV known vectors;
- LF/CRLF normalized proto hash equality;
- header encode/decode round trip;
- checked validation success path;
- checked validation rejection paths for transport version, header length,
  encoding kind, flags, schema ID, schema version, schema hash, payload length,
  and payload checksum;
- trusted payload success path;
- trusted payload schema mismatch and length mismatch;
- response checksum delegation;
- generic runtime dispatch and error propagation.

## Compile-Surface Result

Passed for the core-only dependency surface.

The core crate compiles with `thiserror = "=2.0.17"` and its required
proc-macro transitive dependencies only. It does not compile generated schemas,
codegen, benches, rkyv, serde, serde_json, prost, Arrow, Parquet, zstd, or
adapter crates.

## Limits

This result does not prove full MBT encode/access throughput in the new
workspace. Generated schemas, rkyv archive validation, projections,
metamorphose, transponding, adapters, and benchmark parity remain deferred to
later approved specs.

No runtime performance claim is made by this result review.

## Recommendation

Continue to the generated schema/codegen migration phase only after a new
approved spec, peer audit, implementation plan, and explicit approval.
