use std::fs;

use crate::projection::{
    BenchRow, Comparison, ROW_COUNTS, SchemaName, next_run_path, required_labels, write_report,
};

#[test]
fn projection_report_uses_stable_field_order() {
    let root = std::env::temp_dir().join(format!(
        "mbt-projection-report-order-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("create temp report root");
    let path = root.join("projection_run_1.json");
    let row = sample_row("mathilde_binary_project_no_metadata_public", 100);
    write_report(&path, &[row]).expect("write report");
    let text = fs::read_to_string(&path).expect("read report");
    let expected = [
        "\"label\"",
        "\"schema\"",
        "\"row_count\"",
        "\"output_bytes\"",
        "\"total_milliseconds\"",
        "\"rows_per_second\"",
        "\"mb_per_second\"",
        "\"source_access_milliseconds\"",
        "\"projection_milliseconds\"",
        "\"projected_inspect_milliseconds\"",
        "\"response_checksum\"",
        "\"semantic_checksum\"",
        "\"minimal_projection_checksum\"",
        "\"old_crate_comparison\"",
        "\"current_owned_row_comparison\"",
    ];
    let mut previous = 0;
    for key in expected {
        let Some(position) = text.find(key) else {
            panic!("missing key {key}");
        };
        assert!(position >= previous, "unstable key ordering for {key}");
        previous = position;
    }
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn next_run_path_refuses_overwrite() {
    let root =
        std::env::temp_dir().join(format!("mbt-projection-report-next-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("create temp report root");
    fs::write(root.join("projection_run_1.json"), b"existing").expect("write existing run");
    let path = next_run_path(&root).expect("next run path");
    assert_eq!(
        path.file_name().and_then(|name| name.to_str()),
        Some("projection_run_2.json")
    );
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn required_projection_labels_and_row_counts_are_bound() {
    assert_eq!(required_labels().len(), 12);
    assert_eq!(ROW_COUNTS, [1, 100, 500, 1_000, 10_000, 100_000]);
    assert!(required_labels().contains(&"mathilde_binary_project_ohlcv_only_archived"));
    assert!(required_labels().contains(&"mbt_project_numeric_only_inspect"));
}

fn sample_row(label: &'static str, row_count: usize) -> BenchRow {
    BenchRow {
        label,
        schema: SchemaName::Bars,
        row_count,
        output_bytes: 128,
        total_milliseconds: 1.0,
        rows_per_second: row_count as f64 * 1_000.0,
        mb_per_second: 1.0,
        source_access_milliseconds: 0.1,
        projection_milliseconds: 0.8,
        projected_inspect_milliseconds: 0.1,
        response_checksum: 1,
        semantic_checksum: Some(2),
        minimal_projection_checksum: Some(3),
        old_crate_comparison: Some(Comparison {
            rows_per_second: 10.0,
            mb_per_second: 20.0,
            ratio_rows_per_second: 2.0,
        }),
        current_owned_row_comparison: None,
    }
}
