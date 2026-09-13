use std::error::Error;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

use mbt_core::codec;
use mbt_schema_measurement::measurement_v1::MeasurementRowV1;

pub const MAX_RESPONSE_BYTES: usize = 1_073_741_824;
pub const ROW_COUNTS: [usize; 6] = [1, 100, 500, 1_000, 10_000, 100_000];
// Old-crate evidence is read as a regression oracle, not recalculated here.
pub const REFERENCE_PARITY_EVIDENCE_GLOB: &str = "reference/measurement_parity_run_*.json";
// These features must be enabled for the Measurement regression surface to be complete.
pub const REQUIRED_SCHEMA_FEATURES: [&str; 5] = ["json", "protobuf", "csv", "arrow_ipc", "parquet"];
pub const PROJECTION_EVIDENCE_GLOB: &str = "reference/measurement_projection_run_*.json";

pub type BenchResult<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Clone, Debug)]
pub struct Comparison {
    pub rows_per_second: f64,
    pub mb_per_second: f64,
    pub ratio_rows_per_second: f64,
}

#[derive(Clone, Debug)]
pub struct MeasurementRegressionRow {
    // One row records one measured boundary and its evidence checksums.
    pub label: &'static str,
    pub row_count: usize,
    pub output_bytes: usize,
    pub total_milliseconds: f64,
    pub rows_per_second: f64,
    pub mb_per_second: f64,
    pub response_checksum: u64,
    pub semantic_checksum: Option<u64>,
    pub minimal_projection_checksum: Option<u64>,
    pub reference_baseline_label: Option<&'static str>,
    pub reference_comparison: Option<Comparison>,
    pub serde_json_comparison: Option<Comparison>,
}

#[derive(Clone, Debug)]
pub struct MeasurementRegressionMetadata {
    pub slug: &'static str,
    pub timestamp_utc: String,
    pub operator: String,
    pub command: String,
    pub build_profile: &'static str,
    pub dirty_state: String,
    pub rustc: String,
    pub os_kernel: String,
    pub cpu: String,
    pub ram: String,
    pub dataset_identity: &'static str,
    pub row_counts: &'static [usize],
    pub enabled_schema_features: &'static [&'static str],
    pub max_response_bytes: usize,
    pub reference_parity_evidence_glob: &'static str,
    pub projection_evidence_glob: &'static str,
    pub report_path: String,
    pub cache_mode: &'static str,
}

#[derive(Clone, serde::Serialize)]
pub struct SerdeMeasurementRow {
    // Serde exists only as the direct JSON baseline for this benchmark crate.
    pub schema_version: u16,
    pub device_ordinal: u16,
    pub interval_ordinal: u16,
    pub started_at_ms: i64,
    pub recorded_at_ms: i64,
    pub value_a: f64,
    pub value_b: f64,
    pub value_c: f64,
    pub value_d: f64,
    pub value_e: f64,
    pub value_f: f64,
    pub value_g: f64,
    pub value_h: f64,
    pub value_i: f64,
    pub value_j: f64,
    pub count_a: i64,
    pub count_b: i64,
    pub average_value: f64,
    pub sample_count: i64,
    pub source_ordinal: u16,
    pub process_ordinal: u16,
    pub sites_expected_mask: u64,
    pub sites_observed_mask: u64,
    pub ingested_at_ms: i64,
    pub target_ingested_at_ms: i64,
    pub built_at_ms: i64,
    pub committed_at_ms: i64,
    pub harmonized_at_ms: i64,
    pub recomputed_at_ms: i64,
    pub recomputed_reason_ordinal: u16,
    pub covered_interval_count: i64,
    pub expected_interval_count: i64,
    pub coverage_ratio: f64,
    pub inputs_source_counts_sensor: i64,
    pub inputs_source_counts_api: i64,
    pub inputs_source_counts_synthetic: i64,
    pub inputs_source_counts_corrected: i64,
    pub sensor_window_inputs_coverage_ratio: f64,
    pub sensor_window_expected: i64,
    pub sensor_window_synthetic_count: i64,
    pub sensor_window_synthetic_ratio: f64,
    pub sensor_window_observed_count: i64,
    pub sensor_window_observed_ratio: f64,
    pub age_ms: i64,
    pub presence_bits: u64,
}

pub fn measurement_rows(row_count: usize) -> Vec<MeasurementRowV1> {
    crate::measurement_projection::measurement_rows(row_count)
}

pub fn serde_rows_from_measurement(rows: &[MeasurementRowV1]) -> Vec<SerdeMeasurementRow> {
    rows.iter().map(SerdeMeasurementRow::from).collect()
}

pub fn next_measurement_run_path(report_dir: &Path) -> BenchResult<PathBuf> {
    fs::create_dir_all(report_dir)?;
    for idx in 1..=1_000_u32 {
        let path = report_dir.join(format!("measurement_regression_run_{idx}.json"));
        if !path.exists() {
            return Ok(path);
        }
    }
    Err(io::Error::other("no available Measurement regression run path").into())
}

pub fn measured_rates(row_count: usize, output_bytes: usize, milliseconds: f64) -> (f64, f64) {
    if milliseconds <= 0.0 {
        return (0.0, 0.0);
    }
    let seconds = milliseconds / 1_000.0;
    let rows_per_second = row_count as f64 / seconds;
    let mb_per_second = output_bytes as f64 / (1024.0 * 1024.0) / seconds;
    (rows_per_second, mb_per_second)
}

pub fn response_checksum(bytes: &[u8]) -> u64 {
    codec::response_checksum(bytes)
}

pub fn metadata_for_run(
    command: String,
    report_path: &Path,
) -> BenchResult<MeasurementRegressionMetadata> {
    // Report metadata captures the run boundary required for later evidence review.
    Ok(MeasurementRegressionMetadata {
        slug: "mbt_measurement_regression_benchmark",
        timestamp_utc: command_output("date", &["-u", "+%Y-%m-%dT%H:%M:%SZ"])?,
        operator: operator_name(),
        command,
        build_profile: "release",
        dirty_state: dirty_state()?,
        rustc: command_output("rustc", &["--version"])?,
        os_kernel: command_output("uname", &["-sr"])?,
        cpu: cpu_name(),
        ram: ram_total(),
        dataset_identity: "synthetic_measurement_v1_structural_fixture",
        row_counts: &ROW_COUNTS,
        enabled_schema_features: &REQUIRED_SCHEMA_FEATURES,
        max_response_bytes: MAX_RESPONSE_BYTES,
        reference_parity_evidence_glob: REFERENCE_PARITY_EVIDENCE_GLOB,
        projection_evidence_glob: PROJECTION_EVIDENCE_GLOB,
        report_path: report_path.display().to_string(),
        cache_mode: "in_memory",
    })
}

pub fn write_report(
    path: &Path,
    metadata: &MeasurementRegressionMetadata,
    rows: &[MeasurementRegressionRow],
) -> BenchResult<()> {
    // Reports are append-only artifacts; existing run files are never overwritten.
    if path.exists() {
        return Err(io::Error::other("Measurement regression report already exists").into());
    }
    for row in rows {
        validate_row_numbers(row)?;
    }
    let mut out = String::new();
    out.push_str("{\n  \"metadata\": ");
    write_metadata_json(&mut out, metadata);
    out.push_str(",\n  \"rows\": [\n");
    for (idx, row) in rows.iter().enumerate() {
        if idx > 0 {
            out.push_str(",\n");
        }
        write_row_json(&mut out, row);
    }
    out.push_str("\n  ]\n}\n");
    fs::write(path, out)?;
    Ok(())
}

pub fn required_current_labels() -> &'static [&'static str] {
    &[
        "measurement_mbt_full_encode_inspect_checked",
        "measurement_metamorphose_json_checked",
        "measurement_metamorphose_protobuf_checked",
        "measurement_metamorphose_csv_checked",
        "measurement_metamorphose_json_trusted",
        "measurement_metamorphose_protobuf_trusted",
        "measurement_metamorphose_csv_trusted",
        "measurement_metamorphose_arrow_ipc_trusted",
        "measurement_metamorphose_parquet_trusted",
        "measurement_serde_json_baseline",
    ]
}

pub fn sample_metadata(report_path: &Path) -> MeasurementRegressionMetadata {
    MeasurementRegressionMetadata {
        slug: "mbt_measurement_regression_benchmark",
        timestamp_utc: "2026-06-15T00:00:00Z".to_string(),
        operator: "test".to_string(),
        command: "test".to_string(),
        build_profile: "release",
        dirty_state: "test".to_string(),
        rustc: "test".to_string(),
        os_kernel: "test".to_string(),
        cpu: "test".to_string(),
        ram: "test".to_string(),
        dataset_identity: "test",
        row_counts: &ROW_COUNTS,
        enabled_schema_features: &REQUIRED_SCHEMA_FEATURES,
        max_response_bytes: MAX_RESPONSE_BYTES,
        reference_parity_evidence_glob: REFERENCE_PARITY_EVIDENCE_GLOB,
        projection_evidence_glob: PROJECTION_EVIDENCE_GLOB,
        report_path: report_path.display().to_string(),
        cache_mode: "in_memory",
    }
}

pub fn sample_row(label: &'static str, row_count: usize) -> MeasurementRegressionRow {
    MeasurementRegressionRow {
        label,
        row_count,
        output_bytes: 128,
        total_milliseconds: 1.0,
        rows_per_second: row_count as f64 * 1_000.0,
        mb_per_second: 1.0,
        response_checksum: 1,
        semantic_checksum: Some(2),
        minimal_projection_checksum: Some(3),
        reference_baseline_label: None,
        reference_comparison: None,
        serde_json_comparison: None,
    }
}

impl From<&MeasurementRowV1> for SerdeMeasurementRow {
    fn from(row: &MeasurementRowV1) -> Self {
        Self {
            schema_version: row.schema_version,
            device_ordinal: row.device_ordinal,
            interval_ordinal: row.interval_ordinal,
            started_at_ms: row.started_at_ms,
            recorded_at_ms: row.recorded_at_ms,
            value_a: row.value_a,
            value_b: row.value_b,
            value_c: row.value_c,
            value_d: row.value_d,
            value_e: row.value_e,
            value_f: row.value_f,
            value_g: row.value_g,
            value_h: row.value_h,
            value_i: row.value_i,
            value_j: row.value_j,
            count_a: row.count_a,
            count_b: row.count_b,
            average_value: row.average_value,
            sample_count: row.sample_count,
            source_ordinal: row.source_ordinal,
            process_ordinal: row.process_ordinal,
            sites_expected_mask: row.sites_expected_mask,
            sites_observed_mask: row.sites_observed_mask,
            ingested_at_ms: row.ingested_at_ms,
            target_ingested_at_ms: row.target_ingested_at_ms,
            built_at_ms: row.built_at_ms,
            committed_at_ms: row.committed_at_ms,
            harmonized_at_ms: row.harmonized_at_ms,
            recomputed_at_ms: row.recomputed_at_ms,
            recomputed_reason_ordinal: row.recomputed_reason_ordinal,
            covered_interval_count: row.covered_interval_count,
            expected_interval_count: row.expected_interval_count,
            coverage_ratio: row.coverage_ratio,
            inputs_source_counts_sensor: row.inputs_source_counts_sensor,
            inputs_source_counts_api: row.inputs_source_counts_api,
            inputs_source_counts_synthetic: row.inputs_source_counts_synthetic,
            inputs_source_counts_corrected: row.inputs_source_counts_corrected,
            sensor_window_inputs_coverage_ratio: row.sensor_window_inputs_coverage_ratio,
            sensor_window_expected: row.sensor_window_expected,
            sensor_window_synthetic_count: row.sensor_window_synthetic_count,
            sensor_window_synthetic_ratio: row.sensor_window_synthetic_ratio,
            sensor_window_observed_count: row.sensor_window_observed_count,
            sensor_window_observed_ratio: row.sensor_window_observed_ratio,
            age_ms: row.age_ms,
            presence_bits: row.presence_bits,
        }
    }
}

fn validate_row_numbers(row: &MeasurementRegressionRow) -> BenchResult<()> {
    if !row.total_milliseconds.is_finite()
        || !row.rows_per_second.is_finite()
        || !row.mb_per_second.is_finite()
    {
        return Err(io::Error::other(format!("non-finite benchmark row {}", row.label)).into());
    }
    if row.output_bytes == 0 {
        return Err(io::Error::other(format!("zero output bytes for {}", row.label)).into());
    }
    Ok(())
}

fn command_output(command: &str, args: &[&str]) -> BenchResult<String> {
    let output = Command::new(command).args(args).output()?;
    if !output.status.success() {
        return Err(io::Error::other(format!("metadata command failed: {command}")).into());
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

fn dirty_state() -> BenchResult<String> {
    let output = Command::new("git").args(["status", "--short"]).output()?;
    if !output.status.success() {
        return Err(io::Error::other("git status failed").into());
    }
    let stdout = String::from_utf8(output.stdout)?;
    if stdout.trim().is_empty() {
        Ok("clean".to_string())
    } else {
        Ok("dirty".to_string())
    }
}

fn operator_name() -> String {
    match std::env::var("USER") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => match std::env::var("LOGNAME") {
            Ok(value) if !value.trim().is_empty() => value,
            _ => "unknown".to_string(),
        },
    }
}

fn cpu_name() -> String {
    match fs::read_to_string("/proc/cpuinfo") {
        Ok(text) => {
            for line in text.lines() {
                if let Some(value) = line.strip_prefix("model name") {
                    return value.trim_start_matches(':').trim().to_string();
                }
            }
            "unavailable".to_string()
        }
        Err(_) => "unavailable".to_string(),
    }
}

fn ram_total() -> String {
    match fs::read_to_string("/proc/meminfo") {
        Ok(text) => {
            for line in text.lines() {
                if line.starts_with("MemTotal:") {
                    return line.to_string();
                }
            }
            "unavailable".to_string()
        }
        Err(_) => "unavailable".to_string(),
    }
}

fn write_metadata_json(out: &mut String, metadata: &MeasurementRegressionMetadata) {
    out.push('{');
    write_string_field(out, "slug", metadata.slug, true);
    write_string_field(out, "timestamp_utc", &metadata.timestamp_utc, false);
    write_string_field(out, "operator", &metadata.operator, false);
    write_string_field(out, "command", &metadata.command, false);
    write_string_field(out, "build_profile", metadata.build_profile, false);
    write_string_field(out, "dirty_state", &metadata.dirty_state, false);
    write_string_field(out, "rustc", &metadata.rustc, false);
    write_string_field(out, "os_kernel", &metadata.os_kernel, false);
    write_string_field(out, "cpu", &metadata.cpu, false);
    write_string_field(out, "ram", &metadata.ram, false);
    write_string_field(out, "dataset_identity", metadata.dataset_identity, false);
    write_usize_array_field(out, "row_counts", metadata.row_counts);
    write_string_array_field(
        out,
        "enabled_schema_features",
        metadata.enabled_schema_features,
    );
    write_usize_field(out, "max_response_bytes", metadata.max_response_bytes);
    write_string_field(
        out,
        "reference_parity_evidence_glob",
        metadata.reference_parity_evidence_glob,
        false,
    );
    write_string_field(
        out,
        "projection_evidence_glob",
        metadata.projection_evidence_glob,
        false,
    );
    write_string_field(out, "report_path", &metadata.report_path, false);
    write_string_field(out, "cache_mode", metadata.cache_mode, false);
    out.push_str(" }");
}

fn write_row_json(out: &mut String, row: &MeasurementRegressionRow) {
    out.push_str("    {");
    write_string_field(out, "label", row.label, true);
    write_usize_field(out, "row_count", row.row_count);
    write_usize_field(out, "output_bytes", row.output_bytes);
    write_f64_field(out, "total_milliseconds", row.total_milliseconds);
    write_f64_field(out, "rows_per_second", row.rows_per_second);
    write_f64_field(out, "mb_per_second", row.mb_per_second);
    write_u64_field(out, "response_checksum", row.response_checksum);
    write_optional_u64_field(out, "semantic_checksum", row.semantic_checksum);
    write_optional_u64_field(
        out,
        "minimal_projection_checksum",
        row.minimal_projection_checksum,
    );
    write_optional_string_field(
        out,
        "reference_baseline_label",
        row.reference_baseline_label,
    );
    write_optional_comparison(
        out,
        "reference_comparison",
        row.reference_comparison.as_ref(),
    );
    write_optional_comparison(
        out,
        "serde_json_comparison",
        row.serde_json_comparison.as_ref(),
    );
    out.push_str(" }");
}

fn write_string_field(out: &mut String, name: &str, value: &str, first: bool) {
    if !first {
        out.push(',');
    }
    out.push_str(&format!(" \"{name}\": \"{}\"", escape_json(value)));
}

fn write_optional_string_field(out: &mut String, name: &str, value: Option<&str>) {
    match value {
        Some(inner) => write_string_field(out, name, inner, false),
        None => out.push_str(&format!(", \"{name}\": null")),
    }
}

fn write_usize_field(out: &mut String, name: &str, value: usize) {
    out.push_str(&format!(", \"{name}\": {value}"));
}

fn write_u64_field(out: &mut String, name: &str, value: u64) {
    out.push_str(&format!(", \"{name}\": {value}"));
}

fn write_f64_field(out: &mut String, name: &str, value: f64) {
    out.push_str(&format!(", \"{name}\": {:.6}", value));
}

fn write_optional_u64_field(out: &mut String, name: &str, value: Option<u64>) {
    match value {
        Some(inner) => write_u64_field(out, name, inner),
        None => out.push_str(&format!(", \"{name}\": null")),
    }
}

fn write_optional_comparison(out: &mut String, name: &str, value: Option<&Comparison>) {
    out.push_str(&format!(", \"{name}\": "));
    match value {
        Some(inner) => out.push_str(&format!(
            "{{ \"rows_per_second\": {:.6}, \"mb_per_second\": {:.6}, \"ratio_rows_per_second\": {:.6} }}",
            inner.rows_per_second, inner.mb_per_second, inner.ratio_rows_per_second
        )),
        None => out.push_str("null"),
    }
}

fn write_usize_array_field(out: &mut String, name: &str, values: &[usize]) {
    out.push_str(&format!(", \"{name}\": ["));
    for (idx, value) in values.iter().enumerate() {
        if idx > 0 {
            out.push_str(", ");
        }
        out.push_str(&value.to_string());
    }
    out.push(']');
}

fn write_string_array_field(out: &mut String, name: &str, values: &[&str]) {
    out.push_str(&format!(", \"{name}\": ["));
    for (idx, value) in values.iter().enumerate() {
        if idx > 0 {
            out.push_str(", ");
        }
        out.push_str(&format!("\"{}\"", escape_json(value)));
    }
    out.push(']');
}

fn escape_json(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(ch),
        }
    }
    out
}
