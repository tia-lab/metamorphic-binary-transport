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
