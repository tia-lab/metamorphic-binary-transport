#![forbid(unsafe_code)]
//! JSON boundary writer for generated MBT metamorphose adapters.

use std::fmt::Write as _;

use metamorphic_binary_transport_core::error::{Result, TransportError};
use metamorphic_binary_transport_core::output::{CheckedBytes, write_base64, write_utc};

pub struct JsonWriter {
    bytes: CheckedBytes,
}

impl JsonWriter {
    pub fn with_capacity(max_response_bytes: usize, requested_capacity: usize) -> Self {
        Self {
            bytes: CheckedBytes::with_capacity(max_response_bytes, requested_capacity),
        }
    }

    pub fn finish(self) -> Vec<u8> {
        self.bytes.finish()
    }

    pub fn begin_object(&mut self) -> Result<()> {
        self.bytes.push(b'{')
    }

    pub fn end_object(&mut self) -> Result<()> {
        self.bytes.push(b'}')
    }

    pub fn begin_array(&mut self) -> Result<()> {
        self.bytes.push(b'[')
    }

    pub fn end_array(&mut self) -> Result<()> {
        self.bytes.push(b']')
    }

    pub fn comma(&mut self) -> Result<()> {
        self.bytes.push(b',')
    }

    pub fn raw_static(&mut self, value: &'static [u8]) -> Result<()> {
        self.bytes.extend_from_slice(value)
    }

    pub fn string_value(&mut self, value: &str) -> Result<()> {
        self.bytes.push(b'"')?;
        for byte in value.bytes() {
            match byte {
                b'"' => self.bytes.push_str("\\\"")?,
                b'\\' => self.bytes.push_str("\\\\")?,
                b'\n' => self.bytes.push_str("\\n")?,
                b'\r' => self.bytes.push_str("\\r")?,
                b'\t' => self.bytes.push_str("\\t")?,
                0x08 => self.bytes.push_str("\\b")?,
                0x0c => self.bytes.push_str("\\f")?,
                0x00..=0x1f => {
                    self.bytes.push_str("\\u00")?;
                    self.hex_byte(byte)?;
                }
                _ => self.bytes.push(byte)?,
            }
        }
        self.bytes.push(b'"')
    }

    pub fn u32_value(&mut self, value: u32) -> Result<()> {
        write!(&mut self.bytes, "{value}").map_err(|_| self.bytes.fmt_error())
    }

    pub fn i32_value(&mut self, value: i32) -> Result<()> {
        write!(&mut self.bytes, "{value}").map_err(|_| self.bytes.fmt_error())
    }

    pub fn i64_value(&mut self, value: i64) -> Result<()> {
        write!(&mut self.bytes, "{value}").map_err(|_| self.bytes.fmt_error())
    }

    pub fn f32_value(&mut self, field: &'static str, value: f32) -> Result<()> {
        if !value.is_finite() {
            return Err(TransportError::NonFiniteNumeric(field));
        }
        write!(&mut self.bytes, "{value}").map_err(|_| self.bytes.fmt_error())
    }

    pub fn f64_value(&mut self, field: &'static str, value: f64) -> Result<()> {
        if !value.is_finite() {
            return Err(TransportError::NonFiniteNumeric(field));
        }
        write!(&mut self.bytes, "{value}").map_err(|_| self.bytes.fmt_error())
    }

    pub fn bool_value(&mut self, value: bool) -> Result<()> {
        if value {
            self.bytes.push_str("true")
        } else {
            self.bytes.push_str("false")
        }
    }

    pub fn bytes_value(&mut self, value: &[u8]) -> Result<()> {
        self.bytes.push(b'"')?;
        write_base64(&mut self.bytes, value)?;
        self.bytes.push(b'"')
    }

    pub fn utc_value(&mut self, ms: i64) -> Result<()> {
        self.bytes.push(b'"')?;
        write_utc(&mut self.bytes, ms)?;
        self.bytes.push(b'"')
    }

    pub fn i64_array_value<I>(&mut self, values: I) -> Result<()>
    where
        I: IntoIterator<Item = i64>,
    {
        self.numeric_array(values, |writer, value| writer.i64_value(value))
    }

    pub fn i32_array_value<I>(&mut self, values: I) -> Result<()>
    where
        I: IntoIterator<Item = i32>,
    {
        self.numeric_array(values, |writer, value| writer.i32_value(value))
    }

    pub fn u32_array_value<I>(&mut self, values: I) -> Result<()>
    where
        I: IntoIterator<Item = u32>,
    {
        self.numeric_array(values, |writer, value| writer.u32_value(value))
    }

    pub fn f64_array_value<I>(&mut self, field: &'static str, values: I) -> Result<()>
    where
        I: IntoIterator<Item = f64>,
    {
        self.numeric_array(values, |writer, value| writer.f64_value(field, value))
    }

    pub fn f32_array_value<I>(&mut self, field: &'static str, values: I) -> Result<()>
    where
        I: IntoIterator<Item = f32>,
    {
        self.numeric_array(values, |writer, value| writer.f32_value(field, value))
    }

    fn hex_byte(&mut self, byte: u8) -> Result<()> {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        self.bytes.push(HEX[usize::from(byte >> 4)])?;
        self.bytes.push(HEX[usize::from(byte & 0x0f)])
    }

    fn numeric_array<I, T, F>(&mut self, values: I, mut write_value: F) -> Result<()>
    where
        I: IntoIterator<Item = T>,
        F: FnMut(&mut Self, T) -> Result<()>,
    {
        self.begin_array()?;
        let mut first = true;
        for value in values {
            if first {
                first = false;
            } else {
                self.comma()?;
            }
            write_value(self, value)?;
        }
        self.end_array()
    }
}

#[cfg(test)]
mod tests;
