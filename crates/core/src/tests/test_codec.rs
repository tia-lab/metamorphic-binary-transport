use crate::codec::response_checksum;
use crate::envelope::fnv1a64;

#[test]
fn response_checksum_delegates_to_fnv() {
    let bytes = b"mbt-response";
    assert_eq!(response_checksum(bytes), fnv1a64(bytes));
}
