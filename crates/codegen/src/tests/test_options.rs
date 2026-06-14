use super::*;
use crate::descriptor::load_schema_model;

#[test]
fn options_file_is_proto2_and_mbt_only() -> Result<()> {
    let text = options_proto();
    assert!(text.contains("syntax = \"proto2\";"));
    assert!(text.contains("extend google.protobuf.FileOptions"));
    for forbidden in [
        "postgres", "sqlite", "lookup", "mldb", "cache", "serving", "db.",
    ] {
        assert!(
            !text.contains(forbidden),
            "forbidden option surface {forbidden}"
        );
    }
    Ok(())
}

#[test]
fn extension_lookup_succeeds_through_descriptor_load() -> Result<()> {
    let root = temp_root("options")?;
    write_options_proto(&root)?;
    write_proto(&root, "test/fixture/v1/test.proto", valid_scalar_proto())?;
    let model = load_schema_model(
        &config(&root, "test/fixture/v1/test.proto", "fixture_v1").schema_request(),
    )?;
    assert_eq!(model.schema_id, 11);
    assert_eq!(model.transport_name, "test.fixture.v1");
    Ok(())
}
