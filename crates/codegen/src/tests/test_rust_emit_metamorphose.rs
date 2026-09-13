use super::*;
use crate::config::Adapter;

#[test]
fn json_adapter_surface_is_schema_local_and_checked() -> Result<()> {
    let source = run_metamorphose_codegen_to_string(valid_raw_string_proto(), Adapter::Json)?;
    assert!(source.contains("use crate::fixture_v1::*;"));
    assert!(source.contains("JsonMetamorphoseSchema"));
    assert!(source.contains("pub fn metamorphose_json("));
    assert!(source.contains("pub unsafe fn metamorphose_json_trusted_unchecked("));
    assert!(source.contains("Self::access_archived(bytes)?"));
    assert!(source.contains("Self::access_archived_trusted_unchecked(bytes)?"));
    assert!(source.contains("const JSON_FIELD_TEXT"));
    assert!(!source.contains("serde_json"));
    assert!(!source.contains("prost::Message"));
    Ok(())
}

#[test]
fn protobuf_adapter_uses_proto_numbers_and_no_dto_materialization() -> Result<()> {
    let source = run_metamorphose_codegen_to_string(valid_scalar_proto(), Adapter::Protobuf)?;
    assert!(source.contains("ProtobufMetamorphoseSchema"));
    assert!(source.contains("writer.message_prefix(2, row_len)?;"));
    assert!(source.contains("writer.string(2, entity_symbol"));
    assert!(source.contains("writer.int64(3, row.recorded_at_ms.to_native())?;"));
    assert!(!source.contains("prost::Message"));
    assert!(!source.contains("to_vec()"));
    Ok(())
}

#[test]
fn csv_adapter_uses_static_header_and_checked_writer() -> Result<()> {
    let source = run_metamorphose_codegen_to_string(valid_array_proto(), Adapter::Csv)?;
    assert!(source.contains("CsvMetamorphoseSchema"));
    assert!(source.contains("const CSV_HEADER"));
    assert!(source.contains("writer.raw_static(CSV_HEADER)?;"));
    assert!(source.contains("writer.f32_array_cell"));
    assert!(!source.contains("csv::Writer"));
    assert!(!source.contains("serde"));
    Ok(())
}

#[test]
fn derived_utc_codegen_is_row_format_only_and_nested_for_protobuf() -> Result<()> {
    let json = run_metamorphose_codegen_to_string(valid_nested_derived_utc_proto(), Adapter::Json)?;
    assert!(json.contains("const JSON_FIELD_RECORDED_AT_UTC"));
    assert!(json.contains("const JSON_FIELD_INGESTED_AT_UTC"));
    assert!(json.contains("writer.utc_value(row.recorded_at_ms.to_native())?;"));
    assert!(json.contains("writer.utc_value(row.ingested_at_ms.to_native())?;"));
    assert!(json.contains("PRESENCE_INGESTED_AT_MS"));

    let csv = run_metamorphose_codegen_to_string(valid_nested_derived_utc_proto(), Adapter::Csv)?;
    assert!(csv.contains(
        "schema_version,entity,recorded_at_ms,recorded_at_utc,metadata.ingested_at_ms,metadata.ingested_at_utc"
    ));
    assert!(csv.contains("writer.utc_cell(row.recorded_at_ms.to_native())?;"));
    assert!(csv.contains("writer.utc_cell(row.ingested_at_ms.to_native())?;"));

    let protobuf =
        run_metamorphose_codegen_to_string(valid_nested_derived_utc_proto(), Adapter::Protobuf)?;
    assert!(protobuf.contains("fn encoded_len_metadata("));
    assert!(protobuf.contains("writer.message_prefix(5, message_len)?;"));
    assert!(protobuf.contains("writer.utc(4, row.recorded_at_ms.to_native())?;"));
    assert!(protobuf.contains("writer.utc(2, row.ingested_at_ms.to_native())?;"));
    assert!(protobuf.contains("output::utc_len(row.ingested_at_ms.to_native())?"));
    assert!(!protobuf.contains("ingested_at_utc:"));

    for adapter in [
        Adapter::Transponding,
        Adapter::Arrow,
        Adapter::ArrowIpc,
        Adapter::Parquet,
    ] {
        let source = run_metamorphose_codegen_to_string(valid_nested_derived_utc_proto(), adapter)?;
        assert!(!source.contains("recorded_at_utc"));
        assert!(!source.contains("ingested_at_utc"));
    }
    Ok(())
}

#[test]
fn transponding_adapter_is_hidden_and_schema_specific() -> Result<()> {
    let source = run_metamorphose_codegen_to_string(valid_array_proto(), Adapter::Transponding)?;
    assert!(source.contains("pub(crate) struct FixtureV1ColumnBatch"));
    assert!(source.contains("pub(crate) fn transpond_archived"));
    assert!(source.contains("I64ListColumn::required"));
    assert!(!source.contains("pub fn transpond"));
    assert!(!source.contains("pub trait Transpond"));
    Ok(())
}

#[test]
fn arrow_family_adapters_keep_dependency_boundaries() -> Result<()> {
    let arrow = run_metamorphose_codegen_to_string(valid_scalar_proto(), Adapter::Arrow)?;
    assert!(arrow.contains("ArrowMetamorphoseSchema"));
    assert!(arrow.contains("mbt_adapter_arrow::"));
    assert!(arrow.contains("fn arrow_record_batch"));

    let ipc = run_metamorphose_codegen_to_string(valid_scalar_proto(), Adapter::ArrowIpc)?;
    assert!(ipc.contains("ArrowIpcMetamorphoseSchema"));
    assert!(ipc.contains("write_ipc_stream"));
    assert!(!ipc.contains("mbt_adapter_arrow::"));

    let parquet = run_metamorphose_codegen_to_string(valid_scalar_proto(), Adapter::Parquet)?;
    assert!(parquet.contains("ParquetMetamorphoseSchema"));
    assert!(parquet.contains("write_uncompressed_parquet"));
    assert!(!parquet.contains("mbt_adapter_arrow::"));
    Ok(())
}

#[test]
fn json_adapter_emits_source_and_projection_marker_sections() -> Result<()> {
    let source = run_metamorphose_codegen_to_string(
        valid_alias_and_projection_ignored_proto(),
        Adapter::Json,
    )?;
    assert!(source.contains("impl JsonMetamorphoseSchema for FixtureV1"));
    assert!(source.contains("impl JsonMetamorphoseSchema for SmallProjection"));
    assert!(source.contains("fn write_json_response("));
    assert!(source.contains("fn small_write_json_response("));
    assert!(source.contains("const SMALL_JSON_FIELD_ENTITY"));
    assert_projection_adapter_forbidden_absent(&source);
    Ok(())
}

#[test]
fn protobuf_adapter_emits_source_and_projection_marker_sections() -> Result<()> {
    let source = run_metamorphose_codegen_to_string(
        valid_alias_and_projection_ignored_proto(),
        Adapter::Protobuf,
    )?;
    assert!(source.contains("impl ProtobufMetamorphoseSchema for FixtureV1"));
    assert!(source.contains("impl ProtobufMetamorphoseSchema for SmallProjection"));
    assert!(source.contains("fn write_protobuf_response("));
    assert!(source.contains("fn small_write_protobuf_response("));
    assert!(source.contains("fn small_encoded_len_"));
    assert_projection_adapter_forbidden_absent(&source);
    Ok(())
}

#[test]
fn csv_adapter_emits_source_and_projection_marker_sections() -> Result<()> {
    let source = run_metamorphose_codegen_to_string(
        valid_alias_and_projection_ignored_proto(),
        Adapter::Csv,
    )?;
    assert!(source.contains("impl CsvMetamorphoseSchema for FixtureV1"));
    assert!(source.contains("impl CsvMetamorphoseSchema for SmallProjection"));
    assert!(source.contains("const CSV_HEADER"));
    assert!(source.contains("const SMALL_CSV_HEADER"));
    assert!(source.contains("fn small_write_csv_response("));
    assert_projection_adapter_forbidden_absent(&source);
    Ok(())
}

#[test]
fn transponding_adapter_emits_source_and_projection_marker_sections() -> Result<()> {
    let source = run_metamorphose_codegen_to_string(
        valid_alias_and_projection_ignored_proto(),
        Adapter::Transponding,
    )?;
    assert!(source.contains("pub(crate) struct FixtureV1ColumnBatch"));
    assert!(source.contains("pub(crate) struct SmallProjectionColumnBatch"));
    assert!(source.contains("impl SmallProjection"));
    assert_projection_adapter_forbidden_absent(&source);
    Ok(())
}

#[test]
fn arrow_adapter_emits_source_and_projection_marker_sections() -> Result<()> {
    let source = run_metamorphose_codegen_to_string(
        valid_alias_and_projection_ignored_proto(),
        Adapter::Arrow,
    )?;
    assert!(source.contains("impl ArrowMetamorphoseSchema for FixtureV1"));
    assert!(source.contains("impl ArrowMetamorphoseSchema for SmallProjection"));
    assert!(source.contains("fn arrow_record_batch("));
    assert!(source.contains("fn small_arrow_record_batch("));
    assert_projection_adapter_forbidden_absent(&source);
    Ok(())
}

#[test]
fn arrow_ipc_adapter_emits_source_and_projection_marker_sections() -> Result<()> {
    let source = run_metamorphose_codegen_to_string(
        valid_alias_and_projection_ignored_proto(),
        Adapter::ArrowIpc,
    )?;
    assert!(source.contains("impl ArrowIpcMetamorphoseSchema for FixtureV1"));
    assert!(source.contains("impl ArrowIpcMetamorphoseSchema for SmallProjection"));
    assert!(source.contains("fn small_arrow_record_batch("));
    assert_projection_adapter_forbidden_absent(&source);
    Ok(())
}

#[test]
fn parquet_adapter_emits_source_and_projection_marker_sections() -> Result<()> {
    let source = run_metamorphose_codegen_to_string(
        valid_alias_and_projection_ignored_proto(),
        Adapter::Parquet,
    )?;
    assert!(source.contains("impl ParquetMetamorphoseSchema for FixtureV1"));
    assert!(source.contains("impl ParquetMetamorphoseSchema for SmallProjection"));
    assert!(source.contains("fn small_arrow_record_batch("));
    assert_projection_adapter_forbidden_absent(&source);
    Ok(())
}

#[test]
fn projection_adapter_presence_checks_use_projected_presence_constants() -> Result<()> {
    let source =
        run_metamorphose_codegen_to_string(valid_optional_projection_proto(), Adapter::Json)?;
    assert!(source.contains("SMALL_PRESENCE_OPTIONAL_VALUE"));
    assert!(source.contains("row.presence_bits.to_native() & SMALL_PRESENCE_OPTIONAL_VALUE != 0"));
    assert_projection_adapter_forbidden_absent(&source);
    Ok(())
}

fn assert_projection_adapter_forbidden_absent(source: &str) {
    for forbidden in [
        "serde_json".to_string(),
        "prost::Message".to_string(),
        ["crates/", "serving"].concat(),
        ["Telemetry", "V1"].concat(),
        ["Telemetry", "V1", "Temperature", "Only"].concat(),
        ["Temperature", "Only"].concat(),
        ["temperature", "_only"].concat(),
    ] {
        assert!(
            !source.contains(&forbidden),
            "forbidden projection adapter source {forbidden}"
        );
    }
}

fn valid_optional_projection_proto() -> &'static str {
    r#"
syntax = "proto3";
package test.fixture.v1;
import "mbt/options.proto";

message TestPayloadV1 {
  option (mbt.schema_id) = 19;
  option (mbt.schema_version) = 1;
  option (mbt.transport_name) = "test.optional_projection.v1";
  option (mbt.payload_root) = true;
  option (mbt.projection) = {
    name: "small"
    rust_marker: "SmallProjection"
    include_group: "core"
  };

  uint32 schema_version = 1 [(mbt.const_u16) = 1];
  repeated TestRowV1 rows = 2 [(mbt.repeated_payload) = true];
}

message TestRowV1 {
  uint32 schema_version = 1 [(mbt.const_u16) = 1];
  int64 recorded_at_ms = 2 [(mbt.key_part) = true, (mbt.key_order) = 1];
  optional int64 optional_value = 3 [
    (mbt.presence_bit) = 0,
    (mbt.projection_group) = "core"
  ];
}
"#
}

#[test]
fn csv_bitmask_strings_use_nested_array_escaping() -> Result<()> {
    let proto = invalid_nullable_bitmask_proto().replace(", (mbt.presence_bit) = 0", "");
    let source = run_metamorphose_codegen_to_string(&proto, Adapter::Csv)?;
    assert!(source.contains("writer.array_string_cell(\"a\")?;"));
    assert!(!source.contains("writer.string_cell(\"a\")?;"));
    Ok(())
}
