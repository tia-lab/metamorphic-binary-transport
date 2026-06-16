use mbt_core::error::{Result, TransportError};

use crate::ProtoWriter;

#[test]
fn protobuf_writer_emits_wire_bytes_without_dto() -> Result<()> {
    let mut writer = ProtoWriter::with_capacity(256, 64);
    writer.uint32(1, 150)?;
    writer.string(2, "abc")?;
    writer.bool(3, true)?;
    writer.bytes(4, b"xy")?;

    assert_eq!(
        writer.finish(),
        vec![
            0x08, 0x96, 0x01, 0x12, 0x03, b'a', b'b', b'c', 0x18, 0x01, 0x22, 0x02, b'x', b'y',
        ]
    );
    Ok(())
}

#[test]
fn protobuf_writer_emits_utc_string_field() -> Result<()> {
    let mut writer = ProtoWriter::with_capacity(64, 32);
    writer.utc(1, 0)?;
    assert_eq!(
        writer.finish(),
        [&[0x0a, 20][..], b"1970-01-01T00:00:00Z".as_slice(),].concat()
    );
    Ok(())
}

#[test]
fn protobuf_writer_rejects_non_finite_numbers() {
    let mut writer = ProtoWriter::with_capacity(64, 16);
    assert_eq!(
        writer.float(1, "bad", f32::INFINITY).err(),
        Some(TransportError::NonFiniteNumeric("bad"))
    );
}

#[test]
fn protobuf_writer_enforces_cap_after_encode() {
    let mut writer = ProtoWriter::with_capacity(1, 1);
    assert_eq!(
        writer.string(1, "abc").err(),
        Some(TransportError::ResponseTooLarge {
            observed: 5,
            cap: 1,
        })
    );
}
