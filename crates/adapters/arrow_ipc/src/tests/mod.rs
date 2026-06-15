use std::sync::Arc;

use arrow_array::{ArrayRef, Float64Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use metamorphic_binary_transport_core::error::TransportError;

use crate::{arrow_ipc_bytes_checksum, record_batch_from_ipc_stream, write_ipc_stream};

fn sample_batch() -> RecordBatch {
    let schema = Arc::new(Schema::new(vec![Field::new(
        "value",
        DataType::Float64,
        false,
    )]));
    let values: ArrayRef = Arc::new(Float64Array::from(vec![1.0, 2.0, 3.0]));
    RecordBatch::try_new(schema, vec![values]).unwrap_or_else(|err| panic!("{err}"))
}

#[test]
fn ipc_stream_roundtrips_one_record_batch() {
    let batch = sample_batch();
    let bytes = write_ipc_stream(&batch, usize::MAX).unwrap_or_else(|err| panic!("{err}"));
    assert!(!bytes.is_empty());

    let decoded = record_batch_from_ipc_stream(&bytes).unwrap_or_else(|err| panic!("{err}"));
    assert_eq!(decoded.num_rows(), batch.num_rows());
    assert_eq!(decoded.num_columns(), batch.num_columns());
}

#[test]
fn ipc_stream_enforces_response_cap() {
    let batch = sample_batch();
    let err = write_ipc_stream(&batch, 1).unwrap_err();
    assert!(matches!(err, TransportError::ResponseTooLarge { .. }));
}

#[test]
fn ipc_checksum_is_stable() {
    let batch = sample_batch();
    let bytes = write_ipc_stream(&batch, usize::MAX).unwrap_or_else(|err| panic!("{err}"));
    assert_eq!(
        arrow_ipc_bytes_checksum(&bytes),
        arrow_ipc_bytes_checksum(&bytes)
    );
}
