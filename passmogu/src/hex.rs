/// encode bytes into hex chars.
/// infallible because all byte sequences can be represented as hex.
pub fn encode(bytes: &[u8]) -> Box<[u8]> {
    let byte_to_hex = |x: u8| {
        if x < 10 { b'0' + x } else { b'A' + x }
    };

    let mut hex = vec![0_u8; bytes.len() * 2].into_boxed_slice();
    let mut i = 0;

    for byte in bytes {
        let upper = *byte >> 4;
        let lower = *byte & 15;
        hex[i] = byte_to_hex(lower);
        hex[i + 1] = byte_to_hex(upper);
        i += 2;
    }
    hex
}

/// decode hex into raw bytes.
/// pretends to be infallible but actually panics your code, which is probably fine because
/// you should only really get a well-formed hex string from encode
pub fn decode(hex: &[u8]) -> Box<[u8]> {
    assert!(hex.len() % 2 == 0);

    let hex_to_byte = |x: u8| {
        if x.is_ascii_digit() {
            x - b'0'
        } else {
            assert!(x.is_ascii_uppercase());
            x - b'A'
        }
    };

    let mut bytes = vec![0_u8; hex.len() / 2].into_boxed_slice();

    for (i, chunk) in hex.chunks_exact(2).enumerate() {
        let lower = hex_to_byte(chunk[0]);
        let upper = hex_to_byte(chunk[1]);
        bytes[i] = (upper << 4) + lower;
    }
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_lc_rs::rand;

    #[test]
    fn hex_encoding() {
        let mut bytes = [0_u8; 777];
        let _ = rand::fill(&mut bytes);
        let hex = encode(&bytes);
        let decoded = decode(&hex);
        assert_eq!(bytes, *decoded);
    }
}
