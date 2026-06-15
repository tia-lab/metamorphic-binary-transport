#![forbid(unsafe_code)]
//! Arrow RecordBatch helpers for generated MBT metamorphose adapters.

use std::collections::HashMap;
use std::sync::Arc;

use arrow_array::types::{
    Float32Type, Float64Type, Int32Type, Int64Type, UInt16Type, UInt32Type, UInt64Type,
};
use arrow_array::{
    Array, ArrayRef, BinaryArray, BooleanArray, Float32Array, Float64Array, Int32Array, Int64Array,
    ListArray, RecordBatch, StringArray, UInt16Array, UInt32Array, UInt64Array,
};
use arrow_buffer::{BooleanBuffer, Buffer, NullBuffer, OffsetBuffer, ScalarBuffer};
use arrow_schema::{ArrowError, DataType, Field, SchemaRef};
use metamorphic_binary_transport_core::error::{Result, TransportError};
use metamorphic_binary_transport_transponding::{
    BinaryColumn, BoolColumn, ConstU16Column, F32Column, F32ListColumn, F64Column, F64ListColumn,
    I32Column, I32ListColumn, I64Column, I64ListColumn, OptionalF32Column, OptionalF64Column,
    OptionalI32Column, OptionalI64Column, OptionalU16Column, OptionalU32Column, OptionalU64Column,
    U16Column, U32Column, U32ListColumn, U64Column, Utf8Column, ValidityBitmap, checksum_seed,
    update_bool, update_bytes, update_f32, update_f64, update_i32, update_i64, update_str,
    update_u16, update_u32, update_u64, update_usize,
};

pub use arrow_array::{ArrayRef as ArrowArrayRef, RecordBatch as ArrowRecordBatch};
pub use arrow_schema::{DataType as ArrowDataType, Field as ArrowField, Schema as ArrowSchema};

pub fn arrow_error(err: ArrowError) -> TransportError {
    TransportError::MalformedArchive(err.to_string())
}

pub fn schema_metadata(
    transport_name: &'static str,
    schema_id: u32,
    schema_version: u32,
    schema_hash: u64,
) -> HashMap<String, String> {
    let mut metadata = HashMap::with_capacity(4);
    metadata.insert("mbt.transport_name".to_string(), transport_name.to_string());
    metadata.insert("mbt.schema_id".to_string(), schema_id.to_string());
    metadata.insert("mbt.schema_version".to_string(), schema_version.to_string());
    metadata.insert("mbt.schema_hash".to_string(), schema_hash.to_string());
    metadata
}

pub fn field_metadata(
    field_name: &'static str,
    physical_name: &'static str,
    dictionary: Option<&'static str>,
    bitmask_dictionary: Option<&'static str>,
) -> HashMap<String, String> {
    let mut metadata = HashMap::with_capacity(3);
    if field_name != physical_name {
        metadata.insert("mbt.physical_name".to_string(), physical_name.to_string());
    }
    if let Some(name) = dictionary {
        metadata.insert("mbt.dictionary".to_string(), name.to_string());
    }
    if let Some(name) = bitmask_dictionary {
        metadata.insert("mbt.bitmask_dictionary".to_string(), name.to_string());
    }
    metadata
}

pub fn const_u16_array(column: ConstU16Column) -> Result<ArrayRef> {
    let values = vec![column.value; column.len];
    primitive_array_required::<UInt16Type>(values)
}

pub fn u16_array(column: U16Column) -> Result<ArrayRef> {
    primitive_array_required::<UInt16Type>(column.values)
}

pub fn i32_array(column: I32Column) -> Result<ArrayRef> {
    primitive_array_required::<Int32Type>(column.values)
}

pub fn u32_array(column: U32Column) -> Result<ArrayRef> {
    primitive_array_required::<UInt32Type>(column.values)
}

pub fn u64_array(column: U64Column) -> Result<ArrayRef> {
    primitive_array_required::<UInt64Type>(column.values)
}

pub fn i64_array(column: I64Column) -> Result<ArrayRef> {
    primitive_array_required::<Int64Type>(column.values)
}

pub fn f32_array(column: F32Column) -> Result<ArrayRef> {
    primitive_array_required::<Float32Type>(column.values)
}

pub fn f64_array(column: F64Column) -> Result<ArrayRef> {
    primitive_array_required::<Float64Type>(column.values)
}

pub fn optional_u16_array(column: OptionalU16Column) -> Result<ArrayRef> {
    primitive_array_optional::<UInt16Type>(column.values, column.validity)
}

pub fn optional_i32_array(column: OptionalI32Column) -> Result<ArrayRef> {
    primitive_array_optional::<Int32Type>(column.values, column.validity)
}

pub fn optional_u32_array(column: OptionalU32Column) -> Result<ArrayRef> {
    primitive_array_optional::<UInt32Type>(column.values, column.validity)
}

pub fn optional_u64_array(column: OptionalU64Column) -> Result<ArrayRef> {
    primitive_array_optional::<UInt64Type>(column.values, column.validity)
}

pub fn optional_i64_array(column: OptionalI64Column) -> Result<ArrayRef> {
    primitive_array_optional::<Int64Type>(column.values, column.validity)
}

pub fn optional_f32_array(column: OptionalF32Column) -> Result<ArrayRef> {
    primitive_array_optional::<Float32Type>(column.values, column.validity)
}

pub fn optional_f64_array(column: OptionalF64Column) -> Result<ArrayRef> {
    primitive_array_optional::<Float64Type>(column.values, column.validity)
}

pub fn bool_array(column: BoolColumn) -> Result<ArrayRef> {
    let row_count = column.values.len();
    let nulls = null_buffer(column.validity, row_count)?;
    let buffer = BooleanBuffer::from(column.values);
    Ok(Arc::new(BooleanArray::new(buffer, nulls)))
}

pub fn utf8_array(column: Utf8Column) -> Result<ArrayRef> {
    ensure_offsets(&column.offsets, column.bytes.len())?;
    let row_count = column.offsets.len().saturating_sub(1);
    let nulls = null_buffer(column.validity, row_count)?;
    let offsets = OffsetBuffer::new(ScalarBuffer::from(column.offsets));
    let values = Buffer::from_vec(column.bytes);
    let array = StringArray::try_new(offsets, values, nulls).map_err(arrow_error)?;
    Ok(Arc::new(array))
}

pub fn binary_array(column: BinaryColumn) -> Result<ArrayRef> {
    ensure_offsets(&column.offsets, column.bytes.len())?;
    let row_count = column.offsets.len().saturating_sub(1);
    let nulls = null_buffer(column.validity, row_count)?;
    let offsets = OffsetBuffer::new(ScalarBuffer::from(column.offsets));
    let values = Buffer::from_vec(column.bytes);
    let array = BinaryArray::try_new(offsets, values, nulls).map_err(arrow_error)?;
    Ok(Arc::new(array))
}

pub fn i64_list_array(column: I64ListColumn) -> Result<ArrayRef> {
    primitive_list_array::<Int64Type>(column.offsets, column.values, column.validity)
}

pub fn optional_i64_list_array(column: I64ListColumn) -> Result<ArrayRef> {
    primitive_list_array::<Int64Type>(column.offsets, column.values, column.validity)
}

pub fn i32_list_array(column: I32ListColumn) -> Result<ArrayRef> {
    primitive_list_array::<Int32Type>(column.offsets, column.values, column.validity)
}

pub fn optional_i32_list_array(column: I32ListColumn) -> Result<ArrayRef> {
    primitive_list_array::<Int32Type>(column.offsets, column.values, column.validity)
}

pub fn u32_list_array(column: U32ListColumn) -> Result<ArrayRef> {
    primitive_list_array::<UInt32Type>(column.offsets, column.values, column.validity)
}

pub fn optional_u32_list_array(column: U32ListColumn) -> Result<ArrayRef> {
    primitive_list_array::<UInt32Type>(column.offsets, column.values, column.validity)
}

pub fn f64_list_array(column: F64ListColumn) -> Result<ArrayRef> {
    primitive_list_array::<Float64Type>(column.offsets, column.values, column.validity)
}

pub fn optional_f64_list_array(column: F64ListColumn) -> Result<ArrayRef> {
    primitive_list_array::<Float64Type>(column.offsets, column.values, column.validity)
}

pub fn f32_list_array(column: F32ListColumn) -> Result<ArrayRef> {
    primitive_list_array::<Float32Type>(column.offsets, column.values, column.validity)
}

pub fn optional_f32_list_array(column: F32ListColumn) -> Result<ArrayRef> {
    primitive_list_array::<Float32Type>(column.offsets, column.values, column.validity)
}

pub fn record_batch(
    schema: SchemaRef,
    columns: Vec<ArrayRef>,
    max_arrow_bytes: usize,
) -> Result<RecordBatch> {
    let batch = RecordBatch::try_new(schema, columns).map_err(arrow_error)?;
    let observed = record_batch_byte_len(&batch);
    if observed > max_arrow_bytes {
        return Err(TransportError::ResponseTooLarge {
            observed,
            cap: max_arrow_bytes,
        });
    }
    Ok(batch)
}

pub fn record_batch_byte_len(batch: &RecordBatch) -> usize {
    batch.columns().iter().fold(0_usize, |len, column| {
        len.saturating_add(column.get_buffer_memory_size())
    })
}

pub fn record_batch_checksum(batch: &RecordBatch) -> Result<u64> {
    let mut checksum = checksum_seed("arrow-record-batch");
    checksum = update_usize(checksum, batch.num_rows());
    checksum = update_usize(checksum, batch.num_columns());
    for (field, column) in batch.schema().fields().iter().zip(batch.columns()) {
        checksum = update_str(checksum, field.name());
        checksum = update_str(checksum, &field.data_type().to_string());
        checksum = update_usize(checksum, usize::from(field.is_nullable()));
        checksum = checksum_array(checksum, column.as_ref())?;
    }
    Ok(checksum)
}

fn primitive_array_required<T>(values: Vec<T::Native>) -> Result<ArrayRef>
where
    T: arrow_array::types::ArrowPrimitiveType + 'static,
    arrow_array::PrimitiveArray<T>: Array + 'static,
{
    let array = arrow_array::PrimitiveArray::<T>::try_new(ScalarBuffer::from(values), None)
        .map_err(arrow_error)?;
    Ok(Arc::new(array))
}

fn primitive_array_optional<T>(values: Vec<T::Native>, validity: ValidityBitmap) -> Result<ArrayRef>
where
    T: arrow_array::types::ArrowPrimitiveType + 'static,
    arrow_array::PrimitiveArray<T>: Array + 'static,
{
    let row_count = values.len();
    let nulls = null_buffer(Some(validity), row_count)?;
    let array = arrow_array::PrimitiveArray::<T>::try_new(ScalarBuffer::from(values), nulls)
        .map_err(arrow_error)?;
    Ok(Arc::new(array))
}

fn primitive_list_array<T>(
    offsets: Vec<i32>,
    values: Vec<T::Native>,
    validity: Option<ValidityBitmap>,
) -> Result<ArrayRef>
where
    T: arrow_array::types::ArrowPrimitiveType + 'static,
    arrow_array::PrimitiveArray<T>: Array + 'static,
{
    ensure_offsets(&offsets, values.len())?;
    let row_count = offsets.len().saturating_sub(1);
    let nulls = null_buffer(validity, row_count)?;
    let values = primitive_array_required::<T>(values)?;
    let field = Arc::new(Field::new("item", T::DATA_TYPE, false));
    let offsets = OffsetBuffer::new(ScalarBuffer::from(offsets));
    let array = ListArray::try_new(field, offsets, values, nulls).map_err(arrow_error)?;
    Ok(Arc::new(array))
}

fn null_buffer(
    validity: Option<ValidityBitmap>,
    expected_len: usize,
) -> Result<Option<NullBuffer>> {
    match validity {
        Some(validity) => {
            let len = validity.len();
            if len != expected_len {
                return Err(TransportError::RowCountMismatch {
                    observed: len,
                    expected: expected_len as u64,
                });
            }
            let words = validity.into_words();
            let bit_capacity = words.len().saturating_mul(64);
            if bit_capacity < expected_len {
                return Err(TransportError::RowCountMismatch {
                    observed: bit_capacity,
                    expected: expected_len as u64,
                });
            }
            let buffer = Buffer::from_vec(words);
            Ok(Some(NullBuffer::new(BooleanBuffer::new(
                buffer,
                0,
                expected_len,
            ))))
        }
        None => Ok(None),
    }
}

fn ensure_offsets(offsets: &[i32], values_len: usize) -> Result<()> {
    let Some(first) = offsets.first() else {
        return Err(TransportError::MalformedArchive(
            "Arrow offsets cannot be empty".to_string(),
        ));
    };
    if *first != 0 {
        return Err(TransportError::MalformedArchive(
            "Arrow offsets must start at zero".to_string(),
        ));
    }
    let mut previous = 0_i32;
    for offset in offsets {
        if *offset < previous {
            return Err(TransportError::MalformedArchive(
                "Arrow offsets must be monotonic".to_string(),
            ));
        }
        previous = *offset;
    }
    let Some(last) = offsets.last() else {
        return Err(TransportError::MalformedArchive(
            "Arrow offsets cannot be empty".to_string(),
        ));
    };
    let last_offset = usize::try_from(*last).map_err(|_| {
        TransportError::MalformedArchive("Arrow offsets cannot be negative".to_string())
    })?;
    if last_offset != values_len {
        return Err(TransportError::MalformedArchive(format!(
            "Arrow offsets end at {}, expected {values_len}",
            *last
        )));
    }
    Ok(())
}

fn checksum_array(mut checksum: u64, array: &dyn Array) -> Result<u64> {
    checksum = update_usize(checksum, array.len());
    for idx in 0..array.len() {
        checksum = update_usize(checksum, usize::from(array.is_null(idx)));
    }
    match array.data_type() {
        DataType::UInt16 => {
            let Some(values) = array.as_any().downcast_ref::<UInt16Array>() else {
                return Err(TransportError::MalformedArchive(
                    "UInt16 Arrow downcast failed".to_string(),
                ));
            };
            for idx in 0..values.len() {
                checksum = update_u16(checksum, values.value(idx));
            }
        }
        DataType::Int32 => {
            let Some(values) = array.as_any().downcast_ref::<Int32Array>() else {
                return Err(TransportError::MalformedArchive(
                    "Int32 Arrow downcast failed".to_string(),
                ));
            };
            for idx in 0..values.len() {
                checksum = update_i32(checksum, values.value(idx));
            }
        }
        DataType::UInt32 => {
            let Some(values) = array.as_any().downcast_ref::<UInt32Array>() else {
                return Err(TransportError::MalformedArchive(
                    "UInt32 Arrow downcast failed".to_string(),
                ));
            };
            for idx in 0..values.len() {
                checksum = update_u32(checksum, values.value(idx));
            }
        }
        DataType::UInt64 => {
            let Some(values) = array.as_any().downcast_ref::<UInt64Array>() else {
                return Err(TransportError::MalformedArchive(
                    "UInt64 Arrow downcast failed".to_string(),
                ));
            };
            for idx in 0..values.len() {
                checksum = update_u64(checksum, values.value(idx));
            }
        }
        DataType::Int64 => {
            let Some(values) = array.as_any().downcast_ref::<Int64Array>() else {
                return Err(TransportError::MalformedArchive(
                    "Int64 Arrow downcast failed".to_string(),
                ));
            };
            for idx in 0..values.len() {
                checksum = update_i64(checksum, values.value(idx));
            }
        }
        DataType::Float32 => {
            let Some(values) = array.as_any().downcast_ref::<Float32Array>() else {
                return Err(TransportError::MalformedArchive(
                    "Float32 Arrow downcast failed".to_string(),
                ));
            };
            for idx in 0..values.len() {
                checksum = update_f32(checksum, values.value(idx));
            }
        }
        DataType::Float64 => {
            let Some(values) = array.as_any().downcast_ref::<Float64Array>() else {
                return Err(TransportError::MalformedArchive(
                    "Float64 Arrow downcast failed".to_string(),
                ));
            };
            for idx in 0..values.len() {
                checksum = update_f64(checksum, values.value(idx));
            }
        }
        DataType::Boolean => {
            let Some(values) = array.as_any().downcast_ref::<BooleanArray>() else {
                return Err(TransportError::MalformedArchive(
                    "Boolean Arrow downcast failed".to_string(),
                ));
            };
            for idx in 0..values.len() {
                checksum = update_bool(checksum, values.value(idx));
            }
        }
        DataType::Binary => {
            let Some(values) = array.as_any().downcast_ref::<BinaryArray>() else {
                return Err(TransportError::MalformedArchive(
                    "Binary Arrow downcast failed".to_string(),
                ));
            };
            for idx in 0..values.len() {
                checksum = update_bytes(checksum, values.value(idx));
            }
        }
        DataType::Utf8 => {
            let Some(values) = array.as_any().downcast_ref::<StringArray>() else {
                return Err(TransportError::MalformedArchive(
                    "Utf8 Arrow downcast failed".to_string(),
                ));
            };
            for idx in 0..values.len() {
                checksum = update_str(checksum, values.value(idx));
            }
        }
        DataType::List(_) => {
            let Some(values) = array.as_any().downcast_ref::<ListArray>() else {
                return Err(TransportError::MalformedArchive(
                    "List Arrow downcast failed".to_string(),
                ));
            };
            for offset in values.value_offsets() {
                checksum = update_i32(checksum, *offset);
            }
            checksum = checksum_array(checksum, values.values().as_ref())?;
        }
        other => {
            return Err(TransportError::MalformedArchive(format!(
                "unsupported Arrow checksum type {other}"
            )));
        }
    }
    Ok(checksum)
}

#[cfg(test)]
mod tests;
