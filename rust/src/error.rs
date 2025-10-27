use flutter_rust_bridge::frb;

/// Custom error type for cryptographic operations
#[frb]
#[derive(Debug, Clone)]
pub enum CryptoError {
    /// Invalid input parameters
    InvalidInput { message: String },
    /// Encryption operation failed
    EncryptionFailed { message: String },
    /// Decryption operation failed
    DecryptionFailed { message: String },
    /// Key generation failed
    KeyGenerationFailed { message: String },
    /// File operation failed
    FileOperationFailed { message: String },
    /// Hash operation failed
    HashFailed { message: String },
    /// RSA operation failed
    RsaOperationFailed { message: String },
    /// Signature verification failed
    SignatureVerificationFailed { message: String },
    /// Unsupported algorithm
    UnsupportedAlgorithm { algorithm: String },
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::InvalidInput { message } => write!(f, "Invalid input: {}", message),
            CryptoError::EncryptionFailed { message } => write!(f, "Encryption failed: {}", message),
            CryptoError::DecryptionFailed { message } => write!(f, "Decryption failed: {}", message),
            CryptoError::KeyGenerationFailed { message } => write!(f, "Key generation failed: {}", message),
            CryptoError::FileOperationFailed { message } => write!(f, "File operation failed: {}", message),
            CryptoError::HashFailed { message } => write!(f, "Hash operation failed: {}", message),
            CryptoError::RsaOperationFailed { message } => write!(f, "RSA operation failed: {}", message),
            CryptoError::SignatureVerificationFailed { message } => write!(f, "Signature verification failed: {}", message),
            CryptoError::UnsupportedAlgorithm { algorithm } => write!(f, "Unsupported algorithm: {}", algorithm),
        }
    }
}

impl std::error::Error for CryptoError {}

impl From<std::io::Error> for CryptoError {
    fn from(error: std::io::Error) -> Self {
        CryptoError::FileOperationFailed {
            message: error.to_string(),
        }
    }
}

impl From<base64::DecodeError> for CryptoError {
    fn from(error: base64::DecodeError) -> Self {
        CryptoError::InvalidInput {
            message: format!("Base64 decode error: {}", error),
        }
    }
}

impl From<aes_gcm::Error> for CryptoError {
    fn from(error: aes_gcm::Error) -> Self {
        CryptoError::DecryptionFailed {
            message: format!("AES-GCM error: {}", error),
        }
    }
}

impl From<rsa::Error> for CryptoError {
    fn from(error: rsa::Error) -> Self {
        CryptoError::RsaOperationFailed {
            message: error.to_string(),
        }
    }
}

impl From<rsa::pkcs8::Error> for CryptoError {
    fn from(error: rsa::pkcs8::Error) -> Self {
        CryptoError::RsaOperationFailed {
            message: error.to_string(),
        }
    }
}

impl From<rsa::pkcs8::spki::Error> for CryptoError {
    fn from(error: rsa::pkcs8::spki::Error) -> Self {
        CryptoError::RsaOperationFailed {
            message: error.to_string(),
        }
    }
}
