use crate::error::{CodegenError, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dictionary {
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaModel {
    pub module: String,
    pub proto: std::path::PathBuf,
    pub root: String,
    pub root_type: String,
    pub payload_type: String,
    pub row_type: String,
    pub marker_type: String,
    pub view_type: String,
    pub rows_iter_type: String,
    pub archived_row_type: String,
    pub schema_id: u32,
    pub schema_version: u32,
    pub schema_version_value: u16,
    pub transport_name: String,
    pub payload_root: bool,
    pub row_field_name: String,
    pub row_field_number: u32,
    pub dictionaries: Vec<Dictionary>,
    pub fields: Vec<PhysicalField>,
    pub key_parts: Vec<KeyPart>,
    pub normalized_schema_hash: u64,
    pub projections: Vec<ProjectionModel>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionDefinitionModel {
    pub name: String,
    pub rust_marker: String,
    pub include_groups: Vec<String>,
    pub exclude_groups: Vec<String>,
    pub include_fields: Vec<String>,
    pub exclude_fields: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionFieldMapping {
    pub source_index: usize,
    pub source_presence_bit: Option<u32>,
    pub projected_presence_bit: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionModel {
    pub definition: ProjectionDefinitionModel,
    pub marker_type: String,
    pub payload_type: String,
    pub row_type: String,
    pub view_type: String,
    pub rows_iter_type: String,
    pub archived_row_type: String,
    pub transport_name: String,
    pub dictionaries: Vec<Dictionary>,
    pub fields: Vec<PhysicalField>,
    pub key_parts: Vec<KeyPart>,
    pub normalized_schema_hash: u64,
    pub field_mappings: Vec<ProjectionFieldMapping>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalField {
    pub proto_path: String,
    pub logical_path: String,
    pub proto_name: String,
    pub rust_name: String,
    pub proto_number: u32,
    pub kind: FieldKind,
    pub presence_bit: Option<u32>,
    pub key_order: Option<u32>,
    pub projection_group: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldKind {
    ConstU16 { value: u16 },
    U16Dictionary { dictionary: String, optional: bool },
    U64BitmaskDictionary { dictionary: String },
    I32,
    U32,
    I64,
    F32,
    F64,
    Bool,
    Bytes,
    RawString,
    I64Array,
    I32Array,
    U32Array,
    F64Array,
    F32Array,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyPart {
    pub order: u32,
    pub rust_name: String,
}

pub fn rust_type_name(proto_name: &str) -> String {
    match proto_name.rsplit('.').next() {
        Some(name) => name.to_string(),
        None => proto_name.to_string(),
    }
}

pub fn module_marker_type(module: &str) -> String {
    let mut out = String::new();
    for part in module.split('_') {
        if part.is_empty() {
            continue;
        }
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            out.push(first.to_ascii_uppercase());
            out.extend(chars);
        }
    }
    out
}

pub fn payload_type_from_root(root_type: &str) -> String {
    if root_type.ends_with("Response") {
        root_type.replace("Response", "Payload")
    } else {
        format!("{root_type}Payload")
    }
}

pub fn const_name(value: &str) -> String {
    let mut out = String::new();
    let mut previous_underscore = false;
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_uppercase());
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
        "EMPTY".to_string()
    } else {
        out
    }
}

pub fn dict_prefix(name: &str) -> String {
    const_name(name)
}

pub fn helper_stem(name: &str) -> String {
    name.to_string()
}

pub fn field_kind_hash_name(kind: &FieldKind) -> String {
    match kind {
        FieldKind::ConstU16 { value } => format!("ConstU16({value})"),
        FieldKind::U16Dictionary {
            dictionary,
            optional,
        } => format!("U16Dictionary({dictionary},optional={optional})"),
        FieldKind::U64BitmaskDictionary { dictionary } => {
            format!("U64BitmaskDictionary({dictionary})")
        }
        FieldKind::I32 => "I32".to_string(),
        FieldKind::U32 => "U32".to_string(),
        FieldKind::I64 => "I64".to_string(),
        FieldKind::F32 => "F32".to_string(),
        FieldKind::F64 => "F64".to_string(),
        FieldKind::Bool => "Bool".to_string(),
        FieldKind::Bytes => "Bytes".to_string(),
        FieldKind::RawString => "RawString".to_string(),
        FieldKind::I64Array => "I64Array".to_string(),
        FieldKind::I32Array => "I32Array".to_string(),
        FieldKind::U32Array => "U32Array".to_string(),
        FieldKind::F64Array => "F64Array".to_string(),
        FieldKind::F32Array => "F32Array".to_string(),
    }
}

pub fn validate_module_name(module: &str) -> Result<()> {
    if module.is_empty() {
        return Err(CodegenError::UnsupportedArgument(
            "module cannot be empty".to_string(),
        ));
    }
    let mut previous_underscore = false;
    for (idx, ch) in module.chars().enumerate() {
        let valid = ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_';
        if !valid || (idx == 0 && (ch.is_ascii_digit() || ch == '_')) {
            return Err(CodegenError::UnsupportedArgument(format!(
                "invalid module name {module}"
            )));
        }
        if ch == '_' {
            if previous_underscore {
                return Err(CodegenError::UnsupportedArgument(format!(
                    "invalid module name {module}"
                )));
            }
            previous_underscore = true;
        } else {
            previous_underscore = false;
        }
    }
    if previous_underscore {
        return Err(CodegenError::UnsupportedArgument(format!(
            "invalid module name {module}"
        )));
    }
    Ok(())
}
