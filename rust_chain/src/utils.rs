use std::io::{Error, ErrorKind, Write};

// IntToHex converts an i64 to a Vec<u8>
pub fn int_to_hex(num: i64) -> Vec<u8> {
    num.to_be_bytes().to_vec()
}
