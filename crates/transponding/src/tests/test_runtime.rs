use mbt_core::error::{Result, TransportError};

use crate::{
    F64Column, I32ListColumn, OptionalI64Column, ValidityBitmap, checksum_seed,
    ensure_columnar_size,
};

#[test]
fn validity_bitmap_tracks_present_rows() -> Result<()> {
    let mut validity = ValidityBitmap::new(130);
    validity.set_present(0)?;
    validity.set_present(65)?;
    validity.set_present(129)?;

    assert!(validity.is_present(0));
    assert!(validity.is_present(65));
    assert!(validity.is_present(129));
    assert!(!validity.is_present(64));
    assert!(!validity.is_present(130));
    assert_eq!(validity.byte_len(), 24);
    Ok(())
}

#[test]
fn validity_bitmap_rejects_out_of_bounds_present_bit() {
    let mut validity = ValidityBitmap::new(1);
    assert_eq!(
        validity.set_present(1).err(),
        Some(TransportError::RowCountMismatch {
            observed: 2,
            expected: 1,
        })
    );
}

#[test]
fn optional_numeric_column_keeps_defaults_and_validity_separate() -> Result<()> {
    let mut column = OptionalI64Column::new(3);
    column.push_optional(true, 10)?;
    column.push_optional(false, 0)?;
    column.push_optional(true, -7)?;

    assert_eq!(column.values, vec![10, 0, -7]);
    assert!(column.validity.is_present(0));
    assert!(!column.validity.is_present(1));
    assert!(column.validity.is_present(2));
    Ok(())
}

#[test]
fn list_column_preserves_null_versus_empty_array() -> Result<()> {
    let mut column = I32ListColumn::optional(3);
    column.push_optional(true, [1, 2])?;
    column.push_optional(true, [])?;
    column.push_optional(false, [9])?;

    assert_eq!(column.offsets, vec![0, 2, 2, 2]);
    assert_eq!(column.values, vec![1, 2]);
    match column.validity.as_ref() {
        Some(validity) => {
            assert!(validity.is_present(0));
            assert!(validity.is_present(1));
            assert!(!validity.is_present(2));
        }
        None => {
            return Err(TransportError::MalformedArchive(
                "missing validity".to_string(),
            ));
        }
    }
    Ok(())
}

#[test]
fn checksum_and_size_helpers_are_deterministic() {
    let mut column = F64Column::new(2);
    column.push_required(1.5);
    column.push_required(2.5);

    assert_eq!(
        column.checksum_with_name(checksum_seed("columns"), "value"),
        column.checksum_with_name(checksum_seed("columns"), "value")
    );
    assert_eq!(
        ensure_columnar_size(8, 7).err(),
        Some(TransportError::ResponseTooLarge {
            observed: 8,
            cap: 7,
        })
    );
}
