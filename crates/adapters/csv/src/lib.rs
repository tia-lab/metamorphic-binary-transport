#![forbid(unsafe_code)]
//! CSV boundary writer for generated MBT metamorphose adapters.

use std::fmt::Write as _;

use mbt_core::error::{Result, TransportError};
use mbt_core::output::{CheckedBytes, utc_bytes, write_base64};

pub struct CsvWriter {
    // Generated adapters write capped CSV bytes directly.
    bytes: CheckedBytes,
}

impl CsvWriter {
    pub fn with_capacity(max_response_bytes: usize, requested_capacity: usize) -> Self {
        Self {
            bytes: CheckedBytes::with_capacity(max_response_bytes, requested_capacity),
        }
    }

    pub fn finish(self) -> Vec<u8> {
        self.bytes.finish()
    }

    pub fn raw_static(&mut self, value: &'static [u8]) -> Result<()> {
        self.bytes.extend_from_slice(value)
    }

    pub fn comma(&mut self) -> Result<()> {
        self.bytes.push(b',')
    }

    pub fn newline(&mut self) -> Result<()> {
        self.bytes.push(b'\n')
    }

    pub fn string_cell(&mut self, value: &str) -> Result<()> {
        // CSV quoting and escaping are boundary concerns.
        self.bytes.push(b'"')?;
        for byte in value.bytes() {
            if byte == b'"' {
                self.bytes.push_str("\"\"")?;
            } else {
                self.bytes.push(byte)?;
            }
        }
        self.bytes.push(b'"')
    }

    pub fn u32_cell(&mut self, value: u32) -> Result<()> {
        let mut buffer = itoa::Buffer::new();
        self.bytes.push_str(buffer.format(value))
    }

    pub fn i32_cell(&mut self, value: i32) -> Result<()> {
        let mut buffer = itoa::Buffer::new();
        self.bytes.push_str(buffer.format(value))
    }

    pub fn i64_cell(&mut self, value: i64) -> Result<()> {
        let mut buffer = itoa::Buffer::new();
        self.bytes.push_str(buffer.format(value))
    }

    pub fn f32_cell(&mut self, field: &'static str, value: f32) -> Result<()> {
        if !value.is_finite() {
            return Err(TransportError::NonFiniteNumeric(field));
        }
        write!(&mut self.bytes, "{value}").map_err(|_| self.bytes.fmt_error())
    }

    pub fn f64_cell(&mut self, field: &'static str, value: f64) -> Result<()> {
        if !value.is_finite() {
            return Err(TransportError::NonFiniteNumeric(field));
        }
        write!(&mut self.bytes, "{value}").map_err(|_| self.bytes.fmt_error())
    }

    pub fn bool_cell(&mut self, value: bool) -> Result<()> {
        if value {
            self.bytes.push_str("true")
        } else {
            self.bytes.push_str("false")
        }
    }

    pub fn bytes_cell(&mut self, value: &[u8]) -> Result<()> {
        // Binary cells use base64 text so CSV remains printable.
        self.bytes.push(b'"')?;
        write_base64(&mut self.bytes, value)?;
        self.bytes.push(b'"')
    }

    pub fn utc_cell(&mut self, ms: i64) -> Result<()> {
        let mut buffer = [0_u8; 64];
        let value = utc_bytes(ms, &mut buffer)?;
        self.bytes.push(b'"')?;
        self.bytes.extend_from_slice(value)?;
        self.bytes.push(b'"')
    }

    pub fn i64_array_cell<I>(&mut self, values: I) -> Result<()>
    where
        I: IntoIterator<Item = i64>,
    {
        self.numeric_array_cell(values, |writer, value| writer.i64_cell(value))
    }

    pub fn i32_array_cell<I>(&mut self, values: I) -> Result<()>
    where
        I: IntoIterator<Item = i32>,
    {
        self.numeric_array_cell(values, |writer, value| writer.i32_cell(value))
    }

    pub fn u32_array_cell<I>(&mut self, values: I) -> Result<()>
    where
        I: IntoIterator<Item = u32>,
    {
        self.numeric_array_cell(values, |writer, value| writer.u32_cell(value))
    }

    pub fn f64_array_cell<I>(&mut self, field: &'static str, values: I) -> Result<()>
    where
        I: IntoIterator<Item = f64>,
    {
        self.numeric_array_cell(values, |writer, value| writer.f64_cell(field, value))
    }

    pub fn f32_array_cell<I>(&mut self, field: &'static str, values: I) -> Result<()>
    where
        I: IntoIterator<Item = f32>,
    {
        self.numeric_array_cell(values, |writer, value| writer.f32_cell(field, value))
    }

    pub fn begin_array_cell(&mut self) -> Result<()> {
        self.bytes.push(b'"')?;
        self.bytes.push(b'[')
    }

    pub fn array_cell_comma(&mut self) -> Result<()> {
        self.bytes.push(b',')
    }

    pub fn end_array_cell(&mut self) -> Result<()> {
        self.bytes.push(b']')?;
        self.bytes.push(b'"')
    }

    fn numeric_array_cell<I, T, F>(&mut self, values: I, mut write_value: F) -> Result<()>
    where
        I: IntoIterator<Item = T>,
        F: FnMut(&mut Self, T) -> Result<()>,
    {
        // Repeated values are stored as a JSON-style payload inside one CSV cell.
        self.begin_array_cell()?;
        let mut first = true;
        for value in values {
            if first {
                first = false;
            } else {
                self.array_cell_comma()?;
            }
            write_value(self, value)?;
        }
        self.end_array_cell()
    }
}

#[cfg(test)]
mod tests;
