use crate::error::{Result, TransportError};
use crate::output::{
    CheckedBytes, checked_len_add, encoded_len_message, encoded_len_string, utc_bytes, utc_len,
    write_base64, write_utc,
};

#[test]
fn checked_bytes_enforces_response_cap() -> Result<()> {
    let mut bytes = CheckedBytes::new(3);
    bytes.extend_from_slice(b"abc")?;
    assert_eq!(
        bytes.push(b'd').err(),
        Some(TransportError::ResponseTooLarge {
            observed: 4,
            cap: 3,
        })
    );
    Ok(())
}

#[test]
fn checked_len_add_reports_overflow_and_cap() {
    assert_eq!(
        checked_len_add(2, 3, 4).err(),
        Some(TransportError::ResponseTooLarge {
            observed: 5,
            cap: 4,
        })
    );
    assert_eq!(
        checked_len_add(usize::MAX, 1, usize::MAX).err(),
        Some(TransportError::ResponseTooLarge {
            observed: usize::MAX,
            cap: usize::MAX,
        })
    );
}

#[test]
fn utc_helpers_format_epoch_without_allocation() -> Result<()> {
    let mut out = CheckedBytes::new(32);
    write_utc(&mut out, 0)?;
    assert_eq!(out.finish(), b"1970-01-01T00:00:00Z");

    let mut buffer = [0_u8; 64];
    assert_eq!(
        utc_bytes(1_700_000_000_000, &mut buffer)?,
        b"2023-11-14T22:13:20Z"
    );
    assert_eq!(utc_len(1_700_000_000_000)?, 20);
    Ok(())
}

#[test]
fn base64_writer_handles_padding() -> Result<()> {
    let mut one = CheckedBytes::new(8);
    write_base64(&mut one, b"M")?;
    assert_eq!(one.finish(), b"TQ==");

    let mut two = CheckedBytes::new(8);
    write_base64(&mut two, b"Ma")?;
    assert_eq!(two.finish(), b"TWE=");

    let mut three = CheckedBytes::new(8);
    write_base64(&mut three, b"Man")?;
    assert_eq!(three.finish(), b"TWFu");
    Ok(())
}

#[test]
fn protobuf_length_helpers_match_wire_shape() {
    assert_eq!(encoded_len_string(1, "abc"), 5);
    assert_eq!(encoded_len_message(1, 3), 5);
    assert_eq!(encoded_len_string(16, "abc"), 6);
}
