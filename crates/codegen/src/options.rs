use prost_reflect::{DescriptorPool, DynamicMessage, ExtensionDescriptor, Value};

use crate::error::{CodegenError, Result};
use crate::model::Dictionary;

#[derive(Debug, Clone)]
pub struct MbtExtensions {
    // Centralized handles for every MBT custom option used during descriptor reads.
    pub dictionary_values: ExtensionDescriptor,
    pub schema_id: ExtensionDescriptor,
    pub schema_version: ExtensionDescriptor,
    pub transport_name: ExtensionDescriptor,
    pub payload_root: ExtensionDescriptor,
    pub projection: ExtensionDescriptor,
    pub dictionary: ExtensionDescriptor,
    pub bitmask_dictionary: ExtensionDescriptor,
    pub presence_bit: ExtensionDescriptor,
    pub key_part: ExtensionDescriptor,
    pub key_order: ExtensionDescriptor,
    pub const_u16: ExtensionDescriptor,
    pub repeated_payload: ExtensionDescriptor,
    pub raw_string: ExtensionDescriptor,
    pub ignored: ExtensionDescriptor,
    pub projection_group: ExtensionDescriptor,
    pub derived_utc_from: ExtensionDescriptor,
}

pub fn extensions(pool: &DescriptorPool) -> Result<MbtExtensions> {
    // Resolve option descriptors once so later parsing remains explicit and typed.
    Ok(MbtExtensions {
        dictionary_values: extension(pool, "mathilde.dictionary_values")?,
        schema_id: extension(pool, "mathilde.schema_id")?,
        schema_version: extension(pool, "mathilde.schema_version")?,
        transport_name: extension(pool, "mathilde.transport_name")?,
        payload_root: extension(pool, "mathilde.payload_root")?,
        projection: extension(pool, "mathilde.projection")?,
        dictionary: extension(pool, "mathilde.dictionary")?,
        bitmask_dictionary: extension(pool, "mathilde.bitmask_dictionary")?,
        presence_bit: extension(pool, "mathilde.presence_bit")?,
        key_part: extension(pool, "mathilde.key_part")?,
        key_order: extension(pool, "mathilde.key_order")?,
        const_u16: extension(pool, "mathilde.const_u16")?,
        repeated_payload: extension(pool, "mathilde.repeated_payload")?,
        raw_string: extension(pool, "mathilde.raw_string")?,
        ignored: extension(pool, "mathilde.ignored")?,
        projection_group: extension(pool, "mathilde.projection_group")?,
        derived_utc_from: extension(pool, "mathilde.derived_utc_from")?,
    })
}

fn extension(pool: &DescriptorPool, name: &'static str) -> Result<ExtensionDescriptor> {
    pool.get_extension_by_name(name)
        .ok_or(CodegenError::MissingDescriptor(name))
}

pub fn required_u32(options: &DynamicMessage, ext: &ExtensionDescriptor) -> Result<u32> {
    // Required option helpers fail before any generated code is emitted.
    if !options.has_extension(ext) {
        return Err(CodegenError::MissingOption("u32"));
    }
    match &*options.get_extension(ext) {
        Value::U32(value) => Ok(*value),
        other => Err(CodegenError::InvalidOption {
            name: "u32",
            reason: format!("unexpected value {other:?}"),
        }),
    }
}

pub fn optional_u32(options: &DynamicMessage, ext: &ExtensionDescriptor) -> Result<Option<u32>> {
    // Optional helpers keep absence distinct from invalid option values.
    if options.has_extension(ext) {
        required_u32(options, ext).map(Some)
    } else {
        Ok(None)
    }
}

pub fn required_bool(options: &DynamicMessage, ext: &ExtensionDescriptor) -> Result<bool> {
    if !options.has_extension(ext) {
        return Err(CodegenError::MissingOption("bool"));
    }
    match &*options.get_extension(ext) {
        Value::Bool(value) => Ok(*value),
        other => Err(CodegenError::InvalidOption {
            name: "bool",
            reason: format!("unexpected value {other:?}"),
        }),
    }
}

pub fn optional_bool(options: &DynamicMessage, ext: &ExtensionDescriptor) -> Result<bool> {
    if options.has_extension(ext) {
        required_bool(options, ext)
    } else {
        Ok(false)
    }
}

pub fn required_string(options: &DynamicMessage, ext: &ExtensionDescriptor) -> Result<String> {
    if !options.has_extension(ext) {
        return Err(CodegenError::MissingOption("string"));
    }
    match &*options.get_extension(ext) {
        Value::String(value) => Ok(value.clone()),
        other => Err(CodegenError::InvalidOption {
            name: "string",
            reason: format!("unexpected value {other:?}"),
        }),
    }
}

pub fn optional_string(
    options: &DynamicMessage,
    ext: &ExtensionDescriptor,
) -> Result<Option<String>> {
    if options.has_extension(ext) {
        required_string(options, ext).map(Some)
    } else {
        Ok(None)
    }
}

pub fn dictionary_from_value(value: &Value) -> Result<Dictionary> {
    // Dictionary option payloads become the stable ordinal domain.
    let Value::Message(message) = value else {
        return Err(CodegenError::InvalidOption {
            name: "dictionary_values",
            reason: format!("unexpected value {value:?}"),
        });
    };
    let name = required_message_string(message, "name")?;
    let values = message_string_list(message, "value")?;
    let _aliases = message.get_field_by_name("alias");
    Ok(Dictionary { name, values })
}

fn required_message_string(message: &DynamicMessage, name: &'static str) -> Result<String> {
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

fn message_string_list(message: &DynamicMessage, name: &'static str) -> Result<Vec<String>> {
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
