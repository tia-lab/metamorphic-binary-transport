use super::*;

#[test]
fn generated_core_source_is_deterministic_and_core_only() -> Result<()> {
    let first = run_codegen_to_string(valid_scalar_proto())?;
    let second = run_codegen_to_string(valid_scalar_proto())?;
    assert_eq!(first, second);
    assert!(first.contains("pub struct FixtureV1;"));
    assert!(first.contains("pub struct FixtureV1View<'a>"));
    assert!(first.contains("pub struct FixtureV1Rows<'a>"));
    assert!(first.contains("pub struct ArchivedFixtureV1Row<'a>"));
    assert!(first.contains("pub unsafe fn access_archived_trusted_unchecked"));
    let trusted_body = trusted_access_body(&first)?;
    assert!(trusted_body.contains("trusted_payload_for_schema(bytes, Self::header_spec())?"));
    assert!(
        trusted_body.contains("rkyv::access_unchecked::<ArchivedTestPayloadV1Payload>(payload)")
    );
    assert!(!trusted_body.contains("let header = decode_header(bytes)?"));
    assert!(!trusted_body.contains("validate_archived_payload(archived"));
    assert!(first.contains("fn validate_archived_rows("));
    assert!(first.contains("let row = row_from_archived(archived_row);"));
    assert!(first.contains("validate_row(&row, previous.as_ref())?;"));
    assert!(first.contains(
        "semantic_checksum = checksum_archived_row(semantic_checksum, archived_row, true);"
    ));
    assert!(first.contains("minimal_projection_archived_row"));
    assert!(first.contains("fn update_fixed<const N: usize>(mut checksum: u64, bytes: [u8; N])"));
    assert!(!first.contains("for byte in checksum.to_le_bytes()"));
    assert_forbidden_absent(&first);
    Ok(())
}

fn trusted_access_body(source: &str) -> Result<&str> {
    let start = source
        .find("pub unsafe fn access_archived_trusted_unchecked")
        .ok_or_else(|| {
            CodegenError::InvalidSchema("missing generated trusted access function".to_string())
        })?;
    let tail = &source[start..];
    let end = tail.find("pub fn inspect").ok_or_else(|| {
        CodegenError::InvalidSchema("missing generated inspect function".to_string())
    })?;
    Ok(&tail[..end])
}

#[test]
fn core_surface_ignores_projection_declarations_and_emits_no_projection_symbols() -> Result<()> {
    let source = run_codegen_to_string(valid_alias_and_projection_ignored_proto())?;
    assert!(source.contains("pub struct FixtureV1;"));
    assert!(source.contains("pub struct FixtureV1View<'a>"));
    assert!(!source.contains("SmallProjection"));
    assert!(!source.contains("project_small"));
    assert_forbidden_absent(&source);
    Ok(())
}

#[test]
fn projection_surface_emits_projected_schema_and_source_projection_api() -> Result<()> {
    let source = run_projection_codegen_to_string(valid_alias_and_projection_ignored_proto())?;
    assert!(source.contains("pub struct FixtureV1;"));
    assert!(source.contains("pub struct SmallProjection;"));
    assert!(source.contains("pub fn project_small("));
    assert!(source.contains("pub unsafe fn project_small_trusted_unchecked("));
    assert!(source.contains("SMALL_SCHEMA_HEADER"));
    assert!(source.contains("test.alias.v1.small"));
    assert!(!source.contains("metamorphose"));
    assert!(!source.contains("transpond"));
    assert!(!source.contains("arrow"));
    assert!(!source.contains("parquet"));
    assert!(!source.contains("serde_json"));
    assert!(!source.contains("prost"));
    Ok(())
}

#[test]
fn generated_wide_presence_source_has_presence_words() -> Result<()> {
    let source = run_codegen_to_string(&valid_wide_presence_proto())?;
    assert!(source.contains("pub fn presence_words(&self)"));
    assert!(source.contains("PRESENCE_ALLOWED_MASKS"));
    assert_forbidden_absent(&source);
    Ok(())
}

#[test]
fn generated_fixture_smoke_crate_compiles() -> Result<()> {
    let source = run_codegen_to_string(valid_raw_string_proto())?;
    assert_forbidden_absent(&source);
    smoke_crate(
        &std::env::current_dir()?
            .join("target")
            .join("mbt-codegen-check"),
        &source,
    )
}
