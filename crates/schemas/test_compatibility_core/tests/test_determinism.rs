use std::error::Error;

use mbt_schema_test_compatibility::test_compatibility_v1::*;

const MAX_RESPONSE_BYTES: usize = 1_000_000;

type TestResult = std::result::Result<(), Box<dyn Error>>;

#[test]
fn encoding_and_inspection_are_deterministic() -> TestResult {
    let rows = deterministic_rows();
    let first = TestCompatibilityV1::encode(&rows, MAX_RESPONSE_BYTES)?;
    let second = TestCompatibilityV1::encode(&rows, MAX_RESPONSE_BYTES)?;
    assert_eq!(first, second);

    let first_inspection = TestCompatibilityV1::inspect(&first)?;
    let second_inspection = TestCompatibilityV1::inspect(&second)?;
    assert_eq!(first_inspection, second_inspection);
    assert_eq!(first_inspection.row_count, rows.len());

    let view = TestCompatibilityV1::access(&first)?;
    let keys = view
        .rows()
        .map(|row| (row.tenant_ordinal(), row.entity_ordinal(), row.close_ms()))
        .collect::<Vec<_>>();
    assert_eq!(
        keys,
        vec![
            (TENANT_ALPHA, ENTITY_ENTITY_A, 1_000),
            (TENANT_ALPHA, ENTITY_ENTITY_B, 2_000),
            (TENANT_BETA, ENTITY_ENTITY_A, 3_000),
        ]
    );

    Ok(())
}

fn deterministic_rows() -> Vec<TestCompatibilityRowV1> {
    vec![
        row(TENANT_ALPHA, ENTITY_ENTITY_A, 1_000, STATUS_ACTIVE),
        row(TENANT_ALPHA, ENTITY_ENTITY_B, 2_000, STATUS_PAUSED),
        row(TENANT_BETA, ENTITY_ENTITY_A, 3_000, STATUS_CLOSED),
    ]
}

fn row(
    tenant_ordinal: u16,
    entity_ordinal: u16,
    close_ms: i64,
    status_ordinal: u16,
) -> TestCompatibilityRowV1 {
    TestCompatibilityRowV1 {
        schema_version: SCHEMA_VERSION_VALUE,
        tenant_ordinal,
        entity_ordinal,
        close_ms,
        status_ordinal,
        optional_status_ordinal: 0,
        venues_mask: 1 << VENUE_BINANCE_BIT,
        required_i64: close_ms,
        optional_i64: 0,
        required_i32: 1,
        optional_i32: 0,
        required_u32: 2,
        optional_u32: 0,
        required_f64: 3.0,
        optional_f64: 0.0,
        required_f32: 4.0,
        optional_f32: 0.0,
        required_bool: true,
        optional_bool: false,
        required_text: "required".to_string(),
        optional_text: String::new(),
        required_bytes: vec![1, 2, 3],
        optional_bytes: Vec::new(),
        uuid_text: "00000000-0000-0000-0000-000000000001".to_string(),
        jsonb_text: r#"{"ok":true}"#.to_string(),
        timestamptz_text: "2026-01-01T00:00:00Z".to_string(),
        numeric_text: "1.0".to_string(),
        required_i64_array: vec![1, 2],
        nullable_i64_array: Vec::new(),
        required_i32_array: vec![3, 4],
        nullable_i32_array: Vec::new(),
        required_u32_array: vec![5, 6],
        nullable_u32_array: Vec::new(),
        required_f64_array: vec![7.0, 8.0],
        nullable_f64_array: Vec::new(),
        required_f32_array: vec![9.0, 10.0],
        nullable_f32_array: Vec::new(),
        presence_bits: 0,
    }
}
