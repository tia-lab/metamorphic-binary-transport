use std::error::Error;

use mbt_adapter_arrow_ipc::record_batch_from_ipc_stream;
use mbt_schema_bars::bars_v1::*;

const MAX_RESPONSE_BYTES: usize = 1 << 20;

type TestResult = std::result::Result<(), Box<dyn Error>>;

#[test]
fn no_metadata_projection_metamorphoses_all_enabled_formats() -> TestResult {
    let source = BarsV1::encode(&fixture_rows(), MAX_RESPONSE_BYTES)?;
    let projected = BarsV1::project_no_metadata(&source, MAX_RESPONSE_BYTES)?;
    BarsV1NoMetadata::access(&projected)?;

    let json = BarsV1NoMetadata::metamorphose_json(&projected, MAX_RESPONSE_BYTES)?;
    let trusted_json = unsafe {
        BarsV1NoMetadata::metamorphose_json_trusted_unchecked(&projected, MAX_RESPONSE_BYTES)?
    };
    assert_eq!(json, trusted_json);
    let json = std::str::from_utf8(&json)?;
    assert!(json.contains("\"open_ms\":"));
    assert!(json.contains("\"close_ms\":"));
    assert!(!json.contains("metadata."));
    assert!(!json.contains("ingested_at_ms"));

    let csv = BarsV1NoMetadata::metamorphose_csv(&projected, MAX_RESPONSE_BYTES)?;
    let trusted_csv = unsafe {
        BarsV1NoMetadata::metamorphose_csv_trusted_unchecked(&projected, MAX_RESPONSE_BYTES)?
    };
    assert_eq!(csv, trusted_csv);
    let csv = std::str::from_utf8(&csv)?;
    assert!(csv.starts_with("schema_version,pair,tf,open_ms,close_ms"));
    assert!(!csv.contains("metadata."));
    assert!(!csv.contains("ingested_at_ms"));

    let protobuf = BarsV1NoMetadata::metamorphose_protobuf(&projected, MAX_RESPONSE_BYTES)?;
    let trusted_protobuf = unsafe {
        BarsV1NoMetadata::metamorphose_protobuf_trusted_unchecked(&projected, MAX_RESPONSE_BYTES)?
    };
    assert_eq!(protobuf, trusted_protobuf);
    assert!(!protobuf.is_empty());

    let arrow = BarsV1NoMetadata::metamorphose_arrow(&projected, MAX_RESPONSE_BYTES)?;
    let trusted_arrow = unsafe {
        BarsV1NoMetadata::metamorphose_arrow_trusted_unchecked(&projected, MAX_RESPONSE_BYTES)?
    };
    assert_eq!(arrow.num_rows(), fixture_rows().len());
    assert_eq!(trusted_arrow.num_rows(), arrow.num_rows());
    assert!(schema_has_field(&arrow, "open_ms"));
    assert!(schema_has_field(&arrow, "close_ms"));
    assert!(!schema_has_field(&arrow, "metadata.ingested_at_ms"));
    assert!(!schema_has_field(&arrow, "ingested_at_ms"));

    let arrow_ipc = BarsV1NoMetadata::metamorphose_arrow_ipc(&projected, MAX_RESPONSE_BYTES)?;
    let trusted_arrow_ipc = unsafe {
        BarsV1NoMetadata::metamorphose_arrow_ipc_trusted_unchecked(&projected, MAX_RESPONSE_BYTES)?
    };
    assert_eq!(arrow_ipc, trusted_arrow_ipc);
    let decoded_arrow = record_batch_from_ipc_stream(&arrow_ipc)?;
    assert_eq!(decoded_arrow.num_rows(), arrow.num_rows());
    assert!(!schema_has_field(&decoded_arrow, "metadata.ingested_at_ms"));
    assert!(!schema_has_field(&decoded_arrow, "ingested_at_ms"));

    let parquet = BarsV1NoMetadata::metamorphose_parquet(&projected, MAX_RESPONSE_BYTES)?;
    let trusted_parquet = unsafe {
        BarsV1NoMetadata::metamorphose_parquet_trusted_unchecked(&projected, MAX_RESPONSE_BYTES)?
    };
    assert_eq!(parquet, trusted_parquet);
    assert!(parquet.starts_with(b"PAR1"));
    assert!(parquet.ends_with(b"PAR1"));

    let parquet_source = include_str!("../src/bars_v1_parquet.rs");
    let projection_section = parquet_source
        .split("impl ParquetMetamorphoseSchema for BarsV1NoMetadata")
        .nth(1)
        .ok_or("missing no-metadata parquet projection section")?;
    assert!(projection_section.contains("open_ms"));
    assert!(!projection_section.contains("metadata.ingested_at_ms"));
    assert!(!projection_section.contains("ingested_at_ms"));
    Ok(())
}

fn schema_has_field(batch: &mbt_adapter_arrow::ArrowRecordBatch, name: &str) -> bool {
    batch
        .schema()
        .fields()
        .iter()
        .any(|field| field.name() == name)
}

fn fixture_rows() -> Vec<MathildeBarRowV1> {
    vec![fixture_row(0), fixture_row(1)]
}

fn fixture_row(idx: i64) -> MathildeBarRowV1 {
    let close_ms = 1_700_000_000_000 + idx * 60_000;
    let base = 100.0 + idx as f64;
    MathildeBarRowV1 {
        schema_version: SCHEMA_VERSION_VALUE,
        pair_ordinal: PAIR_BTCUSDT,
        tf_ordinal: TIMEFRAME_1M,
        open_ms: close_ms - 60_000,
        close_ms,
        o: base,
        h: base + 1.0,
        l: base - 1.0,
        c: base + 0.5,
        v: 1_000.0 + idx as f64,
        quote_v: 10_000.0 + idx as f64,
        taker_known_v: 500.0,
        taker_signed_v: -10.0,
        taker_known_quote_v: 5_000.0,
        taker_signed_quote_v: -100.0,
        taker_known_n: 10,
        taker_signed_n: -2,
        vw: base + 0.1,
        n: 20,
        source_ordinal: SOURCE_FRONTIER,
        process_ordinal: PROCESS_DERIVED,
        venues_expected_mask: (1_u64 << VENUE_BINANCE_BIT) | (1_u64 << VENUE_BYBIT_BIT),
        venues_with_trades_mask: 1_u64 << VENUE_BINANCE_BIT,
        ingested_at_ms: close_ms + 1,
        target_ingested_at_ms: close_ms + 2,
        built_at_ms: close_ms + 3,
        committed_at_ms: close_ms + 4,
        harmonized_at_ms: close_ms + 5,
        recomputed_at_ms: close_ms + 6,
        recomputed_reason_ordinal: RECOMPUTED_REASON_CANONICAL_REPAIR,
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
        presence_bits: PRESENCE_ALLOWED_MASK,
    }
}
