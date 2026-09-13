use std::error::Error;

use mbt_adapter_arrow_ipc::record_batch_from_ipc_stream;
use mbt_schema_test_compatibility::test_compatibility_v1::*;

const MAX_RESPONSE_BYTES: usize = 1 << 20;

type TestResult = std::result::Result<(), Box<dyn Error>>;

#[test]
fn no_optional_projection_metamorphoses_required_field_shapes() -> TestResult {
    let source = TestCompatibilityV1::encode(&fixture_rows(), MAX_RESPONSE_BYTES)?;
    let projected = TestCompatibilityV1::project_no_optional(&source, MAX_RESPONSE_BYTES)?;
    TestCompatibilityV1NoOptional::access(&projected)?;

    let json = TestCompatibilityV1NoOptional::metamorphose_json(&projected, MAX_RESPONSE_BYTES)?;
    let trusted_json = unsafe {
        TestCompatibilityV1NoOptional::metamorphose_json_trusted_unchecked(
            &projected,
            MAX_RESPONSE_BYTES,
        )?
    };
    assert_eq!(json, trusted_json);
    let json = std::str::from_utf8(&json)?;
    assert!(json.contains("\"tenant\":\"alpha\""));
    assert!(json.contains("\"required_text\":\"required-alpha\""));
    assert!(json.contains("\"required_bytes\":\"AQID\""));
    assert!(json.contains("\"required_i64_array\":[-1,-2]"));
    assert!(!json.contains("optional_text"));
    assert!(!json.contains("optional_bytes"));

    let csv = TestCompatibilityV1NoOptional::metamorphose_csv(&projected, MAX_RESPONSE_BYTES)?;
    let trusted_csv = unsafe {
        TestCompatibilityV1NoOptional::metamorphose_csv_trusted_unchecked(
            &projected,
            MAX_RESPONSE_BYTES,
        )?
    };
    assert_eq!(csv, trusted_csv);
    let csv = std::str::from_utf8(&csv)?;
    assert!(csv.starts_with("schema_version,tenant,entity,recorded_at_ms,status"));
    assert!(csv.contains("required-alpha"));
    assert!(!csv.contains("optional_text"));

    let protobuf =
        TestCompatibilityV1NoOptional::metamorphose_protobuf(&projected, MAX_RESPONSE_BYTES)?;
    let trusted_protobuf = unsafe {
        TestCompatibilityV1NoOptional::metamorphose_protobuf_trusted_unchecked(
            &projected,
            MAX_RESPONSE_BYTES,
        )?
    };
    assert_eq!(protobuf, trusted_protobuf);
    assert!(!protobuf.is_empty());

    assert_projection_columnar_outputs::<TestCompatibilityV1NoOptional>(
        &projected,
        "required_text",
    )?;
    Ok(())
}

#[test]
fn numeric_only_projection_metamorphoses_nullable_numeric_arrays() -> TestResult {
    let source = TestCompatibilityV1::encode(&fixture_rows(), MAX_RESPONSE_BYTES)?;
    let projected = TestCompatibilityV1::project_numeric_only(&source, MAX_RESPONSE_BYTES)?;
    TestCompatibilityV1NumericOnly::access(&projected)?;

    let json = TestCompatibilityV1NumericOnly::metamorphose_json(&projected, MAX_RESPONSE_BYTES)?;
    let trusted_json = unsafe {
        TestCompatibilityV1NumericOnly::metamorphose_json_trusted_unchecked(
            &projected,
            MAX_RESPONSE_BYTES,
        )?
    };
    assert_eq!(json, trusted_json);
    let json = std::str::from_utf8(&json)?;
    assert!(json.contains("\"optional_i64\":20"));
    assert!(json.contains("\"nullable_i64_array\":[1,2]"));
    assert!(json.contains("\"nullable_f32_array\":[7.5,8.5]"));
    assert!(!json.contains("required_text"));
    assert!(!json.contains("required_bytes"));

    let csv = TestCompatibilityV1NumericOnly::metamorphose_csv(&projected, MAX_RESPONSE_BYTES)?;
    let trusted_csv = unsafe {
        TestCompatibilityV1NumericOnly::metamorphose_csv_trusted_unchecked(
            &projected,
            MAX_RESPONSE_BYTES,
        )?
    };
    assert_eq!(csv, trusted_csv);
    let csv = std::str::from_utf8(&csv)?;
    assert!(csv.contains("nullable_i64_array"));
    assert!(csv.contains("\"[1,2]\""));
    assert!(!csv.contains("required_text"));

    let protobuf =
        TestCompatibilityV1NumericOnly::metamorphose_protobuf(&projected, MAX_RESPONSE_BYTES)?;
    let trusted_protobuf = unsafe {
        TestCompatibilityV1NumericOnly::metamorphose_protobuf_trusted_unchecked(
            &projected,
            MAX_RESPONSE_BYTES,
        )?
    };
    assert_eq!(protobuf, trusted_protobuf);
    assert!(!protobuf.is_empty());

    assert_projection_columnar_outputs::<TestCompatibilityV1NumericOnly>(
        &projected,
        "nullable_i64_array",
    )?;
    Ok(())
}

trait ProjectionAdapters {
    fn arrow(
        bytes: &[u8],
        max_response_bytes: usize,
    ) -> mbt_core::error::Result<mbt_adapter_arrow::ArrowRecordBatch>;
    unsafe fn arrow_trusted(
        bytes: &[u8],
        max_response_bytes: usize,
    ) -> mbt_core::error::Result<mbt_adapter_arrow::ArrowRecordBatch>;
    fn arrow_ipc(bytes: &[u8], max_response_bytes: usize) -> mbt_core::error::Result<Vec<u8>>;
    unsafe fn arrow_ipc_trusted(
        bytes: &[u8],
        max_response_bytes: usize,
    ) -> mbt_core::error::Result<Vec<u8>>;
    fn parquet(bytes: &[u8], max_response_bytes: usize) -> mbt_core::error::Result<Vec<u8>>;
    unsafe fn parquet_trusted(
        bytes: &[u8],
        max_response_bytes: usize,
    ) -> mbt_core::error::Result<Vec<u8>>;
}

impl ProjectionAdapters for TestCompatibilityV1NoOptional {
    fn arrow(
        bytes: &[u8],
        max_response_bytes: usize,
    ) -> mbt_core::error::Result<mbt_adapter_arrow::ArrowRecordBatch> {
        Self::metamorphose_arrow(bytes, max_response_bytes)
    }

    unsafe fn arrow_trusted(
        bytes: &[u8],
        max_response_bytes: usize,
    ) -> mbt_core::error::Result<mbt_adapter_arrow::ArrowRecordBatch> {
        unsafe { Self::metamorphose_arrow_trusted_unchecked(bytes, max_response_bytes) }
    }

    fn arrow_ipc(bytes: &[u8], max_response_bytes: usize) -> mbt_core::error::Result<Vec<u8>> {
        Self::metamorphose_arrow_ipc(bytes, max_response_bytes)
    }

    unsafe fn arrow_ipc_trusted(
        bytes: &[u8],
        max_response_bytes: usize,
    ) -> mbt_core::error::Result<Vec<u8>> {
        unsafe { Self::metamorphose_arrow_ipc_trusted_unchecked(bytes, max_response_bytes) }
    }

    fn parquet(bytes: &[u8], max_response_bytes: usize) -> mbt_core::error::Result<Vec<u8>> {
        Self::metamorphose_parquet(bytes, max_response_bytes)
    }

    unsafe fn parquet_trusted(
        bytes: &[u8],
        max_response_bytes: usize,
    ) -> mbt_core::error::Result<Vec<u8>> {
        unsafe { Self::metamorphose_parquet_trusted_unchecked(bytes, max_response_bytes) }
    }
}

impl ProjectionAdapters for TestCompatibilityV1NumericOnly {
    fn arrow(
        bytes: &[u8],
        max_response_bytes: usize,
    ) -> mbt_core::error::Result<mbt_adapter_arrow::ArrowRecordBatch> {
        Self::metamorphose_arrow(bytes, max_response_bytes)
    }

    unsafe fn arrow_trusted(
        bytes: &[u8],
        max_response_bytes: usize,
    ) -> mbt_core::error::Result<mbt_adapter_arrow::ArrowRecordBatch> {
        unsafe { Self::metamorphose_arrow_trusted_unchecked(bytes, max_response_bytes) }
    }

    fn arrow_ipc(bytes: &[u8], max_response_bytes: usize) -> mbt_core::error::Result<Vec<u8>> {
        Self::metamorphose_arrow_ipc(bytes, max_response_bytes)
    }

    unsafe fn arrow_ipc_trusted(
        bytes: &[u8],
        max_response_bytes: usize,
    ) -> mbt_core::error::Result<Vec<u8>> {
        unsafe { Self::metamorphose_arrow_ipc_trusted_unchecked(bytes, max_response_bytes) }
    }

    fn parquet(bytes: &[u8], max_response_bytes: usize) -> mbt_core::error::Result<Vec<u8>> {
        Self::metamorphose_parquet(bytes, max_response_bytes)
    }

    unsafe fn parquet_trusted(
        bytes: &[u8],
        max_response_bytes: usize,
    ) -> mbt_core::error::Result<Vec<u8>> {
        unsafe { Self::metamorphose_parquet_trusted_unchecked(bytes, max_response_bytes) }
    }
}

fn assert_projection_columnar_outputs<S: ProjectionAdapters>(
    projected: &[u8],
    retained_field: &str,
) -> TestResult {
    let arrow = S::arrow(projected, MAX_RESPONSE_BYTES)?;
    let trusted_arrow = unsafe { S::arrow_trusted(projected, MAX_RESPONSE_BYTES)? };
    assert_eq!(arrow.num_rows(), fixture_rows().len());
    assert_eq!(trusted_arrow.num_rows(), arrow.num_rows());
    assert!(schema_has_field(&arrow, retained_field));

    let arrow_ipc = S::arrow_ipc(projected, MAX_RESPONSE_BYTES)?;
    let trusted_arrow_ipc = unsafe { S::arrow_ipc_trusted(projected, MAX_RESPONSE_BYTES)? };
    assert_eq!(arrow_ipc, trusted_arrow_ipc);
    let decoded_arrow = record_batch_from_ipc_stream(&arrow_ipc)?;
    assert_eq!(decoded_arrow.num_rows(), arrow.num_rows());
    assert!(schema_has_field(&decoded_arrow, retained_field));

    let parquet = S::parquet(projected, MAX_RESPONSE_BYTES)?;
    let trusted_parquet = unsafe { S::parquet_trusted(projected, MAX_RESPONSE_BYTES)? };
    assert_eq!(parquet, trusted_parquet);
    assert!(parquet.starts_with(b"PAR1"));
    assert!(parquet.ends_with(b"PAR1"));
    Ok(())
}

fn schema_has_field(batch: &mbt_adapter_arrow::ArrowRecordBatch, name: &str) -> bool {
    batch
        .schema()
        .fields()
        .iter()
        .any(|field| field.name() == name)
}

fn fixture_rows() -> Vec<TestCompatibilityRowV1> {
    vec![full_row()]
}

fn full_row() -> TestCompatibilityRowV1 {
    TestCompatibilityRowV1 {
        schema_version: SCHEMA_VERSION_VALUE,
        tenant_ordinal: TENANT_ALPHA,
        entity_ordinal: ENTITY_ENTITY_A,
        recorded_at_ms: 1_000,
        status_ordinal: STATUS_ACTIVE,
        optional_status_ordinal: STATUS_PAUSED,
        sites_mask: (1 << SITE_SITE_A_BIT) | (1 << SITE_SITE_B_BIT),
        required_i64: -10,
        optional_i64: 20,
        required_i32: -30,
        optional_i32: 40,
        required_u32: 50,
        optional_u32: 60,
        required_f64: 70.5,
        optional_f64: 80.25,
        required_f32: 90.5,
        optional_f32: 100.25,
        required_bool: true,
        optional_bool: true,
        required_text: "required-alpha".to_string(),
        optional_text: "optional-alpha".to_string(),
        required_bytes: vec![1, 2, 3],
        optional_bytes: vec![4, 5, 6],
        uuid_text: "00000000-0000-0000-0000-000000000001".to_string(),
        jsonb_text: r#"{"kind":"alpha"}"#.to_string(),
        timestamptz_text: "2026-01-01T00:00:00Z".to_string(),
        numeric_text: "123.456".to_string(),
        required_i64_array: vec![-1, -2],
        nullable_i64_array: vec![1, 2],
        required_i32_array: vec![-3, -4],
        nullable_i32_array: vec![3, 4],
        required_u32_array: vec![5, 6],
        nullable_u32_array: vec![7, 8],
        required_f64_array: vec![1.5, 2.5],
        nullable_f64_array: vec![3.5, 4.5],
        required_f32_array: vec![5.5, 6.5],
        nullable_f32_array: vec![7.5, 8.5],
        presence_bits: PRESENCE_ALLOWED_MASK,
    }
}
