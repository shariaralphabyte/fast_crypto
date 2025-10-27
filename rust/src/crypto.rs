use crate::error::CryptoError;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use base64::{engine::general_purpose, Engine as _};
use flutter_rust_bridge::frb;
use pbkdf2::pbkdf2_hmac;
use rand::RngCore;
use rsa::{
    pkcs1v15::{Signature, SigningKey, VerifyingKey},
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey},
    signature::{RandomizedSigner, SignatureEncoding, Verifier},
    RsaPrivateKey, RsaPublicKey,
};
use sha2::{Digest, Sha256, Sha512};
use tokio::fs as async_fs;

/// RSA key pair structure
#[frb]
#[derive(Debug, Clone)]
pub struct RsaKeyPair {
    pub private_key_pem: String,
    pub public_key_pem: String,
}

/// Derive a key from password using PBKDF2
fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, 100_000, &mut key);
    key
}

/// Generate a random salt
fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);
    salt
}

/// Encrypt text using AES-256-GCM
pub fn encrypt_text_impl(text: String, password: String) -> Result<String, CryptoError> {
    if text.is_empty() {
        return Err(CryptoError::InvalidInput {
            message: "Text cannot be empty".to_string(),
        });
    }

    if password.is_empty() {
        return Err(CryptoError::InvalidInput {
            message: "Password cannot be empty".to_string(),
        });
    }

    let salt = generate_salt();
    let key_bytes = derive_key(&password, &salt);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, text.as_bytes())
        .map_err(|e| CryptoError::EncryptionFailed {
            message: e.to_string(),
        })?;

    // Combine salt + nonce + ciphertext
    let mut result = Vec::new();
    result.extend_from_slice(&salt);
    result.extend_from_slice(&nonce);
    result.extend_from_slice(&ciphertext);

    Ok(general_purpose::STANDARD.encode(result))
}

/// Decrypt text using AES-256-GCM
pub fn decrypt_text_impl(encrypted_data: String, password: String) -> Result<String, CryptoError> {
    if encrypted_data.is_empty() {
        return Err(CryptoError::InvalidInput {
            message: "Encrypted data cannot be empty".to_string(),
        });
    }

    if password.is_empty() {
        return Err(CryptoError::InvalidInput {
            message: "Password cannot be empty".to_string(),
        });
    }

    let data = general_purpose::STANDARD.decode(encrypted_data)?;

    if data.len() < 16 + 12 {
        return Err(CryptoError::DecryptionFailed {
            message: "Invalid encrypted data format".to_string(),
        });
    }

    let salt = &data[0..16];
    let nonce = &data[16..28];
    let ciphertext = &data[28..];

    let key_bytes = derive_key(&password, salt);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce);

    let plaintext = cipher.decrypt(nonce, ciphertext)?;

    String::from_utf8(plaintext).map_err(|e| CryptoError::DecryptionFailed {
        message: format!("Invalid UTF-8: {}", e),
    })
}

/// Encrypt a file using AES-256-GCM
pub async fn encrypt_file_impl(
    input_path: String,
    output_path: String,
    password: String,
) -> Result<(), CryptoError> {
    if password.is_empty() {
        return Err(CryptoError::InvalidInput {
            message: "Password cannot be empty".to_string(),
        });
    }

    let data = async_fs::read(&input_path).await?;
    let salt = generate_salt();
    let key_bytes = derive_key(&password, &salt);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, data.as_slice())
        .map_err(|e| CryptoError::EncryptionFailed {
            message: e.to_string(),
        })?;

    // Combine salt + nonce + ciphertext
    let mut result = Vec::new();
    result.extend_from_slice(&salt);
    result.extend_from_slice(&nonce);
    result.extend_from_slice(&ciphertext);

    async_fs::write(&output_path, result).await?;
    Ok(())
}

/// Decrypt a file using AES-256-GCM
pub async fn decrypt_file_impl(
    input_path: String,
    output_path: String,
    password: String,
) -> Result<(), CryptoError> {
    if password.is_empty() {
        return Err(CryptoError::InvalidInput {
            message: "Password cannot be empty".to_string(),
        });
    }

    let data = async_fs::read(&input_path).await?;

    if data.len() < 16 + 12 {
        return Err(CryptoError::DecryptionFailed {
            message: "Invalid encrypted file format".to_string(),
        });
    }

    let salt = &data[0..16];
    let nonce = &data[16..28];
    let ciphertext = &data[28..];

    let key_bytes = derive_key(&password, salt);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce);

    let plaintext = cipher.decrypt(nonce, ciphertext)?;

    async_fs::write(&output_path, plaintext).await?;
    Ok(())
}

/// Hash text using the specified algorithm
pub fn hash_text_impl(text: String, algorithm: String) -> Result<String, CryptoError> {
    if text.is_empty() {
        return Err(CryptoError::InvalidInput {
            message: "Text cannot be empty".to_string(),
        });
    }

    match algorithm.to_lowercase().as_str() {
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(text.as_bytes());
            let result = hasher.finalize();
            Ok(hex::encode(result))
        }
        "sha512" => {
            let mut hasher = Sha512::new();
            hasher.update(text.as_bytes());
            let result = hasher.finalize();
            Ok(hex::encode(result))
        }
        _ => Err(CryptoError::UnsupportedAlgorithm { algorithm }),
    }
}

/// Generate a cryptographically secure random key
pub fn generate_secure_key_impl(length: u32) -> Result<String, CryptoError> {
    if length == 0 {
        return Err(CryptoError::InvalidInput {
            message: "Key length must be greater than 0".to_string(),
        });
    }

    if length > 1024 {
        return Err(CryptoError::InvalidInput {
            message: "Key length cannot exceed 1024 bytes".to_string(),
        });
    }

    let mut key = vec![0u8; length as usize];
    OsRng.fill_bytes(&mut key);
    Ok(general_purpose::STANDARD.encode(key))
}

/// Generate RSA key pair
pub async fn generate_rsa_key_pair_impl(key_size: u32) -> Result<RsaKeyPair, CryptoError> {
    if ![2048, 3072, 4096].contains(&key_size) {
        return Err(CryptoError::InvalidInput {
            message: "Key size must be 2048, 3072, or 4096 bits".to_string(),
        });
    }

    let private_key = RsaPrivateKey::new(&mut OsRng, key_size as usize)
        .map_err(|e| CryptoError::KeyGenerationFailed {
            message: e.to_string(),
        })?;

    let public_key = RsaPublicKey::from(&private_key);

    let private_key_pem = private_key
        .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
        .map_err(|e| CryptoError::KeyGenerationFailed {
            message: e.to_string(),
        })?
        .to_string();

    let public_key_pem = public_key
        .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
        .map_err(|e| CryptoError::KeyGenerationFailed {
            message: e.to_string(),
        })?;

    Ok(RsaKeyPair {
        private_key_pem,
        public_key_pem,
    })
}

/// Encrypt text using RSA public key
pub fn rsa_encrypt_impl(text: String, public_key_pem: String) -> Result<String, CryptoError> {
    if text.is_empty() {
        return Err(CryptoError::InvalidInput {
            message: "Text cannot be empty".to_string(),
        });
    }

    let public_key = RsaPublicKey::from_public_key_pem(&public_key_pem)?;
    let encrypted = public_key
        .encrypt(&mut OsRng, rsa::Pkcs1v15Encrypt, text.as_bytes())
        .map_err(|e| CryptoError::EncryptionFailed {
            message: e.to_string(),
        })?;

    Ok(general_purpose::STANDARD.encode(encrypted))
}

/// Decrypt text using RSA private key
pub fn rsa_decrypt_impl(encrypted_data: String, private_key_pem: String) -> Result<String, CryptoError> {
    if encrypted_data.is_empty() {
        return Err(CryptoError::InvalidInput {
            message: "Encrypted data cannot be empty".to_string(),
        });
    }

    let private_key = RsaPrivateKey::from_pkcs8_pem(&private_key_pem)?;
    let encrypted_bytes = general_purpose::STANDARD.decode(encrypted_data)?;
    
    let decrypted = private_key
        .decrypt(rsa::Pkcs1v15Encrypt, &encrypted_bytes)
        .map_err(|e| CryptoError::DecryptionFailed {
            message: e.to_string(),
        })?;

    String::from_utf8(decrypted).map_err(|e| CryptoError::DecryptionFailed {
        message: format!("Invalid UTF-8: {}", e),
    })
}

/// Sign text using RSA private key
pub fn rsa_sign_impl(text: String, private_key_pem: String) -> Result<String, CryptoError> {
    if text.is_empty() {
        return Err(CryptoError::InvalidInput {
            message: "Text cannot be empty".to_string(),
        });
    }

    let private_key = RsaPrivateKey::from_pkcs8_pem(&private_key_pem)?;
    let signing_key = SigningKey::<Sha256>::new(private_key);
    
    let signature = signing_key.sign_with_rng(&mut OsRng, text.as_bytes());

    Ok(general_purpose::STANDARD.encode(signature.to_bytes()))
}

/// Verify RSA signature
pub fn rsa_verify_impl(text: String, signature: String, public_key_pem: String) -> Result<bool, CryptoError> {
    if text.is_empty() {
        return Err(CryptoError::InvalidInput {
            message: "Text cannot be empty".to_string(),
        });
    }

    if signature.is_empty() {
        return Err(CryptoError::InvalidInput {
            message: "Signature cannot be empty".to_string(),
        });
    }

    let public_key = RsaPublicKey::from_public_key_pem(&public_key_pem)?;
    let verifying_key = VerifyingKey::<Sha256>::new(public_key);
    let signature_bytes = general_purpose::STANDARD.decode(signature)?;
    
    let signature = Signature::try_from(signature_bytes.as_slice())
        .map_err(|e| CryptoError::RsaOperationFailed {
            message: e.to_string(),
        })?;

    match verifying_key.verify(text.as_bytes(), &signature) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
