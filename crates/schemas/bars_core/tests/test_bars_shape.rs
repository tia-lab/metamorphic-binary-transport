use metamorphic_binary_transport_schema_bars::bars_v1::*;

#[test]
fn bars_projection_inspect_binds_expected_shape() {
    let inspect = include_str!("expected_bars_projection_inspect.txt");
    assert!(inspect.contains("transport_name=mathilde.bar.v1"));
    assert!(inspect.contains("projection=no_metadata marker=BarsV1NoMetadata"));
    assert!(inspect.contains("projection=ohlcv_only marker=BarsV1OhlcvOnly"));
    assert!(inspect.contains("dictionary=pair values=ADAUSDT,BTCUSDT,ETHUSDT,LTCUSDT,XRPUSDT"));
    assert!(inspect.contains("dictionary=timeframe values=1m"));
    assert!(inspect.contains("key_part=1 rust_name=pair_ordinal"));
    assert!(inspect.contains("key_part=2 rust_name=tf_ordinal"));
    assert!(inspect.contains("key_part=3 rust_name=close_ms"));
    assert!(!inspect.contains("open_utc"));
    assert!(!inspect.contains("close_utc"));
}

#[test]
fn bars_dictionary_constants_are_generated() {
    assert_eq!(PAIR_BTCUSDT, 1);
    assert_eq!(TIMEFRAME_1M, 0);
    assert_eq!(SOURCE_FRONTIER, 1);
    assert_eq!(PROCESS_DERIVED, 2);
    assert_eq!(RECOMPUTED_REASON_CANONICAL_REPAIR, 1);
    assert_eq!(VENUE_BINANCE_BIT, 0);
}
