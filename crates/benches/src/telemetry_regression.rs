use mbt_schema_telemetry::telemetry_v1::TelemetryV1;
use serde::Serialize;
use std::fs::{self, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

pub use crate::projection::{
    BenchResult, DATASET_IDENTITY, MAX_RESPONSE_BYTES, ROW_COUNTS, measured_rates,
    response_checksum, telemetry_rows,
};
pub const REQUIRED_SCHEMA_FEATURES: [&str; 5] = ["json", "protobuf", "csv", "arrow_ipc", "parquet"];

#[derive(Clone, Debug, Serialize)]
pub struct TelemetryRegressionRow {
    pub label: &'static str,
    pub row_count: usize,
    pub output_bytes: usize,
    pub total_milliseconds: f64,
    pub rows_per_second: f64,
    pub mb_per_second: f64,
    pub response_checksum: u64,
    pub semantic_checksum: Option<u64>,
    pub minimal_projection_checksum: Option<u64>,
}

#[derive(Clone, Debug, Serialize)]
pub struct TelemetryRegressionMetadata {
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
    pub schema_hash: u64,
    pub row_counts: &'static [usize],
    pub enabled_schema_features: &'static [&'static str],
    pub max_response_bytes: usize,
    pub report_path: String,
    pub cache_mode: &'static str,
}

pub fn next_telemetry_run_path(report_dir: &Path) -> BenchResult<PathBuf> {
    fs::create_dir_all(report_dir)?;
    for idx in 1..=1_000_u32 {
        let path = report_dir.join(format!("telemetry_regression_run_{idx}.json"));
        if !path.exists() {
            return Ok(path);
        }
    }
    Err(io::Error::other("no available telemetry regression run path").into())
}

pub fn metadata_for_run(
    command: String,
    report_path: &Path,
) -> BenchResult<TelemetryRegressionMetadata> {
    Ok(TelemetryRegressionMetadata {
        slug: "mbt_telemetry_migration",
        timestamp_utc: command_output("date", &["-u", "+%Y-%m-%dT%H:%M:%SZ"])?,
        operator: operator_name(),
        command,
        build_profile: if cfg!(debug_assertions) {
            "dev"
        } else {
            "release"
        },
        dirty_state: dirty_state()?,
        rustc: command_output("rustc", &["--version"])?,
        os_kernel: command_output("uname", &["-sr"])?,
        cpu: cpu_name(),
        ram: ram_total(),
        dataset_identity: DATASET_IDENTITY,
        schema_hash: TelemetryV1::SCHEMA_HASH,
        row_counts: &ROW_COUNTS,
        enabled_schema_features: &REQUIRED_SCHEMA_FEATURES,
        max_response_bytes: MAX_RESPONSE_BYTES,
        report_path: report_path.display().to_string(),
        cache_mode: "in_memory",
    })
}

pub fn write_report(
    path: &Path,
    metadata: &TelemetryRegressionMetadata,
    rows: &[TelemetryRegressionRow],
) -> BenchResult<()> {
    for row in rows {
        measured_rates(row.row_count, row.output_bytes, row.total_milliseconds)?;
        if !row.rows_per_second.is_finite() || !row.mb_per_second.is_finite() {
            return Err(io::Error::other("non-finite benchmark rate").into());
        }
    }
    #[derive(Serialize)]
    struct Report<'a> {
        metadata: &'a TelemetryRegressionMetadata,
        rows: &'a [TelemetryRegressionRow],
    }
    let file = OpenOptions::new().write(true).create_new(true).open(path)?;
    serde_json::to_writer_pretty(file, &Report { metadata, rows })?;
    Ok(())
}

pub fn required_current_labels() -> &'static [&'static str] {
    &[
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

pub fn operator_name() -> String {
    match std::env::var("USER") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => match std::env::var("LOGNAME") {
            Ok(value) if !value.trim().is_empty() => value,
            _ => "unavailable".to_string(),
        },
    }
}

pub fn cpu_name() -> String {
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

pub fn ram_total() -> String {
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
