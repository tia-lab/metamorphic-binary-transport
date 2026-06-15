use metamorphic_binary_transport_core::error::{Result, TransportError};

use crate::CsvWriter;

#[test]
fn csv_writer_streams_cells_without_csv_crate() -> Result<()> {
    let mut writer = CsvWriter::with_capacity(256, 64);
    writer.raw_static(b"symbol,value,bytes,array\n")?;
    writer.string_cell("BTC\"USDT")?;
    writer.comma()?;
    writer.f64_cell("value", 12.5)?;
    writer.comma()?;
    writer.bytes_cell(b"Man")?;
    writer.comma()?;
    writer.i64_array_cell([1, 2, 3])?;
    writer.newline()?;

    assert_eq!(
        writer.finish(),
        br#"symbol,value,bytes,array
"BTC""USDT",12.5,"TWFu","[1,2,3]"
"#
    );
    Ok(())
}

#[test]
fn csv_writer_rejects_non_finite_numbers() {
    let mut writer = CsvWriter::with_capacity(64, 16);
    assert_eq!(
        writer.f32_cell("bad", f32::NAN).err(),
        Some(TransportError::NonFiniteNumeric("bad"))
    );
}

#[test]
fn csv_writer_enforces_cap() {
    let mut writer = CsvWriter::with_capacity(2, 16);
    assert_eq!(
        writer.raw_static(b"abc").err(),
        Some(TransportError::ResponseTooLarge {
            observed: 3,
            cap: 2,
        })
    );
}
