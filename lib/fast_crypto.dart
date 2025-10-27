
// A high-performance Flutter plugin for AES-256, RSA, and SHA256 encryption
// using Rust backend via flutter_rust_bridge.

export 'src/rust/lib.dart';
export 'src/rust/crypto.dart';
export 'src/rust/error.dart';
export 'src/rust/frb_generated.dart';
export 'src/fast_crypto_platform_interface.dart';
export 'src/fast_crypto_method_channel.dart';

import 'src/rust/frb_generated.dart';

/// Initialize the fast_crypto plugin
/// 
/// This must be called before using any other functions in the plugin.
/// It's recommended to call this in your app's main() function.
Future<void> initFastCrypto() async {
  await RustLib.init();
}
