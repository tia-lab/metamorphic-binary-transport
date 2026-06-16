use std::error::Error;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use metamorphic_binary_transport_core::envelope::fnv1a64;
use metamorphic_binary_transport_core::runtime::BinaryInspection;
use metamorphic_binary_transport_schema_bars::bars_v1::{self, MathildeBarRowV1};
use metamorphic_binary_transport_schema_test_compatibility::test_compatibility_v1::{
    self as tc, TestCompatibilityRowV1,
};

pub const MAX_RESPONSE_BYTES: usize = 1_073_741_824;
pub const ROW_COUNTS: [usize; 6] = [1, 100, 500, 1_000, 10_000, 100_000];
// Historical old-crate table is the regression baseline for Bars projections.
pub const OLD_BENCH_RESULTS: &str = "/media/Development/MATHILDE/experiments/crates/mathilde-binary-transport/docs/bench_results.md";

pub type BenchResult<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Clone, Copy, Debug)]
pub enum SchemaName {
    // Bench reports separate Bars from the all-fields compatibility schema.
    Bars,
    TestCompatibility,
}

impl SchemaName {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Bars => "bars",
            Self::TestCompatibility => "test_compatibility",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Comparison {
    pub rows_per_second: f64,
    pub mb_per_second: f64,
    pub ratio_rows_per_second: f64,
}

#[derive(Clone, Debug)]
pub struct BenchRow {
    // One row records one projection path and its timing components.
    pub label: &'static str,
    pub schema: SchemaName,
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
    pub old_crate_comparison: Option<Comparison>,
    pub current_owned_row_comparison: Option<Comparison>,
}

#[derive(Clone, Debug)]
pub struct BaselineEntry {
    pub label: String,
    pub row_count: usize,
    pub rows_per_second: f64,
    pub mb_per_second: f64,
}

pub fn bars_rows(row_count: usize) -> Vec<MathildeBarRowV1> {
    let mut rows = Vec::with_capacity(row_count);
    for idx in 0..row_count {
        let close_ms = 1_700_000_000_000_i64 + (idx as i64 * 60_000);
        let base = 100.0 + (idx % 10_000) as f64 * 0.01;
        rows.push(MathildeBarRowV1 {
            schema_version: bars_v1::SCHEMA_VERSION_VALUE,
            pair_ordinal: bars_v1::PAIR_BTCUSDT,
            tf_ordinal: bars_v1::TIMEFRAME_1M,
            open_ms: close_ms - 60_000,
            close_ms,
            o: base,
            h: base + 1.0,
            l: base - 1.0,
            c: base + 0.25,
            v: 1_000.0 + idx as f64,
            quote_v: 100_000.0 + idx as f64,
            taker_known_v: 500.0 + idx as f64,
            taker_signed_v: -10.0 + (idx % 20) as f64,
            taker_known_quote_v: 50_000.0 + idx as f64,
            taker_signed_quote_v: -1_000.0 + idx as f64,
            taker_known_n: idx as i64,
            taker_signed_n: (idx as i64) - 10,
            vw: base + 0.1,
            n: (idx as i64) + 1,
            source_ordinal: bars_v1::SOURCE_FRONTIER,
            process_ordinal: bars_v1::PROCESS_DERIVED,
            venues_expected_mask: (1_u64 << bars_v1::VENUE_BINANCE_BIT)
                | (1_u64 << bars_v1::VENUE_BYBIT_BIT),
            venues_with_trades_mask: 1_u64 << bars_v1::VENUE_BINANCE_BIT,
            ingested_at_ms: close_ms + 1,
            target_ingested_at_ms: close_ms + 2,
            built_at_ms: close_ms + 3,
            committed_at_ms: close_ms + 4,
            harmonized_at_ms: close_ms + 5,
            recomputed_at_ms: close_ms + 6,
            recomputed_reason_ordinal: bars_v1::RECOMPUTED_REASON_CANONICAL_REPAIR,
            covered_1m_count: 1,
            expected_1m_count: 1,
            coverage_ratio: 1.0,
            inputs_source_counts_frontier: 1,
            inputs_source_counts_api: 0,
            inputs_source_counts_synthetic: 0,
            inputs_source_counts_fix_data: 0,
            frontier_5s_inputs_coverage_ratio: 1.0,
            frontier_5s_expected: 12,
            frontier_5s_synth_n: 0,
            frontier_5s_synth_ratio: 0.0,
            frontier_5s_trade_n: 12,
            frontier_5s_trade_ratio: 1.0,
            age_ms: 100,
            presence_bits: bars_v1::PRESENCE_ALLOWED_MASK,
        });
    }
    rows
}

pub fn test_compatibility_rows(row_count: usize) -> Vec<TestCompatibilityRowV1> {
    let mut rows = Vec::with_capacity(row_count);
    for idx in 0..row_count {
        let entity = tc::ENTITY_ENTITY_A;
        let tenant = tc::TENANT_ALPHA;
        let close_ms = 1_800_000_000_000_i64 + idx as i64;
        rows.push(TestCompatibilityRowV1 {
            schema_version: tc::SCHEMA_VERSION_VALUE,
            tenant_ordinal: tenant,
            entity_ordinal: entity,
            close_ms,
            status_ordinal: tc::STATUS_ACTIVE,
            optional_status_ordinal: tc::STATUS_PAUSED,
            venues_mask: (1_u64 << tc::VENUE_BINANCE_BIT) | (1_u64 << tc::VENUE_BYBIT_BIT),
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
    if path.exists() {
        return Err(io::Error::other("projection report already exists").into());
    }
    let mut out = String::new();
    out.push_str("{\n  \"rows\": [\n");
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

pub fn write_summary(report_dir: &Path) -> BenchResult<()> {
    let run_1 = report_dir.join("projection_run_1.json");
    let run_2 = report_dir.join("projection_run_2.json");
    let run_3 = report_dir.join("projection_run_3.json");
    if !(run_1.exists() && run_2.exists() && run_3.exists()) {
        return Ok(());
    }
    let summary = report_dir.join("projection_summary.md");
    let text = "# MBT Projection Direct Writer Summary\n\nThree projection benchmark run files are present.\n";
    fs::write(summary, text)?;
    Ok(())
}

pub fn parse_old_crate_projection_baselines(path: &Path) -> BenchResult<Vec<BaselineEntry>> {
    // Old MBT projection baselines are parsed from the canonical markdown table.
    let text = fs::read_to_string(path)?;
    let mut entries = Vec::new();
    for line in text.lines() {
        if !line.contains("mathilde_binary_project_") {
            continue;
        }
        let columns = markdown_columns(line);
        if columns.len() < 7 {
            continue;
        }
        let label = columns[2].trim_matches('`').to_string();
        let Some(row_count) = parse_usize(columns[1]) else {
            continue;
        };
        let Some(rows_per_second) = parse_f64(columns[5]) else {
            continue;
        };
        let Some(mb_per_second) = parse_f64(columns[6]) else {
            continue;
        };
        entries.push(BaselineEntry {
            label,
            row_count,
            rows_per_second,
            mb_per_second,
        });
    }
    Ok(entries)
}

pub fn parse_current_owned_baselines(path: &Path) -> BenchResult<Vec<BaselineEntry>> {
    // Current owned-row baselines are optional because they are produced by a separate run.
    if !path.exists() {
        return Ok(Vec::new());
    }
    parse_json_baselines(&fs::read_to_string(path)?)
}

pub fn comparison_for(
    baselines: &[BaselineEntry],
    label: &str,
    row_count: usize,
    observed_rows_per_second: f64,
) -> Option<Comparison> {
    baselines
        .iter()
        .find(|entry| entry.label == label && entry.row_count == row_count)
        .map(|entry| Comparison {
            rows_per_second: entry.rows_per_second,
            mb_per_second: entry.mb_per_second,
            ratio_rows_per_second: ratio(observed_rows_per_second, entry.rows_per_second),
        })
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
    fnv1a64(bytes)
}

pub fn inspection_checksums(inspection: BinaryInspection) -> (Option<u64>, Option<u64>) {
    // Projection inspections provide semantic and minimal checksum evidence.
    (
        Some(inspection.semantic_checksum),
        Some(inspection.minimal_projection_checksum),
    )
}

pub fn required_labels() -> &'static [&'static str] {
    &[
        "mathilde_binary_project_no_metadata_public",
        "mathilde_binary_project_no_metadata_archived",
        "mathilde_binary_project_no_metadata_inspect",
        "mathilde_binary_project_ohlcv_only_public",
        "mathilde_binary_project_ohlcv_only_archived",
        "mathilde_binary_project_ohlcv_only_inspect",
        "mbt_project_no_optional_public",
        "mbt_project_no_optional_archived",
        "mbt_project_no_optional_inspect",
        "mbt_project_numeric_only_public",
        "mbt_project_numeric_only_archived",
        "mbt_project_numeric_only_inspect",
    ]
}

fn write_row_json(out: &mut String, row: &BenchRow) {
    out.push_str("    {");
    write_string_field(out, "label", row.label, true);
    write_string_field(out, "schema", row.schema.as_str(), false);
    write_usize_field(out, "row_count", row.row_count);
    write_usize_field(out, "output_bytes", row.output_bytes);
    write_f64_field(out, "total_milliseconds", row.total_milliseconds);
    write_f64_field(out, "rows_per_second", row.rows_per_second);
    write_f64_field(out, "mb_per_second", row.mb_per_second);
    write_f64_field(
        out,
        "source_access_milliseconds",
        row.source_access_milliseconds,
    );
    write_f64_field(out, "projection_milliseconds", row.projection_milliseconds);
    write_f64_field(
        out,
        "projected_inspect_milliseconds",
        row.projected_inspect_milliseconds,
    );
    write_u64_field(out, "response_checksum", row.response_checksum);
    write_optional_u64_field(out, "semantic_checksum", row.semantic_checksum);
    write_optional_u64_field(
        out,
        "minimal_projection_checksum",
        row.minimal_projection_checksum,
    );
    write_optional_comparison(
        out,
        "old_crate_comparison",
        row.old_crate_comparison.as_ref(),
    );
    write_optional_comparison(
        out,
        "current_owned_row_comparison",
        row.current_owned_row_comparison.as_ref(),
    );
    out.push_str(" }");
}

fn write_string_field(out: &mut String, name: &str, value: &str, first: bool) {
    if !first {
        out.push(',');
    }
    out.push_str(&format!(" \"{name}\": \"{}\"", escape_json(value)));
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

fn markdown_columns(line: &str) -> Vec<&str> {
    line.trim()
        .trim_matches('|')
        .split('|')
        .map(str::trim)
        .collect()
}

fn parse_usize(value: &str) -> Option<usize> {
    value.replace(',', "").parse::<usize>().ok()
}

fn parse_f64(value: &str) -> Option<f64> {
    value.replace(',', "").parse::<f64>().ok()
}

fn parse_json_baselines(text: &str) -> BenchResult<Vec<BaselineEntry>> {
    let mut entries = Vec::new();
    let mut cursor = text;
    while let Some(label_pos) = cursor.find("\"label\":") {
        cursor = &cursor[label_pos..];
        let label = json_string_after(cursor, "\"label\":")?;
        let row_count = json_number_after(cursor, "\"row_count\":")? as usize;
        let rows_per_second = json_float_after(cursor, "\"rows_per_second\":")?;
        let mb_per_second = json_float_after(cursor, "\"mb_per_second\":")?;
        entries.push(BaselineEntry {
            label,
            row_count,
            rows_per_second,
            mb_per_second,
        });
        cursor = &cursor["\"label\":".len()..];
    }
    Ok(entries)
}

fn json_string_after(haystack: &str, key: &str) -> BenchResult<String> {
    let start = haystack
        .find(key)
        .ok_or_else(|| io::Error::other(format!("missing JSON key {key}")))?;
    let after_key = &haystack[start + key.len()..];
    let quote_start = after_key
        .find('"')
        .ok_or_else(|| io::Error::other("missing JSON string start"))?;
    let after_quote = &after_key[quote_start + 1..];
    let quote_end = after_quote
        .find('"')
        .ok_or_else(|| io::Error::other("missing JSON string end"))?;
    Ok(after_quote[..quote_end].to_string())
}

fn json_number_after(haystack: &str, key: &str) -> BenchResult<u64> {
    let text = json_scalar_after(haystack, key)?;
    Ok(text.parse::<u64>()?)
}

fn json_float_after(haystack: &str, key: &str) -> BenchResult<f64> {
    let text = json_scalar_after(haystack, key)?;
    Ok(text.parse::<f64>()?)
}

fn json_scalar_after<'a>(haystack: &'a str, key: &str) -> BenchResult<&'a str> {
    let start = haystack
        .find(key)
        .ok_or_else(|| io::Error::other(format!("missing JSON key {key}")))?;
    let after_key = haystack[start + key.len()..].trim_start();
    let end = after_key
        .find([',', '}'])
        .ok_or_else(|| io::Error::other("missing JSON scalar end"))?;
    Ok(after_key[..end].trim())
}

fn ratio(observed: f64, baseline: f64) -> f64 {
    if baseline <= 0.0 {
        0.0
    } else {
        observed / baseline
    }
}
