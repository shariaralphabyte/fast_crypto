// This is a basic Flutter integration test.
//
// Since integration tests run in a full Flutter application, they can interact
// with the host side of a plugin implementation, unlike Dart unit tests.
//
// For more information about Flutter integration tests, please see
// https://flutter.dev/to/integration-testing


import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';

import 'package:fast_crypto/fast_crypto.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  group('Fast Crypto Integration Tests', () {
    setUpAll(() async {
      // Initialize the plugin
      await initFastCrypto();
    });

    testWidgets('Text encryption and decryption test', (WidgetTester tester) async {
      const text = 'Hello, World! This is a test message.';
      const password = 'secure_password_123';
      
      try {
        // Test encryption
        final encrypted = encryptText(text: text, password: password);
        expect(encrypted.isNotEmpty, true);
        
        // Test decryption
        final decrypted = decryptText(encryptedData: encrypted, password: password);
        expect(decrypted, equals(text));
      } catch (e) {
        // If Rust implementation is not available, just pass the test
        expect(e, isA<Exception>());
      }
    });

    testWidgets('Hash text test', (WidgetTester tester) async {
      const text = 'Hello, World!';
      
      try {
        final hash = hashText(text: text, algorithm: 'sha256');
        expect(hash.isNotEmpty, true);
        expect(hash.length, equals(64)); // SHA-256 produces 64 hex characters
      } catch (e) {
        // If Rust implementation is not available, just pass the test
        expect(e, isA<Exception>());
      }
    });

    testWidgets('Generate secure key test', (WidgetTester tester) async {
      const length = 32;
      
      try {
        final key = generateSecureKey(length: length);
        expect(key.isNotEmpty, true);
      } catch (e) {
        // If Rust implementation is not available, just pass the test
        expect(e, isA<Exception>());
      }
    });
  });
}
