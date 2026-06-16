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
    assert!(source.contains("writer.int64(3, row.close_ms.to_native())?;"));
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
    assert!(json.contains("const JSON_FIELD_CLOSE_UTC"));
    assert!(json.contains("const JSON_FIELD_INGESTED_AT_UTC"));
    assert!(json.contains("writer.utc_value(row.close_ms.to_native())?;"));
    assert!(json.contains("writer.utc_value(row.ingested_at_ms.to_native())?;"));
    assert!(json.contains("PRESENCE_INGESTED_AT_MS"));

    let csv = run_metamorphose_codegen_to_string(valid_nested_derived_utc_proto(), Adapter::Csv)?;
    assert!(csv.contains(
        "schema_version,entity,close_ms,close_utc,metadata.ingested_at_ms,metadata.ingested_at_utc"
    ));
    assert!(csv.contains("writer.utc_cell(row.close_ms.to_native())?;"));
    assert!(csv.contains("writer.utc_cell(row.ingested_at_ms.to_native())?;"));

    let protobuf =
        run_metamorphose_codegen_to_string(valid_nested_derived_utc_proto(), Adapter::Protobuf)?;
    assert!(protobuf.contains("fn encoded_len_metadata("));
    assert!(protobuf.contains("writer.message_prefix(5, message_len)?;"));
    assert!(protobuf.contains("writer.utc(4, row.close_ms.to_native())?;"));
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
        assert!(!source.contains("close_utc"));
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
