mod frb_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
pub use crate::frb_generated::*; // Re-export generated symbols for use across the crate
use flutter_rust_bridge::frb;

mod crypto;
mod error;

pub use crypto::*;
pub use error::*;

/// Initialize the Rust library for Flutter
#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

/// Encrypt text using AES-256-GCM with PBKDF2 key derivation
/// 
/// # Arguments
/// * `text` - The plaintext to encrypt
/// * `password` - The password to derive the encryption key from
/// 
/// # Returns
/// * `Result<String, CryptoError>` - Base64 encoded encrypted data or error
#[frb(sync)]
pub fn encrypt_text(text: String, password: String) -> Result<String, CryptoError> {
    crypto::encrypt_text_impl(text, password)
}

/// Decrypt text using AES-256-GCM with PBKDF2 key derivation
/// 
/// # Arguments
/// * `encrypted_data` - Base64 encoded encrypted data
/// * `password` - The password to derive the decryption key from
/// 
/// # Returns
/// * `Result<String, CryptoError>` - Decrypted plaintext or error
#[frb(sync)]
pub fn decrypt_text(encrypted_data: String, password: String) -> Result<String, CryptoError> {
    crypto::decrypt_text_impl(encrypted_data, password)
}

/// Encrypt a file using AES-256-GCM
/// 
/// # Arguments
/// * `input_path` - Path to the input file
/// * `output_path` - Path to save the encrypted file
/// * `password` - The password to derive the encryption key from
/// 
/// # Returns
/// * `Result<(), CryptoError>` - Success or error
pub async fn encrypt_file(input_path: String, output_path: String, password: String) -> Result<(), CryptoError> {
    crypto::encrypt_file_impl(input_path, output_path, password).await
}

/// Decrypt a file using AES-256-GCM
/// 
/// # Arguments
/// * `input_path` - Path to the encrypted file
/// * `output_path` - Path to save the decrypted file
/// * `password` - The password to derive the decryption key from
/// 
/// # Returns
/// * `Result<(), CryptoError>` - Success or error
pub async fn decrypt_file(input_path: String, output_path: String, password: String) -> Result<(), CryptoError> {
    crypto::decrypt_file_impl(input_path, output_path, password).await
}

/// Hash text using the specified algorithm
/// 
/// # Arguments
/// * `text` - The text to hash
/// * `algorithm` - The hash algorithm ("sha256", "sha512")
/// 
/// # Returns
/// * `Result<String, CryptoError>` - Hex encoded hash or error
#[frb(sync)]
pub fn hash_text(text: String, algorithm: String) -> Result<String, CryptoError> {
    crypto::hash_text_impl(text, algorithm)
}

/// Generate a cryptographically secure random key
/// 
/// # Arguments
/// * `length` - The length of the key in bytes
/// 
/// # Returns
/// * `Result<String, CryptoError>` - Base64 encoded random key or error
#[frb(sync)]
pub fn generate_secure_key(length: u32) -> Result<String, CryptoError> {
    crypto::generate_secure_key_impl(length)
}

/// Generate RSA key pair
/// 
/// # Arguments
/// * `key_size` - The size of the RSA key in bits (2048, 3072, 4096)
/// 
/// # Returns
/// * `Result<RsaKeyPair, CryptoError>` - RSA key pair or error
pub async fn generate_rsa_key_pair(key_size: u32) -> Result<RsaKeyPair, CryptoError> {
    crypto::generate_rsa_key_pair_impl(key_size).await
}

/// Encrypt text using RSA public key
/// 
/// # Arguments
/// * `text` - The plaintext to encrypt
/// * `public_key_pem` - The RSA public key in PEM format
/// 
/// # Returns
/// * `Result<String, CryptoError>` - Base64 encoded encrypted data or error
#[frb(sync)]
pub fn rsa_encrypt(text: String, public_key_pem: String) -> Result<String, CryptoError> {
    crypto::rsa_encrypt_impl(text, public_key_pem)
}

/// Decrypt text using RSA private key
/// 
/// # Arguments
/// * `encrypted_data` - Base64 encoded encrypted data
/// * `private_key_pem` - The RSA private key in PEM format
/// 
/// # Returns
/// * `Result<String, CryptoError>` - Decrypted plaintext or error
#[frb(sync)]
pub fn rsa_decrypt(encrypted_data: String, private_key_pem: String) -> Result<String, CryptoError> {
    crypto::rsa_decrypt_impl(encrypted_data, private_key_pem)
}

/// Sign text using RSA private key
/// 
/// # Arguments
/// * `text` - The text to sign
/// * `private_key_pem` - The RSA private key in PEM format
/// 
/// # Returns
/// * `Result<String, CryptoError>` - Base64 encoded signature or error
#[frb(sync)]
pub fn rsa_sign(text: String, private_key_pem: String) -> Result<String, CryptoError> {
    crypto::rsa_sign_impl(text, private_key_pem)
}

/// Verify RSA signature
/// 
/// # Arguments
/// * `text` - The original text
/// * `signature` - Base64 encoded signature
/// * `public_key_pem` - The RSA public key in PEM format
/// 
/// # Returns
/// * `Result<bool, CryptoError>` - Verification result or error
#[frb(sync)]
pub fn rsa_verify(text: String, signature: String, public_key_pem: String) -> Result<bool, CryptoError> {
    crypto::rsa_verify_impl(text, signature, public_key_pem)
}
