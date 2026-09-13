use mbt_schema_measurement::measurement_v1::*;

#[test]
fn measurement_projection_inspect_binds_expected_shape() {
    let inspect = include_str!("expected_measurement_projection_inspect.txt");
    assert!(inspect.contains("transport_name=mbt.example.measurement.v1"));
    assert!(inspect.contains("projection=without_details marker=MeasurementV1WithoutDetails"));
    assert!(inspect.contains("projection=values_only marker=MeasurementV1ValuesOnly"));
    assert!(
        inspect.contains("dictionary=device values=SENSOR_A,SENSOR_B,SENSOR_C,SENSOR_D,SENSOR_E")
    );
    assert!(inspect.contains("dictionary=interval values=60s"));
    assert!(inspect.contains("key_part=1 rust_name=device_ordinal"));
    assert!(inspect.contains("key_part=2 rust_name=interval_ordinal"));
    assert!(inspect.contains("key_part=3 rust_name=recorded_at_ms"));
    assert!(!inspect.contains("started_at_utc"));
    assert!(!inspect.contains("recorded_at_utc"));
}

#[test]
fn measurement_dictionary_constants_are_generated() {
    assert_eq!(DEVICE_SENSOR_B, 1);
    assert_eq!(INTERVAL_60S, 0);
    assert_eq!(SOURCE_SENSOR, 1);
    assert_eq!(PROCESS_DERIVED, 2);
    assert_eq!(RECOMPUTED_REASON_CANONICAL_REPAIR, 1);
    assert_eq!(SITE_SITE_A_BIT, 0);
}
