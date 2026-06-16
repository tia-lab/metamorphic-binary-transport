use crate::envelope::fnv1a64;

// Deterministic evidence checksum for reports and tests; not a cryptographic hash.
pub fn response_checksum(bytes: &[u8]) -> u64 {
    fnv1a64(bytes)
}
