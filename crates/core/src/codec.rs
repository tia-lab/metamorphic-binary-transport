use crate::envelope::fnv1a64;

pub fn response_checksum(bytes: &[u8]) -> u64 {
    fnv1a64(bytes)
}
