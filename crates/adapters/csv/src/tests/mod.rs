use mbt_core::error::{Result, TransportError};

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

#[test]
fn csv_array_strings_roundtrip_with_standard_readers()
-> std::result::Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let controls: String = (0_u8..32).map(char::from).collect();
    let values = [
        "indoor",
        "test",
        "",
        "quote\"slash\\",
        controls.as_str(),
        "café 雪",
    ];
    let mut writer = CsvWriter::with_capacity(4096, 128);
    writer.begin_array_cell()?;
    for (index, value) in values.iter().enumerate() {
        if index != 0 {
            writer.array_cell_comma()?;
        }
        writer.array_string_cell(value)?;
    }
    writer.end_array_cell()?;
    let bytes = writer.finish();
    assert!(bytes.starts_with(b"\"[\"\"indoor\"\",\"\"test\"\""));
    let mut child = Command::new("python3").args(["-c", r#"
import csv, io, json, sys
rows = list(csv.reader(io.StringIO(sys.stdin.read()), strict=True))
assert len(rows) == 1 and len(rows[0]) == 1, rows
assert json.loads(rows[0][0]) == ['indoor', 'test', '', 'quote"slash\\', ''.join(map(chr, range(32))), 'café 雪']
"#]).stdin(Stdio::piped()).spawn()?;
    let mut stdin = child.stdin.take().ok_or("missing child stdin")?;
    stdin.write_all(&bytes)?;
    drop(stdin);
    assert!(child.wait()?.success());
    Ok(())
}

#[test]
fn csv_array_string_cap_checks_every_escape() -> Result<()> {
    let value = "\"\\\n雪";
    let mut reference = CsvWriter::with_capacity(256, 16);
    reference.array_string_cell(value)?;
    let expected = reference.finish();
    for cap in 0..expected.len() {
        let mut writer = CsvWriter::with_capacity(cap, 16);
        assert!(
            matches!(writer.array_string_cell(value), Err(TransportError::ResponseTooLarge { cap: actual, .. }) if actual == cap)
        );
    }
    let mut writer = CsvWriter::with_capacity(expected.len(), 16);
    writer.array_string_cell(value)?;
    assert_eq!(writer.finish(), expected);
    Ok(())
}
