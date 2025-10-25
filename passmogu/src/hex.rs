use crate::secret::Secret;

// Valid hex string with invariants:
// 1. len() % 2 == 0
// 2. Hex[i].is_ascii_digit() || (b'A'..=b'F').contains(&Hex[i])
pub struct Hex {
    str: Secret,
}

impl Hex {
    pub fn new(bytes: &[u8]) -> Option<Self> {
        for byte in bytes {
            if !byte.is_ascii_digit() && !(b'A'..=b'F').contains(byte) {
                return None;
            }
        }
        Some(Self {
            str: Secret::new(Box::from(bytes)),
        })
    }

    /// encode raw bytes into hex string
    /// infallible because all byte sequences can be represented as hex.
    pub fn encode(bytes: &[u8]) -> Self {
        let byte_to_hex = |x: u8| {
            if x < 10 { b'0' + x } else { b'A' + x - 10 }
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
        Self { str: hex }
    }

    /// decode hex string into raw bytes.
    /// asserts check invariants of Hex struct.
    pub fn decode(&self) -> Secret {
        debug_assert_eq!(self.str.len() % 2, 0);

        let hex_to_byte = |x: u8| {
            if x.is_ascii_digit() {
                x - b'0'
            } else {
                debug_assert!((b'A'..=b'F').contains(&x));
                x - b'A' + 10
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

    pub fn as_slice(&self) -> &[u8] {
        self.str.expose()
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
