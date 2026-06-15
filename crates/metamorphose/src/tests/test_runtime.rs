use metamorphic_binary_transport_core::error::Result;

use crate::{
    ArrowIpcMetamorphoseSchema, ArrowMetamorphoseSchema, CsvMetamorphoseSchema,
    JsonMetamorphoseSchema, MbtMetamorphoseSchema, MetamorphoseFormat, MetamorphoseOutput,
    ParquetMetamorphoseSchema, ProtobufMetamorphoseSchema, decode,
};

struct TestSchema;

impl MbtMetamorphoseSchema for TestSchema {
    fn metamorphose_mbt(bytes: &[u8], _max_response_bytes: usize) -> Result<&[u8]> {
        Ok(bytes)
    }

    fn metamorphose_mbt_trusted_unchecked(
        bytes: &[u8],
        _max_response_bytes: usize,
        _trusted: crate::runtime::TrustedUnchecked,
    ) -> Result<&[u8]> {
        Ok(bytes)
    }
}

impl JsonMetamorphoseSchema for TestSchema {
    fn metamorphose_json(_bytes: &[u8], _max_response_bytes: usize) -> Result<Vec<u8>> {
        Ok(b"json".to_vec())
    }

    fn metamorphose_json_trusted_unchecked(
        _bytes: &[u8],
        _max_response_bytes: usize,
        _trusted: crate::runtime::TrustedUnchecked,
    ) -> Result<Vec<u8>> {
        Ok(b"json".to_vec())
    }
}

impl ProtobufMetamorphoseSchema for TestSchema {
    fn metamorphose_protobuf(_bytes: &[u8], _max_response_bytes: usize) -> Result<Vec<u8>> {
        Ok(b"protobuf".to_vec())
    }

    fn metamorphose_protobuf_trusted_unchecked(
        _bytes: &[u8],
        _max_response_bytes: usize,
        _trusted: crate::runtime::TrustedUnchecked,
    ) -> Result<Vec<u8>> {
        Ok(b"protobuf".to_vec())
    }
}

impl CsvMetamorphoseSchema for TestSchema {
    fn metamorphose_csv(_bytes: &[u8], _max_response_bytes: usize) -> Result<Vec<u8>> {
        Ok(b"csv".to_vec())
    }

    fn metamorphose_csv_trusted_unchecked(
        _bytes: &[u8],
        _max_response_bytes: usize,
        _trusted: crate::runtime::TrustedUnchecked,
    ) -> Result<Vec<u8>> {
        Ok(b"csv".to_vec())
    }
}

impl ArrowMetamorphoseSchema for TestSchema {
    type RecordBatch = &'static str;

    fn metamorphose_arrow(_bytes: &[u8], _max_response_bytes: usize) -> Result<Self::RecordBatch> {
        Ok("arrow")
    }

    fn metamorphose_arrow_trusted_unchecked(
        _bytes: &[u8],
        _max_response_bytes: usize,
        _trusted: crate::runtime::TrustedUnchecked,
    ) -> Result<Self::RecordBatch> {
        Ok("arrow")
    }
}

impl ArrowIpcMetamorphoseSchema for TestSchema {
    fn metamorphose_arrow_ipc(_bytes: &[u8], _max_response_bytes: usize) -> Result<Vec<u8>> {
        Ok(b"ipc".to_vec())
    }

    fn metamorphose_arrow_ipc_trusted_unchecked(
        _bytes: &[u8],
        _max_response_bytes: usize,
        _trusted: crate::runtime::TrustedUnchecked,
    ) -> Result<Vec<u8>> {
        Ok(b"ipc".to_vec())
    }
}

impl ParquetMetamorphoseSchema for TestSchema {
    fn metamorphose_parquet(_bytes: &[u8], _max_response_bytes: usize) -> Result<Vec<u8>> {
        Ok(b"parquet".to_vec())
    }

    fn metamorphose_parquet_trusted_unchecked(
        _bytes: &[u8],
        _max_response_bytes: usize,
        _trusted: crate::runtime::TrustedUnchecked,
    ) -> Result<Vec<u8>> {
        Ok(b"parquet".to_vec())
    }
}

#[test]
fn row_format_decode_dispatches_to_selected_schema_trait() -> Result<()> {
    assert_eq!(
        decode::<TestSchema>(b"mbt", MetamorphoseFormat::Mbt, 64)?,
        MetamorphoseOutput::Mbt(b"mbt")
    );
    assert_eq!(
        decode::<TestSchema>(b"mbt", MetamorphoseFormat::Json, 64)?,
        MetamorphoseOutput::Json(b"json".to_vec())
    );
    assert_eq!(
        decode::<TestSchema>(b"mbt", MetamorphoseFormat::Protobuf, 64)?,
        MetamorphoseOutput::Protobuf(b"protobuf".to_vec())
    );
    assert_eq!(
        decode::<TestSchema>(b"mbt", MetamorphoseFormat::Csv, 64)?,
        MetamorphoseOutput::Csv(b"csv".to_vec())
    );
    Ok(())
}

#[test]
fn format_specific_helpers_dispatch_to_selected_schema_trait() -> Result<()> {
    assert_eq!(crate::mbt::<TestSchema>(b"mbt", 64)?, b"mbt");
    assert_eq!(crate::json::<TestSchema>(b"mbt", 64)?, b"json");
    assert_eq!(crate::protobuf::<TestSchema>(b"mbt", 64)?, b"protobuf");
    assert_eq!(crate::csv::<TestSchema>(b"mbt", 64)?, b"csv");
    assert_eq!(crate::arrow::<TestSchema>(b"mbt", 64)?, "arrow");
    assert_eq!(crate::arrow_ipc::<TestSchema>(b"mbt", 64)?, b"ipc");
    assert_eq!(crate::parquet::<TestSchema>(b"mbt", 64)?, b"parquet");
    Ok(())
}
