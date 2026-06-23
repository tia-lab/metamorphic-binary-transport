mod test_cli;
mod test_descriptor;
mod test_model;
mod test_options;
mod test_rust_emit_core;
mod test_rust_emit_metamorphose;
mod test_rust_emit_projection_direct_writer;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::config::{Action, Adapter, CodegenConfig, Surface};
use crate::descriptor::load_schema_model;
use crate::emit::format_rust;
use crate::error::{CodegenError, Result};
use crate::rust_emit::{
    generated_metamorphose_adapter_schema, generated_projection_schema, generated_schema,
};

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

fn temp_root(label: &str) -> Result<PathBuf> {
    let mut path = std::env::current_dir()?
        .join("target")
        .join("mbt-codegen-fixtures");
    let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    path.push(format!("{label}-{}-{counter}", std::process::id()));
    fs::create_dir_all(&path)?;
    Ok(path)
}

fn write_options_proto(root: &Path) -> Result<()> {
    write_proto(root, "mathilde/options.proto", options_proto())?;
    Ok(())
}

fn write_proto(root: &Path, relative_path: &str, contents: &str) -> Result<PathBuf> {
    let path = root.join(relative_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, contents)?;
    Ok(path)
}

fn config(root: &Path, schema: &str, module: &str) -> CodegenConfig {
    config_with_surface(root, schema, module, Surface::Core)
}

fn config_with_surface(root: &Path, schema: &str, module: &str, surface: Surface) -> CodegenConfig {
    CodegenConfig {
        action: Action::Inspect,
        proto_roots: vec![root.to_path_buf()],
        schema: PathBuf::from(schema),
        root: "test.fixture.v1.TestPayloadV1".to_string(),
        module: module.to_string(),
        surface,
        adapter: None,
        dictionary_sources: Vec::new(),
        out: None,
    }
}

fn config_with_dictionary_sources(
    root: &Path,
    schema: &str,
    module: &str,
    dictionary_sources: Vec<&str>,
) -> CodegenConfig {
    let mut cfg = config(root, schema, module);
    cfg.dictionary_sources = dictionary_sources.into_iter().map(PathBuf::from).collect();
    cfg
}

fn model_for(proto: &str) -> Result<crate::model::SchemaModel> {
    let root = temp_root("model")?;
    write_options_proto(&root)?;
    write_proto(&root, "test/fixture/v1/test.proto", proto)?;
    load_schema_model(&config(&root, "test/fixture/v1/test.proto", "fixture_v1").schema_request())
}

fn imported_dictionary_model(values: &[&str]) -> Result<crate::model::SchemaModel> {
    let root = temp_root("imported-dictionary")?;
    write_options_proto(&root)?;
    write_proto(
        &root,
        "test/fixture/v1/test.proto",
        imported_dictionary_root_proto(),
    )?;
    write_proto(
        &root,
        "shared/instruments.proto",
        &shared_instrument_dictionary_proto(values),
    )?;
    let cfg = config_with_dictionary_sources(
        &root,
        "test/fixture/v1/test.proto",
        "fixture_v1",
        vec!["shared/instruments.proto"],
    );
    load_schema_model(&cfg.schema_request())
}

fn run_codegen_to_string(proto: &str) -> Result<String> {
    let model = model_for(proto)?;
    format_rust(&generated_schema(&model)?)
}

fn run_projection_codegen_to_string(proto: &str) -> Result<String> {
    let model = model_for(proto)?;
    format_rust(&generated_projection_schema(&model)?)
}

fn run_metamorphose_codegen_to_string(proto: &str, adapter: Adapter) -> Result<String> {
    let model = model_for(proto)?;
    format_rust(&generated_metamorphose_adapter_schema(&model, adapter)?)
}

fn assert_forbidden_absent(source: &str) {
    for forbidden in [
        "metamorphose",
        "transpond",
        "arrow",
        "parquet",
        "serde_json",
        "prost",
        "project_",
        "ProjectionModel",
        "TargetModel",
    ] {
        assert!(
            !source.contains(forbidden),
            "forbidden generated surface {forbidden}"
        );
    }
}

fn smoke_crate(root: &Path, generated_source: &str) -> Result<()> {
    let smoke = crate::emit::smoke_crate(root, generated_source)?;
    let output = Command::new("cargo")
        .arg("check")
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

fn valid_scalar_proto() -> &'static str {
    r#"
syntax = "proto3";
package test.fixture.v1;
import "mathilde/options.proto";

option (mathilde.dictionary_values) = {
  name: "entity"
  value: "btc"
  value: "eth"
};

message TestPayloadV1 {
  option (mathilde.schema_id) = 11;
  option (mathilde.schema_version) = 1;
  option (mathilde.transport_name) = "test.fixture.v1";
  option (mathilde.payload_root) = true;

  uint32 schema_version = 1 [(mathilde.const_u16) = 1];
  repeated TestRowV1 rows = 2 [(mathilde.repeated_payload) = true];
}

message TestRowV1 {
  uint32 schema_version = 1 [(mathilde.const_u16) = 1];
  string entity = 2 [
    (mathilde.dictionary) = "entity",
    (mathilde.key_part) = true,
    (mathilde.key_order) = 1
  ];
  int64 close_ms = 3 [
    (mathilde.key_part) = true,
    (mathilde.key_order) = 2
  ];
  double c = 4;
  float f = 5;
  bool active = 6;
  int32 count_i32 = 7;
  uint32 count_u32 = 8;
}
"#
}

fn valid_raw_string_proto() -> &'static str {
    r#"
syntax = "proto3";
package test.fixture.v1;
import "mathilde/options.proto";

message TestPayloadV1 {
  option (mathilde.schema_id) = 12;
  option (mathilde.schema_version) = 1;
  option (mathilde.transport_name) = "test.raw.v1";
  option (mathilde.payload_root) = true;

  uint32 schema_version = 1 [(mathilde.const_u16) = 1];
  repeated TestRowV1 rows = 2 [(mathilde.repeated_payload) = true];
}

message TestRowV1 {
  uint32 schema_version = 1 [(mathilde.const_u16) = 1];
  int64 close_ms = 2 [(mathilde.key_part) = true, (mathilde.key_order) = 1];
  optional string text = 3 [(mathilde.raw_string) = true, (mathilde.presence_bit) = 0];
  optional bytes raw = 4 [(mathilde.presence_bit) = 1];
}
"#
}

fn valid_array_proto() -> &'static str {
    r#"
syntax = "proto3";
package test.fixture.v1;
import "mathilde/options.proto";

message TestPayloadV1 {
  option (mathilde.schema_id) = 13;
  option (mathilde.schema_version) = 1;
  option (mathilde.transport_name) = "test.array.v1";
  option (mathilde.payload_root) = true;

  uint32 schema_version = 1 [(mathilde.const_u16) = 1];
  repeated TestRowV1 rows = 2 [(mathilde.repeated_payload) = true];
}

message TestRowV1 {
  uint32 schema_version = 1 [(mathilde.const_u16) = 1];
  int64 close_ms = 2 [(mathilde.key_part) = true, (mathilde.key_order) = 1];
  repeated int64 xs_i64 = 3;
  repeated int32 xs_i32 = 4;
  repeated uint32 xs_u32 = 5;
  repeated double xs_f64 = 6;
  repeated float xs_f32 = 7 [(mathilde.presence_bit) = 0];
}
"#
}

fn valid_wide_presence_proto() -> String {
    let mut fields = String::new();
    for idx in 0..70_u32 {
        fields.push_str(&format!(
            "  optional int64 opt_{idx} = {} [(mathilde.presence_bit) = {idx}];\n",
            idx + 3
        ));
    }
    format!(
        r#"
syntax = "proto3";
package test.fixture.v1;
import "mathilde/options.proto";

message TestPayloadV1 {{
  option (mathilde.schema_id) = 14;
  option (mathilde.schema_version) = 1;
  option (mathilde.transport_name) = "test.wide.v1";
  option (mathilde.payload_root) = true;

  uint32 schema_version = 1 [(mathilde.const_u16) = 1];
  repeated TestRowV1 rows = 2 [(mathilde.repeated_payload) = true];
}}

message TestRowV1 {{
  uint32 schema_version = 1 [(mathilde.const_u16) = 1];
  int64 close_ms = 2 [(mathilde.key_part) = true, (mathilde.key_order) = 1];
{fields}}}
"#
    )
}

fn valid_alias_and_projection_ignored_proto() -> &'static str {
    r#"
syntax = "proto3";
package test.fixture.v1;
import "mathilde/options.proto";

option (mathilde.dictionary_values) = {
  name: "entity"
  value: "btc"
  alias: { value: "btc" alias: "xbt" }
};

message TestPayloadV1 {
  option (mathilde.schema_id) = 15;
  option (mathilde.schema_version) = 1;
  option (mathilde.transport_name) = "test.alias.v1";
  option (mathilde.payload_root) = true;
  option (mathilde.projection) = {
    name: "small"
    rust_marker: "SmallProjection"
    include_group: "core"
  };

  uint32 schema_version = 1 [(mathilde.const_u16) = 1];
  repeated TestRowV1 rows = 2 [(mathilde.repeated_payload) = true];
}

message TestRowV1 {
  uint32 schema_version = 1 [(mathilde.const_u16) = 1];
  string entity = 2 [
    (mathilde.dictionary) = "entity",
    (mathilde.key_part) = true,
    (mathilde.key_order) = 1,
    (mathilde.projection_group) = "core"
  ];
  int64 close_ms = 3 [(mathilde.key_part) = true, (mathilde.key_order) = 2];
  string ignored_utc = 4 [(mathilde.ignored) = true, (mathilde.derived_utc_from) = "close_ms"];
}
"#
}

fn valid_nested_derived_utc_proto() -> &'static str {
    r#"
syntax = "proto3";
package test.fixture.v1;
import "mathilde/options.proto";

option (mathilde.dictionary_values) = {
  name: "entity"
  value: "btc"
  value: "eth"
};

message TestPayloadV1 {
  option (mathilde.schema_id) = 18;
  option (mathilde.schema_version) = 1;
  option (mathilde.transport_name) = "test.derived.v1";
  option (mathilde.payload_root) = true;

  uint32 schema_version = 1 [(mathilde.const_u16) = 1];
  repeated TestRowV1 rows = 2 [(mathilde.repeated_payload) = true];
}

message TestRowV1 {
  uint32 schema_version = 1 [(mathilde.const_u16) = 1];
  string entity = 2 [
    (mathilde.dictionary) = "entity",
    (mathilde.key_part) = true,
    (mathilde.key_order) = 1
  ];
  int64 close_ms = 3 [
    (mathilde.key_part) = true,
    (mathilde.key_order) = 2
  ];
  string close_utc = 4 [
    (mathilde.ignored) = true,
    (mathilde.derived_utc_from) = "close_ms"
  ];
  TestMetadataV1 metadata = 5;
}

message TestMetadataV1 {
  optional int64 ingested_at_ms = 1 [(mathilde.presence_bit) = 0];
  optional string ingested_at_utc = 2 [
    (mathilde.ignored) = true,
    (mathilde.derived_utc_from) = "ingested_at_ms"
  ];
}
"#
}

fn imported_dictionary_root_proto() -> &'static str {
    r#"
syntax = "proto3";
package test.fixture.v1;
import "mathilde/options.proto";
import "shared/instruments.proto";

message TestPayloadV1 {
  option (mathilde.schema_id) = 21;
  option (mathilde.schema_version) = 1;
  option (mathilde.transport_name) = "test.imported.dictionary.v1";
  option (mathilde.payload_root) = true;

  uint32 schema_version = 1 [(mathilde.const_u16) = 1];
  repeated TestRowV1 rows = 2 [(mathilde.repeated_payload) = true];
}

message TestRowV1 {
  uint32 schema_version = 1 [(mathilde.const_u16) = 1];
  string instrument = 2 [
    (mathilde.dictionary) = "instrument",
    (mathilde.key_part) = true,
    (mathilde.key_order) = 1
  ];
  int64 close_ms = 3 [
    (mathilde.key_part) = true,
    (mathilde.key_order) = 2
  ];
}
"#
}

fn shared_instrument_dictionary_proto(values: &[&str]) -> String {
    let mut proto = String::from(
        r#"
syntax = "proto3";
package test.shared.v1;
import "mathilde/options.proto";

option (mathilde.dictionary_values) = {
  name: "instrument"
"#,
    );
    for value in values {
        proto.push_str(&format!(r#"  value: "{value}""#));
        proto.push('\n');
    }
    proto.push_str("};\n");
    proto
}

fn invalid_derived_utc_without_ignored_proto() -> &'static str {
    valid_nested_derived_utc_proto()
        .replace(
            r#"  string close_utc = 4 [
    (mathilde.ignored) = true,
    (mathilde.derived_utc_from) = "close_ms"
  ];"#,
            r#"  string close_utc = 4 [
    (mathilde.derived_utc_from) = "close_ms"
  ];"#,
        )
        .leak()
}

fn invalid_derived_utc_unknown_source_proto() -> &'static str {
    valid_nested_derived_utc_proto()
        .replace(
            r#"(mathilde.derived_utc_from) = "close_ms""#,
            r#"(mathilde.derived_utc_from) = "missing_ms""#,
        )
        .leak()
}

fn invalid_derived_utc_non_i64_source_proto() -> &'static str {
    valid_nested_derived_utc_proto()
        .replace(
            r#"(mathilde.derived_utc_from) = "close_ms""#,
            r#"(mathilde.derived_utc_from) = "entity""#,
        )
        .leak()
}

fn invalid_derived_utc_non_string_field_proto() -> &'static str {
    valid_nested_derived_utc_proto()
        .replace("string close_utc = 4", "int64 close_utc = 4")
        .leak()
}

fn invalid_repeated_derived_utc_proto() -> &'static str {
    valid_nested_derived_utc_proto()
        .replace("string close_utc = 4", "repeated string close_utc = 4")
        .leak()
}

fn invalid_duplicate_protobuf_output_tag_proto() -> &'static str {
    valid_nested_derived_utc_proto()
        .replace(
            "optional string ingested_at_utc = 2",
            "optional string ingested_at_utc = 1",
        )
        .leak()
}

fn invalid_duplicate_protobuf_helper_stem_proto() -> &'static str {
    r#"
syntax = "proto3";
package test.fixture.v1;
import "mathilde/options.proto";

message TestPayloadV1 {
  option (mathilde.schema_id) = 19;
  option (mathilde.schema_version) = 1;
  option (mathilde.transport_name) = "test.helper.collision.v1";
  option (mathilde.payload_root) = true;

  uint32 schema_version = 1 [(mathilde.const_u16) = 1];
  repeated TestRowV1 rows = 2 [(mathilde.repeated_payload) = true];
}

message TestRowV1 {
  uint32 schema_version = 1 [(mathilde.const_u16) = 1];
  int64 close_ms = 2 [(mathilde.key_part) = true, (mathilde.key_order) = 1];
  Foo foo = 3;
  FooBar foo_bar = 4;
}

message Foo {
  Bar bar = 1;
}

message Bar {
  int64 first_ms = 1;
}

message FooBar {
  int64 second_ms = 1;
}
"#
}

fn invalid_unannotated_string_proto() -> &'static str {
    valid_scalar_proto()
        .replace("double c = 4;", "string c = 4;")
        .leak()
}

fn invalid_missing_presence_proto() -> &'static str {
    valid_raw_string_proto()
        .replace(
            "optional string text = 3 [(mathilde.raw_string) = true, (mathilde.presence_bit) = 0];",
            "optional string text = 3 [(mathilde.raw_string) = true];",
        )
        .leak()
}

fn invalid_duplicate_presence_proto() -> &'static str {
    valid_raw_string_proto()
        .replace(
            "optional bytes raw = 4 [(mathilde.presence_bit) = 1];",
            "optional bytes raw = 4 [(mathilde.presence_bit) = 0];",
        )
        .leak()
}

fn invalid_gapped_presence_proto() -> &'static str {
    valid_raw_string_proto()
        .replace(
            "optional bytes raw = 4 [(mathilde.presence_bit) = 1];",
            "optional bytes raw = 4 [(mathilde.presence_bit) = 2];",
        )
        .leak()
}

fn invalid_duplicate_key_order_proto() -> &'static str {
    valid_scalar_proto()
        .replace("(mathilde.key_order) = 2", "(mathilde.key_order) = 1")
        .leak()
}

fn invalid_gapped_key_order_proto() -> &'static str {
    valid_scalar_proto()
        .replace("(mathilde.key_order) = 2", "(mathilde.key_order) = 3")
        .leak()
}

fn invalid_nullable_bitmask_proto() -> &'static str {
    r#"
syntax = "proto3";
package test.fixture.v1;
import "mathilde/options.proto";
option (mathilde.dictionary_values) = { name: "tag" value: "a" };
message TestPayloadV1 {
  option (mathilde.schema_id) = 16;
  option (mathilde.schema_version) = 1;
  option (mathilde.transport_name) = "test.invalid.v1";
  option (mathilde.payload_root) = true;
  uint32 schema_version = 1 [(mathilde.const_u16) = 1];
  repeated TestRowV1 rows = 2 [(mathilde.repeated_payload) = true];
}
message TestRowV1 {
  uint32 schema_version = 1 [(mathilde.const_u16) = 1];
  repeated string tags = 2 [(mathilde.bitmask_dictionary) = "tag", (mathilde.presence_bit) = 0];
}
"#
}

fn invalid_repeated_string_proto() -> &'static str {
    valid_array_proto()
        .replace("repeated int64 xs_i64 = 3;", "repeated string xs_i64 = 3;")
        .leak()
}

fn invalid_repeated_bytes_proto() -> &'static str {
    valid_array_proto()
        .replace("repeated int64 xs_i64 = 3;", "repeated bytes xs_i64 = 3;")
        .leak()
}

fn invalid_repeated_bool_proto() -> &'static str {
    valid_array_proto()
        .replace("repeated int64 xs_i64 = 3;", "repeated bool xs_i64 = 3;")
        .leak()
}

fn invalid_duplicate_rust_field_proto() -> &'static str {
    r#"
syntax = "proto3";
package test.fixture.v1;
import "mathilde/options.proto";
option (mathilde.dictionary_values) = { name: "entity" value: "btc" };
message TestPayloadV1 {
  option (mathilde.schema_id) = 17;
  option (mathilde.schema_version) = 1;
  option (mathilde.transport_name) = "test.duplicate.v1";
  option (mathilde.payload_root) = true;
  uint32 schema_version = 1 [(mathilde.const_u16) = 1];
  repeated TestRowV1 rows = 2 [(mathilde.repeated_payload) = true];
}
message TestRowV1 {
  uint32 schema_version = 1 [(mathilde.const_u16) = 1];
  string entity = 2 [(mathilde.dictionary) = "entity"];
  uint32 entity_ordinal = 3;
}
"#
}

fn options_proto() -> &'static str {
    include_str!("../../../../proto/mathilde/options.proto")
}
