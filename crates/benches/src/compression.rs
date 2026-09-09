use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use mbt_compression::{CompressionConfig, DEFAULT_ZSTD_LEVEL, compress_into, decompress_into};
use mbt_core::codec::response_checksum;
use mbt_schema_telemetry::telemetry_v1::{TelemetryV1, TelemetryV1TemperatureOnly};
use serde::Serialize;

use crate::projection::{MAX_RESPONSE_BYTES, telemetry_rows};

pub const ZSTD_VERSION: &str = "0.13.3";

pub type BenchResult<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct RowCountConfig {
    pub label: &'static str,
    pub rows: usize,
    pub iterations: usize,
}

pub const FULL_ROW_COUNTS: [RowCountConfig; 6] = [
    RowCountConfig {
        label: "one",
        rows: 1,
        iterations: 50,
    },
    RowCountConfig {
        label: "small",
        rows: 100,
        iterations: 50,
    },
    RowCountConfig {
        label: "page_500",
        rows: 500,
        iterations: 50,
    },
    RowCountConfig {
        label: "page_1000",
        rows: 1_000,
        iterations: 50,
    },
    RowCountConfig {
        label: "medium",
        rows: 10_000,
        iterations: 10,
    },
    RowCountConfig {
        label: "large",
        rows: 100_000,
        iterations: 3,
    },
];

pub const SMOKE_ROW_COUNTS: [RowCountConfig; 3] = [
    RowCountConfig {
        label: "one",
        rows: 1,
        iterations: 3,
    },
    RowCountConfig {
        label: "small",
        rows: 100,
        iterations: 3,
    },
    RowCountConfig {
        label: "page_1000",
        rows: 1_000,
        iterations: 3,
    },
];

#[derive(Clone, Debug, Serialize)]
pub struct CompressionReport {
    pub experiment: &'static str,
    pub status: &'static str,
    pub utc_ms: u128,
    pub command: String,
    pub build_profile: &'static str,
    pub rustc: String,
    pub os: String,
    pub cpu: String,
    pub operator: String,
    pub ram: String,
    pub cache_mode: &'static str,
    pub git_dirty: String,
    pub dataset_identity: &'static str,
    pub fixture_source: &'static str,
    pub fixture_seed: &'static str,
    pub schema_hashes: SchemaHashes,
    pub zstd_version: &'static str,
    pub zstd_level: i32,
    pub max_response_bytes: usize,
    pub row_counts: Vec<RowCountConfig>,
    pub metrics: Vec<CompressionMetric>,
    pub determinism: DeterminismReport,
}

#[derive(Clone, Debug, Serialize)]
pub struct SchemaHashes {
    pub mbt_full: u64,
    pub mbt_temperature_only: u64,
}

#[derive(Clone, Debug, Serialize)]
pub struct CompressionMetric {
    pub label: &'static str,
    pub lane: &'static str,
    pub rows: usize,
    pub iterations: usize,
    pub uncompressed_bytes: usize,
    pub compressed_bytes: usize,
    pub compression_ratio: f64,
    pub reduction_percent: f64,
    pub compress_us: f64,
    pub decompress_us: f64,
    pub compress_mb_per_sec: f64,
    pub decompress_mb_per_sec: f64,
    pub p50_compress_us: f64,
    pub p95_compress_us: f64,
    pub p99_compress_us: f64,
    pub p50_decompress_us: f64,
    pub p95_decompress_us: f64,
    pub p99_decompress_us: f64,
    pub source_checksum: u64,
    pub compressed_checksum: u64,
    pub decompressed_checksum: u64,
    pub source_schema_hash: u64,
    pub byte_equal: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct DeterminismReport {
    pub all_byte_equal: bool,
    pub source_checksum_stable: bool,
    pub compressed_checksum_stable: bool,
    pub decompressed_checksum_stable: bool,
}

pub fn run_benchmark(report_dir: &Path, smoke: bool, command: String) -> BenchResult<PathBuf> {
    let report_path = next_compression_run_path(report_dir)?;
    let report = measure_report(smoke, command)?;
    write_report(&report_path, &report)?;
    write_summary(report_dir, &report)?;
    write_environment(report_dir, &report, &report_path)?;
    Ok(report_path)
}

pub fn measure_report(smoke: bool, command: String) -> BenchResult<CompressionReport> {
    let row_counts = selected_row_counts(smoke);
    let mut metrics = Vec::with_capacity(row_counts.len() * required_lanes().len());
    let mut compressed_checksum_stable = true;
    let mut decompressed_checksum_stable = true;
    for config in row_counts {
        let sources = source_bytes(config.rows)?;
        for source in sources {
            let measured = measure_metric(config, &source)?;
            if !measured.compressed_checksum_stable {
                compressed_checksum_stable = false;
            }
            if !measured.decompressed_checksum_stable {
                decompressed_checksum_stable = false;
            }
            metrics.push(measured.metric);
        }
    }
    let determinism = determinism_report(
        &metrics,
        compressed_checksum_stable,
        decompressed_checksum_stable,
    );
    Ok(CompressionReport {
        experiment: "mbt_compression",
        status: "completed",
        utc_ms: utc_millis()?,
        command,
        build_profile: build_profile(),
        rustc: command_output("rustc", &["--version"])?,
        os: command_output("uname", &["-sr"])?,
        cpu: cpu_name(),
        operator: crate::telemetry_regression::operator_name(),
        ram: crate::telemetry_regression::ram_total(),
        cache_mode: "in_memory",
        git_dirty: git_dirty()?,
        dataset_identity: "synthetic_telemetry_v1_formula_1",
        fixture_source: "crates/benches/src/projection.rs::telemetry_rows",
        fixture_seed: "not_applicable_no_rng",
        schema_hashes: schema_hashes(),
        zstd_version: ZSTD_VERSION,
        zstd_level: DEFAULT_ZSTD_LEVEL,
        max_response_bytes: MAX_RESPONSE_BYTES,
        row_counts: row_counts.to_vec(),
        metrics,
        determinism,
    })
}

pub fn next_compression_run_path(report_dir: &Path) -> BenchResult<PathBuf> {
    fs::create_dir_all(report_dir)?;
    for idx in 1..=1_000_u32 {
        let path = report_dir.join(format!("compression_run_{idx}.json"));
        if !path.exists() {
            return Ok(path);
        }
    }
    Err(io::Error::other("no available compression run path").into())
}

pub fn write_report(path: &Path, report: &CompressionReport) -> BenchResult<()> {
    if path.exists() {
        return Err(io::Error::other("compression report already exists").into());
    }
    for metric in &report.metrics {
        validate_metric(metric)?;
    }
    let file = OpenOptions::new().write(true).create_new(true).open(path)?;
    serde_json::to_writer_pretty(file, report)?;
    Ok(())
}

pub fn write_summary(report_dir: &Path, report: &CompressionReport) -> BenchResult<()> {
    let mut out = String::new();
    out.push_str("# MBT Compression Summary\n\n");
    out.push_str("This summary is generated from the latest compression benchmark report.\n\n");
    out.push_str("| Label | Lane | Rows | Uncompressed bytes | Compressed bytes | Ratio | Compress MB/s | Decompress MB/s |\n");
    out.push_str("| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |\n");
    for metric in &report.metrics {
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {:.6} | {:.6} | {:.6} |\n",
            metric.label,
            metric.lane,
            metric.rows,
            metric.uncompressed_bytes,
            metric.compressed_bytes,
            metric.compression_ratio,
            metric.compress_mb_per_sec,
            metric.decompress_mb_per_sec
        ));
    }
    fs::write(report_dir.join("compression_summary.md"), out)?;
    Ok(())
}

pub fn write_environment(
    report_dir: &Path,
    report: &CompressionReport,
    report_path: &Path,
) -> BenchResult<()> {
    let text = format!(
        "# MBT Compression Benchmark Environment\n\n\
         - command: `{}`\n\
         - build profile: `{}`\n\
         - rustc: `{}`\n\
         - os: `{}`\n\
         - cpu: `{}`\n\
         - git dirty: `{}`\n\
         - zstd version: `{}`\n\
         - zstd level: `{}`\n\
         - report path: `{}`\n",
        report.command,
        report.build_profile,
        report.rustc,
        report.os,
        report.cpu,
        report.git_dirty,
        report.zstd_version,
        report.zstd_level,
        report_path.display()
    );
    fs::write(report_dir.join("benchmark_environment.md"), text)?;
    Ok(())
}

pub fn full_run_row_counts() -> &'static [RowCountConfig] {
    &FULL_ROW_COUNTS
}

pub fn smoke_run_row_counts() -> &'static [RowCountConfig] {
    &SMOKE_ROW_COUNTS
}

pub fn required_lanes() -> &'static [&'static str] {
    &["mbt_full", "mbt_temperature_only"]
}

pub fn sample_report() -> CompressionReport {
    let metric = CompressionMetric {
        label: "one",
        lane: "mbt_full",
        rows: 1,
        iterations: 3,
        uncompressed_bytes: 128,
        compressed_bytes: 64,
        compression_ratio: 0.5,
        reduction_percent: 50.0,
        compress_us: 1.0,
        decompress_us: 1.0,
        compress_mb_per_sec: 1.0,
        decompress_mb_per_sec: 1.0,
        p50_compress_us: 1.0,
        p95_compress_us: 1.0,
        p99_compress_us: 1.0,
        p50_decompress_us: 1.0,
        p95_decompress_us: 1.0,
        p99_decompress_us: 1.0,
        source_checksum: 1,
        compressed_checksum: 2,
        decompressed_checksum: 1,
        source_schema_hash: TelemetryV1::SCHEMA_HASH,
        byte_equal: true,
    };
    CompressionReport {
        experiment: "mbt_compression",
        status: "completed",
        utc_ms: 1_797_955_200_000,
        command: "sample".to_string(),
        build_profile: "release",
        rustc: "sample".to_string(),
        os: "sample".to_string(),
        cpu: "sample".to_string(),
        operator: "sample".to_string(),
        ram: "sample".to_string(),
        cache_mode: "in_memory",
        git_dirty: "sample".to_string(),
        dataset_identity: "synthetic_telemetry_v1_formula_1",
        fixture_source: "crates/benches/src/projection.rs::telemetry_rows",
        fixture_seed: "not_applicable_no_rng",
        schema_hashes: schema_hashes(),
        zstd_version: ZSTD_VERSION,
        zstd_level: DEFAULT_ZSTD_LEVEL,
        max_response_bytes: MAX_RESPONSE_BYTES,
        row_counts: SMOKE_ROW_COUNTS.to_vec(),
        metrics: vec![metric],
        determinism: DeterminismReport {
            all_byte_equal: true,
            source_checksum_stable: true,
            compressed_checksum_stable: true,
            decompressed_checksum_stable: true,
        },
    }
}

struct SourceBytes {
    lane: &'static str,
    schema_hash: u64,
    bytes: Vec<u8>,
}

struct MeasuredMetric {
    metric: CompressionMetric,
    compressed_checksum_stable: bool,
    decompressed_checksum_stable: bool,
}

fn selected_row_counts(smoke: bool) -> &'static [RowCountConfig] {
    if smoke {
        &SMOKE_ROW_COUNTS
    } else {
        &FULL_ROW_COUNTS
    }
}

fn source_bytes(row_count: usize) -> BenchResult<Vec<SourceBytes>> {
    let rows = telemetry_rows(row_count)?;
    let full = TelemetryV1::encode(&rows, MAX_RESPONSE_BYTES)?;
    let temperature_only = TelemetryV1::project_temperature_only(&full, MAX_RESPONSE_BYTES)?;
    Ok(vec![
        SourceBytes {
            lane: "mbt_full",
            schema_hash: TelemetryV1::SCHEMA_HASH,
            bytes: full,
        },
        SourceBytes {
            lane: "mbt_temperature_only",
            schema_hash: TelemetryV1TemperatureOnly::SCHEMA_HASH,
            bytes: temperature_only,
        },
    ])
}

fn measure_metric(config: &RowCountConfig, source: &SourceBytes) -> BenchResult<MeasuredMetric> {
    let source_checksum = response_checksum(&source.bytes);
    let mut compressed = Vec::with_capacity(source.bytes.len());
    let mut decompressed = Vec::with_capacity(source.bytes.len());
    let mut compress_samples = Vec::with_capacity(config.iterations);
    let mut decompress_samples = Vec::with_capacity(config.iterations);
    let mut compressed_bytes = 0;
    let mut first_compressed_checksum = None;
    let mut first_decompressed_checksum = None;
    let mut compressed_checksum_stable = true;
    let mut decompressed_checksum_stable = true;
    let mut all_byte_equal = true;

    for _ in 0..config.iterations {
        let compress_start = Instant::now();
        compress_into(
            &source.bytes,
            &mut compressed,
            MAX_RESPONSE_BYTES,
            CompressionConfig::default(),
        )?;
        compress_samples.push(compress_start.elapsed().as_secs_f64() * 1_000_000.0);

        let compressed_checksum = response_checksum(&compressed);
        match first_compressed_checksum {
            Some(first) if first != compressed_checksum => compressed_checksum_stable = false,
            Some(_) => {}
            None => {
                first_compressed_checksum = Some(compressed_checksum);
                compressed_bytes = compressed.len();
            }
        }

        let decompress_start = Instant::now();
        decompress_into(&compressed, &mut decompressed, source.bytes.len())?;
        decompress_samples.push(decompress_start.elapsed().as_secs_f64() * 1_000_000.0);

        let decompressed_checksum = response_checksum(&decompressed);
        match first_decompressed_checksum {
            Some(first) if first != decompressed_checksum => decompressed_checksum_stable = false,
            Some(_) => {}
            None => first_decompressed_checksum = Some(decompressed_checksum),
        }
        if decompressed != source.bytes {
            all_byte_equal = false;
        }
    }

    let compressed_checksum = match first_compressed_checksum {
        Some(value) => value,
        None => 0,
    };
    let decompressed_checksum = match first_decompressed_checksum {
        Some(value) => value,
        None => 0,
    };
    if !all_byte_equal || decompressed_checksum != source_checksum {
        return Err(io::Error::other(format!(
            "compression roundtrip failed for {} {}",
            config.label, source.lane
        ))
        .into());
    }

    if compress_samples
        .iter()
        .chain(&decompress_samples)
        .any(|value| !value.is_finite() || *value <= 0.0)
    {
        return Err(io::Error::other("invalid compression sample duration").into());
    }
    let compress_us = sum(&compress_samples);
    let decompress_us = sum(&decompress_samples);
    let uncompressed_bytes = source.bytes.len();
    let compression_ratio = ratio(compressed_bytes as f64, uncompressed_bytes as f64);
    let reduction_percent = (1.0 - compression_ratio) * 100.0;
    let metric = CompressionMetric {
        label: config.label,
        lane: source.lane,
        rows: config.rows,
        iterations: config.iterations,
        uncompressed_bytes,
        compressed_bytes,
        compression_ratio,
        reduction_percent,
        compress_us,
        decompress_us,
        compress_mb_per_sec: mb_per_second(uncompressed_bytes, config.iterations, compress_us),
        decompress_mb_per_sec: mb_per_second(uncompressed_bytes, config.iterations, decompress_us),
        p50_compress_us: percentile(&compress_samples, 0.50),
        p95_compress_us: percentile(&compress_samples, 0.95),
        p99_compress_us: percentile(&compress_samples, 0.99),
        p50_decompress_us: percentile(&decompress_samples, 0.50),
        p95_decompress_us: percentile(&decompress_samples, 0.95),
        p99_decompress_us: percentile(&decompress_samples, 0.99),
        source_checksum,
        compressed_checksum,
        decompressed_checksum,
        source_schema_hash: source.schema_hash,
        byte_equal: all_byte_equal,
    };
    validate_metric(&metric)?;
    Ok(MeasuredMetric {
        metric,
        compressed_checksum_stable,
        decompressed_checksum_stable,
    })
}

fn schema_hashes() -> SchemaHashes {
    SchemaHashes {
        mbt_full: TelemetryV1::SCHEMA_HASH,
        mbt_temperature_only: TelemetryV1TemperatureOnly::SCHEMA_HASH,
    }
}

fn determinism_report(
    metrics: &[CompressionMetric],
    compressed_checksum_stable: bool,
    mut decompressed_checksum_stable: bool,
) -> DeterminismReport {
    let mut all_byte_equal = true;
    let mut source_checksum_stable = true;
    for metric in metrics {
        if !metric.byte_equal {
            all_byte_equal = false;
        }
        if metric.source_checksum != metric.decompressed_checksum {
            source_checksum_stable = false;
            decompressed_checksum_stable = false;
        }
    }
    DeterminismReport {
        all_byte_equal,
        source_checksum_stable,
        compressed_checksum_stable,
        decompressed_checksum_stable,
    }
}

fn validate_metric(metric: &CompressionMetric) -> BenchResult<()> {
    let finite = metric.compression_ratio.is_finite()
        && metric.reduction_percent.is_finite()
        && metric.compress_us.is_finite()
        && metric.decompress_us.is_finite()
        && metric.compress_mb_per_sec.is_finite()
        && metric.decompress_mb_per_sec.is_finite()
        && metric.p50_compress_us.is_finite()
        && metric.p95_compress_us.is_finite()
        && metric.p99_compress_us.is_finite()
        && metric.p50_decompress_us.is_finite()
        && metric.p95_decompress_us.is_finite()
        && metric.p99_decompress_us.is_finite();
    if !finite {
        return Err(io::Error::other(format!(
            "non-finite compression metric {} {}",
            metric.label, metric.lane
        ))
        .into());
    }
    if metric.uncompressed_bytes == 0 || metric.compressed_bytes == 0 {
        return Err(
            io::Error::other(format!("zero byte metric {} {}", metric.label, metric.lane)).into(),
        );
    }
    if !metric.byte_equal {
        return Err(io::Error::other(format!(
            "byte equality failed {} {}",
            metric.label, metric.lane
        ))
        .into());
    }
    Ok(())
}

fn percentile(samples: &[f64], percentile: f64) -> f64 {
    if samples.is_empty() {
        return 0.0;
    }
    let mut sorted = samples.to_vec();
    sorted.sort_by(|left, right| left.total_cmp(right));
    let rank = ((sorted.len() as f64) * percentile).ceil() as usize;
    let index = rank.saturating_sub(1).min(sorted.len() - 1);
    sorted[index]
}

fn sum(samples: &[f64]) -> f64 {
    samples.iter().copied().sum()
}

fn mb_per_second(uncompressed_bytes: usize, iterations: usize, elapsed_us: f64) -> f64 {
    if elapsed_us <= 0.0 {
        return 0.0;
    }
    let bytes = uncompressed_bytes as f64 * iterations as f64;
    let seconds = elapsed_us / 1_000_000.0;
    bytes / (1024.0 * 1024.0) / seconds
}

fn ratio(numerator: f64, denominator: f64) -> f64 {
    if denominator <= 0.0 {
        0.0
    } else {
        numerator / denominator
    }
}

fn utc_millis() -> BenchResult<u128> {
    Ok(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_millis())
}

fn command_output(command: &str, args: &[&str]) -> BenchResult<String> {
    let output = Command::new(command).args(args).output()?;
    if !output.status.success() {
        return Err(io::Error::other(format!("metadata command failed: {command}")).into());
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

fn git_dirty() -> BenchResult<String> {
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

fn build_profile() -> &'static str {
    if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    }
}
