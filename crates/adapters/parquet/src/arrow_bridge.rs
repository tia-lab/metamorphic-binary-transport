use std::collections::HashMap;
use std::sync::Arc;

use arrow_array::types::{
    Float32Type, Float64Type, Int32Type, Int64Type, UInt16Type, UInt32Type, UInt64Type,
};
use arrow_array::{
    Array, ArrayRef, BinaryArray, BooleanArray, ListArray, RecordBatch, StringArray,
};
use arrow_buffer::{BooleanBuffer, Buffer, NullBuffer, OffsetBuffer, ScalarBuffer};
use arrow_schema::{ArrowError, Field, SchemaRef};
use mbt_core::error::{Result, TransportError};
use mbt_transponding::{
    BinaryColumn, BoolColumn, ConstU16Column, F32Column, F32ListColumn, F64Column, F64ListColumn,
    I32Column, I32ListColumn, I64Column, I64ListColumn, OptionalF32Column, OptionalF64Column,
    OptionalI32Column, OptionalI64Column, OptionalU16Column, OptionalU32Column, OptionalU64Column,
    U16Column, U32Column, U32ListColumn, U64Column, Utf8Column, ValidityBitmap,
};

pub use arrow_array::ArrayRef as ArrowArrayRef;
pub use arrow_schema::{DataType as ArrowDataType, Field as ArrowField, Schema as ArrowSchema};

pub fn field_metadata(
    field_name: &'static str,
    physical_name: &'static str,
    dictionary: Option<&'static str>,
    bitmask_dictionary: Option<&'static str>,
) -> HashMap<String, String> {
    // Field metadata preserves physical, dictionary, and bitmask provenance.
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
    primitive_array_required::<UInt16Type>(vec![column.value; column.len])
}

pub fn u16_array(column: U16Column) -> Result<ArrayRef> {
    // Transponded primitive columns become Arrow primitive arrays.
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
    Ok(Arc::new(BooleanArray::new(
        BooleanBuffer::from(column.values),
        nulls,
    )))
}

pub fn utf8_array(column: Utf8Column) -> Result<ArrayRef> {
    ensure_offsets(&column.offsets, column.bytes.len())?;
    let row_count = column.offsets.len().saturating_sub(1);
    let nulls = null_buffer(column.validity, row_count)?;
    let array = StringArray::try_new(
        OffsetBuffer::new(ScalarBuffer::from(column.offsets)),
        Buffer::from_vec(column.bytes),
        nulls,
    )
    .map_err(arrow_error)?;
    Ok(Arc::new(array))
}

pub fn binary_array(column: BinaryColumn) -> Result<ArrayRef> {
    ensure_offsets(&column.offsets, column.bytes.len())?;
    let row_count = column.offsets.len().saturating_sub(1);
    let nulls = null_buffer(column.validity, row_count)?;
    let array = BinaryArray::try_new(
        OffsetBuffer::new(ScalarBuffer::from(column.offsets)),
        Buffer::from_vec(column.bytes),
        nulls,
    )
    .map_err(arrow_error)?;
    Ok(Arc::new(array))
}

pub fn i64_list_array(column: I64ListColumn) -> Result<ArrayRef> {
    // List arrays map offsets, child values, and optional row validity.
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
    // Assemble the schema and arrays, then enforce the caller response cap.
    let batch = RecordBatch::try_new(schema, columns).map_err(arrow_error)?;
    let observed = batch.columns().iter().fold(0_usize, |len, column| {
        len.saturating_add(column.get_buffer_memory_size())
    });
    if observed > max_arrow_bytes {
        return Err(TransportError::ResponseTooLarge {
            observed,
            cap: max_arrow_bytes,
        });
    }
    Ok(batch)
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
    // Convert MBT validity words into Arrow null buffers after row-count checks.
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
            Ok(Some(NullBuffer::new(BooleanBuffer::new(
                Buffer::from_vec(words),
                0,
                expected_len,
            ))))
        }
        None => Ok(None),
    }
}

fn ensure_offsets(offsets: &[i32], values_len: usize) -> Result<()> {
    // Variable-width and list offsets must be monotonic and end at values_len.
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

fn arrow_error(err: ArrowError) -> TransportError {
    TransportError::MalformedArchive(err.to_string())
}
