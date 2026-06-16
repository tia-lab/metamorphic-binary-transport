use std::mem::size_of;

use mbt_core::envelope::{FNV_OFFSET, FNV_PRIME};
use mbt_core::error::{Result, TransportError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidityBitmap {
    // One bit per row tracks optional/null state outside the physical values.
    pub words: Vec<u64>,
    pub len: usize,
}

impl ValidityBitmap {
    pub fn new(len: usize) -> Self {
        let words = if len == 0 { 0 } else { ((len - 1) / 64) + 1 };
        Self {
            words: vec![0; words],
            len,
        }
    }

    pub fn set_present(&mut self, idx: usize) -> Result<()> {
        if idx >= self.len {
            return Err(TransportError::RowCountMismatch {
                observed: idx.saturating_add(1),
                expected: self.len as u64,
            });
        }
        let word_idx = idx / 64;
        let bit_idx = idx % 64;
        let Some(word) = self.words.get_mut(word_idx) else {
            return Err(TransportError::RowCountMismatch {
                observed: idx.saturating_add(1),
                expected: self.len as u64,
            });
        };
        *word |= 1_u64 << bit_idx;
        Ok(())
    }

    pub fn is_present(&self, idx: usize) -> bool {
        if idx >= self.len {
            return false;
        }
        let word_idx = idx / 64;
        let bit_idx = idx % 64;
        match self.words.get(word_idx) {
            Some(word) => word & (1_u64 << bit_idx) != 0,
            None => false,
        }
    }

    pub fn byte_len(&self) -> usize {
        self.words.len() * size_of::<u64>()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn into_words(self) -> Vec<u64> {
        self.words
    }

    pub fn checksum_with_name(&self, mut hash: u64, name: &str) -> u64 {
        hash = update_str(hash, name);
        hash = update_usize(hash, self.len);
        hash = update_usize(hash, self.words.len());
        for word in &self.words {
            hash = update_u64(hash, *word);
        }
        hash
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DictionaryMeta {
    // Columnar adapters carry dictionary identity as metadata, not duplicated values.
    pub name: &'static str,
    pub values: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstU16Column {
    // Constant dictionary columns store the value once with the row count.
    pub value: u16,
    pub len: usize,
}

impl ConstU16Column {
    pub fn new(value: u16, len: usize) -> Self {
        Self { value, len }
    }

    pub fn byte_len(&self) -> usize {
        size_of::<u16>() + size_of::<usize>()
    }

    pub fn checksum_with_name(&self, mut hash: u64, name: &str) -> u64 {
        hash = update_str(hash, name);
        hash = update_u16(hash, self.value);
        update_usize(hash, self.len)
    }
}

macro_rules! required_numeric_column {
    // Required primitive columns share values-only storage across numeric types.
    ($name:ident, $ty:ty, $update:ident) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name {
            pub values: Vec<$ty>,
        }

        impl $name {
            pub fn new(capacity: usize) -> Self {
                Self {
                    values: Vec::with_capacity(capacity),
                }
            }

            pub fn push_required(&mut self, value: $ty) {
                self.values.push(value);
            }

            pub fn byte_len(&self) -> usize {
                self.values.len() * size_of::<$ty>()
            }

            pub fn checksum_with_name(&self, mut hash: u64, name: &str) -> u64 {
                hash = update_str(hash, name);
                hash = update_usize(hash, self.values.len());
                hash = update_u8(hash, 0);
                for value in &self.values {
                    hash = $update(hash, *value);
                }
                hash
            }
        }
    };
}

macro_rules! optional_numeric_column {
    // Optional primitives keep physical defaults separate from validity bits.
    ($name:ident, $ty:ty, $update:ident) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name {
            pub values: Vec<$ty>,
            pub validity: ValidityBitmap,
        }

        impl $name {
            pub fn new(capacity: usize) -> Self {
                Self {
                    values: Vec::with_capacity(capacity),
                    validity: ValidityBitmap::new(capacity),
                }
            }

            pub fn push_optional(&mut self, present: bool, value: $ty) -> Result<()> {
                let idx = self.values.len();
                if present {
                    self.validity.set_present(idx)?;
                }
                self.values.push(value);
                Ok(())
            }

            pub fn byte_len(&self) -> usize {
                (self.values.len() * size_of::<$ty>()) + self.validity.byte_len()
            }

            pub fn checksum_with_name(&self, mut hash: u64, name: &str) -> u64 {
                hash = update_str(hash, name);
                hash = update_usize(hash, self.values.len());
                hash = update_u8(hash, 1);
                hash = self.validity.checksum_with_name(hash, "validity");
                for value in &self.values {
                    hash = $update(hash, *value);
                }
                hash
            }
        }
    };
}

required_numeric_column!(U16Column, u16, update_u16);
required_numeric_column!(I32Column, i32, update_i32);
required_numeric_column!(U32Column, u32, update_u32);
required_numeric_column!(U64Column, u64, update_u64);
required_numeric_column!(I64Column, i64, update_i64);
required_numeric_column!(F32Column, f32, update_f32);
required_numeric_column!(F64Column, f64, update_f64);
optional_numeric_column!(OptionalU16Column, u16, update_u16);
optional_numeric_column!(OptionalI32Column, i32, update_i32);
optional_numeric_column!(OptionalU32Column, u32, update_u32);
optional_numeric_column!(OptionalU64Column, u64, update_u64);
optional_numeric_column!(OptionalI64Column, i64, update_i64);
optional_numeric_column!(OptionalF32Column, f32, update_f32);
optional_numeric_column!(OptionalF64Column, f64, update_f64);

macro_rules! numeric_list_column {
    // List columns preserve offsets, values, and optional null-versus-empty rows.
    ($name:ident, $ty:ty, $update:ident) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name {
            pub offsets: Vec<i32>,
            pub values: Vec<$ty>,
            pub validity: Option<ValidityBitmap>,
        }

        impl $name {
            pub fn required(capacity: usize) -> Self {
                let mut offsets = Vec::with_capacity(capacity.saturating_add(1));
                offsets.push(0);
                Self {
                    offsets,
                    values: Vec::new(),
                    validity: None,
                }
            }

            pub fn optional(capacity: usize) -> Self {
                let mut offsets = Vec::with_capacity(capacity.saturating_add(1));
                offsets.push(0);
                Self {
                    offsets,
                    values: Vec::new(),
                    validity: Some(ValidityBitmap::new(capacity)),
                }
            }

            pub fn push_required<I>(&mut self, values: I) -> Result<()>
            where
                I: IntoIterator<Item = $ty>,
            {
                self.push_values(values)
            }

            pub fn push_optional<I>(&mut self, present: bool, values: I) -> Result<()>
            where
                I: IntoIterator<Item = $ty>,
            {
                let idx = self.offsets.len().saturating_sub(1);
                if present {
                    if let Some(validity) = &mut self.validity {
                        validity.set_present(idx)?;
                    }
                    self.push_values(values)
                } else {
                    self.push_values(core::iter::empty())
                }
            }

            pub fn byte_len(&self) -> usize {
                let data_len = (self.offsets.len() * size_of::<i32>())
                    + (self.values.len() * size_of::<$ty>());
                match &self.validity {
                    Some(validity) => data_len + validity.byte_len(),
                    None => data_len,
                }
            }

            pub fn checksum_with_name(&self, mut hash: u64, name: &str) -> u64 {
                hash = update_str(hash, name);
                hash = update_usize(hash, self.offsets.len());
                hash = update_usize(hash, self.values.len());
                match &self.validity {
                    Some(validity) => {
                        hash = update_u8(hash, 1);
                        hash = validity.checksum_with_name(hash, "validity");
                    }
                    None => {
                        hash = update_u8(hash, 0);
                    }
                }
                for offset in &self.offsets {
                    hash = update_i32(hash, *offset);
                }
                for value in &self.values {
                    hash = $update(hash, *value);
                }
                hash
            }

            fn push_values<I>(&mut self, values: I) -> Result<()>
            where
                I: IntoIterator<Item = $ty>,
            {
                for value in values {
                    self.values.push(value);
                }
                let next_offset = i32::try_from(self.values.len()).map_err(|_| {
                    TransportError::ResponseTooLarge {
                        observed: self.values.len(),
                        cap: i32::MAX as usize,
                    }
                })?;
                self.offsets.push(next_offset);
                Ok(())
            }
        }
    };
}

numeric_list_column!(I64ListColumn, i64, update_i64);
numeric_list_column!(I32ListColumn, i32, update_i32);
numeric_list_column!(U32ListColumn, u32, update_u32);
numeric_list_column!(F64ListColumn, f64, update_f64);
numeric_list_column!(F32ListColumn, f32, update_f32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoolColumn {
    // Bool columns account one byte per row before adapter-specific bit packing.
    pub values: Vec<bool>,
    pub validity: Option<ValidityBitmap>,
}

impl BoolColumn {
    pub fn required(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
            validity: None,
        }
    }

    pub fn optional(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
            validity: Some(ValidityBitmap::new(capacity)),
        }
    }

    pub fn push_required(&mut self, value: bool) {
        self.values.push(value);
    }

    pub fn push_optional(&mut self, present: bool, value: bool) -> Result<()> {
        let idx = self.values.len();
        if present && let Some(validity) = &mut self.validity {
            validity.set_present(idx)?;
        }
        self.values.push(value);
        Ok(())
    }

    pub fn byte_len(&self) -> usize {
        let data_len = self.values.len();
        match &self.validity {
            Some(validity) => data_len + validity.byte_len(),
            None => data_len,
        }
    }

    pub fn checksum_with_name(&self, mut hash: u64, name: &str) -> u64 {
        hash = update_str(hash, name);
        hash = update_usize(hash, self.values.len());
        match &self.validity {
            Some(validity) => {
                hash = update_u8(hash, 1);
                hash = validity.checksum_with_name(hash, "validity");
            }
            None => {
                hash = update_u8(hash, 0);
            }
        }
        for value in &self.values {
            hash = update_bool(hash, *value);
        }
        hash
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Utf8Column {
    // UTF-8 columns use Arrow-style offsets plus concatenated bytes.
    pub offsets: Vec<i32>,
    pub bytes: Vec<u8>,
    pub validity: Option<ValidityBitmap>,
}

impl Utf8Column {
    pub fn required(capacity: usize) -> Self {
        let mut offsets = Vec::with_capacity(capacity.saturating_add(1));
        offsets.push(0);
        Self {
            offsets,
            bytes: Vec::new(),
            validity: None,
        }
    }

    pub fn optional(capacity: usize) -> Self {
        let mut offsets = Vec::with_capacity(capacity.saturating_add(1));
        offsets.push(0);
        Self {
            offsets,
            bytes: Vec::new(),
            validity: Some(ValidityBitmap::new(capacity)),
        }
    }

    pub fn push_required(&mut self, value: &str) -> Result<()> {
        self.push_value(value)
    }

    pub fn push_optional(&mut self, present: bool, value: &str) -> Result<()> {
        let idx = self.offsets.len().saturating_sub(1);
        if present {
            if let Some(validity) = &mut self.validity {
                validity.set_present(idx)?;
            }
            self.push_value(value)
        } else {
            self.push_value("")
        }
    }

    pub fn byte_len(&self) -> usize {
        let data_len = self.offsets.len() * size_of::<u32>() + self.bytes.len();
        match &self.validity {
            Some(validity) => data_len + validity.byte_len(),
            None => data_len,
        }
    }

    pub fn checksum_with_name(&self, mut hash: u64, name: &str) -> u64 {
        hash = update_str(hash, name);
        hash = update_usize(hash, self.offsets.len());
        hash = update_usize(hash, self.bytes.len());
        match &self.validity {
            Some(validity) => {
                hash = update_u8(hash, 1);
                hash = validity.checksum_with_name(hash, "validity");
            }
            None => {
                hash = update_u8(hash, 0);
            }
        }
        for offset in &self.offsets {
            hash = update_i32(hash, *offset);
        }
        update_bytes(hash, &self.bytes)
    }

    fn push_value(&mut self, value: &str) -> Result<()> {
        let next_len =
            self.bytes
                .len()
                .checked_add(value.len())
                .ok_or(TransportError::ResponseTooLarge {
                    observed: usize::MAX,
                    cap: usize::MAX,
                })?;
        let next_offset =
            i32::try_from(next_len).map_err(|_| TransportError::ResponseTooLarge {
                observed: next_len,
                cap: i32::MAX as usize,
            })?;
        self.bytes.extend_from_slice(value.as_bytes());
        self.offsets.push(next_offset);
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryColumn {
    // Binary columns mirror UTF-8 layout without string validation.
    pub offsets: Vec<i32>,
    pub bytes: Vec<u8>,
    pub validity: Option<ValidityBitmap>,
}

impl BinaryColumn {
    pub fn required(capacity: usize) -> Self {
        let mut offsets = Vec::with_capacity(capacity.saturating_add(1));
        offsets.push(0);
        Self {
            offsets,
            bytes: Vec::new(),
            validity: None,
        }
    }

    pub fn optional(capacity: usize) -> Self {
        let mut offsets = Vec::with_capacity(capacity.saturating_add(1));
        offsets.push(0);
        Self {
            offsets,
            bytes: Vec::new(),
            validity: Some(ValidityBitmap::new(capacity)),
        }
    }

    pub fn push_required(&mut self, value: &[u8]) -> Result<()> {
        self.push_value(value)
    }

    pub fn push_optional(&mut self, present: bool, value: &[u8]) -> Result<()> {
        let idx = self.offsets.len().saturating_sub(1);
        if present {
            if let Some(validity) = &mut self.validity {
                validity.set_present(idx)?;
            }
            self.push_value(value)
        } else {
            self.push_value(&[])
        }
    }

    pub fn byte_len(&self) -> usize {
        let data_len = self.offsets.len() * size_of::<u32>() + self.bytes.len();
        match &self.validity {
            Some(validity) => data_len + validity.byte_len(),
            None => data_len,
        }
    }

    pub fn checksum_with_name(&self, mut hash: u64, name: &str) -> u64 {
        hash = update_str(hash, name);
        hash = update_usize(hash, self.offsets.len());
        hash = update_usize(hash, self.bytes.len());
        match &self.validity {
            Some(validity) => {
                hash = update_u8(hash, 1);
                hash = validity.checksum_with_name(hash, "validity");
            }
            None => {
                hash = update_u8(hash, 0);
            }
        }
        for offset in &self.offsets {
            hash = update_i32(hash, *offset);
        }
        update_bytes(hash, &self.bytes)
    }

    fn push_value(&mut self, value: &[u8]) -> Result<()> {
        let next_len =
            self.bytes
                .len()
                .checked_add(value.len())
                .ok_or(TransportError::ResponseTooLarge {
                    observed: usize::MAX,
                    cap: usize::MAX,
                })?;
        let next_offset =
            i32::try_from(next_len).map_err(|_| TransportError::ResponseTooLarge {
                observed: next_len,
                cap: i32::MAX as usize,
            })?;
        self.bytes.extend_from_slice(value);
        self.offsets.push(next_offset);
        Ok(())
    }
}

pub fn ensure_columnar_size(observed: usize, cap: usize) -> Result<()> {
    // Columnar batches are capped before any boundary format writer runs.
    if observed > cap {
        return Err(TransportError::ResponseTooLarge { observed, cap });
    }
    Ok(())
}

pub fn checksum_seed(name: &str) -> u64 {
    // Columnar evidence checksums compose stable field names and typed values.
    update_str(FNV_OFFSET, name)
}

pub fn update_u8(hash: u64, value: u8) -> u64 {
    update_bytes(hash, &[value])
}

pub fn update_u16(hash: u64, value: u16) -> u64 {
    update_bytes(hash, &value.to_le_bytes())
}

pub fn update_u32(hash: u64, value: u32) -> u64 {
    update_bytes(hash, &value.to_le_bytes())
}

pub fn update_i32(hash: u64, value: i32) -> u64 {
    update_bytes(hash, &value.to_le_bytes())
}

pub fn update_u64(hash: u64, value: u64) -> u64 {
    update_bytes(hash, &value.to_le_bytes())
}

pub fn update_i64(hash: u64, value: i64) -> u64 {
    update_bytes(hash, &value.to_le_bytes())
}

pub fn update_f32(hash: u64, value: f32) -> u64 {
    update_bytes(hash, &value.to_le_bytes())
}

pub fn update_f64(hash: u64, value: f64) -> u64 {
    update_bytes(hash, &value.to_le_bytes())
}

pub fn update_bool(hash: u64, value: bool) -> u64 {
    update_u8(hash, u8::from(value))
}

pub fn update_usize(hash: u64, value: usize) -> u64 {
    update_bytes(hash, &(value as u64).to_le_bytes())
}

pub fn update_str(hash: u64, value: &str) -> u64 {
    let hash = update_usize(hash, value.len());
    update_bytes(hash, value.as_bytes())
}

pub fn update_bytes(mut hash: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}
