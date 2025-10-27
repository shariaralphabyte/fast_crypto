import 'package:flutter/material.dart';
import 'dart:async';
import 'package:fast_crypto/fast_crypto.dart';
import 'package:file_picker/file_picker.dart';
import 'package:path_provider/path_provider.dart';

void main() {
  runZonedGuarded(() async {
    WidgetsFlutterBinding.ensureInitialized();

    // Initialize the fast_crypto plugin (with error handling for testing)
    try {
      await initFastCrypto();
    } catch (e, st) {
      debugPrint('[FastCrypto][Init][Error] $e');
      debugPrint(st.toString());
    }

    runApp(const MyApp());
  }, (error, stack) {
    debugPrint('[FastCrypto][Uncaught] $error');
    debugPrint(stack.toString());
  });
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Fast Crypto Demo',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const MyHomePage(title: 'Fast Crypto Demo'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> with TickerProviderStateMixin {
  late TabController _tabController;
  
  // Text encryption controllers
  final TextEditingController _textController = TextEditingController();
  final TextEditingController _passwordController = TextEditingController();
  final TextEditingController _resultController = TextEditingController();
  
  // Hash controllers
  final TextEditingController _hashTextController = TextEditingController();
  final TextEditingController _hashResultController = TextEditingController();
  String _selectedHashAlgorithm = 'sha256';
  
  // RSA controllers
  final TextEditingController _rsaTextController = TextEditingController();
  final TextEditingController _rsaPublicKeyController = TextEditingController();
  final TextEditingController _rsaPrivateKeyController = TextEditingController();
  final TextEditingController _rsaResultController = TextEditingController();
  
  // File encryption
  String? _selectedFilePath;
  String _fileOperationResult = '';
  
  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: 4, vsync: this);
  }

  @override
  void dispose() {
    _tabController.dispose();
    _textController.dispose();
    _passwordController.dispose();
    _resultController.dispose();
    _hashTextController.dispose();
    _hashResultController.dispose();
    _rsaTextController.dispose();
    _rsaPublicKeyController.dispose();
    _rsaPrivateKeyController.dispose();
    _rsaResultController.dispose();
    super.dispose();
  }

  void _showSnackBar(String message, {bool isError = false}) {
    // Also print to console for debugging
    debugPrint('[FastCrypto][Snackbar] ${isError ? 'ERROR' : 'INFO'}: $message');
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(message),
        backgroundColor: isError ? Colors.red : Colors.green,
        duration: const Duration(seconds: 3),
      ),
    );
  }

  void _logError(String context, Object e, StackTrace st) {
    debugPrint('[FastCrypto][Error][$context] $e');
    debugPrint(st.toString());
  }

  Future<void> _encryptText() async {
    try {
      if (_textController.text.isEmpty || _passwordController.text.isEmpty) {
        _showSnackBar('Please enter both text and password', isError: true);
        return;
      }
      
      final encrypted = encryptText(
        text: _textController.text,
        password: _passwordController.text,
      );
      
      setState(() {
        _resultController.text = encrypted;
      });
      
      _showSnackBar('Text encrypted successfully!');
    } catch (e, st) {
      _logError('EncryptText', e, st);
      _showSnackBar('Encryption failed: $e', isError: true);
    }
  }

  Future<void> _decryptText() async {
    try {
      if (_resultController.text.isEmpty || _passwordController.text.isEmpty) {
        _showSnackBar('Please enter both encrypted text and password', isError: true);
        return;
      }
      
      final decrypted = decryptText(
        encryptedData: _resultController.text,
        password: _passwordController.text,
      );
      
      setState(() {
        _textController.text = decrypted;
      });
      
      _showSnackBar('Text decrypted successfully!');
    } catch (e, st) {
      _logError('DecryptText', e, st);
      _showSnackBar('Decryption failed: $e', isError: true);
    }
  }

  Future<void> _hashText() async {
    try {
      if (_hashTextController.text.isEmpty) {
        _showSnackBar('Please enter text to hash', isError: true);
        return;
      }
      
      final hash = hashText(
        text: _hashTextController.text,
        algorithm: _selectedHashAlgorithm,
      );
      
      setState(() {
        _hashResultController.text = hash;
      });
      
      _showSnackBar('Text hashed successfully!');
    } catch (e, st) {
      _logError('HashText', e, st);
      _showSnackBar('Hashing failed: $e', isError: true);
    }
  }

  Future<void> _generateRsaKeyPair() async {
    try {
      final keyPair = await generateRsaKeyPair(keySize: 2048);
      
      setState(() {
        _rsaPublicKeyController.text = keyPair.publicKeyPem;
        _rsaPrivateKeyController.text = keyPair.privateKeyPem;
      });
      
      _showSnackBar('RSA key pair generated successfully!');
    } catch (e, st) {
      _logError('GenerateRsaKeyPair', e, st);
      _showSnackBar('Key generation failed: $e', isError: true);
    }
  }

  Future<void> _rsaEncrypt() async {
    try {
      if (_rsaTextController.text.isEmpty || _rsaPublicKeyController.text.isEmpty) {
        _showSnackBar('Please enter text and public key', isError: true);
        return;
      }
      
      final encrypted = rsaEncrypt(
        text: _rsaTextController.text,
        publicKeyPem: _rsaPublicKeyController.text,
      );
      
      setState(() {
        _rsaResultController.text = encrypted;
      });
      
      _showSnackBar('RSA encryption successful!');
    } catch (e, st) {
      _logError('RsaEncrypt', e, st);
      _showSnackBar('RSA encryption failed: $e', isError: true);
    }
  }

  Future<void> _rsaDecrypt() async {
    try {
      if (_rsaResultController.text.isEmpty || _rsaPrivateKeyController.text.isEmpty) {
        _showSnackBar('Please enter encrypted data and private key', isError: true);
        return;
      }
      
      final decrypted = rsaDecrypt(
        encryptedData: _rsaResultController.text,
        privateKeyPem: _rsaPrivateKeyController.text,
      );
      
      setState(() {
        _rsaTextController.text = decrypted;
      });
      
      _showSnackBar('RSA decryption successful!');
    } catch (e, st) {
      _logError('RsaDecrypt', e, st);
      _showSnackBar('RSA decryption failed: $e', isError: true);
    }
  }

  Future<void> _pickFile() async {
    try {
      FilePickerResult? result = await FilePicker.platform.pickFiles();
      
      if (result != null) {
        setState(() {
          _selectedFilePath = result.files.single.path;
        });
      }
    } catch (e, st) {
      _logError('PickFile', e, st);
      _showSnackBar('File selection failed: $e', isError: true);
    }
  }

  Future<void> _encryptFile() async {
    try {
      if (_selectedFilePath == null || _passwordController.text.isEmpty) {
        _showSnackBar('Please select a file and enter password', isError: true);
        return;
      }
      
      final directory = await getApplicationDocumentsDirectory();
      final outputPath = '${directory.path}/encrypted_file.enc';
      
      await encryptFile(
        inputPath: _selectedFilePath!,
        outputPath: outputPath,
        password: _passwordController.text,
      );
      
      setState(() {
        _fileOperationResult = 'File encrypted successfully!\nSaved to: $outputPath';
      });
      
      _showSnackBar('File encrypted successfully!');
    } catch (e, st) {
      _logError('EncryptFile', e, st);
      _showSnackBar('File encryption failed: $e', isError: true);
    }
  }

  Future<void> _decryptFile() async {
    try {
      if (_selectedFilePath == null || _passwordController.text.isEmpty) {
        _showSnackBar('Please select an encrypted file and enter password', isError: true);
        return;
      }
      
      final directory = await getApplicationDocumentsDirectory();
      final outputPath = '${directory.path}/decrypted_file';
      
      await decryptFile(
        inputPath: _selectedFilePath!,
        outputPath: outputPath,
        password: _passwordController.text,
      );
      
      setState(() {
        _fileOperationResult = 'File decrypted successfully!\nSaved to: $outputPath';
      });
      
      _showSnackBar('File decrypted successfully!');
    } catch (e, st) {
      _logError('DecryptFile', e, st);
      _showSnackBar('File decryption failed: $e', isError: true);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text(widget.title),
        bottom: TabBar(
          controller: _tabController,
          tabs: const [
            Tab(text: 'Text Crypto'),
            Tab(text: 'Hashing'),
            Tab(text: 'RSA'),
            Tab(text: 'File Crypto'),
          ],
        ),
      ),
      body: TabBarView(
        controller: _tabController,
        children: [
          _buildTextCryptoTab(),
          _buildHashingTab(),
          _buildRsaTab(),
          _buildFileCryptoTab(),
        ],
      ),
    );
  }

  Widget _buildTextCryptoTab() {
    return Padding(
      padding: const EdgeInsets.all(16.0),
      child: Column(
        children: [
          TextField(
            controller: _textController,
            decoration: const InputDecoration(
              labelText: 'Text to encrypt/decrypt',
              border: OutlineInputBorder(),
            ),
            maxLines: 3,
          ),
          const SizedBox(height: 16),
          TextField(
            controller: _passwordController,
            decoration: const InputDecoration(
              labelText: 'Password',
              border: OutlineInputBorder(),
            ),
            obscureText: true,
          ),
          const SizedBox(height: 16),
          Row(
            children: [
              Expanded(
                child: ElevatedButton(
                  onPressed: _encryptText,
                  child: const Text('Encrypt'),
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: ElevatedButton(
                  onPressed: _decryptText,
                  child: const Text('Decrypt'),
                ),
              ),
            ],
          ),
          const SizedBox(height: 16),
          Expanded(
            child: TextField(
              controller: _resultController,
              decoration: const InputDecoration(
                labelText: 'Result',
                border: OutlineInputBorder(),
              ),
              maxLines: null,
              expands: true,
              readOnly: true,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildHashingTab() {
    return Padding(
      padding: const EdgeInsets.all(16.0),
      child: Column(
        children: [
          TextField(
            controller: _hashTextController,
            decoration: const InputDecoration(
              labelText: 'Text to hash',
              border: OutlineInputBorder(),
            ),
            maxLines: 3,
          ),
          const SizedBox(height: 16),
          DropdownButtonFormField<String>(
            initialValue: _selectedHashAlgorithm,
            decoration: const InputDecoration(
              labelText: 'Hash Algorithm',
              border: OutlineInputBorder(),
            ),
            items: const [
              DropdownMenuItem(value: 'sha256', child: Text('SHA-256')),
              DropdownMenuItem(value: 'sha512', child: Text('SHA-512')),
            ],
            onChanged: (value) {
              setState(() {
                _selectedHashAlgorithm = value!;
              });
            },
          ),
          const SizedBox(height: 16),
          ElevatedButton(
            onPressed: _hashText,
            child: const Text('Generate Hash'),
          ),
          const SizedBox(height: 16),
          Expanded(
            child: TextField(
              controller: _hashResultController,
              decoration: const InputDecoration(
                labelText: 'Hash Result',
                border: OutlineInputBorder(),
              ),
              maxLines: null,
              expands: true,
              readOnly: true,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildRsaTab() {
    return Padding(
      padding: const EdgeInsets.all(16.0),
      child: SingleChildScrollView(
        child: Column(
          children: [
            ElevatedButton(
              onPressed: _generateRsaKeyPair,
              child: const Text('Generate RSA Key Pair'),
            ),
            const SizedBox(height: 16),
            TextField(
              controller: _rsaTextController,
              decoration: const InputDecoration(
                labelText: 'Text to encrypt/decrypt',
                border: OutlineInputBorder(),
              ),
              maxLines: 2,
            ),
            const SizedBox(height: 16),
            TextField(
              controller: _rsaPublicKeyController,
              decoration: const InputDecoration(
                labelText: 'Public Key (PEM)',
                border: OutlineInputBorder(),
              ),
              maxLines: 4,
            ),
            const SizedBox(height: 16),
            TextField(
              controller: _rsaPrivateKeyController,
              decoration: const InputDecoration(
                labelText: 'Private Key (PEM)',
                border: OutlineInputBorder(),
              ),
              maxLines: 4,
            ),
            const SizedBox(height: 16),
            Row(
              children: [
                Expanded(
                  child: ElevatedButton(
                    onPressed: _rsaEncrypt,
                    child: const Text('RSA Encrypt'),
                  ),
                ),
                const SizedBox(width: 16),
                Expanded(
                  child: ElevatedButton(
                    onPressed: _rsaDecrypt,
                    child: const Text('RSA Decrypt'),
                  ),
                ),
              ],
            ),
            const SizedBox(height: 16),
            TextField(
              controller: _rsaResultController,
              decoration: const InputDecoration(
                labelText: 'RSA Result',
                border: OutlineInputBorder(),
              ),
              maxLines: 4,
              readOnly: true,
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildFileCryptoTab() {
    return Padding(
      padding: const EdgeInsets.all(16.0),
      child: Column(
        children: [
          TextField(
            controller: _passwordController,
            decoration: const InputDecoration(
              labelText: 'Password for file encryption',
              border: OutlineInputBorder(),
            ),
            obscureText: true,
          ),
          const SizedBox(height: 16),
          ElevatedButton(
            onPressed: _pickFile,
            child: const Text('Select File'),
          ),
          const SizedBox(height: 16),
          if (_selectedFilePath != null)
            Text(
              'Selected: ${_selectedFilePath!.split('/').last}',
              style: Theme.of(context).textTheme.bodyMedium,
            ),
          const SizedBox(height: 16),
          Row(
            children: [
              Expanded(
                child: ElevatedButton(
                  onPressed: _encryptFile,
                  child: const Text('Encrypt File'),
                ),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: ElevatedButton(
                  onPressed: _decryptFile,
                  child: const Text('Decrypt File'),
                ),
              ),
            ],
          ),
          const SizedBox(height: 16),
          Expanded(
            child: Container(
              width: double.infinity,
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                border: Border.all(color: Colors.grey),
                borderRadius: BorderRadius.circular(8),
              ),
              child: Text(
                _fileOperationResult.isEmpty 
                    ? 'File operation results will appear here...'
                    : _fileOperationResult,
                style: Theme.of(context).textTheme.bodyMedium,
              ),
            ),
          ),
        ],
      ),
    );
  }
}
