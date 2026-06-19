use crate::config::Adapter;
use crate::error::{CodegenError, Result};
use crate::model::{
    DerivedUtcField, Dictionary, FieldKind, JsonCsvOutputField, PhysicalField, ProjectionModel,
    ProtobufMessageModel, ProtobufOutputField, SchemaModel, const_name, dict_prefix,
};

struct EmitScope<'a> {
    model: &'a SchemaModel,
    symbol_prefix: String,
    fn_prefix: String,
    public_free_items: bool,
}

impl<'a> EmitScope<'a> {
    fn source(model: &'a SchemaModel) -> Self {
        Self {
            model,
            symbol_prefix: String::new(),
            fn_prefix: String::new(),
            public_free_items: true,
        }
    }

    fn projection(model: &'a SchemaModel, projection_name: &str) -> Self {
        Self {
            model,
            symbol_prefix: format!("{}_", const_name(projection_name)),
            fn_prefix: format!("{projection_name}_"),
            public_free_items: false,
        }
    }

    fn item_vis(&self) -> &'static str {
        if self.public_free_items { "pub " } else { "" }
    }

    fn const_name(&self, name: &str) -> String {
        format!("{}{}", self.symbol_prefix, name)
    }

    fn fn_name(&self, name: &str) -> String {
        format!("{}{}", self.fn_prefix, name)
    }
}

pub fn generated_schema(model: &SchemaModel) -> Result<String> {
    // Core schema output owns archive structs, validation, checksums, and access APIs.
    validate_model(model)?;
    let scope = EmitScope::source(model);
    let mut out = String::new();
    emit_header(&mut out, model);
    emit_imports(&mut out);
    emit_schema_constants(&mut out, &scope);
    emit_dictionary_constants(&mut out, model)?;
    emit_presence_constants(&mut out, &scope)?;
    emit_structs(&mut out, model);
    emit_validation(&mut out, &scope)?;
    emit_checksums(&mut out, &scope);
    emit_dictionary_helpers(&mut out, model)?;
    emit_runtime_api(&mut out, &scope);
    emit_runtime_trait(&mut out, model);
    emit_view_types(&mut out, &scope);
    emit_decode_helpers(&mut out, &scope, true);
    Ok(out)
}

pub fn generated_projection_schema(model: &SchemaModel) -> Result<String> {
    // Projection output emits the source schema plus all proto-declared projections.
    validate_model(model)?;
    let source_scope = EmitScope::source(model);
    let projection_sections = model
        .projections
        .iter()
        .map(|projection| projection_schema_model(model, projection))
        .collect::<Vec<_>>();
    for projection in &projection_sections {
        validate_model(projection)?;
    }

    let mut out = String::new();
    emit_header(&mut out, model);
    emit_imports(&mut out);
    emit_schema_constants(&mut out, &source_scope);
    emit_dictionary_constants(&mut out, model)?;
    emit_presence_constants(&mut out, &source_scope)?;
    emit_structs(&mut out, model);
    emit_validation(&mut out, &source_scope)?;
    emit_checksums(&mut out, &source_scope);
    emit_dictionary_helpers(&mut out, model)?;
    emit_runtime_api(&mut out, &source_scope);
    emit_runtime_trait(&mut out, model);
    emit_view_types(&mut out, &source_scope);
    emit_decode_helpers(&mut out, &source_scope, true);
    emit_direct_projection_array_wrappers(&mut out, &projection_sections);

    for (projection, projection_model) in model.projections.iter().zip(projection_sections.iter()) {
        let scope = EmitScope::projection(projection_model, &projection.definition.name);
        emit_schema_constants(&mut out, &scope);
        emit_presence_constants(&mut out, &scope)?;
        emit_structs(&mut out, projection_model);
        emit_validation(&mut out, &scope)?;
        emit_checksums(&mut out, &scope);
        emit_runtime_api(&mut out, &scope);
        emit_runtime_trait(&mut out, projection_model);
        emit_view_types(&mut out, &scope);
        emit_decode_helpers(&mut out, &scope, false);
        emit_source_projection_api(&mut out, model, projection, projection_model);
    }
    Ok(out)
}

pub fn generated_metamorphose_adapter_schema(
    model: &SchemaModel,
    adapter: Adapter,
) -> Result<String> {
    // Metamorphose adapters are opt-in so unrelated formats do not compile.
    validate_model(model)?;
    let mut out = String::new();
    emit_header(&mut out, model);
    match adapter {
        Adapter::Json => emit_metamorphose_json(&mut out, model)?,
        Adapter::Protobuf => emit_metamorphose_protobuf(&mut out, model)?,
        Adapter::Csv => emit_metamorphose_csv(&mut out, model)?,
        Adapter::Transponding => emit_metamorphose_transponding(&mut out, model)?,
        Adapter::Arrow => emit_metamorphose_arrow(&mut out, model)?,
        Adapter::ArrowIpc => emit_metamorphose_arrow_ipc(&mut out, model)?,
        Adapter::Parquet => emit_metamorphose_parquet(&mut out, model)?,
    };
    Ok(out)
}

fn emit_metamorphose_json(out: &mut String, model: &SchemaModel) -> Result<()> {
    // JSON emission writes archived fields directly through JsonWriter helpers.
    emit_adapter_prelude(out, model, true);
    out.push_str("use mbt_adapter_json::JsonWriter;\n");
    out.push_str("use mbt_metamorphose::{runtime::TrustedUnchecked, JsonMetamorphoseSchema};\n\n");
    emit_adapter_sections(out, model, emit_json_adapter_section)
}

fn emit_json_adapter_section(out: &mut String, scope: &EmitScope<'_>) -> Result<()> {
    ensure_json_csv_outputs(scope.model, "JSON")?;
    emit_projection_adapter_presence_constants(out, scope)?;
    emit_json_field_constants(out, scope);
    emit_json_inherent_api(out, scope);
    out.push_str(&format!(
        "impl JsonMetamorphoseSchema for {} {{\n",
        scope.model.marker_type
    ));
    out.push_str("    fn metamorphose_json(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>> { Self::metamorphose_json(bytes, max_response_bytes) }\n");
    out.push_str("    fn metamorphose_json_trusted_unchecked(bytes: &[u8], max_response_bytes: usize, _trusted: TrustedUnchecked) -> Result<Vec<u8>> {\n");
    out.push_str(
        "        unsafe { Self::metamorphose_json_trusted_unchecked(bytes, max_response_bytes) }\n",
    );
    out.push_str("    }\n");
    out.push_str("}\n\n");
    emit_json_writer_helpers(out, scope);
    Ok(())
}

fn emit_metamorphose_protobuf(out: &mut String, model: &SchemaModel) -> Result<()> {
    // Protobuf emission keeps generated length accounting next to wire writes.
    emit_adapter_prelude(out, model, true);
    out.push_str("use mbt_adapter_protobuf::{self as proto, ProtoWriter};\n");
    out.push_str("use mbt_core::output;\n");
    out.push_str(
        "use mbt_metamorphose::{runtime::TrustedUnchecked, ProtobufMetamorphoseSchema};\n\n",
    );
    emit_adapter_sections(out, model, emit_protobuf_adapter_section)
}

fn emit_protobuf_adapter_section(out: &mut String, scope: &EmitScope<'_>) -> Result<()> {
    ensure_protobuf_outputs(scope.model)?;
    emit_projection_adapter_presence_constants(out, scope)?;
    emit_protobuf_inherent_api(out, scope);
    out.push_str(&format!(
        "impl ProtobufMetamorphoseSchema for {} {{\n",
        scope.model.marker_type
    ));
    out.push_str("    fn metamorphose_protobuf(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>> { Self::metamorphose_protobuf(bytes, max_response_bytes) }\n");
    out.push_str("    fn metamorphose_protobuf_trusted_unchecked(bytes: &[u8], max_response_bytes: usize, _trusted: TrustedUnchecked) -> Result<Vec<u8>> {\n");
    out.push_str("        unsafe { Self::metamorphose_protobuf_trusted_unchecked(bytes, max_response_bytes) }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
    emit_protobuf_writer_helpers(out, scope);
    Ok(())
}

fn emit_metamorphose_csv(out: &mut String, model: &SchemaModel) -> Result<()> {
    // CSV emission shares the row-format output order with JSON.
    emit_adapter_prelude(out, model, true);
    out.push_str("use mbt_adapter_csv::CsvWriter;\n");
    out.push_str("use mbt_metamorphose::{runtime::TrustedUnchecked, CsvMetamorphoseSchema};\n\n");
    emit_adapter_sections(out, model, emit_csv_adapter_section)
}

fn emit_csv_adapter_section(out: &mut String, scope: &EmitScope<'_>) -> Result<()> {
    ensure_json_csv_outputs(scope.model, "CSV")?;
    emit_projection_adapter_presence_constants(out, scope)?;
    emit_csv_header(out, scope);
    emit_csv_inherent_api(out, scope);
    out.push_str(&format!(
        "impl CsvMetamorphoseSchema for {} {{\n",
        scope.model.marker_type
    ));
    out.push_str("    fn metamorphose_csv(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>> { Self::metamorphose_csv(bytes, max_response_bytes) }\n");
    out.push_str("    fn metamorphose_csv_trusted_unchecked(bytes: &[u8], max_response_bytes: usize, _trusted: TrustedUnchecked) -> Result<Vec<u8>> {\n");
    out.push_str(
        "        unsafe { Self::metamorphose_csv_trusted_unchecked(bytes, max_response_bytes) }\n",
    );
    out.push_str("    }\n");
    out.push_str("}\n\n");
    emit_csv_writer_helpers(out, scope);
    Ok(())
}

fn emit_adapter_prelude(out: &mut String, model: &SchemaModel, include_archive: bool) {
    out.push_str(&format!("use crate::{}::*;\n", model.module));
    if include_archive {
        out.push_str("use rkyv::Archive;\n");
    }
    out.push_str("use mbt_core::error::Result;\n");
}

fn emit_projection_adapter_presence_constants(
    out: &mut String,
    scope: &EmitScope<'_>,
) -> Result<()> {
    if scope.public_free_items {
        return Ok(());
    }
    let model = scope.model;
    for field in presence_fields(model) {
        let bit = field
            .presence_bit
            .ok_or_else(|| CodegenError::InvalidSchema("presence field without bit".to_string()))?;
        if presence_is_wide(model) {
            out.push_str(&format!(
                "const {}: usize = {};\n",
                presence_word_const(scope, field),
                presence_word(bit)
            ));
            out.push_str(&format!(
                "const {}: u64 = {};\n",
                presence_mask_const(scope, field),
                presence_mask(bit)
            ));
        } else {
            out.push_str(&format!(
                "const {}: u64 = 1 << {bit};\n",
                presence_const(scope, field)
            ));
        }
    }
    if has_presence(model) {
        out.push('\n');
    }
    Ok(())
}

fn emit_adapter_sections(
    out: &mut String,
    model: &SchemaModel,
    emit_section: fn(&mut String, &EmitScope<'_>) -> Result<()>,
) -> Result<()> {
    let source_scope = EmitScope::source(model);
    emit_section(out, &source_scope)?;

    let projection_sections = model
        .projections
        .iter()
        .map(|projection| projection_adapter_schema_model(model, projection))
        .collect::<Result<Vec<_>>>()?;
    for (projection, projection_model) in model.projections.iter().zip(projection_sections.iter()) {
        validate_model(projection_model)?;
        let scope = EmitScope::projection(projection_model, &projection.definition.name);
        emit_section(out, &scope)?;
    }
    Ok(())
}

fn ensure_json_csv_outputs(model: &SchemaModel, adapter: &str) -> Result<()> {
    if model.json_csv_output_fields.is_empty() {
        return Err(CodegenError::InvalidSchema(format!(
            "{adapter} adapter for {} has no retained row-format fields",
            model.marker_type
        )));
    }
    Ok(())
}

fn ensure_protobuf_outputs(model: &SchemaModel) -> Result<()> {
    if model.protobuf_messages.is_empty() || model.protobuf_messages[0].fields.is_empty() {
        return Err(CodegenError::InvalidSchema(format!(
            "protobuf adapter for {} has no retained row-format fields",
            model.marker_type
        )));
    }
    Ok(())
}

fn emit_json_field_constants(out: &mut String, scope: &EmitScope<'_>) {
    let model = scope.model;
    out.push_str(&format!(
        "const {}: &[u8] = b\"\\\"schema_version\\\":\";\n",
        scope.const_name("JSON_SCHEMA_VERSION_FIELD")
    ));
    out.push_str(&format!(
        "const {}: &[u8] = b\"\\\"{}\\\":\";\n",
        scope.const_name("JSON_ROWS_FIELD"),
        model.row_field_name
    ));
    for output in &model.json_csv_output_fields {
        out.push_str(&format!(
            "const {}: &[u8] = b\"\\\"{}\\\":\";\n",
            scope.const_name(&format!(
                "JSON_FIELD_{}",
                json_output_const_name(model, output)
            )),
            json_output_field_name(model, output)
        ));
    }
    out.push('\n');
}

fn emit_json_inherent_api(out: &mut String, scope: &EmitScope<'_>) {
    let model = scope.model;
    let response_fn = scope.fn_name("write_json_response");
    out.push_str(&format!("impl {} {{\n", model.marker_type));
    out.push_str("    pub fn metamorphose_json(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>> {\n");
    out.push_str("        let archived = Self::access_archived(bytes)?;\n");
    out.push_str(&format!(
        "        {response_fn}(archived, max_response_bytes)\n"
    ));
    out.push_str("    }\n\n");
    out.push_str(
        "    /// Metamorphoses immutable bytes already validated for this schema into JSON.\n",
    );
    out.push_str("    ///\n");
    out.push_str("    /// # Safety\n");
    out.push_str("    /// The caller guarantees checked schema validation happened before immutable storage or transport.\n");
    out.push_str("    pub unsafe fn metamorphose_json_trusted_unchecked(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>> {\n");
    out.push_str(
        "        let archived = unsafe { Self::access_archived_trusted_unchecked(bytes)? };\n",
    );
    out.push_str(&format!(
        "        {response_fn}(archived, max_response_bytes)\n"
    ));
    out.push_str("    }\n");
    out.push_str("}\n\n");
}

fn emit_json_writer_helpers(out: &mut String, scope: &EmitScope<'_>) {
    let model = scope.model;
    let response_fn = scope.fn_name("write_json_response");
    let row_fn = scope.fn_name("write_json_row");
    out.push_str(&format!(
        "fn {response_fn}(archived: &Archived{}, max_response_bytes: usize) -> Result<Vec<u8>> {{\n",
        model.payload_type
    ));
    out.push_str("    let mut writer = JsonWriter::with_capacity(max_response_bytes, max_response_bytes.min(4096));\n");
    out.push_str("    writer.begin_object()?;\n");
    out.push_str(&format!(
        "    writer.raw_static({})?;\n",
        scope.const_name("JSON_SCHEMA_VERSION_FIELD")
    ));
    out.push_str(&format!(
        "    writer.u32_value(u32::from({}_VALUE))?;\n",
        "SCHEMA_VERSION"
    ));
    out.push_str("    writer.comma()?;\n");
    out.push_str(&format!(
        "    writer.raw_static({})?;\n",
        scope.const_name("JSON_ROWS_FIELD")
    ));
    out.push_str("    writer.begin_array()?;\n");
    out.push_str("    let mut first_row = true;\n");
    out.push_str(&format!(
        "    for row in archived.{}.iter() {{\n",
        model.row_field_name
    ));
    out.push_str("        if first_row { first_row = false; } else { writer.comma()?; }\n");
    out.push_str(&format!("        {row_fn}(row, &mut writer)?;\n"));
    out.push_str("    }\n");
    out.push_str("    writer.end_array()?;\n");
    out.push_str("    writer.end_object()?;\n");
    out.push_str("    Ok(writer.finish())\n");
    out.push_str("}\n\n");

    out.push_str(&format!(
        "fn {row_fn}(row: &<{} as Archive>::Archived, writer: &mut JsonWriter) -> Result<()> {{\n",
        model.row_type
    ));
    out.push_str("    writer.begin_object()?;\n");
    out.push_str("    let mut first = true;\n");
    for output in &model.json_csv_output_fields {
        if let Some(condition) = json_output_optional_condition(scope, output) {
            out.push_str(&format!("    if {condition} {{\n"));
            emit_json_output_write(out, scope, output, "        ");
            out.push_str("    }\n");
        } else {
            emit_json_output_write(out, scope, output, "    ");
        }
    }
    out.push_str("    writer.end_object()\n");
    out.push_str("}\n\n");
    emit_json_bitmask_helpers(out, scope);
    emit_json_field_prefix_helper(out, scope);
}

fn emit_json_output_write(
    out: &mut String,
    scope: &EmitScope<'_>,
    output: &JsonCsvOutputField,
    indent: &str,
) {
    let model = scope.model;
    let prefix_fn = scope.fn_name("write_json_field_prefix");
    out.push_str(&format!(
        "{indent}{prefix_fn}(writer, &mut first, {})?;\n",
        scope.const_name(&format!(
            "JSON_FIELD_{}",
            json_output_const_name(model, output)
        ))
    ));
    match output {
        JsonCsvOutputField::Physical { field_index } => {
            emit_json_value_write(out, scope, &model.fields[*field_index], indent);
        }
        JsonCsvOutputField::DerivedUtc { derived_index } => {
            let field = &model.derived_utc_fields[*derived_index];
            out.push_str(&format!(
                "{indent}writer.utc_value(row.{}.to_native())?;\n",
                field.source_rust_name
            ));
        }
    }
}

fn emit_json_value_write(
    out: &mut String,
    scope: &EmitScope<'_>,
    field: &PhysicalField,
    indent: &str,
) {
    let model = scope.model;
    let value = archived_value_access_for("row", field);
    match &field.kind {
        FieldKind::ConstU16 { value } => out.push_str(&format!(
            "{indent}writer.u32_value(u32::from({value}_u16))?;\n"
        )),
        FieldKind::U16Dictionary { dictionary, .. } => out.push_str(&format!(
            "{indent}writer.string_value({}_symbol(row.{}.to_native())?)?;\n",
            dictionary_helper_stem(model, dictionary),
            field.rust_name
        )),
        FieldKind::U64BitmaskDictionary { dictionary } => out.push_str(&format!(
            "{indent}{}(row.{}.to_native(), writer)?;\n",
            scope.fn_name(&format!(
                "write_json_{}_bitmask",
                const_name(dictionary).to_ascii_lowercase()
            )),
            field.rust_name
        )),
        FieldKind::I32 => out.push_str(&format!("{indent}writer.i32_value({value})?;\n")),
        FieldKind::U32 => out.push_str(&format!("{indent}writer.u32_value({value})?;\n")),
        FieldKind::I64 => out.push_str(&format!("{indent}writer.i64_value({value})?;\n")),
        FieldKind::F32 => out.push_str(&format!(
            "{indent}writer.f32_value({:?}, {value})?;\n",
            field.logical_path
        )),
        FieldKind::F64 => out.push_str(&format!(
            "{indent}writer.f64_value({:?}, {value})?;\n",
            field.logical_path
        )),
        FieldKind::Bool => out.push_str(&format!("{indent}writer.bool_value({value})?;\n")),
        FieldKind::Bytes => out.push_str(&format!("{indent}writer.bytes_value({value})?;\n")),
        FieldKind::RawString => out.push_str(&format!("{indent}writer.string_value({value})?;\n")),
        FieldKind::I64Array => {
            out.push_str(&format!("{indent}writer.i64_array_value({value})?;\n"))
        }
        FieldKind::I32Array => {
            out.push_str(&format!("{indent}writer.i32_array_value({value})?;\n"))
        }
        FieldKind::U32Array => {
            out.push_str(&format!("{indent}writer.u32_array_value({value})?;\n"))
        }
        FieldKind::F64Array => out.push_str(&format!(
            "{indent}writer.f64_array_value({:?}, {value})?;\n",
            field.logical_path
        )),
        FieldKind::F32Array => out.push_str(&format!(
            "{indent}writer.f32_array_value({:?}, {value})?;\n",
            field.logical_path
        )),
    }
}

fn emit_json_bitmask_helpers(out: &mut String, scope: &EmitScope<'_>) {
    let model = scope.model;
    for dictionary in &model.dictionaries {
        if !dictionary_is_bitmask(model, &dictionary.name) {
            continue;
        }
        let helper = scope.fn_name(&format!(
            "write_json_{}_bitmask",
            const_name(&dictionary.name).to_ascii_lowercase()
        ));
        out.push_str(&format!(
            "fn {helper}(mask: u64, writer: &mut JsonWriter) -> Result<()> {{\n"
        ));
        out.push_str("    writer.begin_array()?;\n");
        out.push_str("    let mut first = true;\n");
        for (idx, value) in dictionary.values.iter().enumerate() {
            out.push_str(&format!("    if mask & (1_u64 << {idx}) != 0 {{\n"));
            out.push_str("        if first { first = false; } else { writer.comma()?; }\n");
            out.push_str(&format!("        writer.string_value({value:?})?;\n"));
            out.push_str("    }\n");
        }
        out.push_str("    let _ = first;\n");
        out.push_str("    writer.end_array()\n");
        out.push_str("}\n\n");
    }
}

fn emit_json_field_prefix_helper(out: &mut String, scope: &EmitScope<'_>) {
    out.push_str(&format!(
        "fn {}(writer: &mut JsonWriter, first: &mut bool, field: &'static [u8]) -> Result<()> {{\n",
        scope.fn_name("write_json_field_prefix")
    ));
    out.push_str("    if *first { *first = false; } else { writer.comma()?; }\n");
    out.push_str("    writer.raw_static(field)\n");
    out.push_str("}\n\n");
}

fn emit_protobuf_inherent_api(out: &mut String, scope: &EmitScope<'_>) {
    let model = scope.model;
    let response_fn = scope.fn_name("write_protobuf_response");
    out.push_str(&format!("impl {} {{\n", model.marker_type));
    out.push_str("    pub fn metamorphose_protobuf(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>> {\n");
    out.push_str("        let archived = Self::access_archived(bytes)?;\n");
    out.push_str(&format!(
        "        {response_fn}(archived, max_response_bytes)\n"
    ));
    out.push_str("    }\n\n");
    out.push_str(
        "    /// Metamorphoses immutable bytes already validated for this schema into protobuf.\n",
    );
    out.push_str("    ///\n");
    out.push_str("    /// # Safety\n");
    out.push_str("    /// The caller guarantees checked schema validation happened before immutable storage or transport.\n");
    out.push_str("    pub unsafe fn metamorphose_protobuf_trusted_unchecked(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>> {\n");
    out.push_str(
        "        let archived = unsafe { Self::access_archived_trusted_unchecked(bytes)? };\n",
    );
    out.push_str(&format!(
        "        {response_fn}(archived, max_response_bytes)\n"
    ));
    out.push_str("    }\n");
    out.push_str("}\n\n");
}

fn emit_protobuf_writer_helpers(out: &mut String, scope: &EmitScope<'_>) {
    let model = scope.model;
    let root = &model.protobuf_messages[0];
    let root_len_fn = scoped_protobuf_len_fn_name(scope, root);
    let root_write_fn = scoped_protobuf_write_fn_name(scope, root);
    let response_fn = scope.fn_name("write_protobuf_response");
    out.push_str(&format!(
        "fn {response_fn}(archived: &Archived{}, max_response_bytes: usize) -> Result<Vec<u8>> {{\n",
        model.payload_type
    ));
    out.push_str("    let mut writer = ProtoWriter::with_capacity(max_response_bytes, max_response_bytes.min(4096));\n");
    out.push_str(&format!(
        "    writer.uint32(1, u32::from({}))?;\n",
        "SCHEMA_VERSION_VALUE"
    ));
    out.push_str(&format!(
        "    for row in archived.{}.iter() {{\n",
        model.row_field_name
    ));
    out.push_str(&format!(
        "        let row_len = {root_len_fn}(row, max_response_bytes)?;\n"
    ));
    out.push_str(&format!(
        "        writer.message_prefix({}, row_len)?;\n",
        model.row_field_number
    ));
    out.push_str(&format!("        {root_write_fn}(row, &mut writer)?;\n"));
    out.push_str("    }\n");
    out.push_str("    Ok(writer.finish())\n");
    out.push_str("}\n\n");

    for message in &model.protobuf_messages {
        emit_protobuf_message_len_helper(out, scope, message);
        emit_protobuf_message_write_helper(out, scope, message);
    }
}

fn emit_protobuf_message_len_helper(
    out: &mut String,
    scope: &EmitScope<'_>,
    message: &ProtobufMessageModel,
) {
    let model = scope.model;
    let len_fn = scoped_protobuf_len_fn_name(scope, message);
    out.push_str(&format!(
        "fn {len_fn}(row: &<{} as Archive>::Archived, max_response_bytes: usize) -> Result<usize> {{\n",
        model.row_type
    ));
    out.push_str("    let mut len = 0_usize;\n");
    for output in &message.fields {
        if let Some(condition) = protobuf_output_optional_condition(scope, output) {
            out.push_str(&format!("    if {condition} {{\n"));
            emit_protobuf_output_len_line(out, scope, output, "        ");
            out.push_str("    }\n");
        } else {
            emit_protobuf_output_len_line(out, scope, output, "    ");
        }
    }
    out.push_str("    Ok(len)\n");
    out.push_str("}\n\n");
}

fn emit_protobuf_message_write_helper(
    out: &mut String,
    scope: &EmitScope<'_>,
    message: &ProtobufMessageModel,
) {
    let model = scope.model;
    let write_fn = scoped_protobuf_write_fn_name(scope, message);
    out.push_str(&format!(
        "fn {write_fn}(row: &<{} as Archive>::Archived, writer: &mut ProtoWriter) -> Result<()> {{\n",
        model.row_type
    ));
    for output in &message.fields {
        if let Some(condition) = protobuf_output_optional_condition(scope, output) {
            out.push_str(&format!("    if {condition} {{\n"));
            emit_protobuf_output_write_line(out, scope, output, "        ");
            out.push_str("    }\n");
        } else {
            emit_protobuf_output_write_line(out, scope, output, "    ");
        }
    }
    out.push_str("    Ok(())\n");
    out.push_str("}\n\n");
}

fn emit_protobuf_output_len_line(
    out: &mut String,
    scope: &EmitScope<'_>,
    output: &ProtobufOutputField,
    indent: &str,
) {
    let model = scope.model;
    match output {
        ProtobufOutputField::Physical { field_index } => {
            emit_protobuf_len_line(out, model, &model.fields[*field_index], indent);
        }
        ProtobufOutputField::DerivedUtc { derived_index } => {
            let field = &model.derived_utc_fields[*derived_index];
            out.push_str(&format!(
                "{indent}len = output::checked_len_add(len, output::encoded_len_message({}, output::utc_len(row.{}.to_native())?), max_response_bytes)?;\n",
                field.proto_number, field.source_rust_name
            ));
        }
        ProtobufOutputField::Message { message_index } => {
            let child = &model.protobuf_messages[*message_index];
            if let Some(tag) = child.enclosing_proto_number {
                let child_len_fn = scoped_protobuf_len_fn_name(scope, child);
                out.push_str(&format!(
                    "{indent}let message_len = {child_len_fn}(row, max_response_bytes)?;\n"
                ));
                out.push_str(&format!(
                    "{indent}if message_len > 0 {{ len = output::checked_len_add(len, output::encoded_len_message({tag}, message_len), max_response_bytes)?; }}\n"
                ));
            }
        }
    }
}

fn emit_protobuf_output_write_line(
    out: &mut String,
    scope: &EmitScope<'_>,
    output: &ProtobufOutputField,
    indent: &str,
) {
    let model = scope.model;
    match output {
        ProtobufOutputField::Physical { field_index } => {
            emit_protobuf_write_line(out, model, &model.fields[*field_index], indent);
        }
        ProtobufOutputField::DerivedUtc { derived_index } => {
            let field = &model.derived_utc_fields[*derived_index];
            out.push_str(&format!(
                "{indent}writer.utc({}, row.{}.to_native())?;\n",
                field.proto_number, field.source_rust_name
            ));
        }
        ProtobufOutputField::Message { message_index } => {
            let child = &model.protobuf_messages[*message_index];
            if let Some(tag) = child.enclosing_proto_number {
                let child_len_fn = scoped_protobuf_len_fn_name(scope, child);
                let child_write_fn = scoped_protobuf_write_fn_name(scope, child);
                out.push_str(&format!(
                    "{indent}let message_len = {child_len_fn}(row, usize::MAX)?;\n"
                ));
                out.push_str(&format!(
                    "{indent}if message_len > 0 {{ writer.message_prefix({tag}, message_len)?; {child_write_fn}(row, writer)?; }}\n"
                ));
            }
        }
    }
}

fn emit_protobuf_len_line(
    out: &mut String,
    model: &SchemaModel,
    field: &PhysicalField,
    indent: &str,
) {
    let value = archived_value_access_for("row", field);
    let tag = field.proto_number;
    match &field.kind {
        FieldKind::ConstU16 { value } => out.push_str(&format!(
            "{indent}len = output::checked_len_add(len, proto::encoded_len_uint32({tag}, u32::from({value}_u16)), max_response_bytes)?;\n"
        )),
        FieldKind::U16Dictionary { dictionary, .. } => out.push_str(&format!(
            "{indent}len = output::checked_len_add(len, output::encoded_len_string({tag}, {}_symbol(row.{}.to_native())?), max_response_bytes)?;\n",
            dictionary_helper_stem(model, dictionary),
            field.rust_name
        )),
        FieldKind::U64BitmaskDictionary { dictionary } => {
            if let Some(dict) = model.dictionaries.iter().find(|dict| dict.name == *dictionary) {
                for (idx, value) in dict.values.iter().enumerate() {
                    out.push_str(&format!(
                        "{indent}if row.{}.to_native() & (1_u64 << {idx}) != 0 {{ len = output::checked_len_add(len, output::encoded_len_string({tag}, {value:?}), max_response_bytes)?; }}\n",
                        field.rust_name
                    ));
                }
            }
        }
        FieldKind::I32 => out.push_str(&format!(
            "{indent}len = output::checked_len_add(len, proto::encoded_len_int32({tag}, {value}), max_response_bytes)?;\n"
        )),
        FieldKind::U32 => out.push_str(&format!(
            "{indent}len = output::checked_len_add(len, proto::encoded_len_uint32({tag}, {value}), max_response_bytes)?;\n"
        )),
        FieldKind::I64 => out.push_str(&format!(
            "{indent}len = output::checked_len_add(len, proto::encoded_len_int64({tag}, {value}), max_response_bytes)?;\n"
        )),
        FieldKind::F32 => out.push_str(&format!(
            "{indent}len = output::checked_len_add(len, proto::encoded_len_float({tag}, {value}), max_response_bytes)?;\n"
        )),
        FieldKind::F64 => out.push_str(&format!(
            "{indent}len = output::checked_len_add(len, proto::encoded_len_double({tag}, {value}), max_response_bytes)?;\n"
        )),
        FieldKind::Bool => out.push_str(&format!(
            "{indent}len = output::checked_len_add(len, proto::encoded_len_bool({tag}, {value}), max_response_bytes)?;\n"
        )),
        FieldKind::Bytes => out.push_str(&format!(
            "{indent}len = output::checked_len_add(len, output::encoded_len_message({tag}, {value}.len()), max_response_bytes)?;\n"
        )),
        FieldKind::RawString => out.push_str(&format!(
            "{indent}len = output::checked_len_add(len, output::encoded_len_string({tag}, {value}), max_response_bytes)?;\n"
        )),
        FieldKind::I64Array => out.push_str(&format!(
            "{indent}for value in {value} {{ len = output::checked_len_add(len, proto::encoded_len_int64({tag}, value), max_response_bytes)?; }}\n"
        )),
        FieldKind::I32Array => out.push_str(&format!(
            "{indent}for value in {value} {{ len = output::checked_len_add(len, proto::encoded_len_int32({tag}, value), max_response_bytes)?; }}\n"
        )),
        FieldKind::U32Array => out.push_str(&format!(
            "{indent}for value in {value} {{ len = output::checked_len_add(len, proto::encoded_len_uint32({tag}, value), max_response_bytes)?; }}\n"
        )),
        FieldKind::F64Array => out.push_str(&format!(
            "{indent}for value in {value} {{ len = output::checked_len_add(len, proto::encoded_len_double({tag}, value), max_response_bytes)?; }}\n"
        )),
        FieldKind::F32Array => out.push_str(&format!(
            "{indent}for value in {value} {{ len = output::checked_len_add(len, proto::encoded_len_float({tag}, value), max_response_bytes)?; }}\n"
        )),
    }
}

fn emit_protobuf_write_line(
    out: &mut String,
    model: &SchemaModel,
    field: &PhysicalField,
    indent: &str,
) {
    let value = archived_value_access_for("row", field);
    let tag = field.proto_number;
    match &field.kind {
        FieldKind::ConstU16 { value } => out.push_str(&format!(
            "{indent}writer.uint32({tag}, u32::from({value}_u16))?;\n"
        )),
        FieldKind::U16Dictionary { dictionary, .. } => out.push_str(&format!(
            "{indent}writer.string({tag}, {}_symbol(row.{}.to_native())?)?;\n",
            dictionary_helper_stem(model, dictionary),
            field.rust_name
        )),
        FieldKind::U64BitmaskDictionary { dictionary } => {
            if let Some(dict) = model
                .dictionaries
                .iter()
                .find(|dict| dict.name == *dictionary)
            {
                for (idx, value) in dict.values.iter().enumerate() {
                    out.push_str(&format!(
                        "{indent}if row.{}.to_native() & (1_u64 << {idx}) != 0 {{ writer.string({tag}, {value:?})?; }}\n",
                        field.rust_name
                    ));
                }
            }
        }
        FieldKind::I32 => out.push_str(&format!("{indent}writer.int32({tag}, {value})?;\n")),
        FieldKind::U32 => out.push_str(&format!("{indent}writer.uint32({tag}, {value})?;\n")),
        FieldKind::I64 => out.push_str(&format!("{indent}writer.int64({tag}, {value})?;\n")),
        FieldKind::F32 => out.push_str(&format!(
            "{indent}writer.float({tag}, {:?}, {value})?;\n",
            field.logical_path
        )),
        FieldKind::F64 => out.push_str(&format!(
            "{indent}writer.double({tag}, {:?}, {value})?;\n",
            field.logical_path
        )),
        FieldKind::Bool => out.push_str(&format!("{indent}writer.bool({tag}, {value})?;\n")),
        FieldKind::Bytes => out.push_str(&format!("{indent}writer.bytes({tag}, {value})?;\n")),
        FieldKind::RawString => out.push_str(&format!("{indent}writer.string({tag}, {value})?;\n")),
        FieldKind::I64Array => out.push_str(&format!(
            "{indent}for value in {value} {{ writer.int64({tag}, value)?; }}\n"
        )),
        FieldKind::I32Array => out.push_str(&format!(
            "{indent}for value in {value} {{ writer.int32({tag}, value)?; }}\n"
        )),
        FieldKind::U32Array => out.push_str(&format!(
            "{indent}for value in {value} {{ writer.uint32({tag}, value)?; }}\n"
        )),
        FieldKind::F64Array => out.push_str(&format!(
            "{indent}for value in {value} {{ writer.double({tag}, {:?}, value)?; }}\n",
            field.logical_path
        )),
        FieldKind::F32Array => out.push_str(&format!(
            "{indent}for value in {value} {{ writer.float({tag}, {:?}, value)?; }}\n",
            field.logical_path
        )),
    }
}

fn emit_csv_header(out: &mut String, scope: &EmitScope<'_>) {
    let model = scope.model;
    let header = model
        .json_csv_output_fields
        .iter()
        .map(|output| csv_output_field_name(model, output))
        .collect::<Vec<_>>()
        .join(",");
    out.push_str(&format!(
        "const {}: &[u8] = b{header:?};\n\n",
        scope.const_name("CSV_HEADER")
    ));
}

fn emit_csv_inherent_api(out: &mut String, scope: &EmitScope<'_>) {
    let model = scope.model;
    let response_fn = scope.fn_name("write_csv_response");
    out.push_str(&format!("impl {} {{\n", model.marker_type));
    out.push_str("    pub fn metamorphose_csv(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>> {\n");
    out.push_str("        let archived = Self::access_archived(bytes)?;\n");
    out.push_str(&format!(
        "        {response_fn}(archived, max_response_bytes)\n"
    ));
    out.push_str("    }\n\n");
    out.push_str(
        "    /// Metamorphoses immutable bytes already validated for this schema into CSV.\n",
    );
    out.push_str("    ///\n");
    out.push_str("    /// # Safety\n");
    out.push_str("    /// The caller guarantees checked schema validation happened before immutable storage or transport.\n");
    out.push_str("    pub unsafe fn metamorphose_csv_trusted_unchecked(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>> {\n");
    out.push_str(
        "        let archived = unsafe { Self::access_archived_trusted_unchecked(bytes)? };\n",
    );
    out.push_str(&format!(
        "        {response_fn}(archived, max_response_bytes)\n"
    ));
    out.push_str("    }\n");
    out.push_str("}\n\n");
}

fn emit_csv_writer_helpers(out: &mut String, scope: &EmitScope<'_>) {
    let model = scope.model;
    let response_fn = scope.fn_name("write_csv_response");
    let row_fn = scope.fn_name("write_csv_row");
    out.push_str(&format!(
        "fn {response_fn}(archived: &Archived{}, max_response_bytes: usize) -> Result<Vec<u8>> {{\n",
        model.payload_type
    ));
    out.push_str("    let mut writer = CsvWriter::with_capacity(max_response_bytes, max_response_bytes.min(4096));\n");
    out.push_str(&format!(
        "    writer.raw_static({})?;\n",
        scope.const_name("CSV_HEADER")
    ));
    out.push_str("    writer.newline()?;\n");
    out.push_str(&format!(
        "    for row in archived.{}.iter() {{\n",
        model.row_field_name
    ));
    out.push_str(&format!("        {row_fn}(row, &mut writer)?;\n"));
    out.push_str("    }\n");
    out.push_str("    Ok(writer.finish())\n");
    out.push_str("}\n\n");

    out.push_str(&format!(
        "fn {row_fn}(row: &<{} as Archive>::Archived, writer: &mut CsvWriter) -> Result<()> {{\n",
        model.row_type
    ));
    for (idx, output) in model.json_csv_output_fields.iter().enumerate() {
        if idx > 0 {
            out.push_str("    writer.comma()?;\n");
        }
        if let Some(condition) = json_output_optional_condition(scope, output) {
            out.push_str(&format!("    if {condition} {{\n"));
            emit_csv_output_write(out, scope, output, "        ");
            out.push_str("    }\n");
        } else {
            emit_csv_output_write(out, scope, output, "    ");
        }
    }
    out.push_str("    writer.newline()\n");
    out.push_str("}\n\n");
    emit_csv_bitmask_helpers(out, scope);
}

fn emit_csv_value_write(
    out: &mut String,
    scope: &EmitScope<'_>,
    field: &PhysicalField,
    indent: &str,
) {
    let model = scope.model;
    let value = archived_value_access_for("row", field);
    match &field.kind {
        FieldKind::ConstU16 { value } => out.push_str(&format!(
            "{indent}writer.u32_cell(u32::from({value}_u16))?;\n"
        )),
        FieldKind::U16Dictionary { dictionary, .. } => out.push_str(&format!(
            "{indent}writer.string_cell({}_symbol(row.{}.to_native())?)?;\n",
            dictionary_helper_stem(model, dictionary),
            field.rust_name
        )),
        FieldKind::U64BitmaskDictionary { dictionary } => out.push_str(&format!(
            "{indent}{}(row.{}.to_native(), writer)?;\n",
            scope.fn_name(&format!(
                "write_csv_{}_bitmask",
                const_name(dictionary).to_ascii_lowercase()
            )),
            field.rust_name
        )),
        FieldKind::I32 => out.push_str(&format!("{indent}writer.i32_cell({value})?;\n")),
        FieldKind::U32 => out.push_str(&format!("{indent}writer.u32_cell({value})?;\n")),
        FieldKind::I64 => out.push_str(&format!("{indent}writer.i64_cell({value})?;\n")),
        FieldKind::F32 => out.push_str(&format!(
            "{indent}writer.f32_cell({:?}, {value})?;\n",
            field.logical_path
        )),
        FieldKind::F64 => out.push_str(&format!(
            "{indent}writer.f64_cell({:?}, {value})?;\n",
            field.logical_path
        )),
        FieldKind::Bool => out.push_str(&format!("{indent}writer.bool_cell({value})?;\n")),
        FieldKind::Bytes => out.push_str(&format!("{indent}writer.bytes_cell({value})?;\n")),
        FieldKind::RawString => out.push_str(&format!("{indent}writer.string_cell({value})?;\n")),
        FieldKind::I64Array => out.push_str(&format!("{indent}writer.i64_array_cell({value})?;\n")),
        FieldKind::I32Array => out.push_str(&format!("{indent}writer.i32_array_cell({value})?;\n")),
        FieldKind::U32Array => out.push_str(&format!("{indent}writer.u32_array_cell({value})?;\n")),
        FieldKind::F64Array => out.push_str(&format!(
            "{indent}writer.f64_array_cell({:?}, {value})?;\n",
            field.logical_path
        )),
        FieldKind::F32Array => out.push_str(&format!(
            "{indent}writer.f32_array_cell({:?}, {value})?;\n",
            field.logical_path
        )),
    }
}

fn emit_csv_output_write(
    out: &mut String,
    scope: &EmitScope<'_>,
    output: &JsonCsvOutputField,
    indent: &str,
) {
    let model = scope.model;
    match output {
        JsonCsvOutputField::Physical { field_index } => {
            emit_csv_value_write(out, scope, &model.fields[*field_index], indent);
        }
        JsonCsvOutputField::DerivedUtc { derived_index } => {
            let field = &model.derived_utc_fields[*derived_index];
            out.push_str(&format!(
                "{indent}writer.utc_cell(row.{}.to_native())?;\n",
                field.source_rust_name
            ));
        }
    }
}

fn emit_csv_bitmask_helpers(out: &mut String, scope: &EmitScope<'_>) {
    let model = scope.model;
    for dictionary in &model.dictionaries {
        if !dictionary_is_bitmask(model, &dictionary.name) {
            continue;
        }
        let helper = scope.fn_name(&format!(
            "write_csv_{}_bitmask",
            const_name(&dictionary.name).to_ascii_lowercase()
        ));
        out.push_str(&format!(
            "fn {helper}(mask: u64, writer: &mut CsvWriter) -> Result<()> {{\n"
        ));
        out.push_str("    writer.begin_array_cell()?;\n");
        out.push_str("    let mut first = true;\n");
        for (idx, value) in dictionary.values.iter().enumerate() {
            out.push_str(&format!("    if mask & (1_u64 << {idx}) != 0 {{\n"));
            out.push_str(
                "        if first { first = false; } else { writer.array_cell_comma()?; }\n",
            );
            out.push_str(&format!("        writer.string_cell({value:?})?;\n"));
            out.push_str("    }\n");
        }
        out.push_str("    let _ = first;\n");
        out.push_str("    writer.end_array_cell()\n");
        out.push_str("}\n\n");
    }
}

fn emit_metamorphose_transponding(out: &mut String, model: &SchemaModel) -> Result<()> {
    // Transponding is generated only for adapters that need columnar batches.
    emit_adapter_prelude(out, model, false);
    out.push_str("use mbt_transponding::*;\n\n");
    emit_adapter_sections(out, model, emit_transponding_adapter_section)
}

fn emit_transponding_adapter_section(out: &mut String, scope: &EmitScope<'_>) -> Result<()> {
    emit_projection_adapter_presence_constants(out, scope)?;
    emit_column_batch(out, scope.model);
    emit_transponding_inherent_api(out, scope);
    Ok(())
}

fn emit_metamorphose_arrow(out: &mut String, model: &SchemaModel) -> Result<()> {
    emit_adapter_prelude(out, model, false);
    out.push_str(&format!("use crate::{}_transponding::*;\n", model.module));
    out.push_str("use mbt_adapter_arrow::{\n");
    out.push_str("    field_metadata, record_batch, ArrowArrayRef, ArrowDataType, ArrowField, ArrowRecordBatch, ArrowSchema,\n");
    out.push_str("};\n");
    out.push_str("use mbt_metamorphose::{runtime::TrustedUnchecked, ArrowMetamorphoseSchema};\n");
    out.push_str("use std::sync::Arc;\n\n");
    emit_adapter_sections(out, model, emit_arrow_adapter_section)
}

fn emit_arrow_adapter_section(out: &mut String, scope: &EmitScope<'_>) -> Result<()> {
    let model = scope.model;
    emit_arrow_inherent_api(out, scope);
    out.push_str(&format!(
        "impl ArrowMetamorphoseSchema for {} {{\n",
        model.marker_type
    ));
    out.push_str("    type RecordBatch = ArrowRecordBatch;\n");
    out.push_str("    fn metamorphose_arrow(bytes: &[u8], max_response_bytes: usize) -> Result<Self::RecordBatch> { Self::metamorphose_arrow(bytes, max_response_bytes) }\n");
    out.push_str("    fn metamorphose_arrow_trusted_unchecked(bytes: &[u8], max_response_bytes: usize, _trusted: TrustedUnchecked) -> Result<Self::RecordBatch> {\n");
    out.push_str("        unsafe { Self::metamorphose_arrow_trusted_unchecked(bytes, max_response_bytes) }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
    emit_arrow_record_batch_helper(out, scope, "mbt_adapter_arrow");
    Ok(())
}

fn emit_metamorphose_arrow_ipc(out: &mut String, model: &SchemaModel) -> Result<()> {
    emit_adapter_prelude(out, model, false);
    out.push_str(&format!("use crate::{}_transponding::*;\n", model.module));
    out.push_str("use mbt_adapter_arrow_ipc::{\n");
    out.push_str("    field_metadata, record_batch, write_ipc_stream, ArrowArrayRef, ArrowDataType, ArrowField, ArrowRecordBatch, ArrowSchema,\n");
    out.push_str("};\n");
    out.push_str(
        "use mbt_metamorphose::{runtime::TrustedUnchecked, ArrowIpcMetamorphoseSchema};\n\n",
    );
    out.push_str("use std::sync::Arc;\n\n");
    emit_adapter_sections(out, model, emit_arrow_ipc_adapter_section)
}

fn emit_arrow_ipc_adapter_section(out: &mut String, scope: &EmitScope<'_>) -> Result<()> {
    let model = scope.model;
    emit_arrow_ipc_inherent_api(out, scope);
    out.push_str(&format!(
        "impl ArrowIpcMetamorphoseSchema for {} {{\n",
        model.marker_type
    ));
    out.push_str("    fn metamorphose_arrow_ipc(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>> { Self::metamorphose_arrow_ipc(bytes, max_response_bytes) }\n");
    out.push_str("    fn metamorphose_arrow_ipc_trusted_unchecked(bytes: &[u8], max_response_bytes: usize, _trusted: TrustedUnchecked) -> Result<Vec<u8>> {\n");
    out.push_str("        unsafe { Self::metamorphose_arrow_ipc_trusted_unchecked(bytes, max_response_bytes) }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
    emit_arrow_record_batch_helper(out, scope, "mbt_adapter_arrow_ipc");
    Ok(())
}

fn emit_metamorphose_parquet(out: &mut String, model: &SchemaModel) -> Result<()> {
    emit_adapter_prelude(out, model, false);
    out.push_str(&format!("use crate::{}_transponding::*;\n", model.module));
    out.push_str("use mbt_adapter_parquet::{\n");
    out.push_str("    field_metadata, record_batch, write_uncompressed_parquet, ArrowArrayRef, ArrowDataType, ArrowField, ArrowRecordBatch, ArrowSchema,\n");
    out.push_str("};\n");
    out.push_str(
        "use mbt_metamorphose::{runtime::TrustedUnchecked, ParquetMetamorphoseSchema};\n\n",
    );
    out.push_str("use std::sync::Arc;\n\n");
    emit_adapter_sections(out, model, emit_parquet_adapter_section)
}

fn emit_parquet_adapter_section(out: &mut String, scope: &EmitScope<'_>) -> Result<()> {
    let model = scope.model;
    emit_parquet_inherent_api(out, scope);
    out.push_str(&format!(
        "impl ParquetMetamorphoseSchema for {} {{\n",
        model.marker_type
    ));
    out.push_str("    fn metamorphose_parquet(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>> { Self::metamorphose_parquet(bytes, max_response_bytes) }\n");
    out.push_str("    fn metamorphose_parquet_trusted_unchecked(bytes: &[u8], max_response_bytes: usize, _trusted: TrustedUnchecked) -> Result<Vec<u8>> {\n");
    out.push_str("        unsafe { Self::metamorphose_parquet_trusted_unchecked(bytes, max_response_bytes) }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
    emit_arrow_record_batch_helper(out, scope, "mbt_adapter_parquet");
    Ok(())
}

fn emit_column_batch(out: &mut String, model: &SchemaModel) {
    // Column batches are schema-specific buffers consumed by columnar adapters.
    out.push_str(&format!(
        "pub(crate) struct {}ColumnBatch {{\n",
        model.marker_type
    ));
    for field in &model.fields {
        out.push_str(&format!(
            "    pub(crate) {}: {},\n",
            field.rust_name,
            column_type(field)
        ));
    }
    out.push_str("}\n\n");
    out.push_str(&format!("impl {}ColumnBatch {{\n", model.marker_type));
    out.push_str("    pub(crate) fn byte_len(&self) -> usize {\n");
    out.push_str("        let mut len = 0_usize;\n");
    for field in &model.fields {
        out.push_str(&format!(
            "        len = len.saturating_add(self.{}.byte_len());\n",
            field.rust_name
        ));
    }
    out.push_str("        len\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
}

fn emit_transponding_inherent_api(out: &mut String, scope: &EmitScope<'_>) {
    let model = scope.model;
    out.push_str(&format!("impl {} {{\n", model.marker_type));
    out.push_str(&format!(
        "    pub(crate) fn transpond_archived(archived: &Archived{}, max_columnar_bytes: usize) -> Result<{}ColumnBatch> {{\n",
        model.payload_type, model.marker_type
    ));
    out.push_str(&format!(
        "        let row_count = archived.{}.len();\n",
        model.row_field_name
    ));
    for field in &model.fields {
        out.push_str(&format!(
            "        let {}{} = {};\n",
            if matches!(field.kind, FieldKind::ConstU16 { .. }) {
                ""
            } else {
                "mut "
            },
            field.rust_name,
            column_init(field, "row_count")
        ));
    }
    out.push_str(&format!(
        "        for row in archived.{}.iter() {{\n",
        model.row_field_name
    ));
    for field in &model.fields {
        emit_column_push(out, scope, field, "            ");
    }
    out.push_str("        }\n");
    out.push_str(&format!(
        "        let batch = {}ColumnBatch {{",
        model.marker_type
    ));
    for field in &model.fields {
        out.push_str(&format!(" {}", field.rust_name));
        out.push(',');
    }
    out.push_str(" };\n");
    out.push_str("        ensure_columnar_size(batch.byte_len(), max_columnar_bytes)?;\n");
    out.push_str("        Ok(batch)\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
}

fn emit_arrow_inherent_api(out: &mut String, scope: &EmitScope<'_>) {
    let model = scope.model;
    let record_batch_fn = scope.fn_name("arrow_record_batch");
    out.push_str(&format!("impl {} {{\n", model.marker_type));
    out.push_str("    pub fn metamorphose_arrow(bytes: &[u8], max_response_bytes: usize) -> Result<ArrowRecordBatch> {\n");
    out.push_str("        let archived = Self::access_archived(bytes)?;\n");
    out.push_str("        let batch = Self::transpond_archived(archived, max_response_bytes)?;\n");
    out.push_str(&format!(
        "        {record_batch_fn}(batch, max_response_bytes)\n"
    ));
    out.push_str("    }\n\n");
    out.push_str(
        "    /// Metamorphoses immutable bytes already validated for this schema into Arrow.\n",
    );
    out.push_str("    ///\n");
    out.push_str("    /// # Safety\n");
    out.push_str("    /// The caller guarantees checked schema validation happened before immutable storage or transport.\n");
    out.push_str("    pub unsafe fn metamorphose_arrow_trusted_unchecked(bytes: &[u8], max_response_bytes: usize) -> Result<ArrowRecordBatch> {\n");
    out.push_str(
        "        let archived = unsafe { Self::access_archived_trusted_unchecked(bytes)? };\n",
    );
    out.push_str("        let batch = Self::transpond_archived(archived, max_response_bytes)?;\n");
    out.push_str(&format!(
        "        {record_batch_fn}(batch, max_response_bytes)\n"
    ));
    out.push_str("    }\n");
    out.push_str("}\n\n");
}

fn emit_arrow_record_batch_helper(out: &mut String, scope: &EmitScope<'_>, adapter_crate: &str) {
    let model = scope.model;
    let record_batch_fn = scope.fn_name("arrow_record_batch");
    out.push_str(&format!(
        "fn {record_batch_fn}(batch: {}ColumnBatch, max_response_bytes: usize) -> Result<ArrowRecordBatch> {{\n",
        model.marker_type
    ));
    out.push_str("    let schema = Arc::new(ArrowSchema::new(vec![\n");
    for field in &model.fields {
        out.push_str(&format!(
            "        ArrowField::new({:?}, {}, {}).with_metadata(field_metadata({:?}, {:?}, {}, {})),\n",
            csv_field_name(field),
            arrow_data_type(field),
            field.presence_bit.is_some(),
            field.logical_path,
            field.rust_name,
            dictionary_metadata_expr(field),
            bitmask_metadata_expr(field)
        ));
    }
    out.push_str("    ]));\n");
    out.push_str("    let columns: Vec<ArrowArrayRef> = vec![\n");
    for field in &model.fields {
        out.push_str(&format!(
            "        {}::{}(batch.{})?,\n",
            adapter_crate,
            arrow_array_fn(field),
            field.rust_name
        ));
    }
    out.push_str("    ];\n");
    out.push_str("    record_batch(schema, columns, max_response_bytes)\n");
    out.push_str("}\n\n");
}

fn emit_arrow_ipc_inherent_api(out: &mut String, scope: &EmitScope<'_>) {
    let model = scope.model;
    let record_batch_fn = scope.fn_name("arrow_record_batch");
    out.push_str(&format!("impl {} {{\n", model.marker_type));
    out.push_str("    pub fn metamorphose_arrow_ipc(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>> {\n");
    out.push_str("        let archived = Self::access_archived(bytes)?;\n");
    out.push_str("        let batch = Self::transpond_archived(archived, max_response_bytes)?;\n");
    out.push_str(&format!(
        "        let arrow = {record_batch_fn}(batch, max_response_bytes)?;\n"
    ));
    out.push_str("        write_ipc_stream(&arrow, max_response_bytes)\n");
    out.push_str("    }\n\n");
    out.push_str(
        "    /// Metamorphoses immutable bytes already validated for this schema into Arrow IPC.\n",
    );
    out.push_str("    ///\n");
    out.push_str("    /// # Safety\n");
    out.push_str("    /// The caller guarantees checked schema validation happened before immutable storage or transport.\n");
    out.push_str("    pub unsafe fn metamorphose_arrow_ipc_trusted_unchecked(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>> {\n");
    out.push_str(
        "        let archived = unsafe { Self::access_archived_trusted_unchecked(bytes)? };\n",
    );
    out.push_str("        let batch = Self::transpond_archived(archived, max_response_bytes)?;\n");
    out.push_str(&format!(
        "        let arrow = {record_batch_fn}(batch, max_response_bytes)?;\n"
    ));
    out.push_str("        write_ipc_stream(&arrow, max_response_bytes)\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
}

fn emit_parquet_inherent_api(out: &mut String, scope: &EmitScope<'_>) {
    let model = scope.model;
    let record_batch_fn = scope.fn_name("arrow_record_batch");
    out.push_str(&format!("impl {} {{\n", model.marker_type));
    out.push_str("    pub fn metamorphose_parquet(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>> {\n");
    out.push_str("        let archived = Self::access_archived(bytes)?;\n");
    out.push_str("        let batch = Self::transpond_archived(archived, max_response_bytes)?;\n");
    out.push_str(&format!(
        "        let arrow = {record_batch_fn}(batch, max_response_bytes)?;\n"
    ));
    out.push_str("        write_uncompressed_parquet(&arrow, max_response_bytes)\n");
    out.push_str("    }\n\n");
    out.push_str(
        "    /// Metamorphoses immutable bytes already validated for this schema into Parquet.\n",
    );
    out.push_str("    ///\n");
    out.push_str("    /// # Safety\n");
    out.push_str("    /// The caller guarantees checked schema validation happened before immutable storage or transport.\n");
    out.push_str("    pub unsafe fn metamorphose_parquet_trusted_unchecked(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>> {\n");
    out.push_str(
        "        let archived = unsafe { Self::access_archived_trusted_unchecked(bytes)? };\n",
    );
    out.push_str("        let batch = Self::transpond_archived(archived, max_response_bytes)?;\n");
    out.push_str(&format!(
        "        let arrow = {record_batch_fn}(batch, max_response_bytes)?;\n"
    ));
    out.push_str("        write_uncompressed_parquet(&arrow, max_response_bytes)\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
}

fn archived_optional_condition(scope: &EmitScope<'_>, field: &PhysicalField) -> Option<String> {
    field
        .presence_bit
        .map(|_| archived_presence_has_expr(scope, "row.presence_bits", field))
}

fn json_output_optional_condition(
    scope: &EmitScope<'_>,
    output: &JsonCsvOutputField,
) -> Option<String> {
    let model = scope.model;
    match output {
        JsonCsvOutputField::Physical { field_index } => {
            archived_optional_condition(scope, &model.fields[*field_index])
        }
        JsonCsvOutputField::DerivedUtc { derived_index } => {
            derived_utc_optional_condition(scope, &model.derived_utc_fields[*derived_index])
        }
    }
}

fn protobuf_output_optional_condition(
    scope: &EmitScope<'_>,
    output: &ProtobufOutputField,
) -> Option<String> {
    let model = scope.model;
    match output {
        ProtobufOutputField::Physical { field_index } => {
            archived_optional_condition(scope, &model.fields[*field_index])
        }
        ProtobufOutputField::DerivedUtc { derived_index } => {
            derived_utc_optional_condition(scope, &model.derived_utc_fields[*derived_index])
        }
        ProtobufOutputField::Message { .. } => None,
    }
}

fn derived_utc_optional_condition(
    scope: &EmitScope<'_>,
    field: &DerivedUtcField,
) -> Option<String> {
    let model = scope.model;
    field.source_presence_bit.map(|_| {
        archived_presence_has_expr(
            scope,
            "row.presence_bits",
            &model.fields[field.source_field_index],
        )
    })
}

fn archived_value_access_for(row: &str, field: &PhysicalField) -> String {
    let access = format!("{row}.{}", field.rust_name);
    match field.kind {
        FieldKind::ConstU16 { .. }
        | FieldKind::U16Dictionary { .. }
        | FieldKind::U64BitmaskDictionary { .. }
        | FieldKind::I32
        | FieldKind::U32
        | FieldKind::I64
        | FieldKind::F32
        | FieldKind::F64 => format!("{access}.to_native()"),
        FieldKind::Bool => access,
        FieldKind::Bytes => format!("{access}.as_slice()"),
        FieldKind::RawString => format!("{access}.as_str()"),
        FieldKind::I64Array
        | FieldKind::I32Array
        | FieldKind::U32Array
        | FieldKind::F64Array
        | FieldKind::F32Array => format!("{access}.iter().map(|value| value.to_native())"),
    }
}

fn archived_to_owned_access_for(row: &str, field: &PhysicalField) -> String {
    let access = format!("{row}.{}", field.rust_name);
    match field.kind {
        FieldKind::ConstU16 { .. }
        | FieldKind::U16Dictionary { .. }
        | FieldKind::U64BitmaskDictionary { .. }
        | FieldKind::I32
        | FieldKind::U32
        | FieldKind::I64
        | FieldKind::F32
        | FieldKind::F64 => format!("{access}.to_native()"),
        FieldKind::Bool => access,
        FieldKind::Bytes => format!("{access}.as_slice().to_vec()"),
        FieldKind::RawString => format!("{access}.as_str().to_string()"),
        FieldKind::I64Array
        | FieldKind::I32Array
        | FieldKind::U32Array
        | FieldKind::F64Array
        | FieldKind::F32Array => {
            format!("{access}.iter().map(|value| value.to_native()).collect()")
        }
    }
}

fn json_field_name(field: &PhysicalField) -> &str {
    &field.logical_path
}

fn json_output_const_name(model: &SchemaModel, output: &JsonCsvOutputField) -> String {
    match output {
        JsonCsvOutputField::Physical { field_index } => {
            const_name(&model.fields[*field_index].rust_name)
        }
        JsonCsvOutputField::DerivedUtc { derived_index } => {
            const_name(&model.derived_utc_fields[*derived_index].rust_name)
        }
    }
}

fn json_output_field_name<'a>(model: &'a SchemaModel, output: &JsonCsvOutputField) -> &'a str {
    match output {
        JsonCsvOutputField::Physical { field_index } => {
            json_field_name(&model.fields[*field_index])
        }
        JsonCsvOutputField::DerivedUtc { derived_index } => {
            &model.derived_utc_fields[*derived_index].logical_path
        }
    }
}

fn csv_field_name(field: &PhysicalField) -> &str {
    &field.logical_path
}

fn csv_output_field_name<'a>(model: &'a SchemaModel, output: &JsonCsvOutputField) -> &'a str {
    match output {
        JsonCsvOutputField::Physical { field_index } => csv_field_name(&model.fields[*field_index]),
        JsonCsvOutputField::DerivedUtc { derived_index } => {
            &model.derived_utc_fields[*derived_index].logical_path
        }
    }
}

fn protobuf_len_fn_name(message: &ProtobufMessageModel) -> String {
    format!("encoded_len_{}", message.rust_helper_stem)
}

fn protobuf_write_fn_name(message: &ProtobufMessageModel) -> String {
    format!("write_protobuf_{}", message.rust_helper_stem)
}

fn scoped_protobuf_len_fn_name(scope: &EmitScope<'_>, message: &ProtobufMessageModel) -> String {
    scope.fn_name(&protobuf_len_fn_name(message))
}

fn scoped_protobuf_write_fn_name(scope: &EmitScope<'_>, message: &ProtobufMessageModel) -> String {
    scope.fn_name(&protobuf_write_fn_name(message))
}

fn column_type(field: &PhysicalField) -> &'static str {
    match field.kind {
        FieldKind::ConstU16 { .. } => "ConstU16Column",
        FieldKind::U16Dictionary { optional: true, .. } => "OptionalU16Column",
        FieldKind::U16Dictionary {
            optional: false, ..
        } => "U16Column",
        FieldKind::U64BitmaskDictionary { .. } => "U64Column",
        FieldKind::I32 if field.presence_bit.is_some() => "OptionalI32Column",
        FieldKind::U32 if field.presence_bit.is_some() => "OptionalU32Column",
        FieldKind::I64 if field.presence_bit.is_some() => "OptionalI64Column",
        FieldKind::F32 if field.presence_bit.is_some() => "OptionalF32Column",
        FieldKind::F64 if field.presence_bit.is_some() => "OptionalF64Column",
        FieldKind::I32 => "I32Column",
        FieldKind::U32 => "U32Column",
        FieldKind::I64 => "I64Column",
        FieldKind::F32 => "F32Column",
        FieldKind::F64 => "F64Column",
        FieldKind::Bool => "BoolColumn",
        FieldKind::Bytes => "BinaryColumn",
        FieldKind::RawString => "Utf8Column",
        FieldKind::I64Array => "I64ListColumn",
        FieldKind::I32Array => "I32ListColumn",
        FieldKind::U32Array => "U32ListColumn",
        FieldKind::F64Array => "F64ListColumn",
        FieldKind::F32Array => "F32ListColumn",
    }
}

fn column_init(field: &PhysicalField, row_count: &str) -> String {
    match field.kind {
        FieldKind::ConstU16 { value } => format!("ConstU16Column::new({value}, {row_count})"),
        FieldKind::U16Dictionary { optional: true, .. } => {
            format!("OptionalU16Column::new({row_count})")
        }
        FieldKind::U16Dictionary {
            optional: false, ..
        } => format!("U16Column::new({row_count})"),
        FieldKind::U64BitmaskDictionary { .. } => format!("U64Column::new({row_count})"),
        FieldKind::I32 if field.presence_bit.is_some() => {
            format!("OptionalI32Column::new({row_count})")
        }
        FieldKind::U32 if field.presence_bit.is_some() => {
            format!("OptionalU32Column::new({row_count})")
        }
        FieldKind::I64 if field.presence_bit.is_some() => {
            format!("OptionalI64Column::new({row_count})")
        }
        FieldKind::F32 if field.presence_bit.is_some() => {
            format!("OptionalF32Column::new({row_count})")
        }
        FieldKind::F64 if field.presence_bit.is_some() => {
            format!("OptionalF64Column::new({row_count})")
        }
        FieldKind::I32 => format!("I32Column::new({row_count})"),
        FieldKind::U32 => format!("U32Column::new({row_count})"),
        FieldKind::I64 => format!("I64Column::new({row_count})"),
        FieldKind::F32 => format!("F32Column::new({row_count})"),
        FieldKind::F64 => format!("F64Column::new({row_count})"),
        FieldKind::Bool if field.presence_bit.is_some() => {
            format!("BoolColumn::optional({row_count})")
        }
        FieldKind::Bool => format!("BoolColumn::required({row_count})"),
        FieldKind::Bytes if field.presence_bit.is_some() => {
            format!("BinaryColumn::optional({row_count})")
        }
        FieldKind::Bytes => format!("BinaryColumn::required({row_count})"),
        FieldKind::RawString if field.presence_bit.is_some() => {
            format!("Utf8Column::optional({row_count})")
        }
        FieldKind::RawString => format!("Utf8Column::required({row_count})"),
        FieldKind::I64Array if field.presence_bit.is_some() => {
            format!("I64ListColumn::optional({row_count})")
        }
        FieldKind::I64Array => format!("I64ListColumn::required({row_count})"),
        FieldKind::I32Array if field.presence_bit.is_some() => {
            format!("I32ListColumn::optional({row_count})")
        }
        FieldKind::I32Array => format!("I32ListColumn::required({row_count})"),
        FieldKind::U32Array if field.presence_bit.is_some() => {
            format!("U32ListColumn::optional({row_count})")
        }
        FieldKind::U32Array => format!("U32ListColumn::required({row_count})"),
        FieldKind::F64Array if field.presence_bit.is_some() => {
            format!("F64ListColumn::optional({row_count})")
        }
        FieldKind::F64Array => format!("F64ListColumn::required({row_count})"),
        FieldKind::F32Array if field.presence_bit.is_some() => {
            format!("F32ListColumn::optional({row_count})")
        }
        FieldKind::F32Array => format!("F32ListColumn::required({row_count})"),
    }
}

fn emit_column_push(out: &mut String, scope: &EmitScope<'_>, field: &PhysicalField, indent: &str) {
    let value = archived_value_access_for("row", field);
    if matches!(field.kind, FieldKind::ConstU16 { .. }) {
        return;
    }
    if field.presence_bit.is_some() {
        let present = archived_presence_has_expr(scope, "row.presence_bits", field);
        out.push_str(&format!(
            "{indent}{}.push_optional({present}, {value})?;\n",
            field.rust_name
        ));
        return;
    }
    match field.kind {
        FieldKind::U16Dictionary { .. }
        | FieldKind::U64BitmaskDictionary { .. }
        | FieldKind::I32
        | FieldKind::U32
        | FieldKind::I64
        | FieldKind::F32
        | FieldKind::F64
        | FieldKind::Bool => {
            out.push_str(&format!(
                "{indent}{}.push_required({value});\n",
                field.rust_name
            ));
        }
        FieldKind::Bytes
        | FieldKind::RawString
        | FieldKind::I64Array
        | FieldKind::I32Array
        | FieldKind::U32Array
        | FieldKind::F64Array
        | FieldKind::F32Array => {
            out.push_str(&format!(
                "{indent}{}.push_required({value})?;\n",
                field.rust_name
            ));
        }
        FieldKind::ConstU16 { .. } => {}
    }
}

fn arrow_data_type(field: &PhysicalField) -> &'static str {
    match field.kind {
        FieldKind::ConstU16 { .. } | FieldKind::U16Dictionary { .. } => "ArrowDataType::UInt16",
        FieldKind::U64BitmaskDictionary { .. } => "ArrowDataType::UInt64",
        FieldKind::I32 => "ArrowDataType::Int32",
        FieldKind::U32 => "ArrowDataType::UInt32",
        FieldKind::I64 => "ArrowDataType::Int64",
        FieldKind::F32 => "ArrowDataType::Float32",
        FieldKind::F64 => "ArrowDataType::Float64",
        FieldKind::Bool => "ArrowDataType::Boolean",
        FieldKind::Bytes => "ArrowDataType::Binary",
        FieldKind::RawString => "ArrowDataType::Utf8",
        FieldKind::I64Array => {
            "ArrowDataType::List(Arc::new(ArrowField::new(\"item\", ArrowDataType::Int64, false)))"
        }
        FieldKind::I32Array => {
            "ArrowDataType::List(Arc::new(ArrowField::new(\"item\", ArrowDataType::Int32, false)))"
        }
        FieldKind::U32Array => {
            "ArrowDataType::List(Arc::new(ArrowField::new(\"item\", ArrowDataType::UInt32, false)))"
        }
        FieldKind::F64Array => {
            "ArrowDataType::List(Arc::new(ArrowField::new(\"item\", ArrowDataType::Float64, false)))"
        }
        FieldKind::F32Array => {
            "ArrowDataType::List(Arc::new(ArrowField::new(\"item\", ArrowDataType::Float32, false)))"
        }
    }
}

fn arrow_array_fn(field: &PhysicalField) -> &'static str {
    match field.kind {
        FieldKind::ConstU16 { .. } => "const_u16_array",
        FieldKind::U16Dictionary { optional: true, .. } => "optional_u16_array",
        FieldKind::U16Dictionary {
            optional: false, ..
        } => "u16_array",
        FieldKind::U64BitmaskDictionary { .. } => "u64_array",
        FieldKind::I32 if field.presence_bit.is_some() => "optional_i32_array",
        FieldKind::U32 if field.presence_bit.is_some() => "optional_u32_array",
        FieldKind::I64 if field.presence_bit.is_some() => "optional_i64_array",
        FieldKind::F32 if field.presence_bit.is_some() => "optional_f32_array",
        FieldKind::F64 if field.presence_bit.is_some() => "optional_f64_array",
        FieldKind::I32 => "i32_array",
        FieldKind::U32 => "u32_array",
        FieldKind::I64 => "i64_array",
        FieldKind::F32 => "f32_array",
        FieldKind::F64 => "f64_array",
        FieldKind::Bool => "bool_array",
        FieldKind::Bytes => "binary_array",
        FieldKind::RawString => "utf8_array",
        FieldKind::I64Array if field.presence_bit.is_some() => "optional_i64_list_array",
        FieldKind::I32Array if field.presence_bit.is_some() => "optional_i32_list_array",
        FieldKind::U32Array if field.presence_bit.is_some() => "optional_u32_list_array",
        FieldKind::F64Array if field.presence_bit.is_some() => "optional_f64_list_array",
        FieldKind::F32Array if field.presence_bit.is_some() => "optional_f32_list_array",
        FieldKind::I64Array => "i64_list_array",
        FieldKind::I32Array => "i32_list_array",
        FieldKind::U32Array => "u32_list_array",
        FieldKind::F64Array => "f64_list_array",
        FieldKind::F32Array => "f32_list_array",
    }
}

fn dictionary_metadata_expr(field: &PhysicalField) -> String {
    match &field.kind {
        FieldKind::U16Dictionary { dictionary, .. } => format!("Some({dictionary:?})"),
        _ => "None".to_string(),
    }
}

fn bitmask_metadata_expr(field: &PhysicalField) -> String {
    match &field.kind {
        FieldKind::U64BitmaskDictionary { dictionary } => format!("Some({dictionary:?})"),
        _ => "None".to_string(),
    }
}

fn projection_schema_model(source: &SchemaModel, projection: &ProjectionModel) -> SchemaModel {
    SchemaModel {
        module: source.module.clone(),
        proto: source.proto.clone(),
        root: source.root.clone(),
        root_type: source.root_type.clone(),
        payload_type: projection.payload_type.clone(),
        row_type: projection.row_type.clone(),
        marker_type: projection.marker_type.clone(),
        view_type: projection.view_type.clone(),
        rows_iter_type: projection.rows_iter_type.clone(),
        archived_row_type: projection.archived_row_type.clone(),
        schema_id: source.schema_id,
        schema_version: source.schema_version,
        schema_version_value: source.schema_version_value,
        transport_name: projection.transport_name.clone(),
        payload_root: source.payload_root,
        row_field_name: source.row_field_name.clone(),
        row_field_number: source.row_field_number,
        dictionaries: projection.dictionaries.clone(),
        fields: projection.fields.clone(),
        derived_utc_fields: Vec::new(),
        json_csv_output_fields: Vec::new(),
        protobuf_messages: Vec::new(),
        key_parts: projection.key_parts.clone(),
        normalized_schema_hash: projection.normalized_schema_hash,
        projections: Vec::new(),
    }
}

fn projection_adapter_schema_model(
    source: &SchemaModel,
    projection: &ProjectionModel,
) -> Result<SchemaModel> {
    let mut model = projection_schema_model(source, projection);
    let field_map = projection_field_index_map(source, projection)?;
    let derived_map =
        remap_projection_derived_utc_fields(source, projection, &field_map, &mut model);
    model.json_csv_output_fields = remap_json_csv_outputs(source, &field_map, &derived_map);
    model.protobuf_messages = remap_protobuf_messages(source, &field_map, &derived_map);
    Ok(model)
}

fn projection_field_index_map(
    source: &SchemaModel,
    projection: &ProjectionModel,
) -> Result<Vec<Option<usize>>> {
    let mut field_map = vec![None; source.fields.len()];
    for (projected_index, mapping) in projection.field_mappings.iter().enumerate() {
        if mapping.source_index >= field_map.len() {
            return Err(CodegenError::InvalidSchema(format!(
                "projection {} references unknown source field index {}",
                projection.definition.name, mapping.source_index
            )));
        }
        field_map[mapping.source_index] = Some(projected_index);
    }
    Ok(field_map)
}

fn remap_projection_derived_utc_fields(
    source: &SchemaModel,
    projection: &ProjectionModel,
    field_map: &[Option<usize>],
    model: &mut SchemaModel,
) -> Vec<Option<usize>> {
    let mut derived_map = vec![None; source.derived_utc_fields.len()];
    for (source_derived_index, source_derived) in source.derived_utc_fields.iter().enumerate() {
        let Some(projected_field_index) = field_map[source_derived.source_field_index] else {
            continue;
        };
        let projected_field = &projection.fields[projected_field_index];
        let mut projected_derived = source_derived.clone();
        projected_derived.source_field_index = projected_field_index;
        projected_derived.source_rust_name = projected_field.rust_name.clone();
        projected_derived.source_presence_bit = projected_field.presence_bit;
        let projected_derived_index = model.derived_utc_fields.len();
        model.derived_utc_fields.push(projected_derived);
        derived_map[source_derived_index] = Some(projected_derived_index);
    }
    derived_map
}

fn remap_json_csv_outputs(
    source: &SchemaModel,
    field_map: &[Option<usize>],
    derived_map: &[Option<usize>],
) -> Vec<JsonCsvOutputField> {
    source
        .json_csv_output_fields
        .iter()
        .filter_map(|output| match output {
            JsonCsvOutputField::Physical { field_index } => field_map[*field_index]
                .map(|field_index| JsonCsvOutputField::Physical { field_index }),
            JsonCsvOutputField::DerivedUtc { derived_index } => derived_map[*derived_index]
                .map(|derived_index| JsonCsvOutputField::DerivedUtc { derived_index }),
        })
        .collect()
}

fn remap_protobuf_messages(
    source: &SchemaModel,
    field_map: &[Option<usize>],
    derived_map: &[Option<usize>],
) -> Vec<ProtobufMessageModel> {
    let mut messages = Vec::new();
    if !source.protobuf_messages.is_empty() {
        let _ = remap_protobuf_message_into(source, 0, field_map, derived_map, &mut messages);
    }
    messages
}

fn remap_protobuf_message_into(
    source: &SchemaModel,
    source_message_index: usize,
    field_map: &[Option<usize>],
    derived_map: &[Option<usize>],
    messages: &mut Vec<ProtobufMessageModel>,
) -> Option<usize> {
    let source_message = &source.protobuf_messages[source_message_index];
    let projected_message_index = messages.len();
    let mut projected_message = source_message.clone();
    projected_message.fields.clear();
    messages.push(projected_message);

    let mut fields = Vec::new();
    for output in &source_message.fields {
        match output {
            ProtobufOutputField::Physical { field_index } => {
                if let Some(projected_field_index) = field_map[*field_index] {
                    fields.push(ProtobufOutputField::Physical {
                        field_index: projected_field_index,
                    });
                }
            }
            ProtobufOutputField::DerivedUtc { derived_index } => {
                if let Some(projected_derived_index) = derived_map[*derived_index] {
                    fields.push(ProtobufOutputField::DerivedUtc {
                        derived_index: projected_derived_index,
                    });
                }
            }
            ProtobufOutputField::Message { message_index } => {
                if let Some(projected_child_index) = remap_protobuf_message_into(
                    source,
                    *message_index,
                    field_map,
                    derived_map,
                    messages,
                ) {
                    fields.push(ProtobufOutputField::Message {
                        message_index: projected_child_index,
                    });
                }
            }
        }
    }

    if fields.is_empty() {
        messages.pop();
        return None;
    }
    messages[projected_message_index].fields = fields;
    Some(projected_message_index)
}

fn validate_model(model: &SchemaModel) -> Result<()> {
    if model.fields.is_empty() {
        return Err(CodegenError::InvalidSchema(
            "schema has no physical fields".to_string(),
        ));
    }
    if !model
        .fields
        .iter()
        .any(|field| matches!(field.kind, FieldKind::ConstU16 { .. }))
    {
        return Err(CodegenError::InvalidSchema(
            "schema has no const_u16 schema version field".to_string(),
        ));
    }
    Ok(())
}

fn emit_header(out: &mut String, model: &SchemaModel) {
    // Generated artifacts carry deterministic identity in their header comment.
    out.push_str("// This file is @generated by mbt_codegen.\n");
    out.push_str("// Do not edit by hand.\n");
    out.push_str(&format!(
        "// schema_id={} schema_version={} schema_hash={}\n\n",
        model.schema_id, model.schema_version, model.normalized_schema_hash
    ));
}

fn emit_imports(out: &mut String) {
    // Core imports are shared by every generated schema surface.
    out.push_str(
        r#"use rkyv::{Archive, Place, Serialize as RkyvSerialize};
use rkyv::rancor::{Error as RkyvError, Fallible, Source};
use rkyv::ser::{Allocator, Writer};
use rkyv::vec::{ArchivedVec, VecResolver};

use mbt_core::envelope::{
    decode_header, encode_header, fnv1a64, trusted_payload_for_schema,
    validate_header_for_schema, SchemaHeaderSpec, TransportHeader, HEADER_LEN,
};
use mbt_core::error::{Result, TransportError};
use mbt_core::runtime::{BinaryInspection, MbtSchema};

"#,
    );
}

fn emit_schema_constants(out: &mut String, scope: &EmitScope<'_>) {
    let model = scope.model;
    out.push_str(&format!(
        "{}const {}: u32 = {};\n",
        scope.item_vis(),
        scope.const_name("SCHEMA_ID"),
        model.schema_id
    ));
    out.push_str(&format!(
        "{}const {}: u16 = {};\n",
        scope.item_vis(),
        scope.const_name("SCHEMA_VERSION_VALUE"),
        model.schema_version_value
    ));
    out.push_str(&format!(
        "{}const {}: u16 = {};\n",
        scope.item_vis(),
        scope.const_name("SCHEMA_VERSION"),
        scope.const_name("SCHEMA_VERSION_VALUE")
    ));
    out.push_str(&format!(
        "{}const {}: u64 = {};\n",
        scope.item_vis(),
        scope.const_name("GENERATED_SCHEMA_HASH"),
        model.normalized_schema_hash
    ));
    out.push_str(&format!(
        "{}const {}: &str = {:?};\n",
        scope.item_vis(),
        scope.const_name("TRANSPORT_NAME"),
        model.transport_name
    ));
    out.push_str(&format!(
        "{}const {}: SchemaHeaderSpec = SchemaHeaderSpec {{\n\
         \tschema_id: {},\n\
         \tschema_version: {},\n\
         \tschema_hash: {},\n\
         }};\n\n",
        scope.item_vis(),
        scope.const_name("SCHEMA_HEADER"),
        scope.const_name("SCHEMA_ID"),
        scope.const_name("SCHEMA_VERSION"),
        scope.const_name("GENERATED_SCHEMA_HASH")
    ));
}

fn emit_dictionary_constants(out: &mut String, model: &SchemaModel) -> Result<()> {
    for dictionary in &model.dictionaries {
        if !dictionary_is_used(model, &dictionary.name) {
            continue;
        }
        let prefix = dict_prefix(&dictionary.name);
        if dictionary_is_bitmask(model, &dictionary.name) {
            emit_bitmask_dictionary(out, dictionary, &prefix);
        } else if dictionary_is_key(model, &dictionary.name) {
            emit_zero_based_dictionary(out, dictionary, &prefix);
        } else {
            emit_one_based_dictionary(out, dictionary, &prefix);
        }
        out.push('\n');
    }
    Ok(())
}

fn emit_zero_based_dictionary(out: &mut String, dictionary: &Dictionary, prefix: &str) {
    for (idx, value) in dictionary.values.iter().enumerate() {
        out.push_str(&format!(
            "pub const {prefix}_{}: u16 = {idx};\n",
            const_name(value)
        ));
    }
    out.push_str(&format!(
        "pub const {prefix}_COUNT: u16 = {};\n",
        dictionary.values.len()
    ));
}

fn emit_one_based_dictionary(out: &mut String, dictionary: &Dictionary, prefix: &str) {
    out.push_str(&format!("pub const {prefix}_NONE: u16 = 0;\n"));
    for (idx, value) in dictionary.values.iter().enumerate() {
        out.push_str(&format!(
            "pub const {prefix}_{}: u16 = {};\n",
            const_name(value),
            idx + 1
        ));
    }
}

fn emit_bitmask_dictionary(out: &mut String, dictionary: &Dictionary, prefix: &str) {
    let mut mask_terms = Vec::new();
    for (idx, value) in dictionary.values.iter().enumerate() {
        let bit_name = format!("{prefix}_{}_BIT", const_name(value));
        out.push_str(&format!("pub const {bit_name}: u64 = {idx};\n"));
        mask_terms.push(format!("(1 << {bit_name})"));
    }
    let mask = if mask_terms.is_empty() {
        "0".to_string()
    } else {
        mask_terms.join(" | ")
    };
    out.push_str(&format!("pub const VALID_{prefix}_MASK: u64 = {mask};\n"));
}

fn emit_presence_constants(out: &mut String, scope: &EmitScope<'_>) -> Result<()> {
    let model = scope.model;
    let fields = presence_fields(model);
    for field in &fields {
        let bit = field
            .presence_bit
            .ok_or_else(|| CodegenError::InvalidSchema("presence field without bit".to_string()))?;
        if presence_is_wide(model) {
            out.push_str(&format!(
                "{}const {}: usize = {};\n",
                scope.item_vis(),
                presence_word_const(scope, field),
                presence_word(bit)
            ));
            out.push_str(&format!(
                "{}const {}: u64 = {};\n",
                scope.item_vis(),
                presence_mask_const(scope, field),
                presence_mask(bit)
            ));
        } else {
            out.push_str(&format!(
                "{}const {}: u64 = 1 << {bit};\n",
                scope.item_vis(),
                presence_const(scope, field)
            ));
        }
    }
    if fields.is_empty() {
        return Ok(());
    } else if presence_is_wide(model) {
        let masks = presence_allowed_masks(model)
            .iter()
            .map(|mask| format!("{mask}_u64"))
            .collect::<Vec<_>>()
            .join(", ");
        out.push_str(&format!(
            "{}const {}: usize = {};\n",
            scope.item_vis(),
            scope.const_name("PRESENCE_WORDS"),
            presence_words(model)
        ));
        out.push_str(&format!(
            "{}const {}: [u64; {}] = [{masks}];\n\n",
            scope.item_vis(),
            scope.const_name("PRESENCE_ALLOWED_MASKS"),
            scope.const_name("PRESENCE_WORDS")
        ));
    } else {
        out.push_str(&format!(
            "{}const {}: u64 = (1 << {}) - 1;\n\n",
            scope.item_vis(),
            scope.const_name("PRESENCE_ALLOWED_MASK"),
            fields.len()
        ));
    }
    Ok(())
}

fn emit_structs(out: &mut String, model: &SchemaModel) {
    out.push_str("#[derive(Clone, Archive, RkyvSerialize)]\n");
    out.push_str(&format!("pub struct {} {{\n", model.payload_type));
    out.push_str("    pub schema_version: u16,\n");
    out.push_str(&format!(
        "    pub {}: Vec<{}>,\n",
        model.row_field_name, model.row_type
    ));
    out.push_str("}\n\n");

    out.push_str("#[derive(Clone, Archive, RkyvSerialize)]\n");
    out.push_str(&format!("pub struct {} {{\n", model.row_type));
    for field in &model.fields {
        out.push_str(&format!(
            "    pub {}: {},\n",
            field.rust_name,
            rust_field_type(field)
        ));
    }
    if has_presence(model) {
        out.push_str(&format!(
            "    pub presence_bits: {},\n",
            presence_storage_type(model)
        ));
    }
    out.push_str("}\n\n");
}

fn emit_validation(out: &mut String, scope: &EmitScope<'_>) -> Result<()> {
    // Owned-row validation is emitted before archive construction.
    let model = scope.model;
    out.push_str(&format!(
        "{}fn {}(rows: &[{}]) -> Result<()> {{\n",
        scope.item_vis(),
        scope.fn_name("validate_rows"),
        model.row_type
    ));
    out.push_str("    let mut previous = None;\n");
    out.push_str("    for row in rows {\n");
    out.push_str(&format!(
        "        {}(row, previous)?;\n",
        scope.fn_name("validate_row")
    ));
    out.push_str("        previous = Some(row);\n");
    out.push_str("    }\n");
    out.push_str("    Ok(())\n");
    out.push_str("}\n\n");

    out.push_str(&format!(
        "{}fn {}(row: &{}, previous: Option<&{}>) -> Result<()> {{\n",
        scope.item_vis(),
        scope.fn_name("validate_row"),
        model.row_type,
        model.row_type
    ));
    for field in &model.fields {
        emit_owned_field_validation(out, model, field)?;
    }
    if has_presence(model) {
        emit_presence_validation(out, scope);
    }
    emit_order_validation(out, model);
    out.push_str("    Ok(())\n");
    out.push_str("}\n\n");
    if scope.public_free_items {
        emit_validation_helpers(out, model);
    }
    Ok(())
}

fn emit_owned_field_validation(
    out: &mut String,
    model: &SchemaModel,
    field: &PhysicalField,
) -> Result<()> {
    match &field.kind {
        FieldKind::ConstU16 { value } => out.push_str(&format!(
            "    if row.{} != {value} {{ return Err(TransportError::SchemaVersionMismatch {{ observed: row.{}, expected: {value} }}); }}\n",
            field.rust_name, field.rust_name
        )),
        FieldKind::U16Dictionary {
            dictionary,
            optional,
        } => {
            let helper = format!("{}_symbol", dictionary_helper_stem(model, dictionary));
            if *optional {
                out.push_str(&format!(
                    "    if row.{} != 0 {{ {helper}(row.{})?; }}\n",
                    field.rust_name, field.rust_name
                ));
            } else {
                out.push_str(&format!("    {helper}(row.{})?;\n", field.rust_name));
            }
        }
        FieldKind::U64BitmaskDictionary { dictionary } => {
            let prefix = dict_prefix(dictionary);
            out.push_str(&format!(
                "    if row.{} & !VALID_{prefix}_MASK != 0 {{ return Err(TransportError::InvalidBitmask {{ field: {:?}, value: row.{} }}); }}\n",
                field.rust_name, field.logical_path, field.rust_name
            ));
        }
        FieldKind::F32 => out.push_str(&format!(
            "    validate_finite_f32({:?}, row.{})?;\n",
            field.logical_path, field.rust_name
        )),
        FieldKind::F64 => out.push_str(&format!(
            "    validate_finite_f64({:?}, row.{})?;\n",
            field.logical_path, field.rust_name
        )),
        FieldKind::F32Array => out.push_str(&format!(
            "    for value in &row.{} {{ validate_finite_f32({:?}, *value)?; }}\n",
            field.rust_name, field.logical_path
        )),
        FieldKind::F64Array => out.push_str(&format!(
            "    for value in &row.{} {{ validate_finite_f64({:?}, *value)?; }}\n",
            field.rust_name, field.logical_path
        )),
        FieldKind::I32
        | FieldKind::U32
        | FieldKind::I64
        | FieldKind::Bool
        | FieldKind::Bytes
        | FieldKind::RawString
        | FieldKind::I64Array
        | FieldKind::I32Array
        | FieldKind::U32Array => {}
    }
    Ok(())
}

fn emit_presence_validation(out: &mut String, scope: &EmitScope<'_>) {
    let model = scope.model;
    if presence_is_wide(model) {
        out.push_str(&format!(
            "    for (word, allowed_mask) in {}.iter().enumerate() {{\n",
            scope.const_name("PRESENCE_ALLOWED_MASKS")
        ));
        out.push_str("        let value = row.presence_bits[word];\n");
        out.push_str("        if value & !allowed_mask != 0 { return Err(TransportError::InvalidPresenceWord { word, value }); }\n");
        out.push_str("    }\n");
    } else {
        out.push_str(&format!(
            "    if row.presence_bits & !{} != 0 {{ return Err(TransportError::InvalidPresenceBits(row.presence_bits)); }}\n",
            scope.const_name("PRESENCE_ALLOWED_MASK")
        ));
    }
    for field in model
        .fields
        .iter()
        .filter(|field| field.presence_bit.is_some())
    {
        let predicate = owned_presence_absent_expr(scope, "row.presence_bits", field);
        let error = presence_error_expr(scope, "row.presence_bits", field);
        match field.kind {
            FieldKind::F32 | FieldKind::F64 => out.push_str(&format!(
                "    if {predicate} && row.{} != 0.0 {{ return Err({error}); }}\n",
                field.rust_name
            )),
            FieldKind::I32 | FieldKind::U32 | FieldKind::I64 | FieldKind::U16Dictionary { .. } => {
                out.push_str(&format!(
                    "    if {predicate} && row.{} != 0 {{ return Err({error}); }}\n",
                    field.rust_name
                ));
            }
            FieldKind::Bool => out.push_str(&format!(
                "    if {predicate} && row.{} {{ return Err({error}); }}\n",
                field.rust_name
            )),
            FieldKind::Bytes
            | FieldKind::RawString
            | FieldKind::I64Array
            | FieldKind::I32Array
            | FieldKind::U32Array
            | FieldKind::F64Array
            | FieldKind::F32Array => out.push_str(&format!(
                "    if {predicate} && !row.{}.is_empty() {{ return Err({error}); }}\n",
                field.rust_name
            )),
            FieldKind::ConstU16 { .. } | FieldKind::U64BitmaskDictionary { .. } => {}
        }
    }
}

fn emit_order_validation(out: &mut String, model: &SchemaModel) {
    // Key order validation preserves deterministic row ordering.
    if model.key_parts.is_empty() {
        return;
    }
    let current = model
        .key_parts
        .iter()
        .map(|part| format!("row.{}", part.rust_name))
        .collect::<Vec<_>>()
        .join(", ");
    let previous = model
        .key_parts
        .iter()
        .map(|part| format!("prev.{}", part.rust_name))
        .collect::<Vec<_>>()
        .join(", ");
    out.push_str(&format!(
        "    if previous.is_some_and(|prev| ({current}) < ({previous})) {{\n"
    ));
    out.push_str(
        "        return Err(TransportError::InvalidTimeGrid(\"key order regression\".to_string()));\n",
    );
    out.push_str("    }\n");
}

fn emit_validation_helpers(out: &mut String, model: &SchemaModel) {
    if model
        .fields
        .iter()
        .any(|field| matches!(field.kind, FieldKind::F32 | FieldKind::F32Array))
    {
        out.push_str(
            r#"fn validate_finite_f32(field: &'static str, value: f32) -> Result<()> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(TransportError::NonFiniteNumeric(field))
    }
}

"#,
        );
    }
    if model
        .fields
        .iter()
        .any(|field| matches!(field.kind, FieldKind::F64 | FieldKind::F64Array))
    {
        out.push_str(
            r#"fn validate_finite_f64(field: &'static str, value: f64) -> Result<()> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(TransportError::NonFiniteNumeric(field))
    }
}

"#,
        );
    }
}

fn emit_checksums(out: &mut String, scope: &EmitScope<'_>) {
    // Checksums are deterministic evidence values over semantic and projection fields.
    let model = scope.model;
    let split_minimal_checksum = !model.projections.is_empty();
    let minimal_fields = checksum_minimal_fields(model);
    let metadata_fields = checksum_metadata_fields(model, &minimal_fields);
    let has_metadata_checksum_fields =
        split_minimal_checksum && (has_presence(model) || !metadata_fields.is_empty());
    if scope.public_free_items {
        out.push_str(&format!(
            "{}fn {}(rows: &[{}]) -> u64 {{\n",
            scope.item_vis(),
            scope.fn_name("semantic_checksum"),
            model.row_type
        ));
        out.push_str(&format!(
            "    let mut checksum = fnv1a64({:?}.as_bytes());\n",
            model.transport_name
        ));
        out.push_str("    for row in rows {\n");
        out.push_str(&format!(
            "        checksum = {}(checksum, row, true);\n",
            scope.fn_name("checksum_row")
        ));
        out.push_str("    }\n");
        out.push_str("    checksum\n");
        out.push_str("}\n\n");

        out.push_str(&format!(
            "{}fn {}(rows: &[{}]) -> u64 {{\n",
            scope.item_vis(),
            scope.fn_name("minimal_projection_checksum"),
            model.row_type
        ));
        if split_minimal_checksum {
            out.push_str(&format!(
                "    let mut checksum = fnv1a64({:?}.as_bytes());\n",
                format!("{}-minimal", model.transport_name)
            ));
        } else {
            out.push_str(&format!(
                "    let mut checksum = fnv1a64({:?}.as_bytes());\n",
                model.transport_name
            ));
        }
        out.push_str("    for row in rows {\n");
        out.push_str(&format!(
            "        checksum = {}(checksum, row, false);\n",
            scope.fn_name("checksum_row")
        ));
        out.push_str("    }\n");
        out.push_str("    checksum\n");
        out.push_str("}\n\n");

        out.push_str(&format!(
            "{}fn {}(mut checksum: u64, row: &{}, include_metadata: bool) -> u64 {{\n",
            scope.item_vis(),
            scope.fn_name("checksum_row"),
            model.row_type
        ));
        for field in &minimal_fields {
            emit_owned_checksum_line_indented(out, field, "row", "    ");
        }
        if has_metadata_checksum_fields {
            out.push_str("    if include_metadata {\n");
            emit_owned_presence_checksum_lines(out, model, "row", "        ");
            for field in &metadata_fields {
                emit_owned_checksum_line_indented(out, field, "row", "        ");
            }
            out.push_str("    }\n");
        } else {
            if !split_minimal_checksum {
                emit_owned_presence_checksum_lines(out, model, "row", "    ");
            } else {
                let _ = has_metadata_checksum_fields;
            }
            out.push_str("    let _ = include_metadata;\n");
        }
        out.push_str("    checksum\n");
        out.push_str("}\n\n");
    }

    out.push_str(&format!(
        "{}fn {}(mut checksum: u64, row: &<{} as Archive>::Archived, include_metadata: bool) -> u64 {{\n",
        scope.item_vis(),
        scope.fn_name("checksum_archived_row"),
        model.row_type
    ));
    for field in &minimal_fields {
        emit_archived_checksum_line_indented(out, field, "row", "    ");
    }
    if has_metadata_checksum_fields {
        out.push_str("    if include_metadata {\n");
        emit_archived_presence_checksum_lines(out, model, "row", "        ");
        for field in &metadata_fields {
            emit_archived_checksum_line_indented(out, field, "row", "        ");
        }
        out.push_str("    }\n");
    } else {
        if !split_minimal_checksum {
            emit_archived_presence_checksum_lines(out, model, "row", "    ");
        } else {
            let _ = has_metadata_checksum_fields;
        }
        out.push_str("    let _ = include_metadata;\n");
    }
    out.push_str("    checksum\n");
    out.push_str("}\n\n");

    out.push_str(&format!(
        "{}fn {}(mut checksum: u64, row: &<{} as Archive>::Archived) -> u64 {{\n",
        scope.item_vis(),
        scope.fn_name("minimal_projection_archived_row"),
        model.row_type
    ));
    for field in &minimal_fields {
        emit_archived_checksum_line_indented(out, field, "row", "    ");
    }
    if !split_minimal_checksum {
        emit_archived_presence_checksum_lines(out, model, "row", "    ");
    }
    out.push_str("    checksum\n");
    out.push_str("}\n\n");

    if scope.public_free_items {
        emit_checksum_helpers(out, model);
    }
}

fn emit_owned_checksum_line_indented(
    out: &mut String,
    field: &PhysicalField,
    row: &str,
    indent: &str,
) {
    let access = format!("{row}.{}", field.rust_name);
    match field.kind {
        FieldKind::ConstU16 { .. } | FieldKind::U16Dictionary { .. } => {
            out.push_str(&format!(
                "{indent}checksum = update_u16(checksum, {access});\n"
            ));
        }
        FieldKind::U64BitmaskDictionary { .. } => {
            out.push_str(&format!(
                "{indent}checksum = update_u64(checksum, {access});\n"
            ));
        }
        FieldKind::I32 => out.push_str(&format!(
            "{indent}checksum = update_i32(checksum, {access});\n"
        )),
        FieldKind::U32 => out.push_str(&format!(
            "{indent}checksum = update_u32(checksum, {access});\n"
        )),
        FieldKind::I64 => out.push_str(&format!(
            "{indent}checksum = update_i64(checksum, {access});\n"
        )),
        FieldKind::F32 => out.push_str(&format!(
            "{indent}checksum = update_f32(checksum, {access});\n"
        )),
        FieldKind::F64 => out.push_str(&format!(
            "{indent}checksum = update_f64(checksum, {access});\n"
        )),
        FieldKind::Bool => out.push_str(&format!(
            "{indent}checksum = update_bool(checksum, {access});\n"
        )),
        FieldKind::Bytes => out.push_str(&format!(
            "{indent}checksum = update_bytes(checksum, {access}.as_slice());\n"
        )),
        FieldKind::RawString => out.push_str(&format!(
            "{indent}checksum = update_bytes(checksum, {access}.as_bytes());\n"
        )),
        FieldKind::I64Array => out.push_str(&format!(
            "{indent}checksum = update_i64_array(checksum, {access}.as_slice());\n"
        )),
        FieldKind::I32Array => out.push_str(&format!(
            "{indent}checksum = update_i32_array(checksum, {access}.as_slice());\n"
        )),
        FieldKind::U32Array => out.push_str(&format!(
            "{indent}checksum = update_u32_array(checksum, {access}.as_slice());\n"
        )),
        FieldKind::F64Array => out.push_str(&format!(
            "{indent}checksum = update_f64_array(checksum, {access}.as_slice());\n"
        )),
        FieldKind::F32Array => out.push_str(&format!(
            "{indent}checksum = update_f32_array(checksum, {access}.as_slice());\n"
        )),
    }
}

fn emit_archived_checksum_line_indented(
    out: &mut String,
    field: &PhysicalField,
    row: &str,
    indent: &str,
) {
    let access = format!("{row}.{}", field.rust_name);
    match field.kind {
        FieldKind::ConstU16 { .. } | FieldKind::U16Dictionary { .. } => {
            out.push_str(&format!(
                "{indent}checksum = update_u16(checksum, {access}.to_native());\n"
            ));
        }
        FieldKind::U64BitmaskDictionary { .. } => {
            out.push_str(&format!(
                "{indent}checksum = update_u64(checksum, {access}.to_native());\n"
            ));
        }
        FieldKind::I32 => out.push_str(&format!(
            "{indent}checksum = update_i32(checksum, {access}.to_native());\n"
        )),
        FieldKind::U32 => out.push_str(&format!(
            "{indent}checksum = update_u32(checksum, {access}.to_native());\n"
        )),
        FieldKind::I64 => out.push_str(&format!(
            "{indent}checksum = update_i64(checksum, {access}.to_native());\n"
        )),
        FieldKind::F32 => out.push_str(&format!(
            "{indent}checksum = update_f32(checksum, {access}.to_native());\n"
        )),
        FieldKind::F64 => out.push_str(&format!(
            "{indent}checksum = update_f64(checksum, {access}.to_native());\n"
        )),
        FieldKind::Bool => out.push_str(&format!(
            "{indent}checksum = update_bool(checksum, {access});\n"
        )),
        FieldKind::Bytes => out.push_str(&format!(
            "{indent}checksum = update_bytes(checksum, {access}.as_slice());\n"
        )),
        FieldKind::RawString => out.push_str(&format!(
            "{indent}checksum = update_bytes(checksum, {access}.as_bytes());\n"
        )),
        FieldKind::I64Array => {
            emit_archived_array_checksum_lines(out, &access, "update_i64", indent)
        }
        FieldKind::I32Array => {
            emit_archived_array_checksum_lines(out, &access, "update_i32", indent)
        }
        FieldKind::U32Array => {
            emit_archived_array_checksum_lines(out, &access, "update_u32", indent)
        }
        FieldKind::F64Array => {
            emit_archived_array_checksum_lines(out, &access, "update_f64", indent)
        }
        FieldKind::F32Array => {
            emit_archived_array_checksum_lines(out, &access, "update_f32", indent)
        }
    }
}

fn emit_archived_array_checksum_lines(
    out: &mut String,
    access: &str,
    element_update_fn: &str,
    indent: &str,
) {
    out.push_str(&format!(
        "{indent}checksum = update_u64(checksum, {access}.len() as u64);\n"
    ));
    out.push_str(&format!(
        "{indent}for value in {access}.iter() {{ checksum = {element_update_fn}(checksum, value.to_native()); }}\n"
    ));
}

fn emit_owned_presence_checksum_lines(
    out: &mut String,
    model: &SchemaModel,
    row: &str,
    indent: &str,
) {
    if !has_presence(model) {
        return;
    }
    if presence_is_wide(model) {
        out.push_str(&format!(
            "{indent}for value in {row}.presence_bits {{ checksum = update_u64(checksum, value); }}\n"
        ));
    } else {
        out.push_str(&format!(
            "{indent}checksum = update_u64(checksum, {row}.presence_bits);\n"
        ));
    }
}

fn emit_archived_presence_checksum_lines(
    out: &mut String,
    model: &SchemaModel,
    row: &str,
    indent: &str,
) {
    if !has_presence(model) {
        return;
    }
    if presence_is_wide(model) {
        out.push_str(&format!(
            "{indent}for value in {row}.presence_bits.iter() {{ checksum = update_u64(checksum, value.to_native()); }}\n"
        ));
    } else {
        out.push_str(&format!(
            "{indent}checksum = update_u64(checksum, {row}.presence_bits.to_native());\n"
        ));
    }
}

fn checksum_minimal_fields(model: &SchemaModel) -> Vec<&PhysicalField> {
    let Some(projection) = model
        .projections
        .iter()
        .min_by_key(|projection| projection.fields.len())
    else {
        return model.fields.iter().collect();
    };
    let mut out = Vec::with_capacity(projection.fields.len());
    for projected in &projection.fields {
        let Some(field) = model
            .fields
            .iter()
            .find(|field| field.logical_path == projected.logical_path)
        else {
            return model.fields.iter().collect();
        };
        out.push(field);
    }
    out
}

fn checksum_metadata_fields<'a>(
    model: &'a SchemaModel,
    minimal_fields: &[&'a PhysicalField],
) -> Vec<&'a PhysicalField> {
    if model.projections.is_empty() {
        return Vec::new();
    }
    model
        .fields
        .iter()
        .filter(|field| {
            !minimal_fields
                .iter()
                .any(|minimal| minimal.logical_path == field.logical_path)
        })
        .collect()
}

fn emit_checksum_helpers(out: &mut String, model: &SchemaModel) {
    let uses_bytes = model
        .fields
        .iter()
        .any(|field| matches!(field.kind, FieldKind::Bytes | FieldKind::RawString));
    let uses_bool = model
        .fields
        .iter()
        .any(|field| matches!(field.kind, FieldKind::Bool));
    let uses_i32 = model
        .fields
        .iter()
        .any(|field| matches!(field.kind, FieldKind::I32 | FieldKind::I32Array));
    let uses_u32 = model.fields.iter().any(|field| {
        matches!(
            field.kind,
            FieldKind::U32 | FieldKind::U32Array | FieldKind::F32 | FieldKind::F32Array
        )
    });
    let uses_i64 = model
        .fields
        .iter()
        .any(|field| matches!(field.kind, FieldKind::I64 | FieldKind::I64Array));
    let uses_u64 = has_presence(model)
        || model.fields.iter().any(|field| {
            matches!(
                field.kind,
                FieldKind::U64BitmaskDictionary { .. }
                    | FieldKind::F64
                    | FieldKind::F64Array
                    | FieldKind::Bool
                    | FieldKind::Bytes
                    | FieldKind::RawString
                    | FieldKind::I64Array
                    | FieldKind::I32Array
                    | FieldKind::U32Array
                    | FieldKind::F32Array
            )
        });
    let uses_f32 = model
        .fields
        .iter()
        .any(|field| matches!(field.kind, FieldKind::F32 | FieldKind::F32Array));
    let uses_f64 = model
        .fields
        .iter()
        .any(|field| matches!(field.kind, FieldKind::F64 | FieldKind::F64Array));
    let uses_i64_array = model
        .fields
        .iter()
        .any(|field| matches!(field.kind, FieldKind::I64Array));
    let uses_i32_array = model
        .fields
        .iter()
        .any(|field| matches!(field.kind, FieldKind::I32Array));
    let uses_u32_array = model
        .fields
        .iter()
        .any(|field| matches!(field.kind, FieldKind::U32Array));
    let uses_f64_array = model
        .fields
        .iter()
        .any(|field| matches!(field.kind, FieldKind::F64Array));
    let uses_f32_array = model
        .fields
        .iter()
        .any(|field| matches!(field.kind, FieldKind::F32Array));

    if uses_bytes {
        out.push_str(
            r#"fn update_bytes(mut checksum: u64, bytes: &[u8]) -> u64 {
    checksum = update_u64(checksum, bytes.len() as u64);
    for byte in bytes {
        checksum = fnv1a64_update(checksum, *byte);
    }
    checksum
}

"#,
        );
    }
    out.push_str(
        r#"fn fnv1a64_update(mut checksum: u64, byte: u8) -> u64 {
    checksum ^= u64::from(byte);
    checksum.wrapping_mul(0x00000100000001B3)
}

fn update_fixed<const N: usize>(mut checksum: u64, bytes: [u8; N]) -> u64 {
    for byte in bytes {
        checksum = fnv1a64_update(checksum, byte);
    }
    checksum
}

"#,
    );
    if uses_bool {
        out.push_str(
            r#"fn update_bool(checksum: u64, value: bool) -> u64 {
    update_u64(checksum, u64::from(value))
}

"#,
        );
    }
    out.push_str(
        r#"fn update_u16(checksum: u64, value: u16) -> u64 {
    update_fixed(checksum, value.to_le_bytes())
}

"#,
    );
    if uses_i32 {
        out.push_str(
            r#"fn update_i32(checksum: u64, value: i32) -> u64 {
    update_fixed(checksum, value.to_le_bytes())
}

"#,
        );
    }
    if uses_u32 {
        out.push_str(
            r#"fn update_u32(checksum: u64, value: u32) -> u64 {
    update_fixed(checksum, value.to_le_bytes())
}

"#,
        );
    }
    if uses_i64 {
        out.push_str(
            r#"fn update_i64(checksum: u64, value: i64) -> u64 {
    update_fixed(checksum, value.to_le_bytes())
}

"#,
        );
    }
    if uses_u64 {
        out.push_str(
            r#"fn update_u64(checksum: u64, value: u64) -> u64 {
    update_fixed(checksum, value.to_le_bytes())
}

"#,
        );
    }
    if uses_f32 {
        out.push_str(
            r#"fn update_f32(checksum: u64, value: f32) -> u64 {
    update_u32(checksum, value.to_bits())
}

"#,
        );
    }
    if uses_f64 {
        out.push_str(
            r#"fn update_f64(checksum: u64, value: f64) -> u64 {
    update_u64(checksum, value.to_bits())
}

"#,
        );
    }
    if uses_i64_array {
        out.push_str(
            r#"fn update_i64_array(mut checksum: u64, values: &[i64]) -> u64 {
    checksum = update_u64(checksum, values.len() as u64);
    for value in values { checksum = update_i64(checksum, *value); }
    checksum
}

"#,
        );
    }
    if uses_i32_array {
        out.push_str(
            r#"fn update_i32_array(mut checksum: u64, values: &[i32]) -> u64 {
    checksum = update_u64(checksum, values.len() as u64);
    for value in values { checksum = update_i32(checksum, *value); }
    checksum
}

"#,
        );
    }
    if uses_u32_array {
        out.push_str(
            r#"fn update_u32_array(mut checksum: u64, values: &[u32]) -> u64 {
    checksum = update_u64(checksum, values.len() as u64);
    for value in values { checksum = update_u32(checksum, *value); }
    checksum
}

"#,
        );
    }
    if uses_f64_array {
        out.push_str(
            r#"fn update_f64_array(mut checksum: u64, values: &[f64]) -> u64 {
    checksum = update_u64(checksum, values.len() as u64);
    for value in values { checksum = update_f64(checksum, *value); }
    checksum
}

"#,
        );
    }
    if uses_f32_array {
        out.push_str(
            r#"fn update_f32_array(mut checksum: u64, values: &[f32]) -> u64 {
    checksum = update_u64(checksum, values.len() as u64);
    for value in values { checksum = update_f32(checksum, *value); }
    checksum
}

"#,
        );
    }
}

fn emit_dictionary_helpers(out: &mut String, model: &SchemaModel) -> Result<()> {
    for dictionary in &model.dictionaries {
        if !dictionary_is_used(model, &dictionary.name) {
            continue;
        }
        let helper = dictionary_helper_stem(model, &dictionary.name);
        let zero_based = dictionary_is_key(model, &dictionary.name);
        out.push_str(&format!(
            "pub fn {helper}_symbol(value: u16) -> Result<&'static str> {{\n"
        ));
        out.push_str("    match value {\n");
        for (idx, value) in dictionary.values.iter().enumerate() {
            let ordinal = if zero_based { idx } else { idx + 1 };
            out.push_str(&format!("        {ordinal} => Ok({value:?}),\n"));
        }
        out.push_str(&format!(
            "        other => Err(TransportError::InvalidEnumOrdinal {{ field: {:?}, value: other }}),\n",
            dictionary.name
        ));
        out.push_str("    }\n");
        out.push_str("}\n\n");

        out.push_str(&format!(
            "pub fn {helper}_ordinal(value: &str) -> Result<u16> {{\n"
        ));
        out.push_str("    match value {\n");
        for (idx, value) in dictionary.values.iter().enumerate() {
            let ordinal = if zero_based { idx } else { idx + 1 };
            out.push_str(&format!("        {value:?} => Ok({ordinal}),\n"));
        }
        out.push_str(&format!(
            "        _ => Err(TransportError::InvalidEnumOrdinal {{ field: {:?}, value: u16::MAX }}),\n",
            dictionary.name
        ));
        out.push_str("    }\n");
        out.push_str("}\n\n");
    }
    Ok(())
}

fn emit_runtime_api(out: &mut String, scope: &EmitScope<'_>) {
    // Runtime API generation keeps checked access and trusted access separate.
    let model = scope.model;
    out.push_str(&format!("pub struct {};\n\n", model.marker_type));
    out.push_str(&format!("impl {} {{\n", model.marker_type));
    out.push_str(&format!(
        "    pub const SCHEMA_ID: u32 = {};\n",
        scope.const_name("SCHEMA_ID")
    ));
    out.push_str(&format!(
        "    pub const SCHEMA_VERSION: u16 = {};\n",
        scope.const_name("SCHEMA_VERSION")
    ));
    out.push_str(&format!(
        "    pub const SCHEMA_HASH: u64 = {};\n",
        scope.const_name("GENERATED_SCHEMA_HASH")
    ));
    out.push_str(&format!(
        "    pub const TRANSPORT_NAME: &'static str = {};\n\n",
        scope.const_name("TRANSPORT_NAME")
    ));
    out.push_str(&format!(
        "    pub fn header_spec() -> SchemaHeaderSpec {{ {} }}\n\n",
        scope.const_name("SCHEMA_HEADER")
    ));
    out.push_str(&format!(
        "    pub fn encode(rows: &[{}], max_response_bytes: usize) -> Result<Vec<u8>> {{\n",
        model.row_type
    ));
    out.push_str("        Self::encode_owned(rows.to_vec(), max_response_bytes)\n");
    out.push_str("    }\n\n");
    out.push_str(&format!(
        "    pub fn encode_owned(rows: Vec<{}>, max_response_bytes: usize) -> Result<Vec<u8>> {{\n",
        model.row_type
    ));
    out.push_str(&format!(
        "        {}(&rows)?;\n",
        scope.fn_name("validate_rows")
    ));
    out.push_str("        let row_count = rows.len();\n");
    if model.row_field_name == "rows" {
        out.push_str(&format!(
            "        let payload = {} {{ schema_version: {}, rows }};\n",
            model.payload_type,
            scope.const_name("SCHEMA_VERSION_VALUE")
        ));
    } else {
        out.push_str(&format!(
            "        let payload = {} {{ schema_version: {}, {}: rows }};\n",
            model.payload_type,
            scope.const_name("SCHEMA_VERSION_VALUE"),
            model.row_field_name
        ));
    }
    out.push_str("        let payload_bytes = rkyv::to_bytes::<RkyvError>(&payload).map_err(|err| TransportError::MalformedArchive(err.to_string()))?;\n");
    out.push_str("        let payload_len = payload_bytes.len();\n");
    out.push_str("        let total_len = HEADER_LEN.checked_add(payload_len).ok_or(TransportError::ResponseTooLarge { observed: usize::MAX, cap: max_response_bytes })?;\n");
    out.push_str("        if total_len > max_response_bytes { return Err(TransportError::ResponseTooLarge { observed: total_len, cap: max_response_bytes }); }\n");
    out.push_str("        let header = TransportHeader::new_with_schema(Self::header_spec(), row_count as u64, payload_len as u64, fnv1a64(&payload_bytes));\n");
    out.push_str("        let mut header_bytes = [0_u8; HEADER_LEN];\n");
    out.push_str("        encode_header(&header, &mut header_bytes);\n");
    out.push_str("        let mut out = Vec::with_capacity(total_len);\n");
    out.push_str("        out.extend_from_slice(&header_bytes);\n");
    out.push_str("        out.extend_from_slice(&payload_bytes);\n");
    out.push_str("        Ok(out)\n");
    out.push_str("    }\n\n");
    out.push_str(&format!(
        "    pub fn access(bytes: &[u8]) -> Result<{}<'_>> {{\n",
        model.view_type
    ));
    out.push_str(&format!(
        "        Ok({} {{ archived: Self::access_archived(bytes)? }})\n",
        model.view_type
    ));
    out.push_str("    }\n\n");
    out.push_str(&format!(
        "    pub(crate) fn access_archived(bytes: &[u8]) -> Result<&Archived{}> {{\n",
        model.payload_type
    ));
    out.push_str("        let (header, payload) = decode_payload(bytes, Self::header_spec())?;\n");
    out.push_str(&format!(
        "        let archived = rkyv::access::<Archived{}, RkyvError>(payload).map_err(|err| TransportError::MalformedArchive(err.to_string()))?;\n",
        model.payload_type
    ));
    out.push_str(&format!(
        "        {}(archived, header.row_count)?;\n",
        scope.fn_name("validate_archived_payload")
    ));
    out.push_str("        Ok(archived)\n");
    out.push_str("    }\n\n");
    out.push_str("    /// Returns an archived payload view for immutable bytes already validated for this schema.\n");
    out.push_str("    ///\n");
    out.push_str("    /// # Safety\n");
    out.push_str(
        "    /// The caller guarantees that bytes were previously accepted by checked MBT access\n",
    );
    out.push_str("    /// for this schema and then stored or transported without mutation.\n");
    out.push_str(&format!(
        "    pub unsafe fn access_archived_trusted_unchecked(bytes: &[u8]) -> Result<&Archived{}> {{\n",
        model.payload_type
    ));
    out.push_str(
        "        let payload = trusted_payload_for_schema(bytes, Self::header_spec())?;\n",
    );
    out.push_str(&format!(
        "        Ok(unsafe {{ rkyv::access_unchecked::<Archived{}>(payload) }})\n",
        model.payload_type
    ));
    out.push_str("    }\n\n");
    out.push_str("    pub fn inspect(bytes: &[u8]) -> Result<BinaryInspection> {\n");
    out.push_str(&format!(
        "        {}(Self::access_archived(bytes)?)\n",
        scope.fn_name("inspect_archived_rows")
    ));
    out.push_str("    }\n");
    out.push_str("}\n\n");
}

fn emit_runtime_trait(out: &mut String, model: &SchemaModel) {
    out.push_str(&format!("impl MbtSchema for {} {{\n", model.marker_type));
    out.push_str(&format!("    type Row = {};\n", model.row_type));
    out.push_str(&format!("    type View<'a> = {}<'a>;\n\n", model.view_type));
    out.push_str("    fn encode_rows(rows: &[Self::Row], max_response_bytes: usize) -> Result<Vec<u8>> { Self::encode(rows, max_response_bytes) }\n");
    out.push_str("    fn encode_owned_rows(rows: Vec<Self::Row>, max_response_bytes: usize) -> Result<Vec<u8>> { Self::encode_owned(rows, max_response_bytes) }\n");
    out.push_str(
        "    fn access_view(bytes: &[u8]) -> Result<Self::View<'_>> { Self::access(bytes) }\n",
    );
    out.push_str(
        "    fn inspect_bytes(bytes: &[u8]) -> Result<BinaryInspection> { Self::inspect(bytes) }\n",
    );
    out.push_str("}\n\n");
}

fn emit_view_types(out: &mut String, scope: &EmitScope<'_>) {
    // Views expose archived rows without decoding the payload into owned DTOs.
    let model = scope.model;
    out.push_str(&format!(
        "pub struct {}<'a> {{\n    archived: &'a Archived{},\n}}\n\n",
        model.view_type, model.payload_type
    ));
    out.push_str(&format!("impl<'a> {}<'a> {{\n", model.view_type));
    out.push_str(&format!(
        "    pub fn len(&self) -> usize {{ self.archived.{}.len() }}\n",
        model.row_field_name
    ));
    out.push_str(&format!(
        "    pub fn is_empty(&self) -> bool {{ self.archived.{}.is_empty() }}\n",
        model.row_field_name
    ));
    out.push_str(&format!(
        "    pub fn rows(&self) -> {}<'a> {{ {} {{ inner: self.archived.{}.iter() }} }}\n",
        model.rows_iter_type, model.rows_iter_type, model.row_field_name
    ));
    out.push_str("}\n\n");

    out.push_str(&format!(
        "pub struct {}<'a> {{\n    inner: core::slice::Iter<'a, <{} as Archive>::Archived>,\n}}\n\n",
        model.rows_iter_type, model.row_type
    ));
    out.push_str(&format!(
        "impl<'a> Iterator for {}<'a> {{\n",
        model.rows_iter_type
    ));
    out.push_str(&format!(
        "    type Item = {}<'a>;\n",
        model.archived_row_type
    ));
    out.push_str(&format!(
        "    fn next(&mut self) -> Option<Self::Item> {{ self.inner.next().map(|row| {} {{ row }}) }}\n",
        model.archived_row_type
    ));
    out.push_str("}\n\n");

    out.push_str(&format!(
        "pub struct {}<'a> {{\n    row: &'a <{} as Archive>::Archived,\n}}\n\n",
        model.archived_row_type, model.row_type
    ));
    out.push_str(&format!("impl<'a> {}<'a> {{\n", model.archived_row_type));
    for field in &model.fields {
        emit_archived_getter(out, scope, field);
    }
    if has_presence(model) {
        if presence_is_wide(model) {
            out.push_str("    pub fn presence_words(&self) -> impl Iterator<Item = u64> + '_ { self.row.presence_bits.iter().map(|word| word.to_native()) }\n");
        } else {
            out.push_str(
                "    pub fn presence_bits(&self) -> u64 { self.row.presence_bits.to_native() }\n",
            );
        }
    }
    out.push_str("}\n\n");
}

fn emit_archived_getter(out: &mut String, scope: &EmitScope<'_>, field: &PhysicalField) {
    let model = scope.model;
    if field.presence_bit.is_some() {
        out.push_str(&format!(
            "    pub fn has_{}(&self) -> bool {{ {} }}\n",
            field.proto_name,
            archived_presence_has_expr(scope, "self.row.presence_bits", field)
        ));
    }
    let return_type = archived_getter_return_type(field);
    out.push_str(&format!(
        "    pub fn {}(&self) -> {} {{ {} }}\n",
        field.rust_name,
        return_type,
        archived_value_access(field)
    ));
    if let FieldKind::U16Dictionary { dictionary, .. } = &field.kind {
        out.push_str(&format!(
            "    pub fn {}(&self) -> Result<&'static str> {{ {}_symbol(self.row.{}.to_native()) }}\n",
            field.proto_name,
            dictionary_helper_stem(model, dictionary),
            field.rust_name
        ));
    }
}

fn emit_decode_helpers(out: &mut String, scope: &EmitScope<'_>, emit_decode_payload: bool) {
    // Decode helpers centralize envelope checks and archived-row validation.
    let model = scope.model;
    if emit_decode_payload {
        out.push_str(
            "fn decode_payload(bytes: &[u8], schema: SchemaHeaderSpec) -> Result<(TransportHeader, &[u8])> {\n\
             \tlet header = decode_header(bytes)?;\n\
             \tlet payload = &bytes[HEADER_LEN..];\n\
             \tvalidate_header_for_schema(&header, payload, schema)?;\n\
             \tOk((header, payload))\n\
             }\n\n",
        );
    }

    out.push_str(&format!(
        "fn {}(archived: &Archived{}, expected_rows: u64) -> Result<()> {{\n",
        scope.fn_name("validate_archived_payload"),
        model.payload_type
    ));
    out.push_str(&format!(
        "    if archived.schema_version.to_native() != {} {{\n",
        scope.const_name("SCHEMA_VERSION_VALUE")
    ));
    out.push_str(&format!(
        "        return Err(TransportError::SchemaVersionMismatch {{ observed: archived.schema_version.to_native(), expected: {} }});\n",
        scope.const_name("SCHEMA_VERSION_VALUE")
    ));
    out.push_str("    }\n");
    out.push_str(&format!(
        "    let expected = usize::try_from(expected_rows).map_err(|err| TransportError::MalformedArchive(err.to_string()))?;\n    if archived.{}.len() != expected {{ return Err(TransportError::RowCountMismatch {{ observed: archived.{}.len(), expected: expected_rows }}); }}\n",
        model.row_field_name, model.row_field_name
    ));
    out.push_str(&format!(
        "    {}(archived)\n",
        scope.fn_name("validate_archived_rows")
    ));
    out.push_str("}\n\n");

    out.push_str(&format!(
        "fn {}(archived: &Archived{}) -> Result<()> {{\n",
        scope.fn_name("validate_archived_rows"),
        model.payload_type
    ));
    out.push_str("    let mut previous = None;\n");
    out.push_str(&format!(
        "    for archived_row in archived.{}.iter() {{\n",
        model.row_field_name
    ));
    out.push_str(&format!(
        "        let row = {}(archived_row);\n",
        scope.fn_name("row_from_archived")
    ));
    out.push_str(&format!(
        "        {}(&row, previous.as_ref())?;\n",
        scope.fn_name("validate_row")
    ));
    out.push_str("        previous = Some(row);\n");
    out.push_str("    }\n");
    out.push_str("    Ok(())\n");
    out.push_str("}\n\n");

    out.push_str(&format!(
        "fn {}(archived: &Archived{}) -> Result<BinaryInspection> {{\n",
        scope.fn_name("inspect_archived_rows"),
        model.payload_type
    ));
    out.push_str("    let mut previous = None;\n");
    out.push_str(&format!(
        "    let mut semantic_checksum = fnv1a64({:?}.as_bytes());\n",
        model.transport_name
    ));
    if model.projections.is_empty() {
        out.push_str("    let mut minimal_projection_checksum = semantic_checksum;\n");
    } else {
        out.push_str(&format!(
            "    let mut minimal_projection_checksum = fnv1a64({:?}.as_bytes());\n",
            format!("{}-minimal", model.transport_name)
        ));
    }
    out.push_str(&format!(
        "    for archived_row in archived.{}.iter() {{\n",
        model.row_field_name
    ));
    out.push_str(&format!(
        "        let row = {}(archived_row);\n",
        scope.fn_name("row_from_archived")
    ));
    out.push_str(&format!(
        "        {}(&row, previous.as_ref())?;\n",
        scope.fn_name("validate_row")
    ));
    out.push_str(&format!(
        "        semantic_checksum = {}(semantic_checksum, archived_row, true);\n",
        scope.fn_name("checksum_archived_row")
    ));
    out.push_str(&format!(
        "        minimal_projection_checksum = {}(minimal_projection_checksum, archived_row);\n",
        scope.fn_name("minimal_projection_archived_row")
    ));
    out.push_str("        previous = Some(row);\n");
    out.push_str("    }\n");
    out.push_str(&format!(
        "    Ok(BinaryInspection {{ row_count: archived.{}.len(), semantic_checksum, minimal_projection_checksum }})\n",
        model.row_field_name
    ));
    out.push_str("}\n\n");

    out.push_str(&format!(
        "fn {}(row: &<{} as Archive>::Archived) -> {} {{\n",
        scope.fn_name("row_from_archived"),
        model.row_type,
        model.row_type
    ));
    out.push_str(&format!("    {} {{\n", model.row_type));
    for field in &model.fields {
        out.push_str(&format!(
            "        {}: {},\n",
            field.rust_name,
            archived_to_owned_access_for("row", field)
        ));
    }
    if has_presence(model) {
        if presence_is_wide(model) {
            out.push_str("        presence_bits: {\n");
            out.push_str(&format!(
                "            let mut words = [0_u64; {}];\n",
                presence_words(model)
            ));
            out.push_str("            for (idx, word) in row.presence_bits.iter().enumerate() { words[idx] = word.to_native(); }\n");
            out.push_str("            words\n");
            out.push_str("        },\n");
        } else {
            out.push_str("        presence_bits: row.presence_bits.to_native(),\n");
        }
    }
    out.push_str("    }\n");
    out.push_str("}\n");
}

fn emit_direct_projection_array_wrappers(out: &mut String, models: &[SchemaModel]) {
    // Direct projection borrows archived arrays instead of materializing row DTOs.
    if models.iter().any(|model| {
        model
            .fields
            .iter()
            .any(|field| field.kind == FieldKind::I64Array)
    }) {
        emit_direct_projection_array_wrapper(out, "DirectI64ArrayRef", "ArchivedI64", "i64");
    }
    if models.iter().any(|model| {
        model
            .fields
            .iter()
            .any(|field| field.kind == FieldKind::I32Array)
    }) {
        emit_direct_projection_array_wrapper(out, "DirectI32ArrayRef", "ArchivedI32", "i32");
    }
    if models.iter().any(|model| {
        model
            .fields
            .iter()
            .any(|field| field.kind == FieldKind::U32Array)
    }) {
        emit_direct_projection_array_wrapper(out, "DirectU32ArrayRef", "ArchivedU32", "u32");
    }
    if models.iter().any(|model| {
        model
            .fields
            .iter()
            .any(|field| field.kind == FieldKind::F64Array)
    }) {
        emit_direct_projection_array_wrapper(out, "DirectF64ArrayRef", "ArchivedF64", "f64");
    }
    if models.iter().any(|model| {
        model
            .fields
            .iter()
            .any(|field| field.kind == FieldKind::F32Array)
    }) {
        emit_direct_projection_array_wrapper(out, "DirectF32ArrayRef", "ArchivedF32", "f32");
    }
}

fn emit_direct_projection_array_wrapper(
    out: &mut String,
    wrapper: &str,
    archived: &str,
    native: &str,
) {
    out.push_str(&format!(
        "struct {wrapper}<'a> {{ values: &'a ArchivedVec<rkyv::primitive::{archived}> }}\n\n"
    ));
    out.push_str(&format!("impl<'a> Archive for {wrapper}<'a> {{\n"));
    out.push_str(&format!(
        "    type Archived = ArchivedVec<rkyv::primitive::{archived}>;\n"
    ));
    out.push_str("    type Resolver = VecResolver;\n");
    out.push_str("    fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {\n");
    out.push_str("        ArchivedVec::resolve_from_len(self.values.len(), resolver, out);\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
    out.push_str(&format!("impl<'a, S> RkyvSerialize<S> for {wrapper}<'a>\n"));
    out.push_str("where\n");
    out.push_str("    S: Fallible + Allocator + Writer + ?Sized,\n");
    out.push_str("{\n");
    out.push_str("    fn serialize(&self, serializer: &mut S) -> std::result::Result<Self::Resolver, S::Error> {\n");
    out.push_str("        let mut values = self.values.iter().map(|value| value.to_native());\n");
    out.push_str(&format!(
        "        ArchivedVec::serialize_from_unknown_length_iter::<{native}, _, _>(&mut values, serializer)\n"
    ));
    out.push_str("    }\n");
    out.push_str("}\n\n");
}

fn emit_source_projection_api(
    out: &mut String,
    source: &SchemaModel,
    projection: &ProjectionModel,
    projection_model: &SchemaModel,
) {
    let project_fn = format!("project_{}", projection.definition.name);
    let trusted_fn = format!("project_{}_trusted_unchecked", projection.definition.name);
    let helper_fn = format!("project_{}_archived_direct", projection.definition.name);
    emit_direct_projection_wrappers(out, source, projection, projection_model, &helper_fn);

    out.push_str(&format!("impl {} {{\n", source.marker_type));
    out.push_str(&format!(
        "    pub fn {project_fn}(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>> {{\n"
    ));
    out.push_str("        let archived = Self::access_archived(bytes)?;\n");
    out.push_str(&format!(
        "        {helper_fn}(archived, max_response_bytes)\n"
    ));
    out.push_str("    }\n\n");
    out.push_str("    /// Projects immutable source bytes already validated for this schema.\n");
    out.push_str("    ///\n");
    out.push_str("    /// # Safety\n");
    out.push_str(
        "    /// The caller guarantees that bytes were previously accepted by checked MBT access\n",
    );
    out.push_str(
        "    /// for the source schema and then stored or transported without mutation.\n",
    );
    out.push_str(&format!(
        "    pub unsafe fn {trusted_fn}(bytes: &[u8], max_response_bytes: usize) -> Result<Vec<u8>> {{\n"
    ));
    out.push_str(
        "        let archived = unsafe { Self::access_archived_trusted_unchecked(bytes)? };\n",
    );
    out.push_str(&format!(
        "        {helper_fn}(archived, max_response_bytes)\n"
    ));
    out.push_str("    }\n");
    out.push_str("}\n\n");

    out.push_str(&format!(
        "fn {helper_fn}(archived: &Archived{}, max_response_bytes: usize) -> Result<Vec<u8>> {{\n",
        source.payload_type
    ));
    out.push_str(&format!(
        "    let row_count = archived.{}.len() as u64;\n",
        source.row_field_name
    ));
    out.push_str(&format!(
        "    let payload = {}ProjectionPayloadRef {{\n",
        projection.marker_type
    ));
    let projection_scope = EmitScope::projection(projection_model, &projection.definition.name);
    out.push_str(&format!(
        "        schema_version: {},\n",
        projection_scope.const_name("SCHEMA_VERSION_VALUE")
    ));
    out.push_str(&format!(
        "        {}: {}ProjectionRowsRef {{ rows: &archived.{} }},\n",
        projection_model.row_field_name, projection.marker_type, source.row_field_name
    ));
    out.push_str("    };\n");
    out.push_str("    let payload_bytes = rkyv::to_bytes::<RkyvError>(&payload).map_err(|_err| TransportError::MalformedArchive(String::new()))?;\n");
    out.push_str("    let payload_len = payload_bytes.len();\n");
    out.push_str("    let total_len = HEADER_LEN.checked_add(payload_len).ok_or(TransportError::ResponseTooLarge { observed: usize::MAX, cap: max_response_bytes })?;\n");
    out.push_str("    if total_len > max_response_bytes { return Err(TransportError::ResponseTooLarge { observed: total_len, cap: max_response_bytes }); }\n");
    out.push_str(&format!(
        "    let header = TransportHeader::new_with_schema({}::header_spec(), row_count, payload_len as u64, fnv1a64(&payload_bytes));\n",
        projection.marker_type
    ));
    out.push_str("    let mut header_bytes = [0_u8; HEADER_LEN];\n");
    out.push_str("    encode_header(&header, &mut header_bytes);\n");
    out.push_str("    let mut out = Vec::with_capacity(total_len);\n");
    out.push_str("    out.extend_from_slice(&header_bytes);\n");
    out.push_str("    out.extend_from_slice(&payload_bytes);\n");
    out.push_str("    Ok(out)\n");
    out.push_str("}\n\n");
}

fn emit_direct_projection_wrappers(
    out: &mut String,
    source: &SchemaModel,
    projection: &ProjectionModel,
    projection_model: &SchemaModel,
    helper_fn: &str,
) {
    // Projection wrappers re-archive selected borrowed fields under the projection schema.
    let payload_ref = format!("{}ProjectionPayloadRef", projection.marker_type);
    let rows_ref = format!("{}ProjectionRowsRef", projection.marker_type);
    let rows_iter = format!("{}ProjectionRowsIter", projection.marker_type);
    let row_ref = format!("{}ProjectionRowRef", projection.marker_type);
    let row_ref_type = if has_direct_borrowed_fields(projection_model) {
        format!("{row_ref}<'a>")
    } else {
        row_ref.clone()
    };

    out.push_str("#[derive(Archive, RkyvSerialize)]\n");
    out.push_str(&format!("struct {payload_ref}<'a> {{\n"));
    out.push_str("    schema_version: u16,\n");
    out.push_str(&format!(
        "    {}: {rows_ref}<'a>,\n",
        projection_model.row_field_name
    ));
    out.push_str("}\n\n");

    out.push_str(&format!(
        "struct {rows_ref}<'a> {{ rows: &'a ArchivedVec<<{} as Archive>::Archived> }}\n\n",
        source.row_type
    ));
    out.push_str(&format!("impl<'a> Archive for {rows_ref}<'a> {{\n"));
    out.push_str(&format!(
        "    type Archived = ArchivedVec<<{row_ref_type} as Archive>::Archived>;\n"
    ));
    out.push_str("    type Resolver = VecResolver;\n");
    out.push_str("    fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {\n");
    out.push_str("        ArchivedVec::resolve_from_len(self.rows.len(), resolver, out);\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
    out.push_str(&format!(
        "impl<'a, S> RkyvSerialize<S> for {rows_ref}<'a>\n"
    ));
    out.push_str("where\n");
    out.push_str("    S: Fallible + Allocator + Writer + ?Sized,\n");
    out.push_str("    S::Error: Source,\n");
    out.push_str("{\n");
    out.push_str("    fn serialize(&self, serializer: &mut S) -> std::result::Result<Self::Resolver, S::Error> {\n");
    out.push_str(&format!(
        "        ArchivedVec::serialize_from_iter::<{row_ref_type}, _, _>({rows_iter} {{ inner: self.rows.as_slice().iter() }}, serializer)\n"
    ));
    out.push_str("    }\n");
    out.push_str("}\n\n");

    out.push_str("#[derive(Clone)]\n");
    out.push_str(&format!("struct {rows_iter}<'a> {{\n"));
    out.push_str(&format!(
        "    inner: core::slice::Iter<'a, <{} as Archive>::Archived>,\n",
        source.row_type
    ));
    out.push_str("}\n\n");
    out.push_str(&format!("impl<'a> Iterator for {rows_iter}<'a> {{\n"));
    out.push_str(&format!("    type Item = {row_ref_type};\n"));
    out.push_str("    fn next(&mut self) -> Option<Self::Item> {\n");
    out.push_str("        let source = self.inner.next()?;\n");
    if has_presence(projection_model) {
        out.push_str(&format!(
            "        let mut presence_bits = {};\n",
            empty_presence_expr(projection_model)
        ));
        emit_projection_presence_repack(out, source, projection, projection_model, "        ");
    }
    out.push_str(&format!("        Some({row_ref} {{\n"));
    for (field, mapping) in projection_model
        .fields
        .iter()
        .zip(projection.field_mappings.iter())
    {
        let source_field = &source.fields[mapping.source_index];
        out.push_str(&format!(
            "            {}: {},\n",
            field.rust_name,
            archived_to_direct_value(source_field, "source")
        ));
    }
    if has_presence(projection_model) {
        out.push_str("            presence_bits,\n");
    }
    out.push_str("        })\n");
    out.push_str("    }\n");
    out.push_str("    fn size_hint(&self) -> (usize, Option<usize>) { self.inner.size_hint() }\n");
    out.push_str("}\n\n");
    out.push_str(&format!(
        "impl<'a> ExactSizeIterator for {rows_iter}<'a> {{\n"
    ));
    out.push_str("    fn len(&self) -> usize { self.inner.len() }\n");
    out.push_str("}\n\n");

    out.push_str("#[derive(Archive, RkyvSerialize)]\n");
    if has_direct_borrowed_fields(projection_model) {
        out.push_str(&format!("struct {row_ref}<'a> {{\n"));
    } else {
        out.push_str(&format!("struct {row_ref} {{\n"));
    }
    for field in &projection_model.fields {
        emit_direct_projection_row_field(out, field);
    }
    if has_presence(projection_model) {
        out.push_str(&format!(
            "    presence_bits: {},\n",
            presence_storage_type(projection_model)
        ));
    }
    out.push_str("}\n\n");

    out.push_str(&format!(
        "const _: fn(&Archived{}, usize) -> Result<Vec<u8>> = {helper_fn};\n\n",
        source.payload_type
    ));
}

fn emit_projection_presence_repack(
    out: &mut String,
    source: &SchemaModel,
    projection: &ProjectionModel,
    projection_model: &SchemaModel,
    indent: &str,
) {
    // Projection presence bits are repacked from source bits into projected bit positions.
    let source_scope = EmitScope::source(source);
    let projection_scope = EmitScope::projection(projection_model, &projection.definition.name);
    for (field, mapping) in projection_model
        .fields
        .iter()
        .zip(projection.field_mappings.iter())
    {
        if mapping.projected_presence_bit.is_none() {
            continue;
        }
        let source_field = &source.fields[mapping.source_index];
        let predicate =
            archived_presence_has_expr(&source_scope, "source.presence_bits", source_field);
        if presence_is_wide(projection_model) {
            out.push_str(&format!(
                "{indent}if {predicate} {{ presence_bits[{}] |= {}; }}\n",
                presence_word_const(&projection_scope, field),
                presence_mask_const(&projection_scope, field)
            ));
        } else {
            out.push_str(&format!(
                "{indent}if {predicate} {{ presence_bits |= {}; }}\n",
                presence_const(&projection_scope, field)
            ));
        }
    }
}

fn empty_presence_expr(model: &SchemaModel) -> String {
    if presence_is_wide(model) {
        format!("[0_u64; {}]", presence_words(model))
    } else {
        "0_u64".to_string()
    }
}

fn emit_direct_projection_row_field(out: &mut String, field: &PhysicalField) {
    match field.kind {
        FieldKind::RawString => {
            out.push_str("    #[rkyv(with = rkyv::with::AsString)]\n");
            out.push_str(&format!("    {}: &'a str,\n", field.rust_name));
        }
        FieldKind::Bytes => {
            out.push_str("    #[rkyv(with = rkyv::with::AsVec)]\n");
            out.push_str(&format!("    {}: &'a [u8],\n", field.rust_name));
        }
        FieldKind::I64Array => out.push_str(&format!(
            "    {}: DirectI64ArrayRef<'a>,\n",
            field.rust_name
        )),
        FieldKind::I32Array => out.push_str(&format!(
            "    {}: DirectI32ArrayRef<'a>,\n",
            field.rust_name
        )),
        FieldKind::U32Array => out.push_str(&format!(
            "    {}: DirectU32ArrayRef<'a>,\n",
            field.rust_name
        )),
        FieldKind::F64Array => out.push_str(&format!(
            "    {}: DirectF64ArrayRef<'a>,\n",
            field.rust_name
        )),
        FieldKind::F32Array => out.push_str(&format!(
            "    {}: DirectF32ArrayRef<'a>,\n",
            field.rust_name
        )),
        _ => out.push_str(&format!(
            "    {}: {},\n",
            field.rust_name,
            rust_field_type(field)
        )),
    }
}

fn has_direct_borrowed_fields(model: &SchemaModel) -> bool {
    model.fields.iter().any(|field| {
        matches!(
            field.kind,
            FieldKind::Bytes
                | FieldKind::RawString
                | FieldKind::I64Array
                | FieldKind::I32Array
                | FieldKind::U32Array
                | FieldKind::F64Array
                | FieldKind::F32Array
        )
    })
}

fn archived_to_direct_value(field: &PhysicalField, row: &str) -> String {
    let access = format!("{row}.{}", field.rust_name);
    match field.kind {
        FieldKind::ConstU16 { .. }
        | FieldKind::U16Dictionary { .. }
        | FieldKind::U64BitmaskDictionary { .. }
        | FieldKind::I32
        | FieldKind::U32
        | FieldKind::I64
        | FieldKind::F32
        | FieldKind::F64 => format!("{access}.to_native()"),
        FieldKind::Bool => access,
        FieldKind::Bytes => format!("{access}.as_slice()"),
        FieldKind::RawString => format!("{access}.as_str()"),
        FieldKind::I64Array => format!("DirectI64ArrayRef {{ values: &{access} }}"),
        FieldKind::I32Array => format!("DirectI32ArrayRef {{ values: &{access} }}"),
        FieldKind::U32Array => format!("DirectU32ArrayRef {{ values: &{access} }}"),
        FieldKind::F64Array => format!("DirectF64ArrayRef {{ values: &{access} }}"),
        FieldKind::F32Array => format!("DirectF32ArrayRef {{ values: &{access} }}"),
    }
}

fn presence_fields(model: &SchemaModel) -> Vec<&PhysicalField> {
    let mut fields: Vec<&PhysicalField> = model
        .fields
        .iter()
        .filter(|field| field.presence_bit.is_some())
        .collect();
    fields.sort_by_key(|field| field.presence_bit);
    fields
}

fn presence_words(model: &SchemaModel) -> usize {
    let count = presence_fields(model).len();
    if count == 0 { 0 } else { count.div_ceil(64) }
}

fn presence_is_wide(model: &SchemaModel) -> bool {
    presence_fields(model).len() > 64
}

fn presence_word(bit: u32) -> usize {
    (bit / 64) as usize
}

fn presence_mask(bit: u32) -> u64 {
    1_u64 << (bit % 64)
}

fn presence_allowed_masks(model: &SchemaModel) -> Vec<u64> {
    let mut masks = vec![0_u64; presence_words(model)];
    for field in presence_fields(model) {
        if let Some(bit) = field.presence_bit {
            masks[presence_word(bit)] |= presence_mask(bit);
        }
    }
    masks
}

fn presence_storage_type(model: &SchemaModel) -> String {
    if presence_is_wide(model) {
        format!("[u64; {}]", presence_words(model))
    } else {
        "u64".to_string()
    }
}

fn presence_const(scope: &EmitScope<'_>, field: &PhysicalField) -> String {
    scope.const_name(&format!("PRESENCE_{}", const_name(&field.rust_name)))
}

fn presence_word_const(scope: &EmitScope<'_>, field: &PhysicalField) -> String {
    format!("{}_WORD", presence_const(scope, field))
}

fn presence_mask_const(scope: &EmitScope<'_>, field: &PhysicalField) -> String {
    format!("{}_MASK", presence_const(scope, field))
}

fn owned_presence_absent_expr(
    scope: &EmitScope<'_>,
    bits_expr: &str,
    field: &PhysicalField,
) -> String {
    let model = scope.model;
    if presence_is_wide(model) {
        format!(
            "{bits_expr}[{}] & {} == 0",
            presence_word_const(scope, field),
            presence_mask_const(scope, field)
        )
    } else {
        format!("{bits_expr} & {} == 0", presence_const(scope, field))
    }
}

fn archived_presence_has_expr(
    scope: &EmitScope<'_>,
    bits_expr: &str,
    field: &PhysicalField,
) -> String {
    let model = scope.model;
    if presence_is_wide(model) {
        format!(
            "{bits_expr}[{}].to_native() & {} != 0",
            presence_word_const(scope, field),
            presence_mask_const(scope, field)
        )
    } else {
        format!(
            "{bits_expr}.to_native() & {} != 0",
            presence_const(scope, field)
        )
    }
}

fn presence_error_expr(scope: &EmitScope<'_>, bits_expr: &str, field: &PhysicalField) -> String {
    let model = scope.model;
    if presence_is_wide(model) {
        format!(
            "TransportError::InvalidPresenceWord {{ word: {}, value: {bits_expr}[{}] }}",
            presence_word_const(scope, field),
            presence_word_const(scope, field)
        )
    } else {
        format!("TransportError::InvalidPresenceBits({bits_expr})")
    }
}

fn has_presence(model: &SchemaModel) -> bool {
    model
        .fields
        .iter()
        .any(|field| field.presence_bit.is_some())
}

fn rust_field_type(field: &PhysicalField) -> &'static str {
    match field.kind {
        FieldKind::ConstU16 { .. } | FieldKind::U16Dictionary { .. } => "u16",
        FieldKind::U64BitmaskDictionary { .. } => "u64",
        FieldKind::I32 => "i32",
        FieldKind::U32 => "u32",
        FieldKind::I64 => "i64",
        FieldKind::F32 => "f32",
        FieldKind::F64 => "f64",
        FieldKind::Bool => "bool",
        FieldKind::Bytes => "Vec<u8>",
        FieldKind::RawString => "String",
        FieldKind::I64Array => "Vec<i64>",
        FieldKind::I32Array => "Vec<i32>",
        FieldKind::U32Array => "Vec<u32>",
        FieldKind::F64Array => "Vec<f64>",
        FieldKind::F32Array => "Vec<f32>",
    }
}

fn archived_getter_return_type(field: &PhysicalField) -> &'static str {
    match field.kind {
        FieldKind::ConstU16 { .. } | FieldKind::U16Dictionary { .. } => "u16",
        FieldKind::U64BitmaskDictionary { .. } => "u64",
        FieldKind::I32 => "i32",
        FieldKind::U32 => "u32",
        FieldKind::I64 => "i64",
        FieldKind::F32 => "f32",
        FieldKind::F64 => "f64",
        FieldKind::Bool => "bool",
        FieldKind::Bytes => "&'a [u8]",
        FieldKind::RawString => "&'a str",
        FieldKind::I64Array => "impl Iterator<Item = i64> + '_",
        FieldKind::I32Array => "impl Iterator<Item = i32> + '_",
        FieldKind::U32Array => "impl Iterator<Item = u32> + '_",
        FieldKind::F64Array => "impl Iterator<Item = f64> + '_",
        FieldKind::F32Array => "impl Iterator<Item = f32> + '_",
    }
}

fn archived_value_access(field: &PhysicalField) -> String {
    let access = format!("self.row.{}", field.rust_name);
    match field.kind {
        FieldKind::ConstU16 { .. }
        | FieldKind::U16Dictionary { .. }
        | FieldKind::U64BitmaskDictionary { .. }
        | FieldKind::I32
        | FieldKind::U32
        | FieldKind::I64
        | FieldKind::F32
        | FieldKind::F64 => format!("{access}.to_native()"),
        FieldKind::Bool => access,
        FieldKind::Bytes => format!("{access}.as_slice()"),
        FieldKind::RawString => format!("{access}.as_str()"),
        FieldKind::I64Array
        | FieldKind::I32Array
        | FieldKind::U32Array
        | FieldKind::F64Array
        | FieldKind::F32Array => format!("{access}.iter().map(|value| value.to_native())"),
    }
}

fn dictionary_is_used(model: &SchemaModel, dictionary: &str) -> bool {
    model.fields.iter().any(|field| match &field.kind {
        FieldKind::U16Dictionary {
            dictionary: used, ..
        }
        | FieldKind::U64BitmaskDictionary { dictionary: used } => used == dictionary,
        _ => false,
    })
}

fn dictionary_is_key(model: &SchemaModel, dictionary: &str) -> bool {
    model.fields.iter().any(|field| {
        field.key_order.is_some()
            && matches!(&field.kind, FieldKind::U16Dictionary { dictionary: used, .. } if used == dictionary)
    })
}

fn dictionary_is_bitmask(model: &SchemaModel, dictionary: &str) -> bool {
    model
        .fields
        .iter()
        .any(|field| matches!(&field.kind, FieldKind::U64BitmaskDictionary { dictionary: used } if used == dictionary))
}

fn dictionary_helper_stem(model: &SchemaModel, dictionary: &str) -> String {
    model
        .fields
        .iter()
        .find_map(|field| match &field.kind {
            FieldKind::U16Dictionary {
                dictionary: used, ..
            } if used == dictionary => Some(field.proto_name.clone()),
            _ => None,
        })
        .map_or_else(|| dictionary.to_string(), |value| value)
}
