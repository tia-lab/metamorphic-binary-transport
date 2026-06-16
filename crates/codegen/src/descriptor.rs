use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use prost_reflect::{Cardinality, DescriptorPool, FieldDescriptor, Kind, MessageDescriptor, Value};

use crate::config::SchemaRequest;
use crate::error::{CodegenError, Result};
use crate::model::{
    DerivedUtcField, Dictionary, FieldKind, JsonCsvOutputField, KeyPart, PhysicalField,
    ProjectionDefinitionModel, ProjectionFieldMapping, ProjectionModel, ProtobufMessageModel,
    ProtobufOutputField, SchemaModel, field_kind_hash_name, module_marker_type,
    payload_type_from_root, rust_type_name,
};
use crate::options::{
    MbtExtensions, dictionary_from_value, extensions, optional_bool, optional_string, optional_u32,
    required_bool, required_string, required_u32,
};

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn load_schema_model(request: &SchemaRequest) -> Result<SchemaModel> {
    // Descriptor options are normalized into one SchemaModel before emission.
    let descriptor_set = raw_descriptor_set(request)?;
    let pool = DescriptorPool::decode(descriptor_set.as_slice())
        .map_err(|err| CodegenError::Descriptor(err.to_string()))?;
    let extensions = extensions(&pool)?;
    let root = pool
        .get_message_by_name(&request.root)
        .ok_or_else(|| CodegenError::InvalidSchema(format!("missing root {}", request.root)))?;
    let source_file = source_file_name(request)?;
    let file = pool
        .get_file_by_name(&source_file)
        .ok_or_else(|| CodegenError::InvalidSchema(format!("missing descriptor {source_file}")))?;
    let dictionaries = dictionaries_from_file(&file.options(), &extensions)?;
    validate_dictionaries(&dictionaries)?;

    let root_options = root.options();
    let schema_id = required_u32(&root_options, &extensions.schema_id)?;
    let schema_version = required_u32(&root_options, &extensions.schema_version)?;
    let schema_version_value = u16::try_from(schema_version).map_err(|_| {
        CodegenError::InvalidSchema(format!("schema_version {schema_version} exceeds u16"))
    })?;
    let transport_name = required_string(&root_options, &extensions.transport_name)?;
    let payload_root = required_bool(&root_options, &extensions.payload_root)?;
    if !payload_root {
        return Err(CodegenError::InvalidSchema(
            "root message is not marked payload_root".to_string(),
        ));
    }

    let row_payload = row_payload_field(&root, &extensions)?;
    let row_type = rust_type_name(row_payload.message.full_name());
    let root_type = rust_type_name(root.full_name());
    let payload_type = payload_type_from_root(&root_type);
    let marker_type = module_marker_type(&request.module);
    let mut fields = Vec::new();
    let mut derived_candidates = Vec::new();
    let output_tree = {
        let mut traversal = SchemaTraversal {
            extensions: &extensions,
            dictionaries: &dictionaries,
            fields: &mut fields,
            derived_fields: &mut derived_candidates,
        };
        collect_schema_fields(
            &mut traversal,
            &row_payload.message,
            "",
            row_payload.message.full_name(),
            "row",
            None,
            None,
        )?
    };
    validate_physical_fields(&fields)?;
    let derived_utc_fields = resolve_derived_utc_fields(&fields, derived_candidates)?;
    let json_csv_output_fields = json_csv_outputs(&output_tree);
    let protobuf_messages = protobuf_outputs(&output_tree);
    validate_row_format_outputs(
        &fields,
        &derived_utc_fields,
        &json_csv_output_fields,
        &protobuf_messages,
    )?;
    let key_parts = key_parts(&fields);
    let mut model = SchemaModel {
        module: request.module.clone(),
        proto: request.schema.clone(),
        root: request.root.clone(),
        root_type,
        payload_type,
        row_type,
        marker_type: marker_type.clone(),
        view_type: format!("{marker_type}View"),
        rows_iter_type: format!("{marker_type}Rows"),
        archived_row_type: format!("Archived{marker_type}Row"),
        schema_id,
        schema_version,
        schema_version_value,
        transport_name,
        payload_root,
        row_field_name: row_payload.field.name().to_string(),
        row_field_number: row_payload.field.number(),
        dictionaries,
        fields,
        derived_utc_fields,
        json_csv_output_fields,
        protobuf_messages,
        key_parts,
        normalized_schema_hash: 0,
        projections: Vec::new(),
    };
    model.normalized_schema_hash = normalized_hash(&model);
    model.projections = projection_models(&root_options, &extensions, &model)?;
    Ok(model)
}

struct RowPayload {
    field: FieldDescriptor,
    message: MessageDescriptor,
}

fn row_payload_field(root: &MessageDescriptor, extensions: &MbtExtensions) -> Result<RowPayload> {
    // A transport root must expose exactly one repeated row payload.
    let mut found = None;
    for field in root.fields() {
        if optional_bool(&field.options(), &extensions.repeated_payload)? {
            if found.is_some() {
                return Err(CodegenError::InvalidSchema(
                    "multiple repeated_payload fields".to_string(),
                ));
            }
            if field.cardinality() != Cardinality::Repeated {
                return Err(CodegenError::InvalidSchema(format!(
                    "{} is repeated_payload but not repeated",
                    field.full_name()
                )));
            }
            let Kind::Message(message) = field.kind() else {
                return Err(CodegenError::InvalidSchema(format!(
                    "{} is repeated_payload but not message",
                    field.full_name()
                )));
            };
            found = Some(RowPayload { field, message });
        }
    }
    found.ok_or_else(|| CodegenError::InvalidSchema("missing repeated_payload field".to_string()))
}

struct OutputMessageNode {
    logical_path: String,
    proto_path: String,
    rust_helper_stem: String,
    enclosing_proto_number: Option<u32>,
    items: Vec<OutputNodeItem>,
}

enum OutputNodeItem {
    Physical { field_index: usize },
    DerivedUtc { candidate_index: usize },
    Message(OutputMessageNode),
}

struct DerivedUtcCandidate {
    proto_path: String,
    logical_path: String,
    parent_proto_path: String,
    parent_logical_path: String,
    proto_name: String,
    rust_name: String,
    proto_number: u32,
    source: String,
}

struct SchemaTraversal<'a> {
    extensions: &'a MbtExtensions,
    dictionaries: &'a [Dictionary],
    fields: &'a mut Vec<PhysicalField>,
    derived_fields: &'a mut Vec<DerivedUtcCandidate>,
}

fn collect_schema_fields(
    traversal: &mut SchemaTraversal<'_>,
    message: &MessageDescriptor,
    prefix: &str,
    proto_path: &str,
    rust_helper_stem: &str,
    enclosing_proto_number: Option<u32>,
    inherited_projection_group: Option<&str>,
) -> Result<OutputMessageNode> {
    // Traverse protobuf messages into physical fields and row-format output nodes.
    let mut node = OutputMessageNode {
        logical_path: prefix.to_string(),
        proto_path: proto_path.to_string(),
        rust_helper_stem: rust_helper_stem.to_string(),
        enclosing_proto_number,
        items: Vec::new(),
    };
    for field in message.fields() {
        if optional_bool(&field.options(), &traversal.extensions.repeated_payload)? {
            return Err(CodegenError::InvalidSchema(format!(
                "{} repeated_payload is only valid on payload root",
                field.full_name()
            )));
        }
        let options = field.options();
        let ignored = optional_bool(&options, &traversal.extensions.ignored)?;
        let derived_utc_from = optional_string(&options, &traversal.extensions.derived_utc_from)?;
        if derived_utc_from.is_some() && !ignored {
            return Err(CodegenError::InvalidSchema(format!(
                "{} uses derived_utc_from without ignored=true",
                field.full_name()
            )));
        }
        if ignored {
            if let Some(source) = derived_utc_from {
                let candidate_index = traversal.derived_fields.len();
                traversal
                    .derived_fields
                    .push(derived_utc_candidate(&field, prefix, proto_path, source)?);
                node.items
                    .push(OutputNodeItem::DerivedUtc { candidate_index });
            }
            continue;
        }
        let projection_group = optional_string(&options, &traversal.extensions.projection_group)?
            .or_else(|| inherited_projection_group.map(str::to_string));
        let logical_path = join_path(prefix, field.name());
        match field.kind() {
            Kind::Message(child) if field.cardinality() != Cardinality::Repeated => {
                let child_node = collect_schema_fields(
                    traversal,
                    &child,
                    &logical_path,
                    child.full_name(),
                    &message_helper_stem(&logical_path),
                    Some(field.number()),
                    projection_group.as_deref(),
                )?;
                if !child_node.items.is_empty() {
                    node.items.push(OutputNodeItem::Message(child_node));
                }
            }
            _ => {
                let field_index = traversal.fields.len();
                traversal.fields.push(physical_field(
                    &field,
                    traversal.extensions,
                    traversal.dictionaries,
                    &logical_path,
                    projection_group,
                )?);
                node.items.push(OutputNodeItem::Physical { field_index });
            }
        }
    }
    Ok(node)
}

fn derived_utc_candidate(
    field: &FieldDescriptor,
    parent_logical_path: &str,
    parent_proto_path: &str,
    source: String,
) -> Result<DerivedUtcCandidate> {
    if field.cardinality() == Cardinality::Repeated {
        return Err(CodegenError::InvalidSchema(format!(
            "{} derived UTC field cannot be repeated",
            field.full_name()
        )));
    }
    if !matches!(field.kind(), Kind::String) {
        return Err(CodegenError::InvalidSchema(format!(
            "{} derived UTC field must be string",
            field.full_name()
        )));
    }
    if source.is_empty() {
        return Err(CodegenError::InvalidOption {
            name: "derived_utc_from",
            reason: format!("{} has empty source", field.full_name()),
        });
    }
    Ok(DerivedUtcCandidate {
        proto_path: field.full_name().to_string(),
        logical_path: join_path(parent_logical_path, field.name()),
        parent_proto_path: parent_proto_path.to_string(),
        parent_logical_path: parent_logical_path.to_string(),
        proto_name: field.name().to_string(),
        rust_name: field.name().to_string(),
        proto_number: field.number(),
        source,
    })
}

fn resolve_derived_utc_fields(
    fields: &[PhysicalField],
    candidates: Vec<DerivedUtcCandidate>,
) -> Result<Vec<DerivedUtcField>> {
    // Derived UTC fields bind to one validated i64 source field.
    let mut out = Vec::with_capacity(candidates.len());
    for candidate in candidates {
        let source_logical_path = if candidate.source.contains('.') {
            candidate.source.clone()
        } else {
            join_path(&candidate.parent_logical_path, &candidate.source)
        };
        let matches = fields
            .iter()
            .enumerate()
            .filter_map(|(idx, field)| (field.logical_path == source_logical_path).then_some(idx))
            .collect::<Vec<_>>();
        let source_field_index = match matches.as_slice() {
            [idx] => *idx,
            [] => {
                return Err(CodegenError::InvalidOption {
                    name: "derived_utc_from",
                    reason: format!(
                        "{} references unknown source {}",
                        candidate.proto_path, candidate.source
                    ),
                });
            }
            _ => {
                return Err(CodegenError::InvalidOption {
                    name: "derived_utc_from",
                    reason: format!(
                        "{} references ambiguous source {}",
                        candidate.proto_path, candidate.source
                    ),
                });
            }
        };
        let source = &fields[source_field_index];
        if !matches!(source.kind, FieldKind::I64) {
            return Err(CodegenError::InvalidOption {
                name: "derived_utc_from",
                reason: format!(
                    "{} source {} is not an i64 field",
                    candidate.proto_path, source.logical_path
                ),
            });
        }
        out.push(DerivedUtcField {
            proto_path: candidate.proto_path,
            logical_path: candidate.logical_path,
            parent_proto_path: candidate.parent_proto_path,
            parent_logical_path: candidate.parent_logical_path,
            proto_name: candidate.proto_name,
            rust_name: candidate.rust_name,
            proto_number: candidate.proto_number,
            source_field_index,
            source_logical_path: source.logical_path.clone(),
            source_rust_name: source.rust_name.clone(),
            source_presence_bit: source.presence_bit,
        });
    }
    Ok(out)
}

fn json_csv_outputs(node: &OutputMessageNode) -> Vec<JsonCsvOutputField> {
    let mut out = Vec::new();
    push_json_csv_outputs(node, &mut out);
    out
}

fn push_json_csv_outputs(node: &OutputMessageNode, out: &mut Vec<JsonCsvOutputField>) {
    for item in &node.items {
        match item {
            OutputNodeItem::Physical { field_index } => {
                out.push(JsonCsvOutputField::Physical {
                    field_index: *field_index,
                });
            }
            OutputNodeItem::DerivedUtc { candidate_index } => {
                out.push(JsonCsvOutputField::DerivedUtc {
                    derived_index: *candidate_index,
                });
            }
            OutputNodeItem::Message(child) => push_json_csv_outputs(child, out),
        }
    }
}

fn protobuf_outputs(node: &OutputMessageNode) -> Vec<ProtobufMessageModel> {
    let mut out = Vec::new();
    push_protobuf_message(node, &mut out);
    out
}

fn push_protobuf_message(node: &OutputMessageNode, out: &mut Vec<ProtobufMessageModel>) -> usize {
    let message_index = out.len();
    out.push(ProtobufMessageModel {
        logical_path: node.logical_path.clone(),
        proto_path: node.proto_path.clone(),
        rust_helper_stem: node.rust_helper_stem.clone(),
        enclosing_proto_number: node.enclosing_proto_number,
        fields: Vec::new(),
    });

    let mut fields = Vec::with_capacity(node.items.len());
    for item in &node.items {
        match item {
            OutputNodeItem::Physical { field_index } => {
                fields.push(ProtobufOutputField::Physical {
                    field_index: *field_index,
                });
            }
            OutputNodeItem::DerivedUtc { candidate_index } => {
                fields.push(ProtobufOutputField::DerivedUtc {
                    derived_index: *candidate_index,
                });
            }
            OutputNodeItem::Message(child) => {
                let child_index = push_protobuf_message(child, out);
                fields.push(ProtobufOutputField::Message {
                    message_index: child_index,
                });
            }
        }
    }
    out[message_index].fields = fields;
    message_index
}

fn message_helper_stem(logical_path: &str) -> String {
    if logical_path.is_empty() {
        return "row".to_string();
    }
    let mut out = String::new();
    let mut previous_underscore = false;
    for ch in logical_path.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            previous_underscore = false;
        } else if !previous_underscore {
            out.push('_');
            previous_underscore = true;
        }
    }
    while out.ends_with('_') {
        out.pop();
    }
    if out.is_empty() {
        "message".to_string()
    } else {
        out
    }
}

fn validate_row_format_outputs(
    fields: &[PhysicalField],
    derived_fields: &[DerivedUtcField],
    json_csv_fields: &[JsonCsvOutputField],
    protobuf_messages: &[ProtobufMessageModel],
) -> Result<()> {
    // Row-format models are checked before any adapter emitter can use them.
    validate_json_csv_outputs(fields, derived_fields, json_csv_fields)?;
    validate_protobuf_outputs(fields, derived_fields, protobuf_messages)
}

fn validate_json_csv_outputs(
    fields: &[PhysicalField],
    derived_fields: &[DerivedUtcField],
    json_csv_fields: &[JsonCsvOutputField],
) -> Result<()> {
    let mut names = Vec::with_capacity(json_csv_fields.len());
    for output in json_csv_fields {
        let name = match output {
            JsonCsvOutputField::Physical { field_index } => &fields[*field_index].logical_path,
            JsonCsvOutputField::DerivedUtc { derived_index } => {
                &derived_fields[*derived_index].logical_path
            }
        };
        if names.iter().any(|seen| seen == name) {
            return Err(CodegenError::InvalidSchema(format!(
                "duplicate row-format field {name}"
            )));
        }
        names.push(name.clone());
    }
    Ok(())
}

fn validate_protobuf_outputs(
    fields: &[PhysicalField],
    derived_fields: &[DerivedUtcField],
    protobuf_messages: &[ProtobufMessageModel],
) -> Result<()> {
    if protobuf_messages.is_empty() {
        return Err(CodegenError::InvalidSchema(
            "schema has no protobuf output messages".to_string(),
        ));
    }
    let mut helper_stems = Vec::with_capacity(protobuf_messages.len());
    for message in protobuf_messages {
        if helper_stems
            .iter()
            .any(|stem| stem == &message.rust_helper_stem)
        {
            return Err(CodegenError::InvalidSchema(format!(
                "duplicate protobuf helper stem {}",
                message.rust_helper_stem
            )));
        }
        helper_stems.push(message.rust_helper_stem.clone());

        let mut tags = Vec::with_capacity(message.fields.len());
        for output in &message.fields {
            let tag = match output {
                ProtobufOutputField::Physical { field_index } => fields[*field_index].proto_number,
                ProtobufOutputField::DerivedUtc { derived_index } => {
                    derived_fields[*derived_index].proto_number
                }
                ProtobufOutputField::Message { message_index } => protobuf_messages[*message_index]
                    .enclosing_proto_number
                    .ok_or_else(|| {
                        CodegenError::InvalidSchema(format!(
                            "protobuf output message {} is nested without tag",
                            protobuf_messages[*message_index].logical_path
                        ))
                    })?,
            };
            if tags.contains(&tag) {
                return Err(CodegenError::InvalidSchema(format!(
                    "duplicate protobuf tag {} in {}",
                    tag, message.proto_path
                )));
            }
            tags.push(tag);
        }
    }
    Ok(())
}

fn physical_field(
    field: &FieldDescriptor,
    extensions: &MbtExtensions,
    dictionaries: &[Dictionary],
    logical_path: &str,
    projection_group: Option<String>,
) -> Result<PhysicalField> {
    // MBT physical annotations are mapped here; unannotated strings are rejected.
    let options = field.options();
    let dictionary = optional_string(&options, &extensions.dictionary)?;
    let bitmask_dictionary = optional_string(&options, &extensions.bitmask_dictionary)?;
    let presence_bit = optional_u32(&options, &extensions.presence_bit)?;
    let key_part = optional_bool(&options, &extensions.key_part)?;
    let key_order = optional_u32(&options, &extensions.key_order)?;
    let const_u16 = optional_u32(&options, &extensions.const_u16)?;
    let raw_string = optional_bool(&options, &extensions.raw_string)?;
    let _projection = &extensions.projection;
    let _derived_utc_from = &extensions.derived_utc_from;

    if raw_string
        && (dictionary.is_some()
            || bitmask_dictionary.is_some()
            || key_part
            || key_order.is_some()
            || const_u16.is_some()
            || field.is_list())
    {
        return Err(CodegenError::InvalidSchema(format!(
            "{} mixes raw_string with another MBT physical annotation",
            field.full_name()
        )));
    }
    if field.supports_presence() && presence_bit.is_none() {
        return Err(CodegenError::MissingOption("presence_bit"));
    }
    if bitmask_dictionary.is_some() && presence_bit.is_some() {
        return Err(CodegenError::InvalidSchema(format!(
            "{} nullable bitmask_dictionary is not supported",
            field.full_name()
        )));
    }
    if key_part && key_order.is_none() {
        return Err(CodegenError::MissingOption("key_order"));
    }

    let kind = match field.kind() {
        Kind::Uint32 if const_u16.is_some() => {
            let value = const_u16.ok_or(CodegenError::MissingOption("const_u16"))?;
            FieldKind::ConstU16 {
                value: u16::try_from(value).map_err(|_| CodegenError::InvalidOption {
                    name: "const_u16",
                    reason: format!("{} exceeds u16", field.full_name()),
                })?,
            }
        }
        Kind::String if bitmask_dictionary.is_some() => {
            if !field.is_list() {
                return Err(CodegenError::InvalidSchema(format!(
                    "{} bitmask_dictionary requires repeated string",
                    field.full_name()
                )));
            }
            let dictionary =
                bitmask_dictionary.ok_or(CodegenError::MissingOption("bitmask_dictionary"))?;
            let dict = require_dictionary(&dictionary, dictionaries)?;
            if dict.values.len() > 64 {
                return Err(CodegenError::InvalidOption {
                    name: "bitmask_dictionary",
                    reason: format!("dictionary {dictionary} has more than 64 values"),
                });
            }
            FieldKind::U64BitmaskDictionary { dictionary }
        }
        Kind::String if dictionary.is_some() => {
            let dictionary = dictionary.ok_or(CodegenError::MissingOption("dictionary"))?;
            require_dictionary(&dictionary, dictionaries)?;
            FieldKind::U16Dictionary {
                dictionary,
                optional: presence_bit.is_some(),
            }
        }
        Kind::String if raw_string => FieldKind::RawString,
        Kind::String => {
            return Err(CodegenError::InvalidSchema(format!(
                "{} is an unannotated string",
                field.full_name()
            )));
        }
        Kind::Int32 if field.is_list() => FieldKind::I32Array,
        Kind::Uint32 if field.is_list() => FieldKind::U32Array,
        Kind::Int64 if field.is_list() => FieldKind::I64Array,
        Kind::Float if field.is_list() => FieldKind::F32Array,
        Kind::Double if field.is_list() => FieldKind::F64Array,
        Kind::Bool if field.is_list() => {
            return Err(CodegenError::InvalidSchema(format!(
                "{} repeated bool is not supported",
                field.full_name()
            )));
        }
        Kind::Bytes if field.is_list() => {
            return Err(CodegenError::InvalidSchema(format!(
                "{} repeated bytes is not supported",
                field.full_name()
            )));
        }
        Kind::Int32 => FieldKind::I32,
        Kind::Uint32 => FieldKind::U32,
        Kind::Int64 => FieldKind::I64,
        Kind::Float => FieldKind::F32,
        Kind::Double => FieldKind::F64,
        Kind::Bool => FieldKind::Bool,
        Kind::Bytes => FieldKind::Bytes,
        other => {
            return Err(CodegenError::InvalidSchema(format!(
                "{} has unsupported kind {other:?}",
                field.full_name()
            )));
        }
    };

    let rust_name = rust_field_name(field.name(), &kind);
    Ok(PhysicalField {
        proto_path: field.full_name().to_string(),
        logical_path: logical_path.to_string(),
        proto_name: field.name().to_string(),
        rust_name,
        proto_number: field.number(),
        kind,
        presence_bit,
        key_order,
        projection_group,
    })
}

fn rust_field_name(name: &str, kind: &FieldKind) -> String {
    match kind {
        FieldKind::U16Dictionary { .. } => format!("{name}_ordinal"),
        FieldKind::U64BitmaskDictionary { .. } => format!("{name}_mask"),
        _ => name.to_string(),
    }
}

fn join_path(prefix: &str, name: &str) -> String {
    if prefix.is_empty() {
        name.to_string()
    } else {
        format!("{prefix}.{name}")
    }
}

fn dictionaries_from_file(
    file_options: &prost_reflect::DynamicMessage,
    extensions: &MbtExtensions,
) -> Result<Vec<Dictionary>> {
    // File-level dictionary options are shared by dictionary and bitmask fields.
    if !file_options.has_extension(&extensions.dictionary_values) {
        return Ok(Vec::new());
    }
    match &*file_options.get_extension(&extensions.dictionary_values) {
        Value::List(values) => {
            let mut out = Vec::with_capacity(values.len());
            for value in values {
                out.push(dictionary_from_value(value)?);
            }
            Ok(out)
        }
        other => Err(CodegenError::InvalidOption {
            name: "dictionary_values",
            reason: format!("unexpected value {other:?}"),
        }),
    }
}

fn validate_dictionaries(dictionaries: &[Dictionary]) -> Result<()> {
    let mut names = Vec::new();
    for dictionary in dictionaries {
        if dictionary.name.is_empty() {
            return Err(CodegenError::InvalidOption {
                name: "dictionary_values.name",
                reason: "empty dictionary name".to_string(),
            });
        }
        if names.iter().any(|name| name == &dictionary.name) {
            return Err(CodegenError::InvalidOption {
                name: "dictionary_values.name",
                reason: format!("duplicate dictionary {}", dictionary.name),
            });
        }
        names.push(dictionary.name.clone());

        let mut values = Vec::new();
        for value in &dictionary.values {
            if values.iter().any(|seen| seen == value) {
                return Err(CodegenError::InvalidOption {
                    name: "dictionary_values.value",
                    reason: format!("duplicate dictionary value {value}"),
                });
            }
            values.push(value.clone());
        }
    }
    Ok(())
}

fn validate_physical_fields(fields: &[PhysicalField]) -> Result<()> {
    // Field validation enforces unique names, dense presence bits, and dense key order.
    if fields.is_empty() {
        return Err(CodegenError::InvalidSchema(
            "schema has no physical fields".to_string(),
        ));
    }
    let mut presence_bits = Vec::new();
    let mut key_orders = Vec::new();
    let mut rust_names = Vec::new();
    for field in fields {
        if rust_names.iter().any(|name| name == &field.rust_name) {
            return Err(CodegenError::InvalidSchema(format!(
                "duplicate generated field {}",
                field.rust_name
            )));
        }
        rust_names.push(field.rust_name.clone());
        if let Some(bit) = field.presence_bit {
            if presence_bits.contains(&bit) {
                return Err(CodegenError::InvalidOption {
                    name: "presence_bit",
                    reason: format!("duplicate bit {bit}"),
                });
            }
            presence_bits.push(bit);
        }
        if let Some(order) = field.key_order {
            if key_orders.contains(&order) {
                return Err(CodegenError::InvalidOption {
                    name: "key_order",
                    reason: format!("duplicate key order {order}"),
                });
            }
            key_orders.push(order);
        }
    }
    presence_bits.sort_unstable();
    for (idx, bit) in presence_bits.iter().enumerate() {
        let expected = u32::try_from(idx)
            .map_err(|err| CodegenError::InvalidSchema(format!("presence overflow: {err}")))?;
        if *bit != expected {
            return Err(CodegenError::InvalidSchema(format!(
                "presence bit gap: expected {expected}, observed {bit}"
            )));
        }
    }
    key_orders.sort_unstable();
    for (idx, order) in key_orders.iter().enumerate() {
        let expected = u32::try_from(idx + 1)
            .map_err(|err| CodegenError::InvalidSchema(format!("key order overflow: {err}")))?;
        if *order != expected {
            return Err(CodegenError::InvalidSchema(format!(
                "key order gap: expected {expected}, observed {order}"
            )));
        }
    }
    Ok(())
}

fn key_parts(fields: &[PhysicalField]) -> Vec<KeyPart> {
    let mut out: Vec<KeyPart> = fields
        .iter()
        .filter_map(|field| {
            field.key_order.map(|order| KeyPart {
                order,
                rust_name: field.rust_name.clone(),
            })
        })
        .collect();
    out.sort_by_key(|part| part.order);
    out
}

fn projection_models(
    root_options: &prost_reflect::DynamicMessage,
    extensions: &MbtExtensions,
    model: &SchemaModel,
) -> Result<Vec<ProjectionModel>> {
    // Root projection options become independent generated projection schema models.
    let definitions = projection_definitions(root_options, extensions)?;
    validate_projection_definitions(&definitions)?;
    let mut projections = Vec::with_capacity(definitions.len());
    for definition in definitions {
        projections.push(build_projection_model(model, definition)?);
    }
    Ok(projections)
}

fn projection_definitions(
    root_options: &prost_reflect::DynamicMessage,
    extensions: &MbtExtensions,
) -> Result<Vec<ProjectionDefinitionModel>> {
    if !root_options.has_extension(&extensions.projection) {
        return Ok(Vec::new());
    }
    match &*root_options.get_extension(&extensions.projection) {
        Value::List(values) => {
            let mut out = Vec::with_capacity(values.len());
            for value in values {
                out.push(projection_definition_from_value(value)?);
            }
            Ok(out)
        }
        other => Err(CodegenError::InvalidOption {
            name: "projection",
            reason: format!("unexpected value {other:?}"),
        }),
    }
}

fn projection_definition_from_value(value: &Value) -> Result<ProjectionDefinitionModel> {
    let Value::Message(message) = value else {
        return Err(CodegenError::InvalidOption {
            name: "projection",
            reason: format!("unexpected value {value:?}"),
        });
    };
    Ok(ProjectionDefinitionModel {
        name: required_projection_string(message, "name")?,
        rust_marker: required_projection_string(message, "rust_marker")?,
        include_groups: projection_string_list(message, "include_group")?,
        exclude_groups: projection_string_list(message, "exclude_group")?,
        include_fields: projection_string_list(message, "include_field")?,
        exclude_fields: projection_string_list(message, "exclude_field")?,
    })
}

fn required_projection_string(
    message: &prost_reflect::DynamicMessage,
    name: &'static str,
) -> Result<String> {
    let Some(value) = message.get_field_by_name(name) else {
        return Err(CodegenError::MissingOption(name));
    };
    match &*value {
        Value::String(value) if !value.is_empty() => Ok(value.clone()),
        Value::String(_) => Err(CodegenError::InvalidOption {
            name,
            reason: "empty string".to_string(),
        }),
        other => Err(CodegenError::InvalidOption {
            name,
            reason: format!("unexpected value {other:?}"),
        }),
    }
}

fn projection_string_list(
    message: &prost_reflect::DynamicMessage,
    name: &'static str,
) -> Result<Vec<String>> {
    let Some(value) = message.get_field_by_name(name) else {
        return Ok(Vec::new());
    };
    match &*value {
        Value::List(values) => {
            let mut out = Vec::with_capacity(values.len());
            for value in values {
                match value {
                    Value::String(value) if !value.is_empty() => out.push(value.clone()),
                    Value::String(_) => {
                        return Err(CodegenError::InvalidOption {
                            name,
                            reason: "empty string".to_string(),
                        });
                    }
                    other => {
                        return Err(CodegenError::InvalidOption {
                            name,
                            reason: format!("unexpected value {other:?}"),
                        });
                    }
                }
            }
            Ok(out)
        }
        other => Err(CodegenError::InvalidOption {
            name,
            reason: format!("unexpected value {other:?}"),
        }),
    }
}

fn validate_projection_definitions(definitions: &[ProjectionDefinitionModel]) -> Result<()> {
    let mut names = Vec::new();
    let mut markers = Vec::new();
    for definition in definitions {
        validate_projection_name(&definition.name)?;
        validate_rust_marker(&definition.rust_marker)?;
        if names.iter().any(|name| name == &definition.name) {
            return Err(CodegenError::InvalidOption {
                name: "projection.name",
                reason: format!("duplicate projection {}", definition.name),
            });
        }
        if markers
            .iter()
            .any(|marker| marker == &definition.rust_marker)
        {
            return Err(CodegenError::InvalidOption {
                name: "projection.rust_marker",
                reason: format!("duplicate marker {}", definition.rust_marker),
            });
        }
        names.push(definition.name.clone());
        markers.push(definition.rust_marker.clone());
    }
    Ok(())
}

fn validate_projection_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(CodegenError::InvalidOption {
            name: "projection.name",
            reason: "empty projection name".to_string(),
        });
    }
    let mut previous_underscore = false;
    for (idx, ch) in name.chars().enumerate() {
        let valid = ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_';
        if !valid || (idx == 0 && (ch.is_ascii_digit() || ch == '_')) {
            return Err(CodegenError::InvalidOption {
                name: "projection.name",
                reason: format!("invalid projection name {name}"),
            });
        }
        if ch == '_' {
            if previous_underscore {
                return Err(CodegenError::InvalidOption {
                    name: "projection.name",
                    reason: format!("invalid projection name {name}"),
                });
            }
            previous_underscore = true;
        } else {
            previous_underscore = false;
        }
    }
    if previous_underscore {
        return Err(CodegenError::InvalidOption {
            name: "projection.name",
            reason: format!("invalid projection name {name}"),
        });
    }
    Ok(())
}

fn validate_rust_marker(marker: &str) -> Result<()> {
    let mut chars = marker.chars();
    let Some(first) = chars.next() else {
        return Err(CodegenError::InvalidOption {
            name: "projection.rust_marker",
            reason: "empty marker".to_string(),
        });
    };
    if !first.is_ascii_uppercase() {
        return Err(CodegenError::InvalidOption {
            name: "projection.rust_marker",
            reason: format!("invalid marker {marker}"),
        });
    }
    if !chars.all(|ch| ch.is_ascii_alphanumeric()) {
        return Err(CodegenError::InvalidOption {
            name: "projection.rust_marker",
            reason: format!("invalid marker {marker}"),
        });
    }
    Ok(())
}

fn build_projection_model(
    model: &SchemaModel,
    definition: ProjectionDefinitionModel,
) -> Result<ProjectionModel> {
    // Projection construction rebases presence bits and recomputes schema identity.
    let selected_indices = selected_projection_indices(model, &definition)?;
    let mut fields = Vec::with_capacity(selected_indices.len());
    let mut field_mappings = Vec::with_capacity(selected_indices.len());
    let mut next_presence = 0_u32;
    for source_index in selected_indices {
        let source = model.fields[source_index].clone();
        let source_presence_bit = source.presence_bit;
        let projected_presence_bit = if source_presence_bit.is_some() {
            let bit = next_presence;
            next_presence = next_presence.checked_add(1).ok_or_else(|| {
                CodegenError::InvalidSchema("projection presence bit overflow".to_string())
            })?;
            Some(bit)
        } else {
            None
        };
        let mut projected = source;
        projected.presence_bit = projected_presence_bit;
        if let FieldKind::U16Dictionary { optional, .. } = &mut projected.kind {
            *optional = projected_presence_bit.is_some();
        }
        fields.push(projected);
        field_mappings.push(ProjectionFieldMapping {
            source_index,
            source_presence_bit,
            projected_presence_bit,
        });
    }
    validate_projection_fields(model, &fields)?;
    let key_parts = key_parts(&fields);
    let dictionaries = selected_dictionaries(&model.dictionaries, &fields);
    let marker_type = definition.rust_marker.clone();
    let suffix = projection_suffix(&model.marker_type, &marker_type);
    let payload_type = format!("{}{}", model.payload_type, suffix);
    let row_type = format!("{}{}", model.row_type, suffix);
    let view_type = format!("{marker_type}View");
    let rows_iter_type = format!("{marker_type}Rows");
    let archived_row_type = format!("Archived{marker_type}Row");
    let transport_name = format!("{}.{}", model.transport_name, definition.name);
    let mut projection = ProjectionModel {
        definition,
        marker_type,
        payload_type,
        row_type,
        view_type,
        rows_iter_type,
        archived_row_type,
        transport_name,
        dictionaries,
        fields,
        key_parts,
        normalized_schema_hash: 0,
        field_mappings,
    };
    projection.normalized_schema_hash = projection_hash(model, &projection);
    Ok(projection)
}

fn selected_projection_indices(
    model: &SchemaModel,
    definition: &ProjectionDefinitionModel,
) -> Result<Vec<usize>> {
    // Include/exclude rules are resolved against mandatory schema and key fields.
    for group in definition
        .include_groups
        .iter()
        .chain(definition.exclude_groups.iter())
    {
        if !model
            .fields
            .iter()
            .any(|field| field.projection_group.as_deref() == Some(group.as_str()))
        {
            return Err(CodegenError::InvalidOption {
                name: "projection.group",
                reason: format!("unknown projection group {group}"),
            });
        }
    }

    let mut selected = vec![false; model.fields.len()];
    for (idx, field) in model.fields.iter().enumerate() {
        if is_mandatory_projection_field(field) {
            selected[idx] = true;
        }
    }

    let has_includes =
        !definition.include_groups.is_empty() || !definition.include_fields.is_empty();
    if has_includes {
        for group in &definition.include_groups {
            for (idx, field) in model.fields.iter().enumerate() {
                if field.projection_group.as_deref() == Some(group.as_str()) {
                    selected[idx] = true;
                }
            }
        }
        for field_name in &definition.include_fields {
            let idx = field_index_by_declared_name(&model.fields, field_name)?;
            selected[idx] = true;
        }
    } else {
        selected.fill(true);
    }

    for group in &definition.exclude_groups {
        for (idx, field) in model.fields.iter().enumerate() {
            if field.projection_group.as_deref() == Some(group.as_str()) {
                selected[idx] = false;
            }
        }
    }
    for field_name in &definition.exclude_fields {
        let idx = field_index_by_declared_name(&model.fields, field_name)?;
        selected[idx] = false;
    }

    for (idx, field) in model.fields.iter().enumerate() {
        if is_mandatory_projection_field(field) {
            selected[idx] = true;
        }
    }

    Ok(selected
        .iter()
        .enumerate()
        .filter_map(|(idx, keep)| keep.then_some(idx))
        .collect())
}

fn field_index_by_declared_name(fields: &[PhysicalField], name: &str) -> Result<usize> {
    let matches: Vec<usize> = fields
        .iter()
        .enumerate()
        .filter_map(|(idx, field)| {
            (field.logical_path == name || field.proto_name == name || field.rust_name == name)
                .then_some(idx)
        })
        .collect();
    match matches.as_slice() {
        [idx] => Ok(*idx),
        [] => Err(CodegenError::InvalidOption {
            name: "projection.field",
            reason: format!("unknown projection field {name}"),
        }),
        _ => Err(CodegenError::InvalidOption {
            name: "projection.field",
            reason: format!("ambiguous projection field {name}"),
        }),
    }
}

fn is_mandatory_projection_field(field: &PhysicalField) -> bool {
    matches!(field.kind, FieldKind::ConstU16 { .. }) || field.key_order.is_some()
}

fn validate_projection_fields(model: &SchemaModel, fields: &[PhysicalField]) -> Result<()> {
    // Projected payloads must keep schema version and deterministic key material.
    if !fields
        .iter()
        .any(|field| matches!(field.kind, FieldKind::ConstU16 { .. }))
    {
        return Err(CodegenError::InvalidSchema(
            "projected schema has no const_u16 schema version field".to_string(),
        ));
    }
    if !model.key_parts.is_empty() && !fields.iter().any(|field| field.key_order.is_some()) {
        return Err(CodegenError::InvalidSchema(
            "projected schema has no deterministic key field".to_string(),
        ));
    }
    validate_physical_fields(fields)
}

fn selected_dictionaries(dictionaries: &[Dictionary], fields: &[PhysicalField]) -> Vec<Dictionary> {
    dictionaries
        .iter()
        .filter(|dictionary| {
            fields.iter().any(|field| match &field.kind {
                FieldKind::U16Dictionary {
                    dictionary: used, ..
                }
                | FieldKind::U64BitmaskDictionary { dictionary: used } => used == &dictionary.name,
                _ => false,
            })
        })
        .cloned()
        .collect()
}

fn projection_suffix(source_marker: &str, projection_marker: &str) -> String {
    projection_marker
        .strip_prefix(source_marker)
        .filter(|suffix| !suffix.is_empty())
        .map_or_else(|| projection_marker.to_string(), ToString::to_string)
}

fn projection_hash(source: &SchemaModel, projection: &ProjectionModel) -> u64 {
    let mut text = String::new();
    text.push_str("projection_schema:\n");
    text.push_str(&format!(
        "source_schema_hash:{}\n",
        source.normalized_schema_hash
    ));
    text.push_str(&format!("source_schema_id:{}\n", source.schema_id));
    text.push_str(&format!(
        "source_schema_version:{}\n",
        source.schema_version
    ));
    text.push_str(&format!(
        "source_schema_version_value:{}\n",
        source.schema_version_value
    ));
    text.push_str(&format!(
        "source_transport_name:{}\n",
        source.transport_name
    ));
    text.push_str(&format!("projection_name:{}\n", projection.definition.name));
    text.push_str(&format!(
        "projection_rust_marker:{}\n",
        projection.definition.rust_marker
    ));
    text.push_str(&format!(
        "projected_transport_name:{}\n",
        projection.transport_name
    ));
    text.push_str(&format!("projected_row_type:{}\n", projection.row_type));
    for dictionary in &projection.dictionaries {
        text.push_str(&format!("projection_dictionary:{}\n", dictionary.name));
        for value in &dictionary.values {
            text.push_str(&format!("dictionary_value:{value}\n"));
        }
    }
    for field in &projection.fields {
        let dictionary_name = match &field.kind {
            FieldKind::U16Dictionary { dictionary, .. }
            | FieldKind::U64BitmaskDictionary { dictionary } => dictionary.as_str(),
            _ => "none",
        };
        let presence = field
            .presence_bit
            .map_or_else(|| "none".to_string(), |bit| bit.to_string());
        let presence_word = field
            .presence_bit
            .map_or_else(|| "none".to_string(), |bit| (bit / 64).to_string());
        let presence_mask = field.presence_bit.map_or_else(
            || "none".to_string(),
            |bit| (1_u64 << (bit % 64)).to_string(),
        );
        let key = field
            .key_order
            .map_or_else(|| "none".to_string(), |order| order.to_string());
        text.push_str(&format!(
            "projection_field:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}\n",
            field.proto_path,
            field.logical_path,
            field.proto_name,
            field.rust_name,
            field_kind_hash_name(&field.kind),
            dictionary_name,
            presence,
            presence_word,
            presence_mask,
            key
        ));
    }
    fnv1a64(text.as_bytes())
}

fn require_dictionary<'a>(name: &str, dictionaries: &'a [Dictionary]) -> Result<&'a Dictionary> {
    dictionaries
        .iter()
        .find(|dictionary| dictionary.name == name)
        .ok_or_else(|| CodegenError::InvalidOption {
            name: "dictionary",
            reason: format!("missing dictionary {name}"),
        })
}

fn raw_descriptor_set(request: &SchemaRequest) -> Result<Vec<u8>> {
    // protoc is used only to create a temporary descriptor set for this request.
    let tmp = temp_dir("descriptor")?;
    fs::create_dir_all(&tmp)?;
    let descriptor_path = tmp.join("schema-descriptor.pb");
    let result = run_protoc(request, &descriptor_path)
        .and_then(|()| fs::read(&descriptor_path).map_err(Into::into));
    let _ = fs::remove_dir_all(&tmp);
    result
}

fn run_protoc(request: &SchemaRequest, descriptor_path: &Path) -> Result<()> {
    let schema_path = schema_input_path(request)?;
    let mut command = Command::new("protoc");
    command
        .arg("--include_imports")
        .arg("--include_source_info")
        .arg("--experimental_allow_proto3_optional");
    for root in &request.proto_roots {
        command.arg(format!("--proto_path={}", root.display()));
    }
    let output = command
        .arg(format!(
            "--descriptor_set_out={}",
            descriptor_path.display()
        ))
        .arg(schema_path)
        .output()?;
    if output.status.success() {
        return Ok(());
    }
    Err(CodegenError::Descriptor(
        String::from_utf8_lossy(&output.stderr).into_owned(),
    ))
}

fn schema_input_path(request: &SchemaRequest) -> Result<PathBuf> {
    if request.schema.is_absolute() {
        return Ok(request.schema.clone());
    }
    for root in &request.proto_roots {
        let candidate = root.join(&request.schema);
        if candidate.exists() {
            return Ok(candidate);
        }
    }
    Ok(request.schema.clone())
}

fn source_file_name(request: &SchemaRequest) -> Result<String> {
    request
        .schema
        .to_str()
        .map(ToString::to_string)
        .ok_or_else(|| CodegenError::InvalidSchema("non-utf8 schema path".to_string()))
}

fn temp_dir(label: &str) -> Result<PathBuf> {
    let mut path = std::env::current_dir()?
        .join("target")
        .join("mbt-codegen-check");
    let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    path.push(format!("{label}-{}-{counter}", std::process::id()));
    Ok(path)
}

pub fn normalized_hash(model: &SchemaModel) -> u64 {
    // Schema hashes are derived from the normalized model, not file formatting.
    let mut text = String::new();
    text.push_str(&format!(
        "schema:{}:{}:{}:{}:{}:{}\n",
        model.schema_id,
        model.schema_version,
        model.schema_version_value,
        model.transport_name,
        model.payload_root,
        model.row_type
    ));
    for dictionary in &model.dictionaries {
        text.push_str(&format!("dictionary:{}", dictionary.name));
        for value in &dictionary.values {
            text.push(':');
            text.push_str(value);
        }
        text.push('\n');
    }
    for field in &model.fields {
        let presence = match field.presence_bit {
            Some(value) => value.to_string(),
            None => "none".to_string(),
        };
        let key = match field.key_order {
            Some(value) => value.to_string(),
            None => "none".to_string(),
        };
        text.push_str(&format!(
            "field:{}:{}:{}:{}:{}\n",
            field.proto_path,
            field.rust_name,
            field_kind_hash_name(&field.kind),
            presence,
            key
        ));
    }
    fnv1a64(text.as_bytes())
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x00000100000001B3_u64);
    }
    hash
}
