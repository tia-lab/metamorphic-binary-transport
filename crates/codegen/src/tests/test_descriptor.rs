use super::*;

#[test]
fn valid_fixture_schemas_load() -> Result<()> {
    for proto in [
        valid_scalar_proto().to_string(),
        valid_raw_string_proto().to_string(),
        valid_array_proto().to_string(),
        valid_wide_presence_proto(),
        valid_alias_and_projection_ignored_proto().to_string(),
    ] {
        let model = model_for(&proto)?;
        assert_eq!(model.row_field_name, "rows");
        assert!(!model.fields.is_empty());
    }
    Ok(())
}

#[test]
fn invalid_fixture_schemas_fail_before_emission() -> Result<()> {
    for proto in [
        invalid_unannotated_string_proto(),
        invalid_missing_presence_proto(),
        invalid_duplicate_presence_proto(),
        invalid_gapped_presence_proto(),
        invalid_duplicate_key_order_proto(),
        invalid_gapped_key_order_proto(),
        invalid_nullable_bitmask_proto(),
        invalid_repeated_string_proto(),
        invalid_repeated_bytes_proto(),
        invalid_repeated_bool_proto(),
        invalid_duplicate_rust_field_proto(),
    ] {
        assert!(model_for(proto).is_err());
    }
    Ok(())
}

#[test]
fn alias_and_projection_do_not_affect_core_hash_or_fields() -> Result<()> {
    let with_alias = model_for(valid_alias_and_projection_ignored_proto())?;
    let without_alias = model_for(
        &valid_alias_and_projection_ignored_proto()
            .replace("  alias: { value: \"btc\" alias: \"xbt\" }\n", ""),
    )?;
    assert_eq!(
        with_alias.normalized_schema_hash,
        without_alias.normalized_schema_hash
    );
    assert!(
        with_alias
            .fields
            .iter()
            .all(|field| field.proto_name != "ignored_utc")
    );
    Ok(())
}
