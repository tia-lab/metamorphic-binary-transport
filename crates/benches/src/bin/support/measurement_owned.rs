//! Benchmark-only reference that materializes projected rows before serialization.
use mbt_core::Result;
use mbt_schema_measurement::measurement_v1::*;
use mbt_schema_test_compatibility::test_compatibility_v1::*;

macro_rules! owned_projection {
    ($checked:ident, $trusted:ident, $copy:ident, $source_marker:ident, $archive:ident,
     $target_marker:ident, $target_row:ident, $row:ident, {$($fields:tt)*}) => {
        pub fn $checked(bytes: &[u8], cap: usize) -> Result<Vec<u8>> {
            $source_marker::access(bytes)?;
            // SAFETY: checked access validated these same immutable source bytes.
            unsafe { $trusted(bytes, cap) }
        }
        /// # Safety
        /// Source bytes must have passed checked access and remained immutable.
        pub unsafe fn $trusted(bytes: &[u8], cap: usize) -> Result<Vec<u8>> {
            // SAFETY: inherited validated immutable input contract.
            let archived = unsafe { $source_marker::access_archived_trusted_unchecked(bytes)? };
            $copy(archived, cap)
        }
        fn $copy(archived: &$archive, cap: usize) -> Result<Vec<u8>> {
            // This allocation/copy is the measured owned-row reference, not a runtime path.
            let rows = archived.rows.iter().map(|$row| $target_row { $($fields)* }).collect();
            $target_marker::encode_owned(rows, cap)
        }
    };
}

owned_projection!(without_details, without_details_trusted, copy_without_details,
MeasurementV1, ArchivedMeasurementResponseV1Payload,
MeasurementV1WithoutDetails, MeasurementRowV1WithoutDetails, r, {
    schema_version: r.schema_version.to_native(),
    device_ordinal: r.device_ordinal.to_native(),
    interval_ordinal: r.interval_ordinal.to_native(),
    started_at_ms: r.started_at_ms.to_native(),
    recorded_at_ms: r.recorded_at_ms.to_native(),
    value_a: r.value_a.to_native(), value_b: r.value_b.to_native(),
    value_c: r.value_c.to_native(), value_d: r.value_d.to_native(),
    value_e: r.value_e.to_native(), value_f: r.value_f.to_native(),
    value_g: r.value_g.to_native(), value_h: r.value_h.to_native(),
    value_i: r.value_i.to_native(), value_j: r.value_j.to_native(),
    count_a: r.count_a.to_native(), count_b: r.count_b.to_native(),
    average_value: r.average_value.to_native(), sample_count: r.sample_count.to_native(),
    age_ms: r.age_ms.to_native(),
    presence_bits: (r.presence_bits.to_native() & 3) | ((r.presence_bits.to_native() >> 21) & 4),
});
owned_projection!(values_only, values_only_trusted, copy_values_only,
MeasurementV1, ArchivedMeasurementResponseV1Payload,
MeasurementV1ValuesOnly, MeasurementRowV1ValuesOnly, r, {
    schema_version: r.schema_version.to_native(),
    device_ordinal: r.device_ordinal.to_native(), interval_ordinal: r.interval_ordinal.to_native(),
    started_at_ms: r.started_at_ms.to_native(), recorded_at_ms: r.recorded_at_ms.to_native(),
    value_a: r.value_a.to_native(), value_b: r.value_b.to_native(),
    value_c: r.value_c.to_native(), value_d: r.value_d.to_native(), value_e: r.value_e.to_native(),
});
owned_projection!(no_optional, no_optional_trusted, copy_no_optional,
TestCompatibilityV1, ArchivedTestCompatibilityResponseV1Payload,
TestCompatibilityV1NoOptional, TestCompatibilityRowV1NoOptional, r, {
    schema_version: r.schema_version.to_native(), tenant_ordinal: r.tenant_ordinal.to_native(),
    entity_ordinal: r.entity_ordinal.to_native(), recorded_at_ms: r.recorded_at_ms.to_native(),
    status_ordinal: r.status_ordinal.to_native(), sites_mask: r.sites_mask.to_native(),
    required_i64: r.required_i64.to_native(), required_i32: r.required_i32.to_native(),
    required_u32: r.required_u32.to_native(), required_f64: r.required_f64.to_native(),
    required_f32: r.required_f32.to_native(), required_bool: r.required_bool,
    required_text: r.required_text.as_str().to_owned(), required_bytes: r.required_bytes.as_slice().to_vec(),
    uuid_text: r.uuid_text.as_str().to_owned(), jsonb_text: r.jsonb_text.as_str().to_owned(),
    timestamptz_text: r.timestamptz_text.as_str().to_owned(), numeric_text: r.numeric_text.as_str().to_owned(),
    required_i64_array: r.required_i64_array.iter().map(|v| v.to_native()).collect(),
    required_i32_array: r.required_i32_array.iter().map(|v| v.to_native()).collect(),
    required_u32_array: r.required_u32_array.iter().map(|v| v.to_native()).collect(),
    required_f64_array: r.required_f64_array.iter().map(|v| v.to_native()).collect(),
    required_f32_array: r.required_f32_array.iter().map(|v| v.to_native()).collect(),
});
owned_projection!(numeric_only, numeric_only_trusted, copy_numeric_only,
TestCompatibilityV1, ArchivedTestCompatibilityResponseV1Payload,
TestCompatibilityV1NumericOnly, TestCompatibilityRowV1NumericOnly, r, {
    schema_version: r.schema_version.to_native(), tenant_ordinal: r.tenant_ordinal.to_native(),
    entity_ordinal: r.entity_ordinal.to_native(), recorded_at_ms: r.recorded_at_ms.to_native(),
    required_i64: r.required_i64.to_native(), optional_i64: r.optional_i64.to_native(),
    required_i32: r.required_i32.to_native(), optional_i32: r.optional_i32.to_native(),
    required_u32: r.required_u32.to_native(), optional_u32: r.optional_u32.to_native(),
    required_f64: r.required_f64.to_native(), optional_f64: r.optional_f64.to_native(),
    required_f32: r.required_f32.to_native(), optional_f32: r.optional_f32.to_native(),
    required_i64_array: r.required_i64_array.iter().map(|v| v.to_native()).collect(),
    nullable_i64_array: r.nullable_i64_array.iter().map(|v| v.to_native()).collect(),
    required_i32_array: r.required_i32_array.iter().map(|v| v.to_native()).collect(),
    nullable_i32_array: r.nullable_i32_array.iter().map(|v| v.to_native()).collect(),
    required_u32_array: r.required_u32_array.iter().map(|v| v.to_native()).collect(),
    nullable_u32_array: r.nullable_u32_array.iter().map(|v| v.to_native()).collect(),
    required_f64_array: r.required_f64_array.iter().map(|v| v.to_native()).collect(),
    nullable_f64_array: r.nullable_f64_array.iter().map(|v| v.to_native()).collect(),
    required_f32_array: r.required_f32_array.iter().map(|v| v.to_native()).collect(),
    nullable_f32_array: r.nullable_f32_array.iter().map(|v| v.to_native()).collect(),
    presence_bits: ((r.presence_bits.to_native() >> 1) & 31) | ((r.presence_bits.to_native() >> 4) & 992),
});

#[cfg(test)]
mod tests {
    use super::*;
    use mbt_benches::measurement_projection::{measurement_rows, test_compatibility_rows};

    #[test]
    fn owned_reference_matches_direct_for_empty_and_populated_inputs() -> Result<()> {
        for count in [0, 1, 3, 100] {
            let cap = 1 << 24;
            let bytes = MeasurementV1::encode(&measurement_rows(count), cap)?;
            assert_eq!(
                without_details(&bytes, cap)?,
                MeasurementV1::project_without_details(&bytes, cap)?
            );
            assert_eq!(
                values_only(&bytes, cap)?,
                MeasurementV1::project_values_only(&bytes, cap)?
            );
            let bytes = TestCompatibilityV1::encode(&test_compatibility_rows(count), cap)?;
            assert_eq!(
                no_optional(&bytes, cap)?,
                TestCompatibilityV1::project_no_optional(&bytes, cap)?
            );
            assert_eq!(
                numeric_only(&bytes, cap)?,
                TestCompatibilityV1::project_numeric_only(&bytes, cap)?
            );
        }
        Ok(())
    }

    #[test]
    fn owned_reference_remaps_optional_presence_and_rejects_bad_input() -> Result<()> {
        let cap = 1 << 20;
        let mut rows = measurement_rows(1);
        rows[0].average_value = 0.0;
        rows[0].sample_count = 0;
        rows[0].age_ms = 0;
        rows[0].presence_bits &= !((1 << 0) | (1 << 1) | (1 << 23));
        let bytes = MeasurementV1::encode(&rows, cap)?;
        assert_eq!(
            without_details(&bytes, cap)?,
            MeasurementV1::project_without_details(&bytes, cap)?
        );
        assert!(without_details(&bytes[..10], cap).is_err());
        assert!(without_details(&bytes, 1).is_err());
        Ok(())
    }
}
