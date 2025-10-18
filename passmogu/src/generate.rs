use aws_lc_rs::{error, rand};

/// greatest multiple of 'number' strictly less then 'upper_limit'
const fn greatest_multiple(number: usize, upper_limit: usize) -> usize {
    ((upper_limit - 1) / number) * number
}

pub fn rand_base62(len: usize) -> Result<String, error::Unspecified> {
    const ALPHABET_LEN: usize = 62;
    static ALPHABET: [char; ALPHABET_LEN] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
        'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let mut result = String::new();
    result.reserve(len);

    const CHUNK_SIZE: usize = 128;
    let mut random_bytes = [0_u8; CHUNK_SIZE];

    loop {
        // refresh with another chunk of random bytes
        rand::fill(&mut random_bytes)?;
        // encode into base62 by indexing into ALPHABET
        for byte in random_bytes {
            // filter so that first elements of ALPHABET aren't statisitcally more likely in result
            let index = usize::from(byte);
            if index < greatest_multiple(ALPHABET_LEN, u8::MAX as usize) {
                result.push(ALPHABET[index % 62]);
                if result.len() == len {
                    return Ok(result);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn basic_base62() {
        let small = rand_base62(1).unwrap();
        assert_eq!(small.len(), 1);
        println!("small: {small}");

        let medium = rand_base62(64).unwrap();
        assert_eq!(medium.len(), 64);
        println!("medium: {medium}");

        let large = rand_base62(1024).unwrap();
        assert_eq!(large.len(), 1024);
        println!("large: {large}");
    }

    #[test]
    /// this test takes a second
    fn base62_char_stats() {
        let mut counts: HashMap<char, usize> = HashMap::new();
        let mut total = 0;
        for _ in 0..50 {
            let string = rand_base62(100000).unwrap();
            for c in string.chars() {
                *counts.entry(c).or_default() += 1;
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
