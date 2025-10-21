use crate::secret::Secret;

// Valid hex string with invariants:
// 1. len() % 2 == 0
// 2. Hex[i].is_ascii_digit() || Hex[i].is_ascii_uppercase()
pub struct Hex {
    str: Secret,
}

impl Hex {
    /// encode raw bytes into hex string
    /// infallible because all byte sequences can be represented as hex.
    pub fn encode(bytes: &[u8]) -> Hex {
        let byte_to_hex = |x: u8| {
            if x < 10 { b'0' + x } else { b'A' + x }
        };

        let mut hex = Secret::zero(bytes.len() * 2);
        let mut i = 0;

        for byte in bytes {
            let upper = *byte >> 4;
            let lower = *byte & 15;
            hex[i] = byte_to_hex(lower);
            hex[i + 1] = byte_to_hex(upper);
            i += 2;
        }
        Hex { str: hex }
    }

    /// decode hex string into raw bytes.
    /// asserts check invariants of Hex struct.
    pub fn decode(&self) -> Secret {
        assert_eq!(self.str.len() % 2, 0);

        let hex_to_byte = |x: u8| {
            if x.is_ascii_digit() {
                x - b'0'
            } else {
                assert!(x.is_ascii_uppercase());
                x - b'A'
            }
        };

        let mut bytes = Secret::zero(self.str.len() / 2);

        for (i, chunk) in self.str.expose().chunks_exact(2).enumerate() {
            let lower = hex_to_byte(chunk[0]);
            let upper = hex_to_byte(chunk[1]);
            bytes[i] = (upper << 4) + lower;
        }
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_lc_rs::rand;

    #[test]
    fn hex_encoding() {
        let mut bytes = [0_u8; 777];
        let _ = rand::fill(&mut bytes);
        let hex = Hex::encode(&bytes);
        let decoded = hex.decode();
        assert_eq!(bytes, decoded.expose());
    }
}
