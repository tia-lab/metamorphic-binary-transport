use crate::projection::*;
use std::fs;

#[test]
fn projection_report_uses_stable_field_order() -> BenchResult<()> {
    let root = std::env::temp_dir().join(format!("mbt-projection-report-{}", std::process::id()));
    fs::create_dir_all(&root)?;
    let path = next_run_path(&root)?;
    let row = BenchRow {
        label: required_labels()[0],
        schema: SchemaName::Telemetry,
        schema_hash: 1,
        row_count: 100,
        output_bytes: 128,
        total_milliseconds: 1.,
        rows_per_second: 100000.,
        mb_per_second: 1.,
        source_access_milliseconds: 0.1,
        projection_milliseconds: 0.8,
        projected_inspect_milliseconds: 0.1,
        response_checksum: 1,
        semantic_checksum: Some(2),
        minimal_projection_checksum: Some(3),
    };
    write_report(&path, &[row.clone()])?;
    let text = fs::read_to_string(&path)?;
    let report: serde_json::Value = serde_json::from_str(&text)?;
    assert_eq!(report["metadata"]["dataset_identity"], DATASET_IDENTITY);
    let row_start = text.find("\"label\"").ok_or("missing label")?;
    let mut previous = 0;
    for key in [
        "label",
        "schema",
        "schema_hash",
        "row_count",
        "output_bytes",
        "total_milliseconds",
        "rows_per_second",
        "mb_per_second",
        "source_access_milliseconds",
        "projection_milliseconds",
        "projected_inspect_milliseconds",
        "response_checksum",
        "semantic_checksum",
        "minimal_projection_checksum",
    ] {
        let pos = text[row_start..]
            .find(&format!("\"{key}\""))
            .ok_or("missing row field")?;
        assert!(pos >= previous);
        previous = pos;
    }
    assert!(report["rows"][0].get("old_crate_comparison").is_none());
    assert!(
        report["rows"][0]
            .get("current_owned_row_comparison")
            .is_none()
    );
    assert!(write_report(&path, &[row]).is_err());
    assert_ne!(next_run_path(&root)?, path);
    fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn required_projection_labels_and_row_counts_are_bound() {
    assert_eq!(ROW_COUNTS, [1, 100, 500, 1000, 10000, 100000]);
    assert_eq!(
        required_labels(),
        [
            "telemetry_project_temperature_only_public",
            "telemetry_project_temperature_only_archived",
            "telemetry_project_temperature_only_inspect",
            "mbt_project_no_optional_public",
            "mbt_project_no_optional_archived",
            "mbt_project_no_optional_inspect",
            "mbt_project_numeric_only_public",
            "mbt_project_numeric_only_archived",
            "mbt_project_numeric_only_inspect",
        ]
    );
}
