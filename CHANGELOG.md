# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-26

### Added
- Initial Android-only release
- AES-256-GCM encryption and decryption for text
- File encryption and decryption for any file size
- RSA key generation (2048/3072/4096-bit)
- RSA encryption, decryption, signing, and verification
- SHA-256 and SHA-512 hashing
- Secure random key generation
- Rust backend for native performance (10-100x faster than Dart)
- cargokit integration for automatic Rust builds
- Comprehensive error handling with stack traces
- Full example app with UI for all features
- **Example App**: Complete demo app showcasing all features
- **Documentation**: Comprehensive README with usage examples and build instructions

### Security Features
- AES-256-GCM authenticated encryption
- PBKDF2 key derivation with 100,000 iterations
- RSA-OAEP padding for RSA encryption
- RSA-PSS signatures with SHA-256
- Cryptographically secure random number generation
- Memory-safe Rust implementation

### Performance
- Optimized Rust backend for maximum performance
- Zero-copy operations where possible
- Efficient memory management
- Release builds with LTO optimization

### Developer Experience
- Simple, intuitive API design
- Comprehensive error handling
- Well-documented functions
- Type-safe operations
- Flutter-friendly async patterns
