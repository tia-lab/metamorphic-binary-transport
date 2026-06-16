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
    // Expected schema identity used by checked and trusted envelope gates.
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
    // Decode only the fixed header before any payload archive access.
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

    // Checked access validates payload integrity before rkyv archive access.
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
    // Trusted access checks identity and length; payload integrity is the caller contract.
    validate_identity_and_len(&header, payload, schema)?;
    Ok(payload)
}

fn validate_identity_and_len(
    header: &TransportHeader,
    payload: &[u8],
    schema: SchemaHeaderSpec,
) -> Result<()> {
    // Shared checked/trusted gate for envelope identity and declared payload length.
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
