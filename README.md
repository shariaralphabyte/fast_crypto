# fast_crypto

🚀 **High-performance Android encryption plugin powered by Rust**

A Flutter plugin providing AES-256-GCM, RSA, SHA-256/512, and file encryption with native Rust performance.

[![pub package](https://img.shields.io/pub/v/fast_crypto.svg)](https://pub.dev/packages/fast_crypto)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## ✨ Features

- **🔒 AES-256-GCM Encryption**: Secure symmetric encryption for text and files
- **🔑 RSA Encryption**: Asymmetric encryption with key generation, encryption, decryption, signing, and verification
- **🔗 SHA-256/SHA-512 Hashing**: Fast cryptographic hashing algorithms
- **🎲 Secure Key Generation**: Cryptographically secure random key generation
- **📁 File Encryption**: Encrypt and decrypt files of any size
- **⚡ High Performance**: Rust backend for maximum speed and safety
- **🤖 Android Only**: Optimized for Android (iOS coming soon)
- **🔄 Async Operations**: All APIs are async for smooth Flutter integration
- **🛡️ Memory Safe**: Rust's memory safety guarantees
- **📚 Well Documented**: Comprehensive documentation and examples

## 🚀 Installation

Add this to your package's `pubspec.yaml` file:

```yaml
dependencies:
  fast_crypto: ^0.1.0
```

Then run:

```bash
flutter pub get
```

## 📋 Requirements

- **Flutter SDK**: >=3.16.0
- **Dart SDK**: >=3.3.0
- **Android**: API level 24+ (Android 7.0+)
- **Rust**: Latest stable version (for building from source)
- **Android NDK**: For building native libraries (handled automatically by cargokit)

## 🛠️ Setup

### For App Developers

Just add the dependency and run `flutter pub get`. The Rust library builds automatically during `flutter build android` thanks to cargokit integration.

**Optional**: Install Rust for faster builds (recommended):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### For Plugin Contributors

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install tools
cargo install flutter_rust_bridge_codegen cargo-ndk

# Add Android targets
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android
```

## 📖 Usage

### Initialize the Plugin

```dart
import 'package:fast_crypto/fast_crypto.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  
  // Initialize the plugin
  await initFastCrypto();
  
  runApp(MyApp());
}
```

### Text Encryption & Decryption

```dart
import 'package:fast_crypto/fast_crypto.dart';

// Encrypt text
String plaintext = "Hello, World! This is a secret message.";
String password = "my_secure_password";

try {
  String encrypted = encryptText(text: plaintext, password: password);
  print("Encrypted: $encrypted");
  
  // Decrypt text
  String decrypted = decryptText(encryptedData: encrypted, password: password);
  print("Decrypted: $decrypted");
} catch (e) {
  print("Error: $e");
}
```

### File Encryption & Decryption

```dart
import 'package:fast_crypto/fast_crypto.dart';

try {
  // Encrypt a file
  await encryptFile(
    inputPath: "/path/to/input/file.txt",
    outputPath: "/path/to/encrypted/file.enc",
    password: "file_password"
  );
  
  // Decrypt a file
  await decryptFile(
    inputPath: "/path/to/encrypted/file.enc",
    outputPath: "/path/to/decrypted/file.txt",
    password: "file_password"
  );
} catch (e) {
  print("File operation error: $e");
}
```

### Hashing

```dart
import 'package:fast_crypto/fast_crypto.dart';

String text = "Hello, World!";

// SHA-256 hash
String sha256Hash = hashText(text: text, algorithm: "sha256");
print("SHA-256: $sha256Hash");

// SHA-512 hash
String sha512Hash = hashText(text: text, algorithm: "sha512");
print("SHA-512: $sha512Hash");
```

### RSA Operations

```dart
import 'package:fast_crypto/fast_crypto.dart';

try {
  // Generate RSA key pair
  RsaKeyPair keyPair = await generateRsaKeyPair(keySize: 2048);
  print("Public Key: ${keyPair.publicKeyPem}");
  print("Private Key: ${keyPair.privateKeyPem}");
  
  // RSA Encryption
  String message = "Hello, RSA!";
  String encrypted = rsaEncrypt(text: message, publicKeyPem: keyPair.publicKeyPem);
  print("RSA Encrypted: $encrypted");
  
  // RSA Decryption
  String decrypted = rsaDecrypt(encryptedData: encrypted, privateKeyPem: keyPair.privateKeyPem);
  print("RSA Decrypted: $decrypted");
  
  // RSA Signing
  String signature = rsaSign(text: message, privateKeyPem: keyPair.privateKeyPem);
  print("Signature: $signature");
  
  // RSA Verification
  bool isValid = rsaVerify(text: message, signature: signature, publicKeyPem: keyPair.publicKeyPem);
  print("Signature valid: $isValid");
} catch (e) {
  print("RSA operation error: $e");
}
```

### Secure Key Generation

```dart
import 'package:fast_crypto/fast_crypto.dart';

// Generate a 32-byte (256-bit) secure random key
String secureKey = generateSecureKey(length: 32);
print("Secure Key: $secureKey");
```

## 🏗️ Building

### Automatic Build (Recommended)

The Rust library builds automatically during `flutter build android` thanks to cargokit.

### Manual Build (For Development)

```bash
# Generate bridge code
flutter_rust_bridge_codegen generate

# Build for Android
export ANDROID_NDK_HOME="$HOME/Library/Android/sdk/ndk/29.0.13846066"
cd rust
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 -o ../android/src/main/jniLibs build --release
```

## 🧪 Testing

### Run Dart Tests

```bash
flutter test
```

### Run Rust Tests

```bash
cd rust
cargo test
```

## 📊 Performance

Fast Crypto delivers native performance with Rust:

- **10-100x faster** than pure Dart crypto implementations
- **AES-256-GCM**: Native-speed encryption
- **RSA operations**: Hardware-accelerated when available
- **SHA hashing**: Optimized for mobile devices
- **Memory efficient**: Rust's zero-cost abstractions
- **Battery friendly**: Minimal CPU overhead

## 🔒 Security Features

- **AES-256-GCM**: Authenticated encryption with associated data (AEAD)
- **PBKDF2**: Key derivation with 100,000 iterations
- **Secure Random**: Cryptographically secure random number generation
- **RSA-OAEP**: Optimal Asymmetric Encryption Padding
- **Memory Safety**: Rust's ownership system prevents memory vulnerabilities

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge) for the excellent Rust-Flutter bridge
- [aes-gcm](https://crates.io/crates/aes-gcm) for AES encryption
- [rsa](https://crates.io/crates/rsa) for RSA operations
- [sha2](https://crates.io/crates/sha2) for SHA hashing
- [pbkdf2](https://crates.io/crates/pbkdf2) for key derivation

## 🗺️ Roadmap

- [x] Android support with cargokit
- [x] AES-256-GCM encryption
- [x] RSA operations
- [x] SHA-256/512 hashing
- [x] File encryption


## 📞 Support

If you have any questions or issues, please [open an issue](https://github.com/your-username/fast_crypto/issues) on GitHub.

---

Made with ❤️ and 🦀 Rust | Android-only v0.1.0

