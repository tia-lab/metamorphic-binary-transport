use std::fmt;

use crate::error::{Result, TransportError};

/// Cap-aware byte buffer shared by generated boundary writers.
pub struct CheckedBytes {
    bytes: Vec<u8>,
    cap: usize,
    overflow_observed: Option<usize>,
}

impl CheckedBytes {
    pub fn new(cap: usize) -> Self {
        Self {
            bytes: Vec::new(),
            cap,
            overflow_observed: None,
        }
    }

    pub fn with_capacity(cap: usize, requested_capacity: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(requested_capacity.min(cap)),
            cap,
            overflow_observed: None,
        }
    }

    pub fn push(&mut self, byte: u8) -> Result<()> {
        let observed = self
            .bytes
            .len()
            .checked_add(1)
            .ok_or(TransportError::ResponseTooLarge {
                observed: usize::MAX,
                cap: self.cap,
            })?;
        self.ensure_len(observed)?;
        self.bytes.push(byte);
        Ok(())
    }

    pub fn push_str(&mut self, value: &str) -> Result<()> {
        self.extend_from_slice(value.as_bytes())
    }

    pub fn extend_from_slice(&mut self, value: &[u8]) -> Result<()> {
        let observed =
            self.bytes
                .len()
                .checked_add(value.len())
                .ok_or(TransportError::ResponseTooLarge {
                    observed: usize::MAX,
                    cap: self.cap,
                })?;
        self.ensure_len(observed)?;
        self.bytes.extend_from_slice(value);
        Ok(())
    }

    pub fn encode_protobuf<F>(&mut self, encode: F) -> Result<()>
    where
        F: FnOnce(&mut Vec<u8>),
    {
        // Prost appends directly into the owned response buffer; the cap is checked after append.
        encode(&mut self.bytes);
        self.ensure_len(self.bytes.len())
    }

    pub fn finish(self) -> Vec<u8> {
        self.bytes
    }

    fn ensure_len(&mut self, observed: usize) -> Result<()> {
        if observed > self.cap {
            self.overflow_observed = Some(observed);
            return Err(TransportError::ResponseTooLarge {
                observed,
                cap: self.cap,
            });
        }
        Ok(())
    }

    pub fn fmt_error(&self) -> TransportError {
        // fmt::Write can only return fmt::Error, so recover the response-cap error here.
        let observed = match self.overflow_observed {
            Some(value) => value,
            None => self.cap.saturating_add(1),
        };
        TransportError::ResponseTooLarge {
            observed,
            cap: self.cap,
        }
    }
}

impl fmt::Write for CheckedBytes {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.push_str(s).map_err(|_| fmt::Error)
    }
}

pub fn checked_len_add(lhs: usize, rhs: usize, cap: usize) -> Result<usize> {
    // Protobuf writers preflight nested lengths before emitting length-delimited fields.
    let observed = lhs
        .checked_add(rhs)
        .ok_or(TransportError::ResponseTooLarge {
            observed: usize::MAX,
            cap,
        })?;
    if observed > cap {
        return Err(TransportError::ResponseTooLarge { observed, cap });
    }
    Ok(observed)
}

pub fn encoded_len_string(tag: u32, value: &str) -> usize {
    encoded_len_key(tag, 2) + encoded_len_varint(value.len() as u64) + value.len()
}

pub fn encoded_len_message(tag: u32, len: usize) -> usize {
    encoded_len_key(tag, 2) + encoded_len_varint(len as u64) + len
}

fn encoded_len_key(tag: u32, wire_type: u8) -> usize {
    encoded_len_varint((u64::from(tag) << 3) | u64::from(wire_type))
}

fn encoded_len_varint(mut value: u64) -> usize {
    let mut len = 1;
    while value >= 0x80 {
        value >>= 7;
        len += 1;
    }
    len
}

pub fn utc_len(ms: i64) -> Result<usize> {
    let mut bytes = [0_u8; 64];
    utc_bytes(ms, &mut bytes).map(|value| value.len())
}

pub fn write_utc(out: &mut CheckedBytes, ms: i64) -> Result<()> {
    // Write UTC text directly into the capped output without allocating a String.
    let seconds = ms.div_euclid(1_000);
    let days = seconds.div_euclid(86_400);
    let sod = seconds.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    let hour = sod / 3_600;
    let minute = (sod % 3_600) / 60;
    let second = sod % 60;
    write_i64_padded(out, year, 4)?;
    out.push(b'-')?;
    write_i64_padded(out, month, 2)?;
    out.push(b'-')?;
    write_i64_padded(out, day, 2)?;
    out.push(b'T')?;
    write_i64_padded(out, hour, 2)?;
    out.push(b':')?;
    write_i64_padded(out, minute, 2)?;
    out.push(b':')?;
    write_i64_padded(out, second, 2)?;
    out.push(b'Z')
}

pub fn utc_bytes(ms: i64, out: &mut [u8; 64]) -> Result<&[u8]> {
    // Stack-buffer variant for writers that need a borrowed byte slice.
    let seconds = ms.div_euclid(1_000);
    let days = seconds.div_euclid(86_400);
    let sod = seconds.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    let hour = sod / 3_600;
    let minute = (sod % 3_600) / 60;
    let second = sod % 60;
    let mut idx = 0;
    write_i64_padded_bytes(out, &mut idx, year, 4)?;
    write_byte(out, &mut idx, b'-')?;
    write_i64_padded_bytes(out, &mut idx, month, 2)?;
    write_byte(out, &mut idx, b'-')?;
    write_i64_padded_bytes(out, &mut idx, day, 2)?;
    write_byte(out, &mut idx, b'T')?;
    write_i64_padded_bytes(out, &mut idx, hour, 2)?;
    write_byte(out, &mut idx, b':')?;
    write_i64_padded_bytes(out, &mut idx, minute, 2)?;
    write_byte(out, &mut idx, b':')?;
    write_i64_padded_bytes(out, &mut idx, second, 2)?;
    write_byte(out, &mut idx, b'Z')?;
    Ok(&out[..idx])
}

pub fn write_base64(out: &mut CheckedBytes, value: &[u8]) -> Result<()> {
    // Bytes become text only at row-format boundaries.
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    for chunk in value.chunks(3) {
        let b0 = chunk[0];
        let b1 = match chunk.get(1) {
            Some(value) => *value,
            None => 0,
        };
        let b2 = match chunk.get(2) {
            Some(value) => *value,
            None => 0,
        };
        out.push(TABLE[usize::from(b0 >> 2)])?;
        out.push(TABLE[usize::from(((b0 & 0x03) << 4) | (b1 >> 4))])?;
        match chunk.len() {
            1 => {
                out.push(b'=')?;
                out.push(b'=')?;
            }
            2 => {
                out.push(TABLE[usize::from(((b1 & 0x0f) << 2) | (b2 >> 6))])?;
                out.push(b'=')?;
            }
            _ => {
                out.push(TABLE[usize::from(((b1 & 0x0f) << 2) | (b2 >> 6))])?;
                out.push(TABLE[usize::from(b2 & 0x3f)])?;
            }
        }
    }
    Ok(())
}

fn write_i64_padded(out: &mut CheckedBytes, value: i64, min_width: usize) -> Result<()> {
    let mut bytes = [0_u8; 32];
    let mut idx = 0;
    write_i64_padded_bytes(&mut bytes, &mut idx, value, min_width)?;
    out.extend_from_slice(&bytes[..idx])
}

fn write_i64_padded_bytes(
    out: &mut [u8],
    idx: &mut usize,
    value: i64,
    min_width: usize,
) -> Result<()> {
    let negative = value < 0;
    let mut remaining = if negative {
        -(i128::from(value))
    } else {
        i128::from(value)
    };
    let mut digits = [0_u8; 40];
    let mut count = 0_usize;
    loop {
        let digit = (remaining % 10) as u8;
        digits[count] = b'0' + digit;
        count += 1;
        remaining /= 10;
        if remaining == 0 {
            break;
        }
    }
    if negative {
        write_byte(out, idx, b'-')?;
    }
    let zero_count = min_width.saturating_sub(count);
    for _ in 0..zero_count {
        write_byte(out, idx, b'0')?;
    }
    for digit in digits[..count].iter().rev() {
        write_byte(out, idx, *digit)?;
    }
    Ok(())
}

fn write_byte(out: &mut [u8], idx: &mut usize, byte: u8) -> Result<()> {
    if *idx >= out.len() {
        return Err(TransportError::InvalidTimeGrid(
            "UTC formatting buffer overflow".to_string(),
        ));
    }
    out[*idx] = byte;
    *idx += 1;
    Ok(())
}

fn civil_from_days(days_since_epoch: i64) -> (i64, i64, i64) {
    // Local civil-date conversion avoids adding a time dependency to the core crate.
    let z = days_since_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if m <= 2 { 1 } else { 0 };
    (year, m, d)
}
