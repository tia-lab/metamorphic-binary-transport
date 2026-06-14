use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use prost_reflect::{Cardinality, DescriptorPool, FieldDescriptor, Kind, MessageDescriptor, Value};

use crate::config::SchemaRequest;
use crate::error::{CodegenError, Result};
use crate::model::{
    Dictionary, FieldKind, KeyPart, PhysicalField, SchemaModel, field_kind_hash_name,
    module_marker_type, payload_type_from_root, rust_type_name,
};
use crate::options::{
    MbtExtensions, dictionary_from_value, extensions, optional_bool, optional_string, optional_u32,
    required_bool, required_string, required_u32,
};

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn load_schema_model(request: &SchemaRequest) -> Result<SchemaModel> {
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
    collect_physical_fields(
        &row_payload.message,
        &extensions,
        &dictionaries,
        "",
        None,
        &mut fields,
    )?;
    validate_physical_fields(&fields)?;
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
        dictionaries,
        fields,
        key_parts,
        normalized_schema_hash: 0,
    };
    model.normalized_schema_hash = normalized_hash(&model);
    Ok(model)
}

struct RowPayload {
    field: FieldDescriptor,
    message: MessageDescriptor,
}

fn row_payload_field(root: &MessageDescriptor, extensions: &MbtExtensions) -> Result<RowPayload> {
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

fn collect_physical_fields(
    message: &MessageDescriptor,
    extensions: &MbtExtensions,
    dictionaries: &[Dictionary],
    prefix: &str,
    inherited_projection_group: Option<&str>,
    out: &mut Vec<PhysicalField>,
) -> Result<()> {
    for field in message.fields() {
        if optional_bool(&field.options(), &extensions.repeated_payload)? {
            return Err(CodegenError::InvalidSchema(format!(
                "{} repeated_payload is only valid on payload root",
                field.full_name()
            )));
        }
        if optional_bool(&field.options(), &extensions.ignored)? {
            continue;
        }
        let options = field.options();
        let projection_group = optional_string(&options, &extensions.projection_group)?
            .or_else(|| inherited_projection_group.map(str::to_string));
        let logical_path = join_path(prefix, field.name());
        match field.kind() {
            Kind::Message(child) if field.cardinality() != Cardinality::Repeated => {
                collect_physical_fields(
                    &child,
                    extensions,
                    dictionaries,
                    &logical_path,
                    projection_group.as_deref(),
                    out,
                )?;
            }
            _ => out.push(physical_field(
                &field,
                extensions,
                dictionaries,
                &logical_path,
                projection_group,
            )?),
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
