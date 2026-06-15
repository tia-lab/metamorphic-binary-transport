use std::error::Error;

use metamorphic_binary_transport_core::runtime::encode;
use metamorphic_binary_transport_schema_bars::bars_v1::*;

const MAX_RESPONSE_BYTES: usize = 1_000_000;

type TestResult = std::result::Result<(), Box<dyn Error>>;

#[test]
fn no_metadata_projection_checked_and_trusted_match() -> TestResult {
    let rows = fixture_rows();
    let source = encode::<BarsV1>(&rows, MAX_RESPONSE_BYTES)?;
    BarsV1::access(&source)?;

    let checked = BarsV1::project_no_metadata(&source, MAX_RESPONSE_BYTES)?;
    let trusted =
        unsafe { BarsV1::project_no_metadata_trusted_unchecked(&source, MAX_RESPONSE_BYTES)? };
    assert_eq!(checked, trusted);

    let view = BarsV1NoMetadata::access(&checked)?;
    assert_eq!(view.len(), rows.len());
    let inspection = BarsV1NoMetadata::inspect(&checked)?;
    assert_eq!(inspection.row_count, rows.len());
    assert_eq!(
        inspection.semantic_checksum,
        inspection.minimal_projection_checksum
    );
    Ok(())
}

#[test]
fn ohlcv_only_projection_checked_and_trusted_match() -> TestResult {
    let rows = fixture_rows();
    let source = encode::<BarsV1>(&rows, MAX_RESPONSE_BYTES)?;
    BarsV1::access(&source)?;

    let checked = BarsV1::project_ohlcv_only(&source, MAX_RESPONSE_BYTES)?;
    let trusted =
        unsafe { BarsV1::project_ohlcv_only_trusted_unchecked(&source, MAX_RESPONSE_BYTES)? };
    assert_eq!(checked, trusted);

    let view = BarsV1OhlcvOnly::access(&checked)?;
    assert_eq!(view.len(), rows.len());
    let inspection = BarsV1OhlcvOnly::inspect(&checked)?;
    assert_eq!(inspection.row_count, rows.len());
    assert_eq!(
        inspection.semantic_checksum,
        inspection.minimal_projection_checksum
    );
    Ok(())
}

#[test]
fn direct_projection_helpers_do_not_copy_owned_rows() {
    let source = include_str!("../src/bars_v1.rs");
    for name in [
        "project_no_metadata_archived_direct",
        "project_ohlcv_only_archived_direct",
    ] {
        let bodies = direct_projection_helper_bodies(source, name);
        assert!(!bodies.is_empty(), "missing helper {name}");
        for body in bodies {
            for forbidden in [
                "Vec::with_capacity(archived.",
                ".to_string()",
                ".to_vec()",
                ".collect()",
                "rows.push(",
                "encode_owned(rows",
            ] {
                assert!(
                    !body.contains(forbidden),
                    "direct helper {name} contains {forbidden}"
                );
            }
        }
    }
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

fn direct_projection_helper_bodies<'a>(source: &'a str, name: &str) -> Vec<&'a str> {
    let needle = format!("fn {name}");
    let mut bodies = Vec::new();
    let mut cursor = source;
    while let Some(relative_start) = cursor.find(&needle) {
        let function_start = source.len() - cursor.len() + relative_start;
        let Some(open_relative) = source[function_start..].find('{') else {
            break;
        };
        let open = function_start + open_relative;
        let Some(close) = matching_close_brace(source, open) else {
            break;
        };
        bodies.push(&source[open + 1..close]);
        cursor = &source[close + 1..];
    }
    bodies
}

fn matching_close_brace(source: &str, open: usize) -> Option<usize> {
    let mut depth = 0_u32;
    for (offset, byte) in source.as_bytes()[open..].iter().enumerate() {
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(open + offset);
                }
            }
            _ => {}
        }
    }
    None
}
