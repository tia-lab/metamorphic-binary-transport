use std::sync::Arc;

use arrow_array::{ArrayRef, Float64Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use bytes::Bytes;
use mbt_core::error::TransportError;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

use crate::{parquet_bytes_checksum, write_uncompressed_parquet};

fn sample_batch() -> RecordBatch {
    let schema = Arc::new(Schema::new(vec![Field::new(
        "value",
        DataType::Float64,
        false,
    )]));
    let values: ArrayRef = Arc::new(Float64Array::from(vec![1.0, 2.0, 3.0]));
    RecordBatch::try_new(schema, vec![values]).unwrap_or_else(|err| panic!("{err}"))
}

fn readback(bytes: &[u8]) -> RecordBatch {
    let reader = Bytes::copy_from_slice(bytes);
    let mut reader = ParquetRecordBatchReaderBuilder::try_new(reader)
        .unwrap_or_else(|err| panic!("{err}"))
        .with_batch_size(usize::MAX)
        .build()
        .unwrap_or_else(|err| panic!("{err}"));
    match reader.next() {
        Some(result) => result.unwrap_or_else(|err| panic!("{err}")),
        None => panic!("parquet readback produced no batches"),
    }
}

#[test]
fn parquet_roundtrips_one_record_batch() {
    let batch = sample_batch();
    let bytes =
        write_uncompressed_parquet(&batch, usize::MAX).unwrap_or_else(|err| panic!("{err}"));
    assert!(bytes.starts_with(b"PAR1"));
    assert!(bytes.ends_with(b"PAR1"));

    let decoded = readback(&bytes);
    assert_eq!(decoded.num_rows(), batch.num_rows());
    assert_eq!(decoded.num_columns(), batch.num_columns());
}

#[test]
fn parquet_enforces_response_cap() {
    let batch = sample_batch();
    let err = write_uncompressed_parquet(&batch, 1).unwrap_err();
    assert!(matches!(err, TransportError::ResponseTooLarge { .. }));
}

#[test]
fn parquet_checksum_is_stable() {
    let batch = sample_batch();
    let bytes =
        write_uncompressed_parquet(&batch, usize::MAX).unwrap_or_else(|err| panic!("{err}"));
    assert_eq!(
        parquet_bytes_checksum(&bytes),
        parquet_bytes_checksum(&bytes)
    );
}
