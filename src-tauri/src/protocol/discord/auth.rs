//! Discord authentication and token encryption
//!
//! Handles bot token encryption/decryption for secure storage.

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::Rng;

/// Encrypt a Discord bot token using AES-256-GCM
///
/// # Arguments
/// * `token` - The plaintext bot token
/// * `key` - 32-byte encryption key
///
/// # Returns
/// Encrypted token as base64-encoded string (nonce + ciphertext)
pub fn encrypt_token(token: &str, key: &[u8]) -> Result<String, String> {
    if key.len() != 32 {
        return Err("Key must be 32 bytes".to_string());
    }

    let cipher =
        Aes256Gcm::new_from_slice(key).map_err(|e| format!("Failed to create cipher: {}", e))?;

    // Generate random nonce
    let nonce_bytes = rand::thread_rng().gen::<[u8; 12]>();
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt token
    let ciphertext = cipher
        .encrypt(nonce, token.as_bytes())
        .map_err(|e| format!("Encryption failed: {}", e))?;

    // Combine nonce + ciphertext and encode
    let mut combined = nonce_bytes.to_vec();
    combined.extend_from_slice(&ciphertext);

    Ok(base64::encode(&combined))
}

/// Decrypt a Discord bot token
///
/// # Arguments
/// * `encrypted` - Base64-encoded encrypted token (nonce + ciphertext)
/// * `key` - 32-byte encryption key
///
/// # Returns
/// Decrypted plaintext token
pub fn decrypt_token(encrypted: &str, key: &[u8]) -> Result<String, String> {
    if key.len() != 32 {
        return Err("Key must be 32 bytes".to_string());
    }

    let cipher =
        Aes256Gcm::new_from_slice(key).map_err(|e| format!("Failed to create cipher: {}", e))?;

    // Decode base64
    let combined = base64::decode(encrypted).map_err(|e| format!("Base64 decode failed: {}", e))?;

    if combined.len() < 13 {
        return Err("Invalid encrypted data".to_string());
    }

    // Split nonce and ciphertext
    let (nonce_bytes, ciphertext) = combined.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    // Decrypt
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| format!("Decryption failed: {}", e))?;

    String::from_utf8(plaintext).map_err(|e| format!("Invalid UTF-8: {}", e))
}

/// Generate an encryption key from machine-specific data
///
/// # Returns
/// 32-byte encryption key
///
/// # Note
/// This is a simple implementation for v1. In production, use OS keychain
/// or secure enclave for key storage.
pub fn generate_key() -> Vec<u8> {
    // For v1, derive key from machine ID or use a fixed key
    // This should be replaced with proper OS keychain integration in v2

    // Try to get machine-specific data
    let machine_id = get_machine_id();

    // Derive 32-byte key using simple hashing
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    machine_id.hash(&mut hasher);
    let hash1 = hasher.finish();

    let mut hasher = DefaultHasher::new();
    hash1.hash(&mut hasher);
    let hash2 = hasher.finish();

    // Combine two 64-bit hashes into 32 bytes
    let mut key = Vec::with_capacity(32);
    key.extend_from_slice(&hash1.to_le_bytes());
    key.extend_from_slice(&hash2.to_le_bytes());
    key.extend_from_slice(&hash1.to_le_bytes());
    key.extend_from_slice(&hash2.to_le_bytes());

    key
}

/// Get machine-specific identifier
///
/// Returns a string that should be unique to this machine
fn get_machine_id() -> String {
    // Try to get hostname
    if let Ok(hostname) = std::env::var("HOSTNAME") {
        return hostname;
    }

    if let Ok(computername) = std::env::var("COMPUTERNAME") {
        return computername;
    }

    // Fallback to a combination of process info
    format!("spoky-{}", std::process::id())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = vec![0u8; 32];
        let token = "MTEyMjMzNDQ1NTY2Nzc4OA.abc123.xyz789";

        let encrypted = encrypt_token(token, &key).unwrap();
        let decrypted = decrypt_token(&encrypted, &key).unwrap();

        assert_eq!(decrypted, token);
    }

    #[test]
    fn test_different_keys() {
        let key1 = vec![0u8; 32];
        let key2 = vec![1u8; 32];
        let token = "test-token";

        let encrypted = encrypt_token(token, &key1).unwrap();
        let result = decrypt_token(&encrypted, &key2);

        assert!(result.is_err());
    }

    #[test]
    fn test_generate_key() {
        let key = generate_key();
        assert_eq!(key.len(), 32);
    }
}
