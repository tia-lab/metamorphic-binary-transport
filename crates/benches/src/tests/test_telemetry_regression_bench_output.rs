use crate::telemetry_regression::*;
use std::error::Error;
use std::fs;

#[test]
fn labels_and_fixture_are_bound() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        required_current_labels(),
        [
            "telemetry_mbt_full_encode_inspect_checked",
            "telemetry_metamorphose_json_checked",
            "telemetry_metamorphose_protobuf_checked",
            "telemetry_metamorphose_csv_checked",
            "telemetry_metamorphose_json_trusted",
            "telemetry_metamorphose_protobuf_trusted",
            "telemetry_metamorphose_csv_trusted",
            "telemetry_metamorphose_arrow_ipc_trusted",
            "telemetry_metamorphose_parquet_trusted",
        ]
    );
    assert_eq!(ROW_COUNTS, [1, 100, 500, 1000, 10000, 100000]);
    assert!(telemetry_rows(usize::MAX).is_err());
    let rows = telemetry_rows(100)?;
    assert_eq!(rows[0].recorded_at_ms, 1_700_000_000_000);
    assert_eq!(rows[1].recorded_at_ms, 1_700_000_001_000);
    assert_eq!(rows[0].temperature_c, 20.0);
    assert_eq!(rows[1].temperature_c, 20.25);
    assert_eq!((rows[0].battery_percent, rows[0].presence_bits), (50.0, 1));
    assert_eq!((rows[1].battery_percent, rows[1].presence_bits), (0.0, 0));
    let a = mbt_schema_telemetry::telemetry_v1::TelemetryV1::encode(&rows, MAX_RESPONSE_BYTES)?;
    let b = mbt_schema_telemetry::telemetry_v1::TelemetryV1::encode(
        &telemetry_rows(100)?,
        MAX_RESPONSE_BYTES,
    )?;
    assert_eq!(a, b);
    assert!(measured_rates(1, 128, 0.0).is_err());
    assert!(measured_rates(1, 128, f64::NAN).is_err());
    Ok(())
}

#[test]
fn report_binds_new_dataset_and_refuses_overwrite() -> Result<(), Box<dyn Error>> {
    let root = std::env::temp_dir().join(format!("telemetry-report-{}", std::process::id()));
    fs::create_dir_all(&root)?;
    let path = next_telemetry_run_path(&root)?;
    let metadata = metadata_for_run("test".to_string(), &path)?;
    let row = TelemetryRegressionRow {
        label: required_current_labels()[0],
        row_count: 1,
        output_bytes: 128,
        total_milliseconds: 1.,
        rows_per_second: 1000.,
        mb_per_second: 1.,
        response_checksum: 1,
        semantic_checksum: Some(2),
        minimal_projection_checksum: Some(3),
    };
    write_report(&path, &metadata, &[row.clone()])?;
    let report: serde_json::Value = serde_json::from_slice(&fs::read(&path)?)?;
    assert_eq!(report["metadata"]["dataset_identity"], DATASET_IDENTITY);
    assert_eq!(report["metadata"]["cache_mode"], "in_memory");
    for key in [
        "timestamp_utc",
        "operator",
        "command",
        "build_profile",
        "dirty_state",
        "rustc",
        "os_kernel",
        "cpu",
        "ram",
        "schema_hash",
        "row_counts",
        "max_response_bytes",
    ] {
        assert!(report["metadata"].get(key).is_some());
    }
    for key in [
        "old_baseline_label",
        "old_crate_comparison",
        "serde_json_comparison",
    ] {
        assert!(report["rows"][0].get(key).is_none());
    }
    assert!(write_report(&path, &metadata, &[row]).is_err());
    assert_ne!(next_telemetry_run_path(&root)?, path);
    fs::remove_dir_all(root)?;
    Ok(())
}
