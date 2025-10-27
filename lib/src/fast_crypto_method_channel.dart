import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';

import 'fast_crypto_platform_interface.dart';

/// An implementation of [FastCryptoPlatform] that uses method channels.
class MethodChannelFastCrypto extends FastCryptoPlatform {
  /// The method channel used to interact with the native platform.
  @visibleForTesting
  final methodChannel = const MethodChannel('fast_crypto');

  @override
  Future<String?> getPlatformVersion() async {
    final version = await methodChannel.invokeMethod<String>('getPlatformVersion');
    return version;
  }
}
