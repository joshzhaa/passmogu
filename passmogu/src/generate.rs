use aws_lc_rs::rand::{SystemRandom, generate};

// an error that bubbles up from cryptography library
#[derive(Debug)]
pub struct CryptoError {}

pub fn random_bytes<const SIZE: usize>() -> Result<[u8; SIZE], CryptoError> {
    let rng = SystemRandom::new(); // program should only use rng here
    match generate(&rng) {
        Ok(val) => Ok(val.expose()),
        Err(_) => Err(CryptoError {}),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_random_bytes() {
        let small: [u8; 1] = random_bytes().unwrap();
        assert_eq!(small.len(), 1);
        println!("{small:?}");
        let medium: [u8; 64] = random_bytes().unwrap();
        assert_eq!(medium.len(), 64);
        println!("{medium:?}");
        let large: [u8; 1024] = random_bytes().unwrap();
        assert_eq!(large.len(), 1024);
        println!("{large:?}");
    }
}
