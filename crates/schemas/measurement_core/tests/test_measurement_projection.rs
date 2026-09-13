use std::error::Error;

use mbt_core::runtime::encode;
use mbt_schema_measurement::measurement_v1::*;

const MAX_RESPONSE_BYTES: usize = 1_000_000;

type TestResult = std::result::Result<(), Box<dyn Error>>;

#[test]
fn without_details_projection_checked_and_trusted_match() -> TestResult {
    let rows = fixture_rows();
    let source = encode::<MeasurementV1>(&rows, MAX_RESPONSE_BYTES)?;
    MeasurementV1::access(&source)?;

    let checked = MeasurementV1::project_without_details(&source, MAX_RESPONSE_BYTES)?;
    let trusted = unsafe {
        MeasurementV1::project_without_details_trusted_unchecked(&source, MAX_RESPONSE_BYTES)?
    };
    assert_eq!(checked, trusted);

    let view = MeasurementV1WithoutDetails::access(&checked)?;
    assert_eq!(view.len(), rows.len());
    let inspection = MeasurementV1WithoutDetails::inspect(&checked)?;
    assert_eq!(inspection.row_count, rows.len());
    assert_eq!(
        inspection.semantic_checksum,
        inspection.minimal_projection_checksum
    );
    Ok(())
}

#[test]
fn values_only_projection_checked_and_trusted_match() -> TestResult {
    let rows = fixture_rows();
    let source = encode::<MeasurementV1>(&rows, MAX_RESPONSE_BYTES)?;
    MeasurementV1::access(&source)?;

    let checked = MeasurementV1::project_values_only(&source, MAX_RESPONSE_BYTES)?;
    let trusted = unsafe {
        MeasurementV1::project_values_only_trusted_unchecked(&source, MAX_RESPONSE_BYTES)?
    };
    assert_eq!(checked, trusted);

    let view = MeasurementV1ValuesOnly::access(&checked)?;
    assert_eq!(view.len(), rows.len());
    let inspection = MeasurementV1ValuesOnly::inspect(&checked)?;
    assert_eq!(inspection.row_count, rows.len());
    assert_eq!(
        inspection.semantic_checksum,
        inspection.minimal_projection_checksum
    );
    Ok(())
}

#[test]
fn direct_projection_helpers_do_not_copy_owned_rows() {
    let source = include_str!("../src/measurement_v1.rs");
    for name in [
        "project_without_details_archived_direct",
        "project_values_only_archived_direct",
    ] {
        let bodies = direct_projection_helper_bodies(source, name);
        assert!(!bodies.is_empty(), "missing helper {name}");
        for body in bodies {
            for forbidden in [
                "Vec::with_capacity(archived.",
                ".to_string()",
                ".to_vec()",
                ".collect()",
                "rows.push(",
                "encode_owned(rows",
            ] {
                assert!(
                    !body.contains(forbidden),
                    "direct helper {name} contains {forbidden}"
                );
            }
        }
    }
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

fn direct_projection_helper_bodies<'a>(source: &'a str, name: &str) -> Vec<&'a str> {
    let needle = format!("fn {name}");
    let mut bodies = Vec::new();
    let mut cursor = source;
    while let Some(relative_start) = cursor.find(&needle) {
        let function_start = source.len() - cursor.len() + relative_start;
        let Some(open_relative) = source[function_start..].find('{') else {
            break;
        };
        let open = function_start + open_relative;
        let Some(close) = matching_close_brace(source, open) else {
            break;
        };
        bodies.push(&source[open + 1..close]);
        cursor = &source[close + 1..];
    }
    bodies
}

fn matching_close_brace(source: &str, open: usize) -> Option<usize> {
    let mut depth = 0_u32;
    for (offset, byte) in source.as_bytes()[open..].iter().enumerate() {
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(open + offset);
                }
            }
            _ => {}
        }
    }
    None
}
