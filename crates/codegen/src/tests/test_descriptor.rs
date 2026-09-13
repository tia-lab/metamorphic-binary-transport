use super::*;
use crate::model::{FieldKind, JsonCsvOutputField, ProtobufOutputField};

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
fn imported_dictionary_source_satisfies_dictionary_field() -> Result<()> {
    let model = imported_dictionary_model(&["sensor_a", "sensor_b"])?;
    assert_eq!(model.dictionaries.len(), 1);
    assert_eq!(model.dictionaries[0].name, "device");
    assert_eq!(
        model.dictionaries[0].values,
        vec!["sensor_a".to_string(), "sensor_b".to_string()]
    );
    assert!(model.fields.iter().any(|field| {
        matches!(
            &field.kind,
            FieldKind::U16Dictionary {
                dictionary,
                optional: false
            } if dictionary == "device"
        )
    }));
    Ok(())
}

#[test]
fn missing_explicit_dictionary_source_fails() -> Result<()> {
    let root = temp_root("missing-dictionary-source")?;
    write_options_proto(&root)?;
    write_proto(
        &root,
        "shared/devices.proto",
        &shared_device_dictionary_proto(&["sensor_a"]),
    )?;
    write_proto(
        &root,
        "test/fixture/v1/test.proto",
        imported_dictionary_root_proto(),
    )?;

    let result = load_schema_model(
        &config(&root, "test/fixture/v1/test.proto", "fixture_v1").schema_request(),
    );
    assert!(matches!(
        result,
        Err(CodegenError::InvalidOption {
            name: "dictionary",
            ..
        })
    ));
    Ok(())
}

#[test]
fn duplicate_dictionary_names_across_sources_fail() -> Result<()> {
    let root = temp_root("duplicate-dictionary-source")?;
    write_options_proto(&root)?;
    write_proto(
        &root,
        "test/fixture/v1/test.proto",
        imported_dictionary_root_proto(),
    )?;
    write_proto(
        &root,
        "shared/devices.proto",
        &shared_device_dictionary_proto(&["sensor_a"]),
    )?;
    write_proto(
        &root,
        "shared/devices_copy.proto",
        &shared_device_dictionary_proto(&["sensor_b"]),
    )?;

    let cfg = config_with_dictionary_sources(
        &root,
        "test/fixture/v1/test.proto",
        "fixture_v1",
        vec!["shared/devices.proto", "shared/devices_copy.proto"],
    );
    let result = load_schema_model(&cfg.schema_request());
    assert!(matches!(
        result,
        Err(CodegenError::InvalidOption {
            name: "dictionary_values.name",
            ..
        })
    ));
    Ok(())
}

#[test]
fn schema_hash_changes_when_explicit_dictionary_values_change() -> Result<()> {
    let first = imported_dictionary_model(&["sensor_a", "sensor_b"])?;
    let second = imported_dictionary_model(&["sensor_a", "sensor_b", "sensor_c"])?;
    assert_ne!(first.normalized_schema_hash, second.normalized_schema_hash);
    Ok(())
}

#[test]
fn source_file_local_dictionary_behavior_still_works() -> Result<()> {
    let model = model_for(valid_scalar_proto())?;
    assert_eq!(model.dictionaries.len(), 1);
    assert_eq!(model.dictionaries[0].name, "entity");
    assert!(model.fields.iter().any(|field| {
        matches!(
            &field.kind,
            FieldKind::U16Dictionary {
                dictionary,
                optional: false
            } if dictionary == "entity"
        )
    }));
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
            .all(|field| field.proto_name != "recorded_at_utc")
    );
    assert!(
        model
            .fields
            .iter()
            .all(|field| field.proto_name != "ingested_at_utc")
    );

    assert_eq!(model.derived_utc_fields.len(), 2);
    assert_eq!(model.derived_utc_fields[0].logical_path, "recorded_at_utc");
    assert_eq!(
        model.derived_utc_fields[0].source_logical_path,
        "recorded_at_ms"
    );
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
    let without_alias = model_for(&valid_alias_and_projection_ignored_proto().replace(
        "  alias: { value: \"sensor_a\" alias: \"primary_sensor\" }\n",
        "",
    ))?;
    let without_projection = model_for(&valid_alias_and_projection_ignored_proto().replace(
        r#"  option (mbt.projection) = {
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
            r#"  option (mbt.projection) = {
    name: "small"
    rust_marker: "SmallProjection"
    include_group: "core"
  };
"#,
            r#"  option (mbt.projection) = {
    name: "small"
    rust_marker: "SmallProjection"
    include_group: "core"
  };
  option (mbt.projection) = {
    name: "small"
    rust_marker: "OtherProjection"
    include_group: "core"
  };
"#,
        ),
        valid_alias_and_projection_ignored_proto().replace(
            r#"  option (mbt.projection) = {
    name: "small"
    rust_marker: "SmallProjection"
    include_group: "core"
  };
"#,
            r#"  option (mbt.projection) = {
    name: "small"
    rust_marker: "SmallProjection"
    include_group: "core"
  };
  option (mbt.projection) = {
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
    let with_extra_selected = model_for(&valid_alias_and_projection_ignored_proto().replace(
        r#"  string ignored_utc = 4 [(mbt.ignored) = true, (mbt.derived_utc_from) = "recorded_at_ms"];"#,
        r#"  double extra = 4 [(mbt.projection_group) = "core"];
  string ignored_utc = 5 [(mbt.ignored) = true, (mbt.derived_utc_from) = "recorded_at_ms"];"#,
    ))?;
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
