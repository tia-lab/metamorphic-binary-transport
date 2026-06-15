use metamorphic_binary_transport_schema_bars::bars_v1::*;

const MAX_RESPONSE_BYTES: usize = 1 << 20;

#[test]
fn bars_json_csv_emit_derived_utc_outputs() -> Result<(), Box<dyn std::error::Error>> {
    let rows = fixture_rows();
    let bytes = BarsV1::encode(&rows, MAX_RESPONSE_BYTES)?;

    let json = BarsV1::metamorphose_json(&bytes, MAX_RESPONSE_BYTES)?;
    let json = std::str::from_utf8(&json)?;
    assert!(json.contains("\"open_utc\":\""));
    assert!(json.contains("\"close_utc\":\""));
    assert!(json.contains("\"metadata.ingested_at_utc\":\""));

    let csv = BarsV1::metamorphose_csv(&bytes, MAX_RESPONSE_BYTES)?;
    let csv = std::str::from_utf8(&csv)?;
    assert!(csv.starts_with("schema_version,pair,tf,open_ms,close_ms,open_utc,close_utc"));
    assert!(csv.contains("metadata.ingested_at_utc"));
    assert!(csv.contains("metadata.target_ingested_at_utc"));
    Ok(())
}

#[test]
fn bars_protobuf_generated_source_keeps_metadata_nested() {
    let source = include_str!("../src/bars_v1_protobuf.rs");
    assert!(source.contains("writer.message_prefix(22, message_len)?;"));
    assert!(source.contains("write_protobuf_metadata(row, writer)?;"));
    assert!(source.contains("writer.utc(6, row.ingested_at_ms.to_native())?;"));
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
