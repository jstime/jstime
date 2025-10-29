# Text Encoding and Cryptography APIs

This document describes the text encoding and cryptography APIs implemented in jstime, following WHATWG and W3C standards.

## Table of Contents

- [Text Encoding API](#text-encoding-api)
- [Web Cryptography API](#web-cryptography-api)

## Text Encoding API

jstime implements the [WHATWG Encoding Standard](https://encoding.spec.whatwg.org/), providing `TextEncoder` and `TextDecoder` for encoding and decoding text as UTF-8 bytes.

**📁 Example:** See [examples/text-encoding-demo.js](../../examples/text-encoding-demo.js) for a complete demonstration.

### Supported APIs

- `TextEncoder` - Encode strings to UTF-8 bytes
- `TextDecoder` - Decode UTF-8 bytes to strings

### TextEncoder

The `TextEncoder` class represents an encoder for UTF-8 encoding. It takes a stream of code points as input and emits a stream of UTF-8 bytes.

#### Constructor

```javascript
new TextEncoder()
```

The `TextEncoder` constructor takes no arguments and always encodes to UTF-8.

#### Properties

- `encoding` (read-only) - Always returns `"utf-8"`

#### Methods

- `encode(input)` - Encodes a string into a `Uint8Array` of UTF-8 bytes
  - `input` (string, optional) - The string to encode. Defaults to empty string.
  - Returns: `Uint8Array` containing the UTF-8 encoded bytes

- `encodeInto(source, destination)` - Encodes a string into an existing `Uint8Array`
  - `source` (string) - The string to encode
  - `destination` (Uint8Array) - The destination buffer to write to
  - Returns: `{ read: number, written: number }` object indicating how many UTF-16 code units were read and how many UTF-8 bytes were written

### TextDecoder

The `TextDecoder` class represents a decoder for UTF-8 encoded text. It takes a stream of UTF-8 bytes as input and emits a stream of code points.

#### Constructor

```javascript
new TextDecoder(label = 'utf-8', options = {})
```

**Parameters:**
- `label` (string, optional) - The encoding label. Only `"utf-8"`, `"utf8"`, and `"unicode-1-1-utf-8"` are supported. Defaults to `"utf-8"`.
- `options` (object, optional) - Currently accepts but ignores `fatal` and `ignoreBOM` options for compatibility

#### Properties

- `encoding` (read-only) - Returns `"utf-8"`
- `fatal` (read-only) - Returns the value set in the constructor options
- `ignoreBOM` (read-only) - Returns the value set in the constructor options

#### Methods

- `decode(input, options)` - Decodes a buffer of UTF-8 bytes into a string
  - `input` (ArrayBuffer | ArrayBufferView, optional) - The bytes to decode
  - `options` (object, optional) - Currently accepts but ignores `stream` option for compatibility
  - Returns: `string` containing the decoded text

### Examples

#### Basic Encoding and Decoding

```javascript
// Create encoder and decoder
const encoder = new TextEncoder();
const decoder = new TextDecoder();

// Encode a string
const text = "Hello, World!";
const encoded = encoder.encode(text);
console.log(encoded); // Uint8Array(13) [72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33]

// Decode the bytes
const decoded = decoder.decode(encoded);
console.log(decoded); // "Hello, World!"
```

#### Encoding UTF-8 Multi-byte Characters

```javascript
const encoder = new TextEncoder();

// Euro sign (3 bytes in UTF-8)
const euro = encoder.encode('€');
console.log(euro); // Uint8Array(3) [226, 130, 172]

// Chinese characters (3 bytes each)
const chinese = encoder.encode('世界');
console.log(chinese); // Uint8Array(6) [228, 184, 150, 231, 149, 140]

// Emoji (4 bytes in UTF-8)
const emoji = encoder.encode('😀');
console.log(emoji); // Uint8Array(4) [240, 159, 152, 128]
```

#### Using encodeInto for Efficient Encoding

```javascript
const encoder = new TextEncoder();

// Pre-allocate a buffer
const buffer = new Uint8Array(100);

// Encode into the buffer
const result = encoder.encodeInto('Hello', buffer);
console.log(result); // { read: 5, written: 5 }

// Check what was written
const written = buffer.slice(0, result.written);
console.log(Array.from(written)); // [72, 101, 108, 108, 111]
```

#### Handling Buffer Overflow with encodeInto

```javascript
const encoder = new TextEncoder();

// Small buffer that can't fit the entire string
const buffer = new Uint8Array(3);

const result = encoder.encodeInto('hello', buffer);
console.log(result); // { read: 3, written: 3 }

// Only 'hel' was written
const decoder = new TextDecoder();
console.log(decoder.decode(buffer)); // "hel"
```

#### Decoding Different Buffer Types

```javascript
const decoder = new TextDecoder();

// Decode from Uint8Array
const uint8 = new Uint8Array([72, 101, 108, 108, 111]);
console.log(decoder.decode(uint8)); // "Hello"

// Decode from ArrayBuffer
const buffer = new Uint8Array([72, 105]).buffer;
console.log(decoder.decode(buffer)); // "Hi"

// Decode from other TypedArray views
const uint16 = new Uint16Array([0x4865, 0x6c6c, 0x6f00]);
const uint8View = new Uint8Array(uint16.buffer, 0, 5);
console.log(decoder.decode(uint8View)); // "Hello"
```

#### Round-trip Encoding and Decoding

```javascript
const encoder = new TextEncoder();
const decoder = new TextDecoder();

// Test with various character sets
const testStrings = [
  'ASCII text',
  'Español',
  '日本語',
  'Emoji: 🌍🌎🌏',
  'Mixed: Hello 世界 🚀',
  'Special chars: \n\t\r\0'
];

testStrings.forEach(original => {
  const encoded = encoder.encode(original);
  const decoded = decoder.decode(encoded);
  console.log(decoded === original); // true for all
});
```

#### Working with File Data

```javascript
import { readFile, writeFile } from 'node:fs/promises';

const encoder = new TextEncoder();
const decoder = new TextDecoder();

// Write text to a file as UTF-8 bytes
const text = 'Hello, World! 🌍';
const bytes = encoder.encode(text);
await writeFile('message.txt', bytes);

// Read UTF-8 bytes from a file and decode
const fileBytes = await readFile('message.txt');
const fileText = decoder.decode(fileBytes);
console.log(fileText); // "Hello, World! 🌍"
```

#### Getting Encoding Information

```javascript
const encoder = new TextEncoder();
const decoder = new TextDecoder();

console.log(encoder.encoding); // "utf-8"
console.log(decoder.encoding); // "utf-8"

// TextEncoder only supports UTF-8
// const encoder2 = new TextEncoder('iso-8859-1'); // Would still be UTF-8

// TextDecoder only supports UTF-8 in jstime
// const decoder2 = new TextDecoder('iso-8859-1'); // Throws RangeError
```

### Use Cases

The Text Encoding API is useful for:

- **Binary data processing**: Converting between strings and byte arrays
- **File I/O**: Reading and writing text files with explicit encoding
- **Network communication**: Encoding/decoding text data for transmission
- **WebAssembly**: Passing string data between JavaScript and WebAssembly modules
- **Cryptography**: Preparing text data for hashing or encryption
- **Data serialization**: Converting text to bytes for storage or transmission

### UTF-8 Support

jstime's `TextEncoder` and `TextDecoder` support the full UTF-8 character set:

- ✅ **ASCII** (1 byte): Basic Latin characters
- ✅ **Latin-1 Supplement** (2 bytes): European characters
- ✅ **BMP** (Basic Multilingual Plane, 2-3 bytes): Most modern scripts
- ✅ **Supplementary Planes** (4 bytes): Emoji, historic scripts, rare characters

```javascript
const encoder = new TextEncoder();

// 1 byte (ASCII)
console.log(encoder.encode('A').length); // 1

// 2 bytes (Latin-1 Supplement)
console.log(encoder.encode('ñ').length); // 2

// 3 bytes (BMP)
console.log(encoder.encode('€').length); // 3
console.log(encoder.encode('世').length); // 3

// 4 bytes (Supplementary Planes)
console.log(encoder.encode('😀').length); // 4
```

### Comparison with Base64

While both deal with text encoding, they serve different purposes:

| Feature | Text Encoding API | Base64 (btoa/atob) |
|---------|------------------|-------------------|
| **Purpose** | Convert between strings and UTF-8 bytes | Convert binary data to ASCII text |
| **Input** | Any Unicode string | Latin-1 strings (0-255) only |
| **Output** | Uint8Array (bytes) | ASCII string |
| **Character support** | Full Unicode (UTF-8) | Latin-1 only |
| **Use case** | Binary data processing | Text-safe binary transmission |

```javascript
const encoder = new TextEncoder();

// Text Encoding: Unicode → UTF-8 bytes
const utf8Bytes = encoder.encode('Hello 世界');
console.log(utf8Bytes); // Uint8Array with UTF-8 bytes

// Base64: Latin-1 string → Base64 ASCII
const base64 = btoa('Hello');
console.log(base64); // "SGVsbG8="

// Base64 cannot handle Unicode directly
// btoa('世界'); // Error!
```

## Web Cryptography API

jstime implements a subset of the [W3C Web Cryptography API](https://w3c.github.io/webcrypto/), providing cryptographically secure operations for generating random values, hashing data, signing/verifying signatures, encrypting/decrypting data, and managing cryptographic keys.

**📁 Example:** See [examples/crypto-advanced-demo.js](../../examples/crypto-advanced-demo.js) for a comprehensive demonstration of all crypto features.

### Supported APIs

#### Random Number Generation

- `crypto.getRandomValues(typedArray)` - Fill a TypedArray with cryptographically strong random values
- `crypto.randomUUID()` - Generate a random UUID v4 string

#### SubtleCrypto Operations

- `crypto.subtle.digest(algorithm, data)` - Compute a hash digest
- `crypto.subtle.sign(algorithm, key, data)` - Generate a digital signature
- `crypto.subtle.verify(algorithm, key, signature, data)` - Verify a digital signature
- `crypto.subtle.encrypt(algorithm, key, data)` - Encrypt data
- `crypto.subtle.decrypt(algorithm, key, data)` - Decrypt data
- `crypto.subtle.generateKey(algorithm, extractable, keyUsages)` - Generate a new cryptographic key
- `crypto.subtle.importKey(format, keyData, algorithm, extractable, keyUsages)` - Import a key
- `crypto.subtle.exportKey(format, key)` - Export a key

### crypto.getRandomValues()

Fills a TypedArray with cryptographically secure random values. The array is modified in-place and also returned.

**Parameters:**
- `typedArray` (TypedArray) - An integer-based TypedArray (Uint8Array, Uint16Array, Uint32Array, etc.)
  - Maximum size: 65,536 bytes

**Returns:** The same TypedArray, filled with random values

**Throws:**
- `TypeError` - If the argument is not a TypedArray
- `Error` - If the array exceeds 65,536 bytes

#### Examples

```javascript
// Fill Uint8Array with random bytes
const array = new Uint8Array(16);
crypto.getRandomValues(array);
console.log(array); // Uint8Array(16) [random values]

// Works with other TypedArrays
const uint32 = new Uint32Array(4);
crypto.getRandomValues(uint32);

// Returns the same array
const result = crypto.getRandomValues(new Uint8Array(32));
console.log(result.length); // 32
```

### crypto.randomUUID()

Generates a cryptographically secure random UUID (Universally Unique Identifier) v4 string.

**Returns:** A string containing a random UUID in the format `xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx`

#### Examples

```javascript
// Generate a UUID
const uuid = crypto.randomUUID();
console.log(uuid); // e.g., "a82be31c-b35d-4f88-8c5e-d5e8f3b0c2a1"

// Each call generates a unique UUID
const uuid1 = crypto.randomUUID();
const uuid2 = crypto.randomUUID();
console.log(uuid1 !== uuid2); // true

// UUID v4 format verification
const uuidRegex = /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/;
console.log(uuidRegex.test(crypto.randomUUID())); // true
```

### crypto.subtle.digest()

Computes a cryptographic hash digest of the provided data. This is an asynchronous operation that returns a Promise.

**Parameters:**
- `algorithm` (string or object) - The hash algorithm to use:
  - `"SHA-256"` - SHA-256 (256-bit hash)
  - `"SHA-384"` - SHA-384 (384-bit hash)
  - `"SHA-512"` - SHA-512 (512-bit hash)
  - Or an object with a `name` property: `{ name: "SHA-256" }`
- `data` (ArrayBuffer or ArrayBufferView) - The data to hash

**Returns:** Promise<ArrayBuffer> - Resolves with the hash digest as an ArrayBuffer

**Throws:**
- `TypeError` - If the algorithm is invalid
- `Error` - If the algorithm is not supported
- `TypeError` - If data is not an ArrayBuffer or ArrayBufferView

#### Examples

##### Basic SHA-256 Hash

```javascript
const encoder = new TextEncoder();
const data = encoder.encode('hello world');

const hashBuffer = await crypto.subtle.digest('SHA-256', data);

// Convert to hex string
const hashArray = Array.from(new Uint8Array(hashBuffer));
const hashHex = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
console.log(hashHex); // b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
```

##### SHA-384 Hash

```javascript
const data = new TextEncoder().encode('hello');
const hash = await crypto.subtle.digest('SHA-384', data);
console.log(hash.byteLength); // 48 (384 bits = 48 bytes)
```

##### SHA-512 Hash

```javascript
const data = new TextEncoder().encode('hello');
const hash = await crypto.subtle.digest('SHA-512', data);
console.log(hash.byteLength); // 64 (512 bits = 64 bytes)
```

##### Hash with ArrayBuffer

```javascript
// Can use ArrayBuffer directly
const buffer = new Uint8Array([104, 101, 108, 108, 111]).buffer; // 'hello'
const hash = await crypto.subtle.digest('SHA-256', buffer);
```

##### Algorithm Object Syntax

```javascript
// Can pass algorithm as object
const data = new TextEncoder().encode('test');
const hash = await crypto.subtle.digest({ name: 'SHA-256' }, data);
```

##### Hash Empty Data

```javascript
// Hashing empty data is supported
const empty = new Uint8Array([]);
const hash = await crypto.subtle.digest('SHA-256', empty);

// Convert to hex
const hashArray = Array.from(new Uint8Array(hash));
const hashHex = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
console.log(hashHex); // e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
```

##### Practical Example: File Integrity Check

```javascript
import { readFile } from 'node:fs/promises';

// Read file as buffer
const fileData = await readFile('./document.pdf');

// Compute SHA-256 hash
const hashBuffer = await crypto.subtle.digest('SHA-256', fileData);

// Convert to hex string for comparison
const hashArray = Array.from(new Uint8Array(hashBuffer));
const hashHex = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');

console.log('File SHA-256:', hashHex);
```

##### Practical Example: Password Hashing Check

```javascript
// Note: For actual password hashing, use a proper password hashing function like bcrypt or argon2
// This is just an example of basic hashing

async function hashPassword(password) {
  const encoder = new TextEncoder();
  const data = encoder.encode(password);
  const hashBuffer = await crypto.subtle.digest('SHA-256', data);
  
  const hashArray = Array.from(new Uint8Array(hashBuffer));
  return hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
}

const password = 'my-secure-password';
const hash = await hashPassword(password);
console.log('Password hash:', hash);
```

### Use Cases

The Web Cryptography API is useful for:

- **Random number generation**: Generate cryptographically secure random values for tokens, IDs, or cryptographic operations
- **UUID generation**: Create unique identifiers for resources, sessions, or tracking
- **Data integrity**: Verify file or data integrity using hash digests
- **Fingerprinting**: Create unique fingerprints of data for comparison or caching
- **Non-reversible data storage**: Store hashed data (though use proper password hashing for passwords)

### Security Notes

- **Random values**: All random values are generated using a cryptographically secure random number generator (ring's `SystemRandom`)
- **Hash algorithms**: SHA-256, SHA-384, and SHA-512 are cryptographically secure hash functions
- **Password hashing**: For password hashing, consider using specialized password hashing functions (like bcrypt, scrypt, or argon2) instead of simple SHA hashing
- **HTTPS**: When transmitting sensitive data, always use HTTPS

### Supported Hash Algorithms

| Algorithm | Output Size | Use Case |
|-----------|-------------|----------|
| SHA-256   | 256 bits (32 bytes) | General purpose, most common |
| SHA-384   | 384 bits (48 bytes) | Higher security than SHA-256 |
| SHA-512   | 512 bits (64 bytes) | Maximum security for hash digests |

### Comparison with Node.js crypto

While jstime's crypto API is based on the Web Cryptography API standard, Node.js uses its own `crypto` module. Here are the key differences:

| Feature | jstime (Web Crypto API) | Node.js crypto |
|---------|------------------------|----------------|
| **API Standard** | W3C Web Cryptography API | Node.js-specific API |
| **getRandomValues** | ✅ Supported | ❌ Use `crypto.randomBytes()` instead |
| **randomUUID** | ✅ Supported | ✅ Supported (crypto.randomUUID()) |
| **subtle.digest** | ✅ Supported (SHA-256/384/512) | ✅ Supported (many algorithms) |
| **subtle.sign/verify** | ✅ Supported (HMAC) | ✅ Supported (HMAC, RSA, ECDSA) |
| **subtle.encrypt/decrypt** | ✅ Supported (AES-GCM) | ✅ Supported (many algorithms) |
| **subtle.generateKey** | ✅ Supported (AES-GCM, HMAC) | ✅ Supported (many algorithms) |
| **subtle.importKey/exportKey** | ✅ Supported (raw format) | ✅ Supported (many formats) |
| **Key formats** | Raw only | JWK, PKCS#8, SPKI, raw |
| **RSA** | ❌ Not yet supported | ✅ Supported |
| **ECDSA** | ❌ Not yet supported | ✅ Supported |

### crypto.subtle.sign()

Generates a digital signature for the provided data using the specified algorithm and key. Returns a Promise that resolves to an ArrayBuffer containing the signature.

**Parameters:**
- `algorithm` (string or object) - The signing algorithm:
  - `"HMAC"` - HMAC (Hash-based Message Authentication Code)
  - Or an object with a `name` property: `{ name: "HMAC" }`
- `key` (CryptoKey) - The key to use for signing (must have `sign` usage)
- `data` (ArrayBuffer or ArrayBufferView) - The data to sign

**Returns:** Promise<ArrayBuffer> - Resolves with the signature

**Supported Algorithms:**
- **HMAC** with SHA-256, SHA-384, or SHA-512

#### Examples

##### Basic HMAC Signing

```javascript
const encoder = new TextEncoder();

// Generate an HMAC key
const key = await crypto.subtle.generateKey(
  {
    name: 'HMAC',
    hash: 'SHA-256',
  },
  false,
  ['sign', 'verify']
);

// Sign some data
const data = encoder.encode('Message to sign');
const signature = await crypto.subtle.sign('HMAC', key, data);

console.log('Signature length:', signature.byteLength); // 32 bytes for SHA-256
```

##### HMAC with Different Hash Algorithms

```javascript
// SHA-384 produces 48-byte signatures
const key384 = await crypto.subtle.generateKey(
  { name: 'HMAC', hash: 'SHA-384' },
  false,
  ['sign']
);
const sig384 = await crypto.subtle.sign('HMAC', key384, data);
console.log(sig384.byteLength); // 48

// SHA-512 produces 64-byte signatures
const key512 = await crypto.subtle.generateKey(
  { name: 'HMAC', hash: 'SHA-512' },
  false,
  ['sign']
);
const sig512 = await crypto.subtle.sign('HMAC', key512, data);
console.log(sig512.byteLength); // 64
```

### crypto.subtle.verify()

Verifies a digital signature using the specified algorithm and key. Returns a Promise that resolves to a boolean indicating whether the signature is valid.

**Parameters:**
- `algorithm` (string or object) - The verification algorithm (same as used for signing)
- `key` (CryptoKey) - The key to use for verification (must have `verify` usage)
- `signature` (ArrayBuffer or ArrayBufferView) - The signature to verify
- `data` (ArrayBuffer or ArrayBufferView) - The original data

**Returns:** Promise<boolean> - Resolves with `true` if valid, `false` otherwise

#### Examples

##### Basic Signature Verification

```javascript
const encoder = new TextEncoder();

// Generate key and sign
const key = await crypto.subtle.generateKey(
  { name: 'HMAC', hash: 'SHA-256' },
  false,
  ['sign', 'verify']
);

const data = encoder.encode('Message to authenticate');
const signature = await crypto.subtle.sign('HMAC', key, data);

// Verify the signature
const isValid = await crypto.subtle.verify('HMAC', key, signature, data);
console.log('Signature valid:', isValid); // true

// Verify with tampered data
const tamperedData = encoder.encode('Modified message');
const isInvalid = await crypto.subtle.verify('HMAC', key, signature, tamperedData);
console.log('Tampered signature valid:', isInvalid); // false
```

##### Message Authentication Example

```javascript
// Sender: Sign a message
const message = encoder.encode('Transfer $100 to Bob');
const senderKey = await crypto.subtle.generateKey(
  { name: 'HMAC', hash: 'SHA-256' },
  true, // extractable for sharing
  ['sign', 'verify']
);

const signature = await crypto.subtle.sign('HMAC', senderKey, message);

// Send: message + signature

// Receiver: Verify the message hasn't been tampered with
const isAuthentic = await crypto.subtle.verify('HMAC', senderKey, signature, message);

if (isAuthentic) {
  console.log('Message is authentic');
} else {
  console.log('Message has been tampered with!');
}
```

### crypto.subtle.encrypt()

Encrypts data using the specified algorithm and key. Returns a Promise that resolves to an ArrayBuffer containing the encrypted data (ciphertext).

**Parameters:**
- `algorithm` (object) - The encryption algorithm and parameters:
  - For AES-GCM: `{ name: "AES-GCM", iv: Uint8Array, additionalData?: Uint8Array }`
- `key` (CryptoKey) - The key to use for encryption (must have `encrypt` usage)
- `data` (ArrayBuffer or ArrayBufferView) - The data to encrypt (plaintext)

**Returns:** Promise<ArrayBuffer> - Resolves with the encrypted data (includes authentication tag for AES-GCM)

**Supported Algorithms:**
- **AES-GCM** (256-bit) - Authenticated encryption with associated data

#### AES-GCM Parameters

- `iv` (Uint8Array) - Initialization vector (12 bytes recommended for AES-GCM)
- `additionalData` (Uint8Array, optional) - Additional authenticated data (AAD) that is authenticated but not encrypted

#### Examples

##### Basic AES-GCM Encryption

```javascript
const encoder = new TextEncoder();

// Generate an AES-GCM key
const key = await crypto.subtle.generateKey(
  {
    name: 'AES-GCM',
    length: 256, // 256-bit key
  },
  false,
  ['encrypt', 'decrypt']
);

// Generate a random IV (initialization vector)
const iv = crypto.getRandomValues(new Uint8Array(12));

// Encrypt data
const plaintext = encoder.encode('Secret message');
const ciphertext = await crypto.subtle.encrypt(
  {
    name: 'AES-GCM',
    iv: iv,
  },
  key,
  plaintext
);

console.log('Ciphertext length:', ciphertext.byteLength);
// Length = plaintext length + 16 bytes (authentication tag)
```

##### AES-GCM with Additional Authenticated Data

```javascript
const encoder = new TextEncoder();

// Generate key
const key = await crypto.subtle.generateKey(
  { name: 'AES-GCM', length: 256 },
  false,
  ['encrypt', 'decrypt']
);

const iv = crypto.getRandomValues(new Uint8Array(12));

// Additional data to authenticate (but not encrypt)
const metadata = encoder.encode('user:alice,timestamp:1234567890');
const sensitiveData = encoder.encode('Password: secret123');

// Encrypt with AAD
const ciphertext = await crypto.subtle.encrypt(
  {
    name: 'AES-GCM',
    iv: iv,
    additionalData: metadata, // Authenticated but not encrypted
  },
  key,
  sensitiveData
);

// The metadata is authenticated - decryption will fail if it's modified
```

### crypto.subtle.decrypt()

Decrypts data using the specified algorithm and key. Returns a Promise that resolves to an ArrayBuffer containing the decrypted data (plaintext).

**Parameters:**
- `algorithm` (object) - The decryption algorithm and parameters (must match encryption)
- `key` (CryptoKey) - The key to use for decryption (must have `decrypt` usage)
- `data` (ArrayBuffer or ArrayBufferView) - The encrypted data (ciphertext)

**Returns:** Promise<ArrayBuffer> - Resolves with the decrypted data

**Throws:**
- `Error` - If decryption fails (wrong key, tampered data, or wrong AAD)

#### Examples

##### Basic AES-GCM Decryption

```javascript
const encoder = new TextEncoder();
const decoder = new TextDecoder();

// Setup: same key and IV as encryption
const key = await crypto.subtle.generateKey(
  { name: 'AES-GCM', length: 256 },
  false,
  ['encrypt', 'decrypt']
);

const iv = crypto.getRandomValues(new Uint8Array(12));

// Encrypt
const plaintext = encoder.encode('Secret message');
const ciphertext = await crypto.subtle.encrypt(
  { name: 'AES-GCM', iv: iv },
  key,
  plaintext
);

// Decrypt
const decrypted = await crypto.subtle.decrypt(
  { name: 'AES-GCM', iv: iv },
  key,
  ciphertext
);

const decryptedText = decoder.decode(decrypted);
console.log(decryptedText); // "Secret message"
```

##### Handling Decryption Failures

```javascript
try {
  // Try to decrypt with wrong IV
  const wrongIv = crypto.getRandomValues(new Uint8Array(12));
  await crypto.subtle.decrypt(
    { name: 'AES-GCM', iv: wrongIv },
    key,
    ciphertext
  );
} catch (e) {
  console.log('Decryption failed:', e.message);
  // This is expected - authentication failed
}
```

##### Complete Encryption/Decryption Example

```javascript
const encoder = new TextEncoder();
const decoder = new TextDecoder();

// Generate key
const key = await crypto.subtle.generateKey(
  { name: 'AES-GCM', length: 256 },
  false,
  ['encrypt', 'decrypt']
);

// Encrypt
const iv = crypto.getRandomValues(new Uint8Array(12));
const message = 'Confidential data';
const encrypted = await crypto.subtle.encrypt(
  { name: 'AES-GCM', iv: iv },
  key,
  encoder.encode(message)
);

// Store/transmit: iv + encrypted

// Decrypt
const decrypted = await crypto.subtle.decrypt(
  { name: 'AES-GCM', iv: iv },
  key,
  encrypted
);

console.log(decoder.decode(decrypted)); // "Confidential data"
```

### crypto.subtle.generateKey()

Generates a new cryptographic key for the specified algorithm. Returns a Promise that resolves to a CryptoKey object.

**Parameters:**
- `algorithm` (object) - The key generation algorithm and parameters:
  - For AES-GCM: `{ name: "AES-GCM", length: number }`
  - For HMAC: `{ name: "HMAC", hash: string, length?: number }`
- `extractable` (boolean) - Whether the key can be exported with `exportKey()`
- `keyUsages` (Array<string>) - What the key can be used for:
  - For AES-GCM: `["encrypt", "decrypt"]`
  - For HMAC: `["sign", "verify"]`

**Returns:** Promise<CryptoKey> - Resolves with the generated key

#### Supported Algorithms

| Algorithm | Parameters | Key Usages | Notes |
|-----------|------------|------------|-------|
| AES-GCM   | `length: 128, 192, or 256` | `encrypt`, `decrypt` | 256-bit recommended |
| HMAC      | `hash: "SHA-256", "SHA-384", or "SHA-512"`, `length?: number` | `sign`, `verify` | Length defaults based on hash |

#### Examples

##### Generate AES-GCM Key

```javascript
// Generate a 256-bit AES-GCM key (recommended)
const aesKey = await crypto.subtle.generateKey(
  {
    name: 'AES-GCM',
    length: 256,
  },
  true, // extractable
  ['encrypt', 'decrypt']
);

console.log('Key type:', aesKey.type); // "secret"
console.log('Extractable:', aesKey.extractable); // true
console.log('Algorithm:', aesKey.algorithm.name); // "AES-GCM"
console.log('Usages:', aesKey.usages); // ["encrypt", "decrypt"]
```

##### Generate HMAC Key

```javascript
// Generate HMAC key for SHA-256
const hmacKey = await crypto.subtle.generateKey(
  {
    name: 'HMAC',
    hash: 'SHA-256',
  },
  false, // not extractable
  ['sign', 'verify']
);

console.log('Key type:', hmacKey.type); // "secret"
console.log('Hash:', hmacKey.algorithm.hash.name); // "SHA-256"
```

##### Generate Keys with Different Security Levels

```javascript
// Different AES key lengths
const aes128 = await crypto.subtle.generateKey(
  { name: 'AES-GCM', length: 128 },
  true,
  ['encrypt', 'decrypt']
);

const aes192 = await crypto.subtle.generateKey(
  { name: 'AES-GCM', length: 192 },
  true,
  ['encrypt', 'decrypt']
);

const aes256 = await crypto.subtle.generateKey(
  { name: 'AES-GCM', length: 256 },
  true,
  ['encrypt', 'decrypt']
);

// Different HMAC hash algorithms
const hmacSha256 = await crypto.subtle.generateKey(
  { name: 'HMAC', hash: 'SHA-256' },
  false,
  ['sign', 'verify']
);

const hmacSha512 = await crypto.subtle.generateKey(
  { name: 'HMAC', hash: 'SHA-512' },
  false,
  ['sign', 'verify']
);
```

### crypto.subtle.importKey()

Imports a cryptographic key from external key data. Returns a Promise that resolves to a CryptoKey object.

**Parameters:**
- `format` (string) - The data format of the key:
  - `"raw"` - Raw bytes (currently the only supported format)
- `keyData` (ArrayBuffer or ArrayBufferView) - The key data in the specified format
- `algorithm` (object) - The algorithm the key will be used with
- `extractable` (boolean) - Whether the key can be exported
- `keyUsages` (Array<string>) - What the key can be used for

**Returns:** Promise<CryptoKey> - Resolves with the imported key

#### Examples

##### Import AES-GCM Key

```javascript
// Key data (32 bytes for AES-256)
const keyData = new Uint8Array(32);
crypto.getRandomValues(keyData);

// Import the key
const key = await crypto.subtle.importKey(
  'raw',
  keyData,
  { name: 'AES-GCM' },
  true,
  ['encrypt', 'decrypt']
);

console.log('Imported key:', key.algorithm.name); // "AES-GCM"
```

##### Import HMAC Key

```javascript
// HMAC key data
const hmacKeyData = new Uint8Array(32);
crypto.getRandomValues(hmacKeyData);

const hmacKey = await crypto.subtle.importKey(
  'raw',
  hmacKeyData,
  {
    name: 'HMAC',
    hash: 'SHA-256',
  },
  false,
  ['sign', 'verify']
);
```

##### Import Key from Hex String

```javascript
// Convert hex string to bytes
function hexToBytes(hex) {
  const bytes = new Uint8Array(hex.length / 2);
  for (let i = 0; i < hex.length; i += 2) {
    bytes[i / 2] = parseInt(hex.substr(i, 2), 16);
  }
  return bytes;
}

const hexKey = 'a1b2c3d4e5f6...'; // 64 hex chars for 32 bytes
const keyBytes = hexToBytes(hexKey);

const key = await crypto.subtle.importKey(
  'raw',
  keyBytes,
  { name: 'AES-GCM' },
  true,
  ['encrypt', 'decrypt']
);
```

### crypto.subtle.exportKey()

Exports a cryptographic key to an external format. Returns a Promise that resolves to an ArrayBuffer containing the key data.

**Parameters:**
- `format` (string) - The data format to export to:
  - `"raw"` - Raw bytes (currently the only supported format)
- `key` (CryptoKey) - The key to export (must be extractable)

**Returns:** Promise<ArrayBuffer> - Resolves with the exported key data

**Throws:**
- `Error` - If the key is not extractable

#### Examples

##### Export and Re-import Key

```javascript
// Generate an extractable key
const originalKey = await crypto.subtle.generateKey(
  { name: 'AES-GCM', length: 256 },
  true, // must be extractable
  ['encrypt', 'decrypt']
);

// Export the key
const exportedKey = await crypto.subtle.exportKey('raw', originalKey);
console.log('Exported key length:', exportedKey.byteLength); // 32 bytes

// Import it back
const importedKey = await crypto.subtle.importKey(
  'raw',
  exportedKey,
  { name: 'AES-GCM' },
  true,
  ['encrypt', 'decrypt']
);
```

##### Convert Key to Hex String

```javascript
function bytesToHex(bytes) {
  return Array.from(new Uint8Array(bytes))
    .map(b => b.toString(16).padStart(2, '0'))
    .join('');
}

const key = await crypto.subtle.generateKey(
  { name: 'AES-GCM', length: 256 },
  true,
  ['encrypt', 'decrypt']
);

const exported = await crypto.subtle.exportKey('raw', key);
const hexKey = bytesToHex(exported);
console.log('Key (hex):', hexKey);
```

##### Non-extractable Keys

```javascript
// Generate a non-extractable key
const secureKey = await crypto.subtle.generateKey(
  { name: 'HMAC', hash: 'SHA-256' },
  false, // not extractable
  ['sign', 'verify']
);

try {
  await crypto.subtle.exportKey('raw', secureKey);
} catch (e) {
  console.log('Cannot export:', e.message);
  // "Key is not extractable"
}
```

### Use Cases

The expanded Web Cryptography API is useful for:

- **Message Authentication**: Use HMAC to verify message integrity and authenticity
- **Data Encryption**: Encrypt sensitive data with AES-GCM for confidentiality and integrity
- **Secure Communication**: Implement encrypted channels with authenticated encryption
- **Key Management**: Generate, import, export, and manage cryptographic keys
- **API Security**: Sign API requests and verify responses
- **Session Management**: Create and verify secure session tokens
- **File Encryption**: Encrypt files before storage or transmission
- **Password Verification**: Hash and verify passwords (though use dedicated password hashing for production)

### Security Best Practices

1. **Use Strong Keys**: Always use 256-bit keys for AES-GCM
2. **Never Reuse IVs**: Each encryption must use a unique IV with the same key
3. **Use AAD When Possible**: Additional authenticated data adds another layer of security
4. **Protect Keys**: Store keys securely and mark them as non-extractable when appropriate
5. **Verify Signatures**: Always verify HMAC signatures before trusting data
6. **Handle Errors**: Decryption failures indicate tampered or corrupted data
7. **Use HTTPS**: Always transmit encrypted data over secure channels

### Algorithm Support Matrix

| Feature | AES-GCM | HMAC |
|---------|---------|------|
| **Key Lengths** | 128, 192, 256 bits | Flexible (defaults: 256, 384, 512 bits) |
| **Hash Algorithms** | N/A | SHA-256, SHA-384, SHA-512 |
| **Operations** | encrypt, decrypt | sign, verify |
| **Additional Data** | Yes (AAD) | No |
| **Authentication** | Built-in (authenticated encryption) | Full message |
| **Key Import/Export** | ✅ Raw format | ✅ Raw format |

### Future Enhancements

Potential additions being considered:

- Additional key formats (JWK, PKCS#8, SPKI)
- RSA encryption and signatures
- ECDSA signatures
- Key derivation (PBKDF2, HKDF)
- Additional AES modes (CBC, CTR)
