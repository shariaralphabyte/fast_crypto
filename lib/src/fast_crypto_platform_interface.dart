import 'package:plugin_platform_interface/plugin_platform_interface.dart';

import 'fast_crypto_method_channel.dart';

abstract class FastCryptoPlatform extends PlatformInterface {
  /// Constructs a FastCryptoPlatform.
  FastCryptoPlatform() : super(token: _token);

  static final Object _token = Object();

  static FastCryptoPlatform _instance = MethodChannelFastCrypto();

  /// The default instance of [FastCryptoPlatform] to use.
  ///
  /// Defaults to [MethodChannelFastCrypto].
  static FastCryptoPlatform get instance => _instance;

  /// Platform-specific implementations should set this with their own
  /// platform-specific class that extends [FastCryptoPlatform] when
  /// they register themselves.
  static set instance(FastCryptoPlatform instance) {
    PlatformInterface.verifyToken(instance, _token);
    _instance = instance;
  }

  Future<String?> getPlatformVersion() {
    throw UnimplementedError('platformVersion() has not been implemented.');
  }
}
