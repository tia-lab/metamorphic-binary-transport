use std::error::Error;
use std::io;

use mbt_core::envelope::HEADER_LEN;
use mbt_core::error::TransportError;
use mbt_schema_test_compatibility::test_compatibility_v1::*;

const MAX_RESPONSE_BYTES: usize = 1_000_000;
const SCHEMA_ID_OFFSET: usize = 16;
const SCHEMA_HASH_OFFSET: usize = 24;

type TestResult = std::result::Result<(), Box<dyn Error>>;

#[test]
fn invalid_schema_version_fails() -> TestResult {
    let mut row = valid_row();
    row.schema_version = SCHEMA_VERSION_VALUE + 1;
    expect_encode_error(
        vec![row],
        TransportError::SchemaVersionMismatch {
            observed: SCHEMA_VERSION_VALUE + 1,
            expected: SCHEMA_VERSION_VALUE,
        },
    )
}

#[test]
fn invalid_dictionary_ordinal_fails() -> TestResult {
    let mut row = valid_row();
    row.status_ordinal = 99;
    expect_encode_error(
        vec![row],
        TransportError::InvalidEnumOrdinal {
            field: "status",
            value: 99,
        },
    )
}

#[test]
fn invalid_bitmask_fails() -> TestResult {
    let mut row = valid_row();
    row.venues_mask = VALID_VENUE_MASK | (1 << 63);
    expect_encode_error(
        vec![row],
        TransportError::InvalidBitmask {
            field: "venues",
            value: VALID_VENUE_MASK | (1 << 63),
        },
    )
}

#[test]
fn nonfinite_f32_fails() -> TestResult {
    let mut row = valid_row();
    row.required_f32 = f32::NAN;
    expect_encode_error(vec![row], TransportError::NonFiniteNumeric("required_f32"))
}

#[test]
fn nonfinite_f64_fails() -> TestResult {
    let mut row = valid_row();
    row.required_f64 = f64::INFINITY;
    expect_encode_error(vec![row], TransportError::NonFiniteNumeric("required_f64"))
}

#[test]
fn absent_optional_scalar_with_value_fails() -> TestResult {
    let mut row = valid_row();
    row.presence_bits &= !PRESENCE_OPTIONAL_I64;
    row.optional_i64 = 10;
    expect_encode_error(vec![row], TransportError::InvalidPresenceBits(0))
}

#[test]
fn absent_nullable_array_with_values_fails() -> TestResult {
    let mut row = valid_row();
    row.presence_bits &= !PRESENCE_NULLABLE_I64_ARRAY;
    row.nullable_i64_array = vec![1];
    expect_encode_error(vec![row], TransportError::InvalidPresenceBits(0))
}

#[test]
fn response_cap_fails() -> TestResult {
    let rows = vec![valid_row()];
    match TestCompatibilityV1::encode(&rows, 1) {
        Err(TransportError::ResponseTooLarge { cap: 1, .. }) => Ok(()),
        Err(err) => Err(io::Error::other(format!("unexpected error: {err:?}")).into()),
        Ok(_) => Err(io::Error::other("expected response cap failure").into()),
    }
}

#[test]
fn corrupt_magic_fails() -> TestResult {
    let mut bytes = valid_bytes()?;
    bytes[0] = b'X';
    expect_access_error(bytes.as_slice(), TransportError::CorruptMagic)
}

#[test]
fn truncated_envelope_fails() -> TestResult {
    let bytes = vec![0_u8; HEADER_LEN - 1];
    expect_access_error(bytes.as_slice(), TransportError::TruncatedPayload)
}

#[test]
fn wrong_schema_id_fails() -> TestResult {
    let mut bytes = valid_bytes()?;
    write_u32_le(&mut bytes, SCHEMA_ID_OFFSET, SCHEMA_ID + 1)?;
    expect_access_error(
        bytes.as_slice(),
        TransportError::UnknownSchemaId(SCHEMA_ID + 1),
    )
}

#[test]
fn wrong_schema_hash_fails() -> TestResult {
    let mut bytes = valid_bytes()?;
    let observed = GENERATED_SCHEMA_HASH + 1;
    write_u64_le(&mut bytes, SCHEMA_HASH_OFFSET, observed)?;
    expect_access_error(
        bytes.as_slice(),
        TransportError::SchemaHashMismatch {
            observed,
            expected: GENERATED_SCHEMA_HASH,
        },
    )
}

fn valid_row() -> TestCompatibilityRowV1 {
    TestCompatibilityRowV1 {
        schema_version: SCHEMA_VERSION_VALUE,
        tenant_ordinal: TENANT_ALPHA,
        entity_ordinal: ENTITY_ENTITY_A,
        close_ms: 1_000,
        status_ordinal: STATUS_ACTIVE,
        optional_status_ordinal: 0,
        venues_mask: 1 << VENUE_BINANCE_BIT,
        required_i64: 1,
        optional_i64: 0,
        required_i32: 2,
        optional_i32: 0,
        required_u32: 3,
        optional_u32: 0,
        required_f64: 4.0,
        optional_f64: 0.0,
        required_f32: 5.0,
        optional_f32: 0.0,
        required_bool: true,
        optional_bool: false,
        required_text: "required".to_string(),
        optional_text: String::new(),
        required_bytes: vec![1, 2, 3],
        optional_bytes: Vec::new(),
        uuid_text: "00000000-0000-0000-0000-000000000001".to_string(),
        jsonb_text: r#"{"ok":true}"#.to_string(),
        timestamptz_text: "2026-01-01T00:00:00Z".to_string(),
        numeric_text: "1.0".to_string(),
        required_i64_array: vec![1],
        nullable_i64_array: Vec::new(),
        required_i32_array: vec![2],
        nullable_i32_array: Vec::new(),
        required_u32_array: vec![3],
        nullable_u32_array: Vec::new(),
        required_f64_array: vec![4.0],
        nullable_f64_array: Vec::new(),
        required_f32_array: vec![5.0],
        nullable_f32_array: Vec::new(),
        presence_bits: 0,
    }
}

fn valid_bytes() -> std::result::Result<Vec<u8>, TransportError> {
    let rows = vec![valid_row()];
    TestCompatibilityV1::encode(&rows, MAX_RESPONSE_BYTES)
}

fn expect_encode_error(rows: Vec<TestCompatibilityRowV1>, expected: TransportError) -> TestResult {
    match TestCompatibilityV1::encode_owned(rows, MAX_RESPONSE_BYTES) {
        Err(err) => assert_transport_error(err, expected),
        Ok(_) => Err(io::Error::other("expected encode failure").into()),
    }
}

fn expect_access_error(bytes: &[u8], expected: TransportError) -> TestResult {
    match TestCompatibilityV1::access(bytes) {
        Err(err) => assert_transport_error(err, expected),
        Ok(_) => Err(io::Error::other("expected access failure").into()),
    }
}

fn assert_transport_error(actual: TransportError, expected: TransportError) -> TestResult {
    if actual == expected {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "unexpected error: actual={actual:?} expected={expected:?}"
        ))
        .into())
    }
}

fn write_u32_le(bytes: &mut [u8], offset: usize, value: u32) -> TestResult {
    let end = offset + 4;
    if bytes.len() < end {
        return Err(io::Error::other("header too short for u32 write").into());
    }
    bytes[offset..end].copy_from_slice(&value.to_le_bytes());
    Ok(())
}

fn write_u64_le(bytes: &mut [u8], offset: usize, value: u64) -> TestResult {
    let end = offset + 8;
    if bytes.len() < end {
        return Err(io::Error::other("header too short for u64 write").into());
    }
    bytes[offset..end].copy_from_slice(&value.to_le_bytes());
    Ok(())
}
