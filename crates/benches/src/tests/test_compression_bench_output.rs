use std::error::Error;
use std::fs;

use crate::compression::{
    ZSTD_VERSION, full_run_row_counts, next_compression_run_path, required_lanes, sample_report,
    smoke_run_row_counts, write_report,
};

#[test]
fn compression_row_counts_are_bound() {
    assert_eq!(
        full_run_row_counts()
            .iter()
            .map(|config| (config.label, config.rows, config.iterations))
            .collect::<Vec<_>>(),
        [
            ("one", 1, 50),
            ("small", 100, 50),
            ("page_500", 500, 50),
            ("page_1000", 1_000, 50),
            ("medium", 10_000, 10),
            ("large", 100_000, 3),
        ]
    );
    assert_eq!(
        smoke_run_row_counts()
            .iter()
            .map(|config| (config.label, config.rows, config.iterations))
            .collect::<Vec<_>>(),
        [("one", 1, 3), ("small", 100, 3), ("page_1000", 1_000, 3)]
    );
}

#[test]
fn compression_lanes_are_bound() {
    assert_eq!(required_lanes(), ["mbt_full", "mbt_temperature_only"]);
}

#[test]
fn compression_report_uses_required_fields() -> Result<(), Box<dyn Error>> {
    let root = std::env::temp_dir().join(format!("mbt-compression-report-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root)?;
    let path = root.join("compression_run_1.json");
    write_report(&path, &sample_report())?;
    let text = fs::read_to_string(&path)?;
    let expected = [
        "\"experiment\"",
        "\"status\"",
        "\"utc_ms\"",
        "\"command\"",
        "\"build_profile\"",
        "\"rustc\"",
        "\"os\"",
        "\"cpu\"",
        "\"git_dirty\"",
        "\"dataset_identity\"",
        "\"fixture_source\"",
        "\"fixture_seed\"",
        "\"schema_hashes\"",
        "\"zstd_version\"",
        "\"zstd_level\"",
        "\"max_response_bytes\"",
        "\"row_counts\"",
        "\"metrics\"",
        "\"determinism\"",
    ];
    let mut previous = 0;
    for key in expected {
        let Some(position) = text.find(key) else {
            return Err(format!("missing key {key}").into());
        };
        assert!(position >= previous, "unstable key ordering for {key}");
        previous = position;
    }
    let metric_expected = [
        "\"label\"",
        "\"lane\"",
        "\"rows\"",
        "\"iterations\"",
        "\"uncompressed_bytes\"",
        "\"compressed_bytes\"",
        "\"compression_ratio\"",
        "\"reduction_percent\"",
        "\"compress_us\"",
        "\"decompress_us\"",
        "\"compress_mb_per_sec\"",
        "\"decompress_mb_per_sec\"",
        "\"p50_compress_us\"",
        "\"p95_compress_us\"",
        "\"p99_compress_us\"",
        "\"p50_decompress_us\"",
        "\"p95_decompress_us\"",
        "\"p99_decompress_us\"",
        "\"source_checksum\"",
        "\"compressed_checksum\"",
        "\"decompressed_checksum\"",
        "\"source_schema_hash\"",
        "\"byte_equal\"",
    ];
    for key in metric_expected {
        if !text.contains(key) {
            return Err(format!("missing metric key {key}").into());
        }
    }
    let _ = fs::remove_dir_all(&root);
    Ok(())
}

#[test]
fn next_compression_run_path_refuses_overwrite() -> Result<(), Box<dyn Error>> {
    let root = std::env::temp_dir().join(format!("mbt-compression-next-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root)?;
    fs::write(root.join("compression_run_1.json"), b"existing")?;
    let path = next_compression_run_path(&root)?;
    assert_eq!(
        path.file_name().and_then(|name| name.to_str()),
        Some("compression_run_2.json")
    );
    let _ = fs::remove_dir_all(&root);
    Ok(())
}

#[test]
fn sample_compression_report_records_zstd_level_three() -> Result<(), Box<dyn Error>> {
    let report = sample_report();
    assert_eq!(report.zstd_version, ZSTD_VERSION);
    assert_eq!(report.zstd_level, 3);
    assert!(report.metrics.iter().all(|metric| metric.byte_equal));
    Ok(())
}
