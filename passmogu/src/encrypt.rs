use crate::secret::Secret;
use aws_lc_rs::{aead, pbkdf2};
use std::num::NonZeroU32;
use zeroize::Zeroizing;

/// We can afford the performance penalty of SIV, still don't reuse nonces.
const ALGORITHM: &aead::Algorithm = &aead::AES_256_GCM_SIV;

/// Returns the secret symmetric encryption key derived from password.
/// The key will be a u8 slice length ALGORITHM.key_len().
pub fn derive_key(password: &[u8], salt: &[u8]) -> Secret {
    const PBKDF2_ITERATIONS: NonZeroU32 = NonZeroU32::new(300_000_u32).unwrap();

    let mut result = Secret::zero(ALGORITHM.key_len());
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        PBKDF2_ITERATIONS,
        salt,
        password,
        result.expose_mut(),
    );
    result
}

/// Returns encrypted concatenated(nonce, ciphertext, tag)
/// Even though what's returned is ciphertext, it doesn't cost us much to zero it out anyway.
pub fn encrypt(mut plaintext: Secret, key: &[u8]) -> Option<Secret> {
    let aead_key = aead::RandomizedNonceKey::new(ALGORITHM, key).ok()?;

    let (nonce, tag) = aead_key
        .seal_in_place_separate_tag(aead::Aad::empty(), plaintext.expose_mut())
        .ok()?;
    // at this point "plaintext" contains the ciphertext (eww aws_lc_rs uses out parameters)
    let ciphertext = &plaintext;

    let nonce = nonce.as_ref();
    let result_len = ciphertext.len() + ALGORITHM.tag_len() + aead::NONCE_LEN;
    let mut result = Secret::zero(result_len);
    // there might be a more idiomatic way to concat arrays in this language w/o unhygienically
    // reallocating everything and leaving copies everywhere on the heap.
    for i in 0..aead::NONCE_LEN {
        result[i] = nonce[i];
    }
    for i in 0..ciphertext.len() {
        result[i + aead::NONCE_LEN] = ciphertext[i];
    }
    for i in 0..ALGORITHM.tag_len() {
        result[i + aead::NONCE_LEN + ciphertext.len()] = tag.as_ref()[i];
    }
    Some(result)
}

/// Decrypts ciphertext into plaintext.
pub fn decrypt(mut ciphertext: Secret, key: &[u8]) -> Option<Secret> {
    let aead_key = aead::RandomizedNonceKey::new(&aead::AES_256_GCM_SIV, key).ok()?;

    let nonce = slice_to_nonce(&ciphertext[0..aead::NONCE_LEN]);
    let len = ciphertext.len();
    let ciphertext = &mut ciphertext.expose_mut()[aead::NONCE_LEN..len];

    let plaintext = aead_key
        .open_in_place(nonce, aead::Aad::empty(), ciphertext)
        .ok()?;

    Some(Secret::new(Box::from(plaintext)))
}

fn slice_to_nonce(slice: &[u8]) -> aead::Nonce {
    let mut buffer = Zeroizing::new([0_u8; aead::NONCE_LEN]);
    buffer.copy_from_slice(slice);
    (&*buffer).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_and_decrypt() {
        // derive key from password
        let password = Secret::new((*b"Phoenix").into());
        println!("password = {password:?}");
        let key = derive_key(password.expose(), b"salt");
        println!("key = {key:?}");
        assert_ne!(password, key);

        // encrypt and decrypt with key
        let message =
            Secret::new((*b"I set my ATM card's number to '0001' because I'm number one!").into());
        println!("message = {}", str::from_utf8(message.expose()).unwrap());
        let ciphertext = encrypt(message.clone(), key.expose()).unwrap();
        println!("ciphertext = {ciphertext:?}");
        let decoded = decrypt(ciphertext, key.expose()).unwrap();
        println!("plaintext = {}", str::from_utf8(decoded.expose()).unwrap());
        assert_eq!(message, decoded);
    }
}
