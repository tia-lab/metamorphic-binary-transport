use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;

use crate::bars_regression::{
    OLD_PARITY_EVIDENCE_GLOB, ROW_COUNTS, bars_rows, next_bars_run_path, required_current_labels,
    sample_metadata, sample_row, serde_rows_from_bars, write_report,
};

#[test]
fn next_run_path_refuses_overwrite() -> Result<(), Box<dyn Error>> {
    let root = temp_root("bars-next-run")?;
    let first = root.join("bars_regression_run_1.json");
    fs::write(&first, b"existing")?;
    let next = next_bars_run_path(&root)?;
    if next.file_name().and_then(|name| name.to_str()) != Some("bars_regression_run_2.json") {
        return Err(err("next Bars run path did not skip existing run"));
    }

    let metadata = sample_metadata(&first);
    let row = sample_row("bars_mbt_full_encode_inspect_checked", 1);
    if write_report(&first, &metadata, &[row]).is_ok() {
        return Err(err("write_report allowed overwrite"));
    }
    fs::remove_dir_all(&root)?;
    Ok(())
}

#[test]
fn required_labels_are_exact() -> Result<(), Box<dyn Error>> {
    let expected = [
        "bars_mbt_full_encode_inspect_checked",
        "bars_metamorphose_json_checked",
        "bars_metamorphose_protobuf_checked",
        "bars_metamorphose_csv_checked",
        "bars_metamorphose_json_trusted",
        "bars_metamorphose_protobuf_trusted",
        "bars_metamorphose_csv_trusted",
        "bars_metamorphose_arrow_ipc_trusted",
        "bars_metamorphose_parquet_trusted",
        "bars_serde_json_baseline",
    ];
    if required_current_labels() != expected {
        return Err(err("Bars regression labels changed"));
    }
    if ROW_COUNTS != [1, 100, 500, 1_000, 10_000, 100_000] {
        return Err(err("Bars regression row counts changed"));
    }
    Ok(())
}

#[test]
fn serde_rows_preserve_selected_fields() -> Result<(), Box<dyn Error>> {
    let source = bars_rows(3);
    let serde_rows = serde_rows_from_bars(&source);
    if serde_rows.len() != source.len() {
        return Err(err("serde row count does not match source row count"));
    }
    let Some(source_row) = source.first() else {
        return Err(err("missing source row"));
    };
    let Some(serde_row) = serde_rows.first() else {
        return Err(err("missing serde row"));
    };
    if serde_row.schema_version != source_row.schema_version {
        return Err(err("schema_version was not preserved"));
    }
    if serde_row.pair_ordinal != source_row.pair_ordinal {
        return Err(err("pair_ordinal was not preserved"));
    }
    if serde_row.tf_ordinal != source_row.tf_ordinal {
        return Err(err("tf_ordinal was not preserved"));
    }
    if serde_row.close_ms != source_row.close_ms {
        return Err(err("close_ms was not preserved"));
    }
    if serde_row.c.to_bits() != source_row.c.to_bits() {
        return Err(err("c was not preserved"));
    }
    if serde_row.presence_bits != source_row.presence_bits {
        return Err(err("presence_bits was not preserved"));
    }
    Ok(())
}

#[test]
fn report_contains_required_fields() -> Result<(), Box<dyn Error>> {
    let root = temp_root("bars-report-fields")?;
    let path = root.join("bars_regression_run_1.json");
    let metadata = sample_metadata(&path);
    let row = sample_row("bars_mbt_full_encode_inspect_checked", 1);
    write_report(&path, &metadata, &[row])?;
    let text = fs::read_to_string(&path)?;
    for key in [
        "\"metadata\"",
        "\"rows\"",
        "\"slug\"",
        "\"timestamp_utc\"",
        "\"operator\"",
        "\"command\"",
        "\"build_profile\"",
        "\"dirty_state\"",
        "\"rustc\"",
        "\"os_kernel\"",
        "\"cpu\"",
        "\"ram\"",
        "\"dataset_identity\"",
        "\"row_counts\"",
        "\"enabled_schema_features\"",
        "\"max_response_bytes\"",
        "\"old_parity_evidence_glob\"",
        "\"projection_evidence_glob\"",
        "\"report_path\"",
        "\"cache_mode\"",
        "\"label\"",
        "\"row_count\"",
        "\"output_bytes\"",
        "\"total_milliseconds\"",
        "\"rows_per_second\"",
        "\"mb_per_second\"",
        "\"response_checksum\"",
        "\"semantic_checksum\"",
        "\"minimal_projection_checksum\"",
        "\"old_baseline_label\"",
        "\"old_crate_comparison\"",
        "\"serde_json_comparison\"",
    ] {
        if !text.contains(key) {
            return Err(err(format!("missing report key {key}")));
        }
    }
    fs::remove_dir_all(&root)?;
    Ok(())
}

#[test]
fn metadata_points_to_old_parity_evidence() -> Result<(), Box<dyn Error>> {
    let root = temp_root("bars-old-parity-metadata")?;
    let path = root.join("bars_regression_run_1.json");
    let metadata = sample_metadata(&path);
    let row = sample_row("bars_mbt_full_encode_inspect_checked", 1);
    write_report(&path, &metadata, &[row])?;
    let text = fs::read_to_string(&path)?;
    if !text.contains(OLD_PARITY_EVIDENCE_GLOB) {
        return Err(err(
            "report metadata does not reference old parity evidence glob",
        ));
    }
    fs::remove_dir_all(&root)?;
    Ok(())
}

fn temp_root(name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let root = std::env::temp_dir().join(format!("{name}-{}", std::process::id()));
    match fs::remove_dir_all(&root) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }
    fs::create_dir_all(&root)?;
    Ok(root)
}

fn err(message: impl Into<String>) -> Box<dyn Error> {
    Box::new(io::Error::other(message.into()))
}
