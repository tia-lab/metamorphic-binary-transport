use crate::envelope::{
    ENCODING_MBT_RKYV, FLAGS_V1, FNV_OFFSET, HEADER_LEN, MAGIC, SchemaHeaderSpec,
    TRANSPORT_VERSION, TransportHeader, decode_header, default_build_id, encode_header, fnv1a64,
    normalized_proto_hash, trusted_payload_for_schema, validate_header_for_schema,
};
use crate::error::{Result, TransportError};

const TEST_SCHEMA: SchemaHeaderSpec = SchemaHeaderSpec {
    schema_id: 42,
    schema_version: 7,
    schema_hash: 9_001,
};

fn header_for(payload: &[u8]) -> TransportHeader {
    TransportHeader::new_with_schema(TEST_SCHEMA, 3, payload.len() as u64, fnv1a64(payload))
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

    assert_eq!(
        decode_header(&bytes).err(),
        Some(TransportError::CorruptMagic)
    );
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
        Some(TransportError::UnsupportedEncodingKind(
            ENCODING_MBT_RKYV + 1
        ))
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
