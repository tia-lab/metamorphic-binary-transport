#![forbid(unsafe_code)]
//! Protobuf wire writer for generated MBT metamorphose adapters.

use prost::bytes::BufMut;
use prost::encoding::{self, WireType};

use metamorphic_binary_transport_core::error::{Result, TransportError};
use metamorphic_binary_transport_core::output::{CheckedBytes, utc_bytes};

pub struct ProtoWriter {
    bytes: CheckedBytes,
}

impl ProtoWriter {
    pub fn with_capacity(max_response_bytes: usize, requested_capacity: usize) -> Self {
        Self {
            bytes: CheckedBytes::with_capacity(max_response_bytes, requested_capacity),
        }
    }

    pub fn finish(self) -> Vec<u8> {
        self.bytes.finish()
    }

    pub fn uint32(&mut self, tag: u32, value: u32) -> Result<()> {
        self.bytes
            .encode_protobuf(|buf| encoding::uint32::encode(tag, &value, buf))
    }

    pub fn uint64(&mut self, tag: u32, value: u64) -> Result<()> {
        self.bytes
            .encode_protobuf(|buf| encoding::uint64::encode(tag, &value, buf))
    }

    pub fn int32(&mut self, tag: u32, value: i32) -> Result<()> {
        self.bytes
            .encode_protobuf(|buf| encoding::int32::encode(tag, &value, buf))
    }

    pub fn int64(&mut self, tag: u32, value: i64) -> Result<()> {
        self.bytes
            .encode_protobuf(|buf| encoding::int64::encode(tag, &value, buf))
    }

    pub fn float(&mut self, tag: u32, field: &'static str, value: f32) -> Result<()> {
        if !value.is_finite() {
            return Err(TransportError::NonFiniteNumeric(field));
        }
        self.bytes
            .encode_protobuf(|buf| encoding::float::encode(tag, &value, buf))
    }

    pub fn double(&mut self, tag: u32, field: &'static str, value: f64) -> Result<()> {
        if !value.is_finite() {
            return Err(TransportError::NonFiniteNumeric(field));
        }
        self.bytes
            .encode_protobuf(|buf| encoding::double::encode(tag, &value, buf))
    }

    pub fn bool(&mut self, tag: u32, value: bool) -> Result<()> {
        self.bytes
            .encode_protobuf(|buf| encoding::bool::encode(tag, &value, buf))
    }

    pub fn string(&mut self, tag: u32, value: &str) -> Result<()> {
        self.bytes.encode_protobuf(|buf| {
            encoding::encode_key(tag, WireType::LengthDelimited, buf);
            encoding::encode_varint(value.len() as u64, buf);
            buf.put_slice(value.as_bytes());
        })
    }

    pub fn bytes(&mut self, tag: u32, value: &[u8]) -> Result<()> {
        self.bytes.encode_protobuf(|buf| {
            encoding::encode_key(tag, WireType::LengthDelimited, buf);
            encoding::encode_varint(value.len() as u64, buf);
            buf.put_slice(value);
        })
    }

    pub fn utc(&mut self, tag: u32, ms: i64) -> Result<()> {
        let mut bytes = [0_u8; 64];
        let value = utc_bytes(ms, &mut bytes)?;
        self.bytes.encode_protobuf(|buf| {
            encoding::encode_key(tag, WireType::LengthDelimited, buf);
            encoding::encode_varint(value.len() as u64, buf);
            buf.put_slice(value);
        })
    }

    pub fn message_prefix(&mut self, tag: u32, len: usize) -> Result<()> {
        self.bytes.encode_protobuf(|buf| {
            encoding::encode_key(tag, WireType::LengthDelimited, buf);
            encoding::encode_varint(len as u64, buf);
        })
    }
}

pub fn encoded_len_uint32(tag: u32, value: u32) -> usize {
    encoding::uint32::encoded_len(tag, &value)
}

pub fn encoded_len_uint64(tag: u32, value: u64) -> usize {
    encoding::uint64::encoded_len(tag, &value)
}

pub fn encoded_len_int32(tag: u32, value: i32) -> usize {
    encoding::int32::encoded_len(tag, &value)
}

pub fn encoded_len_int64(tag: u32, value: i64) -> usize {
    encoding::int64::encoded_len(tag, &value)
}

pub fn encoded_len_float(tag: u32, value: f32) -> usize {
    encoding::float::encoded_len(tag, &value)
}

pub fn encoded_len_double(tag: u32, value: f64) -> usize {
    encoding::double::encoded_len(tag, &value)
}

pub fn encoded_len_bool(tag: u32, value: bool) -> usize {
    encoding::bool::encoded_len(tag, &value)
}

#[cfg(test)]
mod tests;
