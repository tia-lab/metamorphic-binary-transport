use std::error::Error;
use std::io;
use std::path::PathBuf;
use std::time::Instant;

use metamorphic_binary_transport_benches::projection::{
    BenchResult, BenchRow, MAX_RESPONSE_BYTES, OLD_BENCH_RESULTS, SchemaName, comparison_for,
    inspection_checksums, measured_rates, next_run_path, parse_current_owned_baselines,
    parse_old_crate_projection_baselines, response_checksum, write_report, write_summary,
};
use metamorphic_binary_transport_schema_bars::bars_v1::{
    BarsV1, BarsV1NoMetadata, BarsV1OhlcvOnly,
};
use metamorphic_binary_transport_schema_test_compatibility::test_compatibility_v1::{
    TestCompatibilityV1, TestCompatibilityV1NoOptional, TestCompatibilityV1NumericOnly,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> BenchResult<()> {
    let report_dir = parse_report_dir(std::env::args().skip(1))?;
    let old_baselines = parse_old_crate_projection_baselines(OLD_BENCH_RESULTS.as_ref())?;
    verify_old_baselines(&old_baselines)?;
    let current_baselines = parse_current_owned_baselines(
        "docs/evidence/mbt_projection_direct_writer/current_owned_row_projection_baseline.json"
            .as_ref(),
    )?;

    let mut rows = Vec::new();
    for row_count in metamorphic_binary_transport_benches::projection::ROW_COUNTS {
        let bars_rows = metamorphic_binary_transport_benches::projection::bars_rows(row_count);
        let bars_source = BarsV1::encode(&bars_rows, MAX_RESPONSE_BYTES)?;
        rows.extend(measure_bars(
            row_count,
            &bars_source,
            &old_baselines,
            &current_baselines,
        )?);

        let compatibility_rows =
            metamorphic_binary_transport_benches::projection::test_compatibility_rows(row_count);
        let compatibility_source =
            TestCompatibilityV1::encode(&compatibility_rows, MAX_RESPONSE_BYTES)?;
        rows.extend(measure_compatibility(
            row_count,
            &compatibility_source,
            &current_baselines,
        )?);
    }

    let path = next_run_path(&report_dir)?;
    write_report(&path, &rows)?;
    write_summary(&report_dir)?;
    println!("{}", path.display());
    Ok(())
}

fn parse_report_dir(args: impl Iterator<Item = String>) -> BenchResult<PathBuf> {
    let collected = args.collect::<Vec<_>>();
    if collected.len() != 2 || collected[0] != "--report-dir" {
        return Err(io::Error::other("usage: mbt_projection_bench --report-dir <path>").into());
    }
    Ok(PathBuf::from(&collected[1]))
}

fn verify_old_baselines(
    baselines: &[metamorphic_binary_transport_benches::projection::BaselineEntry],
) -> BenchResult<()> {
    for label in [
        "mathilde_binary_project_no_metadata_public",
        "mathilde_binary_project_no_metadata_archived",
        "mathilde_binary_project_no_metadata_inspect",
        "mathilde_binary_project_ohlcv_only_public",
        "mathilde_binary_project_ohlcv_only_archived",
        "mathilde_binary_project_ohlcv_only_inspect",
    ] {
        for row_count in metamorphic_binary_transport_benches::projection::ROW_COUNTS {
            if !baselines
                .iter()
                .any(|entry| entry.label == label && entry.row_count == row_count)
            {
                return Err(io::Error::other(format!(
                    "missing old projection baseline {label} row_count={row_count}"
                ))
                .into());
            }
        }
    }
    Ok(())
}

fn measure_bars(
    row_count: usize,
    source: &[u8],
    old_baselines: &[metamorphic_binary_transport_benches::projection::BaselineEntry],
    current_baselines: &[metamorphic_binary_transport_benches::projection::BaselineEntry],
) -> BenchResult<Vec<BenchRow>> {
    let mut out = Vec::with_capacity(6);
    out.push(measure_public(
        "mathilde_binary_project_no_metadata_public",
        SchemaName::Bars,
        row_count,
        source,
        |bytes| BarsV1::project_no_metadata(bytes, MAX_RESPONSE_BYTES),
        |bytes| BarsV1NoMetadata::inspect(bytes),
        old_baselines,
        current_baselines,
    )?);
    out.push(measure_archived(
        "mathilde_binary_project_no_metadata_archived",
        SchemaName::Bars,
        row_count,
        source,
        |bytes| BarsV1::access(bytes).map(|_| ()),
        |bytes| unsafe { BarsV1::project_no_metadata_trusted_unchecked(bytes, MAX_RESPONSE_BYTES) },
        |bytes| BarsV1NoMetadata::inspect(bytes),
        old_baselines,
        current_baselines,
    )?);
    out.push(measure_inspect(
        "mathilde_binary_project_no_metadata_inspect",
        SchemaName::Bars,
        row_count,
        source,
        |bytes| unsafe { BarsV1::project_no_metadata_trusted_unchecked(bytes, MAX_RESPONSE_BYTES) },
        |bytes| BarsV1NoMetadata::inspect(bytes),
        old_baselines,
        current_baselines,
    )?);
    out.push(measure_public(
        "mathilde_binary_project_ohlcv_only_public",
        SchemaName::Bars,
        row_count,
        source,
        |bytes| BarsV1::project_ohlcv_only(bytes, MAX_RESPONSE_BYTES),
        |bytes| BarsV1OhlcvOnly::inspect(bytes),
        old_baselines,
        current_baselines,
    )?);
    out.push(measure_archived(
        "mathilde_binary_project_ohlcv_only_archived",
        SchemaName::Bars,
        row_count,
        source,
        |bytes| BarsV1::access(bytes).map(|_| ()),
        |bytes| unsafe { BarsV1::project_ohlcv_only_trusted_unchecked(bytes, MAX_RESPONSE_BYTES) },
        |bytes| BarsV1OhlcvOnly::inspect(bytes),
        old_baselines,
        current_baselines,
    )?);
    out.push(measure_inspect(
        "mathilde_binary_project_ohlcv_only_inspect",
        SchemaName::Bars,
        row_count,
        source,
        |bytes| unsafe { BarsV1::project_ohlcv_only_trusted_unchecked(bytes, MAX_RESPONSE_BYTES) },
        |bytes| BarsV1OhlcvOnly::inspect(bytes),
        old_baselines,
        current_baselines,
    )?);
    Ok(out)
}

fn measure_compatibility(
    row_count: usize,
    source: &[u8],
    current_baselines: &[metamorphic_binary_transport_benches::projection::BaselineEntry],
) -> BenchResult<Vec<BenchRow>> {
    let empty_old = [];
    let mut out = Vec::with_capacity(6);
    out.push(measure_public(
        "mbt_project_no_optional_public",
        SchemaName::TestCompatibility,
        row_count,
        source,
        |bytes| TestCompatibilityV1::project_no_optional(bytes, MAX_RESPONSE_BYTES),
        |bytes| TestCompatibilityV1NoOptional::inspect(bytes),
        &empty_old,
        current_baselines,
    )?);
    out.push(measure_archived(
        "mbt_project_no_optional_archived",
        SchemaName::TestCompatibility,
        row_count,
        source,
        |bytes| TestCompatibilityV1::access(bytes).map(|_| ()),
        |bytes| unsafe {
            TestCompatibilityV1::project_no_optional_trusted_unchecked(bytes, MAX_RESPONSE_BYTES)
        },
        |bytes| TestCompatibilityV1NoOptional::inspect(bytes),
        &empty_old,
        current_baselines,
    )?);
    out.push(measure_inspect(
        "mbt_project_no_optional_inspect",
        SchemaName::TestCompatibility,
        row_count,
        source,
        |bytes| unsafe {
            TestCompatibilityV1::project_no_optional_trusted_unchecked(bytes, MAX_RESPONSE_BYTES)
        },
        |bytes| TestCompatibilityV1NoOptional::inspect(bytes),
        &empty_old,
        current_baselines,
    )?);
    out.push(measure_public(
        "mbt_project_numeric_only_public",
        SchemaName::TestCompatibility,
        row_count,
        source,
        |bytes| TestCompatibilityV1::project_numeric_only(bytes, MAX_RESPONSE_BYTES),
        |bytes| TestCompatibilityV1NumericOnly::inspect(bytes),
        &empty_old,
        current_baselines,
    )?);
    out.push(measure_archived(
        "mbt_project_numeric_only_archived",
        SchemaName::TestCompatibility,
        row_count,
        source,
        |bytes| TestCompatibilityV1::access(bytes).map(|_| ()),
        |bytes| unsafe {
            TestCompatibilityV1::project_numeric_only_trusted_unchecked(bytes, MAX_RESPONSE_BYTES)
        },
        |bytes| TestCompatibilityV1NumericOnly::inspect(bytes),
        &empty_old,
        current_baselines,
    )?);
    out.push(measure_inspect(
        "mbt_project_numeric_only_inspect",
        SchemaName::TestCompatibility,
        row_count,
        source,
        |bytes| unsafe {
            TestCompatibilityV1::project_numeric_only_trusted_unchecked(bytes, MAX_RESPONSE_BYTES)
        },
        |bytes| TestCompatibilityV1NumericOnly::inspect(bytes),
        &empty_old,
        current_baselines,
    )?);
    Ok(out)
}

fn measure_public<F, I, E>(
    label: &'static str,
    schema: SchemaName,
    row_count: usize,
    source: &[u8],
    project: F,
    inspect: I,
    old_baselines: &[metamorphic_binary_transport_benches::projection::BaselineEntry],
    current_baselines: &[metamorphic_binary_transport_benches::projection::BaselineEntry],
) -> BenchResult<BenchRow>
where
    F: FnOnce(&[u8]) -> Result<Vec<u8>, E>,
    I: FnOnce(&[u8]) -> Result<metamorphic_binary_transport_core::runtime::BinaryInspection, E>,
    E: Error + 'static,
{
    let started = Instant::now();
    let projected = project(source)?;
    let projection_ms = elapsed_ms(started);
    build_row(
        label,
        schema,
        row_count,
        projected,
        0.0,
        projection_ms,
        0.0,
        inspect,
        old_baselines,
        current_baselines,
    )
}

fn measure_archived<A, F, I, E>(
    label: &'static str,
    schema: SchemaName,
    row_count: usize,
    source: &[u8],
    access: A,
    project: F,
    inspect: I,
    old_baselines: &[metamorphic_binary_transport_benches::projection::BaselineEntry],
    current_baselines: &[metamorphic_binary_transport_benches::projection::BaselineEntry],
) -> BenchResult<BenchRow>
where
    A: FnOnce(&[u8]) -> Result<(), E>,
    F: FnOnce(&[u8]) -> Result<Vec<u8>, E>,
    I: FnOnce(&[u8]) -> Result<metamorphic_binary_transport_core::runtime::BinaryInspection, E>,
    E: Error + 'static,
{
    let access_started = Instant::now();
    access(source)?;
    let access_ms = elapsed_ms(access_started);
    let projection_started = Instant::now();
    let projected = project(source)?;
    let projection_ms = elapsed_ms(projection_started);
    build_row(
        label,
        schema,
        row_count,
        projected,
        access_ms,
        projection_ms,
        0.0,
        inspect,
        old_baselines,
        current_baselines,
    )
}

fn measure_inspect<F, I, E>(
    label: &'static str,
    schema: SchemaName,
    row_count: usize,
    source: &[u8],
    project: F,
    inspect: I,
    old_baselines: &[metamorphic_binary_transport_benches::projection::BaselineEntry],
    current_baselines: &[metamorphic_binary_transport_benches::projection::BaselineEntry],
) -> BenchResult<BenchRow>
where
    F: FnOnce(&[u8]) -> Result<Vec<u8>, E>,
    I: FnOnce(&[u8]) -> Result<metamorphic_binary_transport_core::runtime::BinaryInspection, E>,
    E: Error + 'static,
{
    let projected = project(source)?;
    let inspect_started = Instant::now();
    let inspection = inspect(&projected)?;
    let inspect_ms = elapsed_ms(inspect_started);
    build_inspected_row(
        label,
        schema,
        row_count,
        projected,
        0.0,
        0.0,
        inspect_ms,
        inspection,
        old_baselines,
        current_baselines,
    )
}

fn build_row<I, E>(
    label: &'static str,
    schema: SchemaName,
    row_count: usize,
    projected: Vec<u8>,
    access_ms: f64,
    projection_ms: f64,
    inspect_ms: f64,
    inspect: I,
    old_baselines: &[metamorphic_binary_transport_benches::projection::BaselineEntry],
    current_baselines: &[metamorphic_binary_transport_benches::projection::BaselineEntry],
) -> BenchResult<BenchRow>
where
    I: FnOnce(&[u8]) -> Result<metamorphic_binary_transport_core::runtime::BinaryInspection, E>,
    E: Error + 'static,
{
    let inspection = inspect(&projected)?;
    build_inspected_row(
        label,
        schema,
        row_count,
        projected,
        access_ms,
        projection_ms,
        inspect_ms,
        inspection,
        old_baselines,
        current_baselines,
    )
}

fn build_inspected_row(
    label: &'static str,
    schema: SchemaName,
    row_count: usize,
    projected: Vec<u8>,
    access_ms: f64,
    projection_ms: f64,
    inspect_ms: f64,
    inspection: metamorphic_binary_transport_core::runtime::BinaryInspection,
    old_baselines: &[metamorphic_binary_transport_benches::projection::BaselineEntry],
    current_baselines: &[metamorphic_binary_transport_benches::projection::BaselineEntry],
) -> BenchResult<BenchRow> {
    let total_ms = access_ms + projection_ms + inspect_ms;
    let (rows_per_second, mb_per_second) = measured_rates(row_count, projected.len(), total_ms);
    let (semantic_checksum, minimal_projection_checksum) = inspection_checksums(inspection);
    Ok(BenchRow {
        label,
        schema,
        row_count,
        output_bytes: projected.len(),
        total_milliseconds: total_ms,
        rows_per_second,
        mb_per_second,
        source_access_milliseconds: access_ms,
        projection_milliseconds: projection_ms,
        projected_inspect_milliseconds: inspect_ms,
        response_checksum: response_checksum(&projected),
        semantic_checksum,
        minimal_projection_checksum,
        old_crate_comparison: comparison_for(old_baselines, label, row_count, rows_per_second),
        current_owned_row_comparison: comparison_for(
            current_baselines,
            label,
            row_count,
            rows_per_second,
        ),
    })
}

fn elapsed_ms(started: Instant) -> f64 {
    started.elapsed().as_secs_f64() * 1_000.0
}
