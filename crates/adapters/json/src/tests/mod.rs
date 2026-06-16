use mbt_core::error::{Result, TransportError};

use crate::JsonWriter;

#[test]
fn json_writer_streams_object_without_serde() -> Result<()> {
    let mut writer = JsonWriter::with_capacity(256, 64);
    writer.begin_object()?;
    writer.raw_static(br#""s":"#)?;
    writer.string_value("a\"b\n")?;
    writer.comma()?;
    writer.raw_static(br#""n":"#)?;
    writer.i64_value(-7)?;
    writer.comma()?;
    writer.raw_static(br#""b":"#)?;
    writer.bool_value(true)?;
    writer.comma()?;
    writer.raw_static(br#""bytes":"#)?;
    writer.bytes_value(b"Ma")?;
    writer.comma()?;
    writer.raw_static(br#""arr":"#)?;
    writer.i32_array_value([1, -2, 3])?;
    writer.end_object()?;

    assert_eq!(
        writer.finish(),
        br#"{"s":"a\"b\n","n":-7,"b":true,"bytes":"TWE=","arr":[1,-2,3]}"#
    );
    Ok(())
}

#[test]
fn json_writer_rejects_non_finite_numbers() {
    let mut writer = JsonWriter::with_capacity(64, 16);
    assert_eq!(
        writer.f64_value("bad", f64::NAN).err(),
        Some(TransportError::NonFiniteNumeric("bad"))
    );
}

#[test]
fn json_writer_enforces_cap() {
    let mut writer = JsonWriter::with_capacity(2, 16);
    assert_eq!(
        writer.raw_static(b"abc").err(),
        Some(TransportError::ResponseTooLarge {
            observed: 3,
            cap: 2,
        })
    );
}
