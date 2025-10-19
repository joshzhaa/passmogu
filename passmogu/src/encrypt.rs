use aws_lc_rs::{aead, pbkdf2};
use std::num::NonZeroU32;
use zeroize::Zeroizing;
use crate::safe_string::SafeString;

/// We can afford the performance penalty of SIV, still don't reuse nonces.
const ALGORITHM: &aead::Algorithm = &aead::AES_256_GCM_SIV;

/// Returns the secret symmetric encryption key derived from password.
/// The key will be a u8 slice length ALGORITHM.key_len()
pub fn vault_key(password: &[u8], salt: &[u8]) -> SafeString {
    const PBKDF2_ITERATIONS: NonZeroU32 = NonZeroU32::new(300_000_u32).unwrap();

    let mut result = SafeString::new(
        vec![0_u8; ALGORITHM.key_len()].into_boxed_slice()
    );
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        PBKDF2_ITERATIONS,
        salt,
        password,
        &mut result,
    );
    result
}

/// Returns encrypted ciphertext and the nonce necessary to decrypt it.
/// The ciphertext will be the same length as the plaintext
/// Even though what's returned is ciphertext, it doesn't cost us much to zero it out anyway.
pub fn encrypt(mut plaintext: SafeString, key: &[u8]) -> Option<(SafeString, aead::Nonce)> {
    let aead_key = aead::RandomizedNonceKey::new(
        ALGORITHM,
        key,
    ).ok()?;

    let (nonce, tag) = aead_key.seal_in_place_separate_tag(
        aead::Aad::empty(),
        &mut plaintext
    ).ok()?;
    // at this point "plaintext" contains the ciphertext (eww aws_lc_rs uses out parameters)
    let ciphertext = &plaintext;

    let result_len = ciphertext.len() + ALGORITHM.tag_len();
    let mut result = SafeString::new(
        vec![0_u8; result_len].into_boxed_slice()
    );
    // there might be a more idiomatic way to concat arrays in this language w/o unhygienically
    // reallocating everything and leaving copies everywhere on the heap.
    for i in 0..result_len {
        if i < ciphertext.len() {
            result[i] = ciphertext[i];
        } else {
            result[i] = tag.as_ref()[i - ciphertext.len()];
        }
    }
    Some((result, nonce))
}

/// Decrypts ciphertext into plaintext.
pub fn decrypt(mut ciphertext: SafeString, key: &[u8], nonce: aead::Nonce) -> Option<SafeString> {
    let aead_key = aead::RandomizedNonceKey::new(
        &aead::AES_256_GCM_SIV,
        key,
    ).ok()?;

    let plaintext = aead_key.open_in_place(
        nonce,
        aead::Aad::empty(),
        &mut ciphertext,
    ).ok()?;

    Some(Zeroizing::new(Box::from(plaintext)))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_and_decrypt()  {
        // derive key from password
        let password = SafeString::new((*b"Phoenix").into());
        println!("password = {password:?}");
        let key = vault_key(&password, b"salt");
        println!("key = {key:?}");
        assert_ne!(password, key);

        // encrypt and decrypt with key
        let message = SafeString::new((*b"I set my ATM card's number to '0001' because I'm number one!").into());
        println!("message = {}", str::from_utf8(&message).unwrap());
        let (ciphertext, nonce) = encrypt(message.clone(), &key).unwrap();
        println!("ciphertext = {ciphertext:?}");
        let decoded = decrypt(ciphertext, &key, nonce).unwrap();
        println!("plaintext = {}", str::from_utf8(&decoded).unwrap());
        assert_eq!(message, decoded);
    }
}
