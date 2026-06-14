use super::*;
use crate::model::{FieldKind, validate_module_name};

#[test]
fn field_kind_mapping_covers_core_scalars_and_arrays() -> Result<()> {
    let scalar = model_for(valid_scalar_proto())?;
    assert!(
        scalar
            .fields
            .iter()
            .any(|field| matches!(field.kind, FieldKind::I64))
    );
    assert!(
        scalar
            .fields
            .iter()
            .any(|field| matches!(field.kind, FieldKind::F64))
    );
    assert!(
        scalar
            .fields
            .iter()
            .any(|field| matches!(field.kind, FieldKind::F32))
    );
    assert!(
        scalar
            .fields
            .iter()
            .any(|field| matches!(field.kind, FieldKind::Bool))
    );
    assert!(
        scalar
            .fields
            .iter()
            .any(|field| matches!(field.kind, FieldKind::I32))
    );
    assert!(
        scalar
            .fields
            .iter()
            .any(|field| matches!(field.kind, FieldKind::U32))
    );

    let arrays = model_for(valid_array_proto())?;
    assert!(
        arrays
            .fields
            .iter()
            .any(|field| matches!(field.kind, FieldKind::I64Array))
    );
    assert!(
        arrays
            .fields
            .iter()
            .any(|field| matches!(field.kind, FieldKind::I32Array))
    );
    assert!(
        arrays
            .fields
            .iter()
            .any(|field| matches!(field.kind, FieldKind::U32Array))
    );
    assert!(
        arrays
            .fields
            .iter()
            .any(|field| matches!(field.kind, FieldKind::F64Array))
    );
    assert!(
        arrays
            .fields
            .iter()
            .any(|field| matches!(field.kind, FieldKind::F32Array))
    );
    Ok(())
}

#[test]
fn module_name_validation_is_strict() -> Result<()> {
    validate_module_name("fixture_v1")?;
    assert!(validate_module_name("FixtureV1").is_err());
    assert!(validate_module_name("_fixture").is_err());
    assert!(validate_module_name("fixture__v1").is_err());
    assert!(validate_module_name("fixture_").is_err());
    Ok(())
}

#[test]
fn hash_changes_only_for_core_included_changes() -> Result<()> {
    let base = model_for(valid_scalar_proto())?;
    let changed_field =
        model_for(&valid_scalar_proto().replace("double c = 4;", "double price = 4;"))?;
    let comment_changed = model_for(
        &valid_scalar_proto().replace("message TestRowV1 {", "// comment\nmessage TestRowV1 {"),
    )?;
    assert_ne!(
        base.normalized_schema_hash,
        changed_field.normalized_schema_hash
    );
    assert_eq!(
        base.normalized_schema_hash,
        comment_changed.normalized_schema_hash
    );
    Ok(())
}
