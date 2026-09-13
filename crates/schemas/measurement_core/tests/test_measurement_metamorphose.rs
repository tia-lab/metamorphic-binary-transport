#![cfg(all(feature = "json", feature = "csv", feature = "protobuf"))]
use mbt_schema_measurement::measurement_v1::*;

const MAX_RESPONSE_BYTES: usize = 1 << 20;

#[test]
fn measurement_json_csv_emit_derived_utc_outputs() -> Result<(), Box<dyn std::error::Error>> {
    let rows = fixture_rows();
    let bytes = MeasurementV1::encode(&rows, MAX_RESPONSE_BYTES)?;

    let json = MeasurementV1::metamorphose_json(&bytes, MAX_RESPONSE_BYTES)?;
    let json = std::str::from_utf8(&json)?;
    assert!(json.contains("\"started_at_utc\":\""));
    assert!(json.contains("\"recorded_at_utc\":\""));
    assert!(json.contains("\"metadata.ingested_at_utc\":\""));

    let csv = MeasurementV1::metamorphose_csv(&bytes, MAX_RESPONSE_BYTES)?;
    let csv = std::str::from_utf8(&csv)?;
    assert!(csv.starts_with(
        "schema_version,device,interval,started_at_ms,recorded_at_ms,started_at_utc,recorded_at_utc"
    ));
    assert!(csv.contains("metadata.ingested_at_utc"));
    assert!(csv.contains("metadata.target_ingested_at_utc"));
    Ok(())
}

#[test]
fn measurement_protobuf_generated_source_keeps_metadata_nested() {
    let source = include_str!("../src/measurement_v1_protobuf.rs");
    assert!(source.contains("writer.message_prefix(22, message_len)?;"));
    assert!(source.contains("write_protobuf_metadata(row, writer)?;"));
    assert!(source.contains("writer.utc(6, row.ingested_at_ms.to_native())?;"));
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
