use argon2::Argon2;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key, Nonce,
};
use rand::rngs::OsRng;
use rand::RngCore;
use zeroize::Zeroize;

use crate::error::DevVaultError;

const SALT_LENGTH: usize = 16;
const NONCE_LENGTH: usize = 12;
const KEY_LENGTH: usize = 32;

pub fn derive_key(password: &[u8], salt: &[u8]) -> Result<[u8; KEY_LENGTH], DevVaultError> {
    let mut key = [0u8; KEY_LENGTH];
    let argon2 = Argon2::default();
    argon2
        .hash_password_into(password, salt, &mut key)
        .map_err(|e| DevVaultError::PasswordError(e.to_string()))?;
    Ok(key)
}

pub fn encrypt(plaintext: &[u8], password: &[u8]) -> Result<Vec<u8>, DevVaultError> {
    let mut salt = [0u8; SALT_LENGTH];
    OsRng.fill_bytes(&mut salt);
    let key = derive_key(password, &salt)?;
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
    let mut nonce_bytes = [0u8; NONCE_LENGTH];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| DevVaultError::EncryptionError(e.to_string()))?;
    let mut result = Vec::with_capacity(SALT_LENGTH + NONCE_LENGTH + ciphertext.len());
    result.extend_from_slice(&salt);
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);
    // Zeroize sensitive data
    let mut key = key;
    key.zeroize();
    Ok(result)
}

pub fn decrypt(ciphertext: &[u8], password: &[u8]) -> Result<Vec<u8>, DevVaultError> {
    if ciphertext.len() < SALT_LENGTH + NONCE_LENGTH {
        return Err(DevVaultError::DecryptionError("Ciphertext too short".to_string()));
    }
    let salt = &ciphertext[..SALT_LENGTH];
    let nonce_bytes = &ciphertext[SALT_LENGTH..SALT_LENGTH + NONCE_LENGTH];
    let encrypted_data = &ciphertext[SALT_LENGTH + NONCE_LENGTH..];
    let key = derive_key(password, salt)?;
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
    let nonce = Nonce::from_slice(nonce_bytes);
    let plaintext = cipher
        .decrypt(nonce, encrypted_data)
        .map_err(|e| DevVaultError::DecryptionError(e.to_string()))?;
    // Zeroize sensitive data
    let mut key = key;
    key.zeroize();
    Ok(plaintext)
}