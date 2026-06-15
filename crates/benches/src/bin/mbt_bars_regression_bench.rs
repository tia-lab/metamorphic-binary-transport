use std::io;
use std::path::PathBuf;
use std::time::Instant;

use metamorphic_binary_transport_benches::bars_regression::{
    BarsRegressionRow, BenchResult, Comparison, MAX_RESPONSE_BYTES, OLD_BENCH_RESULTS, ROW_COUNTS,
    bars_rows, comparison_for, measured_rates, metadata_for_run, next_bars_run_path, old_label_for,
    parse_old_bars_baselines, response_checksum, serde_rows_from_bars,
    verify_required_old_baselines, write_report,
};
use metamorphic_binary_transport_schema_bars::bars_v1::BarsV1;

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
    let old_baselines = parse_old_bars_baselines(OLD_BENCH_RESULTS.as_ref())?;
    verify_required_old_baselines(&old_baselines)?;

    let mut rows = Vec::with_capacity(60);
    for row_count in ROW_COUNTS {
        rows.extend(measure_row_count(row_count, &old_baselines)?);
    }

    let path = next_bars_run_path(&report_dir)?;
    let metadata = metadata_for_run(command, &path)?;
    write_report(&path, &metadata, &rows)?;
    println!("{}", path.display());
    Ok(())
}

fn parse_report_dir(args: &[String]) -> BenchResult<PathBuf> {
    if args.len() != 2 || args[0] != "--report-dir" {
        return Err(
            io::Error::other("usage: mbt_bars_regression_bench --report-dir <path>").into(),
        );
    }
    Ok(PathBuf::from(&args[1]))
}

fn measure_row_count(
    row_count: usize,
    old_baselines: &[metamorphic_binary_transport_benches::bars_regression::OldBaselineEntry],
) -> BenchResult<Vec<BarsRegressionRow>> {
    let source_rows = bars_rows(row_count);
    let serde_rows = serde_rows_from_bars(&source_rows);
    let encoded = BarsV1::encode(&source_rows, MAX_RESPONSE_BYTES)?;
    let inspection = BarsV1::inspect(&encoded)?;
    let semantic_checksum = Some(inspection.semantic_checksum);
    let minimal_projection_checksum = Some(inspection.minimal_projection_checksum);
    let serde_baseline = measure_serde_json(row_count, &serde_rows)?;

    let mut out = Vec::with_capacity(10);
    out.push(measure_full_mbt(row_count, &source_rows, old_baselines)?);
    let mut json_checked = measure_output(
        "bars_metamorphose_json_checked",
        row_count,
        semantic_checksum,
        minimal_projection_checksum,
        old_baselines,
        None,
        || Ok(BarsV1::metamorphose_json(&encoded, MAX_RESPONSE_BYTES)?),
    )?;
    json_checked.serde_json_comparison = Some(comparison_against_row(
        json_checked.rows_per_second,
        &serde_baseline,
    ));
    out.push(json_checked);
    out.push(measure_output(
        "bars_metamorphose_protobuf_checked",
        row_count,
        semantic_checksum,
        minimal_projection_checksum,
        old_baselines,
        None,
        || Ok(BarsV1::metamorphose_protobuf(&encoded, MAX_RESPONSE_BYTES)?),
    )?);
    out.push(measure_output(
        "bars_metamorphose_csv_checked",
        row_count,
        semantic_checksum,
        minimal_projection_checksum,
        old_baselines,
        None,
        || Ok(BarsV1::metamorphose_csv(&encoded, MAX_RESPONSE_BYTES)?),
    )?);
    out.push(measure_output(
        "bars_metamorphose_json_trusted",
        row_count,
        semantic_checksum,
        minimal_projection_checksum,
        old_baselines,
        None,
        || {
            Ok(unsafe {
                BarsV1::metamorphose_json_trusted_unchecked(&encoded, MAX_RESPONSE_BYTES)
            }?)
        },
    )?);
    out.push(measure_output(
        "bars_metamorphose_protobuf_trusted",
        row_count,
        semantic_checksum,
        minimal_projection_checksum,
        old_baselines,
        None,
        || {
            Ok(unsafe {
                BarsV1::metamorphose_protobuf_trusted_unchecked(&encoded, MAX_RESPONSE_BYTES)
            }?)
        },
    )?);
    out.push(measure_output(
        "bars_metamorphose_csv_trusted",
        row_count,
        semantic_checksum,
        minimal_projection_checksum,
        old_baselines,
        None,
        || Ok(unsafe { BarsV1::metamorphose_csv_trusted_unchecked(&encoded, MAX_RESPONSE_BYTES) }?),
    )?);
    out.push(measure_output(
        "bars_metamorphose_arrow_ipc_trusted",
        row_count,
        semantic_checksum,
        minimal_projection_checksum,
        old_baselines,
        None,
        || {
            Ok(unsafe {
                BarsV1::metamorphose_arrow_ipc_trusted_unchecked(&encoded, MAX_RESPONSE_BYTES)
            }?)
        },
    )?);
    out.push(measure_output(
        "bars_metamorphose_parquet_trusted",
        row_count,
        semantic_checksum,
        minimal_projection_checksum,
        old_baselines,
        None,
        || {
            Ok(unsafe {
                BarsV1::metamorphose_parquet_trusted_unchecked(&encoded, MAX_RESPONSE_BYTES)
            }?)
        },
    )?);
    out.push(serde_baseline);
    Ok(out)
}

fn measure_full_mbt(
    row_count: usize,
    source_rows: &[metamorphic_binary_transport_schema_bars::bars_v1::MathildeBarRowV1],
    old_baselines: &[metamorphic_binary_transport_benches::bars_regression::OldBaselineEntry],
) -> BenchResult<BarsRegressionRow> {
    let start = Instant::now();
    let bytes = BarsV1::encode(source_rows, MAX_RESPONSE_BYTES)?;
    let inspection = BarsV1::inspect(&bytes)?;
    let milliseconds = start.elapsed().as_secs_f64() * 1_000.0;
    let (rows_per_second, mb_per_second) = measured_rates(row_count, bytes.len(), milliseconds);
    let label = "bars_mbt_full_encode_inspect_checked";
    let old_label = old_label_for(label);
    Ok(BarsRegressionRow {
        label,
        row_count,
        output_bytes: bytes.len(),
        total_milliseconds: milliseconds,
        rows_per_second,
        mb_per_second,
        response_checksum: response_checksum(&bytes),
        semantic_checksum: Some(inspection.semantic_checksum),
        minimal_projection_checksum: Some(inspection.minimal_projection_checksum),
        old_baseline_label: old_label,
        old_crate_comparison: old_label.and_then(|baseline| {
            comparison_for(old_baselines, baseline, row_count, rows_per_second)
        }),
        serde_json_comparison: None,
    })
}

fn measure_output<F>(
    label: &'static str,
    row_count: usize,
    semantic_checksum: Option<u64>,
    minimal_projection_checksum: Option<u64>,
    old_baselines: &[metamorphic_binary_transport_benches::bars_regression::OldBaselineEntry],
    serde_json_comparison: Option<Comparison>,
    run: F,
) -> BenchResult<BarsRegressionRow>
where
    F: FnOnce() -> BenchResult<Vec<u8>>,
{
    let start = Instant::now();
    let bytes = run()?;
    let milliseconds = start.elapsed().as_secs_f64() * 1_000.0;
    let (rows_per_second, mb_per_second) = measured_rates(row_count, bytes.len(), milliseconds);
    let old_label = old_label_for(label);
    Ok(BarsRegressionRow {
        label,
        row_count,
        output_bytes: bytes.len(),
        total_milliseconds: milliseconds,
        rows_per_second,
        mb_per_second,
        response_checksum: response_checksum(&bytes),
        semantic_checksum,
        minimal_projection_checksum,
        old_baseline_label: old_label,
        old_crate_comparison: old_label.and_then(|baseline| {
            comparison_for(old_baselines, baseline, row_count, rows_per_second)
        }),
        serde_json_comparison,
    })
}

fn measure_serde_json(
    row_count: usize,
    rows: &[metamorphic_binary_transport_benches::bars_regression::SerdeBarRow],
) -> BenchResult<BarsRegressionRow> {
    let start = Instant::now();
    let bytes = serde_json::to_vec(rows)?;
    let milliseconds = start.elapsed().as_secs_f64() * 1_000.0;
    let (rows_per_second, mb_per_second) = measured_rates(row_count, bytes.len(), milliseconds);
    Ok(BarsRegressionRow {
        label: "bars_serde_json_baseline",
        row_count,
        output_bytes: bytes.len(),
        total_milliseconds: milliseconds,
        rows_per_second,
        mb_per_second,
        response_checksum: response_checksum(&bytes),
        semantic_checksum: None,
        minimal_projection_checksum: None,
        old_baseline_label: None,
        old_crate_comparison: None,
        serde_json_comparison: None,
    })
}

fn comparison_against_row(
    observed_rows_per_second: f64,
    baseline: &BarsRegressionRow,
) -> Comparison {
    let ratio_rows_per_second = if baseline.rows_per_second <= 0.0 {
        0.0
    } else {
        observed_rows_per_second / baseline.rows_per_second
    };
    Comparison {
        rows_per_second: baseline.rows_per_second,
        mb_per_second: baseline.mb_per_second,
        ratio_rows_per_second,
    }
}
