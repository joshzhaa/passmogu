use aws_lc_rs::pbkdf2;
use std::num::NonZeroU32;
use zeroize::Zeroizing;

pub fn vault_key(password: &[u8], salt: &[u8]) -> Zeroizing<Box<[u8]>> {
    let mut result = Zeroizing::new(vec![0_u8; 256].into_boxed_slice());
    const PBKDF2_ITERATIONS: NonZeroU32 = NonZeroU32::new(300_000_u32).unwrap();
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        PBKDF2_ITERATIONS,
        salt,
        password,
        &mut result,
    );
    result
}
