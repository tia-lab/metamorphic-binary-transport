//! Public metamorphose traits, dispatch helpers, and trusted-access token.

use metamorphic_binary_transport_core::error::Result;

/// Construction token for trusted trait implementations.
///
/// Public trusted helpers are unsafe; generated trait implementations receive
/// this token so callers cannot invoke trusted paths through safe construction.
#[derive(Debug, Clone, Copy)]
pub struct TrustedUnchecked {
    _private: (),
}

impl TrustedUnchecked {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetamorphoseFormat {
    // Safe dispatch surface for row-format metamorphose outputs.
    Mbt,
    Json,
    Protobuf,
    Csv,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MetamorphoseOutput<'a> {
    // MBT remains borrowed; boundary formats are owned byte buffers.
    Mbt(&'a [u8]),
    Json(Vec<u8>),
    Protobuf(Vec<u8>),
    Csv(Vec<u8>),
}

pub trait MbtMetamorphoseSchema {
    // Implemented by generated schema adapter modules, not hand-written DTOs.
    fn metamorphose_mbt(bytes: &[u8], max_response_bytes: usize) -> Result<&[u8]>;

    fn metamorphose_mbt_trusted_unchecked(
        bytes: &[u8],
        max_response_bytes: usize,
        trusted: TrustedUnchecked,
    ) -> Result<&[u8]>;
}

pub trait JsonMetamorphoseSchema {
    fn metamorphose_json(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>>;

    fn metamorphose_json_trusted_unchecked(
        bytes: &[u8],
        max_response_bytes: usize,
        trusted: TrustedUnchecked,
    ) -> Result<Vec<u8>>;
}

pub trait ProtobufMetamorphoseSchema {
    fn metamorphose_protobuf(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>>;

    fn metamorphose_protobuf_trusted_unchecked(
        bytes: &[u8],
        max_response_bytes: usize,
        trusted: TrustedUnchecked,
    ) -> Result<Vec<u8>>;
}

pub trait CsvMetamorphoseSchema {
    fn metamorphose_csv(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>>;

    fn metamorphose_csv_trusted_unchecked(
        bytes: &[u8],
        max_response_bytes: usize,
        trusted: TrustedUnchecked,
    ) -> Result<Vec<u8>>;
}

pub trait ArrowMetamorphoseSchema {
    type RecordBatch;

    fn metamorphose_arrow(bytes: &[u8], max_response_bytes: usize) -> Result<Self::RecordBatch>;

    fn metamorphose_arrow_trusted_unchecked(
        bytes: &[u8],
        max_response_bytes: usize,
        trusted: TrustedUnchecked,
    ) -> Result<Self::RecordBatch>;
}

pub trait ArrowIpcMetamorphoseSchema {
    fn metamorphose_arrow_ipc(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>>;

    fn metamorphose_arrow_ipc_trusted_unchecked(
        bytes: &[u8],
        max_response_bytes: usize,
        trusted: TrustedUnchecked,
    ) -> Result<Vec<u8>>;
}

pub trait ParquetMetamorphoseSchema {
    fn metamorphose_parquet(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>>;

    fn metamorphose_parquet_trusted_unchecked(
        bytes: &[u8],
        max_response_bytes: usize,
        trusted: TrustedUnchecked,
    ) -> Result<Vec<u8>>;
}

pub fn decode<S>(
    bytes: &[u8],
    format: MetamorphoseFormat,
    max_response_bytes: usize,
) -> Result<MetamorphoseOutput<'_>>
where
    S: MbtMetamorphoseSchema
        + JsonMetamorphoseSchema
        + ProtobufMetamorphoseSchema
        + CsvMetamorphoseSchema,
{
    // Format selection delegates validation and writing to schema-specific adapters.
    match format {
        MetamorphoseFormat::Mbt => Ok(MetamorphoseOutput::Mbt(S::metamorphose_mbt(
            bytes,
            max_response_bytes,
        )?)),
        MetamorphoseFormat::Json => Ok(MetamorphoseOutput::Json(S::metamorphose_json(
            bytes,
            max_response_bytes,
        )?)),
        MetamorphoseFormat::Protobuf => Ok(MetamorphoseOutput::Protobuf(S::metamorphose_protobuf(
            bytes,
            max_response_bytes,
        )?)),
        MetamorphoseFormat::Csv => Ok(MetamorphoseOutput::Csv(S::metamorphose_csv(
            bytes,
            max_response_bytes,
        )?)),
    }
}

pub fn mbt<S: MbtMetamorphoseSchema>(bytes: &[u8], max_response_bytes: usize) -> Result<&[u8]> {
    S::metamorphose_mbt(bytes, max_response_bytes)
}

pub fn json<S: JsonMetamorphoseSchema>(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>> {
    S::metamorphose_json(bytes, max_response_bytes)
}

pub fn protobuf<S: ProtobufMetamorphoseSchema>(
    bytes: &[u8],
    max_response_bytes: usize,
) -> Result<Vec<u8>> {
    S::metamorphose_protobuf(bytes, max_response_bytes)
}

pub fn csv<S: CsvMetamorphoseSchema>(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>> {
    S::metamorphose_csv(bytes, max_response_bytes)
}

pub fn arrow<S: ArrowMetamorphoseSchema>(
    bytes: &[u8],
    max_response_bytes: usize,
) -> Result<S::RecordBatch> {
    S::metamorphose_arrow(bytes, max_response_bytes)
}

pub fn arrow_ipc<S: ArrowIpcMetamorphoseSchema>(
    bytes: &[u8],
    max_response_bytes: usize,
) -> Result<Vec<u8>> {
    S::metamorphose_arrow_ipc(bytes, max_response_bytes)
}

pub fn parquet<S: ParquetMetamorphoseSchema>(
    bytes: &[u8],
    max_response_bytes: usize,
) -> Result<Vec<u8>> {
    S::metamorphose_parquet(bytes, max_response_bytes)
}

/// # Safety
///
/// The caller guarantees that `bytes` were previously accepted by checked
/// access for the same schema and then stored or transported without mutation.
pub unsafe fn mbt_trusted_unchecked<S: MbtMetamorphoseSchema>(
    bytes: &[u8],
    max_response_bytes: usize,
) -> Result<&[u8]> {
    S::metamorphose_mbt_trusted_unchecked(bytes, max_response_bytes, TrustedUnchecked::new())
}

/// # Safety
///
/// The caller guarantees that `bytes` were previously accepted by checked
/// access for the same schema and then stored or transported without mutation.
pub unsafe fn json_trusted_unchecked<S: JsonMetamorphoseSchema>(
    bytes: &[u8],
    max_response_bytes: usize,
) -> Result<Vec<u8>> {
    S::metamorphose_json_trusted_unchecked(bytes, max_response_bytes, TrustedUnchecked::new())
}

/// # Safety
///
/// The caller guarantees that `bytes` were previously accepted by checked
/// access for the same schema and then stored or transported without mutation.
pub unsafe fn protobuf_trusted_unchecked<S: ProtobufMetamorphoseSchema>(
    bytes: &[u8],
    max_response_bytes: usize,
) -> Result<Vec<u8>> {
    S::metamorphose_protobuf_trusted_unchecked(bytes, max_response_bytes, TrustedUnchecked::new())
}

/// # Safety
///
/// The caller guarantees that `bytes` were previously accepted by checked
/// access for the same schema and then stored or transported without mutation.
pub unsafe fn csv_trusted_unchecked<S: CsvMetamorphoseSchema>(
    bytes: &[u8],
    max_response_bytes: usize,
) -> Result<Vec<u8>> {
    S::metamorphose_csv_trusted_unchecked(bytes, max_response_bytes, TrustedUnchecked::new())
}

/// # Safety
///
/// The caller guarantees that `bytes` were previously accepted by checked
/// access for the same schema and then stored or transported without mutation.
pub unsafe fn arrow_trusted_unchecked<S: ArrowMetamorphoseSchema>(
    bytes: &[u8],
    max_response_bytes: usize,
) -> Result<S::RecordBatch> {
    S::metamorphose_arrow_trusted_unchecked(bytes, max_response_bytes, TrustedUnchecked::new())
}

/// # Safety
///
/// The caller guarantees that `bytes` were previously accepted by checked
/// access for the same schema and then stored or transported without mutation.
pub unsafe fn arrow_ipc_trusted_unchecked<S: ArrowIpcMetamorphoseSchema>(
    bytes: &[u8],
    max_response_bytes: usize,
) -> Result<Vec<u8>> {
    S::metamorphose_arrow_ipc_trusted_unchecked(bytes, max_response_bytes, TrustedUnchecked::new())
}

/// # Safety
///
/// The caller guarantees that `bytes` were previously accepted by checked
/// access for the same schema and then stored or transported without mutation.
pub unsafe fn parquet_trusted_unchecked<S: ParquetMetamorphoseSchema>(
    bytes: &[u8],
    max_response_bytes: usize,
) -> Result<Vec<u8>> {
    S::metamorphose_parquet_trusted_unchecked(bytes, max_response_bytes, TrustedUnchecked::new())
}
