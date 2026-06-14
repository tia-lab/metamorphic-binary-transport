use std::error::Error;
use std::io;

use metamorphic_binary_transport_core::runtime::{access, encode, inspect};
use metamorphic_binary_transport_schema_test_compatibility::test_compatibility_v1::*;

const MAX_RESPONSE_BYTES: usize = 1_000_000;

type TestResult = std::result::Result<(), Box<dyn Error>>;

#[test]
fn roundtrip_covers_all_fields() -> TestResult {
    let rows = fixture_rows();
    let bytes = encode::<TestCompatibilityV1>(&rows, MAX_RESPONSE_BYTES)?;
    let view = access::<TestCompatibilityV1>(&bytes)?;
    assert_eq!(view.len(), rows.len());
    assert!(!view.is_empty());

    let archived_rows = view.rows().collect::<Vec<_>>();
    assert_eq!(archived_rows.len(), rows.len());
    for (idx, expected) in rows.iter().enumerate() {
        let actual = archived_rows
            .get(idx)
            .ok_or_else(|| io::Error::other("missing archived row"))?;
        assert_row(actual, expected)?;
    }

    let trait_inspection = inspect::<TestCompatibilityV1>(&bytes)?;
    let direct_inspection = TestCompatibilityV1::inspect(&bytes)?;
    assert_eq!(trait_inspection, direct_inspection);
    assert_eq!(trait_inspection.row_count, rows.len());
    assert_eq!(
        trait_inspection.semantic_checksum,
        trait_inspection.minimal_projection_checksum
    );

    Ok(())
}

fn fixture_rows() -> Vec<TestCompatibilityRowV1> {
    vec![full_row(), absent_row(), present_empty_array_row()]
}

fn full_row() -> TestCompatibilityRowV1 {
    TestCompatibilityRowV1 {
        schema_version: SCHEMA_VERSION_VALUE,
        tenant_ordinal: TENANT_ALPHA,
        entity_ordinal: ENTITY_ENTITY_A,
        close_ms: 1_000,
        status_ordinal: STATUS_ACTIVE,
        optional_status_ordinal: STATUS_PAUSED,
        venues_mask: (1 << VENUE_BINANCE_BIT) | (1 << VENUE_BYBIT_BIT),
        required_i64: -10,
        optional_i64: 20,
        required_i32: -30,
        optional_i32: 40,
        required_u32: 50,
        optional_u32: 60,
        required_f64: 70.5,
        optional_f64: 80.25,
        required_f32: 90.5,
        optional_f32: 100.25,
        required_bool: true,
        optional_bool: true,
        required_text: "required-alpha".to_string(),
        optional_text: "optional-alpha".to_string(),
        required_bytes: vec![1, 2, 3],
        optional_bytes: vec![4, 5, 6],
        uuid_text: "00000000-0000-0000-0000-000000000001".to_string(),
        jsonb_text: r#"{"kind":"alpha"}"#.to_string(),
        timestamptz_text: "2026-01-01T00:00:00Z".to_string(),
        numeric_text: "123.456".to_string(),
        required_i64_array: vec![-1, -2],
        nullable_i64_array: vec![1, 2],
        required_i32_array: vec![-3, -4],
        nullable_i32_array: vec![3, 4],
        required_u32_array: vec![5, 6],
        nullable_u32_array: vec![7, 8],
        required_f64_array: vec![1.5, 2.5],
        nullable_f64_array: vec![3.5, 4.5],
        required_f32_array: vec![5.5, 6.5],
        nullable_f32_array: vec![7.5, 8.5],
        presence_bits: PRESENCE_ALLOWED_MASK,
    }
}

fn absent_row() -> TestCompatibilityRowV1 {
    TestCompatibilityRowV1 {
        schema_version: SCHEMA_VERSION_VALUE,
        tenant_ordinal: TENANT_ALPHA,
        entity_ordinal: ENTITY_ENTITY_B,
        close_ms: 2_000,
        status_ordinal: STATUS_CLOSED,
        optional_status_ordinal: 0,
        venues_mask: 1 << VENUE_OKX_BIT,
        required_i64: -110,
        optional_i64: 0,
        required_i32: -130,
        optional_i32: 0,
        required_u32: 150,
        optional_u32: 0,
        required_f64: 170.5,
        optional_f64: 0.0,
        required_f32: 190.5,
        optional_f32: 0.0,
        required_bool: false,
        optional_bool: false,
        required_text: "required-beta".to_string(),
        optional_text: String::new(),
        required_bytes: vec![7, 8, 9],
        optional_bytes: Vec::new(),
        uuid_text: "00000000-0000-0000-0000-000000000002".to_string(),
        jsonb_text: r#"{"kind":"beta"}"#.to_string(),
        timestamptz_text: "2026-01-01T00:01:00Z".to_string(),
        numeric_text: "456.789".to_string(),
        required_i64_array: vec![-11, -12],
        nullable_i64_array: Vec::new(),
        required_i32_array: vec![-13, -14],
        nullable_i32_array: Vec::new(),
        required_u32_array: vec![15, 16],
        nullable_u32_array: Vec::new(),
        required_f64_array: vec![11.5, 12.5],
        nullable_f64_array: Vec::new(),
        required_f32_array: vec![13.5, 14.5],
        nullable_f32_array: Vec::new(),
        presence_bits: 0,
    }
}

fn present_empty_array_row() -> TestCompatibilityRowV1 {
    TestCompatibilityRowV1 {
        schema_version: SCHEMA_VERSION_VALUE,
        tenant_ordinal: TENANT_BETA,
        entity_ordinal: ENTITY_ENTITY_A,
        close_ms: 3_000,
        status_ordinal: STATUS_PAUSED,
        optional_status_ordinal: 0,
        venues_mask: 0,
        required_i64: -210,
        optional_i64: 0,
        required_i32: -230,
        optional_i32: 0,
        required_u32: 250,
        optional_u32: 0,
        required_f64: 270.5,
        optional_f64: 0.0,
        required_f32: 290.5,
        optional_f32: 0.0,
        required_bool: true,
        optional_bool: false,
        required_text: "required-gamma".to_string(),
        optional_text: String::new(),
        required_bytes: vec![10, 11, 12],
        optional_bytes: Vec::new(),
        uuid_text: "00000000-0000-0000-0000-000000000003".to_string(),
        jsonb_text: r#"{"kind":"gamma"}"#.to_string(),
        timestamptz_text: "2026-01-01T00:02:00Z".to_string(),
        numeric_text: "789.012".to_string(),
        required_i64_array: Vec::new(),
        nullable_i64_array: Vec::new(),
        required_i32_array: Vec::new(),
        nullable_i32_array: Vec::new(),
        required_u32_array: Vec::new(),
        nullable_u32_array: Vec::new(),
        required_f64_array: Vec::new(),
        nullable_f64_array: Vec::new(),
        required_f32_array: Vec::new(),
        nullable_f32_array: Vec::new(),
        presence_bits: PRESENCE_NULLABLE_I64_ARRAY
            | PRESENCE_NULLABLE_I32_ARRAY
            | PRESENCE_NULLABLE_U32_ARRAY
            | PRESENCE_NULLABLE_F64_ARRAY
            | PRESENCE_NULLABLE_F32_ARRAY,
    }
}

fn assert_row(
    actual: &ArchivedTestCompatibilityV1Row<'_>,
    expected: &TestCompatibilityRowV1,
) -> TestResult {
    assert_eq!(actual.schema_version(), expected.schema_version);
    assert_eq!(actual.tenant_ordinal(), expected.tenant_ordinal);
    assert_eq!(actual.tenant()?, tenant_symbol(expected.tenant_ordinal)?);
    assert_eq!(actual.entity_ordinal(), expected.entity_ordinal);
    assert_eq!(actual.entity()?, entity_symbol(expected.entity_ordinal)?);
    assert_eq!(actual.close_ms(), expected.close_ms);
    assert_eq!(actual.status_ordinal(), expected.status_ordinal);
    assert_eq!(actual.status()?, status_symbol(expected.status_ordinal)?);
    assert_eq!(actual.venues_mask(), expected.venues_mask);
    assert_eq!(actual.required_i64(), expected.required_i64);
    assert_eq!(actual.optional_i64(), expected.optional_i64);
    assert_eq!(actual.required_i32(), expected.required_i32);
    assert_eq!(actual.optional_i32(), expected.optional_i32);
    assert_eq!(actual.required_u32(), expected.required_u32);
    assert_eq!(actual.optional_u32(), expected.optional_u32);
    assert_eq!(actual.required_f64(), expected.required_f64);
    assert_eq!(actual.optional_f64(), expected.optional_f64);
    assert_eq!(actual.required_f32(), expected.required_f32);
    assert_eq!(actual.optional_f32(), expected.optional_f32);
    assert_eq!(actual.required_bool(), expected.required_bool);
    assert_eq!(actual.optional_bool(), expected.optional_bool);
    assert_eq!(actual.required_text(), expected.required_text.as_str());
    assert_eq!(actual.optional_text(), expected.optional_text.as_str());
    assert_eq!(actual.required_bytes(), expected.required_bytes.as_slice());
    assert_eq!(actual.optional_bytes(), expected.optional_bytes.as_slice());
    assert_eq!(actual.uuid_text(), expected.uuid_text.as_str());
    assert_eq!(actual.jsonb_text(), expected.jsonb_text.as_str());
    assert_eq!(
        actual.timestamptz_text(),
        expected.timestamptz_text.as_str()
    );
    assert_eq!(actual.numeric_text(), expected.numeric_text.as_str());
    assert_eq!(
        actual.required_i64_array().collect::<Vec<_>>(),
        expected.required_i64_array
    );
    assert_eq!(
        actual.nullable_i64_array().collect::<Vec<_>>(),
        expected.nullable_i64_array
    );
    assert_eq!(
        actual.required_i32_array().collect::<Vec<_>>(),
        expected.required_i32_array
    );
    assert_eq!(
        actual.nullable_i32_array().collect::<Vec<_>>(),
        expected.nullable_i32_array
    );
    assert_eq!(
        actual.required_u32_array().collect::<Vec<_>>(),
        expected.required_u32_array
    );
    assert_eq!(
        actual.nullable_u32_array().collect::<Vec<_>>(),
        expected.nullable_u32_array
    );
    assert_eq!(
        actual.required_f64_array().collect::<Vec<_>>(),
        expected.required_f64_array
    );
    assert_eq!(
        actual.nullable_f64_array().collect::<Vec<_>>(),
        expected.nullable_f64_array
    );
    assert_eq!(
        actual.required_f32_array().collect::<Vec<_>>(),
        expected.required_f32_array
    );
    assert_eq!(
        actual.nullable_f32_array().collect::<Vec<_>>(),
        expected.nullable_f32_array
    );
    assert_eq!(actual.presence_bits(), expected.presence_bits);
    assert_optional_flags(actual, expected);
    Ok(())
}

fn assert_optional_flags(
    actual: &ArchivedTestCompatibilityV1Row<'_>,
    expected: &TestCompatibilityRowV1,
) {
    assert_eq!(
        actual.has_optional_status(),
        expected.presence_bits & PRESENCE_OPTIONAL_STATUS_ORDINAL != 0
    );
    assert_eq!(
        actual.has_optional_i64(),
        expected.presence_bits & PRESENCE_OPTIONAL_I64 != 0
    );
    assert_eq!(
        actual.has_optional_i32(),
        expected.presence_bits & PRESENCE_OPTIONAL_I32 != 0
    );
    assert_eq!(
        actual.has_optional_u32(),
        expected.presence_bits & PRESENCE_OPTIONAL_U32 != 0
    );
    assert_eq!(
        actual.has_optional_f64(),
        expected.presence_bits & PRESENCE_OPTIONAL_F64 != 0
    );
    assert_eq!(
        actual.has_optional_f32(),
        expected.presence_bits & PRESENCE_OPTIONAL_F32 != 0
    );
    assert_eq!(
        actual.has_optional_bool(),
        expected.presence_bits & PRESENCE_OPTIONAL_BOOL != 0
    );
    assert_eq!(
        actual.has_optional_text(),
        expected.presence_bits & PRESENCE_OPTIONAL_TEXT != 0
    );
    assert_eq!(
        actual.has_optional_bytes(),
        expected.presence_bits & PRESENCE_OPTIONAL_BYTES != 0
    );
    assert_eq!(
        actual.has_nullable_i64_array(),
        expected.presence_bits & PRESENCE_NULLABLE_I64_ARRAY != 0
    );
    assert_eq!(
        actual.has_nullable_i32_array(),
        expected.presence_bits & PRESENCE_NULLABLE_I32_ARRAY != 0
    );
    assert_eq!(
        actual.has_nullable_u32_array(),
        expected.presence_bits & PRESENCE_NULLABLE_U32_ARRAY != 0
    );
    assert_eq!(
        actual.has_nullable_f64_array(),
        expected.presence_bits & PRESENCE_NULLABLE_F64_ARRAY != 0
    );
    assert_eq!(
        actual.has_nullable_f32_array(),
        expected.presence_bits & PRESENCE_NULLABLE_F32_ARRAY != 0
    );
}
