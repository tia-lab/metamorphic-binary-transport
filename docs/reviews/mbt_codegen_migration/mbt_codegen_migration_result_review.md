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

# Result Review: MBT Codegen Migration

Status: `IMPLEMENTED_AND_VALIDATED`

Slug: `mbt_codegen_migration`

Spec:

```text
docs/specs/mbt_codegen_migration_SPEC.md
```

Implementation plan:

```text
docs/reviews/mbt_codegen_migration/mbt_codegen_migration_implementation_plan.md
```

## Built

Implemented `crates/codegen` as the core-only MBT schema generator:

- explicit CLI: `--inspect`, `--write`, `--check`;
- explicit schema inputs: repeated `--proto-root`, `--schema`, `--root`,
  `--module`, `--surface core`;
- MBT-only `proto/mathilde/options.proto`;
- descriptor loading through `protoc` and `prost-reflect`;
- core-only schema model;
- deterministic core Rust emitter;
- generated checked access and unsafe trusted access;
- generated row views and archived row wrappers;
- generated dictionary helpers, presence accessors, validation, and checksums;
- temporary generated smoke crate support;
- fixture-based tests with no committed generated schema outputs.

No `crates/core` source file was changed.

## Dependency Evidence

Direct codegen dependencies are:

```text
prost-reflect = { version = "=0.16.4", default-features = false }
thiserror = "=2.0.17"
```

Run evidence:

```text
cargo tree -p metamorphic_binary_transport_codegen
```

Observed:

```text
metamorphic_binary_transport_codegen
├── prost-reflect v0.16.4
└── thiserror v2.0.17
```

Run evidence:

```text
cargo tree -p metamorphic_binary_transport_core
```

Observed:

```text
metamorphic_binary_transport_core
└── thiserror v2.0.17
```

`rkyv` is not a dependency of `crates/codegen`; it appears only in generated
source text and in the temporary smoke crate.

## Source Size Evidence

Run evidence:

```text
wc -l crates/codegen/src/*.rs crates/codegen/src/tests/*.rs proto/mathilde/options.proto
```

Observed:

```text
3260 total
```

Largest source files:

```text
1174 crates/codegen/src/rust_emit.rs
565  crates/codegen/src/descriptor.rs
379  crates/codegen/src/tests/mod.rs
```

## Validation Evidence

Commands run and observed status:

```text
cargo fmt --all --check
```

Passed.

```text
cargo check -p metamorphic_binary_transport_codegen
```

Passed.

```text
cargo test -p metamorphic_binary_transport_codegen
```

Passed:

```text
14 passed; 0 failed
```

```text
cargo clippy -p metamorphic_binary_transport_codegen --all-targets -- -D warnings
```

Passed.

```text
cargo check --workspace
```

Passed.

```text
cargo check --manifest-path target/mbt-codegen-check/smoke/Cargo.toml
```

Passed.

```text
cargo --version
protoc --version
rustfmt --version
```

Observed:

```text
cargo 1.90.0 (840b83a10 2025-07-30)
libprotoc 3.12.4
rustfmt 1.8.0-stable (1159e78c47 2025-09-14)
```

Manual CLI smoke:

```text
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --inspect ...
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --write ...
cargo run -p metamorphic_binary_transport_codegen --bin mbt_codegen -- --check ...
```

Passed. The inspect output included:

```text
module=scalar_v1
schema_id=11
schema_version=1
transport_name=test.fixture.v1
payload_root=true
schema_hash=14846786060390922084
generated_lines=389
```

## Correctness Evidence

The codegen tests cover:

- MBT option extension lookup;
- MBT option file is proto2 and MBT-only;
- scalar, raw string, array, and wide-presence schemas load;
- unsupported field combinations fail before Rust emission;
- dictionary aliases are accepted but excluded from ordinals and core hash;
- projection declarations are accepted but excluded from core generated output;
- schema hash changes for included physical fields;
- schema hash ignores comments;
- generated source is deterministic across repeated runs;
- generated source contains marker, view, rows, archived row, checked access,
  and trusted access APIs;
- generated source excludes forbidden adapter/projection/transponding strings;
- generated fixture compiles in a temporary smoke crate;
- CLI action and argument validation;
- `--check` detects changed generated output.

## Proved Invariants

Proved by code-read and run evidence:

- `.proto + mathilde/options.proto` is the source of truth for generated core
  schema output.
- `crates/codegen` is isolated from runtime core and adapter crates.
- `crates/core` dependency graph remains unchanged.
- Generated core output compiles against `metamorphic_binary_transport_core`.
- Core generation does not emit projection, metamorphose, adapter,
  transponding, or prost sidecar surfaces.
- Checked and trusted access are separate in generated output.
- Generated output is deterministic for tested fixtures.

## Not Proved

This migration did not measure:

- runtime encode/access throughput;
- adapter throughput;
- projection throughput;
- wide production schema compile time;
- production Bars or Primitives generated crate parity.

Those are explicitly outside this migration spec.

## Notes

Temporary validation artifacts were written under:

```text
target/mbt-codegen-check/
crates/codegen/target/mbt-codegen-fixtures/
```

They are generated validation artifacts and are not committed schema outputs.
