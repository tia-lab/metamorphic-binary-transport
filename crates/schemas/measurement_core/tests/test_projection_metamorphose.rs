#![cfg(all(
    feature = "json",
    feature = "csv",
    feature = "protobuf",
    feature = "arrow",
    feature = "arrow_ipc",
    feature = "parquet"
))]
use std::error::Error;

use mbt_adapter_arrow_ipc::record_batch_from_ipc_stream;
use mbt_schema_measurement::measurement_v1::*;

const MAX_RESPONSE_BYTES: usize = 1 << 20;

type TestResult = std::result::Result<(), Box<dyn Error>>;

#[test]
fn without_details_projection_metamorphoses_all_enabled_formats() -> TestResult {
    let source = MeasurementV1::encode(&fixture_rows(), MAX_RESPONSE_BYTES)?;
    let projected = MeasurementV1::project_without_details(&source, MAX_RESPONSE_BYTES)?;
    MeasurementV1WithoutDetails::access(&projected)?;

    let json = MeasurementV1WithoutDetails::metamorphose_json(&projected, MAX_RESPONSE_BYTES)?;
    let trusted_json = unsafe {
        MeasurementV1WithoutDetails::metamorphose_json_trusted_unchecked(
            &projected,
            MAX_RESPONSE_BYTES,
        )?
    };
    assert_eq!(json, trusted_json);
    let json = std::str::from_utf8(&json)?;
    assert!(json.contains("\"started_at_ms\":"));
    assert!(json.contains("\"recorded_at_ms\":"));
    assert!(!json.contains("metadata."));
    assert!(!json.contains("ingested_at_ms"));

    let csv = MeasurementV1WithoutDetails::metamorphose_csv(&projected, MAX_RESPONSE_BYTES)?;
    let trusted_csv = unsafe {
        MeasurementV1WithoutDetails::metamorphose_csv_trusted_unchecked(
            &projected,
            MAX_RESPONSE_BYTES,
        )?
    };
    assert_eq!(csv, trusted_csv);
    let csv = std::str::from_utf8(&csv)?;
    assert!(csv.starts_with("schema_version,device,interval,started_at_ms,recorded_at_ms"));
    assert!(!csv.contains("metadata."));
    assert!(!csv.contains("ingested_at_ms"));

    let protobuf =
        MeasurementV1WithoutDetails::metamorphose_protobuf(&projected, MAX_RESPONSE_BYTES)?;
    let trusted_protobuf = unsafe {
        MeasurementV1WithoutDetails::metamorphose_protobuf_trusted_unchecked(
            &projected,
            MAX_RESPONSE_BYTES,
        )?
    };
    assert_eq!(protobuf, trusted_protobuf);
    assert!(!protobuf.is_empty());

    let arrow = MeasurementV1WithoutDetails::metamorphose_arrow(&projected, MAX_RESPONSE_BYTES)?;
    let trusted_arrow = unsafe {
        MeasurementV1WithoutDetails::metamorphose_arrow_trusted_unchecked(
            &projected,
            MAX_RESPONSE_BYTES,
        )?
    };
    assert_eq!(arrow.num_rows(), fixture_rows().len());
    assert_eq!(trusted_arrow.num_rows(), arrow.num_rows());
    assert!(schema_has_field(&arrow, "started_at_ms"));
    assert!(schema_has_field(&arrow, "recorded_at_ms"));
    assert!(!schema_has_field(&arrow, "metadata.ingested_at_ms"));
    assert!(!schema_has_field(&arrow, "ingested_at_ms"));

    let arrow_ipc =
        MeasurementV1WithoutDetails::metamorphose_arrow_ipc(&projected, MAX_RESPONSE_BYTES)?;
    let trusted_arrow_ipc = unsafe {
        MeasurementV1WithoutDetails::metamorphose_arrow_ipc_trusted_unchecked(
            &projected,
            MAX_RESPONSE_BYTES,
        )?
    };
    assert_eq!(arrow_ipc, trusted_arrow_ipc);
    let decoded_arrow = record_batch_from_ipc_stream(&arrow_ipc)?;
    assert_eq!(decoded_arrow.num_rows(), arrow.num_rows());
    assert!(!schema_has_field(&decoded_arrow, "metadata.ingested_at_ms"));
    assert!(!schema_has_field(&decoded_arrow, "ingested_at_ms"));

    let parquet =
        MeasurementV1WithoutDetails::metamorphose_parquet(&projected, MAX_RESPONSE_BYTES)?;
    let trusted_parquet = unsafe {
        MeasurementV1WithoutDetails::metamorphose_parquet_trusted_unchecked(
            &projected,
            MAX_RESPONSE_BYTES,
        )?
    };
    assert_eq!(parquet, trusted_parquet);
    assert!(parquet.starts_with(b"PAR1"));
    assert!(parquet.ends_with(b"PAR1"));

    let parquet_source = include_str!("../src/measurement_v1_parquet.rs");
    let projection_section = parquet_source
        .split("impl ParquetMetamorphoseSchema for MeasurementV1WithoutDetails")
        .nth(1)
        .ok_or("missing no-metadata parquet projection section")?;
    assert!(projection_section.contains("started_at_ms"));
    assert!(!projection_section.contains("metadata.ingested_at_ms"));
    assert!(!projection_section.contains("ingested_at_ms"));
    Ok(())
}

fn schema_has_field(batch: &mbt_adapter_arrow::ArrowRecordBatch, name: &str) -> bool {
    batch
        .schema()
        .fields()
        .iter()
        .any(|field| field.name() == name)
}

fn fixture_rows() -> Vec<MeasurementRowV1> {
    vec![fixture_row(0), fixture_row(1)]
}

fn fixture_row(idx: i64) -> MeasurementRowV1 {
    let recorded_at_ms = 1_700_000_000_000 + idx * 60_000;
    let base = 100.0 + idx as f64;
    MeasurementRowV1 {
        schema_version: SCHEMA_VERSION_VALUE,
        device_ordinal: DEVICE_SENSOR_B,
        interval_ordinal: INTERVAL_60S,
        started_at_ms: recorded_at_ms - 60_000,
        recorded_at_ms,
        value_a: base,
        value_b: base + 1.0,
        value_c: base - 1.0,
        value_d: base + 0.5,
        value_e: 1_000.0 + idx as f64,
        value_f: 10_000.0 + idx as f64,
        value_g: 500.0,
        value_h: -10.0,
        value_i: 5_000.0,
        value_j: -100.0,
        count_a: 10,
        count_b: -2,
        average_value: base + 0.1,
        sample_count: 20,
        source_ordinal: SOURCE_SENSOR,
        process_ordinal: PROCESS_DERIVED,
        sites_expected_mask: (1_u64 << SITE_SITE_A_BIT) | (1_u64 << SITE_SITE_B_BIT),
        sites_observed_mask: 1_u64 << SITE_SITE_A_BIT,
        ingested_at_ms: recorded_at_ms + 1,
        target_ingested_at_ms: recorded_at_ms + 2,
        built_at_ms: recorded_at_ms + 3,
        committed_at_ms: recorded_at_ms + 4,
        harmonized_at_ms: recorded_at_ms + 5,
        recomputed_at_ms: recorded_at_ms + 6,
        recomputed_reason_ordinal: RECOMPUTED_REASON_CANONICAL_REPAIR,
        covered_interval_count: 1,
        expected_interval_count: 1,
        coverage_ratio: 1.0,
        inputs_source_counts_sensor: 1,
        inputs_source_counts_api: 0,
        inputs_source_counts_synthetic: 0,
        inputs_source_counts_corrected: 0,
        sensor_window_inputs_coverage_ratio: 1.0,
        sensor_window_expected: 12,
        sensor_window_synthetic_count: 0,
        sensor_window_synthetic_ratio: 0.0,
        sensor_window_observed_count: 12,
        sensor_window_observed_ratio: 1.0,
        age_ms: 100,
        presence_bits: PRESENCE_ALLOWED_MASK,
    }
}
