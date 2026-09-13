use std::sync::Arc;

use arrow_array::{Array, Float64Array, Int32Array, Int64Array, ListArray, StringArray};
use arrow_schema::{DataType, Field, Schema};
use mbt_core::error::TransportError;
use mbt_transponding::{F64Column, I32ListColumn, OptionalI64Column, Utf8Column};

use crate::{
    f64_array, i32_list_array, optional_i64_array, record_batch, record_batch_checksum, utf8_array,
};

#[test]
fn required_numeric_column_becomes_arrow_array() {
    let mut column = F64Column::new(2);
    column.push_required(1.25);
    column.push_required(2.5);

    let array = f64_array(column).unwrap_or_else(|err| panic!("{err}"));
    let Some(values) = array.as_any().downcast_ref::<Float64Array>() else {
        panic!("expected Float64Array");
    };

    assert_eq!(values.len(), 2);
    assert_eq!(values.value(0), 1.25);
    assert_eq!(values.value(1), 2.5);
}

#[test]
fn optional_numeric_column_preserves_nulls() {
    let mut column = OptionalI64Column::new(3);
    column
        .push_optional(true, 10)
        .unwrap_or_else(|err| panic!("{err}"));
    column
        .push_optional(false, 0)
        .unwrap_or_else(|err| panic!("{err}"));
    column
        .push_optional(true, -4)
        .unwrap_or_else(|err| panic!("{err}"));

    let array = optional_i64_array(column).unwrap_or_else(|err| panic!("{err}"));
    let Some(values) = array.as_any().downcast_ref::<Int64Array>() else {
        panic!("expected Int64Array");
    };

    assert_eq!(values.len(), 3);
    assert!(!values.is_null(0));
    assert!(values.is_null(1));
    assert!(!values.is_null(2));
    assert_eq!(values.value(0), 10);
    assert_eq!(values.value(2), -4);
}

#[test]
fn utf8_and_list_columns_preserve_offsets() {
    let mut names = Utf8Column::required(2);
    names
        .push_required("sensor_a")
        .unwrap_or_else(|err| panic!("{err}"));
    names
        .push_required("eth")
        .unwrap_or_else(|err| panic!("{err}"));
    let names = utf8_array(names).unwrap_or_else(|err| panic!("{err}"));
    let Some(names) = names.as_any().downcast_ref::<StringArray>() else {
        panic!("expected StringArray");
    };
    assert_eq!(names.value(0), "sensor_a");
    assert_eq!(names.value(1), "eth");

    let mut lists = I32ListColumn::required(2);
    lists
        .push_required([1, 2])
        .unwrap_or_else(|err| panic!("{err}"));
    lists
        .push_required([3])
        .unwrap_or_else(|err| panic!("{err}"));
    let lists = i32_list_array(lists).unwrap_or_else(|err| panic!("{err}"));
    let Some(lists) = lists.as_any().downcast_ref::<ListArray>() else {
        panic!("expected ListArray");
    };
    assert_eq!(lists.value_offsets(), &[0, 2, 3]);
    let Some(values) = lists.values().as_any().downcast_ref::<Int32Array>() else {
        panic!("expected Int32Array child");
    };
    assert_eq!(values.value(0), 1);
    assert_eq!(values.value(1), 2);
    assert_eq!(values.value(2), 3);
}

#[test]
fn record_batch_enforces_size_cap_and_has_stable_checksum() {
    let mut column = F64Column::new(2);
    column.push_required(1.0);
    column.push_required(2.0);
    let array = f64_array(column).unwrap_or_else(|err| panic!("{err}"));
    let schema = Arc::new(Schema::new(vec![Field::new(
        "value",
        DataType::Float64,
        false,
    )]));

    let batch = record_batch(Arc::clone(&schema), vec![Arc::clone(&array)], usize::MAX)
        .unwrap_or_else(|err| panic!("{err}"));
    assert_eq!(batch.num_rows(), 2);

    let checksum_a = record_batch_checksum(&batch).unwrap_or_else(|err| panic!("{err}"));
    let checksum_b = record_batch_checksum(&batch).unwrap_or_else(|err| panic!("{err}"));
    assert_eq!(checksum_a, checksum_b);

    let err = record_batch(schema, vec![array], 0).unwrap_err();
    assert!(matches!(err, TransportError::ResponseTooLarge { .. }));
}
