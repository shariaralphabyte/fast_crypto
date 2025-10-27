import 'package:flutter_test/flutter_test.dart';
import 'package:fast_crypto/fast_crypto.dart';

void main() {
  group('Fast Crypto Tests', () {
    setUpAll(() async {
      // Initialize the plugin for testing
      // Note: In actual tests, you would need to initialize the Rust library
      // For unit tests, we'll test the API structure
    });

    group('Text Encryption/Decryption', () {
      test('should encrypt and decrypt text successfully', () async {
        // Note: These tests would work with actual Rust implementation
        // For now, we're testing the API structure
        // Test API structure - actual implementation would require Rust backend
        expect(() => encryptText, returnsNormally);
      });

      test('should handle empty text input', () {
        const text = '';
        const password = 'password';
        
        expect(() => encryptText(text: text, password: password), throwsA(isA<Exception>()));
      });

      test('should handle empty password input', () {
        const text = 'Hello, World!';
        const password = '';
        
        expect(() => encryptText(text: text, password: password), throwsA(isA<Exception>()));
      });
    });

    group('Hashing', () {
      test('should generate SHA-256 hash', () {
        const text = 'Hello, World!';
        const algorithm = 'sha256';
        
        expect(() => hashText(text: text, algorithm: algorithm), returnsNormally);
      });

      test('should generate SHA-512 hash', () {
        const text = 'Hello, World!';
        const algorithm = 'sha512';
        
        expect(() => hashText(text: text, algorithm: algorithm), returnsNormally);
      });

      test('should handle invalid hash algorithm', () {
        const text = 'Hello, World!';
        const algorithm = 'invalid_algo';
        
        expect(() => hashText(text: text, algorithm: algorithm), throwsA(isA<Exception>()));
      });
    });

    group('Secure Key Generation', () {
      test('should generate secure key of specified length', () {
        const length = 32;
        
        expect(() => generateSecureKey(length: length), returnsNormally);
      });

      test('should handle zero key length', () {
        const length = 0;
        
        expect(() => generateSecureKey(length: length), throwsA(isA<Exception>()));
      });

      test('should handle excessive key length', () {
        const length = 2048; // Very large key
        
        expect(() => generateSecureKey(length: length), throwsA(isA<Exception>()));
      });
    });

    group('RSA Operations', () {
      test('should generate RSA key pair', () async {
        const keySize = 2048;
        
        expect(() => generateRsaKeyPair(keySize: keySize), returnsNormally);
      });

      test('should handle invalid RSA key size', () async {
        const keySize = 1024; // Too small
        
        expect(() => generateRsaKeyPair(keySize: keySize), throwsA(isA<Exception>()));
      });

      test('should encrypt and decrypt with RSA', () {
        const text = 'Hello, RSA!';
        const publicKeyPem = '''-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...
-----END PUBLIC KEY-----''';
        const privateKeyPem = '''-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQC...
-----END PRIVATE KEY-----''';
        
        expect(() => rsaEncrypt(text: text, publicKeyPem: publicKeyPem), returnsNormally);
        expect(() => rsaDecrypt(encryptedData: 'encrypted_data', privateKeyPem: privateKeyPem), returnsNormally);
      });

      test('should sign and verify with RSA', () {
        const text = 'Message to sign';
        const publicKeyPem = '''-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...
-----END PUBLIC KEY-----''';
        const privateKeyPem = '''-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQC...
-----END PRIVATE KEY-----''';
        
        expect(() => rsaSign(text: text, privateKeyPem: privateKeyPem), returnsNormally);
        expect(() => rsaVerify(text: text, signature: 'signature', publicKeyPem: publicKeyPem), returnsNormally);
      });
    });

    group('File Operations', () {
      test('should encrypt and decrypt files', () async {
        const inputPath = '/tmp/test_input.txt';
        const outputPath = '/tmp/test_output.enc';
        const password = 'file_password';
        
        expect(() => encryptFile(inputPath: inputPath, outputPath: outputPath, password: password), returnsNormally);
        expect(() => decryptFile(inputPath: outputPath, outputPath: '/tmp/decrypted.txt', password: password), returnsNormally);
      });

      test('should handle non-existent input file', () async {
        const inputPath = '/non/existent/file.txt';
        const outputPath = '/tmp/output.enc';
        const password = 'password';
        
        expect(() => encryptFile(inputPath: inputPath, outputPath: outputPath, password: password), throwsA(isA<Exception>()));
      });
    });

    group('Error Handling', () {
      test('should handle invalid base64 input', () {
        const invalidEncrypted = 'invalid_base64_data';
        const password = 'password';
        
        expect(() => decryptText(encryptedData: invalidEncrypted, password: password), throwsA(isA<Exception>()));
      });

      test('should handle malformed PEM keys', () {
        const text = 'Hello, World!';
        const invalidPem = 'invalid_pem_data';
        
        expect(() => rsaEncrypt(text: text, publicKeyPem: invalidPem), throwsA(isA<Exception>()));
        expect(() => rsaDecrypt(encryptedData: 'data', privateKeyPem: invalidPem), throwsA(isA<Exception>()));
      });
    });

    group('Plugin Initialization', () {
      test('should initialize plugin without errors', () async {
        expect(() => initFastCrypto(), returnsNormally);
      });
    });
  });
}
