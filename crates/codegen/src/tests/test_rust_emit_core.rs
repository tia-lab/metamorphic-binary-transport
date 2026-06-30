use super::*;

#[test]
fn generated_core_source_is_deterministic_and_core_only() -> Result<()> {
    let first = run_codegen_to_string(valid_scalar_proto())?;
    let second = run_codegen_to_string(valid_scalar_proto())?;
    assert_eq!(first, second);
    assert!(first.contains("pub struct FixtureV1;"));
    assert!(first.contains("pub struct FixtureV1View<'a>"));
    assert!(first.contains("pub struct TestRowV1EncodeRow"));
    assert!(first.contains("pub fn encode_views_into("));
    assert!(first.contains("type EncodeRow<'a> = TestRowV1EncodeRow;"));
    assert!(first.contains("to_bytes_in_with_alloc"));
    assert!(first.contains("Buffer::from(payload_out)"));
    assert!(first.contains("SubAllocator::new"));
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

#[test]
fn generated_borrowed_encode_matches_owned_bytes() -> Result<()> {
    let source = run_codegen_to_string(valid_raw_string_proto())?;
    assert_forbidden_absent(&source);
    let root = std::env::current_dir()?
        .join("target")
        .join("mbt-codegen-borrowed-encode-check");
    let smoke = crate::emit::smoke_crate(&root, &source)?;
    let tests = smoke.join("tests");
    std::fs::create_dir_all(&tests)?;
    std::fs::write(
        tests.join("borrowed_encode.rs"),
        r#"
use mbt_codegen_smoke::generated_fixture::{FixtureV1, TestRowV1, TestRowV1EncodeRow};

#[test]
fn borrowed_encode_matches_owned_bytes() -> mbt_core::error::Result<()> {
    let owned_rows = vec![TestRowV1 {
        schema_version: 1,
        close_ms: 1,
        text: "alpha".to_string(),
        raw: vec![1, 2, 3],
        presence_bits: 3,
    }];
    let borrowed_rows = [TestRowV1EncodeRow {
        schema_version: 1,
        close_ms: 1,
        text: "alpha",
        raw: &[1, 2, 3],
        presence_bits: 3,
    }];
    let owned = FixtureV1::encode_owned(owned_rows, 4096)?;
    let mut out = [0_u8; 4096];
    let written = FixtureV1::encode_views_into(&borrowed_rows, &mut out, 4096)?;
    assert_eq!(&out[..written], owned.as_slice());
    FixtureV1::access(&out[..written])?;
    Ok(())
}

#[test]
fn borrowed_encode_enforces_output_cap() -> mbt_core::error::Result<()> {
    let borrowed_rows = [TestRowV1EncodeRow {
        schema_version: 1,
        close_ms: 1,
        text: "alpha",
        raw: &[1, 2, 3],
        presence_bits: 3,
    }];
    let mut out = [0_u8; 8];
    assert!(FixtureV1::encode_views_into(&borrowed_rows, &mut out, 8).is_err());
    Ok(())
}
"#,
    )?;
    let output = Command::new("cargo")
        .arg("test")
        .arg("--manifest-path")
        .arg(smoke.join("Cargo.toml"))
        .output()?;
    if output.status.success() {
        Ok(())
    } else {
        Err(CodegenError::Descriptor(
            String::from_utf8_lossy(&output.stderr).into_owned(),
        ))
    }
}
