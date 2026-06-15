use super::*;
use crate::model::{JsonCsvOutputField, ProtobufOutputField};

#[test]
fn valid_fixture_schemas_load() -> Result<()> {
    for proto in [
        valid_scalar_proto().to_string(),
        valid_raw_string_proto().to_string(),
        valid_array_proto().to_string(),
        valid_wide_presence_proto(),
        valid_alias_and_projection_ignored_proto().to_string(),
        valid_nested_derived_utc_proto().to_string(),
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
        invalid_derived_utc_without_ignored_proto(),
        invalid_derived_utc_unknown_source_proto(),
        invalid_derived_utc_non_i64_source_proto(),
        invalid_derived_utc_non_string_field_proto(),
        invalid_repeated_derived_utc_proto(),
        invalid_duplicate_protobuf_output_tag_proto(),
        invalid_duplicate_protobuf_helper_stem_proto(),
    ] {
        assert!(model_for(proto).is_err());
    }
    Ok(())
}

#[test]
fn derived_utc_fields_are_row_format_only() -> Result<()> {
    let model = model_for(valid_nested_derived_utc_proto())?;
    assert_eq!(model.fields.len(), 4);
    assert!(
        model
            .fields
            .iter()
            .all(|field| field.proto_name != "close_utc")
    );
    assert!(
        model
            .fields
            .iter()
            .all(|field| field.proto_name != "ingested_at_utc")
    );

    assert_eq!(model.derived_utc_fields.len(), 2);
    assert_eq!(model.derived_utc_fields[0].logical_path, "close_utc");
    assert_eq!(model.derived_utc_fields[0].source_logical_path, "close_ms");
    assert_eq!(
        model.derived_utc_fields[1].logical_path,
        "metadata.ingested_at_utc"
    );
    assert_eq!(
        model.derived_utc_fields[1].source_logical_path,
        "metadata.ingested_at_ms"
    );
    assert_eq!(model.derived_utc_fields[1].source_presence_bit, Some(0));

    assert!(matches!(
        model.json_csv_output_fields[3],
        JsonCsvOutputField::DerivedUtc { derived_index: 0 }
    ));
    assert!(matches!(
        model.json_csv_output_fields[5],
        JsonCsvOutputField::DerivedUtc { derived_index: 1 }
    ));

    assert_eq!(model.protobuf_messages.len(), 2);
    assert_eq!(model.protobuf_messages[0].rust_helper_stem, "row");
    assert_eq!(model.protobuf_messages[1].rust_helper_stem, "metadata");
    assert!(model.protobuf_messages[0].fields.iter().any(|field| {
        matches!(
            field,
            ProtobufOutputField::Message { message_index } if *message_index == 1
        )
    }));
    assert!(model.protobuf_messages[1].fields.iter().any(|field| {
        matches!(
            field,
            ProtobufOutputField::DerivedUtc { derived_index } if *derived_index == 1
        )
    }));
    Ok(())
}

#[test]
fn alias_and_projection_do_not_affect_core_hash_or_fields() -> Result<()> {
    let with_alias = model_for(valid_alias_and_projection_ignored_proto())?;
    let without_alias = model_for(
        &valid_alias_and_projection_ignored_proto()
            .replace("  alias: { value: \"btc\" alias: \"xbt\" }\n", ""),
    )?;
    let without_projection = model_for(&valid_alias_and_projection_ignored_proto().replace(
        r#"  option (mathilde.projection) = {
    name: "small"
    rust_marker: "SmallProjection"
    include_group: "core"
  };
"#,
        "",
    ))?;
    assert_eq!(
        with_alias.normalized_schema_hash,
        without_alias.normalized_schema_hash
    );
    assert_eq!(
        with_alias.normalized_schema_hash,
        without_projection.normalized_schema_hash
    );
    assert!(
        with_alias
            .fields
            .iter()
            .all(|field| field.proto_name != "ignored_utc")
    );
    Ok(())
}

#[test]
fn projection_definitions_are_parsed_from_root_options() -> Result<()> {
    let model = model_for(valid_alias_and_projection_ignored_proto())?;
    assert_eq!(model.projections.len(), 1);
    let projection = &model.projections[0];
    assert_eq!(projection.definition.name, "small");
    assert_eq!(projection.definition.rust_marker, "SmallProjection");
    assert_eq!(projection.marker_type, "SmallProjection");
    assert_eq!(projection.transport_name, "test.alias.v1.small");
    assert!(
        projection
            .fields
            .iter()
            .any(|field| field.proto_name == "entity")
    );
    assert!(
        projection
            .fields
            .iter()
            .all(|field| field.proto_name != "ignored_utc")
    );
    Ok(())
}

#[test]
fn invalid_projection_definitions_fail_before_emission() {
    for proto in [
        valid_alias_and_projection_ignored_proto().replace("name: \"small\"", "name: \"Small\""),
        valid_alias_and_projection_ignored_proto().replace(
            "rust_marker: \"SmallProjection\"",
            "rust_marker: \"small_projection\"",
        ),
        valid_alias_and_projection_ignored_proto()
            .replace("include_group: \"core\"", "include_group: \"unknown\""),
        valid_alias_and_projection_ignored_proto()
            .replace("include_group: \"core\"", "include_field: \"unknown\""),
        valid_alias_and_projection_ignored_proto().replace(
            r#"  option (mathilde.projection) = {
    name: "small"
    rust_marker: "SmallProjection"
    include_group: "core"
  };
"#,
            r#"  option (mathilde.projection) = {
    name: "small"
    rust_marker: "SmallProjection"
    include_group: "core"
  };
  option (mathilde.projection) = {
    name: "small"
    rust_marker: "OtherProjection"
    include_group: "core"
  };
"#,
        ),
        valid_alias_and_projection_ignored_proto().replace(
            r#"  option (mathilde.projection) = {
    name: "small"
    rust_marker: "SmallProjection"
    include_group: "core"
  };
"#,
            r#"  option (mathilde.projection) = {
    name: "small"
    rust_marker: "SmallProjection"
    include_group: "core"
  };
  option (mathilde.projection) = {
    name: "other"
    rust_marker: "SmallProjection"
    include_group: "core"
  };
"#,
        ),
    ] {
        assert!(model_for(&proto).is_err());
    }
}

#[test]
fn projection_hash_changes_with_projection_shape() -> Result<()> {
    let base = model_for(valid_alias_and_projection_ignored_proto())?;
    let with_extra_selected = model_for(
        &valid_alias_and_projection_ignored_proto().replace(
            r#"  string ignored_utc = 4 [(mathilde.ignored) = true, (mathilde.derived_utc_from) = "close_ms"];"#,
            r#"  double extra = 4 [(mathilde.projection_group) = "core"];
  string ignored_utc = 5 [(mathilde.ignored) = true, (mathilde.derived_utc_from) = "close_ms"];"#,
        ),
    )?;
    let renamed = model_for(
        &valid_alias_and_projection_ignored_proto().replace("name: \"small\"", "name: \"smaller\""),
    )?;
    assert_ne!(
        base.projections[0].normalized_schema_hash,
        with_extra_selected.projections[0].normalized_schema_hash
    );
    assert_ne!(
        base.projections[0].normalized_schema_hash,
        renamed.projections[0].normalized_schema_hash
    );
    Ok(())
}
