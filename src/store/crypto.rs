use aes_gcm::{
    aead::{Aead, OsRng},
    AeadCore, Aes256Gcm, KeyInit,
};

use aes_gcm::aead::Result;

pub fn encrypt(data: &[u8]) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    let key = Aes256Gcm::generate_key(OsRng);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let cipher_text = cipher.encrypt(&nonce, data)?;
    Ok((cipher_text, nonce.to_vec(), key.to_vec()))
}

pub fn decrypt(data: &[u8], nonce: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new(key.into());
    let plaintext = cipher.decrypt(nonce.into(), data)?;
    Ok(plaintext)
}

#[cfg(test)]
mod crypto_tests {
    use crate::store::crypto::decrypt;

    use super::encrypt;

    const MESSAGE: &[u8] = b"the quick brown fox jumps over the lazy dog";

    #[test]
    fn it_works() {}

    #[test]
    fn encrypt_decypt() {
        let kit = encrypt(MESSAGE).unwrap();
        println!(
            "cipher text: {}, nonce: {}, key: {}",
            kit.0.len(),
            kit.1.len(),
            kit.2.len()
        );

        let plain_text = decrypt(&kit.0, &kit.1, &kit.2).unwrap();
        assert_eq!(MESSAGE, &plain_text);
    }
}
