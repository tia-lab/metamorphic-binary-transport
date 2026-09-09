use std::error::Error;
use std::path::PathBuf;
use std::time::Instant;
use std::{eprintln, io, println};

use mbt_benches::projection::{
    BenchResult, BenchRow, MAX_RESPONSE_BYTES, SchemaName, inspection_checksums, measured_rates,
    next_run_path, response_checksum, write_report,
};
use mbt_schema_telemetry::telemetry_v1::{TelemetryV1, TelemetryV1TemperatureOnly};
use mbt_schema_test_compatibility::test_compatibility_v1::{
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
    let mut rows = Vec::new();
    for row_count in mbt_benches::projection::ROW_COUNTS {
        let telemetry_rows = mbt_benches::projection::telemetry_rows(row_count)?;
        let telemetry_source = TelemetryV1::encode(&telemetry_rows, MAX_RESPONSE_BYTES)?;
        rows.extend(measure_telemetry(row_count, &telemetry_source)?);

        let compatibility_rows = mbt_benches::projection::test_compatibility_rows(row_count);
        let compatibility_source =
            TestCompatibilityV1::encode(&compatibility_rows, MAX_RESPONSE_BYTES)?;
        rows.extend(measure_compatibility(row_count, &compatibility_source)?);
    }

    let path = next_run_path(&report_dir)?;
    write_report(&path, &rows)?;
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

fn measure_telemetry(row_count: usize, source: &[u8]) -> BenchResult<Vec<BenchRow>> {
    // Validate once before any trusted projection of these immutable bytes.
    TelemetryV1::access(source)?;
    let mut out = Vec::with_capacity(6);
    out.push(measure_public(
        "telemetry_project_temperature_only_public",
        SchemaName::Telemetry,
        row_count,
        source,
        |bytes| TelemetryV1::project_temperature_only(bytes, MAX_RESPONSE_BYTES),
        |bytes| TelemetryV1TemperatureOnly::inspect(bytes),
    )?);
    out.push(measure_archived(
        "telemetry_project_temperature_only_archived",
        SchemaName::Telemetry,
        row_count,
        source,
        |bytes| TelemetryV1::access(bytes).map(|_| ()),
        |bytes| unsafe {
            TelemetryV1::project_temperature_only_trusted_unchecked(bytes, MAX_RESPONSE_BYTES)
        },
        |bytes| TelemetryV1TemperatureOnly::inspect(bytes),
    )?);
    out.push(measure_inspect(
        "telemetry_project_temperature_only_inspect",
        SchemaName::Telemetry,
        row_count,
        source,
        |bytes| unsafe {
            TelemetryV1::project_temperature_only_trusted_unchecked(bytes, MAX_RESPONSE_BYTES)
        },
        |bytes| TelemetryV1TemperatureOnly::inspect(bytes),
    )?);
    Ok(out)
}

fn measure_compatibility(row_count: usize, source: &[u8]) -> BenchResult<Vec<BenchRow>> {
    // Validate once before any trusted projection of these immutable bytes.
    TestCompatibilityV1::access(source)?;
    let mut out = Vec::with_capacity(6);
    out.push(measure_public(
        "mbt_project_no_optional_public",
        SchemaName::TestCompatibility,
        row_count,
        source,
        |bytes| TestCompatibilityV1::project_no_optional(bytes, MAX_RESPONSE_BYTES),
        |bytes| TestCompatibilityV1NoOptional::inspect(bytes),
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
    )?);
    out.push(measure_public(
        "mbt_project_numeric_only_public",
        SchemaName::TestCompatibility,
        row_count,
        source,
        |bytes| TestCompatibilityV1::project_numeric_only(bytes, MAX_RESPONSE_BYTES),
        |bytes| TestCompatibilityV1NumericOnly::inspect(bytes),
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
) -> BenchResult<BenchRow>
where
    F: FnOnce(&[u8]) -> Result<Vec<u8>, E>,
    I: FnOnce(&[u8]) -> Result<mbt_core::runtime::BinaryInspection, E>,
    E: Error + 'static,
{
    // Public projection includes checked source access inside the generated API.
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
) -> BenchResult<BenchRow>
where
    A: FnOnce(&[u8]) -> Result<(), E>,
    F: FnOnce(&[u8]) -> Result<Vec<u8>, E>,
    I: FnOnce(&[u8]) -> Result<mbt_core::runtime::BinaryInspection, E>,
    E: Error + 'static,
{
    // Archived timing separates source access from trusted projection work.
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
    )
}

fn measure_inspect<F, I, E>(
    label: &'static str,
    schema: SchemaName,
    row_count: usize,
    source: &[u8],
    project: F,
    inspect: I,
) -> BenchResult<BenchRow>
where
    F: FnOnce(&[u8]) -> Result<Vec<u8>, E>,
    I: FnOnce(&[u8]) -> Result<mbt_core::runtime::BinaryInspection, E>,
    E: Error + 'static,
{
    // Inspect timing measures projected-byte validation and checksum evidence.
    let projected = project(source)?;
    let inspect_started = Instant::now();
    let inspection = inspect(&projected)?;
    let inspect_ms = elapsed_ms(inspect_started);
    build_inspected_row(
        label, schema, row_count, projected, 0.0, 0.0, inspect_ms, inspection,
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
) -> BenchResult<BenchRow>
where
    I: FnOnce(&[u8]) -> Result<mbt_core::runtime::BinaryInspection, E>,
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
    inspection: mbt_core::runtime::BinaryInspection,
) -> BenchResult<BenchRow> {
    let total_ms = access_ms + projection_ms + inspect_ms;
    let (rows_per_second, mb_per_second) = measured_rates(row_count, projected.len(), total_ms)?;
    let (semantic_checksum, minimal_projection_checksum) = inspection_checksums(inspection);
    Ok(BenchRow {
        label,
        schema,
        schema_hash: mbt_core::envelope::decode_header(&projected)?.logical_proto_schema_hash,
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
    })
}

fn elapsed_ms(started: Instant) -> f64 {
    started.elapsed().as_secs_f64() * 1_000.0
}
