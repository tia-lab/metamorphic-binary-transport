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

# SPEC: MBT Core Runtime Migration

## 1. Identification

Slug: `mbt_core_runtime_migration`

Repository: `/home/tia/_DEV/MATHILDE/metamorphic-binary-transport`

Task class: spec authoring.

Research brief:

```text
docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_research_brief.md
```

Source runtime:

```text
/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport
```

## 2. Status

Status: `DRAFT_AWAITING_PEER_AUDIT_V5`

This spec does not authorize implementation.

Implementation may start only after:

1. this runtime migration spec passes peer audit v5;
2. the runtime implementation plan is amended from the rejected verbose crate
   path to the canonical `crates/core` path;
3. the amended implementation plan is explicitly approved.

## 3. Purpose

Migrate the schema-agnostic MBT runtime core from the experiment crate into:

```text
crates/core
```

The migration must preserve the existing envelope and runtime dispatch
architecture while removing experiment-crate pollution from core:

- no schema-specific constants;
- no generated schema code;
- no adapter modules;
- no benchmark modules;
- no CLI/report errors;
- no Arrow/Parquet/protobuf/JSON/compression dependencies;
- no rkyv dependency in core for this slice.

The goal is a small production core that generated schema crates and future
adapter crates can depend on without inheriting unrelated compile surfaces.

## 4. Non-goals

This spec does not:

- implement generated schema crates;
- implement codegen;
- implement protobuf option parsing;
- migrate rkyv row/archive types;
- migrate Bars v1 generated code;
- migrate Primitives or compatibility generated code;
- migrate projections;
- migrate metamorphose JSON/protobuf/CSV;
- migrate transponding;
- migrate Arrow, Arrow IPC, or Parquet adapters;
- migrate compression benchmarks;
- migrate benchmark fixtures;
- redesign the MBT wire header;
- rename the wire magic;
- change response byte semantics;
- make full MBT runtime throughput claims;
- make compile-time improvement claims beyond dependency-tree evidence for
  this core crate.

## 5. Measured Object

The measured object is core runtime extraction:

```text
experiment src/{codec,envelope,error,runtime}.rs
  -> crates/core/src/{codec,envelope,error,runtime}.rs
```

Measured by this spec:

- core crate compile behavior;
- core dependency tree;
- envelope unit correctness;
- checksum unit correctness;
- generic runtime dispatch unit correctness;
- forbidden dependency and forbidden module absence.

Not measured by this spec:

- generated schema encode/access throughput;
- rkyv archive validation throughput;
- adapter throughput;
- full benchmark parity;
- SDK behavior.

## 6. Schema Source Contract

Core runtime must be schema-agnostic.

Core owns the schema identity carrier type:

```rust
pub struct SchemaHeaderSpec {
    pub schema_id: u32,
    pub schema_version: u16,
    pub schema_hash: u64,
}
```

Core does not own any concrete schema.

Forbidden in core:

```text
SCHEMA_ID_MATHILDE_BAR_ROW
SCHEMA_VERSION_V1
PROTO_BYTES
schema_hash()
crate::schema
crate::generated
```

Generated schema crates must later provide their own schema constants and pass
`SchemaHeaderSpec` into core envelope functions.

## 7. Wire and Archive Contract

The core migration must preserve the experiment wire header layout exactly for
the fields owned by core.

Required constants:

```rust
pub const MAGIC: &[u8; 8] = b"MATBT001";
pub const HEADER_LEN: usize = 128;
pub const TRANSPORT_VERSION: u16 = 1;
pub const ENCODING_MBT_RKYV: u16 = 1;
pub const FLAGS_V1: u16 = 0;
pub const FNV_OFFSET: u64 = 0xcbf29ce484222325;
pub const FNV_PRIME: u64 = 0x00000100000001B3;
```

`ENCODING_MBT_RKYV` replaces the experiment spelling
`ENCODING_MATHILDE_RKYV` while preserving the numeric value. No wire byte may
change because of the constant rename.

The default build ID input must remain unchanged in this migration:

```rust
pub const BUILD_ID_INPUT: &str = "mathilde_binary_transport:v1:synthetic_benchmark";
```

The string is retained to preserve current output bytes. A later wire-format
spec may rename or version it, but this migration must not.

Header byte layout:

| Byte range | Field |
| --- | --- |
| `0..8` | magic |
| `8..10` | transport version, little-endian `u16` |
| `10..12` | header length, little-endian `u16` |
| `12..14` | encoding kind, little-endian `u16` |
| `14..16` | flags, little-endian `u16` |
| `16..20` | schema id, little-endian `u32` |
| `20..22` | schema version, little-endian `u16` |
| `22..24` | reserved zero bytes |
| `24..32` | logical proto schema hash, little-endian `u64` |
| `32..40` | build id, little-endian `u64` |
| `40..48` | row count, little-endian `u64` |
| `48..56` | payload length, little-endian `u64` |
| `56..64` | payload checksum, little-endian `u64` |
| `64..128` | reserved zero bytes |

Required public envelope API:

```rust
pub struct TransportHeader { ... }

impl TransportHeader {
    pub fn new_with_schema(
        schema: SchemaHeaderSpec,
        row_count: u64,
        payload_len: u64,
        payload_checksum: u64,
    ) -> Self;
}

pub fn fnv1a64(bytes: &[u8]) -> u64;
pub fn normalized_proto_hash(proto_bytes: &[u8]) -> u64;
pub fn default_build_id() -> u64;
pub fn encode_header(header: &TransportHeader, dst: &mut [u8; HEADER_LEN]);
pub fn decode_header(bytes: &[u8]) -> Result<TransportHeader>;
pub fn validate_header_for_schema(
    header: &TransportHeader,
    payload: &[u8],
    schema: SchemaHeaderSpec,
) -> Result<()>;
pub fn trusted_payload_for_schema(bytes: &[u8], schema: SchemaHeaderSpec) -> Result<&[u8]>;
```

Forbidden envelope API in core:

```rust
TransportHeader::new(...)
validate_header(...)
schema_hash(...)
```

Those experiment helpers are schema-specific and must not exist in the new
core.

Archive contract:

- core validates envelope identity, payload length, and payload checksum;
- generated schema crates validate rkyv archive shape and row contracts;
- core does not call `rkyv::access` or `rkyv::access_unchecked`;
- core does not depend on `rkyv` in this migration.

## 8. Checked and Trusted Access Contract

Checked path:

```text
decode_header(bytes)
payload = bytes[HEADER_LEN..]
validate_header_for_schema(header, payload, schema)
generated schema validates archive and rows
```

`validate_header_for_schema` must reject:

- unsupported transport version;
- invalid header length;
- unsupported encoding kind;
- invalid flags;
- unknown schema id;
- schema version mismatch;
- schema hash mismatch;
- payload length mismatch;
- payload checksum mismatch.

Trusted path:

```text
trusted_payload_for_schema(bytes, schema)
generated schema unsafe archive access
```

`trusted_payload_for_schema` must validate:

- minimum header length;
- magic;
- reserved header bytes;
- transport version;
- header length;
- encoding kind;
- flags;
- schema id;
- schema version;
- schema hash;
- payload length.

It must not recompute payload checksum. That is the caller contract for
immutable already-validated bytes.

The trusted function returns only the payload byte slice. It does not perform
rkyv access.

## 9. Codegen Contract

No codegen is implemented in this migration.

The core API must be shaped so later generated schema code can use it without
schema-specific branches in core:

```text
generated schema marker
  -> builds SchemaHeaderSpec
  -> calls TransportHeader::new_with_schema
  -> calls encode_header
  -> calls validate_header_for_schema
  -> calls trusted_payload_for_schema
  -> implements MbtSchema
```

Generated schema code must remain outside core.

No generated artifacts are allowed in this spec.

## 10. Crate Boundary Contract

Allowed crate to edit:

```text
crates/core
```

Allowed root file to edit:

```text
Cargo.toml
```

Root `Cargo.toml` may be edited only to add a workspace dependency if the
implementation plan chooses workspace-managed dependency versions.

Allowed Cargo-generated dependency artifact:

```text
Cargo.lock
```

`Cargo.lock` must not be edited manually. It may change only because Cargo
resolves `thiserror = "=2.0.17"` and its required transitive dependencies
during validation commands. Any other lockfile package addition is rejected
unless `cargo tree -p metamorphic_binary_transport_core` proves it belongs to
that dependency chain.

No other crate may be edited.

Core public modules:

```rust
pub mod codec;
pub mod envelope;
pub mod error;
pub mod runtime;

pub use error::Result;
pub use runtime::{access, encode, encode_owned, inspect, BinaryInspection, MbtSchema};
```

Core must not expose:

```text
generated
schema
metamorphose
transponding
arrow
arrow_ipc
parquet
benches
codegen
tests as public modules
```

## 11. Dependency Contract

Allowed core dependency:

```toml
thiserror = "=2.0.17"
```

The implementation plan may place it directly in the core crate or in
`[workspace.dependencies]`, but the resulting `cargo tree -p
metamorphic_binary_transport_core` must show no dependencies other than
`thiserror` and its required transitive dependencies.

If adding this dependency updates `Cargo.lock`, the lockfile update is accepted
only as a Cargo-generated dependency artifact. The implementation plan and
result review must record the resulting dependency tree.

Forbidden core dependencies:

```text
rkyv
serde
serde_json
prost
prost-build
prost-reflect
arrow-array
arrow-buffer
arrow-ipc
arrow-schema
parquet
zstd
itoa
```

Forbidden core error variants:

```text
Arrow
ArrowIpc
Parquet
Compression
Cli
ReportIo
Json
ProtobufDecode
ProtobufEncode
```

Allowed core error variants:

```text
CorruptMagic
UnsupportedTransportVersion(u16)
UnsupportedEncodingKind(u16)
InvalidFlags(u16)
InvalidHeaderLength(u16)
UnknownSchemaId(u32)
SchemaVersionMismatch { observed: u16, expected: u16 }
SchemaHashMismatch { observed: u64, expected: u64 }
PayloadLengthMismatch { observed: usize, expected: u64 }
PayloadChecksumMismatch { observed: u64, expected: u64 }
TruncatedPayload
MalformedArchive(String)
RowCountMismatch { observed: usize, expected: u64 }
InvalidEnumOrdinal { field: &'static str, value: u16 }
InvalidBitmask { field: &'static str, value: u64 }
InvalidTimeGrid(String)
NonFiniteNumeric(&'static str)
InvalidPresenceBits(u64)
InvalidPresenceWord { word: usize, value: u64 }
ResponseTooLarge { observed: usize, cap: usize }
```

The generic `InvalidBitmask` replaces experiment schema-specific mask variants
for future generated code. Existing experiment variants such as
`InvalidPairOrdinal`, `InvalidTimeframeOrdinal`, and `InvalidVenueMask` must
not be migrated into core.

## 12. Determinism Contract

Core must preserve deterministic behavior:

- `fnv1a64` produces stable FNV-1a values;
- `normalized_proto_hash` treats LF and CRLF schema text identically;
- `default_build_id` is deterministic;
- `encode_header` zero-fills reserved bytes;
- `decode_header` rejects non-zero reserved bytes;
- checked validation recomputes payload checksum deterministically;
- trusted payload validation performs no checksum recomputation.

## 13. Failure Contract

Failure behavior must be explicit and typed through `TransportError`.

Core must not:

- panic on malformed input;
- use `unwrap`;
- use `expect`;
- use `panic!`;
- use `todo!`;
- use `unreachable!`;
- silently default invalid values.

Header read helpers must return `TransportError::TruncatedPayload` when the
input is too short.

`decode_header` must reject:

- payload shorter than `HEADER_LEN`;
- wrong magic;
- non-zero bytes in reserved range `22..24`;
- non-zero bytes in reserved range `64..128`.

The error for non-zero reserved bytes may reuse
`TransportError::InvalidHeaderLength` in this migration to preserve the
experiment behavior. A later error-surface spec may introduce a more specific
reserved-byte error.

## 14. Compile-Surface Budget

The implementation must prove:

```text
cargo check -p metamorphic_binary_transport_core
```

passes without warnings.

The implementation must prove by dependency tree that core does not compile:

- generated schemas;
- codegen;
- benches;
- rkyv;
- serde or serde_json;
- prost;
- Arrow;
- Parquet;
- zstd;
- adapter crates.

No numeric compile-time threshold is set for this migration because the core
crate is still small and no generated schema code is present.

## 15. Runtime Performance Budget

No full runtime throughput claim is authorized by this spec.

The implementation must not alter the core algorithms that affect existing
wire/performance baselines:

- FNV-1a update loop;
- header field offsets;
- checked validation ordering;
- trusted validation omission of checksum recomputation;
- response checksum helper.

Full performance parity is required later when generated schema and benchmark
surfaces are migrated. The later parity baseline is the experiment benchmark
evidence in:

```text
/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/docs/bench_results.md
```

This spec records that baseline but does not claim it is proved in the new
workspace yet.

## 16. Correctness Oracle

Core correctness oracle:

1. FNV-1a known vectors match experiment tests:
   - `fnv1a64(b"") == FNV_OFFSET`
   - `fnv1a64(b"a") == 0xaf63dc4c8601ec8c`
2. LF and CRLF schema text produce the same `normalized_proto_hash`.
3. `TransportHeader::new_with_schema` plus `encode_header` plus
   `decode_header` round-trips exactly.
4. Checked validation accepts valid header and payload with matching schema.
5. Checked validation rejects each invalid header field and checksum mismatch.
6. Trusted validation returns payload for valid schema and rejects schema
   mismatch without checksum recomputation.
7. `response_checksum(bytes) == fnv1a64(bytes)`.
8. Generic runtime wrapper functions dispatch to a local test schema
   implementation of `MbtSchema`.

No rkyv archive correctness oracle is included in this spec.

## 17. Benchmark Methodology

No runtime benchmark is required for this core-only migration.

Validation is by correctness tests, dependency checks, and source audits.

The future generated-schema migration spec must add benchmark methodology for:

- encode;
- checked access;
- trusted access;
- inspect;
- projection if included;
- logical payload equality with experiment baseline.

## 18. Test Plan

Create core tests under:

```text
crates/core/src/tests/mod.rs
crates/core/src/tests/test_envelope.rs
crates/core/src/tests/test_codec.rs
crates/core/src/tests/test_runtime.rs
```

Required tests:

- fixed header and FNV vectors;
- normalized proto hash CRLF/LF equality;
- schema-agnostic header round trip;
- corrupt magic rejection;
- reserved bytes rejection;
- unsupported transport version rejection;
- unsupported encoding kind rejection;
- invalid flags rejection;
- schema id mismatch rejection;
- schema version mismatch rejection;
- schema hash mismatch rejection;
- payload length mismatch rejection;
- payload checksum mismatch rejection;
- trusted payload success;
- trusted payload schema mismatch rejection;
- trusted payload length mismatch rejection;
- response checksum delegates to FNV;
- generic `encode`, `encode_owned`, `access`, and `inspect` dispatch through a
  local test-only `MbtSchema`.

Validation commands:

```bash
cargo fmt --check
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

The forbidden dependency checks intentionally do not reject the literal
`ENCODING_MBT_RKYV` constant. They reject forbidden compiled dependencies,
forbidden imports, and forbidden public/private modules. The implementation
plan must repeat the same distinction.

## 19. Code Bindings

Allowed implementation files:

```text
Cargo.toml
Cargo.lock
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
```

`Cargo.toml` may be edited only if the implementation plan uses
workspace-managed dependency versions.

`Cargo.lock` may be updated only by Cargo dependency resolution for
`thiserror = "=2.0.17"` and its required transitive dependencies. It must not
be manually edited.

Forbidden implementation files:

```text
crates/codegen/*
crates/projection/*
crates/metamorphose/*
crates/transponding/*
crates/adapters/*
crates/benches/*
crates/schemas/*
proto/*
```

The implementation plan must bind exact source text or exact source deltas for
every allowed file.

## 20. Generated Artifact Bindings

No generated artifacts are allowed.

Forbidden generated paths:

```text
crates/*/src/generated/*
src/generated/*
target/generated/*
```

If implementation discovers a need for generated code, work must stop and this
spec must be amended and re-audited.

## 21. Review Artifact Bindings

Required artifacts:

```text
docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_research_brief.md
docs/specs/mbt_core_runtime_migration_SPEC.md
docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_peer_audit_v5.md
docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_implementation_plan.md
docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_result_review.md
```

The peer audit is required before implementation planning.

## 22. Implementation Plan Requirement

Before code changes, the implementation plan must bind:

- exact dependency declaration for `thiserror`;
- exact `Cargo.lock` update policy;
- exact core error enum source;
- exact envelope source;
- exact codec source;
- exact runtime source;
- exact tests;
- exact validation commands;
- expected outputs;
- rollback boundary.

The implementation plan must explicitly state that generated schemas,
generated artifacts, codegen, adapters, benchmarks, and benchmark fixtures are
not touched.

## 23. Approval Checklist

Implementation is not approved until all are true:

- required reads complete;
- research brief exists;
- this spec exists;
- peer audit passes;
- implementation plan exists;
- implementation plan is explicitly approved;
- exact files are bound;
- dependency changes are bound;
- tests are bound;
- generated artifacts remain forbidden;
- runtime performance claims remain deferred.

## 24. Open Questions

No open questions remain for this spec draft.

Deferred to later specs:

1. generated schema crate migration;
2. codegen migration;
3. adapter crate migration;
4. full benchmark parity against experiment MBT;
5. wire-format rename/version cleanup, including `BUILD_ID_INPUT`.
