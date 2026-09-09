# Implementation Plan: MBT Core Runtime Migration

Status: `AWAITING_APPROVAL`

Spec:

```text
docs/specs/mbt_core_runtime_migration_SPEC.md
```

Peer audit:

```text
docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_peer_audit_v5.md
```

## 1. Gate Result

Implementation must not start until this amended plan is explicitly approved.

The implementation requires adding:

```toml
thiserror = "=2.0.17"
```

to:

```text
crates/core/Cargo.toml
```

That dependency change will update:

```text
Cargo.lock
```

The v5 spec binds `Cargo.lock` as a Cargo-generated dependency artifact. The
lockfile must not be manually edited. It may change only because Cargo resolves
`thiserror = "=2.0.17"` and its required transitive dependencies.

## 2. Required Reads

Completed for this plan:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/implementation_protocol.md`
- `docs/protocols/code_style_protocol.md`
- `docs/protocols/spec_protocol.md`
- `docs/specs/mbt_core_runtime_migration_SPEC.md`
- `docs/reviews/mbt_core_runtime_migration/mbt_core_runtime_migration_peer_audit_v5.md`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/envelope.rs`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/runtime.rs`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/codec.rs`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/error.rs`
- `/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/src/generated/bars_v1.rs`

## 3. Lockfile Policy

The v5 spec permits `Cargo.lock` as an allowed dependency-lock artifact.
Implementation must follow this policy:

- `Cargo.lock` must not be manually edited;
- it may change only because `cargo check`, `cargo test`, or `cargo clippy`
  resolves `thiserror = "=2.0.17"` and its required transitive dependencies;
- no other lockfile package additions are accepted except what `cargo tree -p
metamorphic_binary_transport_core` proves belongs to `thiserror`.

## 4. Approved Scope

Files to replace:

```text
crates/core/Cargo.toml
crates/core/src/lib.rs
crates/core/src/codec.rs
crates/core/src/envelope.rs
crates/core/src/error.rs
crates/core/src/runtime.rs
crates/core/src/tests/mod.rs
```

Files to create:

```text
crates/core/src/tests/test_codec.rs
crates/core/src/tests/test_envelope.rs
crates/core/src/tests/test_runtime.rs
```

Cargo-generated dependency artifact:

```text
Cargo.lock
```

Files not touched:

```text
Cargo.toml
crates/codegen/*
crates/projection/*
crates/metamorphose/*
crates/transponding/*
crates/adapters/*
crates/benches/*
crates/schemas/*
proto/*
src/generated/*
crates/*/src/generated/*
```

No generated artifacts are created.

## 5. Dependency Binding

Use a direct core dependency only:

```toml
[dependencies]
thiserror = "=2.0.17"
```

Root `Cargo.toml` remains unchanged. The dependency is not placed in
`[workspace.dependencies]` in this migration because only the core crate needs
it.

Expected dependency proof:

```bash
cargo tree -p metamorphic_binary_transport_core
```

Expected allowed dependency shape:

```text
metamorphic_binary_transport_core v0.1.0
└── thiserror v2.0.17
    └── thiserror-impl v2.0.17 (proc-macro)
        ├── proc-macro2 ...
        ├── quote ...
        └── syn ...
```

No `rkyv`, `serde`, `serde_json`, `prost`, Arrow, Parquet, zstd, codegen,
generated schema, or benchmark dependency is allowed.

## 6. Exact Source Bindings

### `crates/core/Cargo.toml`

Replace with:

```toml
[package]
name = "metamorphic_binary_transport_core"
version.workspace = true
edition.workspace = true
authors.workspace = true
publish = false

[dependencies]
thiserror = "=2.0.17"
```

### `crates/core/src/lib.rs`

Replace with:

```rust
//! Schema-agnostic MBT core runtime.
//!
//! This crate owns only the envelope, checksum, error, and dispatch contracts.

pub mod codec;
pub mod envelope;
pub mod error;
pub mod runtime;

pub use error::Result;
pub use runtime::{access, encode, encode_owned, inspect, BinaryInspection, MbtSchema};

#[cfg(test)]
mod tests;
```

### `crates/core/src/error.rs`

Replace with:

```rust
use thiserror::Error;

pub type Result<T> = std::result::Result<T, TransportError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TransportError {
    #[error("corrupt magic")]
    CorruptMagic,
    #[error("unsupported transport version {0}")]
    UnsupportedTransportVersion(u16),
    #[error("unsupported encoding kind {0}")]
    UnsupportedEncodingKind(u16),
    #[error("invalid flags {0}")]
    InvalidFlags(u16),
    #[error("invalid header length {0}")]
    InvalidHeaderLength(u16),
    #[error("unknown schema id {0}")]
    UnknownSchemaId(u32),
    #[error("schema version mismatch {observed}, expected {expected}")]
    SchemaVersionMismatch { observed: u16, expected: u16 },
    #[error("schema hash mismatch {observed}, expected {expected}")]
    SchemaHashMismatch { observed: u64, expected: u64 },
    #[error("payload length mismatch {observed}, expected {expected}")]
    PayloadLengthMismatch { observed: usize, expected: u64 },
    #[error("payload checksum mismatch {observed}, expected {expected}")]
    PayloadChecksumMismatch { observed: u64, expected: u64 },
    #[error("truncated payload")]
    TruncatedPayload,
    #[error("malformed archive: {0}")]
    MalformedArchive(String),
    #[error("row count mismatch {observed}, expected {expected}")]
    RowCountMismatch { observed: usize, expected: u64 },
    #[error("invalid enum ordinal field={field} value={value}")]
    InvalidEnumOrdinal { field: &'static str, value: u16 },
    #[error("invalid bitmask field={field} value={value}")]
    InvalidBitmask { field: &'static str, value: u64 },
    #[error("invalid time grid: {0}")]
    InvalidTimeGrid(String),
    #[error("non-finite numeric field {0}")]
    NonFiniteNumeric(&'static str),
    #[error("invalid presence bits {0}")]
    InvalidPresenceBits(u64),
    #[error("invalid presence word {word}: {value}")]
    InvalidPresenceWord { word: usize, value: u64 },
    #[error("response bytes {observed} exceed cap {cap}")]
    ResponseTooLarge { observed: usize, cap: usize },
}
```

### `crates/core/src/envelope.rs`

Replace with:

```rust
use crate::error::{Result, TransportError};

pub const MAGIC: &[u8; 8] = b"MATBT001";
pub const HEADER_LEN: usize = 128;
pub const TRANSPORT_VERSION: u16 = 1;
pub const ENCODING_MBT_RKYV: u16 = 1;
pub const FLAGS_V1: u16 = 0;
pub const FNV_OFFSET: u64 = 0xcbf29ce484222325;
pub const FNV_PRIME: u64 = 0x00000100000001B3;
pub const BUILD_ID_INPUT: &str = "mathilde_binary_transport:v1:synthetic_benchmark";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchemaHeaderSpec {
    pub schema_id: u32,
    pub schema_version: u16,
    pub schema_hash: u64,
}

// Transport headers bind schema identity, row count, payload length, and payload checksum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransportHeader {
    pub transport_version: u16,
    pub header_len: u16,
    pub encoding_kind: u16,
    pub flags: u16,
    pub schema_id: u32,
    pub schema_version: u16,
    pub logical_proto_schema_hash: u64,
    pub build_id: u64,
    pub row_count: u64,
    pub payload_len: u64,
    pub payload_checksum: u64,
}

impl TransportHeader {
    pub fn new_with_schema(
        schema: SchemaHeaderSpec,
        row_count: u64,
        payload_len: u64,
        payload_checksum: u64,
    ) -> Self {
        Self {
            transport_version: TRANSPORT_VERSION,
            header_len: HEADER_LEN as u16,
            encoding_kind: ENCODING_MBT_RKYV,
            flags: FLAGS_V1,
            schema_id: schema.schema_id,
            schema_version: schema.schema_version,
            logical_proto_schema_hash: schema.schema_hash,
            build_id: default_build_id(),
            row_count,
            payload_len,
            payload_checksum,
        }
    }
}

pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = FNV_OFFSET;
    for byte in bytes {
        hash = fnv1a64_update(hash, *byte);
    }
    hash
}

pub fn normalized_proto_hash(proto_bytes: &[u8]) -> u64 {
    let mut hash = FNV_OFFSET;
    let mut idx = 0;
    while idx < proto_bytes.len() {
        if proto_bytes[idx] == b'\r' {
            hash = fnv1a64_update(hash, b'\n');
            if idx + 1 < proto_bytes.len() && proto_bytes[idx + 1] == b'\n' {
                idx += 2;
            } else {
                idx += 1;
            }
        } else {
            hash = fnv1a64_update(hash, proto_bytes[idx]);
            idx += 1;
        }
    }
    hash
}

pub fn default_build_id() -> u64 {
    fnv1a64(BUILD_ID_INPUT.as_bytes())
}

// The envelope is a fixed 128-byte little-endian header followed by payload bytes.
pub fn encode_header(header: &TransportHeader, dst: &mut [u8; HEADER_LEN]) {
    dst.fill(0);
    dst[0..8].copy_from_slice(MAGIC);
    put_u16(dst, 8, header.transport_version);
    put_u16(dst, 10, header.header_len);
    put_u16(dst, 12, header.encoding_kind);
    put_u16(dst, 14, header.flags);
    put_u32(dst, 16, header.schema_id);
    put_u16(dst, 20, header.schema_version);
    put_u64(dst, 24, header.logical_proto_schema_hash);
    put_u64(dst, 32, header.build_id);
    put_u64(dst, 40, header.row_count);
    put_u64(dst, 48, header.payload_len);
    put_u64(dst, 56, header.payload_checksum);
}

pub fn decode_header(bytes: &[u8]) -> Result<TransportHeader> {
    if bytes.len() < HEADER_LEN {
        return Err(TransportError::TruncatedPayload);
    }
    if &bytes[0..8] != MAGIC {
        return Err(TransportError::CorruptMagic);
    }

    let reserved_short = read_u16(bytes, 22)?;
    if reserved_short != 0 {
        return Err(TransportError::InvalidHeaderLength(reserved_short));
    }
    if bytes[64..128].iter().any(|byte| *byte != 0) {
        return Err(TransportError::InvalidHeaderLength(HEADER_LEN as u16));
    }

    Ok(TransportHeader {
        transport_version: read_u16(bytes, 8)?,
        header_len: read_u16(bytes, 10)?,
        encoding_kind: read_u16(bytes, 12)?,
        flags: read_u16(bytes, 14)?,
        schema_id: read_u32(bytes, 16)?,
        schema_version: read_u16(bytes, 20)?,
        logical_proto_schema_hash: read_u64(bytes, 24)?,
        build_id: read_u64(bytes, 32)?,
        row_count: read_u64(bytes, 40)?,
        payload_len: read_u64(bytes, 48)?,
        payload_checksum: read_u64(bytes, 56)?,
    })
}

pub fn validate_header_for_schema(
    header: &TransportHeader,
    payload: &[u8],
    schema: SchemaHeaderSpec,
) -> Result<()> {
    validate_identity_and_len(header, payload, schema)?;

    let checksum = fnv1a64(payload);
    if checksum != header.payload_checksum {
        return Err(TransportError::PayloadChecksumMismatch {
            observed: checksum,
            expected: header.payload_checksum,
        });
    }

    Ok(())
}

pub fn trusted_payload_for_schema(bytes: &[u8], schema: SchemaHeaderSpec) -> Result<&[u8]> {
    if bytes.len() < HEADER_LEN {
        return Err(TransportError::TruncatedPayload);
    }

    let header = decode_header(bytes)?;
    let payload = &bytes[HEADER_LEN..];
    validate_identity_and_len(&header, payload, schema)?;
    Ok(payload)
}

fn validate_identity_and_len(
    header: &TransportHeader,
    payload: &[u8],
    schema: SchemaHeaderSpec,
) -> Result<()> {
    if header.transport_version != TRANSPORT_VERSION {
        return Err(TransportError::UnsupportedTransportVersion(
            header.transport_version,
        ));
    }
    if usize::from(header.header_len) != HEADER_LEN {
        return Err(TransportError::InvalidHeaderLength(header.header_len));
    }
    if header.encoding_kind != ENCODING_MBT_RKYV {
        return Err(TransportError::UnsupportedEncodingKind(
            header.encoding_kind,
        ));
    }
    if header.flags != FLAGS_V1 {
        return Err(TransportError::InvalidFlags(header.flags));
    }
    if header.schema_id != schema.schema_id {
        return Err(TransportError::UnknownSchemaId(header.schema_id));
    }
    if header.schema_version != schema.schema_version {
        return Err(TransportError::SchemaVersionMismatch {
            observed: header.schema_version,
            expected: schema.schema_version,
        });
    }
    if header.logical_proto_schema_hash != schema.schema_hash {
        return Err(TransportError::SchemaHashMismatch {
            observed: header.logical_proto_schema_hash,
            expected: schema.schema_hash,
        });
    }
    if payload.len() as u64 != header.payload_len {
        return Err(TransportError::PayloadLengthMismatch {
            observed: payload.len(),
            expected: header.payload_len,
        });
    }

    Ok(())
}

fn fnv1a64_update(mut hash: u64, byte: u8) -> u64 {
    hash ^= u64::from(byte);
    hash.wrapping_mul(FNV_PRIME)
}

fn put_u16(dst: &mut [u8], offset: usize, value: u16) {
    dst[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}

fn put_u32(dst: &mut [u8], offset: usize, value: u32) {
    dst[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn put_u64(dst: &mut [u8], offset: usize, value: u64) {
    dst[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}

fn read_u16(bytes: &[u8], offset: usize) -> Result<u16> {
    if bytes.len() < offset + 2 {
        return Err(TransportError::TruncatedPayload);
    }
    let mut value = [0_u8; 2];
    value.copy_from_slice(&bytes[offset..offset + 2]);
    Ok(u16::from_le_bytes(value))
}

fn read_u32(bytes: &[u8], offset: usize) -> Result<u32> {
    if bytes.len() < offset + 4 {
        return Err(TransportError::TruncatedPayload);
    }
    let mut value = [0_u8; 4];
    value.copy_from_slice(&bytes[offset..offset + 4]);
    Ok(u32::from_le_bytes(value))
}

fn read_u64(bytes: &[u8], offset: usize) -> Result<u64> {
    if bytes.len() < offset + 8 {
        return Err(TransportError::TruncatedPayload);
    }
    let mut value = [0_u8; 8];
    value.copy_from_slice(&bytes[offset..offset + 8]);
    Ok(u64::from_le_bytes(value))
}
```

### `crates/core/src/codec.rs`

Replace with:

```rust
use crate::envelope::fnv1a64;

pub fn response_checksum(bytes: &[u8]) -> u64 {
    fnv1a64(bytes)
}
```

### `crates/core/src/runtime.rs`

Replace with:

```rust
use crate::error::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BinaryInspection {
    pub row_count: usize,
    pub semantic_checksum: u64,
    pub minimal_projection_checksum: u64,
}

pub trait MbtSchema {
    type Row;
    type View<'a>
    where
        Self: 'a;

    fn encode_rows(rows: &[Self::Row], max_response_bytes: usize) -> Result<Vec<u8>>;
    fn encode_owned_rows(rows: Vec<Self::Row>, max_response_bytes: usize) -> Result<Vec<u8>>;
    fn access_view(bytes: &[u8]) -> Result<Self::View<'_>>;
    fn inspect_bytes(bytes: &[u8]) -> Result<BinaryInspection>;
}

pub fn encode<S: MbtSchema>(rows: &[S::Row], max_response_bytes: usize) -> Result<Vec<u8>> {
    S::encode_rows(rows, max_response_bytes)
}

pub fn encode_owned<S: MbtSchema>(rows: Vec<S::Row>, max_response_bytes: usize) -> Result<Vec<u8>> {
    S::encode_owned_rows(rows, max_response_bytes)
}

pub fn access<S: MbtSchema>(bytes: &[u8]) -> Result<S::View<'_>> {
    S::access_view(bytes)
}

pub fn inspect<S: MbtSchema>(bytes: &[u8]) -> Result<BinaryInspection> {
    S::inspect_bytes(bytes)
}
```

### `crates/core/src/tests/mod.rs`

Replace with:

```rust
mod test_codec;
mod test_envelope;
mod test_runtime;
```

### `crates/core/src/tests/test_codec.rs`

Create with:

```rust
use crate::codec::response_checksum;
use crate::envelope::fnv1a64;

#[test]
fn response_checksum_delegates_to_fnv() {
    let bytes = b"mbt-response";
    assert_eq!(response_checksum(bytes), fnv1a64(bytes));
}
```

### `crates/core/src/tests/test_envelope.rs`

Create with:

```rust
use crate::envelope::{
    decode_header, default_build_id, encode_header, fnv1a64, normalized_proto_hash,
    trusted_payload_for_schema, validate_header_for_schema, SchemaHeaderSpec, TransportHeader,
    ENCODING_MBT_RKYV, FLAGS_V1, FNV_OFFSET, HEADER_LEN, MAGIC, TRANSPORT_VERSION,
};
use crate::error::{Result, TransportError};

const TEST_SCHEMA: SchemaHeaderSpec = SchemaHeaderSpec {
    schema_id: 42,
    schema_version: 7,
    schema_hash: 9_001,
};

fn header_for(payload: &[u8]) -> TransportHeader {
    TransportHeader::new_with_schema(
        TEST_SCHEMA,
        3,
        payload.len() as u64,
        fnv1a64(payload),
    )
}

fn encoded_transport(payload: &[u8]) -> Vec<u8> {
    let header = header_for(payload);
    let mut header_bytes = [0_u8; HEADER_LEN];
    encode_header(&header, &mut header_bytes);

    let mut bytes = Vec::with_capacity(HEADER_LEN + payload.len());
    bytes.extend_from_slice(&header_bytes);
    bytes.extend_from_slice(payload);
    bytes
}

#[test]
fn fnv_vectors_match_contract() {
    assert_eq!(fnv1a64(b""), FNV_OFFSET);
    assert_eq!(fnv1a64(b"a"), 0xaf63_dc4c_8601_ec8c);
}

#[test]
fn normalized_proto_hash_treats_lf_and_crlf_equally() {
    assert_eq!(
        normalized_proto_hash(b"message A {\n  uint64 x = 1;\n}\n"),
        normalized_proto_hash(b"message A {\r\n  uint64 x = 1;\r\n}\r\n")
    );
}

#[test]
fn header_round_trips_without_payload() -> Result<()> {
    let payload = b"payload";
    let header = header_for(payload);
    let mut bytes = [0_u8; HEADER_LEN];

    encode_header(&header, &mut bytes);
    assert_eq!(&bytes[0..8], MAGIC);
    assert_eq!(decode_header(&bytes)?, header);
    Ok(())
}

#[test]
fn checked_validation_accepts_valid_payload() -> Result<()> {
    let payload = b"payload";
    let header = header_for(payload);
    validate_header_for_schema(&header, payload, TEST_SCHEMA)
}

#[test]
fn decode_rejects_corrupt_magic() {
    let mut bytes = encoded_transport(b"payload");
    bytes[0] = b'X';

    assert_eq!(decode_header(&bytes).err(), Some(TransportError::CorruptMagic));
}

#[test]
fn decode_rejects_non_zero_reserved_short() {
    let mut bytes = encoded_transport(b"payload");
    bytes[22] = 1;

    assert_eq!(
        decode_header(&bytes).err(),
        Some(TransportError::InvalidHeaderLength(1))
    );
}

#[test]
fn decode_rejects_non_zero_reserved_tail() {
    let mut bytes = encoded_transport(b"payload");
    bytes[64] = 1;

    assert_eq!(
        decode_header(&bytes).err(),
        Some(TransportError::InvalidHeaderLength(HEADER_LEN as u16))
    );
}

#[test]
fn decode_rejects_truncated_payload() {
    assert_eq!(
        decode_header(&[0_u8; HEADER_LEN - 1]).err(),
        Some(TransportError::TruncatedPayload)
    );
}

#[test]
fn checked_validation_rejects_transport_version() {
    let payload = b"payload";
    let mut header = header_for(payload);
    header.transport_version = TRANSPORT_VERSION + 1;

    assert_eq!(
        validate_header_for_schema(&header, payload, TEST_SCHEMA).err(),
        Some(TransportError::UnsupportedTransportVersion(
            TRANSPORT_VERSION + 1
        ))
    );
}

#[test]
fn checked_validation_rejects_header_length() {
    let payload = b"payload";
    let mut header = header_for(payload);
    header.header_len = HEADER_LEN as u16 + 1;

    assert_eq!(
        validate_header_for_schema(&header, payload, TEST_SCHEMA).err(),
        Some(TransportError::InvalidHeaderLength(HEADER_LEN as u16 + 1))
    );
}

#[test]
fn checked_validation_rejects_encoding_kind() {
    let payload = b"payload";
    let mut header = header_for(payload);
    header.encoding_kind = ENCODING_MBT_RKYV + 1;

    assert_eq!(
        validate_header_for_schema(&header, payload, TEST_SCHEMA).err(),
        Some(TransportError::UnsupportedEncodingKind(ENCODING_MBT_RKYV + 1))
    );
}

#[test]
fn checked_validation_rejects_flags() {
    let payload = b"payload";
    let mut header = header_for(payload);
    header.flags = FLAGS_V1 + 1;

    assert_eq!(
        validate_header_for_schema(&header, payload, TEST_SCHEMA).err(),
        Some(TransportError::InvalidFlags(FLAGS_V1 + 1))
    );
}

#[test]
fn checked_validation_rejects_schema_id() {
    let payload = b"payload";
    let mut header = header_for(payload);
    header.schema_id += 1;

    assert_eq!(
        validate_header_for_schema(&header, payload, TEST_SCHEMA).err(),
        Some(TransportError::UnknownSchemaId(TEST_SCHEMA.schema_id + 1))
    );
}

#[test]
fn checked_validation_rejects_schema_version() {
    let payload = b"payload";
    let mut header = header_for(payload);
    header.schema_version += 1;

    assert_eq!(
        validate_header_for_schema(&header, payload, TEST_SCHEMA).err(),
        Some(TransportError::SchemaVersionMismatch {
            observed: TEST_SCHEMA.schema_version + 1,
            expected: TEST_SCHEMA.schema_version,
        })
    );
}

#[test]
fn checked_validation_rejects_schema_hash() {
    let payload = b"payload";
    let mut header = header_for(payload);
    header.logical_proto_schema_hash += 1;

    assert_eq!(
        validate_header_for_schema(&header, payload, TEST_SCHEMA).err(),
        Some(TransportError::SchemaHashMismatch {
            observed: TEST_SCHEMA.schema_hash + 1,
            expected: TEST_SCHEMA.schema_hash,
        })
    );
}

#[test]
fn checked_validation_rejects_payload_length() {
    let payload = b"payload";
    let mut header = header_for(payload);
    header.payload_len += 1;

    assert_eq!(
        validate_header_for_schema(&header, payload, TEST_SCHEMA).err(),
        Some(TransportError::PayloadLengthMismatch {
            observed: payload.len(),
            expected: payload.len() as u64 + 1,
        })
    );
}

#[test]
fn checked_validation_rejects_payload_checksum() {
    let payload = b"payload";
    let mut header = header_for(payload);
    header.payload_checksum += 1;

    assert_eq!(
        validate_header_for_schema(&header, payload, TEST_SCHEMA).err(),
        Some(TransportError::PayloadChecksumMismatch {
            observed: fnv1a64(payload),
            expected: fnv1a64(payload) + 1,
        })
    );
}

#[test]
fn trusted_payload_returns_payload_without_checksum_requirement() -> Result<()> {
    let payload = b"payload";
    let bytes = encoded_transport(payload);

    assert_eq!(trusted_payload_for_schema(&bytes, TEST_SCHEMA)?, payload);
    Ok(())
}

#[test]
fn trusted_payload_rejects_schema_mismatch() {
    let payload = b"payload";
    let bytes = encoded_transport(payload);
    let wrong_schema = SchemaHeaderSpec {
        schema_id: TEST_SCHEMA.schema_id + 1,
        ..TEST_SCHEMA
    };

    assert_eq!(
        trusted_payload_for_schema(&bytes, wrong_schema).err(),
        Some(TransportError::UnknownSchemaId(TEST_SCHEMA.schema_id))
    );
}

#[test]
fn trusted_payload_rejects_length_mismatch() {
    let payload = b"payload";
    let mut header = header_for(payload);
    header.payload_len += 1;
    let mut header_bytes = [0_u8; HEADER_LEN];
    encode_header(&header, &mut header_bytes);

    let mut bytes = Vec::with_capacity(HEADER_LEN + payload.len());
    bytes.extend_from_slice(&header_bytes);
    bytes.extend_from_slice(payload);

    assert_eq!(
        trusted_payload_for_schema(&bytes, TEST_SCHEMA).err(),
        Some(TransportError::PayloadLengthMismatch {
            observed: payload.len(),
            expected: payload.len() as u64 + 1,
        })
    );
}

#[test]
fn default_build_id_is_deterministic() {
    assert_eq!(default_build_id(), default_build_id());
}
```

### `crates/core/src/tests/test_runtime.rs`

Create with:

```rust
use crate::codec::response_checksum;
use crate::error::{Result, TransportError};
use crate::runtime::{access, encode, encode_owned, inspect, BinaryInspection, MbtSchema};

struct TestSchema;

struct Row(u8);

struct View<'a> {
    bytes: &'a [u8],
}

impl MbtSchema for TestSchema {
    type Row = Row;
    type View<'a> = View<'a>;

    fn encode_rows(rows: &[Self::Row], max_response_bytes: usize) -> Result<Vec<u8>> {
        let mut bytes = Vec::with_capacity(rows.len());
        for row in rows {
            bytes.push(row.0);
        }
        if bytes.len() > max_response_bytes {
            return Err(TransportError::ResponseTooLarge {
                observed: bytes.len(),
                cap: max_response_bytes,
            });
        }
        Ok(bytes)
    }

    fn encode_owned_rows(rows: Vec<Self::Row>, max_response_bytes: usize) -> Result<Vec<u8>> {
        Self::encode_rows(&rows, max_response_bytes)
    }

    fn access_view(bytes: &[u8]) -> Result<Self::View<'_>> {
        Ok(View { bytes })
    }

    fn inspect_bytes(bytes: &[u8]) -> Result<BinaryInspection> {
        let checksum = response_checksum(bytes);
        Ok(BinaryInspection {
            row_count: bytes.len(),
            semantic_checksum: checksum,
            minimal_projection_checksum: checksum,
        })
    }
}

#[test]
fn runtime_dispatches_to_schema_implementation() -> Result<()> {
    let rows = [Row(1), Row(2), Row(3)];

    let encoded = encode::<TestSchema>(&rows, 8)?;
    assert_eq!(encoded, vec![1, 2, 3]);

    let owned = encode_owned::<TestSchema>(vec![Row(4), Row(5)], 8)?;
    assert_eq!(owned, vec![4, 5]);

    let view = access::<TestSchema>(&encoded)?;
    assert_eq!(view.bytes, encoded.as_slice());

    let inspected = inspect::<TestSchema>(&encoded)?;
    assert_eq!(
        inspected,
        BinaryInspection {
            row_count: 3,
            semantic_checksum: response_checksum(&encoded),
            minimal_projection_checksum: response_checksum(&encoded),
        }
    );

    Ok(())
}

#[test]
fn runtime_dispatch_propagates_schema_errors() {
    let rows = [Row(1), Row(2), Row(3)];

    assert_eq!(
        encode::<TestSchema>(&rows, 2).err(),
        Some(TransportError::ResponseTooLarge {
            observed: 3,
            cap: 2,
        })
    );
}
```

## 7. Validation Commands

Run after implementation:

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

Expected outputs:

- all `cargo fmt`, `cargo check`, `cargo test`, and `cargo clippy` commands
  exit successfully;
- `cargo check` and `cargo clippy` emit no warnings;
- `cargo test -p metamorphic_binary_transport_core` reports all core tests
  passed;
- forbidden `rg` commands return no matches;
- `cargo tree -p metamorphic_binary_transport_core` contains only core,
  `thiserror v2.0.17`, `thiserror-impl v2.0.17`, and required proc-macro
  transitive dependencies;
- `git diff --check` exits successfully.

## 8. Correctness Proof Mapping

| Spec oracle                  | Planned test                                                                  |
| ---------------------------- | ----------------------------------------------------------------------------- |
| FNV vectors                  | `test_envelope::fnv_vectors_match_contract`                                   |
| LF/CRLF schema hash equality | `test_envelope::normalized_proto_hash_treats_lf_and_crlf_equally`             |
| header round trip            | `test_envelope::header_round_trips_without_payload`                           |
| checked valid header         | `test_envelope::checked_validation_accepts_valid_payload`                     |
| invalid fields and checksum  | individual `checked_validation_rejects_*` tests                               |
| trusted payload success      | `test_envelope::trusted_payload_returns_payload_without_checksum_requirement` |
| trusted schema mismatch      | `test_envelope::trusted_payload_rejects_schema_mismatch`                      |
| trusted length mismatch      | `test_envelope::trusted_payload_rejects_length_mismatch`                      |
| response checksum            | `test_codec::response_checksum_delegates_to_fnv`                              |
| runtime dispatch             | `test_runtime::runtime_dispatches_to_schema_implementation`                   |
| runtime error propagation    | `test_runtime::runtime_dispatch_propagates_schema_errors`                     |

## 9. Rollback Boundary

If validation fails, revert only:

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

No other crate or root workspace metadata should change.

## 10. Known Risks

Risk: `Cargo.lock` mutation could introduce an unapproved dependency.

Mitigation: the v5 spec binds the lockfile as a Cargo-generated artifact only.
The result review must record `cargo tree -p metamorphic_binary_transport_core`
and the forbidden dependency checks must pass.

Risk: exact dependency tree may include transitive proc-macro dependencies from
`thiserror`.

Mitigation: dependency validation checks only forbid non-core runtime and
adapter dependencies. Transitive proc-macro dependencies belonging to
`thiserror` are allowed by spec.

Risk: this migration does not prove full MBT encode/access performance.

Mitigation: no runtime throughput claim is made. Full parity remains deferred
to generated schema and benchmark migration specs.

## 11. Approval State

This plan is implementation-ready only after explicit user approval.

Implementation must not begin before that approval.
