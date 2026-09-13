use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};

use mbt_core::envelope::fnv1a64;
use mbt_core::runtime::BinaryInspection;
use mbt_schema_telemetry::telemetry_v1::*;
use mbt_schema_test_compatibility::test_compatibility_v1::{self as tc, TestCompatibilityRowV1};
use serde::Serialize;

pub const MAX_RESPONSE_BYTES: usize = 1_073_741_824;
pub const ROW_COUNTS: [usize; 6] = [1, 100, 500, 1_000, 10_000, 100_000];
pub const DATASET_IDENTITY: &str = "synthetic_telemetry_v1_formula_1";
pub type BenchResult<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SchemaName {
    Telemetry,
    TestCompatibility,
}

#[derive(Clone, Debug, Serialize)]
pub struct BenchRow {
    pub label: &'static str,
    pub schema: SchemaName,
    pub schema_hash: u64,
    pub row_count: usize,
    pub output_bytes: usize,
    pub total_milliseconds: f64,
    pub rows_per_second: f64,
    pub mb_per_second: f64,
    pub source_access_milliseconds: f64,
    pub projection_milliseconds: f64,
    pub projected_inspect_milliseconds: f64,
    pub response_checksum: u64,
    pub semantic_checksum: Option<u64>,
    pub minimal_projection_checksum: Option<u64>,
}

pub fn telemetry_rows(row_count: usize) -> BenchResult<Vec<TelemetryRowV1>> {
    if !ROW_COUNTS.contains(&row_count) {
        return Err(io::Error::other("unsupported telemetry benchmark row count").into());
    }
    let mut rows = Vec::with_capacity(row_count);
    let statuses = [STATUS_ACTIVE, STATUS_IDLE, STATUS_OFFLINE];
    for idx in 0..row_count {
        let tick = i64::try_from(idx)?;
        let even = idx % 2 == 0;
        rows.push(TelemetryRowV1 {
            schema_version: SCHEMA_VERSION_VALUE,
            device_ordinal: DEVICE_SENSOR_A,
            recorded_at_ms: 1_700_000_000_000 + tick * 1_000,
            temperature_c: 20.0 + f64::from(u32::try_from(idx % 16)?) / 4.0,
            battery_percent: if even { 50.0 } else { 0.0 },
            status_ordinal: statuses[idx % statuses.len()],
            tags_mask: (1 << TAG_TEST_BIT)
                | (1 << if even {
                    TAG_INDOOR_BIT
                } else {
                    TAG_OUTDOOR_BIT
                }),
            presence_bits: if even { PRESENCE_BATTERY_PERCENT } else { 0 },
        });
    }
    Ok(rows)
}

pub fn test_compatibility_rows(row_count: usize) -> Vec<TestCompatibilityRowV1> {
    let mut rows = Vec::with_capacity(row_count);
    for idx in 0..row_count {
        let entity = tc::ENTITY_ENTITY_A;
        let tenant = tc::TENANT_ALPHA;
        let recorded_at_ms = 1_800_000_000_000_i64 + idx as i64;
        rows.push(TestCompatibilityRowV1 {
            schema_version: tc::SCHEMA_VERSION_VALUE,
            tenant_ordinal: tenant,
            entity_ordinal: entity,
            recorded_at_ms,
            status_ordinal: tc::STATUS_ACTIVE,
            optional_status_ordinal: tc::STATUS_PAUSED,
            sites_mask: (1_u64 << tc::SITE_SITE_A_BIT) | (1_u64 << tc::SITE_SITE_B_BIT),
            required_i64: idx as i64,
            optional_i64: idx as i64 + 1,
            required_i32: idx as i32,
            optional_i32: idx as i32 + 1,
            required_u32: idx as u32,
            optional_u32: idx as u32 + 1,
            required_f64: idx as f64 + 0.5,
            optional_f64: idx as f64 + 1.5,
            required_f32: idx as f32 + 0.25,
            optional_f32: idx as f32 + 1.25,
            required_bool: idx % 2 == 0,
            optional_bool: idx % 2 != 0,
            required_text: format!("required-{idx}"),
            optional_text: format!("optional-{idx}"),
            required_bytes: vec![idx as u8, idx.wrapping_add(1) as u8],
            optional_bytes: vec![idx.wrapping_add(2) as u8],
            uuid_text: format!("00000000-0000-0000-0000-{idx:012}"),
            jsonb_text: format!(r#"{{"idx":{idx}}}"#),
            timestamptz_text: "2026-01-01T00:00:00Z".to_string(),
            numeric_text: format!("{idx}.125"),
            required_i64_array: vec![idx as i64, idx as i64 + 1],
            nullable_i64_array: vec![idx as i64 + 2],
            required_i32_array: vec![idx as i32, idx as i32 + 1],
            nullable_i32_array: vec![idx as i32 + 2],
            required_u32_array: vec![idx as u32, idx as u32 + 1],
            nullable_u32_array: vec![idx as u32 + 2],
            required_f64_array: vec![idx as f64 + 0.5],
            nullable_f64_array: vec![idx as f64 + 1.5],
            required_f32_array: vec![idx as f32 + 0.5],
            nullable_f32_array: vec![idx as f32 + 1.5],
            presence_bits: tc::PRESENCE_ALLOWED_MASK,
        });
    }
    rows
}

pub fn next_run_path(report_dir: &Path) -> BenchResult<PathBuf> {
    fs::create_dir_all(report_dir)?;
    for idx in 1..=1_000_u32 {
        let path = report_dir.join(format!("projection_run_{idx}.json"));
        if !path.exists() {
            return Ok(path);
        }
    }
    Err(io::Error::other("no available projection run path").into())
}

pub fn write_report(path: &Path, rows: &[BenchRow]) -> BenchResult<()> {
    for row in rows {
        measured_rates(row.row_count, row.output_bytes, row.total_milliseconds)?;
        for duration in [
            row.source_access_milliseconds,
            row.projection_milliseconds,
            row.projected_inspect_milliseconds,
        ] {
            if !duration.is_finite() || duration < 0.0 {
                return Err(io::Error::other("invalid component duration").into());
            }
        }
    }
    let metadata = crate::telemetry_regression::metadata_for_run(
        std::env::args().collect::<Vec<_>>().join(" "),
        path,
    )?;
    let file = OpenOptions::new().write(true).create_new(true).open(path)?;
    #[derive(Serialize)]
    struct Report<'a> {
        metadata: crate::telemetry_regression::TelemetryRegressionMetadata,
        compatibility_dataset_identity: &'static str,
        rows: &'a [BenchRow],
    }
    serde_json::to_writer_pretty(
        file,
        &Report {
            metadata,
            compatibility_dataset_identity: "synthetic_all_fields_v2",
            rows,
        },
    )?;
    Ok(())
}

pub fn measured_rates(
    row_count: usize,
    output_bytes: usize,
    milliseconds: f64,
) -> BenchResult<(f64, f64)> {
    if !milliseconds.is_finite() || milliseconds <= 0.0 || output_bytes == 0 {
        return Err(io::Error::other("invalid benchmark duration or output size").into());
    }
    let seconds = milliseconds / 1_000.0;
    let rates = (
        row_count as f64 / seconds,
        output_bytes as f64 / (1024.0 * 1024.0) / seconds,
    );
    if !rates.0.is_finite() || !rates.1.is_finite() {
        return Err(io::Error::other("non-finite benchmark rate").into());
    }
    Ok(rates)
}

pub fn response_checksum(bytes: &[u8]) -> u64 {
    fnv1a64(bytes)
}
pub fn inspection_checksums(inspection: BinaryInspection) -> (Option<u64>, Option<u64>) {
    (
        Some(inspection.semantic_checksum),
        Some(inspection.minimal_projection_checksum),
    )
}
pub fn required_labels() -> &'static [&'static str] {
    &[
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
}
