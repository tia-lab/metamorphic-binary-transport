use std::io;
use std::path::PathBuf;
use std::time::Instant;

use mbt_benches::telemetry_regression::{
    BenchResult, MAX_RESPONSE_BYTES, ROW_COUNTS, TelemetryRegressionRow, measured_rates,
    metadata_for_run, next_telemetry_run_path, response_checksum, telemetry_rows, write_report,
};
use mbt_schema_telemetry::telemetry_v1::TelemetryV1;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> BenchResult<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let report_dir = parse_report_dir(&args[1..])?;
    let command = args.join(" ");

    let mut rows = Vec::with_capacity(54);
    for row_count in ROW_COUNTS {
        rows.extend(measure_row_count(row_count)?);
    }

    let path = next_telemetry_run_path(&report_dir)?;
    let metadata = metadata_for_run(command, &path)?;
    write_report(&path, &metadata, &rows)?;
    println!("{}", path.display());
    Ok(())
}

fn parse_report_dir(args: &[String]) -> BenchResult<PathBuf> {
    if args.len() != 2 || args[0] != "--report-dir" {
        return Err(
            io::Error::other("usage: mbt_telemetry_regression_bench --report-dir <path>").into(),
        );
    }
    Ok(PathBuf::from(&args[1]))
}

fn measure_row_count(row_count: usize) -> BenchResult<Vec<TelemetryRegressionRow>> {
    // Each row count is measured across MBT, boundary formats, with checked and trusted access.
    let source_rows = telemetry_rows(row_count)?;
    let encoded = TelemetryV1::encode(&source_rows, MAX_RESPONSE_BYTES)?;
    let inspection = TelemetryV1::inspect(&encoded)?;
    // SAFETY: inspect validates this immutable buffer before every trusted adapter call below.
    let semantic_checksum = Some(inspection.semantic_checksum);
    let minimal_projection_checksum = Some(inspection.minimal_projection_checksum);

    let mut out = Vec::with_capacity(9);
    out.push(measure_full_mbt(row_count, &source_rows)?);
    out.push(measure_output(
        "telemetry_metamorphose_json_checked",
        row_count,
        semantic_checksum,
        minimal_projection_checksum,
        || {
            Ok(TelemetryV1::metamorphose_json(
                &encoded,
                MAX_RESPONSE_BYTES,
            )?)
        },
    )?);
    out.push(measure_output(
        "telemetry_metamorphose_protobuf_checked",
        row_count,
        semantic_checksum,
        minimal_projection_checksum,
        || {
            Ok(TelemetryV1::metamorphose_protobuf(
                &encoded,
                MAX_RESPONSE_BYTES,
            )?)
        },
    )?);
    out.push(measure_output(
        "telemetry_metamorphose_csv_checked",
        row_count,
        semantic_checksum,
        minimal_projection_checksum,
        || Ok(TelemetryV1::metamorphose_csv(&encoded, MAX_RESPONSE_BYTES)?),
    )?);
    out.push(measure_output(
        "telemetry_metamorphose_json_trusted",
        row_count,
        semantic_checksum,
        minimal_projection_checksum,
        || {
            Ok(unsafe {
                TelemetryV1::metamorphose_json_trusted_unchecked(&encoded, MAX_RESPONSE_BYTES)
            }?)
        },
    )?);
    out.push(measure_output(
        "telemetry_metamorphose_protobuf_trusted",
        row_count,
        semantic_checksum,
        minimal_projection_checksum,
        || {
            Ok(unsafe {
                TelemetryV1::metamorphose_protobuf_trusted_unchecked(&encoded, MAX_RESPONSE_BYTES)
            }?)
        },
    )?);
    out.push(measure_output(
        "telemetry_metamorphose_csv_trusted",
        row_count,
        semantic_checksum,
        minimal_projection_checksum,
        || {
            Ok(unsafe {
                TelemetryV1::metamorphose_csv_trusted_unchecked(&encoded, MAX_RESPONSE_BYTES)
            }?)
        },
    )?);
    out.push(measure_output(
        "telemetry_metamorphose_arrow_ipc_trusted",
        row_count,
        semantic_checksum,
        minimal_projection_checksum,
        || {
            Ok(unsafe {
                TelemetryV1::metamorphose_arrow_ipc_trusted_unchecked(&encoded, MAX_RESPONSE_BYTES)
            }?)
        },
    )?);
    out.push(measure_output(
        "telemetry_metamorphose_parquet_trusted",
        row_count,
        semantic_checksum,
        minimal_projection_checksum,
        || {
            Ok(unsafe {
                TelemetryV1::metamorphose_parquet_trusted_unchecked(&encoded, MAX_RESPONSE_BYTES)
            }?)
        },
    )?);
    Ok(out)
}

fn measure_full_mbt(
    row_count: usize,
    source_rows: &[mbt_schema_telemetry::telemetry_v1::TelemetryRowV1],
) -> BenchResult<TelemetryRegressionRow> {
    // Full MBT timing includes encode plus checked inspection by design.
    let start = Instant::now();
    let bytes = TelemetryV1::encode(source_rows, MAX_RESPONSE_BYTES)?;
    let inspection = TelemetryV1::inspect(&bytes)?;
    let milliseconds = start.elapsed().as_secs_f64() * 1_000.0;
    let (rows_per_second, mb_per_second) = measured_rates(row_count, bytes.len(), milliseconds)?;
    let label = "telemetry_mbt_full_encode_inspect_checked";
    Ok(TelemetryRegressionRow {
        label,
        row_count,
        output_bytes: bytes.len(),
        total_milliseconds: milliseconds,
        rows_per_second,
        mb_per_second,
        response_checksum: response_checksum(&bytes),
        semantic_checksum: Some(inspection.semantic_checksum),
        minimal_projection_checksum: Some(inspection.minimal_projection_checksum),
    })
}

fn measure_output<F>(
    label: &'static str,
    row_count: usize,
    semantic_checksum: Option<u64>,
    minimal_projection_checksum: Option<u64>,
    run: F,
) -> BenchResult<TelemetryRegressionRow>
where
    F: FnOnce() -> BenchResult<Vec<u8>>,
{
    // Boundary-format timing starts inside the supplied closure.
    let start = Instant::now();
    let bytes = run()?;
    let milliseconds = start.elapsed().as_secs_f64() * 1_000.0;
    let (rows_per_second, mb_per_second) = measured_rates(row_count, bytes.len(), milliseconds)?;
    Ok(TelemetryRegressionRow {
        label,
        row_count,
        output_bytes: bytes.len(),
        total_milliseconds: milliseconds,
        rows_per_second,
        mb_per_second,
        response_checksum: response_checksum(&bytes),
        semantic_checksum,
        minimal_projection_checksum,
    })
}
