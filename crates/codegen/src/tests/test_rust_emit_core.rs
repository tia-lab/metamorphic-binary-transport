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
    assert_forbidden_absent(&first);
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
