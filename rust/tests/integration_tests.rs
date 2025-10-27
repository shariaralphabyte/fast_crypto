use fast_crypto::*;

#[tokio::test]
async fn test_text_encryption_decryption() {
    let text = "Hello, World! This is a test message.";
    let password = "secure_password_123";
    
    // Test encryption
    let encrypted = encrypt_text(text.to_string(), password.to_string()).unwrap();
    assert!(!encrypted.is_empty());
    assert_ne!(encrypted, text);
    
    // Test decryption
    let decrypted = decrypt_text(encrypted, password.to_string()).unwrap();
    assert_eq!(decrypted, text);
}

#[tokio::test]
async fn test_text_encryption_wrong_password() {
    let text = "Hello, World!";
    let password = "correct_password";
    let wrong_password = "wrong_password";
    
    let encrypted = encrypt_text(text.to_string(), password.to_string()).unwrap();
    let result = decrypt_text(encrypted, wrong_password.to_string());
    
    assert!(result.is_err());
}

#[tokio::test]
async fn test_hash_text_sha256() {
    let text = "Hello, World!";
    let hash = hash_text(text.to_string(), "sha256".to_string()).unwrap();
    
    // SHA-256 hash should be 64 characters (32 bytes in hex)
    assert_eq!(hash.len(), 64);
    
    // Test consistency
    let hash2 = hash_text(text.to_string(), "sha256".to_string()).unwrap();
    assert_eq!(hash, hash2);
}

#[tokio::test]
async fn test_hash_text_sha512() {
    let text = "Hello, World!";
    let hash = hash_text(text.to_string(), "sha512".to_string()).unwrap();
    
    // SHA-512 hash should be 128 characters (64 bytes in hex)
    assert_eq!(hash.len(), 128);
}

#[tokio::test]
async fn test_generate_secure_key() {
    let key_length = 32;
    let key = generate_secure_key(key_length).unwrap();
    
    // Base64 encoded 32 bytes should be 44 characters (with padding)
    assert_eq!(key.len(), 44);
    
    // Test that two generated keys are different
    let key2 = generate_secure_key(key_length).unwrap();
    assert_ne!(key, key2);
}

#[tokio::test]
async fn test_rsa_key_generation() {
    let key_pair = generate_rsa_key_pair(2048).await.unwrap();
    
    assert!(key_pair.private_key_pem.contains("BEGIN PRIVATE KEY"));
    assert!(key_pair.private_key_pem.contains("END PRIVATE KEY"));
    assert!(key_pair.public_key_pem.contains("BEGIN PUBLIC KEY"));
    assert!(key_pair.public_key_pem.contains("END PUBLIC KEY"));
}

#[tokio::test]
async fn test_rsa_encryption_decryption() {
    let key_pair = generate_rsa_key_pair(2048).await.unwrap();
    let text = "Hello, RSA!";
    
    // Test RSA encryption
    let encrypted = rsa_encrypt(text.to_string(), key_pair.public_key_pem.clone()).unwrap();
    assert!(!encrypted.is_empty());
    assert_ne!(encrypted, text);
    
    // Test RSA decryption
    let decrypted = rsa_decrypt(encrypted, key_pair.private_key_pem).unwrap();
    assert_eq!(decrypted, text);
}

#[tokio::test]
async fn test_rsa_sign_verify() {
    let key_pair = generate_rsa_key_pair(2048).await.unwrap();
    let text = "Message to sign";
    
    // Test RSA signing
    let signature = rsa_sign(text.to_string(), key_pair.private_key_pem).unwrap();
    assert!(!signature.is_empty());
    
    // Test RSA verification
    let is_valid = rsa_verify(text.to_string(), signature.clone(), key_pair.public_key_pem.clone()).unwrap();
    assert!(is_valid);
    
    // Test with wrong message
    let is_invalid = rsa_verify("Wrong message".to_string(), signature, key_pair.public_key_pem).unwrap();
    assert!(!is_invalid);
}

#[tokio::test]
async fn test_file_encryption_decryption() {
    use std::fs;
    use std::io::Write;
    
    let test_content = "This is test file content for encryption.";
    let password = "file_password_123";
    
    // Create test file
    let input_path = "/tmp/test_input.txt";
    let encrypted_path = "/tmp/test_encrypted.enc";
    let decrypted_path = "/tmp/test_decrypted.txt";
    
    fs::write(input_path, test_content).unwrap();
    
    // Test file encryption
    encrypt_file(input_path.to_string(), encrypted_path.to_string(), password.to_string()).await.unwrap();
    assert!(fs::metadata(encrypted_path).is_ok());
    
    // Test file decryption
    decrypt_file(encrypted_path.to_string(), decrypted_path.to_string(), password.to_string()).await.unwrap();
    
    let decrypted_content = fs::read_to_string(decrypted_path).unwrap();
    assert_eq!(decrypted_content, test_content);
    
    // Cleanup
    let _ = fs::remove_file(input_path);
    let _ = fs::remove_file(encrypted_path);
    let _ = fs::remove_file(decrypted_path);
}

#[tokio::test]
async fn test_invalid_inputs() {
    // Test empty text encryption
    let result = encrypt_text("".to_string(), "password".to_string());
    assert!(result.is_err());
    
    // Test empty password
    let result = encrypt_text("text".to_string(), "".to_string());
    assert!(result.is_err());
    
    // Test invalid hash algorithm
    let result = hash_text("text".to_string(), "invalid_algo".to_string());
    assert!(result.is_err());
    
    // Test zero key length
    let result = generate_secure_key(0);
    assert!(result.is_err());
    
    // Test invalid RSA key size
    let result = generate_rsa_key_pair(1024).await;
    assert!(result.is_err());
}
