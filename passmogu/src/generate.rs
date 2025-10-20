use crate::safe_string::SafeString;
use aws_lc_rs::rand;
use zeroize::Zeroizing;

pub fn rand_xkcd(_len: usize, _dictionary: &[&str]) -> Option<SafeString> {
    todo!()
}

/// Generate a random base62 String (A-Z, a-z, 0-9)
/// Resulting chars in String are uniformly distributed in the base62 alphabet
pub fn rand_base62(len: usize) -> Option<SafeString> {
    const ALPHABET: [u8; 62] = [
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E',
        b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T',
        b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i',
        b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x',
        b'y', b'z',
    ];

    let mut result = Zeroizing::new(vec![0_u8; len].into_boxed_slice());
    let mut write_head = 0;

    const CHUNK_SIZE: usize = 128; // we'll populate result CHUNK_SIZE bytes at a time
    let mut random_bytes = Zeroizing::new([0_u8; CHUNK_SIZE]);

    loop {
        // refresh with another chunk of random bytes
        rand::fill(&mut *random_bytes).ok()?;
        // encode into base62 by indexing into ALPHABET
        for byte in *random_bytes {
            let index = usize::from(byte);
            // filter so that first elements of ALPHABET aren't statistically more likely in result
            if index >= greatest_multiple(ALPHABET.len(), u8::MAX as usize) {
                continue;
            }
            result[write_head] = ALPHABET[index % 62];
            write_head += 1;
            if write_head == len {
                return Some(result);
            }
        }
    }
}

/// greatest multiple of 'number' strictly less then 'upper_limit'
const fn greatest_multiple(number: usize, upper_limit: usize) -> usize {
    ((upper_limit - 1) / number) * number
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn basic_base62() {
        let small = rand_base62(1).unwrap();
        assert_eq!(small.len(), 1);
        println!("small: {}", str::from_utf8(&small).unwrap());

        let medium = rand_base62(64).unwrap();
        assert_eq!(medium.len(), 64);
        println!("medium: {}", str::from_utf8(&medium).unwrap());

        let large = rand_base62(1024).unwrap();
        assert_eq!(large.len(), 1024);
        println!("large: {}", str::from_utf8(&large).unwrap());
    }

    #[test]
    /// detects statistical bias in string gen
    fn base62_char_distribution() {
        let mut counts: HashMap<u8, usize> = HashMap::new();
        let mut total = 0;
        for _ in 0..50 {
            let string = rand_base62(10000).unwrap();
            for c in string.iter() {
                *counts.entry(*c).or_default() += 1;
                total += 1;
            }
        }
        for v in counts.values() {
            let frequency = *v as f64 / total as f64;
            const EXPECT: f64 = 1_f64 / 62_f64;
            const TOLERANCE: f64 = 0.001;
            assert!((frequency - EXPECT).abs() < TOLERANCE);
        }
    }
}
