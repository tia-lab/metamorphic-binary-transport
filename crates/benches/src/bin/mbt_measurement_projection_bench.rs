#[path = "support/measurement_owned.rs"]
mod measurement_owned;
use std::error::Error;
use std::path::PathBuf;
use std::time::Instant;
use std::{eprintln, format, io, println};

use mbt_benches::measurement_projection::{
    BenchResult, BenchRow, MAX_RESPONSE_BYTES, SchemaName, comparison_for, inspection_checksums,
    load_reference_run, measured_rates, next_run_path, response_checksum, write_report,
    write_summary,
};
use mbt_schema_measurement::measurement_v1::{
    MeasurementV1, MeasurementV1ValuesOnly, MeasurementV1WithoutDetails,
};
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
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let report_dir = parse_report_dir(arguments.iter().take(2).cloned())?;
    let reference_baselines = match arguments.as_slice() {
        [_, _] => Vec::new(),
        [_, _, flag, path] if flag == "--reference-baseline" => {
            let entries = load_reference_run(path.as_ref())?;
            verify_reference_baselines(&entries)?;
            entries
        }
        _ => {
            return Err(io::Error::other(
                "expected --report-dir <path> [--reference-baseline <path>]",
            )
            .into());
        }
    };
    let mut owned_rows = Vec::new();

    let mut rows = Vec::new();
    for row_count in mbt_benches::measurement_projection::ROW_COUNTS {
        let measurement_rows = mbt_benches::measurement_projection::measurement_rows(row_count);
        let measurement_source = MeasurementV1::encode(&measurement_rows, MAX_RESPONSE_BYTES)?;
        MeasurementV1::access(&measurement_source)?;
        let owned = measure_owned_measurement(row_count, &measurement_source, &[], &[])?;
        let current_baselines = owned.iter().map(as_baseline).collect::<Vec<_>>();
        owned_rows.extend(owned);
        rows.extend(measure_measurement(
            row_count,
            &measurement_source,
            &reference_baselines,
            &current_baselines,
        )?);

        let compatibility_rows =
            mbt_benches::measurement_projection::test_compatibility_rows(row_count);
        let compatibility_source =
            TestCompatibilityV1::encode(&compatibility_rows, MAX_RESPONSE_BYTES)?;
        TestCompatibilityV1::access(&compatibility_source)?;
        let owned = measure_owned_compatibility(row_count, &compatibility_source, &[])?;
        let current_baselines = owned.iter().map(as_baseline).collect::<Vec<_>>();
        owned_rows.extend(owned);
        rows.extend(measure_compatibility(
            row_count,
            &compatibility_source,
            &current_baselines,
        )?);
    }

    let path = next_run_path(&report_dir)?;
    for row in &rows {
        let baseline = owned_rows
            .iter()
            .find(|b| b.label == row.label && b.row_count == row.row_count)
            .ok_or_else(|| io::Error::other("missing owned reference lane"))?;
        if row.response_checksum != baseline.response_checksum
            || row.output_bytes != baseline.output_bytes
        {
            return Err(io::Error::other("direct/owned output mismatch").into());
        }
    }
    write_report(&path, &rows)?;
    let owned_path = path.with_file_name(format!(
        "owned_{}",
        path.file_name()
            .ok_or_else(|| io::Error::other("missing report filename"))?
            .to_string_lossy()
    ));
    write_report(&owned_path, &owned_rows)?;
    let mut report: serde_json::Value = serde_json::from_slice(&std::fs::read(&path)?)?;
    report["reference_status"] = serde_json::Value::String(
        if reference_baselines.is_empty() {
            "unavailable: external implementation not supplied"
        } else {
            "supplied"
        }
        .to_string(),
    );
    report["dataset_identity"] =
        serde_json::Value::String("synthetic_measurement_v1_structural_fixture".to_string());
    report["measurement_schema_hash"] = MeasurementV1::SCHEMA_HASH.into();
    report["compatibility_schema_hash"] = TestCompatibilityV1::SCHEMA_HASH.into();
    std::fs::write(&path, serde_json::to_vec_pretty(&report)?)?;
    write_summary(&report_dir)?;
    println!("{}", path.display());
    Ok(())
}

fn parse_report_dir(args: impl Iterator<Item = String>) -> BenchResult<PathBuf> {
    let collected = args.collect::<Vec<_>>();
    if collected.len() != 2 || collected[0] != "--report-dir" {
        return Err(io::Error::other(
            "usage: mbt_measurement_projection_bench --report-dir <path>",
        )
        .into());
    }
    Ok(PathBuf::from(&collected[1]))
}

fn verify_reference_baselines(
    baselines: &[mbt_benches::measurement_projection::BaselineEntry],
) -> BenchResult<()> {
    // Missing old baselines make the regression comparison invalid.
    for label in [
        "measurement_project_without_details_public",
        "measurement_project_without_details_archived",
        "measurement_project_without_details_inspect",
        "measurement_project_values_only_public",
        "measurement_project_values_only_archived",
        "measurement_project_values_only_inspect",
    ] {
        for row_count in mbt_benches::measurement_projection::ROW_COUNTS {
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

fn measure_measurement(
    row_count: usize,
    source: &[u8],
    reference_baselines: &[mbt_benches::measurement_projection::BaselineEntry],
    current_baselines: &[mbt_benches::measurement_projection::BaselineEntry],
) -> BenchResult<Vec<BenchRow>> {
    // Measurement projection paths are compared against the historical MBT crate.
    let mut out = Vec::with_capacity(6);
    out.push(measure_public(
        "measurement_project_without_details_public",
        SchemaName::Measurement,
        row_count,
        source,
        |bytes| MeasurementV1::project_without_details(bytes, MAX_RESPONSE_BYTES),
        |bytes| MeasurementV1WithoutDetails::inspect(bytes),
        reference_baselines,
        current_baselines,
    )?);
    out.push(measure_archived(
        "measurement_project_without_details_archived",
        SchemaName::Measurement,
        row_count,
        source,
        |bytes| MeasurementV1::access(bytes).map(|_| ()),
        |bytes| unsafe {
            MeasurementV1::project_without_details_trusted_unchecked(bytes, MAX_RESPONSE_BYTES)
        },
        |bytes| MeasurementV1WithoutDetails::inspect(bytes),
        reference_baselines,
        current_baselines,
    )?);
    out.push(measure_inspect(
        "measurement_project_without_details_inspect",
        SchemaName::Measurement,
        row_count,
        source,
        |bytes| unsafe {
            MeasurementV1::project_without_details_trusted_unchecked(bytes, MAX_RESPONSE_BYTES)
        },
        |bytes| MeasurementV1WithoutDetails::inspect(bytes),
        reference_baselines,
        current_baselines,
    )?);
    out.push(measure_public(
        "measurement_project_values_only_public",
        SchemaName::Measurement,
        row_count,
        source,
        |bytes| MeasurementV1::project_values_only(bytes, MAX_RESPONSE_BYTES),
        |bytes| MeasurementV1ValuesOnly::inspect(bytes),
        reference_baselines,
        current_baselines,
    )?);
    out.push(measure_archived(
        "measurement_project_values_only_archived",
        SchemaName::Measurement,
        row_count,
        source,
        |bytes| MeasurementV1::access(bytes).map(|_| ()),
        |bytes| unsafe {
            MeasurementV1::project_values_only_trusted_unchecked(bytes, MAX_RESPONSE_BYTES)
        },
        |bytes| MeasurementV1ValuesOnly::inspect(bytes),
        reference_baselines,
        current_baselines,
    )?);
    out.push(measure_inspect(
        "measurement_project_values_only_inspect",
        SchemaName::Measurement,
        row_count,
        source,
        |bytes| unsafe {
            MeasurementV1::project_values_only_trusted_unchecked(bytes, MAX_RESPONSE_BYTES)
        },
        |bytes| MeasurementV1ValuesOnly::inspect(bytes),
        reference_baselines,
        current_baselines,
    )?);
    Ok(out)
}

fn measure_compatibility(
    row_count: usize,
    source: &[u8],
    current_baselines: &[mbt_benches::measurement_projection::BaselineEntry],
) -> BenchResult<Vec<BenchRow>> {
    // Compatibility projections prove the generator path beyond the Measurement schema.
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
    reference_baselines: &[mbt_benches::measurement_projection::BaselineEntry],
    current_baselines: &[mbt_benches::measurement_projection::BaselineEntry],
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
        reference_baselines,
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
    reference_baselines: &[mbt_benches::measurement_projection::BaselineEntry],
    current_baselines: &[mbt_benches::measurement_projection::BaselineEntry],
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
        reference_baselines,
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
    reference_baselines: &[mbt_benches::measurement_projection::BaselineEntry],
    current_baselines: &[mbt_benches::measurement_projection::BaselineEntry],
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
        label,
        schema,
        row_count,
        projected,
        0.0,
        0.0,
        inspect_ms,
        inspection,
        reference_baselines,
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
    reference_baselines: &[mbt_benches::measurement_projection::BaselineEntry],
    current_baselines: &[mbt_benches::measurement_projection::BaselineEntry],
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
        reference_baselines,
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
    inspection: mbt_core::runtime::BinaryInspection,
    reference_baselines: &[mbt_benches::measurement_projection::BaselineEntry],
    current_baselines: &[mbt_benches::measurement_projection::BaselineEntry],
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
        reference_comparison: comparison_for(
            reference_baselines,
            label,
            row_count,
            rows_per_second,
        ),
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

fn measure_owned_measurement(
    row_count: usize,
    source: &[u8],
    reference_baselines: &[mbt_benches::measurement_projection::BaselineEntry],
    current_baselines: &[mbt_benches::measurement_projection::BaselineEntry],
) -> BenchResult<Vec<BenchRow>> {
    // Measurement projection paths are compared against the historical MBT crate.
    let mut out = Vec::with_capacity(6);
    out.push(measure_public(
        "measurement_project_without_details_public",
        SchemaName::Measurement,
        row_count,
        source,
        |bytes| measurement_owned::without_details(bytes, MAX_RESPONSE_BYTES),
        |bytes| MeasurementV1WithoutDetails::inspect(bytes),
        reference_baselines,
        current_baselines,
    )?);
    out.push(measure_archived(
        "measurement_project_without_details_archived",
        SchemaName::Measurement,
        row_count,
        source,
        |bytes| MeasurementV1::access(bytes).map(|_| ()),
        |bytes| unsafe { measurement_owned::without_details_trusted(bytes, MAX_RESPONSE_BYTES) },
        |bytes| MeasurementV1WithoutDetails::inspect(bytes),
        reference_baselines,
        current_baselines,
    )?);
    out.push(measure_inspect(
        "measurement_project_without_details_inspect",
        SchemaName::Measurement,
        row_count,
        source,
        |bytes| unsafe { measurement_owned::without_details_trusted(bytes, MAX_RESPONSE_BYTES) },
        |bytes| MeasurementV1WithoutDetails::inspect(bytes),
        reference_baselines,
        current_baselines,
    )?);
    out.push(measure_public(
        "measurement_project_values_only_public",
        SchemaName::Measurement,
        row_count,
        source,
        |bytes| measurement_owned::values_only(bytes, MAX_RESPONSE_BYTES),
        |bytes| MeasurementV1ValuesOnly::inspect(bytes),
        reference_baselines,
        current_baselines,
    )?);
    out.push(measure_archived(
        "measurement_project_values_only_archived",
        SchemaName::Measurement,
        row_count,
        source,
        |bytes| MeasurementV1::access(bytes).map(|_| ()),
        |bytes| unsafe { measurement_owned::values_only_trusted(bytes, MAX_RESPONSE_BYTES) },
        |bytes| MeasurementV1ValuesOnly::inspect(bytes),
        reference_baselines,
        current_baselines,
    )?);
    out.push(measure_inspect(
        "measurement_project_values_only_inspect",
        SchemaName::Measurement,
        row_count,
        source,
        |bytes| unsafe { measurement_owned::values_only_trusted(bytes, MAX_RESPONSE_BYTES) },
        |bytes| MeasurementV1ValuesOnly::inspect(bytes),
        reference_baselines,
        current_baselines,
    )?);
    Ok(out)
}

fn measure_owned_compatibility(
    row_count: usize,
    source: &[u8],
    current_baselines: &[mbt_benches::measurement_projection::BaselineEntry],
) -> BenchResult<Vec<BenchRow>> {
    // Compatibility projections prove the generator path beyond the Measurement schema.
    let empty_old = [];
    let mut out = Vec::with_capacity(6);
    out.push(measure_public(
        "mbt_project_no_optional_public",
        SchemaName::TestCompatibility,
        row_count,
        source,
        |bytes| measurement_owned::no_optional(bytes, MAX_RESPONSE_BYTES),
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
        |bytes| unsafe { measurement_owned::no_optional_trusted(bytes, MAX_RESPONSE_BYTES) },
        |bytes| TestCompatibilityV1NoOptional::inspect(bytes),
        &empty_old,
        current_baselines,
    )?);
    out.push(measure_inspect(
        "mbt_project_no_optional_inspect",
        SchemaName::TestCompatibility,
        row_count,
        source,
        |bytes| unsafe { measurement_owned::no_optional_trusted(bytes, MAX_RESPONSE_BYTES) },
        |bytes| TestCompatibilityV1NoOptional::inspect(bytes),
        &empty_old,
        current_baselines,
    )?);
    out.push(measure_public(
        "mbt_project_numeric_only_public",
        SchemaName::TestCompatibility,
        row_count,
        source,
        |bytes| measurement_owned::numeric_only(bytes, MAX_RESPONSE_BYTES),
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
        |bytes| unsafe { measurement_owned::numeric_only_trusted(bytes, MAX_RESPONSE_BYTES) },
        |bytes| TestCompatibilityV1NumericOnly::inspect(bytes),
        &empty_old,
        current_baselines,
    )?);
    out.push(measure_inspect(
        "mbt_project_numeric_only_inspect",
        SchemaName::TestCompatibility,
        row_count,
        source,
        |bytes| unsafe { measurement_owned::numeric_only_trusted(bytes, MAX_RESPONSE_BYTES) },
        |bytes| TestCompatibilityV1NumericOnly::inspect(bytes),
        &empty_old,
        current_baselines,
    )?);
    Ok(out)
}

fn as_baseline(row: &BenchRow) -> mbt_benches::measurement_projection::BaselineEntry {
    mbt_benches::measurement_projection::BaselineEntry {
        label: row.label.to_string(),
        row_count: row.row_count,
        rows_per_second: row.rows_per_second,
        mb_per_second: row.mb_per_second,
    }
}
