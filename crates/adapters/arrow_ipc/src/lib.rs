#![forbid(unsafe_code)]
//! Arrow IPC stream helpers for generated MBT metamorphose adapters.

use std::io::{self, Cursor, Write};

use arrow_array::RecordBatch;
use arrow_ipc::reader::StreamReader;
use arrow_ipc::writer::StreamWriter;
use arrow_schema::ArrowError;
use metamorphic_binary_transport_core::codec::response_checksum;
use metamorphic_binary_transport_core::error::{Result, TransportError};

mod arrow_bridge;

const IPC_STREAM_OVERHEAD_BYTES: usize = 4096;

pub use arrow_array::RecordBatch as ArrowRecordBatch;
pub use arrow_bridge::*;

pub fn write_ipc_stream(batch: &RecordBatch, max_response_bytes: usize) -> Result<Vec<u8>> {
    // Arrow IPC is a boundary encoding; the sink enforces the response cap.
    let mut sink = CheckedArrowIpcWriter::new(
        max_response_bytes,
        initial_capacity(batch, max_response_bytes),
    );
    let mut writer = match StreamWriter::try_new(&mut sink, batch.schema().as_ref()) {
        Ok(writer) => writer,
        Err(err) => return Err(arrow_ipc_or_overflow_error(&sink, err)),
    };
    if let Err(err) = writer.write(batch) {
        drop(writer);
        return Err(arrow_ipc_or_overflow_error(&sink, err));
    }
    if let Err(err) = writer.finish() {
        return Err(arrow_ipc_or_overflow_error(&sink, err));
    }
    sink.finish()
}

pub fn record_batch_from_ipc_stream(bytes: &[u8]) -> Result<RecordBatch> {
    // Decode helper is used by tests and validation, not by the hot writer path.
    let mut reader = StreamReader::try_new(Cursor::new(bytes), None).map_err(arrow_ipc_error)?;
    let first = match reader.next() {
        Some(result) => result.map_err(arrow_ipc_error)?,
        None => {
            return Err(TransportError::MalformedArchive(
                "Arrow IPC readback produced no record batches".to_string(),
            ));
        }
    };
    if reader.next().is_some() {
        return Err(TransportError::MalformedArchive(
            "Arrow IPC readback produced multiple record batches".to_string(),
        ));
    }
    Ok(first)
}

pub fn arrow_ipc_bytes_checksum(bytes: &[u8]) -> u64 {
    response_checksum(bytes)
}

// The IPC writer is checked at the sink boundary so Arrow cannot overshoot the response cap.
struct CheckedArrowIpcWriter {
    bytes: Vec<u8>,
    cap: usize,
    overflow: Option<usize>,
}

impl CheckedArrowIpcWriter {
    fn new(cap: usize, initial_capacity: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(initial_capacity),
            cap,
            overflow: None,
        }
    }

    fn finish(self) -> Result<Vec<u8>> {
        if let Some(observed) = self.overflow {
            return Err(TransportError::ResponseTooLarge {
                observed,
                cap: self.cap,
            });
        }
        Ok(self.bytes)
    }
}

impl Write for CheckedArrowIpcWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let observed = match self.bytes.len().checked_add(buf.len()) {
            Some(observed) => observed,
            None => {
                self.overflow = Some(usize::MAX);
                return Err(io::Error::new(
                    io::ErrorKind::OutOfMemory,
                    "Arrow IPC response size overflow",
                ));
            }
        };
        if observed > self.cap {
            self.overflow = Some(observed);
            return Err(io::Error::new(
                io::ErrorKind::OutOfMemory,
                "Arrow IPC response exceeds cap",
            ));
        }
        self.bytes.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn arrow_ipc_error(err: ArrowError) -> TransportError {
    TransportError::MalformedArchive(err.to_string())
}

fn initial_capacity(batch: &RecordBatch, max_response_bytes: usize) -> usize {
    // This is only a sizing hint; CheckedArrowIpcWriter remains authoritative.
    record_batch_byte_len(batch)
        .saturating_add(IPC_STREAM_OVERHEAD_BYTES)
        .min(max_response_bytes)
}

fn record_batch_byte_len(batch: &RecordBatch) -> usize {
    batch.columns().iter().fold(0_usize, |len, column| {
        len.saturating_add(column.get_buffer_memory_size())
    })
}

fn arrow_ipc_or_overflow_error(sink: &CheckedArrowIpcWriter, err: ArrowError) -> TransportError {
    // Preserve cap failures when Arrow reports the sink write as an Arrow error.
    match sink.overflow {
        Some(observed) => TransportError::ResponseTooLarge {
            observed,
            cap: sink.cap,
        },
        None => arrow_ipc_error(err),
    }
}

#[cfg(test)]
mod tests;
